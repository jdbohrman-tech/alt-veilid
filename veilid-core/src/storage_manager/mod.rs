mod debug;
mod get_value;
mod keys;
mod limited_size;
mod record_store;
mod record_store_limits;
mod set_value;
mod storage_manager_inner;
mod tasks;
mod types;
mod watch_value;

use keys::*;
use limited_size::*;
use record_store::*;
use record_store_limits::*;
use storage_manager_inner::*;

pub use types::*;

use super::*;
use network_manager::*;
use routing_table::*;
use rpc_processor::*;

/// The maximum size of a single subkey
const MAX_SUBKEY_SIZE: usize = ValueData::MAX_LEN;
/// The maximum total size of all subkeys of a record
const MAX_RECORD_DATA_SIZE: usize = 1_048_576;
/// Frequency to flush record stores to disk
const FLUSH_RECORD_STORES_INTERVAL_SECS: u32 = 1;
/// Frequency to check for offline subkeys writes to send to the network
const OFFLINE_SUBKEY_WRITES_INTERVAL_SECS: u32 = 1;
/// Frequency to send ValueChanged notifications to the network
const SEND_VALUE_CHANGES_INTERVAL_SECS: u32 = 1;
/// Frequence to check for dead nodes and routes for active watches
const CHECK_ACTIVE_WATCHES_INTERVAL_SECS: u32 = 1;

#[derive(Debug, Clone)]
/// A single 'value changed' message to send
struct ValueChangedInfo {
    target: Target,
    key: TypedKey,
    subkeys: ValueSubkeyRangeSet,
    count: u32,
    value: Arc<SignedValueData>,
}

struct StorageManagerUnlockedInner {
    config: VeilidConfig,
    crypto: Crypto,
    table_store: TableStore,
    #[cfg(feature = "unstable-blockstore")]
    block_store: BlockStore,

    // Background processes
    flush_record_stores_task: TickTask<EyreReport>,
    offline_subkey_writes_task: TickTask<EyreReport>,
    send_value_changes_task: TickTask<EyreReport>,
    check_active_watches_task: TickTask<EyreReport>,

    // Anonymous watch keys
    anonymous_watch_keys: TypedKeyPairGroup,
}

#[derive(Clone)]
pub(crate) struct StorageManager {
    unlocked_inner: Arc<StorageManagerUnlockedInner>,
    inner: Arc<AsyncMutex<StorageManagerInner>>,
}

impl StorageManager {
    fn new_unlocked_inner(
        config: VeilidConfig,
        crypto: Crypto,
        table_store: TableStore,
        #[cfg(feature = "unstable-blockstore")] block_store: BlockStore,
    ) -> StorageManagerUnlockedInner {
        // Generate keys to use for anonymous watches
        let mut anonymous_watch_keys = TypedKeyPairGroup::new();
        for ck in VALID_CRYPTO_KINDS {
            let vcrypto = crypto.get(ck).unwrap();
            let kp = vcrypto.generate_keypair();
            anonymous_watch_keys.add(TypedKeyPair::new(ck, kp));
        }

        StorageManagerUnlockedInner {
            config,
            crypto,
            table_store,
            #[cfg(feature = "unstable-blockstore")]
            block_store,
            flush_record_stores_task: TickTask::new(FLUSH_RECORD_STORES_INTERVAL_SECS),
            offline_subkey_writes_task: TickTask::new(OFFLINE_SUBKEY_WRITES_INTERVAL_SECS),
            send_value_changes_task: TickTask::new(SEND_VALUE_CHANGES_INTERVAL_SECS),
            check_active_watches_task: TickTask::new(CHECK_ACTIVE_WATCHES_INTERVAL_SECS),

            anonymous_watch_keys,
        }
    }
    fn new_inner(unlocked_inner: Arc<StorageManagerUnlockedInner>) -> StorageManagerInner {
        StorageManagerInner::new(unlocked_inner)
    }

