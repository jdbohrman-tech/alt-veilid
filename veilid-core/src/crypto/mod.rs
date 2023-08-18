mod blake3digest512;
mod dh_cache;
mod envelope;
mod receipt;
mod types;

pub mod crypto_system;
#[cfg(feature = "enable-crypto-none")]
pub mod none;
pub mod tests;
#[cfg(feature = "enable-crypto-vld0")]
pub mod vld0;

pub use blake3digest512::*;

pub use crypto_system::*;
pub use dh_cache::*;
pub use envelope::*;
pub use receipt::*;
pub use types::*;

#[cfg(feature = "enable-crypto-none")]
pub use none::*;
#[cfg(feature = "enable-crypto-vld0")]
pub use vld0::*;

use super::*;
use core::convert::TryInto;
use hashlink::linked_hash_map::Entry;
use hashlink::LruCache;

/// Handle to a particular cryptosystem
pub type CryptoSystemVersion = Arc<dyn CryptoSystem + Send + Sync>;

cfg_if! {
    if #[cfg(all(feature = "enable-crypto-none", feature = "enable-crypto-vld0"))] {
        /// Crypto kinds in order of preference, best cryptosystem is the first one, worst is the last one
        pub const VALID_CRYPTO_KINDS: [CryptoKind; 2] = [CRYPTO_KIND_VLD0, CRYPTO_KIND_NONE];
    }
    else if #[cfg(feature = "enable-crypto-none")] {
        /// Crypto kinds in order of preference, best cryptosystem is the first one, worst is the last one
        pub const VALID_CRYPTO_KINDS: [CryptoKind; 1] = [CRYPTO_KIND_NONE];
    }
    else if #[cfg(feature = "enable-crypto-vld0")] {
        /// Crypto kinds in order of preference, best cryptosystem is the first one, worst is the last one
        pub const VALID_CRYPTO_KINDS: [CryptoKind; 1] = [CRYPTO_KIND_VLD0];
    }
    else {
        compile_error!("No crypto kinds enabled, specify an enable-crypto- feature");
    }
}
/// Number of cryptosystem signatures to keep on structures if many are present beyond the ones we consider valid
pub const MAX_CRYPTO_KINDS: usize = 3;
/// Return the best cryptosystem kind we support
pub fn best_crypto_kind() -> CryptoKind {
    VALID_CRYPTO_KINDS[0]
}

/// Version number of envelope format
pub type EnvelopeVersion = u8;

/// Envelope versions in order of preference, best envelope version is the first one, worst is the last one
pub const VALID_ENVELOPE_VERSIONS: [EnvelopeVersion; 1] = [0u8];
/// Number of envelope versions to keep on structures if many are present beyond the ones we consider valid
pub const MAX_ENVELOPE_VERSIONS: usize = 3;
/// Return the best envelope version we support
pub fn best_envelope_version() -> EnvelopeVersion {
    VALID_ENVELOPE_VERSIONS[0]
}

struct CryptoInner {
    dh_cache: DHCache,
    flush_future: Option<SendPinBoxFuture<()>>,
    #[cfg(feature = "enable-crypto-vld0")]
    crypto_vld0: Option<Arc<dyn CryptoSystem + Send + Sync>>,
    #[cfg(feature = "enable-crypto-none")]
    crypto_none: Option<Arc<dyn CryptoSystem + Send + Sync>>,
}

struct CryptoUnlockedInner {
    config: VeilidConfig,
    table_store: TableStore,
}

/// Crypto factory implementation
#[derive(Clone)]
pub struct Crypto {
    unlocked_inner: Arc<CryptoUnlockedInner>,
    inner: Arc<Mutex<CryptoInner>>,
}

impl Crypto {
    fn new_inner() -> CryptoInner {
        CryptoInner {
            dh_cache: DHCache::new(DH_CACHE_SIZE),
            flush_future: None,
            #[cfg(feature = "enable-crypto-vld0")]
            crypto_vld0: None,
            #[cfg(feature = "enable-crypto-none")]
            crypto_none: None,
        }
    }

    pub fn new(config: VeilidConfig, table_store: TableStore) -> Self {
        let out = Self {
            unlocked_inner: Arc::new(CryptoUnlockedInner {
                config,
                table_store,
            }),
            inner: Arc::new(Mutex::new(Self::new_inner())),
        };

        #[cfg(feature = "enable-crypto-vld0")]
        {
            out.inner.lock().crypto_vld0 = Some(Arc::new(vld0::CryptoSystemVLD0::new(out.clone())));
        }

        #[cfg(feature = "enable-crypto-none")]
        {
            out.inner.lock().crypto_none = Some(Arc::new(none::CryptoSystemNONE::new(out.clone())));
        }

        out
    }

    pub fn config(&self) -> VeilidConfig {
        self.unlocked_inner.config.clone()
    }

