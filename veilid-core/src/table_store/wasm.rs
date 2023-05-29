use super::*;
pub use keyvaluedb_web::*;

#[derive(Clone)]
pub struct TableStoreDriver {
    _config: VeilidConfig,
}

impl TableStoreDriver {
    pub(crate) fn new(config: VeilidConfig) -> Self {
        Self { _config: config }
    }

    pub async fn open(&self, table_name: &str, column_count: u32) -> VeilidAPIResult<Database> {
        let db = Database::open(table_name, column_count, false)
            .await
            .map_err(VeilidAPIError::generic)?;
        trace!(
            "opened table store '{}' with {} columns",
            table_name,
            column_count
        );
        Ok(db)
    }

    /// Delete a TableDB table by name
    pub async fn delete(&self, table_name: &str) -> VeilidAPIResult<bool> {
        if is_browser() {
            let out = Database::delete(table_name).await.is_ok();
            if out {
                trace!("TableStore::delete {} deleted", table_name);
            } else {
                debug!("TableStore::delete {} not deleted", table_name);
            }
            Ok(out)
        } else {
            unimplemented!();
        }
    }
}
