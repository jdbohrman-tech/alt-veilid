mod do_get_value;
mod keys;
mod record_store;
mod record_store_limits;
mod storage_manager_inner;
mod tasks;
mod types;

use keys::*;
use record_store::*;
use record_store_limits::*;
use storage_manager_inner::*;

pub use types::*;

use super::*;
use crate::rpc_processor::*;

/// The maximum size of a single subkey
const MAX_SUBKEY_SIZE: usize = ValueData::MAX_LEN;
/// The maximum total size of all subkeys of a record
const MAX_RECORD_DATA_SIZE: usize = 1_048_576;
/// Frequency to flush record stores to disk
const FLUSH_RECORD_STORES_INTERVAL_SECS: u32 = 1;

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
    fn new_inner(unlocked_inner: Arc<StorageManagerUnlockedInner>) -> StorageManagerInner {
        StorageManagerInner::new(unlocked_inner)
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
        let unlocked_inner = Arc::new(Self::new_unlocked_inner(
            config,
            crypto,
            protected_store,
            table_store,
            block_store,
        ));
        let this = StorageManager {
            unlocked_inner: unlocked_inner.clone(),
            inner: Arc::new(AsyncMutex::new(Self::new_inner(unlocked_inner))),
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
        *inner = Self::new_inner(self.unlocked_inner.clone());

        debug!("finished storage manager shutdown");
    }

    pub async fn set_rpc_processor(&self, opt_rpc_processor: Option<RPCProcessor>) {
        let mut inner = self.inner.lock().await;
        inner.rpc_processor = opt_rpc_processor
    }

    async fn lock(&self) -> Result<AsyncMutexGuardArc<StorageManagerInner>, VeilidAPIError> {
        let inner = asyncmutex_lock_arc!(&self.inner);
        if !inner.initialized {
            apibail_not_initialized!();
        }
        Ok(inner)
    }

    /// Create a local record from scratch with a new owner key, open it, and return the opened descriptor
    pub async fn create_record(
        &self,
        kind: CryptoKind,
        schema: DHTSchema,
        safety_selection: SafetySelection,
    ) -> Result<DHTRecordDescriptor, VeilidAPIError> {
        let mut inner = self.lock().await?;

        // Create a new owned local record from scratch
        let (key, owner) = inner
            .create_new_owned_local_record(kind, schema, safety_selection)
            .await?;

        // Now that the record is made we should always succeed to open the existing record
        // The initial writer is the owner of the record
        inner
            .open_existing_record(key, Some(owner), safety_selection)
            .map(|r| r.unwrap())
    }

    /// Open an existing local record if it exists,
    /// and if it doesnt exist locally, try to pull it from the network and
    /// open it and return the opened descriptor
    pub async fn open_record(
        &self,
        key: TypedKey,
        writer: Option<KeyPair>,
        safety_selection: SafetySelection,
    ) -> Result<DHTRecordDescriptor, VeilidAPIError> {
        let mut inner = self.lock().await?;

        // See if we have a local record already or not
        if let Some(res) = inner.open_existing_record(key, writer, safety_selection)? {
            return Ok(res);
        }

        // No record yet, try to get it from the network

        // Get rpc processor and drop mutex so we don't block while getting the value from the network
        let Some(rpc_processor) = inner.rpc_processor.clone() else {
            // Offline, try again later
            apibail_try_again!();
        };

        // Drop the mutex so we dont block during network access
        drop(inner);

        // No last descriptor, no last value
        // Use the safety selection we opened the record with
        let subkey: ValueSubkey = 0;
        let subkey_result = self
            .do_get_value(
                rpc_processor,
                key,
                subkey,
                safety_selection,
                SubkeyResult::default(),
            )
            .await?;

        // If we got nothing back, the key wasn't found
        if subkey_result.value.is_none() && subkey_result.descriptor.is_none() {
            // No result
            apibail_key_not_found!(key);
        };

        // Reopen inner to store value we just got
        let mut inner = self.lock().await?;

        // Open the new record
        inner
            .open_new_record(key, writer, subkey, subkey_result, safety_selection)
            .await
    }

    /// Close an opened local record
    pub async fn close_record(&self, key: TypedKey) -> Result<(), VeilidAPIError> {
        let mut inner = self.lock().await?;
        inner.close_record(key)
    }

    /// Delete a local record
    pub async fn delete_record(&self, key: TypedKey) -> Result<(), VeilidAPIError> {
        let mut inner = self.lock().await?;

        // Ensure the record is closed
        if inner.opened_records.contains_key(&key) {
            inner.close_record(key)?;
        }

        let Some(local_record_store) = inner.local_record_store.as_mut() else {
            apibail_not_initialized!();
        };

        // Remove the record from the local store
        local_record_store.delete_record(key).await
    }

    /// Get the value of a subkey from an opened local record
    /// may refresh the record, and will if it is forced to or the subkey is not available locally yet
    pub async fn get_value(
        &self,
        key: TypedKey,
        subkey: ValueSubkey,
        force_refresh: bool,
    ) -> Result<Option<ValueData>, VeilidAPIError> {
        let mut inner = self.lock().await?;
        let Some(opened_record) = inner.opened_records.remove(&key) else {
            apibail_generic!("record not open");
        };

        // See if the requested subkey is our local record store
        let last_subkey_result = inner.handle_get_local_value(key, subkey, true)?;

        // Return the existing value if we have one unless we are forcing a refresh
        if !force_refresh {
            if let Some(last_subkey_result_value) = last_subkey_result.value {
                return Ok(Some(last_subkey_result_value.into_value_data()));
            }
        }

        // Refresh if we can

        // Get rpc processor and drop mutex so we don't block while getting the value from the network
        let Some(rpc_processor) = inner.rpc_processor.clone() else {
            // Offline, try again later
            apibail_try_again!();
        };

        // Drop the lock for network access
        drop(inner);

        // May have last descriptor / value
        // Use the safety selection we opened the record with
        let opt_last_seq = last_subkey_result
            .value
            .as_ref()
            .map(|v| v.value_data().seq());
        let subkey_result = self
            .do_get_value(
                rpc_processor,
                key,
                subkey,
                opened_record.safety_selection(),
                last_subkey_result,
            )
            .await?;

        // See if we got a value back
        let Some(subkey_result_value) = subkey_result.value else {
            // If we got nothing back then we also had nothing beforehand, return nothing
            return Ok(None);
        };

        // If we got a new value back then write it to the opened record
        if Some(subkey_result_value.value_data().seq()) != opt_last_seq {
            let mut inner = self.lock().await?;
            inner
                .handle_set_local_value(key, subkey, subkey_result_value.clone())
                .await?;
        }
        Ok(Some(subkey_result_value.into_value_data()))
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
