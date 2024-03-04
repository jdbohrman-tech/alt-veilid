use super::*;
pub use keyvaluedb_sqlite::*;
use std::path::PathBuf;

#[derive(Clone)]
pub(crate) struct TableStoreDriver {
    config: VeilidConfig,
}

impl TableStoreDriver {
    pub fn new(config: VeilidConfig) -> Self {
        Self { config }
    }

    fn get_dbpath(&self, table: &str) -> VeilidAPIResult<PathBuf> {
        let c = self.config.get();
        let tablestoredir = c.table_store.directory.clone();
        std::fs::create_dir_all(&tablestoredir).map_err(VeilidAPIError::from)?;

        let c = self.config.get();
        let namespace = c.namespace.clone();
        let dbpath: PathBuf = if namespace.is_empty() {
            [tablestoredir, String::from(table)].iter().collect()
        } else {
            [tablestoredir, format!("{}_{}", namespace, table)]
                .iter()
                .collect()
        };
        Ok(dbpath)
    }

    pub async fn open(&self, table_name: &str, column_count: u32) -> VeilidAPIResult<Database> {
        let dbpath = self.get_dbpath(table_name)?;

        // Ensure permissions are correct
        ensure_file_private_owner(&dbpath).map_err(VeilidAPIError::internal)?;

        let cfg = DatabaseConfig::with_columns(column_count);
        let db = Database::open(&dbpath, cfg).map_err(VeilidAPIError::from)?;

        // Ensure permissions are correct
        ensure_file_private_owner(&dbpath).map_err(VeilidAPIError::internal)?;

        trace!(
            "opened table store '{}' at path '{:?}' with {} columns",
            table_name,
            dbpath,
            column_count
        );
        Ok(db)
    }

    pub async fn delete(&self, table_name: &str) -> VeilidAPIResult<bool> {
        let dbpath = self.get_dbpath(table_name)?;
        if !dbpath.exists() {
            return Ok(false);
        }
        std::fs::remove_file(dbpath).map_err(VeilidAPIError::from)?;
        Ok(true)
    }
}
