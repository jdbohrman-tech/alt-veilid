mod envelope;
mod key;
mod receipt;
mod value;

pub mod tests;

pub use envelope::*;
pub use key::*;
pub use receipt::*;
pub use value::*;

pub const MIN_CRYPTO_VERSION: u8 = 0u8;
pub const MAX_CRYPTO_VERSION: u8 = 0u8;

use crate::xx::*;
use crate::*;
use chacha20::cipher::{KeyIvInit, StreamCipher};
use chacha20::XChaCha20;
use chacha20poly1305 as ch;
use chacha20poly1305::aead::{AeadInPlace, NewAead};
use core::convert::TryInto;
use curve25519_dalek as cd;
use ed25519_dalek as ed;
use hashlink::linked_hash_map::Entry;
use hashlink::LruCache;
use serde::{Deserialize, Serialize};
use x25519_dalek as xd;

pub type SharedSecret = [u8; 32];
pub type Nonce = [u8; 24];

const DH_CACHE_SIZE: usize = 1024;
pub const AEAD_OVERHEAD: usize = 16;

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
        }
    }

    pub fn new(config: VeilidConfig, table_store: TableStore) -> Self {
        Self {
            config,
            inner: Arc::new(Mutex::new(Self::new_inner(table_store))),
        }
    }

    pub async fn init(&self) -> EyreResult<()> {
        trace!("Crypto::init");

        // make local copy of node id for easy access
        let (table_store, node_id) = {
            let mut inner = self.inner.lock();
            let c = self.config.get();
            inner.node_id = c.network.node_id;
            inner.node_id_secret = c.network.node_id_secret;
            (inner.table_store.clone(), c.network.node_id)
        };

        // load caches if they are valid for this node id
        let mut db = table_store.open("crypto_caches", 1).await?;
        let caches_valid = match db.load(0, b"node_id").await? {
            Some(v) => v.as_slice() == node_id.bytes,
            None => false,
        };
        if caches_valid {
            if let Some(b) = db.load(0, b"dh_cache").await? {
                let mut inner = self.inner.lock();
                bytes_to_cache(&b, &mut inner.dh_cache);
            }
        } else {
            drop(db);
            table_store.delete("crypto_caches").await?;
            db = table_store.open("crypto_caches", 1).await?;
            db.store(0, b"node_id", &node_id.bytes).await?;
        }

        // Schedule flushing
        let this = self.clone();
        let flush_future = intf::interval(60000, move || {
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

    fn ed25519_to_x25519_pk(key: &ed::PublicKey) -> Result<xd::PublicKey, VeilidAPIError> {
        let bytes = key.to_bytes();
        let compressed = cd::edwards::CompressedEdwardsY(bytes);
        let point = compressed
            .decompress()
            .ok_or_else(|| VeilidAPIError::internal("ed25519_to_x25519_pk failed"))?;
        let mp = point.to_montgomery();
        Ok(xd::PublicKey::from(mp.to_bytes()))
    }
    fn ed25519_to_x25519_sk(key: &ed::SecretKey) -> Result<xd::StaticSecret, VeilidAPIError> {
        let exp = ed::ExpandedSecretKey::from(key);
        let bytes: [u8; ed::EXPANDED_SECRET_KEY_LENGTH] = exp.to_bytes();
        let lowbytes: [u8; 32] = bytes[0..32].try_into().map_err(VeilidAPIError::internal)?;
        Ok(xd::StaticSecret::from(lowbytes))
    }

    pub fn cached_dh(
        &self,
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
                    let shared_secret = Self::compute_dh(key, secret)?;
                    e.insert(DHCacheValue { shared_secret });
                    shared_secret
                }
            },
        )
    }

    ///////////
    // These are safe to use regardless of initialization status

    pub fn compute_dh(key: &DHTKey, secret: &DHTKeySecret) -> Result<SharedSecret, VeilidAPIError> {
        assert!(key.valid);
        assert!(secret.valid);
        let pk_ed = ed::PublicKey::from_bytes(&key.bytes).map_err(VeilidAPIError::internal)?;
        let pk_xd = Self::ed25519_to_x25519_pk(&pk_ed)?;
        let sk_ed = ed::SecretKey::from_bytes(&secret.bytes).map_err(VeilidAPIError::internal)?;
        let sk_xd = Self::ed25519_to_x25519_sk(&sk_ed)?;
        Ok(sk_xd.diffie_hellman(&pk_xd).to_bytes())
    }

    pub fn get_random_nonce() -> Nonce {
        let mut nonce = [0u8; 24];
        intf::random_bytes(&mut nonce).unwrap();
        nonce
    }

    pub fn get_random_secret() -> SharedSecret {
        let mut s = [0u8; 32];
        intf::random_bytes(&mut s).unwrap();
        s
    }

    pub fn decrypt_in_place_aead(
        body: &mut Vec<u8>,
        nonce: &Nonce,
        shared_secret: &SharedSecret,
        associated_data: Option<&[u8]>,
    ) -> Result<(), VeilidAPIError> {
        let key = ch::Key::from(*shared_secret);
        let xnonce = ch::XNonce::from(*nonce);
        let aead = ch::XChaCha20Poly1305::new(&key);
        aead.decrypt_in_place(&xnonce, associated_data.unwrap_or(b""), body)
            .map_err(map_to_string)
            .map_err(VeilidAPIError::generic)
    }

    pub fn decrypt_aead(
        body: &[u8],
        nonce: &Nonce,
        shared_secret: &SharedSecret,
        associated_data: Option<&[u8]>,
    ) -> Result<Vec<u8>, VeilidAPIError> {
        let mut out = body.to_vec();
        Self::decrypt_in_place_aead(&mut out, nonce, shared_secret, associated_data)
            .map_err(map_to_string)
            .map_err(VeilidAPIError::generic)?;
        Ok(out)
    }

    pub fn encrypt_in_place_aead(
        body: &mut Vec<u8>,
        nonce: &Nonce,
        shared_secret: &SharedSecret,
        associated_data: Option<&[u8]>,
    ) -> Result<(), VeilidAPIError> {
        let key = ch::Key::from(*shared_secret);
        let xnonce = ch::XNonce::from(*nonce);
        let aead = ch::XChaCha20Poly1305::new(&key);

        aead.encrypt_in_place(&xnonce, associated_data.unwrap_or(b""), body)
            .map_err(map_to_string)
            .map_err(VeilidAPIError::generic)
    }

    pub fn encrypt_aead(
        body: &[u8],
        nonce: &Nonce,
        shared_secret: &SharedSecret,
        associated_data: Option<&[u8]>,
    ) -> Result<Vec<u8>, VeilidAPIError> {
        let mut out = body.to_vec();
        Self::encrypt_in_place_aead(&mut out, nonce, shared_secret, associated_data)
            .map_err(map_to_string)
            .map_err(VeilidAPIError::generic)?;
        Ok(out)
    }

    pub fn crypt_in_place_no_auth(body: &mut Vec<u8>, nonce: &Nonce, shared_secret: &SharedSecret) {
        let mut cipher = XChaCha20::new(shared_secret.into(), nonce.into());
        cipher.apply_keystream(body);
    }

    pub fn crypt_b2b_no_auth(
        in_buf: &[u8],
        nonce: &Nonce,
        shared_secret: &SharedSecret,
    ) -> Vec<u8> {
        let mut cipher = XChaCha20::new(shared_secret.into(), nonce.into());
        // Allocate uninitialized memory, aligned to 8 byte boundary because capnp is faster this way
        // and the Vec returned here will be used to hold decrypted rpc messages
        let mut out_buf = unsafe { aligned_8_u8_vec_uninit(in_buf.len()) };
        cipher.apply_keystream_b2b(in_buf, &mut out_buf).unwrap();
        out_buf
    }

    pub fn crypt_no_auth(body: &[u8], nonce: &Nonce, shared_secret: &SharedSecret) -> Vec<u8> {
        Self::crypt_b2b_no_auth(body, nonce, shared_secret)
    }
}
