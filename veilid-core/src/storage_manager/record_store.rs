/// RecordStore
/// Keeps an LRU cache of dht keys and their associated subkey valuedata.
/// Instances of this store are used for 'local' (persistent) and 'remote' (ephemeral) dht key storage.
/// This store does not perform any validation on the schema, and all ValueRecordData passed in must have been previously validated.
/// Uses an in-memory store for the records, backed by the TableStore. Subkey data is LRU cached and rotated out by a limits policy,
/// and backed to the TableStore for persistence.
use super::*;
use hashlink::LruCache;

pub struct RecordStore<D>
where
    D: Clone + RkyvArchive + RkyvSerialize<RkyvSerializer>,
    for<'t> <D as RkyvArchive>::Archived: CheckBytes<RkyvDefaultValidator<'t>>,
    <D as RkyvArchive>::Archived: RkyvDeserialize<D, SharedDeserializeMap>,
{
    table_store: TableStore,
    name: String,
    limits: RecordStoreLimits,

    record_table: Option<TableDB>,
    subkey_table: Option<TableDB>,
    record_index: LruCache<RecordTableKey, Record<D>>,
    subkey_cache: LruCache<SubkeyTableKey, RecordData>,
    subkey_cache_total_size: usize,
    total_storage_space: usize,

    dead_records: Vec<(RecordTableKey, Record<D>)>,
    changed_records: HashSet<RecordTableKey>,

    purge_dead_records_mutex: Arc<AsyncMutex<()>>,
}

/// The result of the do_get_value_operation
#[derive(Default, Debug)]
pub struct SubkeyResult {
    /// The subkey value if we got one
    pub value: Option<SignedValueData>,
    /// The descriptor if we got a fresh one or empty if no descriptor was needed
    pub descriptor: Option<SignedValueDescriptor>,
}

