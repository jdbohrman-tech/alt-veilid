/// RecordStore
/// Keeps an LRU cache of dht keys and their associated subkey valuedata.
/// Instances of this store are used for 'local' (persistent) and 'remote' (ephemeral) dht key storage.
/// This store does not perform any validation on the schema, and all ValueRecordData passed in must have been previously validated.
/// Uses an in-memory store for the records, backed by the TableStore. Subkey data is LRU cached and rotated out by a limits policy,
/// and backed to the TableStore for persistence.
use super::*;
use hashlink::LruCache;

pub struct RecordStore {
    table_store: TableStore,
    name: String,
    limits: RecordStoreLimits,

    record_table: Option<TableDB>,
    subkey_table: Option<TableDB>,
    record_index: LruCache<RecordTableKey, ValueRecord>,
    subkey_cache: LruCache<SubkeyTableKey, ValueRecordData>,

    dead_records: Vec<(RecordTableKey, ValueRecord)>,
    changed_records: HashSet<(RecordTableKey, Timestamp)>,
}

impl RecordStore {
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
            dead_records: Vec::new(),
            changed_records: HashSet::new(),
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
        let mut record_index_saved: Vec<(RecordTableKey, ValueRecord)> =
            Vec::with_capacity(record_table_keys.len());
        for rtk in record_table_keys {
            if let Some(vr) = record_table.load_rkyv::<ValueRecord>(0, &rtk)? {
                let rik = RecordTableKey::try_from(rtk.as_ref())?;
                record_index_saved.push((rik, vr));
            }
        }

        // Sort the record index by last touched time and insert in sorted order
        record_index_saved.sort_by(|a, b| a.1.last_touched().cmp(&b.1.last_touched()));
        let mut dead_records = Vec::new();
        for ri in record_index_saved {
            self.record_index.insert(ri.0, ri.1, |k, v| {
                // If the configuration change, we only want to keep the 'limits.max_records' records
                dead_records.push((k, v));
            });
        }

