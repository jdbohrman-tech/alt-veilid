mod envelope;
mod key;
mod receipt;
mod value;

pub mod crypto_system;
pub mod tests;
pub mod v0;

pub use crypto_system::*;
pub use envelope::*;
pub use key::*;
pub use receipt::*;
pub use value::*;

pub type CryptoVersion = u8;
pub const MIN_CRYPTO_VERSION: CryptoVersion = 0u8;
pub const MAX_CRYPTO_VERSION: CryptoVersion = 0u8;

use crate::*;
use core::convert::TryInto;
use hashlink::linked_hash_map::Entry;
use hashlink::LruCache;
use serde::{Deserialize, Serialize};

pub type SharedSecret = [u8; 32];
pub type Nonce = [u8; 24];

pub type CryptoSystemVersion = Arc<dyn CryptoSystem + Send + Sync>;

const DH_CACHE_SIZE: usize = 4096;

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
struct DHCacheKey {
    key: DHTKey,
    secret: DHTKeySecret,
}

#[derive(Serialize, Deserialize)]
struct DHCacheValue {
    shared_secret: SharedSecret,
}
type DHCache = LruCache<DHCacheKey, DHCacheValue>;

fn cache_to_bytes(cache: &DHCache) -> Vec<u8> {
    let cnt: usize = cache.len();
    let mut out: Vec<u8> = Vec::with_capacity(cnt * (32 + 32 + 32));
    for e in cache.iter() {
        out.extend(&e.0.key.bytes);
        out.extend(&e.0.secret.bytes);
        out.extend(&e.1.shared_secret);
    }
    let mut rev: Vec<u8> = Vec::with_capacity(out.len());
    for d in out.chunks(32 + 32 + 32).rev() {
        rev.extend(d);
    }
    rev
}

fn bytes_to_cache(bytes: &[u8], cache: &mut DHCache) {
    for d in bytes.chunks(32 + 32 + 32) {
        let k = DHCacheKey {
            key: DHTKey::new(d[0..32].try_into().expect("asdf")),
            secret: DHTKeySecret::new(d[32..64].try_into().expect("asdf")),
        };
        let v = DHCacheValue {
            shared_secret: d[64..96].try_into().expect("asdf"),
        };
        cache.insert(k, v);
    }
}

struct CryptoInner {
    table_store: TableStore,
    node_id: DHTKey,
    node_id_secret: DHTKeySecret,
    dh_cache: DHCache,
    flush_future: Option<SendPinBoxFuture<()>>,
    crypto_v0: Option<Arc<dyn CryptoSystem + Send + Sync>>,
}

#[derive(Clone)]
pub struct Crypto {
    config: VeilidConfig,
    inner: Arc<Mutex<CryptoInner>>,
}

impl Crypto {
    fn new_inner(table_store: TableStore) -> CryptoInner {
        CryptoInner {
            table_store,
            node_id: Default::default(),
            node_id_secret: Default::default(),
            dh_cache: DHCache::new(DH_CACHE_SIZE),
            flush_future: None,
            crypto_v0: None,
        }
    }

    pub fn new(config: VeilidConfig, table_store: TableStore) -> Self {
        let out = Self {
            config,
            inner: Arc::new(Mutex::new(Self::new_inner(table_store))),
        };

        out.inner.lock().crypto_v0 = Some(Arc::new(v0::CryptoV0System::new(out.clone())));

        out
    }

    pub async fn init(&self) -> EyreResult<()> {
        trace!("Crypto::init");

        // make local copy of node id for easy access
        let (table_store, node_id) = {
            let mut inner = self.inner.lock();
            let c = self.config.get();
            inner.node_id = c.network.node_id.unwrap();
            inner.node_id_secret = c.network.node_id_secret.unwrap();
            (inner.table_store.clone(), c.network.node_id)
        };

        // load caches if they are valid for this node id
        let mut db = table_store.open("crypto_caches", 1).await?;
        let caches_valid = match db.load(0, b"node_id")? {
            Some(v) => v.as_slice() == node_id.unwrap().bytes,
            None => false,
        };
        if caches_valid {
            if let Some(b) = db.load(0, b"dh_cache")? {
                let mut inner = self.inner.lock();
                bytes_to_cache(&b, &mut inner.dh_cache);
            }
        } else {
            drop(db);
            table_store.delete("crypto_caches").await?;
            db = table_store.open("crypto_caches", 1).await?;
            db.store(0, b"node_id", &node_id.unwrap().bytes).await?;
        }

        // Schedule flushing
        let this = self.clone();
        let flush_future = interval(60000, move || {
            let this = this.clone();
            async move {
                if let Err(e) = this.flush().await {
                    warn!("flush failed: {}", e);
                }
            }
        });
        self.inner.lock().flush_future = Some(flush_future);

        Ok(())
    }

    pub async fn flush(&self) -> EyreResult<()> {
        //trace!("Crypto::flush");
        let (table_store, cache_bytes) = {
            let inner = self.inner.lock();
            let cache_bytes = cache_to_bytes(&inner.dh_cache);
            (inner.table_store.clone(), cache_bytes)
        };

        let db = table_store.open("crypto_caches", 1).await?;
        db.store(0, b"dh_cache", &cache_bytes).await?;
        Ok(())
    }

    pub async fn terminate(&self) {
        trace!("Crypto::terminate");
        let flush_future = self.inner.lock().flush_future.take();
        if let Some(f) = flush_future {
            f.await;
        }
        trace!("starting termination flush");
        match self.flush().await {
            Ok(_) => {
                trace!("finished termination flush");
            }
            Err(e) => {
                error!("failed termination flush: {}", e);
            }
        };
    }

    // Factory
    fn get(&self, version: CryptoVersion) -> Result<CryptoSystemVersion, VeilidAPIError> {
        let inner = self.inner.lock();
        match version {
            0u8 => Ok(inner.crypto_v0.clone().unwrap()),
            _ => Err(VeilidAPIError::InvalidArgument {
                context: "Unsupported crypto version".to_owned(),
                argument: "version".to_owned(),
                value: format!("{}", version),
            }),
        }
    }

    // Internal utilities

    fn cached_dh_internal<T: CryptoSystem>(
        &self,
        vcrypto: &T,
        key: &DHTKey,
        secret: &DHTKeySecret,
    ) -> Result<SharedSecret, VeilidAPIError> {
        Ok(
            match self.inner.lock().dh_cache.entry(DHCacheKey {
                key: *key,
                secret: *secret,
            }) {
                Entry::Occupied(e) => e.get().shared_secret,
                Entry::Vacant(e) => {
                    let shared_secret = vcrypto.compute_dh(key, secret)?;
                    e.insert(DHCacheValue { shared_secret });
                    shared_secret
                }
            },
        )
    }
}
