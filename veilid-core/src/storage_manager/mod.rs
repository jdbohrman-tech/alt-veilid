mod debug;
mod get_value;
mod inspect_value;
mod outbound_watch_manager;
mod record_store;
mod set_value;
mod tasks;
mod types;
mod watch_value;

use super::*;
use outbound_watch_manager::*;
use record_store::*;
use routing_table::*;
use rpc_processor::*;

pub use record_store::{InboundWatchParameters, InboundWatchResult};

pub use types::*;

impl_veilid_log_facility!("stor");

/// The maximum size of a single subkey
pub(crate) const MAX_SUBKEY_SIZE: usize = ValueData::MAX_LEN;
/// The maximum total size of all subkeys of a record
pub(crate) const MAX_RECORD_DATA_SIZE: usize = 1_048_576;
/// Frequency to flush record stores to disk
const FLUSH_RECORD_STORES_INTERVAL_SECS: u32 = 1;
/// Frequency to save metadata to disk
const SAVE_METADATA_INTERVAL_SECS: u32 = 30;
/// Frequency to check for offline subkeys writes to send to the network
const OFFLINE_SUBKEY_WRITES_INTERVAL_SECS: u32 = 5;
/// Frequency to send ValueChanged notifications to the network
const SEND_VALUE_CHANGES_INTERVAL_SECS: u32 = 1;
/// Frequency to check for dead nodes and routes for client-side outbound watches
const CHECK_OUTBOUND_WATCHES_INTERVAL_SECS: u32 = 1;
/// Frequency to retry reconciliation of watches that are not at consensus
const RECONCILE_OUTBOUND_WATCHES_INTERVAL_SECS: u32 = 300;
/// How long before expiration to try to renew per-node watches
const RENEW_OUTBOUND_WATCHES_DURATION_SECS: u32 = 30;
/// Frequency to check for expired server-side watched records
const CHECK_WATCHED_RECORDS_INTERVAL_SECS: u32 = 1;
/// Table store table for storage manager metadata
const STORAGE_MANAGER_METADATA: &str = "storage_manager_metadata";
/// Storage manager metadata key name for offline subkey write persistence
const OFFLINE_SUBKEY_WRITES: &[u8] = b"offline_subkey_writes";
/// Outbound watch manager metadata key name for watch persistence
const OUTBOUND_WATCH_MANAGER: &[u8] = b"outbound_watch_manager";

#[derive(Debug, Clone)]
/// A single 'value changed' message to send
struct ValueChangedInfo {
    target: Target,
    key: TypedKey,
    subkeys: ValueSubkeyRangeSet,
    count: u32,
    watch_id: u64,
    value: Option<Arc<SignedValueData>>,
}

/// Locked structure for storage manager
#[derive(Default)]
struct StorageManagerInner {
    /// Records that have been 'opened' and are not yet closed
    pub opened_records: HashMap<TypedKey, OpenedRecord>,
    /// Records that have ever been 'created' or 'opened' by this node, things we care about that we must republish to keep alive
    pub local_record_store: Option<RecordStore<LocalRecordDetail>>,
    /// Records that have been pushed to this node for distribution by other nodes, that we make an effort to republish
    pub remote_record_store: Option<RecordStore<RemoteRecordDetail>>,
    /// Record subkeys that have not been pushed to the network because they were written to offline
    pub offline_subkey_writes: HashMap<TypedKey, tasks::offline_subkey_writes::OfflineSubkeyWrite>,
    /// Record subkeys that are currently being written to in the foreground
    pub active_subkey_writes: HashMap<TypedKey, ValueSubkeyRangeSet>,
    /// State management for outbound watches
    pub outbound_watch_manager: OutboundWatchManager,
    /// Storage manager metadata that is persistent, including copy of offline subkey writes
    pub metadata_db: Option<TableDB>,
    /// Background processing task (not part of attachment manager tick tree so it happens when detached too)
    pub tick_future: Option<PinBoxFutureStatic<()>>,
}

impl fmt::Debug for StorageManagerInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StorageManagerInner")
            // .field("unlocked_inner", &self.unlocked_inner)
            .field("opened_records", &self.opened_records)
            .field("local_record_store", &self.local_record_store)
            .field("remote_record_store", &self.remote_record_store)
            .field("offline_subkey_writes", &self.offline_subkey_writes)
            .field("active_subkey_writes", &self.active_subkey_writes)
            .field("outbound_watch_manager", &self.outbound_watch_manager)
            //.field("metadata_db", &self.metadata_db)
            //.field("tick_future", &self.tick_future)
            .finish()
    }
}

pub(crate) struct StorageManager {
    registry: VeilidComponentRegistry,
    inner: AsyncMutex<StorageManagerInner>,

    // Background processes
    save_metadata_task: TickTask<EyreReport>,
    flush_record_stores_task: TickTask<EyreReport>,
    offline_subkey_writes_task: TickTask<EyreReport>,
    send_value_changes_task: TickTask<EyreReport>,
    check_outbound_watches_task: TickTask<EyreReport>,
    check_inbound_watches_task: TickTask<EyreReport>,

    // Anonymous watch keys
    anonymous_watch_keys: TypedKeyPairGroup,

    // Outbound watch operation lock
    // Keeps changes to watches to one-at-a-time per record
    outbound_watch_lock_table: AsyncTagLockTable<TypedKey>,

    // Background operation processor
    // for offline subkey writes, watch changes, and any other
    // background operations the storage manager wants to perform
    background_operation_processor: DeferredStreamProcessor,
}

impl fmt::Debug for StorageManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StorageManager")
            .field("registry", &self.registry)
            .field("inner", &self.inner)
            // .field("flush_record_stores_task", &self.flush_record_stores_task)
            // .field(
            //     "offline_subkey_writes_task",
            //     &self.offline_subkey_writes_task,
            // )
            // .field("send_value_changes_task", &self.send_value_changes_task)
            // .field("check_active_watches_task", &self.check_active_watches_task)
            // .field(
            //     "check_watched_records_task",
            //     &self.check_watched_records_task,
            // )
            .field("outbound_watch_lock_table", &self.outbound_watch_lock_table)
            .field(
                "background_operation_processor",
                &self.background_operation_processor,
            )
            .field("anonymous_watch_keys", &self.anonymous_watch_keys)
            .finish()
    }
}

impl_veilid_component!(StorageManager);

impl StorageManager {
    fn new_inner() -> StorageManagerInner {
        StorageManagerInner::default()
    }

