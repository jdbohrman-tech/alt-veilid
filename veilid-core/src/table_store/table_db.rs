use crate::*;

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        use keyvaluedb_web::*;
        use keyvaluedb::*;
    } else {
        use keyvaluedb_sqlite::*;
        use keyvaluedb::*;
    }
}

struct CryptInfo {
    vcrypto: CryptoSystemVersion,
    key: SharedSecret,
}
impl CryptInfo {
    pub fn new(crypto: Crypto, typed_key: TypedSharedSecret) -> Self {
        let vcrypto = crypto.get(typed_key.kind).unwrap();
        let key = typed_key.value;
        Self { vcrypto, key }
    }
}

pub struct TableDBUnlockedInner {
    table: String,
    table_store: TableStore,
    crypto: Crypto,
    database: Database,
    // Encryption and decryption key will be the same unless configured for an in-place migration
    encrypt_info: Option<CryptInfo>,
    decrypt_info: Option<CryptInfo>,
}

impl fmt::Debug for TableDBUnlockedInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TableDBInner(table={})", self.table)
    }
}

impl Drop for TableDBUnlockedInner {
    fn drop(&mut self) {
        self.table_store.on_table_db_drop(self.table.clone());
    }
}

#[derive(Debug, Clone)]
pub struct TableDB {
    unlocked_inner: Arc<TableDBUnlockedInner>,
}

impl TableDB {
    pub(super) fn new(
        table: String,
        table_store: TableStore,
        crypto: Crypto,
        database: Database,
        encryption_key: Option<TypedSharedSecret>,
        decryption_key: Option<TypedSharedSecret>,
    ) -> Self {
        let encrypt_info = encryption_key.map(|ek| CryptInfo::new(crypto.clone(), ek));
        let decrypt_info = dcryption_key.map(|dk| CryptInfo::new(crypto.clone(), dk));

        Self {
            unlocked_inner: Arc::new(TableDBUnlockedInner {
                table,
                table_store,
                crypto,
                database,
                encrypt_info,
                decrypt_info,
            }),
        }
    }

    pub(super) fn try_new_from_weak_inner(weak_inner: Weak<TableDBUnlockedInner>) -> Option<Self> {
        weak_inner.upgrade().map(|table_db_unlocked_inner| Self {
            unlocked_inner: table_db_unlocked_inner,
        })
    }

    pub(super) fn weak_inner(&self) -> Weak<TableDBUnlockedInner> {
        Arc::downgrade(&self.unlocked_inner)
    }

    /// Get the total number of columns in the TableDB
    pub fn get_column_count(&self) -> VeilidAPIResult<u32> {
        let db = &self.unlocked_inner.database;
        db.num_columns().map_err(VeilidAPIError::from)
    }

    fn maybe_encrypt(&self, data: &[u8]) -> Vec<u8> {
        if let Some(ei) = &self.unlocked_inner.encrypt_info {
            let mut out = unsafe { unaligned_u8_vec_uninit(NONCE_LENGTH + data.len()) };
            random_bytes(&mut out[0..NONCE_LENGTH]);

            ei.vcrypto.crypt_b2b_no_auth(
                data,
                &mut out[NONCE_LENGTH..],
                &out[0..NONCE_LENGTH],
                &ei.key,
            );
            out
        } else {
            data.to_vec()
        }
    }

    fn maybe_decrypt(&self, data: &[u8]) -> VeilidAPIResult<Vec<u8>> {
        if let Some(di) = &self.unlocked_inner.decrypt_info {
            if data.len() <= NONCE_LENGTH {
                return Err(VeilidAPIError::internal("data too short"));
            }
            xxxx make decrypt
            let mut out = unsafe { unaligned_u8_vec_uninit(NONCE_LENGTH + data.len()) };
            random_bytes(&mut out[0..NONCE_LENGTH]);

            ei.vcrypto.crypt_b2b_no_auth(
                data,
                &mut out[NONCE_LENGTH..],
                &out[0..NONCE_LENGTH],
                &ei.key,
            );
            out
        } else {
            Ok(data.to_vec())
        }
    }

