/// RecordStore
/// Keeps an LRU cache of dht keys and their associated subkey valuedata.
/// Instances of this store are used for 'local' (persistent) and 'remote' (ephemeral) dht key storage.
/// This store does not perform any validation on the schema, and all ValueRecordData passed in must have been previously validated.
/// Uses an in-memory store for the records, backed by the TableStore. Subkey data is LRU cached and rotated out by a limits policy,
/// and backed to the TableStore for persistence.
mod inspect_cache;
mod keys;
mod limited_size;
mod local_record_detail;
mod opened_record;
mod record;
mod record_data;
mod record_store_limits;
mod remote_record_detail;
mod watch;

pub(super) use inspect_cache::*;
pub(super) use keys::*;
pub(super) use limited_size::*;
pub(super) use local_record_detail::*;
pub(super) use opened_record::*;
pub(super) use record::*;
pub(super) use record_data::*;
pub(super) use record_store_limits::*;
pub(super) use remote_record_detail::*;
pub(super) use watch::*;
pub use watch::{WatchParameters, WatchResult};

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
    /// The in-memory cache of commonly accessed sequence number data so we don't have to keep hitting the db
    inspect_cache: InspectCache,
    /// Total storage space or subkey data inclusive of structures in memory
    subkey_cache_total_size: LimitedSize<usize>,
    /// Total storage space of records in the tabledb inclusive of subkey data and structures
    total_storage_space: LimitedSize<u64>,
    /// Records to be removed from the tabledb upon next purge
    dead_records: Vec<DeadRecord<D>>,
    /// The list of records that have changed since last flush to disk (optimization for batched writes)
    changed_records: HashSet<RecordTableKey>,
    /// The list of records being watched for changes
    watched_records: HashMap<RecordTableKey, WatchList>,
    /// The list of watched records that have changed values since last notification
    changed_watched_values: HashSet<RecordTableKey>,
    /// A mutex to ensure we handle this concurrently
    purge_dead_records_mutex: Arc<AsyncMutex<()>>,
}

/// The result of the do_get_value_operation
#[derive(Default, Clone, Debug)]
pub struct GetResult {
    /// The subkey value if we got one
    pub opt_value: Option<Arc<SignedValueData>>,
    /// The descriptor if we got a fresh one or empty if no descriptor was needed
    pub opt_descriptor: Option<Arc<SignedValueDescriptor>>,
}

/// The result of the do_inspect_value_operation
#[derive(Default, Clone, Debug)]
pub struct InspectResult {
    /// The actual in-schema subkey range being reported on
    pub subkeys: ValueSubkeyRangeSet,
    /// The sequence map
    pub seqs: Vec<ValueSeqNum>,
    /// The descriptor if we got a fresh one or empty if no descriptor was needed
    pub opt_descriptor: Option<Arc<SignedValueDescriptor>>,
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
            inspect_cache: InspectCache::new(subkey_cache_size),
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

            // Record should have no watches now
            if self.watched_records.contains_key(&dr.key) {
                log_stor!(error "dead record found in watches: {:?}", dr.key);
            }

            // Record should have no watch changes now
            if self.changed_watched_values.contains(&dr.key) {
                log_stor!(error "dead record found in watch changes: {:?}", dr.key);
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

        // Remove watches
        self.watched_records.remove(&rtk);

        // Remove watch changes
        self.changed_watched_values.remove(&rtk);

        // Invalidate inspect cache for this key
        self.inspect_cache.invalidate(&rtk.key);

        // Remove from table store immediately
        self.add_dead_record(rtk, record);
        self.purge_dead_records(false).await;

        Ok(())
    }

    pub(super) fn contains_record(&mut self, key: TypedKey) -> bool {
        let rtk = RecordTableKey { key };
        self.record_index.contains_key(&rtk)
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
    ) -> VeilidAPIResult<Option<GetResult>> {
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
            return Ok(Some(GetResult {
                opt_value: None,
                opt_descriptor,
            }));
        }

