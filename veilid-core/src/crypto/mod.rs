mod blake3digest512;
mod dh_cache;
mod envelope;
mod guard;
mod receipt;
mod types;

pub mod crypto_system;
#[cfg(feature = "enable-crypto-none")]
pub mod none;
#[doc(hidden)]
pub mod tests;
#[cfg(feature = "enable-crypto-vld0")]
pub mod vld0;

pub use blake3digest512::*;

pub use crypto_system::*;
pub use envelope::*;
pub use guard::*;
pub use receipt::*;
pub use types::*;

#[cfg(feature = "enable-crypto-none")]
pub use none::*;
#[cfg(feature = "enable-crypto-vld0")]
pub use vld0::*;

use super::*;
use core::convert::TryInto;
use dh_cache::*;
use hashlink::linked_hash_map::Entry;
use hashlink::LruCache;
use std::marker::PhantomData;

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
}

impl fmt::Debug for CryptoInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CryptoInner")
            //.field("dh_cache", &self.dh_cache)
            // .field("flush_future", &self.flush_future)
            // .field("crypto_vld0", &self.crypto_vld0)
            // .field("crypto_none", &self.crypto_none)
            .finish()
    }
}

/// Crypto factory implementation
pub struct Crypto {
    registry: VeilidComponentRegistry,
    inner: Mutex<CryptoInner>,
    #[cfg(feature = "enable-crypto-vld0")]
    crypto_vld0: Arc<dyn CryptoSystem + Send + Sync>,
    #[cfg(feature = "enable-crypto-none")]
    crypto_none: Arc<dyn CryptoSystem + Send + Sync>,
}

impl_veilid_component!(Crypto);

impl fmt::Debug for Crypto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Crypto")
            //.field("registry", &self.registry)
            .field("inner", &self.inner)
            // .field("crypto_vld0", &self.crypto_vld0)
            // .field("crypto_none", &self.crypto_none)
            .finish()
    }
}

impl Crypto {
    fn new_inner() -> CryptoInner {
        CryptoInner {
            dh_cache: DHCache::new(DH_CACHE_SIZE),
            flush_future: None,
        }
    }

    pub fn new(registry: VeilidComponentRegistry) -> Self {
        Self {
            registry: registry.clone(),
            inner: Mutex::new(Self::new_inner()),
            #[cfg(feature = "enable-crypto-vld0")]
            crypto_vld0: Arc::new(vld0::CryptoSystemVLD0::new(registry.clone())),
            #[cfg(feature = "enable-crypto-none")]
            crypto_none: Arc::new(none::CryptoSystemNONE::new(registry.clone())),
        }
    }

    #[instrument(level = "trace", target = "crypto", skip_all, err)]
    async fn init_async(&self) -> EyreResult<()> {
        // Nothing to initialize at this time
        Ok(())
    }

    // Setup called by table store after it get initialized
    #[instrument(level = "trace", target = "crypto", skip_all, err)]
    pub(crate) async fn table_store_setup(&self, table_store: &TableStore) -> EyreResult<()> {
        // Init node id from config
        if let Err(e) = self.setup_node_ids(table_store).await {
            return Err(e).wrap_err("init node id failed");
        }

        // make local copy of node id for easy access
        let mut cache_validity_key: Vec<u8> = Vec::new();
        self.config().with(|c| {
            for ck in VALID_CRYPTO_KINDS {
                if let Some(nid) = c.network.routing_table.node_id.get(ck) {
                    cache_validity_key.append(&mut nid.value.bytes.to_vec());
                }
            }
        });

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
        Ok(())
    }

    #[instrument(level = "trace", target = "crypto", skip_all, err)]
    async fn post_init_async(&self) -> EyreResult<()> {
        // Schedule flushing
        let registry = self.registry();
        let flush_future = interval("crypto flush", 60000, move || {
            let crypto = registry.crypto();
            async move {
                if let Err(e) = crypto.flush().await {
                    warn!("flush failed: {}", e);
                }
            }
        });
        self.inner.lock().flush_future = Some(flush_future);

        Ok(())
    }

    pub async fn flush(&self) -> EyreResult<()> {
        let cache_bytes = {
            let inner = self.inner.lock();
            cache_to_bytes(&inner.dh_cache)
        };

        let db = self.table_store().open("crypto_caches", 1).await?;
        db.store(0, b"dh_cache", &cache_bytes).await?;
        Ok(())
    }

    async fn pre_terminate_async(&self) {
        let flush_future = self.inner.lock().flush_future.take();
        if let Some(f) = flush_future {
            f.await;
        }
        log_crypto!("starting termination flush");
        match self.flush().await {
            Ok(_) => {
                log_crypto!("finished termination flush");
            }
            Err(e) => {
                error!("failed termination flush: {}", e);
            }
        };
    }

    async fn terminate_async(&self) {
        // Nothing to terminate at this time
    }

