use super::*;
pub use keyvaluedb_sqlite::*;
use std::path::PathBuf;

#[derive(Clone)]
pub(in crate::table_store) struct TableStoreDriver {
    registry: VeilidComponentRegistry,
}

impl_veilid_component_registry_accessor!(TableStoreDriver);

impl TableStoreDriver {
    pub fn new(registry: VeilidComponentRegistry) -> Self {
        Self { registry }
    }

    fn get_dbpath(&self, table: &str) -> VeilidAPIResult<PathBuf> {
        let config = self.registry().config();
        let c = config.get();
        let tablestoredir = c.table_store.directory.clone();
        std::fs::create_dir_all(&tablestoredir).map_err(VeilidAPIError::from)?;

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

        veilid_log!(self trace
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
            veilid_log!(self debug "TableStore::delete '{}' at path '{:?}' not deleted", table_name, dbpath);
            return Ok(false);
        }

        veilid_log!(self trace "TableStore::delete '{}' at path '{:?}' deleted", table_name, dbpath);
        std::fs::remove_file(dbpath).map_err(VeilidAPIError::from)?;

        Ok(true)
    }
}