impl<D> RecordStore<D>
where
    D: Clone + RkyvArchive + RkyvSerialize<RkyvSerializer>,
    for<'t> <D as RkyvArchive>::Archived: CheckBytes<RkyvDefaultValidator<'t>>,
    <D as RkyvArchive>::Archived: RkyvDeserialize<D, SharedDeserializeMap>,
{
    pub fn new(table_store: TableStore, name: &str, limits: RecordStoreLimits) -> Self {
        let subkey_cache_size = limits.subkey_cache_size as usize;
        Self {
            table_store,
            name: name.to_owned(),
            limits,
            record_table: None,
            subkey_table: None,
            record_index: LruCache::new(limits.max_records.unwrap_or(usize::MAX)),
            subkey_cache: LruCache::new(subkey_cache_size),
            subkey_cache_total_size: 0,
            total_storage_space: 0,
            dead_records: Vec::new(),
            changed_records: HashSet::new(),
            purge_dead_records_mutex: Arc::new(AsyncMutex::new(())),
        }
    }

    pub async fn init(&mut self) -> EyreResult<()> {
        let record_table = self
            .table_store
            .open(&format!("{}_records", self.name), 1)
            .await?;
        let subkey_table = self
            .table_store
            .open(&&format!("{}_subkeys", self.name), 1)
            .await?;

        // Pull record index from table into a vector to ensure we sort them
        let record_table_keys = record_table.get_keys(0)?;
        let mut record_index_saved: Vec<(RecordTableKey, Record<D>)> =
            Vec::with_capacity(record_table_keys.len());
        for rtk in record_table_keys {
            if let Some(vr) = record_table.load_rkyv::<Record<D>>(0, &rtk)? {
                let rik = RecordTableKey::try_from(rtk.as_ref())?;
                record_index_saved.push((rik, vr));
            }
        }

        // Sort the record index by last touched time and insert in sorted order
        record_index_saved.sort_by(|a, b| a.1.last_touched().cmp(&b.1.last_touched()));
        let mut dead_records = Vec::new();
        for ri in record_index_saved {
            // total the storage space
            self.total_storage_space += mem::size_of::<RecordTableKey>();
            self.total_storage_space += ri.1.total_size();

            // add to index and ensure we deduplicate in the case of an error
            if let Some(v) = self.record_index.insert(ri.0, ri.1, |k, v| {
                // If the configuration change, we only want to keep the 'limits.max_records' records
                dead_records.push((k, v));
            }) {
                // This shouldn't happen, but deduplicate anyway
                log_stor!(warn "duplicate record in table: {:?}", ri.0);
                dead_records.push((ri.0, v));
            }
        }
        for (k, v) in dead_records {
            self.add_dead_record(k, v);
        }

        self.record_table = Some(record_table);
        self.subkey_table = Some(subkey_table);
        Ok(())
    }

    fn add_dead_record(&mut self, key: RecordTableKey, record: Record<D>) {
        self.dead_records.push((key, record));
    }

    fn mark_record_changed(&mut self, key: RecordTableKey) {
        self.changed_records.insert(key);
    }

    fn add_to_subkey_cache(&mut self, key: SubkeyTableKey, record_data: RecordData) {
        let record_data_total_size = record_data.total_size();
        // Write to subkey cache
        let mut dead_size = 0usize;
        if let Some(old_record_data) = self.subkey_cache.insert(key, record_data, |_, v| {
            // LRU out
            dead_size += v.total_size();
        }) {
            // Old data
            dead_size += old_record_data.total_size();
        }
        self.subkey_cache_total_size -= dead_size;
        self.subkey_cache_total_size += record_data_total_size;

        // Purge over size limit
        if let Some(max_subkey_cache_memory_mb) = self.limits.max_subkey_cache_memory_mb {
            while self.subkey_cache_total_size > (max_subkey_cache_memory_mb * 1_048_576usize) {
                if let Some((_, v)) = self.subkey_cache.remove_lru() {
                    self.subkey_cache_total_size -= v.total_size();
                } else {
                    break;
                }
            }
        }
    }

    fn remove_from_subkey_cache(&mut self, key: SubkeyTableKey) {
        if let Some(dead_record_data) = self.subkey_cache.remove(&key) {
            self.subkey_cache_total_size -= dead_record_data.total_size();
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
        for (k, v) in dead_records {
            // Record should already be gone from index
            if self.record_index.contains_key(&k) {
                log_stor!(error "dead record found in index: {:?}", k);
            }

            // Delete record
            rt_xact.delete(0, &k.bytes());

            // Delete subkeys
            let subkey_count = v.subkey_count() as u32;
            for sk in 0..subkey_count {
                // From table
                let stk = SubkeyTableKey {
                    key: k.key,
                    subkey: sk,
                };
                st_xact.delete(0, &stk.bytes());

                // From cache
                self.remove_from_subkey_cache(stk);
            }

            // Remove from total size
            self.total_storage_space -= mem::size_of::<RecordTableKey>();
            self.total_storage_space -= v.total_size();
        }
        if let Err(e) = rt_xact.commit().await {
            log_stor!(error "failed to commit record table transaction: {}", e);
        }
        if let Err(e) = st_xact.commit().await {
            log_stor!(error "failed to commit subkey table transaction: {}", e);
        }
    }

    async fn flush_changed_records(&mut self) {
        // touch records
        if self.changed_records.is_empty() {
            return;
        }

        let record_table = self.record_table.clone().unwrap();

        let rt_xact = record_table.transact();
        let changed_records = mem::take(&mut self.changed_records);
        for rtk in changed_records {
            // Get the changed record and save it to the table
            if let Some(r) = self.record_index.peek(&rtk) {
                if let Err(e) = rt_xact.store_rkyv(0, &rtk.bytes(), r) {
                    log_stor!(error "failed to save record: {}", e);
                }
            }
        }
        if let Err(e) = rt_xact.commit().await {
            log_stor!(error "failed to commit record table transaction: {}", e);
        }
    }

    pub async fn tick(&mut self) -> EyreResult<()> {
        self.flush_changed_records().await;
        self.purge_dead_records(true).await;
        Ok(())
    }

    pub async fn new_record(
        &mut self,
        key: TypedKey,
        record: Record<D>,
    ) -> Result<(), VeilidAPIError> {
        let rtk = RecordTableKey { key };
        if self.record_index.contains_key(&rtk) {
            apibail_internal!("record already exists");
        }

        // Get record table
        let Some(record_table) = self.record_table.clone() else {
            apibail_internal!("record store not initialized");
        };

        // If over size limit, dont create record
        let new_total_storage_space =
            self.total_storage_space + mem::size_of::<RecordTableKey>() + record.total_size();
        if let Some(max_storage_space_mb) = &self.limits.max_storage_space_mb {
            if new_total_storage_space > (max_storage_space_mb * 1_048_576usize) {
                apibail_try_again!();
            }
        }

        // Save to record table
        record_table
            .store_rkyv(0, &rtk.bytes(), &record)
            .await
            .map_err(VeilidAPIError::internal)?;

        // Save to record index
        let mut dead_records = Vec::new();
        if let Some(v) = self.record_index.insert(rtk, record, |k, v| {
            dead_records.push((k, v));
        }) {
            // Shouldn't happen but log it
            log_stor!(warn "new duplicate record in table: {:?}", rtk);
            self.add_dead_record(rtk, v);
        }
        for dr in dead_records {
            self.add_dead_record(dr.0, dr.1);
        }

        // Update storage space
        self.total_storage_space = new_total_storage_space;

        Ok(())
    }

    pub async fn delete_record(&mut self, key: TypedKey) -> Result<(), VeilidAPIError> {
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
            self.mark_record_changed(rtk);
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
            self.mark_record_changed(rtk);
        }

        out
    }

    // pub fn get_descriptor(&mut self, key: TypedKey) -> Option<SignedValueDescriptor> {
    //     self.with_record(key, |record| record.descriptor().clone())
    // }

    pub fn get_subkey(
        &mut self,
        key: TypedKey,
        subkey: ValueSubkey,
        want_descriptor: bool,
    ) -> Result<Option<SubkeyResult>, VeilidAPIError> {
        // record from index
        let Some((subkey_count, opt_descriptor)) = self.with_record(key, |record| {
            (record.subkey_count(), if want_descriptor {
                Some(record.descriptor().clone())
            } else {
                None
            })
        }) else {
            // Record not available
            return Ok(None);
        };

        // Check if the subkey is in range
        if subkey as usize >= subkey_count {
            apibail_invalid_argument!("subkey out of range", "subkey", subkey);
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
        // If not in cache, try to pull from table store
        if let Some(record_data) = subkey_table
            .load_rkyv::<RecordData>(0, &stk.bytes())
            .map_err(VeilidAPIError::internal)?
        {
            let out = record_data.signed_value_data().clone();

            // Add to cache, do nothing with lru out
            self.add_to_subkey_cache(stk, record_data);

            return Ok(Some(SubkeyResult {
                value: Some(out),
                descriptor: opt_descriptor,
            }));
        };

        // Record was available, but subkey was not found, maybe descriptor gets returned
        Ok(Some(SubkeyResult {
            value: None,
            descriptor: opt_descriptor,
        }))
    }

    pub async fn set_subkey(
        &mut self,
        key: TypedKey,
        subkey: ValueSubkey,
        signed_value_data: SignedValueData,
    ) -> Result<(), VeilidAPIError> {
        // Check size limit for data
        if signed_value_data.value_data().data().len() > self.limits.max_subkey_size {
            return Err(VeilidAPIError::generic("record subkey too large"));
        }

        // Get record from index
        let Some((subkey_count, total_size)) = self.with_record(key, |record| {
            (record.subkey_count(), record.total_size())
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
        let mut prior_record_data_size = 0usize;

        // If subkey exists in subkey cache, use that
        let stk = SubkeyTableKey { key, subkey };
        let stk_bytes = stk.bytes();

        if let Some(record_data) = self.subkey_cache.peek(&stk) {
            prior_record_data_size = record_data.total_size();
        } else {
            // If not in cache, try to pull from table store
            if let Some(record_data) = subkey_table
                .load_rkyv::<RecordData>(0, &stk_bytes)
                .map_err(VeilidAPIError::internal)?
            {
                prior_record_data_size = record_data.total_size();
            }
        }

        // Make new record data
        let record_data = RecordData::new(signed_value_data);

        // Check new total record size
        let new_record_data_size = record_data.total_size();
        let new_total_size = total_size + new_record_data_size - prior_record_data_size;
        if new_total_size > self.limits.max_record_total_size {
            apibail_generic!("dht record too large");
        }

        // Check new total storage space
        let new_total_storage_space =
            self.total_storage_space + new_record_data_size - prior_record_data_size;
        if let Some(max_storage_space_mb) = self.limits.max_storage_space_mb {
            if new_total_storage_space > (max_storage_space_mb * 1_048_576usize) {
                apibail_try_again!();
            }
        }

        // Write subkey
        subkey_table
            .store_rkyv(0, &stk_bytes, &record_data)
            .await
            .map_err(VeilidAPIError::internal)?;

        // Write to subkey cache
        self.add_to_subkey_cache(stk, record_data);

        // Update record
        self.with_record_mut(key, |record| {
            record.set_record_data_size(new_record_data_size);
        })
        .expect("record should still be here");

        Ok(())
    }

    /// LRU out some records until we reclaim the amount of space requested
    /// This will force a garbage collection of the space immediately
    /// If zero is passed in here, a garbage collection will be performed of dead records
    /// without removing any live records
    pub async fn reclaim_space(&mut self, space: usize) {
        let mut reclaimed = 0usize;
        while reclaimed < space {
            if let Some((k, v)) = self.record_index.remove_lru() {
                reclaimed += mem::size_of::<RecordTableKey>();
                reclaimed += v.total_size();
                self.add_dead_record(k, v);
            }
        }
        self.purge_dead_records(false).await;
    }
}