    /// Get the list of keys in a column of the TableDB
    pub async fn get_keys(&self, col: u32) -> VeilidAPIResult<Vec<Vec<u8>>> {
        let db = self.unlocked_inner.database.clone();
        let mut out: Vec<Box<[u8]>> = Vec::new();
        db.iter(col, None, |kv| {
            out.push(kv.0.clone().into_boxed_slice());
            Ok(Option::<()>::None)
        })
        .await
        .map_err(VeilidAPIError::from)?;
        Ok(out)
    }

    /// Start a TableDB write transaction. The transaction object must be committed or rolled back before dropping.
    pub fn transact(&self) -> TableDBTransaction {
        let dbt = self.unlocked_inner.database.transaction();
        TableDBTransaction::new(self.clone(), dbt)
    }

    /// Store a key with a value in a column in the TableDB. Performs a single transaction immediately.
    pub async fn store(&self, col: u32, key: &[u8], value: &[u8]) -> VeilidAPIResult<()> {
        let db = self.unlocked_inner.database.clone();
        let mut dbt = db.transaction();
        dbt.put(col, key, value);
        db.write(dbt).await.map_err(VeilidAPIError::generic)
    }

    /// Store a key in rkyv format with a value in a column in the TableDB. Performs a single transaction immediately.
    pub async fn store_rkyv<T>(&self, col: u32, key: &[u8], value: &T) -> VeilidAPIResult<()>
    where
        T: RkyvSerialize<DefaultVeilidRkyvSerializer>,
    {
        let v = to_rkyv(value)?;

        let db = self.unlocked_inner.database.clone();
        let mut dbt = db.transaction();
        dbt.put(col, key, v.as_slice());
        db.write(dbt).await.map_err(VeilidAPIError::generic)
    }

    /// Store a key in json format with a value in a column in the TableDB. Performs a single transaction immediately.
    pub async fn store_json<T>(&self, col: u32, key: &[u8], value: &T) -> VeilidAPIResult<()>
    where
        T: serde::Serialize,
    {
        let v = serde_json::to_vec(value).map_err(VeilidAPIError::internal)?;

        let db = self.unlocked_inner.database.clone();
        let mut dbt = db.transaction();
        dbt.put(col, key, v.as_slice());
        db.write(dbt).await.map_err(VeilidAPIError::generic)
    }

    /// Read a key from a column in the TableDB immediately.
    pub async fn load(&self, col: u32, key: &[u8]) -> VeilidAPIResult<Option<Vec<u8>>> {
        let db = self.unlocked_inner.database.clone();
        db.get(col, key).await.map_err(VeilidAPIError::from)
    }

    /// Read an rkyv key from a column in the TableDB immediately
    pub async fn load_rkyv<T>(&self, col: u32, key: &[u8]) -> VeilidAPIResult<Option<T>>
    where
        T: RkyvArchive,
        <T as RkyvArchive>::Archived:
            for<'t> CheckBytes<rkyv::validation::validators::DefaultValidator<'t>>,
        <T as RkyvArchive>::Archived: RkyvDeserialize<T, VeilidSharedDeserializeMap>,
    {
        let out = match self.load(col, key).await? {
            Some(v) => Some(from_rkyv(v)?),
            None => None,
        };
        Ok(out)
    }