    pub fn new(
        config: VeilidConfig,
        crypto: Crypto,
        table_store: TableStore,
        #[cfg(feature = "unstable-blockstore")] block_store: BlockStore,
    ) -> StorageManager {
        let unlocked_inner = Arc::new(Self::new_unlocked_inner(
            config,
            crypto,
            table_store,
            #[cfg(feature = "unstable-blockstore")]
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
    pub async fn init(&self, update_callback: UpdateCallback) -> EyreResult<()> {
        debug!("startup storage manager");

        let mut inner = self.inner.lock().await;
        inner.init(self.clone(), update_callback).await?;

        Ok(())
    }

    #[instrument(level = "debug", skip_all)]
    pub async fn terminate(&self) {
        debug!("starting storage manager shutdown");

        let mut inner = self.inner.lock().await;
        inner.terminate().await;

        // Cancel all tasks
        self.cancel_tasks().await;

        // Release the storage manager
        *inner = Self::new_inner(self.unlocked_inner.clone());

        debug!("finished storage manager shutdown");
    }

    pub async fn set_rpc_processor(&self, opt_rpc_processor: Option<RPCProcessor>) {
        let mut inner = self.inner.lock().await;
        inner.opt_rpc_processor = opt_rpc_processor
    }

    pub async fn set_routing_table(&self, opt_routing_table: Option<RoutingTable>) {
        let mut inner = self.inner.lock().await;
        inner.opt_routing_table = opt_routing_table
    }

    async fn lock(&self) -> VeilidAPIResult<AsyncMutexGuardArc<StorageManagerInner>> {
        let inner = asyncmutex_lock_arc!(&self.inner);
        if !inner.initialized {
            apibail_not_initialized!();
        }
        Ok(inner)
    }

    fn online_writes_ready_inner(inner: &StorageManagerInner) -> Option<RPCProcessor> {
        if let Some(rpc_processor) = { inner.opt_rpc_processor.clone() } {
            if let Some(network_class) = rpc_processor
                .routing_table()
                .get_network_class(RoutingDomain::PublicInternet)
            {
                // If our PublicInternet network class is valid we're ready to talk
                if network_class != NetworkClass::Invalid {
                    Some(rpc_processor)
                } else {
                    None
                }
            } else {
                // If we haven't gotten a network class yet we shouldnt try to use the DHT
                None
            }
        } else {
            // If we aren't attached, we won't have an rpc processor
            None
        }
    }

    async fn online_writes_ready(&self) -> EyreResult<Option<RPCProcessor>> {
        let inner = self.lock().await?;
        Ok(Self::online_writes_ready_inner(&inner))
    }

    async fn has_offline_subkey_writes(&self) -> EyreResult<bool> {
        let inner = self.lock().await?;
        Ok(!inner.offline_subkey_writes.is_empty())
    }

    /// Create a local record from scratch with a new owner key, open it, and return the opened descriptor
    pub async fn create_record(
        &self,
        kind: CryptoKind,
        schema: DHTSchema,
        safety_selection: SafetySelection,
    ) -> VeilidAPIResult<DHTRecordDescriptor> {
        let mut inner = self.lock().await?;

        // Create a new owned local record from scratch
        let (key, owner) = inner
            .create_new_owned_local_record(kind, schema, safety_selection)
            .await?;

        // Now that the record is made we should always succeed to open the existing record
        // The initial writer is the owner of the record
        inner
            .open_existing_record(key, Some(owner), safety_selection)
            .await
            .map(|r| r.unwrap())
    }

    /// Open an existing local record if it exists, and if it doesnt exist locally, try to pull it from the network and open it and return the opened descriptor
    pub async fn open_record(
        &self,
        key: TypedKey,
        writer: Option<KeyPair>,
        safety_selection: SafetySelection,
    ) -> VeilidAPIResult<DHTRecordDescriptor> {
        let mut inner = self.lock().await?;

        // See if we have a local record already or not
        if let Some(res) = inner
            .open_existing_record(key, writer, safety_selection)
            .await?
        {
            return Ok(res);
        }

        // No record yet, try to get it from the network

        // Get rpc processor and drop mutex so we don't block while getting the value from the network
        let Some(rpc_processor) = inner.opt_rpc_processor.clone() else {
            apibail_try_again!("offline, try again later");
        };

        // Drop the mutex so we dont block during network access
        drop(inner);

        // No last descriptor, no last value
        // Use the safety selection we opened the record with
        let subkey: ValueSubkey = 0;
        let result = self
            .outbound_get_value(
                rpc_processor,
                key,
                subkey,
                safety_selection,
                SubkeyResult::default(),
            )
            .await?;

        // If we got nothing back, the key wasn't found
        if result.subkey_result.value.is_none() && result.subkey_result.descriptor.is_none() {
            // No result
            apibail_key_not_found!(key);
        };

        // Reopen inner to store value we just got
        let mut inner = self.lock().await?;

        // Check again to see if we have a local record already or not
        // because waiting for the outbound_get_value action could result in the key being opened
        // via some parallel process

        if let Some(res) = inner
            .open_existing_record(key, writer, safety_selection)
            .await?
        {
            return Ok(res);
        }

        // Open the new record
        inner
            .open_new_record(key, writer, subkey, result.subkey_result, safety_selection)
            .await
    }

    /// Close an opened local record
    pub async fn close_record(&self, key: TypedKey) -> VeilidAPIResult<()> {
        let (opt_opened_record, opt_rpc_processor) = {
            let mut inner = self.lock().await?;
            (inner.close_record(key)?, inner.opt_rpc_processor.clone())
        };

        // Send a one-time cancel request for the watch if we have one and we're online
        if let Some(opened_record) = opt_opened_record {
            if let Some(active_watch) = opened_record.active_watch() {
                if let Some(rpc_processor) = opt_rpc_processor {
                    // Use the safety selection we opened the record with
                    // Use the writer we opened with as the 'watcher' as well
                    let opt_owvresult = self
                        .outbound_watch_value(
                            rpc_processor,
                            key,
                            ValueSubkeyRangeSet::full(),
                            Timestamp::new(0),
                            0,
                            opened_record.safety_selection(),
                            opened_record.writer().cloned(),
                            Some(active_watch.watch_node),
                        )
                        .await?;
                    if let Some(owvresult) = opt_owvresult {
                        if owvresult.expiration_ts.as_u64() != 0 {
                            log_stor!(debug
                                "close record watch cancel got unexpected expiration: {}",
                                owvresult.expiration_ts
                            );
                        }
                    } else {
                        log_stor!(debug "close record watch cancel unsuccessful");
                    }
                } else {
                    log_stor!(debug "skipping last-ditch watch cancel because we are offline");
                }
            }
        }

        Ok(())
    }

    /// Delete a local record
    pub async fn delete_record(&self, key: TypedKey) -> VeilidAPIResult<()> {
        // Ensure the record is closed
        self.close_record(key).await?;

        // Get record from the local store
        let mut inner = self.lock().await?;
        let Some(local_record_store) = inner.local_record_store.as_mut() else {
            apibail_not_initialized!();
        };

        // Remove the record from the local store
        local_record_store.delete_record(key).await
    }

    /// Get the value of a subkey from an opened local record
    pub async fn get_value(
        &self,
        key: TypedKey,
        subkey: ValueSubkey,
        force_refresh: bool,
    ) -> VeilidAPIResult<Option<ValueData>> {
        let mut inner = self.lock().await?;
        let safety_selection = {
            let Some(opened_record) = inner.opened_records.get(&key) else {
                apibail_generic!("record not open");
            };
            opened_record.safety_selection()
        };

        // See if the requested subkey is our local record store
        let last_subkey_result = inner.handle_get_local_value(key, subkey, true).await?;

        // Return the existing value if we have one unless we are forcing a refresh
        if !force_refresh {
            if let Some(last_subkey_result_value) = last_subkey_result.value {
                return Ok(Some(last_subkey_result_value.value_data().clone()));
            }
        }

        // Refresh if we can

        // Get rpc processor and drop mutex so we don't block while getting the value from the network
        let Some(rpc_processor) = inner.opt_rpc_processor.clone() else {
            // Return the existing value if we have one if we aren't online
            if let Some(last_subkey_result_value) = last_subkey_result.value {
                return Ok(Some(last_subkey_result_value.value_data().clone()));
            }
            apibail_try_again!("offline, try again later");
        };

        // Drop the lock for network access
        drop(inner);

        // May have last descriptor / value
        // Use the safety selection we opened the record with
        let opt_last_seq = last_subkey_result
            .value
            .as_ref()
            .map(|v| v.value_data().seq());
        let result = self
            .outbound_get_value(
                rpc_processor,
                key,
                subkey,
                safety_selection,
                last_subkey_result,
            )
            .await?;

        // See if we got a value back
        let Some(subkey_result_value) = result.subkey_result.value else {
            // If we got nothing back then we also had nothing beforehand, return nothing
            return Ok(None);
        };

        // Keep the list of nodes that returned a value for later reference
        let mut inner = self.lock().await?;
        inner.set_value_nodes(key, result.value_nodes)?;

        // If we got a new value back then write it to the opened record
        if Some(subkey_result_value.value_data().seq()) != opt_last_seq {
            inner
                .handle_set_local_value(key, subkey, subkey_result_value.clone())
                .await?;
        }
        Ok(Some(subkey_result_value.value_data().clone()))
    }

    /// Set the value of a subkey on an opened local record
    pub async fn set_value(
        &self,
        key: TypedKey,
        subkey: ValueSubkey,
        data: Vec<u8>,
    ) -> VeilidAPIResult<Option<ValueData>> {
        let mut inner = self.lock().await?;

        // Get cryptosystem
        let Some(vcrypto) = self.unlocked_inner.crypto.get(key.kind) else {
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

        // If we don't have a writer then we can't write
        let Some(writer) = opt_writer else {
            apibail_generic!("value is not writable");
        };

        // See if the subkey we are modifying has a last known local value
        let last_subkey_result = inner.handle_get_local_value(key, subkey, true).await?;

        // Get the descriptor and schema for the key
        let Some(descriptor) = last_subkey_result.descriptor else {
            apibail_generic!("must have a descriptor");
        };
        let schema = descriptor.schema()?;

        // Make new subkey data
        let value_data = if let Some(last_signed_value_data) = last_subkey_result.value {
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
            vcrypto,
            writer.secret,
        )?);

        // Get rpc processor and drop mutex so we don't block while getting the value from the network
        let Some(rpc_processor) = Self::online_writes_ready_inner(&inner) else {
            log_stor!(debug "Writing subkey locally: {}:{} len={}", key, subkey, signed_value_data.value_data().data().len() );

            // Offline, just write it locally and return immediately
            inner
                .handle_set_local_value(key, subkey, signed_value_data.clone())
                .await?;

            log_stor!(debug "Writing subkey offline: {}:{} len={}", key, subkey, signed_value_data.value_data().data().len() );
            // Add to offline writes to flush
            inner
                .offline_subkey_writes
                .entry(key)
                .and_modify(|x| {
                    x.subkeys.insert(subkey);
                })
                .or_insert(OfflineSubkeyWrite {
                    safety_selection,
                    subkeys: ValueSubkeyRangeSet::single(subkey),
                });
            return Ok(None);
        };

        // Drop the lock for network access
        drop(inner);

        // Use the safety selection we opened the record with
        let result = self
            .outbound_set_value(
                rpc_processor,
                key,
                subkey,
                safety_selection,
                signed_value_data.clone(),
                descriptor,
            )
            .await?;

        // Keep the list of nodes that returned a value for later reference
        let mut inner = self.lock().await?;
        inner.set_value_nodes(key, result.value_nodes)?;

        // Whatever record we got back, store it locally, might be newer than the one we asked to save
        inner
            .handle_set_local_value(key, subkey, result.signed_value_data.clone())
            .await?;

        // Return the new value if it differs from what was asked to set
        if result.signed_value_data.value_data() != signed_value_data.value_data() {
            return Ok(Some(result.signed_value_data.value_data().clone()));
        }

        // If the original value was set, return None
        Ok(None)
    }

    /// Add a watch to a DHT value
    pub async fn watch_values(
        &self,
        key: TypedKey,
        subkeys: ValueSubkeyRangeSet,
        expiration: Timestamp,
        count: u32,
    ) -> VeilidAPIResult<Timestamp> {
        let inner = self.lock().await?;

        // Rewrite subkey range if empty to full
        let subkeys = if subkeys.is_empty() {
            ValueSubkeyRangeSet::full()
        } else {
            subkeys
        };

        // Get the safety selection and the writer we opened this record with
        let (safety_selection, opt_writer, opt_watch_node) = {
            let Some(opened_record) = inner.opened_records.get(&key) else {
                apibail_generic!("record not open");
            };
            (
                opened_record.safety_selection(),
                opened_record.writer().cloned(),
                opened_record.active_watch().map(|aw| aw.watch_node.clone()),
            )
        };

        // Get rpc processor and drop mutex so we don't block while requesting the watch from the network
        let Some(rpc_processor) = inner.opt_rpc_processor.clone() else {
            apibail_try_again!("offline, try again later");
        };

        // Drop the lock for network access
        drop(inner);

        // Use the safety selection we opened the record with
        // Use the writer we opened with as the 'watcher' as well
        let opt_owvresult = self
            .outbound_watch_value(
                rpc_processor,
                key,
                subkeys.clone(),
                expiration,
                count,
                safety_selection,
                opt_writer,
                opt_watch_node,
            )
            .await?;

        // If we did not get a valid response return a zero timestamp
        let Some(owvresult) = opt_owvresult else {
            return Ok(Timestamp::new(0));
        };

        // Clear any existing watch if the watch succeeded or got cancelled
        let mut inner = self.lock().await?;
        let Some(opened_record) = inner.opened_records.get_mut(&key) else {
            apibail_generic!("record not open");
        };
        opened_record.clear_active_watch();

        // Get the minimum expiration timestamp we will accept
        let rpc_timeout_us = {
            let c = self.unlocked_inner.config.get();
            TimestampDuration::from(ms_to_us(c.network.rpc.timeout_ms))
        };
        let cur_ts = get_timestamp();
        let min_expiration_ts = cur_ts + rpc_timeout_us.as_u64();

        // If the expiration time is less than our minimum expiration time or greater than the requested time, consider this watch cancelled
        if owvresult.expiration_ts.as_u64() < min_expiration_ts
            || owvresult.expiration_ts.as_u64() > expiration.as_u64()
        {
            // Don't set the watch so we ignore any stray valuechanged messages
            return Ok(Timestamp::new(0));
        }

        // If we requested a cancellation, then consider this watch cancelled
        if count == 0 {
            return Ok(Timestamp::new(0));
        }

        // Keep a record of the watch
        opened_record.set_active_watch(ActiveWatch {
            expiration_ts: owvresult.expiration_ts,
            watch_node: owvresult.watch_node,
            opt_value_changed_route: owvresult.opt_value_changed_route,
            subkeys,
            count,
        });

        Ok(owvresult.expiration_ts)
    }

    pub async fn cancel_watch_values(
        &self,
        key: TypedKey,
        subkeys: ValueSubkeyRangeSet,
    ) -> VeilidAPIResult<bool> {
        let (subkeys, active_watch) = {
            let inner = self.lock().await?;
            let Some(opened_record) = inner.opened_records.get(&key) else {
                apibail_generic!("record not open");
            };

            // See what watch we have currently if any
            let Some(active_watch) = opened_record.active_watch() else {
                // If we didn't have an active watch, then we can just return false because there's nothing to do here
                return Ok(false);
            };

            // Rewrite subkey range if empty to full
            let subkeys = if subkeys.is_empty() {
                ValueSubkeyRangeSet::full()
            } else {
                subkeys
            };

            // Reduce the subkey range
            let new_subkeys = active_watch.subkeys.difference(&subkeys);

            (new_subkeys, active_watch)
        };

        // If we have no subkeys left, then set the count to zero to indicate a full cancellation
        let count = if subkeys.is_empty() {
            0
        } else {
            active_watch.count
        };

        // Update the watch
        let expiration_ts = self
            .watch_values(key, subkeys, active_watch.expiration_ts, count)
            .await?;

        // A zero expiration time means the watch is done or nothing is left, and the watch is no longer active
        if expiration_ts.as_u64() == 0 {
            return Ok(false);
        }

        Ok(true)
    }

    // Send single value change out to the network
    #[instrument(level = "trace", skip(self), err)]
    async fn send_value_change(&self, vc: ValueChangedInfo) -> VeilidAPIResult<()> {
        let rpc_processor = {
            let inner = self.inner.lock().await;
            if let Some(rpc_processor) = &inner.opt_rpc_processor {
                rpc_processor.clone()
            } else {
                apibail_try_again!("network is not available");
            }
        };

        let dest = rpc_processor
            .resolve_target_to_destination(
                vc.target,
                SafetySelection::Unsafe(Sequencing::NoPreference),
            )
            .await
            .map_err(VeilidAPIError::from)?;

        network_result_value_or_log!(rpc_processor
        .rpc_call_value_changed(dest, vc.key, vc.subkeys, vc.count, (*vc.value).clone())
        .await
        .map_err(VeilidAPIError::from)? => [format!(": dest={:?} vc={:?}", dest, vc)] {
        });

        Ok(())
    }
}
