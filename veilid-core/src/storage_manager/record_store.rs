/// RecordStore
/// Keeps an LRU cache of dht keys and their associated subkey valuedata.
/// Instances of this store are used for 'local' (persistent) and 'remote' (ephemeral) dht key storage.
/// This store does not perform any validation on the schema, and all ValueRecordData passed in must have been previously validated.
/// Uses an in-memory store for the records, backed by the TableStore. Subkey data is LRU cached and rotated out by a limits policy,
/// and backed to the TableStore for persistence.
use super::*;
use hashlink::LruCache;

#[derive(Debug, Clone)]
/// A dead record that is yet to be purged from disk and statistics
struct DeadRecord<D>
where
    D: fmt::Debug + Clone + Serialize + for<'d> Deserialize<'d>,
{
    /// The key used in the record_index
    key: RecordTableKey,
    /// The actual record
    record: Record<D>,
    /// True if this record is accounted for in the total storage
    /// and needs to have the statistics updated or not when purged
    in_total_storage: bool,
}

/// An individual watch
#[derive(Debug, Clone)]
struct WatchedRecordWatch {
    subkeys: ValueSubkeyRangeSet,
    expiration: Timestamp,
    count: u32,
    target: Target,
    watcher: CryptoKey,
    changed: ValueSubkeyRangeSet,
}

#[derive(Debug, Default, Clone)]
/// A record being watched for changes
struct WatchedRecord {
    /// The list of active watchers
    watchers: Vec<WatchedRecordWatch>,
}

pub(super) enum WatchUpdateMode {
    NoUpdate,
    UpdateAll,
    ExcludeTarget(Target),
}

pub(super) struct RecordStore<D>
where
    D: fmt::Debug + Clone + Serialize + for<'d> Deserialize<'d>,
{
    table_store: TableStore,
    name: String,
    limits: RecordStoreLimits,

    /// The tabledb used for record data
    record_table: Option<TableDB>,
    /// The tabledb used for subkey data
    subkey_table: Option<TableDB>,
    /// The in-memory index that keeps track of what records are in the tabledb
    record_index: LruCache<RecordTableKey, Record<D>>,
    /// The in-memory cache of commonly accessed subkey data so we don't have to keep hitting the db
    subkey_cache: LruCache<SubkeyTableKey, RecordData>,
    /// Total storage space or subkey data inclusive of structures in memory
    subkey_cache_total_size: LimitedSize<usize>,
    /// Total storage space of records in the tabledb inclusive of subkey data and structures
    total_storage_space: LimitedSize<u64>,
    /// Records to be removed from the tabledb upon next purge
    dead_records: Vec<DeadRecord<D>>,
    /// The list of records that have changed since last flush to disk (optimization for batched writes)
    changed_records: HashSet<RecordTableKey>,
    /// The list of records being watched for changes
    watched_records: HashMap<RecordTableKey, WatchedRecord>,
    /// The list of watched records that have changed values since last notification
    changed_watched_values: HashSet<RecordTableKey>,

    /// A mutex to ensure we handle this concurrently
    purge_dead_records_mutex: Arc<AsyncMutex<()>>,
}

/// The result of the do_get_value_operation
#[derive(Default, Debug)]
pub struct SubkeyResult {
    /// The subkey value if we got one
    pub value: Option<Arc<SignedValueData>>,
    /// The descriptor if we got a fresh one or empty if no descriptor was needed
    pub descriptor: Option<Arc<SignedValueDescriptor>>,
}

