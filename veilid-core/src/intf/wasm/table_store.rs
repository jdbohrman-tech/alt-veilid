use super::*;
use crate::intf::table_db::TableDBUnlockedInner;
pub use crate::intf::table_db::{TableDB, TableDBTransaction};
use keyvaluedb_web::*;

struct TableStoreInner {
    opened: BTreeMap<String, Weak<TableDBUnlockedInner>>,
}

#[derive(Clone)]
pub struct TableStore {
    config: VeilidConfig,
    inner: Arc<Mutex<TableStoreInner>>,
    async_lock: Arc<AsyncMutex<()>>,
}

impl TableStore {
    fn new_inner() -> TableStoreInner {
        TableStoreInner {
            opened: BTreeMap::new(),
        }
    }
    pub(crate) fn new(config: VeilidConfig) -> Self {
        Self {
            config,
            inner: Arc::new(Mutex::new(Self::new_inner())),
            async_lock: Arc::new(AsyncMutex::new(())),
        }
    }

    /// Delete all known tables
    pub async fn delete_all(&self) {
        for ktn in &KNOWN_TABLE_NAMES {
            if let Err(e) = self.delete(ktn).await {
                error!("failed to delete '{}': {}", ktn, e);
            }
        }
    }

    pub(crate) async fn init(&self) -> EyreResult<()> {
        let _async_guard = self.async_lock.lock().await;
        Ok(())
    }

    pub(crate) async fn terminate(&self) {
        let _async_guard = self.async_lock.lock().await;
        assert!(
            self.inner.lock().opened.len() == 0,
            "all open databases should have been closed"
        );
    }

    pub(crate) fn on_table_db_drop(&self, table: String) {
        let mut inner = self.inner.lock();
        match inner.opened.remove(&table) {
            Some(_) => (),
            None => {
                assert!(false, "should have removed an item");
            }
        }
    }

    fn get_table_name(&self, table: &str) -> EyreResult<String> {
        if !table
            .chars()
            .all(|c| char::is_alphanumeric(c) || c == '_' || c == '-')
        {
            bail!("table name '{}' is invalid", table);
        }
        let c = self.config.get();
        let namespace = c.namespace.clone();
        Ok(if namespace.len() == 0 {
            format!("{}", table)
        } else {
            format!("_ns_{}_{}", namespace, table)
        })
    }

    /// Get or create a TableDB database table. If the column count is greater than an
    /// existing TableDB's column count, the database will be upgraded to add the missing columns
    pub async fn open(&self, name: &str, column_count: u32) -> EyreResult<TableDB> {
        let _async_guard = self.async_lock.lock().await;
        let table_name = self.get_table_name(name)?;

        {
            let mut inner = self.inner.lock();
            if let Some(table_db_weak_inner) = inner.opened.get(&table_name) {
                match TableDB::try_new_from_weak_inner(table_db_weak_inner.clone()) {
                    Some(tdb) => {
                        return Ok(tdb);
                    }
                    None => {
                        inner.opened.remove(&table_name);
                    }
                };
            }
        }
        let db = Database::open(table_name.clone(), column_count, false)
            .await
            .wrap_err("failed to open tabledb")?;
        trace!(
            "opened table store '{}' with table name '{:?}' with {} columns",
            name,
            table_name,
            column_count
        );

        let table_db = TableDB::new(table_name.clone(), self.clone(), db);

        {
            let mut inner = self.inner.lock();
            inner.opened.insert(table_name, table_db.weak_inner());
        }

        Ok(table_db)
    }

    /// Delete a TableDB table by name
    pub async fn delete(&self, name: &str) -> EyreResult<bool> {
        let _async_guard = self.async_lock.lock().await;
        trace!("TableStore::delete {}", name);
        let table_name = self.get_table_name(name)?;

        {
            let inner = self.inner.lock();
            if inner.opened.contains_key(&table_name) {
                trace!(
                    "TableStore::delete {}: Not deleting, still open.",
                    table_name
                );
                bail!("Not deleting table that is still opened");
            }
        }

        if is_browser() {
            let out = match Database::delete(table_name.clone()).await {
                Ok(_) => true,
                Err(_) => false,
            };
            //.map_err(|e| format!("failed to delete tabledb at: {} ({})", table_name, e))?;
            trace!("TableStore::deleted {}", table_name);
            Ok(out)
        } else {
            unimplemented!();
        }
    }
}
