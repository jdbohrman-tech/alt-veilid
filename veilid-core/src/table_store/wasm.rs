use super::*;
pub use keyvaluedb_web::*;

#[derive(Clone)]
pub(in crate::table_store) struct TableStoreDriver {
    registry: VeilidComponentRegistry,
}

impl_veilid_component_registry_accessor!(TableStoreDriver);

impl TableStoreDriver {
    pub fn new(registry: VeilidComponentRegistry) -> Self {
        Self { registry }
    }

    fn get_namespaced_table_name(&self, table: &str) -> String {
        let config = self.registry().config();
        let c = config.get();
        let namespace = c.namespace.clone();
        if namespace.is_empty() {
            table.to_owned()
        } else {
            format!("{}_{}", namespace, table)
        }
    }

    pub async fn open(&self, table_name: &str, column_count: u32) -> VeilidAPIResult<Database> {
        let namespaced_table_name = self.get_namespaced_table_name(table_name);
        let db = Database::open(&namespaced_table_name, column_count, false)
            .await
            .map_err(VeilidAPIError::generic)?;
        veilid_log!(self trace
            "opened table store '{}' with {} columns",
            namespaced_table_name,
            column_count
        );
        Ok(db)
    }

    /// Delete a TableDB table by name
    pub async fn delete(&self, table_name: &str) -> VeilidAPIResult<bool> {
        if is_browser() {
            let namespaced_table_name = self.get_namespaced_table_name(table_name);
            let out = Database::delete(&namespaced_table_name).await.is_ok();
            if out {
                veilid_log!(self trace "TableStore::delete {} deleted", namespaced_table_name);
            } else {
                veilid_log!(self debug "TableStore::delete {} not deleted", namespaced_table_name);
            }
            Ok(out)
        } else {
            unimplemented!();
        }
    }
}