    /// Read an serde-json key from a column in the TableDB immediately
    pub async fn load_json<T>(&self, col: u32, key: &[u8]) -> VeilidAPIResult<Option<T>>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        let out = match self.load(col, key).await? {
            Some(v) => Some(serde_json::from_slice(&v).map_err(VeilidAPIError::internal)?),
            None => None,
        };
        Ok(out)
    }

    /// Delete key with from a column in the TableDB
    pub async fn delete(&self, col: u32, key: &[u8]) -> VeilidAPIResult<Option<Vec<u8>>> {
        let db = self.unlocked_inner.database.clone();
        let old_value = db.delete(col, key).await.map_err(VeilidAPIError::from)?;
        Ok(old_value)
    }

    /// Delete rkyv key with from a column in the TableDB
    pub async fn delete_rkyv<T>(&self, col: u32, key: &[u8]) -> VeilidAPIResult<Option<T>>
    where
        T: RkyvArchive,
        <T as RkyvArchive>::Archived:
            for<'t> CheckBytes<rkyv::validation::validators::DefaultValidator<'t>>,
        <T as RkyvArchive>::Archived: RkyvDeserialize<T, VeilidSharedDeserializeMap>,
    {
        let db = self.unlocked_inner.database.clone();
        let old_value = match db.delete(col, key).await.map_err(VeilidAPIError::from)? {
            Some(v) => Some(from_rkyv(v)?),
            None => None,
        };
        Ok(old_value)
    }

    /// Delete serde-json key with from a column in the TableDB
    pub async fn delete_json<T>(&self, col: u32, key: &[u8]) -> VeilidAPIResult<Option<T>>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        let db = self.unlocked_inner.database.clone();
        let old_value = match db.delete(col, key).await.map_err(VeilidAPIError::from)? {
            Some(v) => Some(serde_json::from_slice(&v).map_err(VeilidAPIError::internal)?),
            None => None,
        };
        Ok(old_value)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct TableDBTransactionInner {
    dbt: Option<DBTransaction>,
}

impl fmt::Debug for TableDBTransactionInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TableDBTransactionInner({})",
            match &self.dbt {
                Some(dbt) => format!("len={}", dbt.ops.len()),
                None => "".to_owned(),
            }
        )
    }
}

/// A TableDB transaction
/// Atomically commits a group of writes or deletes to the TableDB
#[derive(Debug, Clone)]
pub struct TableDBTransaction {
    db: TableDB,
    inner: Arc<Mutex<TableDBTransactionInner>>,
}

impl TableDBTransaction {
    fn new(db: TableDB, dbt: DBTransaction) -> Self {
        Self {
            db,
            inner: Arc::new(Mutex::new(TableDBTransactionInner { dbt: Some(dbt) })),
        }
    }

    /// Commit the transaction. Performs all actions atomically.
    pub async fn commit(self) -> VeilidAPIResult<()> {
        let dbt = {
            let mut inner = self.inner.lock();
            inner
                .dbt
                .take()
                .ok_or_else(|| VeilidAPIError::generic("transaction already completed"))?
        };
        let db = self.db.unlocked_inner.database.clone();
        db.write(dbt)
            .await
            .map_err(|e| VeilidAPIError::generic(format!("commit failed, transaction lost: {}", e)))
    }

    /// Rollback the transaction. Does nothing to the TableDB.
    pub fn rollback(self) {
        let mut inner = self.inner.lock();
        inner.dbt = None;
    }

    /// Store a key with a value in a column in the TableDB
    pub fn store(&self, col: u32, key: &[u8], value: &[u8]) {
        let mut inner = self.inner.lock();
        inner.dbt.as_mut().unwrap().put(col, key, value);
    }

    /// Store a key in rkyv format with a value in a column in the TableDB
    pub fn store_rkyv<T>(&self, col: u32, key: &[u8], value: &T) -> VeilidAPIResult<()>
    where
        T: RkyvSerialize<DefaultVeilidRkyvSerializer>,
    {
        let v = to_rkyv(value)?;
        let mut inner = self.inner.lock();
        inner.dbt.as_mut().unwrap().put(col, key, v.as_slice());
        Ok(())
    }

    /// Store a key in rkyv format with a value in a column in the TableDB
    pub fn store_json<T>(&self, col: u32, key: &[u8], value: &T) -> VeilidAPIResult<()>
    where
        T: serde::Serialize,
    {
        let v = serde_json::to_vec(value).map_err(VeilidAPIError::internal)?;
        let mut inner = self.inner.lock();
        inner.dbt.as_mut().unwrap().put(col, key, v.as_slice());
        Ok(())
    }

    /// Delete key with from a column in the TableDB
    pub fn delete(&self, col: u32, key: &[u8]) {
        let mut inner = self.inner.lock();
        inner.dbt.as_mut().unwrap().delete(col, key);
    }
}

impl Drop for TableDBTransactionInner {
    fn drop(&mut self) {
        if self.dbt.is_some() {
            warn!("Dropped transaction without commit or rollback");
        }
    }
}
