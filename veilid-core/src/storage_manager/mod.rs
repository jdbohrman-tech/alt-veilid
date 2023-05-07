mod do_get_value;
mod keys;
mod record_store;
mod record_store_limits;
mod tasks;
mod types;

use keys::*;
use record_store::*;
use record_store_limits::*;

pub use types::*;

use super::*;
use crate::rpc_processor::*;

/// The maximum size of a single subkey
const MAX_SUBKEY_SIZE: usize = ValueData::MAX_LEN;
/// The maximum total size of all subkeys of a record
const MAX_RECORD_DATA_SIZE: usize = 1_048_576;
/// Frequency to flush record stores to disk
const FLUSH_RECORD_STORES_INTERVAL_SECS: u32 = 1;

/// Locked structure for storage manager
struct StorageManagerInner {
    /// If we are started up
    initialized: bool,
    /// Records that have been 'opened' and are not yet closed
    opened_records: HashMap<TypedKey, OpenedRecord>,
    /// Records that have ever been 'created' or 'opened' by this node, things we care about that we must republish to keep alive
    local_record_store: Option<RecordStore<LocalRecordDetail>>,
    /// Records that have been pushed to this node for distribution by other nodes, that we make an effort to republish
    remote_record_store: Option<RecordStore<RemoteRecordDetail>>,
    /// RPC processor if it is available
    rpc_processor: Option<RPCProcessor>,
    /// Background processing task (not part of attachment manager tick tree so it happens when detached too)
    tick_future: Option<SendPinBoxFuture<()>>,
}

struct StorageManagerUnlockedInner {
    config: VeilidConfig,
    crypto: Crypto,
    protected_store: ProtectedStore,
    table_store: TableStore,
    block_store: BlockStore,

    // Background processes
    flush_record_stores_task: TickTask<EyreReport>,
}

#[derive(Clone)]
pub struct StorageManager {
    unlocked_inner: Arc<StorageManagerUnlockedInner>,
    inner: Arc<AsyncMutex<StorageManagerInner>>,
}