    pub async fn init(&self) -> EyreResult<()> {
        trace!("Crypto::init");
        let table_store = self.unlocked_inner.table_store.clone();
        // Init node id from config
        if let Err(e) = self
            .unlocked_inner
            .config
            .init_node_ids(self.clone(), table_store.clone())
            .await
        {
            return Err(e).wrap_err("init node id failed");
        }

        // make local copy of node id for easy access
        let mut cache_validity_key: Vec<u8> = Vec::new();
        {
            let c = self.unlocked_inner.config.get();
            for ck in VALID_CRYPTO_KINDS {
                if let Some(nid) = c.network.routing_table.node_id.get(ck) {
                    cache_validity_key.append(&mut nid.value.bytes.to_vec());
                }
            }
        };

        // load caches if they are valid for this node id
        let mut db = table_store
            .open("crypto_caches", 1)
            .await
            .wrap_err("failed to open crypto_caches")?;
        let caches_valid = match db.load(0, b"cache_validity_key").await? {
            Some(v) => v == cache_validity_key,
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
            db.store(0, b"cache_validity_key", &cache_validity_key)
                .await?;
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
        let cache_bytes = {
            let inner = self.inner.lock();
            cache_to_bytes(&inner.dh_cache)
        };

        let db = self
            .unlocked_inner
            .table_store
            .open("crypto_caches", 1)
            .await?;
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

    /// Factory method to get a specific crypto version
    pub fn get(&self, kind: CryptoKind) -> Option<CryptoSystemVersion> {
        let inner = self.inner.lock();
        match kind {
            #[cfg(feature = "enable-crypto-vld0")]
            CRYPTO_KIND_VLD0 => Some(inner.crypto_vld0.clone().unwrap()),
            #[cfg(feature = "enable-crypto-none")]
            CRYPTO_KIND_NONE => Some(inner.crypto_none.clone().unwrap()),
            _ => None,
        }
    }

    // Factory method to get the best crypto version
    pub fn best(&self) -> CryptoSystemVersion {
        self.get(best_crypto_kind()).unwrap()
    }

    /// Signature set verification
    /// Returns the set of signature cryptokinds that validate and are supported
    /// If any cryptokinds are supported and do not validate, the whole operation
    /// returns an error
    pub fn verify_signatures(
        &self,
        node_ids: &[TypedKey],
        data: &[u8],
        typed_signatures: &[TypedSignature],
    ) -> VeilidAPIResult<TypedKeyGroup> {
        let mut out = TypedKeyGroup::with_capacity(node_ids.len());
        for sig in typed_signatures {
            for nid in node_ids {
                if nid.kind == sig.kind {
                    if let Some(vcrypto) = self.get(sig.kind) {
                        vcrypto.verify(&nid.value, data, &sig.value)?;
                        out.add(*nid);
                    }
                }
            }
        }
        Ok(out)
    }

    /// Signature set generation
    /// Generates the set of signatures that are supported
    /// Any cryptokinds that are not supported are silently dropped
    pub fn generate_signatures<F, R>(
        &self,
        data: &[u8],
        typed_key_pairs: &[TypedKeyPair],
        transform: F,
    ) -> VeilidAPIResult<Vec<R>>
    where
        F: Fn(&TypedKeyPair, Signature) -> R,
    {
        let mut out = Vec::<R>::with_capacity(typed_key_pairs.len());
        for kp in typed_key_pairs {
            if let Some(vcrypto) = self.get(kp.kind) {
                let sig = vcrypto.sign(&kp.value.key, &kp.value.secret, data)?;
                out.push(transform(kp, sig))
            }
        }
        Ok(out)
    }

    /// Generate keypair
    /// Does not require startup/init
    pub fn generate_keypair(crypto_kind: CryptoKind) -> VeilidAPIResult<TypedKeyPair> {
        #[cfg(feature = "enable-crypto-vld0")]
        if crypto_kind == CRYPTO_KIND_VLD0 {
            let kp = vld0_generate_keypair();
            return Ok(TypedKeyPair::new(crypto_kind, kp));
        }
        #[cfg(feature = "enable-crypto-none")]
        if crypto_kind == CRYPTO_KIND_NONE {
            let kp = none_generate_keypair();
            return Ok(TypedKeyPair::new(crypto_kind, kp));
        }
        Err(VeilidAPIError::generic("invalid crypto kind"))
    }

    // Internal utilities

    fn cached_dh_internal<T: CryptoSystem>(
        &self,
        vcrypto: &T,
        key: &PublicKey,
        secret: &SecretKey,
    ) -> VeilidAPIResult<SharedSecret> {
        Ok(
            match self.inner.lock().dh_cache.entry(
                DHCacheKey {
                    key: *key,
                    secret: *secret,
                },
                |_k, _v| {},
            ) {
                Entry::Occupied(e) => e.get().shared_secret,
                Entry::Vacant(e) => {
                    let shared_secret = vcrypto.compute_dh(key, secret)?;
                    e.insert(DHCacheValue { shared_secret });
                    shared_secret
                }
            },
        )
    }

    pub(crate) fn validate_crypto_kind(kind: CryptoKind) -> VeilidAPIResult<()> {
        if !VALID_CRYPTO_KINDS.contains(&kind) {
            apibail_generic!("invalid crypto kind");
        }
        Ok(())
    }
}
