use super::*;
use hashlink::LruCache;

pub type RecordIndex = u32;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct RecordIndexKey {
    pub key: TypedKey,
}
impl RecordIndexKey {
    pub fn bytes(&self) -> [u8; PUBLIC_KEY_LENGTH + 4] {
        let mut bytes = [0u8; PUBLIC_KEY_LENGTH + 4];
        bytes[0..4] = self.key.kind.0;
        bytes[4..PUBLIC_KEY_LENGTH + 4] = self.key.value.bytes;
        bytes
    }
}

impl TryFrom<&[u8]> for RecordIndexKey {
    type Error = EyreReport;
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() != PUBLIC_KEY_LENGTH + 4 {
            bail!("invalid bytes length");
        }
        let kind = FourCC::try_from(&bytes[0..4]).wrap_err("invalid kind")?;
        let value =
            PublicKey::try_from(&bytes[4..PUBLIC_KEY_LENGTH + 4]).wrap_err("invalid value")?;
        let key = TypedKey::new(kind, value);
        Ok(RecordIndexKey { key })
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct SubkeyCacheKey {
    pub key: TypedKey,
    pub subkey: ValueSubkey,
}
impl SubkeyCacheKey {
    pub fn bytes(&self) -> [u8; PUBLIC_KEY_LENGTH + 4 + 4] {
        let mut bytes = [0u8; PUBLIC_KEY_LENGTH + 4 + 4];
        bytes[0..4] = self.key.kind.0;
        bytes[4..PUBLIC_KEY_LENGTH + 4] = self.key.value.bytes;
        bytes[PUBLIC_KEY_LENGTH + 4..PUBLIC_KEY_LENGTH + 4 + 4] = self.subkey.to_le_bytes();
        bytes
    }
}
impl TryFrom<&[u8]> for SubkeyCacheKey {
    type Error = EyreReport;
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() != PUBLIC_KEY_LENGTH + 4 {
            bail!("invalid bytes length");
        }
        let kind = FourCC::try_from(&bytes[0..4]).wrap_err("invalid kind")?;
        let value =
            PublicKey::try_from(&bytes[4..PUBLIC_KEY_LENGTH + 4]).wrap_err("invalid value")?;
        let subkey =
            ValueSubkey::try_from(&bytes[PUBLIC_KEY_LENGTH + 4..PUBLIC_KEY_LENGTH + 4 + 4])
                .wrap_err("invalid subkey")?;

        let key = TypedKey::new(kind, value);
        Ok(SubkeyCacheKey { key, subkey })
    }
}

pub struct RecordStore {
    table_store: TableStore,
    name: String,
    limits: RecordStoreLimits,

    record_table: Option<TableDB>,
    subkey_table: Option<TableDB>,
    record_index: LruCache<RecordIndexKey, ValueRecord>,
    subkey_cache: LruCache<SubkeyCacheKey, ValueRecordData>,
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
        let mut record_index_saved: Vec<(RecordIndexKey, ValueRecord)> =
            Vec::with_capacity(record_table_keys.len());
        for rtk in record_table_keys {
            if let Some(vr) = record_table.load_rkyv::<ValueRecord>(0, &rtk)? {
                record_index_saved.push((rtk, vr));
            }
        }

        // Sort the record index by last touched time and insert in sorted order
        record_index_saved.sort_by(|a, b| a.1.last_touched().cmp(&b.1.last_touched()));
        let mut dead_records = Vec::new();
        for ri in record_index_saved {
            let rik = RecordIndexKey::try_from(&ri.0)?;
            self.record_index.insert(rik, ri.1, |k, v| {
                // If the configuration change, we only want to keep the 'limits.max_records' records
                dead_records.push((k, v));
            })
        }

        // Delete dead keys
        if !dead_records.empty() {
            let rt_xact = record_table.transact();
            let st_xact = subkey_table.transact();
            for (k, v) in dead_records {
                // Delete record
                rt_xact.delete(0, &k.bytes());

                // Delete subkeys
                let subkey_count = v.subkey_count();
                for sk in 0..subkey_count {
                    let sck = SubkeyCacheKey {
                        key: k.key,
                        subkey: sk,
                    };
                    st_xact.delete(0, &sck.bytes())?;
                }
            }
            rt_xact.commit().await?;
            st_xact.commit().await?;
        }

