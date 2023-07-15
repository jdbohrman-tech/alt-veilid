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
    opened_column_count: u32,
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
        opened_column_count: u32,
    ) -> Self {
        let encrypt_info = encryption_key.map(|ek| CryptInfo::new(crypto.clone(), ek));
        let decrypt_info = decryption_key.map(|dk| CryptInfo::new(crypto.clone(), dk));

        Self {
            opened_column_count,
            unlocked_inner: Arc::new(TableDBUnlockedInner {
                table,
                table_store,
                database,
                encrypt_info,
                decrypt_info,
            }),
        }
    }

    pub(super) fn try_new_from_weak_inner(
        weak_inner: Weak<TableDBUnlockedInner>,
        opened_column_count: u32,
    ) -> Option<Self> {
        weak_inner.upgrade().map(|table_db_unlocked_inner| Self {
            opened_column_count,
            unlocked_inner: table_db_unlocked_inner,
        })
    }

    pub(super) fn weak_inner(&self) -> Weak<TableDBUnlockedInner> {
        Arc::downgrade(&self.unlocked_inner)
    }

    /// Get the total number of columns in the TableDB
    /// Not the number of columns that were opened, rather the total number that could be opened
    pub fn get_column_count(&self) -> VeilidAPIResult<u32> {
        let db = &self.unlocked_inner.database;
        db.num_columns().map_err(VeilidAPIError::from)
    }

    /// Encrypt buffer using encrypt key and prepend nonce to output
    /// Keyed nonces are unique because keys must be unique
    /// Normally they must be sequential or random, but the critical
    /// requirement is that they are different for each encryption
    /// but if the contents are guaranteed to be unique, then a nonce
    /// can be generated from the hash of the contents and the encryption key itself
    fn maybe_encrypt(&self, data: &[u8], keyed_nonce: bool) -> Vec<u8> {
        if let Some(ei) = &self.unlocked_inner.encrypt_info {
            let mut out = unsafe { unaligned_u8_vec_uninit(NONCE_LENGTH + data.len()) };

            if keyed_nonce {
                // Key content nonce
                let mut noncedata = Vec::with_capacity(data.len() + PUBLIC_KEY_LENGTH);
                noncedata.extend_from_slice(data);
                noncedata.extend_from_slice(&ei.key.bytes);
                let noncehash = ei.vcrypto.generate_hash(&noncedata);
                out[0..NONCE_LENGTH].copy_from_slice(&noncehash[0..NONCE_LENGTH])
            } else {
                // Random nonce
                random_bytes(&mut out[0..NONCE_LENGTH]);
            }

            let (nonce, encout) = out.split_at_mut(NONCE_LENGTH);
            ei.vcrypto.crypt_b2b_no_auth(
                data,
                encout,
                (nonce as &[u8]).try_into().unwrap(),
                &ei.key,
            );
            out
        } else {
            data.to_vec()
        }
    }

    /// Decrypt buffer using decrypt key with nonce prepended to input
    fn maybe_decrypt(&self, data: &[u8]) -> Vec<u8> {
        if let Some(di) = &self.unlocked_inner.decrypt_info {
            assert!(data.len() >= NONCE_LENGTH);
            if data.len() == NONCE_LENGTH {
                return Vec::new();
            }

            let mut out = unsafe { unaligned_u8_vec_uninit(data.len() - NONCE_LENGTH) };

            di.vcrypto.crypt_b2b_no_auth(
                &data[NONCE_LENGTH..],
                &mut out,
                (&data[0..NONCE_LENGTH]).try_into().unwrap(),
                &di.key,
            );
            out
        } else {
            data.to_vec()
        }
    }

    /// Get the list of keys in a column of the TableDAB
    pub async fn get_keys(&self, col: u32) -> VeilidAPIResult<Vec<Vec<u8>>> {
        if col >= self.opened_column_count {
            apibail_generic!(format!(
                "Column exceeds opened column count {} >= {}",
                col, self.opened_column_count
            ));
        }
        let db = self.unlocked_inner.database.clone();
        let mut out = Vec::new();
        db.iter_keys(col, None, |k| {
            out.push(self.maybe_decrypt(k));
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
        if col >= self.opened_column_count {
            apibail_generic!(format!(
                "Column exceeds opened column count {} >= {}",
                col, self.opened_column_count
            ));
        }
        let db = self.unlocked_inner.database.clone();
        let mut dbt = db.transaction();
        dbt.put(
            col,
            self.maybe_encrypt(key, true),
            self.maybe_encrypt(value, false),
        );
        db.write(dbt).await.map_err(VeilidAPIError::generic)
    }

    /// Store a key in json format with a value in a column in the TableDB. Performs a single transaction immediately.
    pub async fn store_json<T>(&self, col: u32, key: &[u8], value: &T) -> VeilidAPIResult<()>
    where
        T: serde::Serialize,
    {
        let value = serde_json::to_vec(value).map_err(VeilidAPIError::internal)?;
        self.store(col, key, &value).await
    }

    /// Read a key from a column in the TableDB immediately.
    pub async fn load(&self, col: u32, key: &[u8]) -> VeilidAPIResult<Option<Vec<u8>>> {
        if col >= self.opened_column_count {
            apibail_generic!(format!(
                "Column exceeds opened column count {} >= {}",
                col, self.opened_column_count
            ));
        }
        let db = self.unlocked_inner.database.clone();
        let key = self.maybe_encrypt(key, true);
        Ok(db
            .get(col, &key)
            .await
            .map_err(VeilidAPIError::from)?
            .map(|v| self.maybe_decrypt(&v)))
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
        if col >= self.opened_column_count {
            apibail_generic!(format!(
                "Column exceeds opened column count {} >= {}",
                col, self.opened_column_count
            ));
        }
        let key = self.maybe_encrypt(key, true);

        let db = self.unlocked_inner.database.clone();
        let old_value = db
            .delete(col, &key)
            .await
            .map_err(VeilidAPIError::from)?
            .map(|v| self.maybe_decrypt(&v));
        Ok(old_value)
    }

    /// Delete serde-json key with from a column in the TableDB
    pub async fn delete_json<T>(&self, col: u32, key: &[u8]) -> VeilidAPIResult<Option<T>>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        let old_value = match self.delete(col, key).await? {
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
    pub fn store(&self, col: u32, key: &[u8], value: &[u8]) -> VeilidAPIResult<()> {
        if col >= self.db.opened_column_count {
            apibail_generic!(format!(
                "Column exceeds opened column count {} >= {}",
                col, self.db.opened_column_count
            ));
        }

        let key = self.db.maybe_encrypt(key, true);
        let value = self.db.maybe_encrypt(value, false);
        let mut inner = self.inner.lock();
        inner.dbt.as_mut().unwrap().put_owned(col, key, value);
        Ok(())
    }

    /// Store a key in json format with a value in a column in the TableDB
    pub fn store_json<T>(&self, col: u32, key: &[u8], value: &T) -> VeilidAPIResult<()>
    where
        T: serde::Serialize,
    {
        let value = serde_json::to_vec(value).map_err(VeilidAPIError::internal)?;
        self.store(col, key, &value)
    }

    /// Delete key with from a column in the TableDB
    pub fn delete(&self, col: u32, key: &[u8]) -> VeilidAPIResult<()> {
        if col >= self.db.opened_column_count {
            apibail_generic!(format!(
                "Column exceeds opened column count {} >= {}",
                col, self.db.opened_column_count
            ));
        }

        let key = self.db.maybe_encrypt(key, true);
        let mut inner = self.inner.lock();
        inner.dbt.as_mut().unwrap().delete_owned(col, key);
        Ok(())
    }
}

impl Drop for TableDBTransactionInner {
    fn drop(&mut self) {
        if self.dbt.is_some() {
            warn!("Dropped transaction without commit or rollback");
        }
    }
}