        // Get subkey table
        let Some(subkey_table) = self.subkey_table.clone() else {
            apibail_internal!("record store not initialized");
        };

        // If subkey exists in subkey cache, use that
        let stk = SubkeyTableKey { key, subkey };
        if let Some(record_data) = self.subkey_cache.get(&stk) {
            let out = record_data.signed_value_data().clone();

            return Ok(Some(GetResult {
                opt_value: Some(out),
                opt_descriptor,
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

        Ok(Some(GetResult {
            opt_value: Some(out),
            opt_descriptor,
        }))
    }

    pub(crate) async fn peek_subkey(
        &self,
        key: TypedKey,
        subkey: ValueSubkey,
        want_descriptor: bool,
    ) -> VeilidAPIResult<Option<GetResult>> {
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
            return Ok(Some(GetResult {
                opt_value: None,
                opt_descriptor,
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

            return Ok(Some(GetResult {
                opt_value: Some(out),
                opt_descriptor,
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

        Ok(Some(GetResult {
            opt_value: Some(out),
            opt_descriptor,
        }))
    }

    async fn update_watched_value(
        &mut self,
        key: TypedKey,
        subkey: ValueSubkey,
        watch_update_mode: WatchUpdateMode,
    ) {
        let (do_update, opt_ignore_target) = match watch_update_mode {
            WatchUpdateMode::NoUpdate => (false, None),
            WatchUpdateMode::UpdateAll => (true, None),
            WatchUpdateMode::ExcludeTarget(target) => (true, Some(target)),
        };
        if !do_update {
            return;
        }

        let rtk = RecordTableKey { key };
        let Some(wr) = self.watched_records.get_mut(&rtk) else {
            return;
        };

        // Update all watchers
        let mut changed = false;
        for w in &mut wr.watches {
            // If this watcher is watching the changed subkey then add to the watcher's changed list
            // Don't bother marking changes for value sets coming from the same watching node/target because they
            // are already going to be aware of the changes in that case
            if Some(&w.params.target) != opt_ignore_target.as_ref()
                && w.params.subkeys.contains(subkey)
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

        // Write to inspect cache
        self.inspect_cache.replace_subkey_seq(
            &stk.key,
            subkey,
            subkey_record_data.signed_value_data().value_data().seq(),
        );

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

        // Send updates to
        self.update_watched_value(key, subkey, watch_update_mode)
            .await;

        Ok(())
    }

    pub async fn inspect_record(
        &mut self,
        key: TypedKey,
        subkeys: ValueSubkeyRangeSet,
        want_descriptor: bool,
    ) -> VeilidAPIResult<Option<InspectResult>> {
        // Get subkey table
        let Some(subkey_table) = self.subkey_table.clone() else {
            apibail_internal!("record store not initialized");
        };

        // Get record from index
        let Some((subkeys, opt_descriptor)) = self.with_record(key, |record| {
            // Get number of subkeys from schema and ensure we are getting the
            // right number of sequence numbers betwen that and what we asked for
            let truncated_subkeys = record
                .schema()
                .truncate_subkeys(&subkeys, Some(MAX_INSPECT_VALUE_A_SEQS_LEN));
            (
                truncated_subkeys,
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

        // Check if we can return some subkeys
        if subkeys.is_empty() {
            apibail_invalid_argument!("subkeys set does not overlap schema", "subkeys", subkeys);
        }

        // See if we have this inspection cached
        if let Some(icv) = self.inspect_cache.get(&key, &subkeys) {
            return Ok(Some(InspectResult {
                subkeys,
                seqs: icv.seqs,
                opt_descriptor,
            }));
        }

        // Build sequence number list to return
        #[allow(clippy::unnecessary_cast)]
        let mut seqs = Vec::with_capacity(subkeys.len() as usize);
        for subkey in subkeys.iter() {
            let stk = SubkeyTableKey { key, subkey };
            let seq = if let Some(record_data) = self.subkey_cache.peek(&stk) {
                record_data.signed_value_data().value_data().seq()
            } else {
                // If not in cache, try to pull from table store if it is in our stored subkey set
                // XXX: This would be better if it didn't have to pull the whole record data to get the seq.
                if let Some(record_data) = subkey_table
                    .load_json::<RecordData>(0, &stk.bytes())
                    .await
                    .map_err(VeilidAPIError::internal)?
                {
                    record_data.signed_value_data().value_data().seq()
                } else {
                    // Subkey not written to
                    ValueSubkey::MAX
                }
            };
            seqs.push(seq)
        }

        // Save seqs cache
        self.inspect_cache.put(
            key,
            subkeys.clone(),
            InspectCacheL2Value { seqs: seqs.clone() },
        );

        Ok(Some(InspectResult {
            subkeys,
            seqs,
            opt_descriptor,
        }))
    }

    pub async fn _change_existing_watch(
        &mut self,
        key: TypedKey,
        params: WatchParameters,
        watch_id: u64,
    ) -> VeilidAPIResult<WatchResult> {
        if params.count == 0 {
            apibail_internal!("cancel watch should not have gotten here");
        }
        if params.expiration.as_u64() == 0 {
            apibail_internal!("zero expiration should have been resolved to max by now");
        }
        // Get the watch list for this record
        let rtk = RecordTableKey { key };
        let Some(watch_list) = self.watched_records.get_mut(&rtk) else {
            // No watches, nothing to change
            return Ok(WatchResult::Rejected);
        };

        // Check each watch to see if we have an exact match for the id to change
        for w in &mut watch_list.watches {
            // If the watch id doesn't match, then we're not updating
            // Also do not allow the watcher key to change
            if w.id == watch_id && w.params.watcher == params.watcher {
                // Updating an existing watch
                w.params = params;
                return Ok(WatchResult::Changed {
                    expiration: w.params.expiration,
                });
            }
        }

        // No existing watch found
        Ok(WatchResult::Rejected)
    }

    pub async fn _create_new_watch(
        &mut self,
        key: TypedKey,
        params: WatchParameters,
        member_check: Box<dyn Fn(PublicKey) -> bool + Send>,
    ) -> VeilidAPIResult<WatchResult> {
        // Generate a record-unique watch id > 0
        let rtk = RecordTableKey { key };
        let mut id = 0;
        while id == 0 {
            id = get_random_u64();
        }
        if let Some(watched_record) = self.watched_records.get_mut(&rtk) {
            // Make sure it doesn't match any other id (unlikely, but lets be certain)
            'x: loop {
                for w in &mut watched_record.watches {
                    if w.id == id {
                        loop {
                            id = id.overflowing_add(1).0;
                            if id != 0 {
                                break;
                            }
                        }
                        continue 'x;
                    }
                }
                break;
            }
        }

        // Calculate watch limits
        let mut watch_count = 0;
        let mut target_watch_count = 0;

        let is_member = member_check(params.watcher);

        let rtk = RecordTableKey { key };
        if let Some(watched_record) = self.watched_records.get_mut(&rtk) {
            // Total up the number of watches for this key
            for w in &mut watched_record.watches {
                // See if this watch should be counted toward any limits
                let count_watch = if is_member {
                    // If the watcher is a member of the schema, then consider the total per-watcher key
                    w.params.watcher == params.watcher
                } else {
                    // If the watcher is not a member of the schema, the check if this watch is an anonymous watch and contributes to per-record key total
                    !member_check(w.params.watcher)
                };

                // For any watch, if the target matches our also tally that separately
                // If the watcher is a member of the schema, then consider the total per-target-per-watcher key
                // If the watcher is not a member of the schema, then it is an anonymous watch and the total is per-target-per-record key
                if count_watch {
                    watch_count += 1;
                    if w.params.target == params.target {
                        target_watch_count += 1;
                    }
                }
            }
        }

        // For members, no more than one watch per target per watcher per record
        // For anonymous, no more than one watch per target per record
        if target_watch_count > 0 {
            // Too many watches
            return Ok(WatchResult::Rejected);
        }

        // Check watch table for limits
        let watch_limit = if is_member {
            self.limits.member_watch_limit
        } else {
            self.limits.public_watch_limit
        };
        if watch_count >= watch_limit {
            return Ok(WatchResult::Rejected);
        }

        // Ok this is an acceptable new watch, add it
        let watch_list = self.watched_records.entry(rtk).or_default();
        let expiration = params.expiration;
        watch_list.watches.push(Watch {
            params,
            id,
            changed: ValueSubkeyRangeSet::new(),
        });
        Ok(WatchResult::Created { id, expiration })
    }

    /// Add or update an inbound record watch for changes
    #[allow(clippy::too_many_arguments)]
    pub async fn watch_record(
        &mut self,
        key: TypedKey,
        mut params: WatchParameters,
        opt_watch_id: Option<u64>,
    ) -> VeilidAPIResult<WatchResult> {
        // If count is zero then we're cancelling a watch completely
        if params.count == 0 {
            if let Some(watch_id) = opt_watch_id {
                let cancelled = self.cancel_watch(key, watch_id, params.watcher).await?;
                if cancelled {
                    return Ok(WatchResult::Cancelled);
                }
                return Ok(WatchResult::Rejected);
            }
            apibail_internal!("shouldn't have let a None watch id get here");
        }

        // See if expiration timestamp is too far in the future or not enough in the future
        let cur_ts = get_timestamp();
        let max_ts = cur_ts + self.limits.max_watch_expiration.as_u64();
        let min_ts = cur_ts + self.limits.min_watch_expiration.as_u64();

        if params.expiration.as_u64() == 0 || params.expiration.as_u64() > max_ts {
            // Clamp expiration max time (or set zero expiration to max)
            params.expiration = Timestamp::new(max_ts);
        } else if params.expiration.as_u64() < min_ts {
            // Don't add watches with too low of an expiration time
            if let Some(watch_id) = opt_watch_id {
                let cancelled = self.cancel_watch(key, watch_id, params.watcher).await?;
                if cancelled {
                    return Ok(WatchResult::Cancelled);
                }
            }
            return Ok(WatchResult::Rejected);
        }

        // Make a closure to check for member vs anonymous
        let Some(member_check) = self.with_record(key, |record| {
            let schema = record.schema();
            let owner = *record.owner();
            Box::new(move |watcher| owner == params.watcher || schema.is_member(&watcher))
        }) else {
            // Record not found
            return Ok(WatchResult::Rejected);
        };

        // Create or update depending on if a watch id is specified or not
        if let Some(watch_id) = opt_watch_id {
            self._change_existing_watch(key, params, watch_id).await
        } else {
            self._create_new_watch(key, params, member_check).await
        }
    }

    /// Clear a specific watch for a record
    /// returns true if the watch was found and cancelled
    async fn cancel_watch(
        &mut self,
        key: TypedKey,
        watch_id: u64,
        watcher: PublicKey,
    ) -> VeilidAPIResult<bool> {
        if watch_id == 0 {
            apibail_internal!("should not have let a zero watch id get here");
        }
        // See if we are cancelling an existing watch
        let rtk = RecordTableKey { key };
        let mut is_empty = false;
        let mut ret = false;
        if let Some(watch_list) = self.watched_records.get_mut(&rtk) {
            let mut dead_watcher = None;
            for (wn, w) in watch_list.watches.iter_mut().enumerate() {
                // Must match the watch id and the watcher key to cancel
                if w.id == watch_id && w.params.watcher == watcher {
                    // Canceling an existing watch
                    dead_watcher = Some(wn);
                    ret = true;
                    break;
                }
            }
            if let Some(dw) = dead_watcher {
                watch_list.watches.remove(dw);
                if watch_list.watches.is_empty() {
                    is_empty = true;
                }
            }
        }
        if is_empty {
            self.watched_records.remove(&rtk);
        }

        Ok(ret)
    }

    /// Move watches from one store to another
    pub fn move_watches(
        &mut self,
        key: TypedKey,
        in_watch: Option<(WatchList, bool)>,
    ) -> Option<(WatchList, bool)> {
        let rtk = RecordTableKey { key };
        let out = self.watched_records.remove(&rtk);
        if let Some(in_watch) = in_watch {
            self.watched_records.insert(rtk, in_watch.0);
            if in_watch.1 {
                self.changed_watched_values.insert(rtk);
            }
        }
        let is_watched = self.changed_watched_values.remove(&rtk);
        out.map(|r| (r, is_watched))
    }

    /// See if any watched records have expired and clear them out
    pub fn check_watched_records(&mut self) {
        let now = get_aligned_timestamp();
        self.watched_records.retain(|key, watch_list| {
            watch_list.watches.retain(|w| {
                w.params.count != 0 && w.params.expiration > now && !w.params.subkeys.is_empty()
            });
            if watch_list.watches.is_empty() {
                // If we're removing the watched record, drop any changed watch values too
                self.changed_watched_values.remove(key);
                false
            } else {
                true
            }
        });
    }

    pub async fn take_value_changes(&mut self, changes: &mut Vec<ValueChangedInfo>) {
        // ValueChangedInfo but without the subkey data that requires a double mutable borrow to get
        struct EarlyValueChangedInfo {
            target: Target,
            key: TypedKey,
            subkeys: ValueSubkeyRangeSet,
            count: u32,
            watch_id: u64,
        }

        let mut evcis = vec![];
        let mut empty_watched_records = vec![];
        for rtk in self.changed_watched_values.drain() {
            if let Some(watch) = self.watched_records.get_mut(&rtk) {
                // Process watch notifications
                let mut dead_watchers = vec![];
                for (wn, w) in watch.watches.iter_mut().enumerate() {
                    // Get the subkeys that have changed
                    let subkeys = w.changed.clone();

                    // If no subkeys on this watcher have changed then skip it
                    if subkeys.is_empty() {
                        continue;
                    }

                    w.changed.clear();

                    // Reduce the count of changes sent
                    // if count goes to zero mark this watcher dead
                    w.params.count -= 1;
                    let count = w.params.count;
                    if count == 0 {
                        dead_watchers.push(wn);
                    }

                    evcis.push(EarlyValueChangedInfo {
                        target: w.params.target,
                        key: rtk.key,
                        subkeys,
                        count,
                        watch_id: w.id,
                    });
                }

                // Remove in reverse so we don't have to offset the index to remove the right key
                for dw in dead_watchers.iter().rev().copied() {
                    watch.watches.remove(dw);
                    if watch.watches.is_empty() {
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
            let get_result = match self.get_subkey(evci.key, first_subkey, false).await {
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
            let Some(value) = get_result.opt_value else {
                log_stor!(error "first subkey should have had value for value change notification");
                continue;
            };

            changes.push(ValueChangedInfo {
                target: evci.target,
                key: evci.key,
                subkeys: evci.subkeys,
                count: evci.count,
                watch_id: evci.watch_id,
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
        let record_info = self
            .peek_record(key, |r| format!("{:#?}", r))
            .unwrap_or("Not found".to_owned());
        let watched_record = match self.watched_records.get(&RecordTableKey { key }) {
            Some(w) => {
                format!("Remote Watches: {:#?}", w)
            }
            None => "No remote watches".to_owned(),
        };
        format!("{}\n{}\n", record_info, watched_record)
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