    pub fn new(registry: VeilidComponentRegistry) -> StorageManager {
        let crypto = registry.crypto();

        // Generate keys to use for anonymous watches
        let mut anonymous_watch_keys = TypedKeyPairGroup::new();
        for ck in VALID_CRYPTO_KINDS {
            let vcrypto = crypto.get(ck).unwrap();
            let kp = vcrypto.generate_keypair();
            anonymous_watch_keys.add(TypedKeyPair::new(ck, kp));
        }

        let inner = Self::new_inner();
        let this = StorageManager {
            registry,
            inner: AsyncMutex::new(inner),

            save_metadata_task: TickTask::new("save_metadata_task", SAVE_METADATA_INTERVAL_SECS),
            flush_record_stores_task: TickTask::new(
                "flush_record_stores_task",
                FLUSH_RECORD_STORES_INTERVAL_SECS,
            ),
            offline_subkey_writes_task: TickTask::new(
                "offline_subkey_writes_task",
                OFFLINE_SUBKEY_WRITES_INTERVAL_SECS,
            ),
            send_value_changes_task: TickTask::new(
                "send_value_changes_task",
                SEND_VALUE_CHANGES_INTERVAL_SECS,
            ),
            check_outbound_watches_task: TickTask::new(
                "check_active_watches_task",
                CHECK_OUTBOUND_WATCHES_INTERVAL_SECS,
            ),
            check_inbound_watches_task: TickTask::new(
                "check_watched_records_task",
                CHECK_WATCHED_RECORDS_INTERVAL_SECS,
            ),
            outbound_watch_lock_table: AsyncTagLockTable::new(),
            anonymous_watch_keys,
            background_operation_processor: DeferredStreamProcessor::new(),
        };

        this.setup_tasks();

        this
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
            public_watch_limit: c.network.dht.public_watch_limit as usize,
            member_watch_limit: c.network.dht.member_watch_limit as usize,
            max_watch_expiration: TimestampDuration::new(ms_to_us(
                c.network.dht.max_watch_expiration_ms,
            )),
            min_watch_expiration: TimestampDuration::new(ms_to_us(c.network.rpc.timeout_ms)),
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
            public_watch_limit: c.network.dht.public_watch_limit as usize,
            member_watch_limit: c.network.dht.member_watch_limit as usize,
            max_watch_expiration: TimestampDuration::new(ms_to_us(
                c.network.dht.max_watch_expiration_ms,
            )),
            min_watch_expiration: TimestampDuration::new(ms_to_us(c.network.rpc.timeout_ms)),
        }
    }

    #[instrument(level = "debug", skip_all, err)]
    async fn init_async(&self) -> EyreResult<()> {
        veilid_log!(self debug "startup storage manager");
        let table_store = self.table_store();
        let config = self.config();

        let metadata_db = table_store.open(STORAGE_MANAGER_METADATA, 1).await?;

        let local_limits = Self::local_limits_from_config(config.clone());
        let remote_limits = Self::remote_limits_from_config(config.clone());

        let local_record_store =
            RecordStore::try_create(&table_store, "local", local_limits).await?;
        let remote_record_store =
            RecordStore::try_create(&table_store, "remote", remote_limits).await?;

        {
            let mut inner = self.inner.lock().await;
            inner.metadata_db = Some(metadata_db);
            inner.local_record_store = Some(local_record_store);
            inner.remote_record_store = Some(remote_record_store);
            self.load_metadata_inner(&mut inner).await?;
        }

        // Start deferred results processors
        self.background_operation_processor.init();

        Ok(())
    }

    #[instrument(level = "trace", target = "tstore", skip_all)]
    async fn post_init_async(&self) -> EyreResult<()> {
        let mut inner = self.inner.lock().await;

        // Resolve outbound watch manager noderefs
        inner.outbound_watch_manager.prepare(self.routing_table());

        // Schedule tick
        let registry = self.registry();
        let tick_future = interval("storage manager tick", 1000, move || {
            let registry = registry.clone();
            async move {
                let this = registry.storage_manager();
                if let Err(e) = this.tick().await {
                    veilid_log!(this warn "storage manager tick failed: {}", e);
                }
            }
        });
        inner.tick_future = Some(tick_future);

        Ok(())
    }

    #[instrument(level = "trace", target = "tstore", skip_all)]
    async fn pre_terminate_async(&self) {
        // Stop the background ticker process
        {
            let mut inner = self.inner.lock().await;
            // Stop ticker
            let tick_future = inner.tick_future.take();
            if let Some(f) = tick_future {
                f.await;
            }
        }

        // Cancel all tasks associated with the tick future
        self.cancel_tasks().await;
    }

    #[instrument(level = "debug", skip_all)]
    async fn terminate_async(&self) {
        veilid_log!(self debug "starting storage manager shutdown");

        // Stop deferred result processor
        self.background_operation_processor.terminate().await;

        // Terminate and release the storage manager
        {
            let mut inner = self.inner.lock().await;

            // Final flush on record stores
            if let Some(mut local_record_store) = inner.local_record_store.take() {
                if let Err(e) = local_record_store.flush().await {
                    veilid_log!(self error "termination local record store tick failed: {}", e);
                }
            }
            if let Some(mut remote_record_store) = inner.remote_record_store.take() {
                if let Err(e) = remote_record_store.flush().await {
                    veilid_log!(self error "termination remote record store tick failed: {}", e);
                }
            }

            // Save metadata
            if let Err(e) = self.save_metadata_inner(&mut inner).await {
                veilid_log!(self error "termination metadata save failed: {}", e);
            }

            // Reset inner state
            *inner = Self::new_inner();
        }

        veilid_log!(self debug "finished storage manager shutdown");
    }

    async fn save_metadata_inner(&self, inner: &mut StorageManagerInner) -> EyreResult<()> {
        if let Some(metadata_db) = &inner.metadata_db {
            let tx = metadata_db.transact();
            tx.store_json(0, OFFLINE_SUBKEY_WRITES, &inner.offline_subkey_writes)?;
            tx.store_json(0, OUTBOUND_WATCH_MANAGER, &inner.outbound_watch_manager)?;
            tx.commit().await.wrap_err("failed to commit")?
        }
        Ok(())
    }

    async fn load_metadata_inner(&self, inner: &mut StorageManagerInner) -> EyreResult<()> {
        if let Some(metadata_db) = &inner.metadata_db {
            inner.offline_subkey_writes = match metadata_db
                .load_json(0, OFFLINE_SUBKEY_WRITES)
                .await
            {
                Ok(v) => v.unwrap_or_default(),
                Err(_) => {
                    if let Err(e) = metadata_db.delete(0, OFFLINE_SUBKEY_WRITES).await {
                        veilid_log!(self debug "offline_subkey_writes format changed, clearing: {}", e);
                    }
                    Default::default()
                }
            };
            inner.outbound_watch_manager = match metadata_db
                .load_json(0, OUTBOUND_WATCH_MANAGER)
                .await
            {
                Ok(v) => v.unwrap_or_default(),
                Err(_) => {
                    if let Err(e) = metadata_db.delete(0, OUTBOUND_WATCH_MANAGER).await {
                        veilid_log!(self debug "outbound_watch_manager format changed, clearing: {}", e);
                    }
                    Default::default()
                }
            };
        }
        Ok(())
    }

    pub(super) async fn has_offline_subkey_writes(&self) -> bool {
        !self.inner.lock().await.offline_subkey_writes.is_empty()
    }

    pub(super) fn dht_is_online(&self) -> bool {
        // Check if we have published peer info
        // Note, this is a best-effort check, subject to race conditions on the network's state
        if self
            .routing_table()
            .get_published_peer_info(RoutingDomain::PublicInternet)
            .is_none()
        {
            return false;
        }

        true
    }

    /// Get the set of nodes in our active watches
    pub async fn get_outbound_watch_nodes(&self) -> Vec<Destination> {
        let inner = self.inner.lock().await;

        let mut out = vec![];
        let mut node_set = HashSet::new();
        for v in inner.outbound_watch_manager.outbound_watches.values() {
            if let Some(current) = v.state() {
                let node_refs =
                    current.watch_node_refs(&inner.outbound_watch_manager.per_node_states);
                for node_ref in &node_refs {
                    let mut found = false;
                    for nid in node_ref.node_ids().iter() {
                        if node_set.contains(nid) {
                            found = true;
                            break;
                        }
                    }
                    if found {
                        continue;
                    }

                    node_set.insert(node_ref.best_node_id());
                    out.push(
                        Destination::direct(
                            node_ref.routing_domain_filtered(RoutingDomain::PublicInternet),
                        )
                        .with_safety(current.params().safety_selection),
                    )
                }
            }
        }

        out
    }

    /// Builds the record key for a given schema and owner
    #[instrument(level = "trace", target = "stor", skip_all)]
    pub fn get_record_key(
        &self,
        kind: CryptoKind,
        schema: DHTSchema,
        owner_key: &PublicKey,
    ) -> VeilidAPIResult<TypedKey> {
        // Get cryptosystem
        let crypto = self.crypto();
        let Some(vcrypto) = crypto.get(kind) else {
            apibail_generic!("unsupported cryptosystem");
        };

        // Validate schema
        schema.validate()?;
        let schema_data = schema.compile();

        Ok(Self::get_key(&vcrypto, owner_key, &schema_data))
    }

    /// Create a local record from scratch with a new owner key, open it, and return the opened descriptor
    pub async fn create_record(
        &self,
        kind: CryptoKind,
        schema: DHTSchema,
        owner: Option<KeyPair>,
        safety_selection: SafetySelection,
    ) -> VeilidAPIResult<DHTRecordDescriptor> {
        // Validate schema
        schema.validate()?;

        // Lock access to the record stores
        let mut inner = self.inner.lock().await;

        // Create a new owned local record from scratch
        let (key, owner) = self
            .create_new_owned_local_record_inner(&mut inner, kind, schema, owner, safety_selection)
            .await?;

        // Now that the record is made we should always succeed to open the existing record
        // The initial writer is the owner of the record
        self.open_existing_record_inner(&mut inner, key, Some(owner), safety_selection)
            .await
            .map(|r| r.unwrap())
    }

    /// Open an existing local record if it exists, and if it doesnt exist locally, try to pull it from the network and open it and return the opened descriptor
    #[instrument(level = "trace", target = "stor", skip_all)]
    pub async fn open_record(
        &self,
        key: TypedKey,
        writer: Option<KeyPair>,
        safety_selection: SafetySelection,
    ) -> VeilidAPIResult<DHTRecordDescriptor> {
        let mut inner = self.inner.lock().await;

        // See if we have a local record already or not
        if let Some(res) = self
            .open_existing_record_inner(&mut inner, key, writer, safety_selection)
            .await?
        {
            return Ok(res);
        }

        // No record yet, try to get it from the network
        if !self.dht_is_online() {
            apibail_try_again!("offline, try again later");
        };

        // Drop the mutex so we dont block during network access
        drop(inner);

        // No last descriptor, no last value
        // Use the safety selection we opened the record with
        let subkey: ValueSubkey = 0;
        let res_rx = self
            .outbound_get_value(key, subkey, safety_selection, GetResult::default())
            .await?;
        // Wait for the first result
        let Ok(result) = res_rx.recv_async().await else {
            apibail_internal!("failed to receive results");
        };
        let result = result?;

        // If we got nothing back, the key wasn't found
        if result.get_result.opt_value.is_none() && result.get_result.opt_descriptor.is_none() {
            // No result
            apibail_key_not_found!(key);
        };
        let opt_last_seq = result
            .get_result
            .opt_value
            .as_ref()
            .map(|s| s.value_data().seq());

        // Reopen inner to store value we just got
        let out = {
            let mut inner = self.inner.lock().await;

            // Check again to see if we have a local record already or not
            // because waiting for the outbound_get_value action could result in the key being opened
            // via some parallel process

            if let Some(res) = self
                .open_existing_record_inner(&mut inner, key, writer, safety_selection)
                .await?
            {
                return Ok(res);
            }

            // Open the new record
            self.open_new_record_inner(
                &mut inner,
                key,
                writer,
                subkey,
                result.get_result,
                safety_selection,
            )
            .await
        };

        if out.is_ok() {
            if let Some(last_seq) = opt_last_seq {
                self.process_deferred_outbound_get_value_result(res_rx, key, subkey, last_seq);
            }
        }
        out
    }

    /// Close an opened local record
    #[instrument(level = "trace", target = "stor", skip_all)]
    pub async fn close_record(&self, key: TypedKey) -> VeilidAPIResult<()> {
        // Attempt to close the record, returning the opened record if it wasn't already closed
        let mut inner = self.inner.lock().await;
        Self::close_record_inner(&mut inner, key)?;
        Ok(())
    }

    /// Close all opened records
    #[instrument(level = "trace", target = "stor", skip_all)]
    pub async fn close_all_records(&self) -> VeilidAPIResult<()> {
        // Attempt to close the record, returning the opened record if it wasn't already closed
        let mut inner = self.inner.lock().await;
        let keys = inner.opened_records.keys().copied().collect::<Vec<_>>();
        for key in keys {
            Self::close_record_inner(&mut inner, key)?;
        }

        Ok(())
    }

    /// Delete a local record
    #[instrument(level = "trace", target = "stor", skip_all)]
    pub async fn delete_record(&self, key: TypedKey) -> VeilidAPIResult<()> {
        // Ensure the record is closed
        let mut inner = self.inner.lock().await;
        Self::close_record_inner(&mut inner, key)?;

        // Get record from the local store
        let Some(local_record_store) = inner.local_record_store.as_mut() else {
            apibail_not_initialized!();
        };

        // Remove the record from the local store
        local_record_store.delete_record(key).await
    }

    /// Get the value of a subkey from an opened local record
    #[instrument(level = "trace", target = "stor", skip_all)]
    pub async fn get_value(
        &self,
        key: TypedKey,
        subkey: ValueSubkey,
        force_refresh: bool,
    ) -> VeilidAPIResult<Option<ValueData>> {
        let mut inner = self.inner.lock().await;
        let safety_selection = {
            let Some(opened_record) = inner.opened_records.get(&key) else {
                apibail_generic!("record not open");
            };
            opened_record.safety_selection()
        };

        // See if the requested subkey is our local record store
        let last_get_result =
            Self::handle_get_local_value_inner(&mut inner, key, subkey, true).await?;

        // Return the existing value if we have one unless we are forcing a refresh
        if !force_refresh {
            if let Some(last_get_result_value) = last_get_result.opt_value {
                return Ok(Some(last_get_result_value.value_data().clone()));
            }
        }

        // Refresh if we can
        if !self.dht_is_online() {
            // Return the existing value if we have one if we aren't online
            if let Some(last_get_result_value) = last_get_result.opt_value {
                return Ok(Some(last_get_result_value.value_data().clone()));
            }
            apibail_try_again!("offline, try again later");
        };

        // Drop the lock for network access
        drop(inner);

        // May have last descriptor / value
        // Use the safety selection we opened the record with
        let opt_last_seq = last_get_result
            .opt_value
            .as_ref()
            .map(|v| v.value_data().seq());
        let res_rx = self
            .outbound_get_value(key, subkey, safety_selection, last_get_result)
            .await?;

        // Wait for the first result
        let Ok(result) = res_rx.recv_async().await else {
            apibail_internal!("failed to receive results");
        };
        let result = result?;
        let partial = result.fanout_result.kind.is_incomplete();

        // Process the returned result
        let out = self
            .process_outbound_get_value_result(key, subkey, opt_last_seq, result)
            .await?;

        if let Some(out) = &out {
            // If there's more to process, do it in the background
            if partial {
                self.process_deferred_outbound_get_value_result(res_rx, key, subkey, out.seq());
            }
        }

        Ok(out)
    }

    /// Set the value of a subkey on an opened local record
    #[instrument(level = "trace", target = "stor", skip_all)]
    pub async fn set_value(
        &self,
        key: TypedKey,
        subkey: ValueSubkey,
        data: Vec<u8>,
        writer: Option<KeyPair>,
    ) -> VeilidAPIResult<Option<ValueData>> {
        let mut inner = self.inner.lock().await;

        // Get cryptosystem
        let crypto = self.crypto();
        let Some(vcrypto) = crypto.get(key.kind) else {
            apibail_generic!("unsupported cryptosystem");
        };

        let (safety_selection, opt_writer) = {
            let Some(opened_record) = inner.opened_records.get(&key) else {
                apibail_generic!("record not open");
            };
            (
                opened_record.safety_selection(),
                opened_record.writer().cloned(),
            )
        };

        // Use the specified writer, or if not specified, the default writer when the record was opened
        let opt_writer = writer.or(opt_writer);

        // If we don't have a writer then we can't write
        let Some(writer) = opt_writer else {
            apibail_generic!("value is not writable");
        };

        // See if the subkey we are modifying has a last known local value
        let last_get_result =
            Self::handle_get_local_value_inner(&mut inner, key, subkey, true).await?;

        // Get the descriptor and schema for the key
        let Some(descriptor) = last_get_result.opt_descriptor else {
            apibail_generic!("must have a descriptor");
        };
        let schema = descriptor.schema()?;

        // Make new subkey data
        let value_data = if let Some(last_signed_value_data) = last_get_result.opt_value {
            if last_signed_value_data.value_data().data() == data
                && last_signed_value_data.value_data().writer() == &writer.key
            {
                // Data and writer is the same, nothing is changing,
                // just return that we set it, but no network activity needs to happen
                return Ok(None);
            }
            let seq = last_signed_value_data.value_data().seq();
            ValueData::new_with_seq(seq + 1, data, writer.key)?
        } else {
            ValueData::new(data, writer.key)?
        };

        // Validate with schema
        if !schema.check_subkey_value_data(descriptor.owner(), subkey, &value_data) {
            // Validation failed, ignore this value
            apibail_generic!("failed schema validation");
        }

        // Sign the new value data with the writer
        let signed_value_data = Arc::new(SignedValueData::make_signature(
            value_data,
            descriptor.owner(),
            subkey,
            &vcrypto,
            writer.secret,
        )?);

        // Write the value locally first
        veilid_log!(self debug "Writing subkey locally: {}:{} len={}", key, subkey, signed_value_data.value_data().data().len() );
        Self::handle_set_local_value_inner(
            &mut inner,
            key,
            subkey,
            signed_value_data.clone(),
            InboundWatchUpdateMode::NoUpdate,
        )
        .await?;

        // Note that we are writing this subkey actively
        // If it appears we are already doing this, then put it to the offline queue
        let already_writing = {
            let asw = inner.active_subkey_writes.entry(key).or_default();
            if asw.contains(subkey) {
                veilid_log!(self debug "Already writing to this subkey: {}:{}", key, subkey);
                true
            } else {
                // Add to our list of active subkey writes
                asw.insert(subkey);
                false
            }
        };

        if already_writing || !self.dht_is_online() {
            veilid_log!(self debug "Writing subkey offline: {}:{} len={}", key, subkey, signed_value_data.value_data().data().len() );
            // Add to offline writes to flush
            Self::add_offline_subkey_write_inner(&mut inner, key, subkey, safety_selection);
            return Ok(None);
        };

        // Drop the lock for network access
        drop(inner);

        veilid_log!(self debug "Writing subkey to the network: {}:{} len={}", key, subkey, signed_value_data.value_data().data().len() );

        // Use the safety selection we opened the record with
        let res_rx = match self
            .outbound_set_value(
                key,
                subkey,
                safety_selection,
                signed_value_data.clone(),
                descriptor,
            )
            .await
        {
            Ok(v) => v,
            Err(e) => {
                // Failed to write, try again later
                let mut inner = self.inner.lock().await;
                Self::add_offline_subkey_write_inner(&mut inner, key, subkey, safety_selection);

                // Remove from active subkey writes
                let asw = inner.active_subkey_writes.get_mut(&key).unwrap();
                if !asw.remove(subkey) {
                    panic!("missing active subkey write: {}:{}", key, subkey);
                }
                if asw.is_empty() {
                    inner.active_subkey_writes.remove(&key);
                }
                return Err(e);
            }
        };

        let process = || async {
            // Wait for the first result
            let Ok(result) = res_rx.recv_async().await else {
                apibail_internal!("failed to receive results");
            };
            let result = result?;
            let partial = result.fanout_result.kind.is_incomplete();

            // Process the returned result
            let out = self
                .process_outbound_set_value_result(
                    key,
                    subkey,
                    signed_value_data.value_data().clone(),
                    safety_selection,
                    result,
                )
                .await?;

            // If there's more to process, do it in the background
            if partial {
                self.process_deferred_outbound_set_value_result(
                    res_rx,
                    key,
                    subkey,
                    out.clone()
                        .unwrap_or_else(|| signed_value_data.value_data().clone()),
                    safety_selection,
                );
            }

            Ok(out)
        };

        let out = process().await;

        // Remove active subkey write
        let mut inner = self.inner.lock().await;

        // Remove from active subkey writes
        let asw = inner.active_subkey_writes.get_mut(&key).unwrap();
        if !asw.remove(subkey) {
            panic!("missing active subkey write: {}:{}", key, subkey);
        }
        if asw.is_empty() {
            inner.active_subkey_writes.remove(&key);
        }

        out
    }

    /// Create, update or cancel an outbound watch to a DHT value
    #[instrument(level = "trace", target = "stor", skip_all)]
    pub async fn watch_values(
        &self,
        key: TypedKey,
        subkeys: ValueSubkeyRangeSet,
        expiration: Timestamp,
        count: u32,
    ) -> VeilidAPIResult<bool> {
        // Obtain the watch change lock
        // (may need to wait for background operations to complete on the watch)
        let watch_lock = self.outbound_watch_lock_table.lock_tag(key).await;

        self.watch_values_inner(watch_lock, subkeys, expiration, count)
            .await
    }

    #[instrument(level = "trace", target = "stor", skip_all)]
    async fn watch_values_inner(
        &self,
        watch_lock: AsyncTagLockGuard<TypedKey>,
        subkeys: ValueSubkeyRangeSet,
        expiration: Timestamp,
        count: u32,
    ) -> VeilidAPIResult<bool> {
        let key = watch_lock.tag();

        // Obtain the inner state lock
        let mut inner = self.inner.lock().await;

        // Get the safety selection and the writer we opened this record
        let (safety_selection, opt_watcher) = {
            let Some(opened_record) = inner.opened_records.get(&key) else {
                // Record must be opened already to change watch
                apibail_generic!("record not open");
            };
            (
                opened_record.safety_selection(),
                opened_record.writer().cloned(),
            )
        };

        // Rewrite subkey range if empty to full
        let subkeys = if subkeys.is_empty() {
            ValueSubkeyRangeSet::full()
        } else {
            subkeys
        };

        // Get the schema so we can truncate the watch to the number of subkeys
        let schema = if let Some(lrs) = inner.local_record_store.as_ref() {
            let Some(schema) = lrs.peek_record(key, |r| r.schema()) else {
                apibail_generic!("no local record found");
            };
            schema
        } else {
            apibail_not_initialized!();
        };
        let subkeys = schema.truncate_subkeys(&subkeys, None);

        // Calculate desired watch parameters
        let desired_params = if count == 0 {
            // Cancel
            None
        } else {
            // Get the minimum expiration timestamp we will accept
            let rpc_timeout_us = self
                .config()
                .with(|c| TimestampDuration::from(ms_to_us(c.network.rpc.timeout_ms)));
            let cur_ts = get_timestamp();
            let min_expiration_ts = Timestamp::new(cur_ts + rpc_timeout_us.as_u64());
            let expiration_ts = if expiration.as_u64() == 0 {
                expiration
            } else if expiration < min_expiration_ts {
                apibail_invalid_argument!("expiration is too soon", "expiration", expiration);
            } else {
                expiration
            };

            // Create or modify
            Some(OutboundWatchParameters {
                expiration_ts,
                count,
                subkeys,
                opt_watcher,
                safety_selection,
            })
        };

        // Modify the 'desired' state of the watch or add one if it does not exist
        inner
            .outbound_watch_manager
            .set_desired_watch(key, desired_params);

        // Drop the lock for network access
        drop(inner);

        // Process this watch's state machine operations until we are done
        loop {
            let opt_op_fut = {
                let mut inner = self.inner.lock().await;
                let Some(outbound_watch) =
                    inner.outbound_watch_manager.outbound_watches.get_mut(&key)
                else {
                    // Watch is gone
                    return Ok(false);
                };
                self.get_next_outbound_watch_operation(
                    key,
                    Some(watch_lock.clone()),
                    Timestamp::now(),
                    outbound_watch,
                )
            };
            let Some(op_fut) = opt_op_fut else {
                break;
            };
            op_fut.await;
        }

        Ok(true)
    }

    #[instrument(level = "trace", target = "stor", skip_all)]
    pub async fn cancel_watch_values(
        &self,
        key: TypedKey,
        subkeys: ValueSubkeyRangeSet,
    ) -> VeilidAPIResult<bool> {
        // Obtain the watch change lock
        // (may need to wait for background operations to complete on the watch)
        let watch_lock = self.outbound_watch_lock_table.lock_tag(key).await;

        // Calculate change to existing watch
        let (subkeys, count, expiration_ts) = {
            let inner = self.inner.lock().await;
            let Some(_opened_record) = inner.opened_records.get(&key) else {
                apibail_generic!("record not open");
            };

            // See what watch we have currently if any
            let Some(outbound_watch) = inner.outbound_watch_manager.outbound_watches.get(&key)
            else {
                // If we didn't have an active watch, then we can just return false because there's nothing to do here
                return Ok(false);
            };

            // Ensure we have a 'desired' watch state
            let Some(desired) = outbound_watch.desired() else {
                // If we didn't have a desired watch, then we're already cancelling
                let still_active = outbound_watch.state().is_some();
                return Ok(still_active);
            };

            // Rewrite subkey range if empty to full
            let subkeys = if subkeys.is_empty() {
                ValueSubkeyRangeSet::full()
            } else {
                subkeys
            };

            // Reduce the subkey range
            let new_subkeys = desired.subkeys.difference(&subkeys);

            // If no change is happening return false
            if new_subkeys == desired.subkeys {
                return Ok(false);
            }

            // If we have no subkeys left, then set the count to zero to indicate a full cancellation
            let count = if new_subkeys.is_empty() {
                0
            } else if let Some(state) = outbound_watch.state() {
                state.remaining_count()
            } else {
                desired.count
            };

            (new_subkeys, count, desired.expiration_ts)
        };

        // Update the watch. This just calls through to the above watch_values_inner() function
        // This will update the active_watch so we don't need to do that in this routine.
        self.watch_values_inner(watch_lock, subkeys, expiration_ts, count)
            .await
    }

    /// Inspect an opened DHT record for its subkey sequence numbers
    #[instrument(level = "trace", target = "stor", skip_all)]
    pub async fn inspect_record(
        &self,
        key: TypedKey,
        subkeys: ValueSubkeyRangeSet,
        scope: DHTReportScope,
    ) -> VeilidAPIResult<DHTRecordReport> {
        let subkeys = if subkeys.is_empty() {
            ValueSubkeyRangeSet::full()
        } else {
            subkeys
        };

        // Get cryptosystem
        let crypto = self.crypto();
        let Some(vcrypto) = crypto.get(key.kind) else {
            apibail_generic!("unsupported cryptosystem");
        };

        let mut inner = self.inner.lock().await;
        let safety_selection = {
            let Some(opened_record) = inner.opened_records.get(&key) else {
                apibail_generic!("record not open");
            };
            opened_record.safety_selection()
        };

        // See if the requested record is our local record store
        let mut local_inspect_result =
            Self::handle_inspect_local_value_inner(&mut inner, key, subkeys.clone(), true).await?;

        #[allow(clippy::unnecessary_cast)]
        {
            assert!(
                local_inspect_result.subkeys.len() as u64 == local_inspect_result.seqs.len() as u64,
                "mismatch between local subkeys returned and sequence number list returned"
            );
        }
        assert!(
            local_inspect_result.subkeys.is_subset(&subkeys),
            "more subkeys returned locally than requested"
        );

        // Get the offline subkeys for this record still only returning the ones we're inspecting
        // Merge in the currently offline in-flight records and the actively written records as well
        let active_subkey_writes = inner
            .active_subkey_writes
            .get(&key)
            .cloned()
            .unwrap_or_default();
        let offline_subkey_writes = inner
            .offline_subkey_writes
            .get(&key)
            .map(|o| o.subkeys.union(&o.subkeys_in_flight))
            .unwrap_or_default()
            .union(&active_subkey_writes)
            .intersect(&subkeys);

        // If this is the maximum scope we're interested in, return the report
        if matches!(scope, DHTReportScope::Local) {
            return Ok(DHTRecordReport::new(
                local_inspect_result.subkeys,
                offline_subkey_writes,
                local_inspect_result.seqs,
                vec![],
            ));
        }

        // Get rpc processor and drop mutex so we don't block while getting the value from the network
        if !self.dht_is_online() {
            apibail_try_again!("offline, try again later");
        };

        // Drop the lock for network access
        drop(inner);

        // If we're simulating a set, increase the previous sequence number we have by 1
        if matches!(scope, DHTReportScope::UpdateSet) {
            for seq in &mut local_inspect_result.seqs {
                *seq = seq.overflowing_add(1).0;
            }
        }

        // Get the inspect record report from the network
        let result = self
            .outbound_inspect_value(
                key,
                subkeys,
                safety_selection,
                if matches!(scope, DHTReportScope::SyncGet | DHTReportScope::SyncSet) {
                    InspectResult::default()
                } else {
                    local_inspect_result.clone()
                },
                matches!(scope, DHTReportScope::UpdateSet | DHTReportScope::SyncSet),
            )
            .await?;

        // Sanity check before zip
        #[allow(clippy::unnecessary_cast)]
        {
            assert_eq!(
                result.inspect_result.subkeys.len() as u64,
                result.subkey_fanout_results.len() as u64,
                "mismatch between subkeys returned and fanout results returned"
            );
        }
        if !local_inspect_result.subkeys.is_empty() && !result.inspect_result.subkeys.is_empty() {
            assert_eq!(
                result.inspect_result.subkeys.len(),
                local_inspect_result.subkeys.len(),
                "mismatch between local subkeys returned and network results returned"
            );
        }

        // Keep the list of nodes that returned a value for later reference
        let mut inner = self.inner.lock().await;
        let results_iter = result
            .inspect_result
            .subkeys
            .iter()
            .map(ValueSubkeyRangeSet::single)
            .zip(result.subkey_fanout_results.into_iter());

        Self::process_fanout_results_inner(
            &mut inner,
            &vcrypto,
            key,
            results_iter,
            false,
            self.config()
                .with(|c| c.network.dht.set_value_count as usize),
        );

        Ok(DHTRecordReport::new(
            result.inspect_result.subkeys,
            offline_subkey_writes,
            local_inspect_result.seqs,
            result.inspect_result.seqs,
        ))
    }

    // Send single value change out to the network
    #[instrument(level = "trace", target = "stor", skip(self), err)]
    async fn send_value_change(&self, vc: ValueChangedInfo) -> VeilidAPIResult<()> {
        if !self.dht_is_online() {
            apibail_try_again!("network is not available");
        };

        let rpc_processor = self.rpc_processor();

        let dest = rpc_processor
            .resolve_target_to_destination(
                vc.target,
                SafetySelection::Unsafe(Sequencing::PreferOrdered),
            )
            .await
            .map_err(VeilidAPIError::from)?;

        network_result_value_or_log!(self rpc_processor
            .rpc_call_value_changed(dest, vc.key, vc.subkeys.clone(), vc.count, vc.watch_id, vc.value.map(|v| (*v).clone()) )
            .await
            .map_err(VeilidAPIError::from)? => [format!(": dest={:?} vc={:?}", dest, vc)] {});

        Ok(())
    }

    // Send a value change up through the callback
    #[instrument(level = "trace", target = "stor", skip(self, value))]
    fn update_callback_value_change(
        &self,
        key: TypedKey,
        subkeys: ValueSubkeyRangeSet,
        count: u32,
        value: Option<ValueData>,
    ) {
        let update_callback = self.update_callback();
        update_callback(VeilidUpdate::ValueChange(Box::new(VeilidValueChange {
            key,
            subkeys,
            count,
            value,
        })));
    }

    #[instrument(level = "trace", target = "stor", skip_all)]
    fn check_fanout_set_offline(
        &self,
        key: TypedKey,
        subkey: ValueSubkey,
        fanout_result: &FanoutResult,
    ) -> bool {
        match fanout_result.kind {
            FanoutResultKind::Incomplete => false,
            FanoutResultKind::Timeout => {
                let get_consensus = self
                    .config()
                    .with(|c| c.network.dht.get_value_count as usize);
                let value_node_count = fanout_result.consensus_nodes.len();
                if value_node_count < get_consensus {
                    veilid_log!(self debug "timeout with insufficient consensus ({}<{}), adding offline subkey: {}:{}",
                        value_node_count, get_consensus,
                        key, subkey);
                    true
                } else {
                    veilid_log!(self debug "timeout with sufficient consensus ({}>={}): set_value {}:{}",
                        value_node_count, get_consensus,
                        key, subkey);
                    false
                }
            }
            FanoutResultKind::Exhausted => {
                let get_consensus = self
                    .config()
                    .with(|c| c.network.dht.get_value_count as usize);
                let value_node_count = fanout_result.consensus_nodes.len();
                if value_node_count < get_consensus {
                    veilid_log!(self debug "exhausted with insufficient consensus ({}<{}), adding offline subkey: {}:{}",
                        value_node_count, get_consensus,
                        key, subkey);
                    true
                } else {
                    veilid_log!(self debug "exhausted with sufficient consensus ({}>={}): set_value {}:{}",
                        value_node_count, get_consensus,
                        key, subkey);
                    false
                }
            }
            FanoutResultKind::Consensus => false,
        }
    }

    ////////////////////////////////////////////////////////////////////////
    #[instrument(level = "trace", target = "stor", skip_all, err)]
    async fn create_new_owned_local_record_inner(
        &self,
        inner: &mut StorageManagerInner,
        kind: CryptoKind,
        schema: DHTSchema,
        owner: Option<KeyPair>,
        safety_selection: SafetySelection,
    ) -> VeilidAPIResult<(TypedKey, KeyPair)> {
        // Get cryptosystem
        let crypto = self.crypto();
        let Some(vcrypto) = crypto.get(kind) else {
            apibail_generic!("unsupported cryptosystem");
        };

        // Get local record store
        let Some(local_record_store) = inner.local_record_store.as_mut() else {
            apibail_not_initialized!();
        };

        // Verify the dht schema does not contain the node id
        {
            let config = self.config();
            let cfg = config.get();
            if let Some(node_id) = cfg.network.routing_table.node_id.get(kind) {
                if schema.is_member(&node_id.value) {
                    apibail_invalid_argument!(
                        "node id can not be schema member",
                        "schema",
                        node_id.value
                    );
                }
            }
        }

        // Compile the dht schema
        let schema_data = schema.compile();

        // New values require a new owner key if not given
        let owner = owner.unwrap_or_else(|| vcrypto.generate_keypair());

        // Calculate dht key
        let dht_key = Self::get_key(&vcrypto, &owner.key, &schema_data);

        // Make a signed value descriptor for this dht value
        let signed_value_descriptor = Arc::new(SignedValueDescriptor::make_signature(
            owner.key,
            schema_data,
            &vcrypto,
            owner.secret,
        )?);

        // Add new local value record
        let cur_ts = Timestamp::now();
        let local_record_detail = LocalRecordDetail::new(safety_selection);
        let record =
            Record::<LocalRecordDetail>::new(cur_ts, signed_value_descriptor, local_record_detail)?;

        local_record_store.new_record(dht_key, record).await?;

        Ok((dht_key, owner))
    }

    #[instrument(level = "trace", target = "stor", skip_all, err)]
    async fn move_remote_record_to_local_inner(
        &self,
        inner: &mut StorageManagerInner,
        key: TypedKey,
        safety_selection: SafetySelection,
    ) -> VeilidAPIResult<Option<(PublicKey, DHTSchema)>> {
        // Get local record store
        let Some(local_record_store) = inner.local_record_store.as_mut() else {
            apibail_not_initialized!();
        };

        // Get remote record store
        let Some(remote_record_store) = inner.remote_record_store.as_mut() else {
            apibail_not_initialized!();
        };

        let rcb = |r: &Record<RemoteRecordDetail>| {
            // Return record details
            r.clone()
        };
        let Some(remote_record) = remote_record_store.with_record(key, rcb) else {
            // No local or remote record found, return None
            return Ok(None);
        };

        // Make local record
        let cur_ts = Timestamp::now();
        let local_record = Record::new(
            cur_ts,
            remote_record.descriptor().clone(),
            LocalRecordDetail::new(safety_selection),
        )?;
        local_record_store.new_record(key, local_record).await?;

        // Move copy subkey data from remote to local store
        for subkey in remote_record.stored_subkeys().iter() {
            let Some(get_result) = remote_record_store.get_subkey(key, subkey, false).await? else {
                // Subkey was missing
                veilid_log!(self warn "Subkey was missing: {} #{}", key, subkey);
                continue;
            };
            let Some(subkey_data) = get_result.opt_value else {
                // Subkey was missing
                veilid_log!(self warn "Subkey data was missing: {} #{}", key, subkey);
                continue;
            };
            local_record_store
                .set_subkey(key, subkey, subkey_data, InboundWatchUpdateMode::NoUpdate)
                .await?;
        }

        // Move watches
        local_record_store.move_watches(key, remote_record_store.move_watches(key, None));

        // Delete remote record from store
        remote_record_store.delete_record(key).await?;

        // Return record information as transferred to local record
        Ok(Some((*remote_record.owner(), remote_record.schema())))
    }

    #[instrument(level = "trace", target = "stor", skip_all, err)]
    pub async fn open_existing_record_inner(
        &self,
        inner: &mut StorageManagerInner,
        key: TypedKey,
        writer: Option<KeyPair>,
        safety_selection: SafetySelection,
    ) -> VeilidAPIResult<Option<DHTRecordDescriptor>> {
        // Get local record store
        let Some(local_record_store) = inner.local_record_store.as_mut() else {
            apibail_not_initialized!();
        };

        // See if we have a local record already or not
        let cb = |r: &mut Record<LocalRecordDetail>| {
            // Process local record

            // Keep the safety selection we opened the record with
            r.detail_mut().safety_selection = safety_selection;

            // Return record details
            (*r.owner(), r.schema())
        };
        let (owner, schema) = match local_record_store.with_record_mut(key, cb) {
            Some(v) => v,
            None => {
                // If we don't have a local record yet, check to see if we have a remote record
                // if so, migrate it to a local record
                let Some(v) = self
                    .move_remote_record_to_local_inner(&mut *inner, key, safety_selection)
                    .await?
                else {
                    // No remote record either
                    return Ok(None);
                };
                v
            }
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
        inner
            .opened_records
            .entry(key)
            .and_modify(|e| {
                e.set_writer(writer);
                e.set_safety_selection(safety_selection);
            })
            .or_insert_with(|| OpenedRecord::new(writer, safety_selection));

        // Make DHT Record Descriptor to return
        let descriptor = DHTRecordDescriptor::new(key, owner, owner_secret, schema);
        Ok(Some(descriptor))
    }

    #[instrument(level = "trace", target = "stor", skip_all, err)]
    pub async fn open_new_record_inner(
        &self,
        inner: &mut StorageManagerInner,
        key: TypedKey,
        writer: Option<KeyPair>,
        subkey: ValueSubkey,
        get_result: GetResult,
        safety_selection: SafetySelection,
    ) -> VeilidAPIResult<DHTRecordDescriptor> {
        // Ensure the record is closed
        if inner.opened_records.contains_key(&key) {
            panic!("new record should never be opened at this point");
        }

        // Must have descriptor
        let Some(signed_value_descriptor) = get_result.opt_descriptor else {
            // No descriptor for new record, can't store this
            apibail_generic!("no descriptor");
        };
        // Get owner
        let owner = *signed_value_descriptor.owner();

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

        // Get local record store
        let Some(local_record_store) = inner.local_record_store.as_mut() else {
            apibail_not_initialized!();
        };

        // Make and store a new record for this descriptor
        let record = Record::<LocalRecordDetail>::new(
            Timestamp::now(),
            signed_value_descriptor,
            LocalRecordDetail::new(safety_selection),
        )?;
        local_record_store.new_record(key, record).await?;

        // If we got a subkey with the getvalue, it has already been validated against the schema, so store it
        if let Some(signed_value_data) = get_result.opt_value {
            // Write subkey to local store
            local_record_store
                .set_subkey(
                    key,
                    subkey,
                    signed_value_data,
                    InboundWatchUpdateMode::NoUpdate,
                )
                .await?;
        }

        // Write open record
        inner
            .opened_records
            .insert(key, OpenedRecord::new(writer, safety_selection));

        // Make DHT Record Descriptor to return
        let descriptor = DHTRecordDescriptor::new(key, owner, owner_secret, schema);
        Ok(descriptor)
    }

    #[instrument(level = "trace", target = "stor", skip_all, err)]
    pub async fn get_value_nodes(&self, key: TypedKey) -> VeilidAPIResult<Option<Vec<NodeRef>>> {
        let inner = self.inner.lock().await;
        // Get local record store
        let Some(local_record_store) = inner.local_record_store.as_ref() else {
            apibail_not_initialized!();
        };

        // Get routing table to see if we still know about these nodes
        let routing_table = self.routing_table();

        let opt_value_nodes = local_record_store.peek_record(key, |r| {
            let d = r.detail();
            d.nodes
                .keys()
                .copied()
                .filter_map(|x| {
                    routing_table
                        .lookup_node_ref(TypedKey::new(key.kind, x))
                        .ok()
                        .flatten()
                })
                .collect()
        });

        Ok(opt_value_nodes)
    }

    #[instrument(level = "trace", target = "stor", skip_all)]
    pub(super) fn process_fanout_results_inner<
        I: IntoIterator<Item = (ValueSubkeyRangeSet, FanoutResult)>,
    >(
        inner: &mut StorageManagerInner,
        vcrypto: &CryptoSystemGuard<'_>,
        record_key: TypedKey,
        subkey_results_iter: I,
        is_set: bool,
        consensus_count: usize,
    ) {
        // Get local record store
        let local_record_store = inner.local_record_store.as_mut().unwrap();

        let cur_ts = Timestamp::now();
        local_record_store.with_record_mut(record_key, |r| {
            let d = r.detail_mut();

            for (subkeys, fanout_result) in subkey_results_iter {
                for node_id in fanout_result
                    .value_nodes
                    .iter()
                    .filter_map(|x| x.node_ids().get(record_key.kind).map(|k| k.value))
                {
                    let pnd = d.nodes.entry(node_id).or_default();
                    if is_set || pnd.last_set == Timestamp::default() {
                        pnd.last_set = cur_ts;
                    }
                    pnd.last_seen = cur_ts;
                    pnd.subkeys = pnd.subkeys.union(&subkeys);
                }
            }

            // Purge nodes down to the N most recently seen, where N is the consensus count for a set operation
            let mut nodes_ts = d
                .nodes
                .iter()
                .map(|kv| (*kv.0, kv.1.last_seen))
                .collect::<Vec<_>>();
            nodes_ts.sort_by(|a, b| {
                // Timestamp is first metric
                let res = b.1.cmp(&a.1);
                if res != cmp::Ordering::Equal {
                    return res;
                }
                // Distance is the next metric, closer nodes first
                let da = vcrypto.distance(&a.0, &record_key.value);
                let db = vcrypto.distance(&b.0, &record_key.value);
                da.cmp(&db)
            });

            for dead_node_key in nodes_ts.iter().skip(consensus_count) {
                d.nodes.remove(&dead_node_key.0);
            }
        });
    }

    fn close_record_inner(inner: &mut StorageManagerInner, key: TypedKey) -> VeilidAPIResult<()> {
        let Some(local_record_store) = inner.local_record_store.as_mut() else {
            apibail_not_initialized!();
        };
        if local_record_store.peek_record(key, |_| {}).is_none() {
            apibail_key_not_found!(key);
        }

        if inner.opened_records.remove(&key).is_some() {
            // Set the watch to cancelled if we have one
            // Will process cancellation in the background
            inner.outbound_watch_manager.set_desired_watch(key, None);
        }

        Ok(())
    }

    #[instrument(level = "trace", target = "stor", skip_all, err)]
    async fn handle_get_local_value_inner(
        inner: &mut StorageManagerInner,
        key: TypedKey,
        subkey: ValueSubkey,
        want_descriptor: bool,
    ) -> VeilidAPIResult<GetResult> {
        // See if it's in the local record store
        let Some(local_record_store) = inner.local_record_store.as_mut() else {
            apibail_not_initialized!();
        };
        if let Some(get_result) = local_record_store
            .get_subkey(key, subkey, want_descriptor)
            .await?
        {
            return Ok(get_result);
        }

        Ok(GetResult {
            opt_value: None,
            opt_descriptor: None,
        })
    }

    #[instrument(level = "trace", target = "stor", skip_all, err)]
    pub(super) async fn handle_set_local_value_inner(
        inner: &mut StorageManagerInner,
        key: TypedKey,
        subkey: ValueSubkey,
        signed_value_data: Arc<SignedValueData>,
        watch_update_mode: InboundWatchUpdateMode,
    ) -> VeilidAPIResult<()> {
        // See if it's in the local record store
        let Some(local_record_store) = inner.local_record_store.as_mut() else {
            apibail_not_initialized!();
        };

        // Write subkey to local store
        local_record_store
            .set_subkey(key, subkey, signed_value_data, watch_update_mode)
            .await?;

        Ok(())
    }

    #[instrument(level = "trace", target = "stor", skip_all, err)]
    pub(super) async fn handle_inspect_local_value_inner(
        inner: &mut StorageManagerInner,
        key: TypedKey,
        subkeys: ValueSubkeyRangeSet,
        want_descriptor: bool,
    ) -> VeilidAPIResult<InspectResult> {
        // See if it's in the local record store
        let Some(local_record_store) = inner.local_record_store.as_mut() else {
            apibail_not_initialized!();
        };
        if let Some(inspect_result) = local_record_store
            .inspect_record(key, subkeys, want_descriptor)
            .await?
        {
            return Ok(inspect_result);
        }

        Ok(InspectResult {
            subkeys: ValueSubkeyRangeSet::new(),
            seqs: vec![],
            opt_descriptor: None,
        })
    }

    #[instrument(level = "trace", target = "stor", skip_all, err)]
    pub(super) async fn handle_get_remote_value_inner(
        inner: &mut StorageManagerInner,
        key: TypedKey,
        subkey: ValueSubkey,
        want_descriptor: bool,
    ) -> VeilidAPIResult<GetResult> {
        // See if it's in the remote record store
        let Some(remote_record_store) = inner.remote_record_store.as_mut() else {
            apibail_not_initialized!();
        };
        if let Some(get_result) = remote_record_store
            .get_subkey(key, subkey, want_descriptor)
            .await?
        {
            return Ok(get_result);
        }

        Ok(GetResult {
            opt_value: None,
            opt_descriptor: None,
        })
    }

    #[instrument(level = "trace", target = "stor", skip_all, err)]
    pub(super) async fn handle_set_remote_value_inner(
        inner: &mut StorageManagerInner,
        key: TypedKey,
        subkey: ValueSubkey,
        signed_value_data: Arc<SignedValueData>,
        signed_value_descriptor: Arc<SignedValueDescriptor>,
        watch_update_mode: InboundWatchUpdateMode,
    ) -> VeilidAPIResult<()> {
        // See if it's in the remote record store
        let Some(remote_record_store) = inner.remote_record_store.as_mut() else {
            apibail_not_initialized!();
        };

        // See if we have a remote record already or not
        if remote_record_store.with_record(key, |_| {}).is_none() {
            // record didn't exist, make it
            let cur_ts = Timestamp::now();
            let remote_record_detail = RemoteRecordDetail {};
            let record = Record::<RemoteRecordDetail>::new(
                cur_ts,
                signed_value_descriptor,
                remote_record_detail,
            )?;
            remote_record_store.new_record(key, record).await?
        };

        // Write subkey to remote store
        remote_record_store
            .set_subkey(key, subkey, signed_value_data, watch_update_mode)
            .await?;

        Ok(())
    }

    #[instrument(level = "trace", target = "stor", skip_all, err)]
    pub(super) async fn handle_inspect_remote_value_inner(
        inner: &mut StorageManagerInner,
        key: TypedKey,
        subkeys: ValueSubkeyRangeSet,
        want_descriptor: bool,
    ) -> VeilidAPIResult<InspectResult> {
        // See if it's in the local record store
        let Some(remote_record_store) = inner.remote_record_store.as_mut() else {
            apibail_not_initialized!();
        };
        if let Some(inspect_result) = remote_record_store
            .inspect_record(key, subkeys, want_descriptor)
            .await?
        {
            return Ok(inspect_result);
        }

        Ok(InspectResult {
            subkeys: ValueSubkeyRangeSet::new(),
            seqs: vec![],
            opt_descriptor: None,
        })
    }

    fn get_key(
        vcrypto: &CryptoSystemGuard<'_>,
        owner_key: &PublicKey,
        schema_data: &[u8],
    ) -> TypedKey {
        let mut hash_data = Vec::<u8>::with_capacity(PUBLIC_KEY_LENGTH + 4 + schema_data.len());
        hash_data.extend_from_slice(&vcrypto.kind().0);
        hash_data.extend_from_slice(&owner_key.bytes);
        hash_data.extend_from_slice(schema_data);
        let hash = vcrypto.generate_hash(&hash_data);
        TypedKey::new(vcrypto.kind(), hash)
    }

    #[instrument(level = "trace", target = "stor", skip_all)]
    pub(super) fn add_offline_subkey_write_inner(
        inner: &mut StorageManagerInner,
        key: TypedKey,
        subkey: ValueSubkey,
        safety_selection: SafetySelection,
    ) {
        inner
            .offline_subkey_writes
            .entry(key)
            .and_modify(|x| {
                x.subkeys.insert(subkey);
            })
            .or_insert(tasks::offline_subkey_writes::OfflineSubkeyWrite {
                safety_selection,
                subkeys: ValueSubkeyRangeSet::single(subkey),
                subkeys_in_flight: ValueSubkeyRangeSet::new(),
            });
    }

    #[instrument(level = "trace", target = "stor", skip_all)]
    pub(super) fn process_deferred_results<T: Send + 'static>(
        &self,
        receiver: flume::Receiver<T>,
        handler: impl FnMut(T) -> PinBoxFutureStatic<bool> + Send + 'static,
    ) -> bool {
        self.background_operation_processor
            .add_stream(receiver.into_stream(), handler)
    }
}