    /// Factory method to get a specific crypto version
    pub fn get(&self, kind: CryptoKind) -> Option<CryptoSystemGuard<'_>> {
        match kind {
            #[cfg(feature = "enable-crypto-vld0")]
            CRYPTO_KIND_VLD0 => Some(CryptoSystemGuard::new(self.crypto_vld0.clone())),
            #[cfg(feature = "enable-crypto-none")]
            CRYPTO_KIND_NONE => Some(CryptoSystemGuard::new(self.crypto_none.clone())),
            _ => None,
        }
    }

    /// Factory method to get a specific crypto version for async use
    pub fn get_async(&self, kind: CryptoKind) -> Option<AsyncCryptoSystemGuard<'_>> {
        self.get(kind).map(|x| x.as_async())
    }

    // Factory method to get the best crypto version
    pub fn best(&self) -> CryptoSystemGuard<'_> {
        self.get(best_crypto_kind()).unwrap()
    }

    // Factory method to get the best crypto version for async use
    pub fn best_async(&self) -> AsyncCryptoSystemGuard<'_> {
        self.get_async(best_crypto_kind()).unwrap()
    }

    /// Signature set verification
    /// Returns Some() the set of signature cryptokinds that validate and are supported
    /// Returns None if any cryptokinds are supported and do not validate
    pub fn verify_signatures(
        &self,
        public_keys: &[TypedKey],
        data: &[u8],
        typed_signatures: &[TypedSignature],
    ) -> VeilidAPIResult<Option<TypedKeyGroup>> {
        let mut out = TypedKeyGroup::with_capacity(public_keys.len());
        for sig in typed_signatures {
            for nid in public_keys {
                if nid.kind == sig.kind {
                    if let Some(vcrypto) = self.get(sig.kind) {
                        if !vcrypto.verify(&nid.value, data, &sig.value)? {
                            return Ok(None);
                        }
                        out.add(*nid);
                    }
                }
            }
        }
        Ok(Some(out))
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

    pub(crate) fn validate_crypto_kind(kind: CryptoKind) -> VeilidAPIResult<()> {
        if !VALID_CRYPTO_KINDS.contains(&kind) {
            apibail_generic!("invalid crypto kind");
        }
        Ok(())
    }

    #[cfg(not(test))]
    async fn setup_node_id(
        &self,
        vcrypto: AsyncCryptoSystemGuard<'_>,
        table_store: &TableStore,
    ) -> VeilidAPIResult<(TypedKey, TypedSecret)> {
        let config = self.config();
        let ck = vcrypto.kind();
        let (mut node_id, mut node_id_secret) = config.with(|c| {
            (
                c.network.routing_table.node_id.get(ck),
                c.network.routing_table.node_id_secret.get(ck),
            )
        });

        // See if node id was previously stored in the table store
        let config_table = table_store.open("__veilid_config", 1).await?;

        let table_key_node_id = format!("node_id_{}", ck);
        let table_key_node_id_secret = format!("node_id_secret_{}", ck);

        if node_id.is_none() {
            log_crypto!(debug "pulling {} from storage", table_key_node_id);
            if let Ok(Some(stored_node_id)) = config_table
                .load_json::<TypedKey>(0, table_key_node_id.as_bytes())
                .await
            {
                log_crypto!(debug "{} found in storage", table_key_node_id);
                node_id = Some(stored_node_id);
            } else {
                log_crypto!(debug "{} not found in storage", table_key_node_id);
            }
        }

        // See if node id secret was previously stored in the protected store
        if node_id_secret.is_none() {
            log_crypto!(debug "pulling {} from storage", table_key_node_id_secret);
            if let Ok(Some(stored_node_id_secret)) = config_table
                .load_json::<TypedSecret>(0, table_key_node_id_secret.as_bytes())
                .await
            {
                log_crypto!(debug "{} found in storage", table_key_node_id_secret);
                node_id_secret = Some(stored_node_id_secret);
            } else {
                log_crypto!(debug "{} not found in storage", table_key_node_id_secret);
            }
        }

        // If we have a node id from storage, check it
        let (node_id, node_id_secret) =
            if let (Some(node_id), Some(node_id_secret)) = (node_id, node_id_secret) {
                // Validate node id
                if !vcrypto
                    .validate_keypair(&node_id.value, &node_id_secret.value)
                    .await
                {
                    apibail_generic!(format!(
                        "node_id_secret_{} and node_id_key_{} don't match",
                        ck, ck
                    ));
                }
                (node_id, node_id_secret)
            } else {
                // If we still don't have a valid node id, generate one
                log_crypto!(debug "generating new node_id_{}", ck);
                let kp = vcrypto.generate_keypair().await;
                (TypedKey::new(ck, kp.key), TypedSecret::new(ck, kp.secret))
            };
        info!("Node Id: {}", node_id);

        // Save the node id / secret in storage
        config_table
            .store_json(0, table_key_node_id.as_bytes(), &node_id)
            .await?;
        config_table
            .store_json(0, table_key_node_id_secret.as_bytes(), &node_id_secret)
            .await?;

        Ok((node_id, node_id_secret))
    }

    /// Get the node id from config if one is specified.
    /// Must be done -after- protected store is initialized, during table store init
    #[cfg_attr(test, allow(unused_variables))]
    async fn setup_node_ids(&self, table_store: &TableStore) -> VeilidAPIResult<()> {
        let mut out_node_id = TypedKeyGroup::new();
        let mut out_node_id_secret = TypedSecretGroup::new();

        for ck in VALID_CRYPTO_KINDS {
            let vcrypto = self
                .get_async(ck)
                .expect("Valid crypto kind is not actually valid.");

            #[cfg(test)]
            let (node_id, node_id_secret) = {
                let kp = vcrypto.generate_keypair().await;
                (TypedKey::new(ck, kp.key), TypedSecret::new(ck, kp.secret))
            };
            #[cfg(not(test))]
            let (node_id, node_id_secret) = self.setup_node_id(vcrypto, table_store).await?;

            // Save for config
            out_node_id.add(node_id);
            out_node_id_secret.add(node_id_secret);
        }

        // Commit back to config
        self.config().try_with_mut(|c| {
            c.network.routing_table.node_id = out_node_id;
            c.network.routing_table.node_id_secret = out_node_id_secret;
            Ok(())
        })?;

        Ok(())
    }
}
