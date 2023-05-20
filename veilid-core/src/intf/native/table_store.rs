use super::*;
use crate::intf::table_db::TableDBUnlockedInner;
pub use crate::intf::table_db::{TableDB, TableDBTransaction};
use keyvaluedb_sqlite::*;
use std::path::PathBuf;

struct TableStoreInner {
    opened: BTreeMap<String, Weak<Mutex<TableDBUnlockedInner>>>,
}

/// Veilid Table Storage
/// Database for storing key value pairs persistently across runs
#[derive(Clone)]
pub struct TableStore {
    config: VeilidConfig,
    inner: Arc<Mutex<TableStoreInner>>,
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
        }
    }

    /// Delete all known tables
    pub async fn delete_all(&self) {
        for ktn in &KNOWN_TABLE_NAMES {
            if let Err(e) = self.delete(ktn).await {
                error!("failed to delete '{}': {}", ktn, e);
            } else {
                debug!("deleted table '{}'", ktn);
            }
        }
    }

    pub(crate) async fn init(&self) -> EyreResult<()> {
        Ok(())
    }

    pub(crate) async fn terminate(&self) {
        assert!(
            self.inner.lock().opened.is_empty(),
            "all open databases should have been closed"
        );
    }

    pub(crate) fn on_table_db_drop(&self, table: String) {
        let mut inner = self.inner.lock();
        if inner.opened.remove(&table).is_none() {
            unreachable!("should have removed an item");
        }
    }

    fn get_dbpath(&self, table: &str) -> EyreResult<PathBuf> {
        if !table
            .chars()
            .all(|c| char::is_alphanumeric(c) || c == '_' || c == '-')
        {
            bail!("table name '{}' is invalid", table);
        }
        let c = self.config.get();
        let tablestoredir = c.table_store.directory.clone();
        std::fs::create_dir_all(&tablestoredir).wrap_err("failed to create tablestore path")?;

        let dbpath: PathBuf = [tablestoredir, String::from(table)].iter().collect();
        Ok(dbpath)
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
        Ok(if namespace.is_empty() {
            table.to_string()
        } else {
            format!("_ns_{}_{}", namespace, table)
        })
    }

    /// Get or create a TableDB database table. If the column count is greater than an
    /// existing TableDB's column count, the database will be upgraded to add the missing columns
    pub async fn open(&self, name: &str, column_count: u32) -> EyreResult<TableDB> {
        let table_name = self.get_table_name(name)?;

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

        let dbpath = self.get_dbpath(&table_name)?;

        // Ensure permissions are correct
        ensure_file_private_owner(&dbpath)?;

        let cfg = DatabaseConfig::with_columns(column_count);
        let db = Database::open(&dbpath, cfg).wrap_err("failed to open tabledb")?;

        // Ensure permissions are correct
        ensure_file_private_owner(&dbpath)?;

        trace!(
            "opened table store '{}' at path '{:?}' with {} columns",
            name,
            dbpath,
            column_count
        );
        let table_db = TableDB::new(table_name.clone(), self.clone(), db);

        inner.opened.insert(table_name, table_db.weak_inner());

        Ok(table_db)
    }

    /// Delete a TableDB table by name
    pub async fn delete(&self, name: &str) -> EyreResult<bool> {
        let table_name = self.get_table_name(name)?;

        let inner = self.inner.lock();
        if inner.opened.contains_key(&table_name) {
            bail!("Not deleting table that is still opened");
        }
        let dbpath = self.get_dbpath(&table_name)?;
        let ret = std::fs::remove_file(dbpath).is_ok();
        Ok(ret)
    }
}