impl StorageManager {
    fn new_unlocked_inner(
        config: VeilidConfig,
        crypto: Crypto,
        protected_store: ProtectedStore,
        table_store: TableStore,
        block_store: BlockStore,
    ) -> StorageManagerUnlockedInner {
        StorageManagerUnlockedInner {
            config,
            crypto,
            protected_store,
            table_store,
            block_store,
            flush_record_stores_task: TickTask::new(FLUSH_RECORD_STORES_INTERVAL_SECS),
        }
    }
    fn new_inner() -> StorageManagerInner {
        StorageManagerInner {
            initialized: false,
            opened_records: HashMap::new(),
            local_record_store: None,
            remote_record_store: None,
            rpc_processor: None,
            tick_future: None,
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
    ) -> StorageManager {
        let this = StorageManager {
            unlocked_inner: Arc::new(Self::new_unlocked_inner(
                config,
                crypto,
                protected_store,
                table_store,
                block_store,
            )),
            inner: Arc::new(AsyncMutex::new(Self::new_inner())),
        };

        this.setup_tasks();

        this
    }

    #[instrument(level = "debug", skip_all, err)]
    pub async fn init(&self) -> EyreResult<()> {
        debug!("startup storage manager");
        let mut inner = self.inner.lock().await;

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

        // Schedule tick
        let this = self.clone();
        let tick_future = interval(1000, move || {
            let this = this.clone();
            async move {
                if let Err(e) = this.tick().await {
                    warn!("storage manager tick failed: {}", e);
                }
            }
        });
        inner.tick_future = Some(tick_future);

        inner.initialized = true;

        Ok(())
    }

    pub async fn terminate(&self) {
        debug!("starting storage manager shutdown");

        let mut inner = self.inner.lock().await;

        // Stop ticker
        let tick_future = inner.tick_future.take();
        if let Some(f) = tick_future {
            f.await;
        }

        // Cancel all tasks
        self.cancel_tasks().await;

        // Release the storage manager
        *inner = Self::new_inner();

        debug!("finished storage manager shutdown");
    }

    pub async fn set_rpc_processor(&self, opt_rpc_processor: Option<RPCProcessor>) {
        let mut inner = self.inner.lock().await;
        inner.rpc_processor = opt_rpc_processor
    }

    /// # DHT Key = Hash(ownerKeyKind) of: [ ownerKeyValue, schema ]
    fn get_key<D>(vcrypto: CryptoSystemVersion, record: &Record<D>) -> TypedKey
    where
        D: Clone + RkyvArchive + RkyvSerialize<RkyvSerializer>,
        for<'t> <D as RkyvArchive>::Archived: CheckBytes<RkyvDefaultValidator<'t>>,
        <D as RkyvArchive>::Archived: RkyvDeserialize<D, SharedDeserializeMap>,
    {
        let compiled = record.descriptor().schema_data();
        let mut hash_data = Vec::<u8>::with_capacity(PUBLIC_KEY_LENGTH + 4 + compiled.len());
        hash_data.extend_from_slice(&vcrypto.kind().0);
        hash_data.extend_from_slice(&record.owner().bytes);
        hash_data.extend_from_slice(compiled);
        let hash = vcrypto.generate_hash(&hash_data);
        TypedKey::new(vcrypto.kind(), hash)
    }

    async fn lock(&self) -> Result<AsyncMutexGuardArc<StorageManagerInner>, VeilidAPIError> {
        let inner = asyncmutex_lock_arc!(&self.inner);
        if !inner.initialized {
            apibail_not_initialized!();
        }
        Ok(inner)
    }

    pub async fn create_record(
        &self,
        kind: CryptoKind,
        schema: DHTSchema,
        safety_selection: SafetySelection,
    ) -> Result<DHTRecordDescriptor, VeilidAPIError> {
        let mut inner = self.lock().await?;

        // Get cryptosystem
        let Some(vcrypto) = self.unlocked_inner.crypto.get(kind) else {
            apibail_generic!("unsupported cryptosystem");
        };

        // Get local record store
        let Some(local_record_store) = inner.local_record_store.as_mut() else {
            apibail_not_initialized!();
        };

        // Compile the dht schema
        let schema_data = schema.compile();

        // New values require a new owner key
        let owner = vcrypto.generate_keypair();

        // Make a signed value descriptor for this dht value
        let signed_value_descriptor = SignedValueDescriptor::make_signature(
            owner.key,
            schema_data,
            vcrypto.clone(),
            owner.secret,
        )?;

        // Add new local value record
        let cur_ts = get_aligned_timestamp();
        let local_record_detail = LocalRecordDetail { safety_selection };
        let record =
            Record::<LocalRecordDetail>::new(cur_ts, signed_value_descriptor, local_record_detail)?;

        let dht_key = Self::get_key(vcrypto.clone(), &record);
        local_record_store.new_record(dht_key, record).await?;

        // Open the record
        self.open_record_inner(inner, dht_key, Some(owner), safety_selection)
            .await
    }

    fn open_record_inner_check_existing(
        &self,
        mut inner: AsyncMutexGuardArc<StorageManagerInner>,
        key: TypedKey,
        writer: Option<KeyPair>,
        safety_selection: SafetySelection,
    ) -> Option<Result<DHTRecordDescriptor, VeilidAPIError>> {
        // Get local record store
        let Some(local_record_store) = inner.local_record_store.as_mut() else {
            return Some(Err(VeilidAPIError::not_initialized()));
        };

        // See if we have a local record already or not
        let cb = |r: &mut Record<LocalRecordDetail>| {
            // Process local record

            // Keep the safety selection we opened the record with
            r.detail_mut().safety_selection = safety_selection;

            // Return record details
            (r.owner().clone(), r.schema())
        };
        let Some((owner, schema)) = local_record_store.with_record_mut(key, cb) else {
            return None;
        };
        // Had local record

        // If the writer we chose is also the owner, we have the owner secret
        // Otherwise this is just another subkey writer
        let owner_secret = if let Some(writer) = writer {
            if writer.key == owner {
                Some(writer.secret)
            } else {
                None
            }
        } else {
            None
        };

        // Write open record
        inner.opened_records.insert(key, OpenedRecord::new(writer));

        // Make DHT Record Descriptor to return
        let descriptor = DHTRecordDescriptor::new(key, owner, owner_secret, schema);
        Some(Ok(descriptor))
    }

    async fn open_record_inner(
        &self,
        inner: AsyncMutexGuardArc<StorageManagerInner>,
        key: TypedKey,
        writer: Option<KeyPair>,
        safety_selection: SafetySelection,
    ) -> Result<DHTRecordDescriptor, VeilidAPIError> {
        // Ensure the record is closed
        if inner.opened_records.contains_key(&key) {
            apibail_generic!("record is already open and should be closed first");
        }

        // See if we have a local record already or not
        if let Some(res) =
            self.open_record_inner_check_existing(inner, key, writer, safety_selection)
        {
            return res;
        }

        // No record yet, try to get it from the network

        // Get rpc processor and drop mutex so we don't block while getting the value from the network
        let rpc_processor = {
            let inner = self.lock().await?;
            let Some(rpc_processor) = inner.rpc_processor.clone() else {
                // Offline, try again later
                apibail_try_again!();
            };
            rpc_processor
        };

        // No last descriptor, no last value
        let subkey: ValueSubkey = 0;
        let result = self
            .do_get_value(rpc_processor, key, subkey, None, None, safety_selection)
            .await?;

        // If we got nothing back, the key wasn't found
        if result.value.is_none() && result.descriptor.is_none() {
            // No result
            apibail_key_not_found!(key);
        };

        // Must have descriptor
        let Some(signed_value_descriptor) = result.descriptor else {
            // No descriptor for new record, can't store this
            apibail_generic!("no descriptor");
        };

        let owner = signed_value_descriptor.owner().clone();
        // If the writer we chose is also the owner, we have the owner secret
        // Otherwise this is just another subkey writer
        let owner_secret = if let Some(writer) = writer {
            if writer.key == owner {
                Some(writer.secret)
            } else {
                None
            }
        } else {
            None
        };
        let schema = signed_value_descriptor.schema()?;

        // Reopen inner to store value we just got
        let mut inner = self.lock().await?;

        // Get local record store
        let Some(local_record_store) = inner.local_record_store.as_mut() else {
            apibail_not_initialized!();
        };

        // Make and store a new record for this descriptor
        let record = Record::<LocalRecordDetail>::new(
            get_aligned_timestamp(),
            signed_value_descriptor,
            LocalRecordDetail { safety_selection },
        )?;
        local_record_store.new_record(key, record).await?;

        // If we got a subkey with the getvalue, it has already been validated against the schema, so store it
        if let Some(signed_value_data) = result.value {
            // Write subkey to local store
            local_record_store
                .set_subkey(key, subkey, signed_value_data)
                .await?;
        }

        // Write open record
        inner.opened_records.insert(key, OpenedRecord::new(writer));

        // Make DHT Record Descriptor to return
        let descriptor = DHTRecordDescriptor::new(key, owner, owner_secret, schema);
        Ok(descriptor)
    }

    pub async fn open_record(
        &self,
        key: TypedKey,
        writer: Option<KeyPair>,
        safety_selection: SafetySelection,
    ) -> Result<DHTRecordDescriptor, VeilidAPIError> {
        let inner = self.lock().await?;
        self.open_record_inner(inner, key, writer, safety_selection)
            .await
    }

    async fn close_record_inner(
        &self,
        inner: &mut AsyncMutexGuardArc<StorageManagerInner>,
        key: TypedKey,
    ) -> Result<(), VeilidAPIError> {
        let Some(_opened_record) = inner.opened_records.remove(&key) else {
            apibail_generic!("record not open");
        };
        Ok(())
    }

    pub async fn close_record(&self, key: TypedKey) -> Result<(), VeilidAPIError> {
        let mut inner = self.lock().await?;
        self.close_record_inner(&mut inner, key).await
    }

    pub async fn delete_record(&self, key: TypedKey) -> Result<(), VeilidAPIError> {
        let mut inner = self.lock().await?;

        // Ensure the record is closed
        if inner.opened_records.contains_key(&key) {
            self.close_record_inner(&mut inner, key).await?;
        }

        let Some(local_record_store) = inner.local_record_store.as_mut() else {
            apibail_not_initialized!();
        };

        // Remove the record from the local store
        local_record_store.delete_record(key).await
    }

    pub async fn get_value(
        &self,
        key: TypedKey,
        subkey: ValueSubkey,
        force_refresh: bool,
    ) -> Result<Option<ValueData>, VeilidAPIError> {
        let inner = self.lock().await?;
        unimplemented!();
    }

    pub async fn set_value(
        &self,
        key: TypedKey,
        subkey: ValueSubkey,
        data: Vec<u8>,
    ) -> Result<Option<ValueData>, VeilidAPIError> {
        let inner = self.lock().await?;
        unimplemented!();
    }

    pub async fn watch_values(
        &self,
        key: TypedKey,
        subkeys: &[ValueSubkeyRange],
        expiration: Timestamp,
        count: u32,
    ) -> Result<Timestamp, VeilidAPIError> {
        let inner = self.lock().await?;
        unimplemented!();
    }

    pub async fn cancel_watch_values(
        &self,
        key: TypedKey,
        subkeys: &[ValueSubkeyRange],
    ) -> Result<bool, VeilidAPIError> {
        let inner = self.lock().await?;
        unimplemented!();
    }
}