        self.record_table = Some(record_table);
        self.subkey_table = Some(record_table);
        Ok(())
    }

    fix up new record

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

    pub fn with_record<R, F>(&mut self, key: TypedKey, f: F) -> EyreResult<Option<R>>
    where
        F: FnOnce(&mut RecordStore, TypedKey, &ValueRecord) -> R,
    {
        // Get record table
        let Some(record_table) = self.record_table.clone() else {
            bail!("record store not initialized");
        };

        // If record exists in cache, use that
        let rck = RecordIndexKey { key };
        if let Some(r) = self.record_cache.get(&rck) {
            // Callback
            return Ok(Some(f(self, key, r)));
        }
        // If not in cache, try to pull from table store
        let k = rck.bytes();
        if let Some(r) = record_table.load_rkyv(0, &k)? {
            // Callback
            let out = f(self, key, &r);

            // Add to cache, do nothing with lru out
            self.record_cache.insert(rck, r, |_| {});

            return Ok(Some(out));
        };

        return Ok(None);
    }

    pub fn with_record_mut<R, F>(&mut self, key: TypedKey, f: F) -> EyreResult<Option<R>>
    where
        F: FnOnce(&mut RecordStore, TypedKey, &mut ValueRecord) -> R,
    {
        // Get record table
        let Some(record_table) = self.record_table.clone() else {
            bail!("record store not initialized");
        };

        // If record exists in cache, use that
        let rck = RecordIndexKey { key };
        if let Some(r) = self.record_cache.get_mut(&rck) {
            // Callback
            return Ok(Some(f(self, key, r)));
        }
        // If not in cache, try to pull from table store
        let k = rck.bytes();
        if let Some(r) = record_table.load_rkyv(0, &k)? {
            // Callback
            let out = f(self, key, &mut r);

            // Save changes back to record table
            record_table.store_rkyv(0, &k, &r).await?;

            // Add to cache, do nothing with lru out
            self.record_cache.insert(rck, r, |_| {});

            return Ok(Some(out));
        };

        Ok(None)
    }

    pub fn with_subkey<R, F>(
        &mut self,
        key: TypedKey,
        subkey: ValueSubkey,
        f: F,
    ) -> EyreResult<Option<R>>
    where
        F: FnOnce(&mut RecordStore, TypedKey, ValueSubkey, &ValueRecordData) -> R,
    {
        // Get subkey table
        let Some(subkey_table) = self.subkey_table.clone() else {
            bail!("record store not initialized");
        };

        // If subkey exists in subkey cache, use that
        let skck = SubkeyCacheKey { key, subkey };
        if let Some(rd) = self.subkey_cache.get(&skck) {
            // Callback
            return Ok(Some(f(self, key, subkey, rd)));
        }
        // If not in cache, try to pull from table store
        let k = skck.bytes();
        if let Some(rd) = subkey_table.load_rkyv(0, &k)? {
            // Callback
            let out = f(self, key, subkey, &rd);

            // Add to cache, do nothing with lru out
            self.subkey_cache.insert(skck, r, |_| {});

            return Ok(Some(out));
        };

        return Ok(None);
    }

    pub fn with_subkey_mut<R, F>(
        &mut self,
        key: TypedKey,
        subkey: ValueSubkey,
        f: F,
    ) -> EyreResult<Option<R>>
    where
        F: FnOnce(&mut RecordStore, TypedKey, ValueSubkey, &mut ValueRecord) -> R,
    {
        // Get record table
        let Some(subkey_table) = self.subkey_table.clone() else {
            bail!("record store not initialized");
        };

        // If subkey exists in cache, use that
        let skck = SubkeyCacheKey { key, subkey };
        if let Some(rd) = self.subkey_cache.get_mut(&skck) {
            // Callback
            return Ok(Some(f(self, key, subkey, rd)));
        }
        // If not in cache, try to pull from table store
        let k = skck.bytes();
        if let Some(rd) = subkey_table.load_rkyv(0, &k)? {
            // Callback
            let out = f(self, key, subkey, &mut rd);

            // Save changes back to record table
            subkey_table.store_rkyv(0, &k, &rd).await?;

            // Add to cache, do nothing with lru out
            self.subkey_cache.insert(key, r, |_| {});

            return Ok(Some(out));
        };

        Ok(None)
    }
}
