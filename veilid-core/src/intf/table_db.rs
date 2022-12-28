use crate::*;
use rkyv::{Archive as RkyvArchive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        use keyvaluedb_web::*;
        use keyvaluedb::*;
    } else {
        use keyvaluedb_sqlite::*;
        use keyvaluedb::*;
    }
}

pub struct TableDBInner {
    table: String,
    table_store: TableStore,
    database: Database,
}

impl fmt::Debug for TableDBInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TableDBInner(table={})", self.table)
    }
}

impl Drop for TableDBInner {
    fn drop(&mut self) {
        self.table_store.on_table_db_drop(self.table.clone());
    }
}

#[derive(Debug, Clone)]
pub struct TableDB {
    inner: Arc<Mutex<TableDBInner>>,
}

impl TableDB {
    pub(super) fn new(table: String, table_store: TableStore, database: Database) -> Self {
        Self {
            inner: Arc::new(Mutex::new(TableDBInner {
                table,
                table_store,
                database,
            })),
        }
    }

    pub(super) fn try_new_from_weak_inner(weak_inner: Weak<Mutex<TableDBInner>>) -> Option<Self> {
        weak_inner.upgrade().map(|table_db_inner| Self {
            inner: table_db_inner,
        })
    }

    pub(super) fn weak_inner(&self) -> Weak<Mutex<TableDBInner>> {
        Arc::downgrade(&self.inner)
    }

    /// Get the total number of columns in the TableDB
    pub fn get_column_count(&self) -> EyreResult<u32> {
        let db = &self.inner.lock().database;
        db.num_columns().wrap_err("failed to get column count: {}")
    }

    /// Get the list of keys in a column of the TableDB
    pub fn get_keys(&self, col: u32) -> EyreResult<Vec<Box<[u8]>>> {
        let db = &self.inner.lock().database;
        let mut out: Vec<Box<[u8]>> = Vec::new();
        db.iter(col, None, &mut |kv| {
            out.push(kv.0.clone().into_boxed_slice());
            Ok(true)
        })
        .wrap_err("failed to get keys for column")?;
        Ok(out)
    }

    /// Start a TableDB write transaction. The transaction object must be committed or rolled back before dropping.
    pub fn transact(&self) -> TableDBTransaction {
        let dbt = {
            let db = &self.inner.lock().database;
            db.transaction()
        };
        TableDBTransaction::new(self.clone(), dbt)
    }

    /// Store a key with a value in a column in the TableDB. Performs a single transaction immediately.
    pub async fn store(&self, col: u32, key: &[u8], value: &[u8]) -> EyreResult<()> {
        let db = self.inner.lock().database.clone();
        let mut dbt = db.transaction();
        dbt.put(col, key, value);
        db.write(dbt).await.wrap_err("failed to store key")
    }

    /// Store a key in rkyv format with a value in a column in the TableDB. Performs a single transaction immediately.
    pub async fn store_rkyv<T>(&self, col: u32, key: &[u8], value: &T) -> EyreResult<()>
    where
        T: RkyvSerialize<rkyv::ser::serializers::AllocSerializer<1024>>,
    {
        let v = to_rkyv(value)?;

        let db = self.inner.lock().database.clone();
        let mut dbt = db.transaction();
        dbt.put(col, key, v.as_slice());
        db.write(dbt).await.wrap_err("failed to store key")
    }

    /// Store a key in json format with a value in a column in the TableDB. Performs a single transaction immediately.
    pub async fn store_json<T>(&self, col: u32, key: &[u8], value: &T) -> EyreResult<()>
    where
        T: serde::Serialize,
    {
        let v = serde_json::to_vec(value)?;

        let db = self.inner.lock().database.clone();
        let mut dbt = db.transaction();
        dbt.put(col, key, v.as_slice());
        db.write(dbt).await.wrap_err("failed to store key")
    }

    /// Read a key from a column in the TableDB immediately.
    pub fn load(&self, col: u32, key: &[u8]) -> EyreResult<Option<Vec<u8>>> {
        let db = self.inner.lock().database.clone();
        db.get(col, key).wrap_err("failed to get key")
    }

    /// Read an rkyv key from a column in the TableDB immediately
    pub fn load_rkyv<T>(&self, col: u32, key: &[u8]) -> EyreResult<Option<T>>
    where
        T: RkyvArchive,
        <T as RkyvArchive>::Archived:
            for<'t> bytecheck::CheckBytes<rkyv::validation::validators::DefaultValidator<'t>>,
        <T as RkyvArchive>::Archived:
            RkyvDeserialize<T, rkyv::de::deserializers::SharedDeserializeMap>,
    {
        let db = self.inner.lock().database.clone();
        let out = db.get(col, key).wrap_err("failed to get key")?;
        let b = match out {
            Some(v) => v,
            None => {
                return Ok(None);
            }
        };
        let obj = from_rkyv(b)?;
        Ok(Some(obj))
    }

    /// Read an serde-json key from a column in the TableDB immediately
    pub fn load_json<T>(&self, col: u32, key: &[u8]) -> EyreResult<Option<T>>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        let db = self.inner.lock().database.clone();
        let out = db.get(col, key).wrap_err("failed to get key")?;
        let b = match out {
            Some(v) => v,
            None => {
                return Ok(None);
            }
        };
        let obj = serde_json::from_slice(&b)?;
        Ok(Some(obj))
    }

    /// Delete key with from a column in the TableDB
    pub async fn delete(&self, col: u32, key: &[u8]) -> EyreResult<bool> {
        let db = self.inner.lock().database.clone();
        let found = db.get(col, key).wrap_err("failed to get key")?;
        match found {
            None => Ok(false),
            Some(_) => {
                let mut dbt = db.transaction();
                dbt.delete(col, key);
                db.write(dbt).await.wrap_err("failed to delete key")?;
                Ok(true)
            }
        }
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
    pub async fn commit(self) -> EyreResult<()> {
        let dbt = {
            let mut inner = self.inner.lock();
            inner
                .dbt
                .take()
                .ok_or_else(|| eyre!("transaction already completed"))?
        };
        let db = self.db.inner.lock().database.clone();
        db.write(dbt)
            .await
            .wrap_err("commit failed, transaction lost")
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
    pub fn store_rkyv<T>(&self, col: u32, key: &[u8], value: &T) -> EyreResult<()>
    where
        T: RkyvSerialize<rkyv::ser::serializers::AllocSerializer<1024>>,
    {
        let v = to_rkyv(value)?;
        let mut inner = self.inner.lock();
        inner.dbt.as_mut().unwrap().put(col, key, v.as_slice());
        Ok(())
    }

    /// Store a key in rkyv format with a value in a column in the TableDB
    pub fn store_json<T>(&self, col: u32, key: &[u8], value: &T) -> EyreResult<()>
    where
        T: serde::Serialize,
    {
        let v = serde_json::to_vec(value)?;
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
