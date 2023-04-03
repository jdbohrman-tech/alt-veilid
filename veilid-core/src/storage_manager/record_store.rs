use super::*;
use hashlink::LruCache;

pub type RecordIndex = u32;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct RecordCacheKey {
    record_idx: RecordIndex,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct SubkeyCacheKey {
    record_idx: RecordIndex,
    subkey: ValueSubkey,
}

pub struct RecordStore {
    table_store: TableStore,
    name: String,
    limits: RecordStoreLimits,

    record_table: Option<TableDB>,
    subkey_table: Option<TableDB>,
    record_index: HashMap<TypedKey, RecordIndex>,
    free_record_index_list: Vec<RecordIndex>,
    record_cache: LruCache<RecordIndex, ValueRecord>,
    subkey_cache: LruCache<SubkeyCacheKey, ValueRecordData>,
}

impl RecordStore {
    pub fn new(table_store: TableStore, name: &str, limits: RecordStoreLimits) -> Self {
        let record_cache_size = limits.record_cache_size as usize;
        let subkey_cache_size = limits.subkey_cache_size as usize;
        Self {
            table_store,
            name: name.to_owned(),
            limits,
            record_table: None,
            subkey_table: None,
            record_index: HashMap::new(),
            free_record_index_list: Vec::new(), // xxx can this be auto-recovered? should we ever compact the allocated indexes?
            record_cache: LruCache::new(record_cache_size),
            subkey_cache: LruCache::new(subkey_cache_size),
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

        // xxx get record index and free record index list

        self.record_table = Some(record_table);
        self.subkey_table = Some(record_table);
        Ok(())
    }

    fn key_bytes(key: TypedKey) -> [u8; PUBLIC_KEY_LENGTH + 4] {
        let mut bytes = [0u8; PUBLIC_KEY_LENGTH + 4];
        bytes[0..4] = key.kind.0;
        bytes[4..PUBLIC_KEY_LENGTH + 4] = key.value.bytes;
        bytes
    }

    pub fn with_record<R, F: FnOnce(TypedKey, &ValueRecord) -> R>(
        &mut self,
        key: TypedKey,
        f: F,
    ) -> EyreResult<Option<R>> {
        // Get record table
        let Some(record_table) = self.record_table.clone() else {
            bail!("record store not initialized");
        };

        // If record exists in cache, use that
        if let Some(r) = self.record_cache.get(&key) {
            // Callback
            return Ok(Some(f(key, r)));
        }
        // If not in cache, try to pull from table store
        let k = Self::key_bytes(key);
        if let Some(r) = record_table.load_rkyv(0, &k)? {
            // Callback
            let out = f(key, &r);

            // Add to cache, do nothing with lru out
            self.record_cache.insert(key, r, |_| {});

            return Ok(Some(out));
        };

        return Ok(None);
    }

    pub fn with_record_mut<R, F: FnOnce(TypedKey, &mut ValueRecord) -> R>(
        &mut self,
        key: TypedKey,
        f: F,
    ) -> EyreResult<Option<R>> {
        // Get record table
        let Some(record_table) = self.record_table.clone() else {
            bail!("record store not initialized");
        };

        // If record exists in cache, use that
        if let Some(r) = self.record_cache.get_mut(&key) {
            // Callback
            return Ok(Some(f(key, r)));
        }
        // If not in cache, try to pull from table store
        let k = Self::key_bytes(key);
        if let Some(r) = record_table.load_rkyv(0, &k)? {
            // Callback
            let out = f(key, &mut r);

            // Save changes back to record table
            record_table.store_rkyv(0, &k, &r).await?;

            // Add to cache, do nothing with lru out
            self.record_cache.insert(key, r, |_| {});

            return Ok(Some(out));
        };

        Ok(None)
    }

    pub fn new_record(&mut self, key: TypedKey, record: ValueRecord) -> EyreResult<()> {
        if self.with_record(key, |_| {})?.is_some() {
            bail!("record already exists");
        }

        // Get record table
        let Some(record_table) = self.record_table.clone() else {
            bail!("record store not initialized");
        };

        // Save to record table
        record_table.store_rkyv(0, &key, &r).await?;

        // Cache it
        self.record_cache.insert(key, value, |_| {});

        Ok(())
    }
}