        self.record_table = Some(record_table);
        self.subkey_table = Some(subkey_table);
        Ok(())
    }

    fn add_dead_record(&mut self, key: RecordTableKey, record: ValueRecord) {
        self.dead_records.push((key, record));
    }

    fn mark_record_changed(&mut self, key: RecordTableKey) {
        let cur_ts = get_aligned_timestamp();
        self.changed_records.insert((key, cur_ts));
    }

    async fn purge_dead_records(&mut self) {
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
            // Delete record
            rt_xact.delete(0, &k.bytes());

            // Delete subkeys
            let subkey_count = v.subkey_count() as u32;
            for sk in 0..subkey_count {
                // From table
                let sck = SubkeyTableKey {
                    key: k.key,
                    subkey: sk,
                };
                st_xact.delete(0, &sck.bytes());

                // From cache
                self.subkey_cache.remove(&sck);
            }
        }
        if let Err(e) = rt_xact.commit().await {
            log_stor!(error "failed to commit record table transaction: {}", e);
        }
        if let Err(e) = st_xact.commit().await {
            log_stor!(error "failed to commit subkey table transaction: {}", e);
        }
    }

    async fn flush_records(&mut self) {
        // touch records
        if self.changed_records.empty() {
            return;
        }

        let record_table = self.record_table.clone().unwrap();
        let subkey_table = self.subkey_table.clone().unwrap();

        let rt_xact = record_table.transact();
        let mut changed_records = mem::take(&mut self.changed_records);
        for (rik, ts) in changed_records {
            // Flush changed records
            if let Some(r) = self.record_index.peek(&rik) {
                record_table.store_rkyv(0, &rtk)?;
                xxx
            }
        }
        if let Err(e) = rt_xact.commit().await {
            log_stor!(error "failed to commit record table transaction: {}", e);
        }
    }

    pub async fn tick(&mut self, last_ts: Timestamp, cur_ts: Timestamp) {
        self.flush_records().await;
        self.purge_dead_records().await;
    }

    pub fn new_record(&mut self, key: TypedKey, record: ValueRecord) -> Result<(), VeilidAPIError> {
        let rik = RecordTableKey { key };
        if self.record_index.contains_key(&rik) {
            apibail_generic!("record already exists");
        }

        // Get record table
        let Some(record_table) = self.record_table.clone() else {
            apibail_internal!("record store not initialized");
        };

        // Save to record table
        record_table
            .store_rkyv(0, &rik, &r)
            .await
            .map_err(VeilidAPIError::internal)?;

        // Cache it
        self.record_cache.insert(key, value, |k, v| {
            self.add_dead_record(k, v);
        });

        Ok(())
    }

    pub fn with_record<R, F>(&mut self, key: TypedKey, f: F) -> Option<R>
    where
        F: FnOnce(&ValueRecord) -> R,
    {
        // Get record from index
        let rck = RecordTableKey { key };
        if let Some(r) = self.record_index.get_mut(&rck) {
            // Touch
            r.touch(get_aligned_timestamp());
            self.mark_record_changed(&rck);

            // Callback
            return Some(f(key, r));
        }
        None
    }

    pub fn get_subkey<R, F>(
        &mut self,
        key: TypedKey,
        subkey: ValueSubkey,
    ) -> Result<Option<ValueRecordData>, VeilidAPIError> {
        // record from index
        let rck = RecordTableKey { key };
        let Some(r) = self.record_index.get_mut(&rck) else {
            apibail_invalid_argument!("no record at this key", "key", key);
        };

        // Touch
        r.touch(get_aligned_timestamp());
        self.mark_record_changed(&rck);

        // Check if the subkey is in range
        if subkey >= r.subkey_count() {
            apibail_invalid_argument!("subkey out of range", "subkey", subkey);
        }

        // Get subkey table
        let Some(subkey_table) = self.subkey_table.clone() else {
            apibail_internal!("record store not initialized");
        };

        // If subkey exists in subkey cache, use that
        let skck = SubkeyTableKey { key, subkey };
        if let Some(rd) = self.subkey_cache.get_mut(&skck) {
            let out = rd.clone();

            return Ok(Some(out));
        }
        // If not in cache, try to pull from table store
        let k = skck.bytes();
        if let Some(rd) = subkey_table
            .load_rkyv::<ValueRecordData>(0, &k)
            .map_err(VeilidAPIError::internal)?
        {
            let out = rd.clone();

            // Add to cache, do nothing with lru out
            self.subkey_cache.insert(skck, rd, |_| {});

            return Ok(Some(out));
        };

        return Ok(None);
    }

    pub fn set_subkey<R, F>(
        &mut self,
        key: TypedKey,
        subkey: ValueSubkey,
        data: ValueRecordData,
    ) -> Result<(), VeilidAPIError> {
        // Check size limit for data
        if data.data.len() > self.limits.max_subkey_size {
            return Err(VeilidAPIError::generic("record subkey too large"));
        }

        // Get record from index
        let rck = RecordTableKey { key };
        let Some(r) = self.record_index.get_mut(&rck) else {
            apibail_invalid_argument!("no record at this key", "key", key);
        };

        // Touch
        r.touch(get_aligned_timestamp());
        self.mark_record_changed(&rck);

        // Check if the subkey is in range
        if subkey >= r.subkey_count() {
            apibail_invalid_argument!("subkey out of range", "subkey", subkey);
        }

        // Get subkey table
        let Some(subkey_table) = self.subkey_table.clone() else {
            apibail_internal!("record store not initialized");
        };

        // Get the previous subkey and ensure we aren't going over the record size limit
        let mut prior_subkey_size = 0usize;

        // If subkey exists in subkey cache, use that
        let skck = SubkeyTableKey { key, subkey };
        if let Some(rd) = self.subkey_cache.peek(&skck) {
            prior_subkey_size = rd.data.data().len();
        } else {
            // If not in cache, try to pull from table store
            let k = skck.bytes();
            if let Some(rd) = subkey_table
                .load_rkyv::<ValueRecordData>(0, &k)
                .map_err(VeilidAPIError::internal)?
            {
                prior_subkey_size = rd.data.data().len();
            }
        }

        // Check new data size
        let new_data_size = r.data_size() + data.data().len() - priod_subkey_size;
        if new_data_size > self.limits.max_record_data_size {
            return Err(VeilidAPIError::generic("dht record too large"));
        }

        // Write subkey
        let k = skck.bytes();
        subkey_table.store_rkyv(0, &k, &data)?;

        // Write to subkey cache
        let skck = SubkeyTableKey { key, subkey };
        self.subkey_cache.insert(skck, data, |_, _| {});

        // Update record
        r.set_data_size(new_data_size);

        Ok(())
    }
}
