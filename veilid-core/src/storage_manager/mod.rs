mod record_store;
mod record_store_limits;
mod value_record;
use record_store::*;
use record_store_limits::*;
use value_record::*;

use super::*;
use crate::rpc_processor::*;

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
        RecordStoreLimits {
            subkey_cache_size: todo!(),
            max_records: None,
            max_subkey_cache_memory_mb: Some(xxx),
            max_disk_space_mb: None,
        }
    }

    fn remote_limits_from_config(config: VeilidConfig) -> RecordStoreLimits {
        RecordStoreLimits {
            subkey_cache_size: todo!(),
            max_records: Some(xxx),
            max_subkey_cache_memory_mb: Some(xxx),
            max_disk_space_mb: Some(xxx),
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

        let local_limits = Self::local_limits_from_config(config.clone());
        let remote_limits = Self::remote_limits_from_config(config.clone());
        inner.local_record_store = Some(RecordStore::new(
            self.unlocked_inner.table_store.clone(),
            "local",
            local_limits,
        ));
        inner.remote_record_store = Some(RecordStore::new(
            self.unlocked_inner.table_store.clone(),
            "remote",
            remote_limits,
        ));

        Ok(())
    }

    pub fn terminate(&self) {
        debug!("starting storage manager shutdown");

        // Release the storage manager
        *self.inner.lock() = Self::new_inner();

        debug!("finished storage manager shutdown");
    }

    async fn new_local_record(&self, key: TypedKey, record: ValueRecord) -> EyreResult<()> {
        // add value record to record store
        let mut inner = self.inner.lock();
        let Some(local_record_store) = inner.local_record_store else {
            apibail_generic!("not initialized");

        };
        local_record_store.new_record(key, record)
    }

    pub async fn create_record(
        &self,
        kind: CryptoKind,
        schema: &DHTSchema,
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
        let record = ValueRecord::new(cur_ts, Some(secret), schema, safety_selection);
        self.new_local_record(key, record)
            .await
            .map_err(VeilidAPIError::internal)?;

        Ok(key)
    }

    pub async fn open_record(
        key: TypedKey,
        secret: Option<SecretKey>,
        safety_selection: SafetySelection,
    ) -> Result<DHTRecordDescriptor, VeilidAPIError> {
        unimplemented!();
    }

    pub async fn close_record(key: TypedKey) -> Result<(), VeilidAPIError> {
        unimplemented!();
    }

    pub async fn delete_value(key: TypedKey) -> Result<(), VeilidAPIError> {
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

    pub async fn watch_value(
        &self,
        key: TypedKey,
        subkeys: &[ValueSubkeyRange],
        expiration: Timestamp,
        count: u32,
    ) -> Result<Timestamp, VeilidAPIError> {
        unimplemented!();
    }

    pub async fn cancel_watch_value(
        &self,
        key: TypedKey,
        subkeys: &[ValueSubkeyRange],
    ) -> Result<bool, VeilidAPIError> {
        unimplemented!();
    }
}