impl<D> RecordStore<D>
where
    D: fmt::Debug + Clone + Serialize + for<'d> Deserialize<'d>,
{
    pub fn new(table_store: TableStore, name: &str, limits: RecordStoreLimits) -> Self {
        let subkey_cache_size = limits.subkey_cache_size;
        let limit_subkey_cache_total_size = limits
            .max_subkey_cache_memory_mb
            .map(|mb| mb * 1_048_576usize);
        let limit_max_storage_space = limits
            .max_storage_space_mb
            .map(|mb| mb as u64 * 1_048_576u64);

        Self {
            table_store,
            name: name.to_owned(),
            limits,
            record_table: None,
            subkey_table: None,
            record_index: LruCache::new(limits.max_records.unwrap_or(usize::MAX)),
            subkey_cache: LruCache::new(subkey_cache_size),
            subkey_cache_total_size: LimitedSize::new(
                "subkey_cache_total_size",
                0,
                limit_subkey_cache_total_size,
            ),
            total_storage_space: LimitedSize::new(
                "total_storage_space",
                0,
                limit_max_storage_space,
            ),
            dead_records: Vec::new(),
            changed_records: HashSet::new(),
            watched_records: HashMap::new(),
            purge_dead_records_mutex: Arc::new(AsyncMutex::new(())),
            changed_watched_values: HashSet::new(),
        }
    }

    pub async fn init(&mut self) -> EyreResult<()> {
        let record_table = self
            .table_store
            .open(&format!("{}_records", self.name), 1)
            .await?;
        let subkey_table = self
            .table_store
            .open(&format!("{}_subkeys", self.name), 1)
            .await?;

        // Pull record index from table into a vector to ensure we sort them
        let record_table_keys = record_table.get_keys(0).await?;
        let mut record_index_saved: Vec<(RecordTableKey, Record<D>)> =
            Vec::with_capacity(record_table_keys.len());
        for rtk in record_table_keys {
            if let Some(vr) = record_table.load_json::<Record<D>>(0, &rtk).await? {
                let rik = RecordTableKey::try_from(rtk.as_ref())?;
                record_index_saved.push((rik, vr));
            }
        }

        // Sort the record index by last touched time and insert in sorted order
        record_index_saved.sort_by(|a, b| a.1.last_touched().cmp(&b.1.last_touched()));
        let mut dead_records = Vec::<DeadRecord<D>>::new();
        for ri in record_index_saved {
            // total the storage space
            self.total_storage_space
                .add((mem::size_of::<RecordTableKey>() + ri.1.total_size()) as u64)
                .unwrap();
            if self.total_storage_space.commit().is_err() {
                // Revert the total storage space because the commit failed
                self.total_storage_space.rollback();

                // If we overflow the limit, kill off the record, noting that it has not yet been added to the total storage space
                dead_records.push(DeadRecord {
                    key: ri.0,
                    record: ri.1,
                    in_total_storage: false,
                });
                continue;
            }

            // add to index and ensure we deduplicate in the case of an error
            if let Some(v) = self.record_index.insert_with_callback(ri.0, ri.1, |k, v| {
                // If the configuration change, we only want to keep the 'limits.max_records' records
                dead_records.push(DeadRecord {
                    key: k,
                    record: v,
                    in_total_storage: true,
                });
            }) {
                // This shouldn't happen, but deduplicate anyway
                log_stor!(warn "duplicate record in table: {:?}", ri.0);
                dead_records.push(DeadRecord {
                    key: ri.0,
                    record: v,
                    in_total_storage: true,
                });
            }
        }
        for dr in dead_records {
            self.dead_records.push(dr);
        }

        self.record_table = Some(record_table);
        self.subkey_table = Some(subkey_table);
        Ok(())
    }

    fn add_dead_record(&mut self, key: RecordTableKey, record: Record<D>) {
        self.dead_records.push(DeadRecord {
            key,
            record,
            in_total_storage: true,
        });
    }

    fn add_to_subkey_cache(&mut self, key: SubkeyTableKey, record_data: RecordData) {
        let record_data_total_size = record_data.total_size();
        // Write to subkey cache
        let mut dead_size = 0usize;
        if let Some(old_record_data) =
            self.subkey_cache
                .insert_with_callback(key, record_data, |_, v| {
                    // LRU out
                    dead_size += v.total_size();
                })
        {
            // Old data
            dead_size += old_record_data.total_size();
        }
        self.subkey_cache_total_size.sub(dead_size).unwrap();
        self.subkey_cache_total_size
            .add(record_data_total_size)
            .unwrap();

        // Purge over size limit
        while self.subkey_cache_total_size.commit().is_err() {
            if let Some((_, v)) = self.subkey_cache.remove_lru() {
                self.subkey_cache_total_size.saturating_sub(v.total_size());
            } else {
                self.subkey_cache_total_size.rollback();

                log_stor!(error "subkey cache should not be empty, has {} bytes unaccounted for",self.subkey_cache_total_size.get());

                self.subkey_cache_total_size.set(0);
                self.subkey_cache_total_size.commit().unwrap();
                break;
            }
        }
    }

    fn remove_from_subkey_cache(&mut self, key: SubkeyTableKey) {
        if let Some(dead_record_data) = self.subkey_cache.remove(&key) {
            self.subkey_cache_total_size
                .saturating_sub(dead_record_data.total_size());
            self.subkey_cache_total_size.commit().unwrap();
        }
    }

    async fn purge_dead_records(&mut self, lazy: bool) {
        let purge_dead_records_mutex = self.purge_dead_records_mutex.clone();
        let _lock = if lazy {
            match asyncmutex_try_lock!(purge_dead_records_mutex) {
                Some(v) => v,
                None => {
                    // If not ready now, just skip it if we're lazy
                    return;
                }
            }
        } else {
            // Not lazy, must wait
            purge_dead_records_mutex.lock().await
        };

        // Delete dead keys
        if self.dead_records.is_empty() {
            return;
        }

        let record_table = self.record_table.clone().unwrap();
        let subkey_table = self.subkey_table.clone().unwrap();

        let rt_xact = record_table.transact();
        let st_xact = subkey_table.transact();
        let dead_records = mem::take(&mut self.dead_records);
        for dr in dead_records {
            // Record should already be gone from index
            if self.record_index.contains_key(&dr.key) {
                log_stor!(error "dead record found in index: {:?}", dr.key);
            }

            // Delete record
            if let Err(e) = rt_xact.delete(0, &dr.key.bytes()) {
                log_stor!(error "record could not be deleted: {}", e);
            }

            // Delete subkeys
            let stored_subkeys = dr.record.stored_subkeys();
            for sk in stored_subkeys.iter() {
                // From table
                let stk = SubkeyTableKey {
                    key: dr.key.key,
                    subkey: sk,
                };
                let stkb = stk.bytes();
                if let Err(e) = st_xact.delete(0, &stkb) {
                    log_stor!(error "subkey could not be deleted: {}", e);
                }

                // From cache
                self.remove_from_subkey_cache(stk);
            }

            // Remove from total size
            if dr.in_total_storage {
                self.total_storage_space.saturating_sub(
                    (mem::size_of::<RecordTableKey>() + dr.record.total_size()) as u64,
                );
                self.total_storage_space.commit().unwrap();
            }
        }
        if let Err(e) = rt_xact.commit().await {
            log_stor!(error "failed to commit record table transaction: {}", e);
        }
        if let Err(e) = st_xact.commit().await {
            log_stor!(error "failed to commit subkey table transaction: {}", e);
        }
    }

    async fn flush_changed_records(&mut self) {
        if self.changed_records.is_empty() {
            return;
        }

        let record_table = self.record_table.clone().unwrap();

        let rt_xact = record_table.transact();
        let changed_records = mem::take(&mut self.changed_records);
        for rtk in changed_records {
            // Get the changed record and save it to the table
            if let Some(r) = self.record_index.peek(&rtk) {
                if let Err(e) = rt_xact.store_json(0, &rtk.bytes(), r) {
                    log_stor!(error "failed to save record: {}", e);
                }
            }
        }
        if let Err(e) = rt_xact.commit().await {
            log_stor!(error "failed to commit record table transaction: {}", e);
        }
    }

    pub async fn flush(&mut self) -> EyreResult<()> {
        self.flush_changed_records().await;
        self.purge_dead_records(true).await;
        Ok(())
    }

    pub async fn new_record(&mut self, key: TypedKey, record: Record<D>) -> VeilidAPIResult<()> {
        let rtk = RecordTableKey { key };
        if self.record_index.contains_key(&rtk) {
            apibail_internal!("record already exists");
        }

        // Get record table
        let Some(record_table) = self.record_table.clone() else {
            apibail_internal!("record store not initialized");
        };

        // If over size limit, dont create record
        self.total_storage_space
            .add((mem::size_of::<RecordTableKey>() + record.total_size()) as u64)
            .unwrap();
        if !self.total_storage_space.check_limit() {
            self.total_storage_space.rollback();
            apibail_try_again!("out of storage space");
        }

        // Save to record table
        record_table
            .store_json(0, &rtk.bytes(), &record)
            .await
            .map_err(VeilidAPIError::internal)?;

        // Update storage space (won't fail due to check_limit above)
        self.total_storage_space.commit().unwrap();

        // Save to record index
        let mut dead_records = Vec::new();
        if let Some(v) = self.record_index.insert_with_callback(rtk, record, |k, v| {
            dead_records.push((k, v));
        }) {
            // Shouldn't happen but log it
            log_stor!(warn "new duplicate record in table: {:?}", rtk);
            self.add_dead_record(rtk, v);
        }
        for dr in dead_records {
            self.add_dead_record(dr.0, dr.1);
        }

        Ok(())
    }

    pub async fn delete_record(&mut self, key: TypedKey) -> VeilidAPIResult<()> {
        // Get the record table key
        let rtk = RecordTableKey { key };

        // Remove record from the index
        let Some(record) = self.record_index.remove(&rtk) else {
            apibail_key_not_found!(key);
        };

        self.add_dead_record(rtk, record);

        self.purge_dead_records(false).await;

        Ok(())
    }

    pub(super) fn with_record<R, F>(&mut self, key: TypedKey, f: F) -> Option<R>
    where
        F: FnOnce(&Record<D>) -> R,
    {
        // Get record from index
        let mut out = None;
        let rtk = RecordTableKey { key };
        if let Some(record) = self.record_index.get_mut(&rtk) {
            // Callback
            out = Some(f(record));

            // Touch
            record.touch(get_aligned_timestamp());
        }
        if out.is_some() {
            // Marks as changed because the record was touched and we want to keep the
            // LRU ordering serialized
            self.changed_records.insert(rtk);
        }

        out
    }

    pub(super) fn peek_record<R, F>(&self, key: TypedKey, f: F) -> Option<R>
    where
        F: FnOnce(&Record<D>) -> R,
    {
        // Get record from index
        let mut out = None;
        let rtk = RecordTableKey { key };
        if let Some(record) = self.record_index.peek(&rtk) {
            // Callback
            out = Some(f(record));
        }
        out
    }

    pub(super) fn with_record_mut<R, F>(&mut self, key: TypedKey, f: F) -> Option<R>
    where
        F: FnOnce(&mut Record<D>) -> R,
    {
        // Get record from index
        let mut out = None;
        let rtk = RecordTableKey { key };
        if let Some(record) = self.record_index.get_mut(&rtk) {
            // Callback
            out = Some(f(record));

            // Touch
            record.touch(get_aligned_timestamp());
        }
        if out.is_some() {
            // Marks as changed because the record was touched and we want to keep the
            // LRU ordering serialized
            self.changed_records.insert(rtk);
        }

        out
    }

    pub async fn get_subkey(
        &mut self,
        key: TypedKey,
        subkey: ValueSubkey,
        want_descriptor: bool,
    ) -> VeilidAPIResult<Option<SubkeyResult>> {
        // Get record from index
        let Some((subkey_count, has_subkey, opt_descriptor)) = self.with_record(key, |record| {
            (
                record.subkey_count(),
                record.stored_subkeys().contains(subkey),
                if want_descriptor {
                    Some(record.descriptor().clone())
                } else {
                    None
                },
            )
        }) else {
            // Record not available
            return Ok(None);
        };

        // Check if the subkey is in range
        if subkey as usize >= subkey_count {
            apibail_invalid_argument!("subkey out of range", "subkey", subkey);
        }

        // See if we have this subkey stored
        if !has_subkey {
            // If not, return no value but maybe with descriptor
            return Ok(Some(SubkeyResult {
                value: None,
                descriptor: opt_descriptor,
            }));
        }

        // Get subkey table
        let Some(subkey_table) = self.subkey_table.clone() else {
            apibail_internal!("record store not initialized");
        };

        // If subkey exists in subkey cache, use that
        let stk = SubkeyTableKey { key, subkey };
        if let Some(record_data) = self.subkey_cache.get_mut(&stk) {
            let out = record_data.signed_value_data().clone();

            return Ok(Some(SubkeyResult {
                value: Some(out),
                descriptor: opt_descriptor,
            }));
        }
        // If not in cache, try to pull from table store if it is in our stored subkey set
        let Some(record_data) = subkey_table
            .load_json::<RecordData>(0, &stk.bytes())
            .await
            .map_err(VeilidAPIError::internal)?
        else {
            apibail_internal!("failed to get subkey that was stored");
        };

        let out = record_data.signed_value_data().clone();

        // Add to cache, do nothing with lru out
        self.add_to_subkey_cache(stk, record_data);

        Ok(Some(SubkeyResult {
            value: Some(out),
            descriptor: opt_descriptor,
        }))
    }

    pub(crate) async fn peek_subkey(
        &self,
        key: TypedKey,
        subkey: ValueSubkey,
        want_descriptor: bool,
    ) -> VeilidAPIResult<Option<SubkeyResult>> {
        // record from index
        let Some((subkey_count, has_subkey, opt_descriptor)) = self.peek_record(key, |record| {
            (
                record.subkey_count(),
                record.stored_subkeys().contains(subkey),
                if want_descriptor {
                    Some(record.descriptor().clone())
                } else {
                    None
                },
            )
        }) else {
            // Record not available
            return Ok(None);
        };

        // Check if the subkey is in range
        if subkey as usize >= subkey_count {
            apibail_invalid_argument!("subkey out of range", "subkey", subkey);
        }

        // See if we have this subkey stored
        if !has_subkey {
            // If not, return no value but maybe with descriptor
            return Ok(Some(SubkeyResult {
                value: None,
                descriptor: opt_descriptor,
            }));
        }

        // Get subkey table
        let Some(subkey_table) = self.subkey_table.clone() else {
            apibail_internal!("record store not initialized");
        };

        // If subkey exists in subkey cache, use that
        let stk = SubkeyTableKey { key, subkey };
        if let Some(record_data) = self.subkey_cache.peek(&stk) {
            let out = record_data.signed_value_data().clone();

            return Ok(Some(SubkeyResult {
                value: Some(out),
                descriptor: opt_descriptor,
            }));
        }
        // If not in cache, try to pull from table store if it is in our stored subkey set
        let Some(record_data) = subkey_table
            .load_json::<RecordData>(0, &stk.bytes())
            .await
            .map_err(VeilidAPIError::internal)?
        else {
            apibail_internal!("failed to peek subkey that was stored");
        };

        let out = record_data.signed_value_data().clone();

        Ok(Some(SubkeyResult {
            value: Some(out),
            descriptor: opt_descriptor,
        }))
    }

    async fn update_watched_value(
        &mut self,
        key: TypedKey,
        subkey: ValueSubkey,
        opt_ignore_target: Option<Target>,
    ) {
        let rtk = RecordTableKey { key };
        let Some(wr) = self.watched_records.get_mut(&rtk) else {
            return;
        };
        // Update all watchers
        let mut changed = false;
        for w in &mut wr.watchers {
            // If this watcher is watching the changed subkey then add to the watcher's changed list
            // Don't bother marking changes for value sets coming from the same watching node/target because they
            // are already going to be aware of the changes in that case
            if Some(&w.target) != opt_ignore_target.as_ref()
                && w.subkeys.contains(subkey)
                && w.changed.insert(subkey)
            {
                changed = true;
            }
        }
        if changed {
            self.changed_watched_values.insert(rtk);
        }
    }

    pub async fn set_subkey(
        &mut self,
        key: TypedKey,
        subkey: ValueSubkey,
        signed_value_data: Arc<SignedValueData>,
        watch_update_mode: WatchUpdateMode,
    ) -> VeilidAPIResult<()> {
        // Check size limit for data
        if signed_value_data.value_data().data().len() > self.limits.max_subkey_size {
            apibail_invalid_argument!(
                "record subkey too large",
                "signed_value_data.value_data.data.len",
                signed_value_data.value_data().data().len()
            );
        }

        // Get record subkey count and total size of all record subkey data exclusive of structures
        let Some((subkey_count, prior_record_data_size)) = self.with_record(key, |record| {
            (record.subkey_count(), record.record_data_size())
        }) else {
            apibail_invalid_argument!("no record at this key", "key", key);
        };

        // Check if the subkey is in range
        if subkey as usize >= subkey_count {
            apibail_invalid_argument!("subkey out of range", "subkey", subkey);
        }

        // Get subkey table
        let Some(subkey_table) = self.subkey_table.clone() else {
            apibail_internal!("record store not initialized");
        };

        // Get the previous subkey and ensure we aren't going over the record size limit
        let mut prior_subkey_size = 0usize;

        // If subkey exists in subkey cache, use that
        let stk = SubkeyTableKey { key, subkey };
        let stk_bytes = stk.bytes();

        if let Some(record_data) = self.subkey_cache.peek(&stk) {
            prior_subkey_size = record_data.data_size();
        } else {
            // If not in cache, try to pull from table store
            if let Some(record_data) = subkey_table
                .load_json::<RecordData>(0, &stk_bytes)
                .await
                .map_err(VeilidAPIError::internal)?
            {
                prior_subkey_size = record_data.data_size();
            }
        }

        // Make new record data
        let subkey_record_data = RecordData::new(signed_value_data);

        // Check new total record size
        let new_subkey_size = subkey_record_data.data_size();
        let new_record_data_size = prior_record_data_size - prior_subkey_size + new_subkey_size;
        if new_record_data_size > self.limits.max_record_total_size {
            apibail_generic!("dht record too large");
        }

        // Check new total storage space
        self.total_storage_space
            .sub(prior_subkey_size as u64)
            .unwrap();
        self.total_storage_space
            .add(new_subkey_size as u64)
            .unwrap();
        if !self.total_storage_space.check_limit() {
            apibail_try_again!("out of storage space");
        }

        // Write subkey
        subkey_table
            .store_json(0, &stk_bytes, &subkey_record_data)
            .await
            .map_err(VeilidAPIError::internal)?;

        // Write to subkey cache
        self.add_to_subkey_cache(stk, subkey_record_data);

        // Update record
        self.with_record_mut(key, |record| {
            record.store_subkey(subkey);
            record.set_record_data_size(new_record_data_size);
        })
        .expect("record should still be here");

        // Update storage space
        self.total_storage_space.commit().unwrap();

        // Update watched value

        let (do_update, opt_ignore_target) = match watch_update_mode {
            WatchUpdateMode::NoUpdate => (false, None),
            WatchUpdateMode::UpdateAll => (true, None),
            WatchUpdateMode::ExcludeTarget(target) => (true, Some(target)),
        };
        if do_update {
            self.update_watched_value(key, subkey, opt_ignore_target)
                .await;
        }

        Ok(())
    }

    /// Add a record watch for changes
    pub async fn watch_record(
        &mut self,
        key: TypedKey,
        subkeys: ValueSubkeyRangeSet,
        mut expiration: Timestamp,
        count: u32,
        target: Target,
        watcher: CryptoKey,
    ) -> VeilidAPIResult<Option<Timestamp>> {
        // If subkeys is empty or count is zero then we're cancelling a watch completely
        if subkeys.is_empty() || count == 0 {
            return self.cancel_watch(key, target, watcher).await;
        }

        // See if expiration timestamp is too far in the future or not enough in the future
        let cur_ts = get_timestamp();
        let max_ts = cur_ts + self.limits.max_watch_expiration.as_u64();
        let min_ts = cur_ts + self.limits.min_watch_expiration.as_u64();

        if expiration.as_u64() == 0 || expiration.as_u64() > max_ts {
            // Clamp expiration max time (or set zero expiration to max)
            expiration = Timestamp::new(max_ts);
        } else if expiration.as_u64() < min_ts {
            // Don't add watches with too low of an expiration time
            return Ok(None);
        }

        // Get the record being watched
        let Some(is_member) = self.with_record(key, |record| {
            // Check if the watcher specified is a schema member
            let schema = record.schema();
            (*record.owner()) == watcher || schema.is_member(&watcher)
        }) else {
            // Record not found
            return Ok(None);
        };

        // See if we are updating an existing watch
        // with the watcher matched on target
        let mut watch_count = 0;
        let rtk = RecordTableKey { key };
        if let Some(watch) = self.watched_records.get_mut(&rtk) {
            for w in &mut watch.watchers {
                if w.watcher == watcher {
                    watch_count += 1;

                    // Only one watch for an anonymous watcher
                    // Allow members to have one watch per target
                    if !is_member || w.target == target {
                        // Updating an existing watch
                        w.subkeys = subkeys;
                        w.expiration = expiration;
                        w.count = count;
                        return Ok(Some(expiration));
                    }
                }
            }
        }

        // Adding a new watcher to a watch
        // Check watch table for limits
        if is_member {
            // Member watch
            if watch_count >= self.limits.member_watch_limit {
                // Too many watches
                return Ok(None);
            }
        } else {
            // Public watch
            if watch_count >= self.limits.public_watch_limit {
                // Too many watches
                return Ok(None);
            }
        }

        // Ok this is an acceptable new watch, add it
        let watch = self.watched_records.entry(rtk).or_default();
        watch.watchers.push(WatchedRecordWatch {
            subkeys,
            expiration,
            count,
            target,
            watcher,
            changed: ValueSubkeyRangeSet::new(),
        });
        Ok(Some(expiration))
    }

    /// Add a record watch for changes
    async fn cancel_watch(
        &mut self,
        key: TypedKey,
        target: Target,
        watcher: CryptoKey,
    ) -> VeilidAPIResult<Option<Timestamp>> {
        // Get the record being watched
        let Some(is_member) = self.with_record(key, |record| {
            // Check if the watcher specified is a schema member
            let schema = record.schema();
            (*record.owner()) == watcher || schema.is_member(&watcher)
        }) else {
            // Record not found
            return Ok(None);
        };

        // See if we are cancelling an existing watch
        // with the watcher matched on target
        let rtk = RecordTableKey { key };
        let mut is_empty = false;
        let mut ret_timestamp = None;
        if let Some(watch) = self.watched_records.get_mut(&rtk) {
            let mut dead_watcher = None;
            for (wn, w) in watch.watchers.iter_mut().enumerate() {
                if w.watcher == watcher {
                    // Only one watch for an anonymous watcher
                    // Allow members to have one watch per target
                    if !is_member || w.target == target {
                        // Canceling an existing watch
                        dead_watcher = Some(wn);
                        ret_timestamp = Some(w.expiration);
                        break;
                    }
                }
            }
            if let Some(dw) = dead_watcher {
                watch.watchers.remove(dw);
                if watch.watchers.is_empty() {
                    is_empty = true;
                }
            }
        }
        if is_empty {
            self.watched_records.remove(&rtk);
        }

        Ok(ret_timestamp)
    }

    pub async fn take_value_changes(&mut self, changes: &mut Vec<ValueChangedInfo>) {
        // ValueChangedInfo but without the subkey data that requires a double mutable borrow to get
        struct EarlyValueChangedInfo {
            target: Target,
            key: TypedKey,
            subkeys: ValueSubkeyRangeSet,
            count: u32,
        }

        let mut evcis = vec![];
        let mut empty_watched_records = vec![];
        for rtk in self.changed_watched_values.drain() {
            if let Some(watch) = self.watched_records.get_mut(&rtk) {
                // Process watch notifications
                let mut dead_watchers = vec![];
                for (wn, w) in watch.watchers.iter_mut().enumerate() {
                    // Get the subkeys that have changed
                    let subkeys = w.changed.clone();
                    w.changed.clear();

                    // Reduce the count of changes sent
                    // if count goes to zero mark this watcher dead
                    w.count -= 1;
                    let count = w.count;
                    if count == 0 {
                        dead_watchers.push(wn);
                    }

                    evcis.push(EarlyValueChangedInfo {
                        target: w.target,
                        key: rtk.key,
                        subkeys,
                        count,
                    });
                }

                // Remove in reverse so we don't have to offset the index to remove the right key
                for dw in dead_watchers.iter().rev().copied() {
                    watch.watchers.remove(dw);
                    if watch.watchers.is_empty() {
                        empty_watched_records.push(rtk);
                    }
                }
            }
        }
        for ewr in empty_watched_records {
            self.watched_records.remove(&ewr);
        }

        for evci in evcis {
            // Get the first subkey data
            let Some(first_subkey) = evci.subkeys.first() else {
                log_stor!(error "first subkey should exist for value change notification");
                continue;
            };
            let subkey_result = match self.get_subkey(evci.key, first_subkey, false).await {
                Ok(Some(skr)) => skr,
                Ok(None) => {
                    log_stor!(error "subkey should have data for value change notification");
                    continue;
                }
                Err(e) => {
                    log_stor!(error "error getting subkey data for value change notification: {}", e);
                    continue;
                }
            };
            let Some(value) = subkey_result.value else {
                log_stor!(error "first subkey should have had value for value change notification");
                continue;
            };

            changes.push(ValueChangedInfo {
                target: evci.target,
                key: evci.key,
                subkeys: evci.subkeys,
                count: evci.count,
                value,
            });
        }
    }

    /// LRU out some records until we reclaim the amount of space requested
    /// This will force a garbage collection of the space immediately
    /// If zero is passed in here, a garbage collection will be performed of dead records
    /// without removing any live records
    pub async fn reclaim_space(&mut self, space: usize) -> usize {
        let mut reclaimed = 0usize;
        while reclaimed < space {
            if let Some((k, v)) = self.record_index.remove_lru() {
                reclaimed += mem::size_of::<RecordTableKey>();
                reclaimed += v.total_size();
                self.add_dead_record(k, v);
            } else {
                break;
            }
        }
        self.purge_dead_records(false).await;
        reclaimed
    }

    pub fn debug_records(&self) -> String {
        // Dump fields in an abbreviated way
        let mut out = String::new();

        out += "Record Index:\n";
        for (rik, rec) in &self.record_index {
            out += &format!(
                "  {} age={} len={} subkeys={}\n",
                rik.key,
                debug_duration(get_timestamp() - rec.last_touched().as_u64()),
                rec.record_data_size(),
                rec.stored_subkeys(),
            );
        }
        out += &format!("Subkey Cache Count: {}\n", self.subkey_cache.len());
        out += &format!(
            "Subkey Cache Total Size: {}\n",
            self.subkey_cache_total_size.get()
        );
        out += &format!("Total Storage Space: {}\n", self.total_storage_space.get());
        out += &format!("Dead Records: {}\n", self.dead_records.len());
        for dr in &self.dead_records {
            out += &format!("  {}\n", dr.key.key);
        }
        out += &format!("Changed Records: {}\n", self.changed_records.len());
        for cr in &self.changed_records {
            out += &format!("  {}\n", cr.key);
        }

        out
    }

    pub fn debug_record_info(&self, key: TypedKey) -> String {
        self.peek_record(key, |r| format!("{:#?}", r))
            .unwrap_or("Not found".to_owned())
    }

    pub async fn debug_record_subkey_info(&self, key: TypedKey, subkey: ValueSubkey) -> String {
        match self.peek_subkey(key, subkey, true).await {
            Ok(Some(v)) => {
                format!("{:#?}", v)
            }
            Ok(None) => "Subkey not available".to_owned(),
            Err(e) => format!("{}", e),
        }
    }
}
