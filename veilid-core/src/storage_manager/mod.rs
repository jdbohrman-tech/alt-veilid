mod keys;
mod record_store;
mod record_store_limits;
mod value_record;

use keys::*;
use record_store::*;
use record_store_limits::*;
use value_record::*;

use super::*;
use crate::rpc_processor::*;

/// The maximum size of a single subkey
const MAX_SUBKEY_SIZE: usize = ValueData::MAX_LEN;
/// The maximum total size of all subkeys of a record
const MAX_RECORD_DATA_SIZE: usize = 1_048_576;

/// Locked structure for storage manager
struct StorageManagerInner {
    /// Records that have been 'created' or 'opened' by this node
    local_record_store: Option<RecordStore>,
    /// Records that have been pushed to this node for distribution by other nodes
    remote_record_store: Option<RecordStore>,
}

struct StorageManagerUnlockedInner {
    config: VeilidConfig,
    crypto: Crypto,
    protected_store: ProtectedStore,
    table_store: TableStore,
    block_store: BlockStore,
    rpc_processor: RPCProcessor,
}

#[derive(Clone)]
pub struct StorageManager {
    unlocked_inner: Arc<StorageManagerUnlockedInner>,
    inner: Arc<Mutex<StorageManagerInner>>,
}

impl StorageManager {
    fn new_unlocked_inner(
        config: VeilidConfig,
        crypto: Crypto,
        protected_store: ProtectedStore,
        table_store: TableStore,
        block_store: BlockStore,
        rpc_processor: RPCProcessor,
    ) -> StorageManagerUnlockedInner {
        StorageManagerUnlockedInner {
            config,
            crypto,
            protected_store,
            table_store,
            block_store,
            rpc_processor,
        }
    }
    fn new_inner() -> StorageManagerInner {
        StorageManagerInner {
            local_record_store: None,
            remote_record_store: None,
        }
    }

    fn local_limits_from_config(config: VeilidConfig) -> RecordStoreLimits {
        let c = config.get();
        RecordStoreLimits {
            subkey_cache_size: c.network.dht.local_subkey_cache_size as usize,
            max_subkey_size: MAX_SUBKEY_SIZE,
            max_record_total_size: MAX_RECORD_DATA_SIZE,
            max_records: None,
            max_subkey_cache_memory_mb: Some(
                c.network.dht.local_max_subkey_cache_memory_mb as usize,
            ),
            max_storage_space_mb: None,
        }
    }

    fn remote_limits_from_config(config: VeilidConfig) -> RecordStoreLimits {
        let c = config.get();
        RecordStoreLimits {
            subkey_cache_size: c.network.dht.remote_subkey_cache_size as usize,
            max_subkey_size: MAX_SUBKEY_SIZE,
            max_record_total_size: MAX_RECORD_DATA_SIZE,
            max_records: Some(c.network.dht.remote_max_records as usize),
            max_subkey_cache_memory_mb: Some(
                c.network.dht.remote_max_subkey_cache_memory_mb as usize,
            ),
            max_storage_space_mb: Some(c.network.dht.remote_max_storage_space_mb as usize),
        }
    }

    pub fn new(
        config: VeilidConfig,
        crypto: Crypto,
        protected_store: ProtectedStore,
        table_store: TableStore,
        block_store: BlockStore,
        rpc_processor: RPCProcessor,
    ) -> StorageManager {
        StorageManager {
            unlocked_inner: Arc::new(Self::new_unlocked_inner(
                config,
                crypto,
                protected_store,
                table_store,
                block_store,
                rpc_processor,
            )),
            inner: Arc::new(Mutex::new(Self::new_inner())),
        }
    }

    #[instrument(level = "debug", skip_all, err)]
    pub async fn init(&self) -> EyreResult<()> {
        debug!("startup storage manager");
        let mut inner = self.inner.lock();

        let local_limits = Self::local_limits_from_config(self.unlocked_inner.config.clone());
        let remote_limits = Self::remote_limits_from_config(self.unlocked_inner.config.clone());

        let mut local_record_store = RecordStore::new(
            self.unlocked_inner.table_store.clone(),
            "local",
            local_limits,
        );
        local_record_store.init().await?;

        let mut remote_record_store = RecordStore::new(
            self.unlocked_inner.table_store.clone(),
            "remote",
            remote_limits,
        );
        remote_record_store.init().await?;

        inner.local_record_store = Some(local_record_store);
        inner.remote_record_store = Some(remote_record_store);

        Ok(())
    }

    pub async fn terminate(&self) {
        debug!("starting storage manager shutdown");

        // Release the storage manager
        *self.inner.lock() = Self::new_inner();

        debug!("finished storage manager shutdown");
    }

    async fn new_local_record(&self, key: TypedKey, record: Record) -> Result<(), VeilidAPIError> {
        // add value record to record store
        let mut inner = self.inner.lock();
        let Some(local_record_store) = inner.local_record_store.as_mut() else {
            apibail_generic!("not initialized");
        };
        local_record_store.new_record(key, record).await
    }

    pub async fn create_record(
        &self,
        kind: CryptoKind,
        schema: DHTSchema,
        safety_selection: SafetySelection,
    ) -> Result<TypedKey, VeilidAPIError> {
        // Get cryptosystem
        let Some(vcrypto) = self.unlocked_inner.crypto.get(kind) else {
            apibail_generic!("unsupported cryptosystem");
        };

        // New values require a new owner key
        let keypair = vcrypto.generate_keypair();
        let key = TypedKey::new(kind, keypair.key);
        let secret = keypair.secret;

        // Add new local value record
        let cur_ts = get_aligned_timestamp();
        let record = Record::new(cur_ts, Some(secret), schema, safety_selection);
        self.new_local_record(key, record)
            .await
            .map_err(VeilidAPIError::internal)?;

        Ok(key)
    }

    pub async fn open_record(
        &self,
        key: TypedKey,
        secret: Option<SecretKey>,
        safety_selection: SafetySelection,
    ) -> Result<DHTRecordDescriptor, VeilidAPIError> {
        unimplemented!();
    }

    pub async fn close_record(&self, key: TypedKey) -> Result<(), VeilidAPIError> {
        unimplemented!();
    }

    pub async fn delete_record(&self, key: TypedKey) -> Result<(), VeilidAPIError> {
        unimplemented!();
    }

    pub async fn get_value(
        &self,
        key: TypedKey,
        subkey: ValueSubkey,
        force_refresh: bool,
    ) -> Result<Option<ValueData>, VeilidAPIError> {
        unimplemented!();
    }

    pub async fn set_value(
        &self,
        key: TypedKey,
        subkey: ValueSubkey,
        data: Vec<u8>,
    ) -> Result<Option<ValueData>, VeilidAPIError> {
        unimplemented!();
    }

    pub async fn watch_values(
        &self,
        key: TypedKey,
        subkeys: &[ValueSubkeyRange],
        expiration: Timestamp,
        count: u32,
    ) -> Result<Timestamp, VeilidAPIError> {
        unimplemented!();
    }

    pub async fn cancel_watch_values(
        &self,
        key: TypedKey,
        subkeys: &[ValueSubkeyRange],
    ) -> Result<bool, VeilidAPIError> {
        unimplemented!();
    }
}
