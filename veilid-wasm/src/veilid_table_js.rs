#![allow(non_snake_case)]
use super::*;

#[wasm_bindgen()]
pub struct VeilidTable {
    id: u32,
    tableName: String,
    columnCount: u32,
}

#[wasm_bindgen()]
impl VeilidTable {
    #[wasm_bindgen(constructor)]
    pub fn new(tableName: String, columnCount: u32) -> VeilidTable {
        VeilidTable {
            id: 0,
            tableName,
            columnCount,
        }
    }

    pub async fn openTable(&mut self) -> Result<u32, VeilidAPIError> {
        let veilid_api = get_veilid_api()?;
        let tstore = veilid_api.table_store()?;
        let table_db = tstore
            .open(&self.tableName, self.columnCount)
            .await
            .map_err(veilid_core::VeilidAPIError::generic)?;
        let new_id = add_table_db(table_db);
        self.id = new_id;
        APIResult::Ok(new_id)
    }

    pub fn releaseTable(&mut self) -> bool {
        let mut tdbs = (*TABLE_DBS).borrow_mut();
        let status = tdbs.remove(&self.id);
        self.id = 0;
        if status.is_none() {
            return false;
        }
        return true;
    }

    pub async fn deleteTable(&mut self) -> Result<bool, VeilidAPIError> {
        self.releaseTable();

        let veilid_api = get_veilid_api()?;
        let tstore = veilid_api.table_store()?;
        let deleted = tstore
            .delete(&self.tableName)
            .await
            .map_err(veilid_core::VeilidAPIError::generic)?;
        APIResult::Ok(deleted)
    }

    async fn ensureOpen(&mut self) {
        if self.id == 0 {
            let _ = self.openTable().await;
        }
    }

    pub async fn load(
        &mut self,
        columnId: u32,
        key: String,
    ) -> Result<Option<String>, VeilidAPIError> {
        self.ensureOpen().await;

        let table_db = {
            let table_dbs = (*TABLE_DBS).borrow();
            let Some(table_db) = table_dbs.get(&self.id) else {
                return APIResult::Err(veilid_core::VeilidAPIError::invalid_argument("table_db_load", "id", self.id));
            };
            table_db.clone()
        };

        let out = table_db.load(columnId, key.as_bytes()).await?.unwrap();
        let out = Some(str::from_utf8(&out).unwrap().to_owned());
        // let out = serde_wasm_bindgen::to_value(&out)
        //     .expect("Could not parse using serde_wasm_bindgen");
        APIResult::Ok(out)
    }

    pub async fn store(
        &mut self,
        columnId: u32,
        key: String,
        value: String,
    ) -> Result<(), VeilidAPIError> {
        self.ensureOpen().await;

        let table_db = {
            let table_dbs = (*TABLE_DBS).borrow();
            let Some(table_db) = table_dbs.get(&self.id) else {
                return APIResult::Err(veilid_core::VeilidAPIError::invalid_argument("table_db_store", "id", self.id));
            };
            table_db.clone()
        };

        table_db
            .store(columnId, key.as_bytes(), value.as_bytes())
            .await?;
        APIRESULT_UNDEFINED
    }

    pub async fn delete(
        &mut self,
        columnId: u32,
        key: String,
    ) -> Result<Option<String>, VeilidAPIError> {
        self.ensureOpen().await;

        let table_db = {
            let table_dbs = (*TABLE_DBS).borrow();
            let Some(table_db) = table_dbs.get(&self.id) else {
                return APIResult::Err(veilid_core::VeilidAPIError::invalid_argument("table_db_delete", "id", self.id));
            };
            table_db.clone()
        };

        // TODO: will crash when trying to .unwrap() of None (trying to delete key that doesn't exist)
        let out = table_db.delete(columnId, key.as_bytes()).await?.unwrap();
        let out = Some(str::from_utf8(&out).unwrap().to_owned());
        APIResult::Ok(out)
    }

    // TODO try and figure out how to result a String[], maybe Box<[String]>?
    pub async fn getKeys(&mut self, columnId: u32) -> Result<JsValue, VeilidAPIError> {
        self.ensureOpen().await;

        let table_db = {
            let table_dbs = (*TABLE_DBS).borrow();
            let Some(table_db) = table_dbs.get(&self.id) else {
                return APIResult::Err(veilid_core::VeilidAPIError::invalid_argument("table_db_store", "id", self.id));
            };
            table_db.clone()
        };

        let keys = table_db.clone().get_keys(columnId).await?;
        let out: Vec<String> = keys
            .into_iter()
            .map(|k| str::from_utf8(&k).unwrap().to_owned())
            .collect();
        let out =
            serde_wasm_bindgen::to_value(&out).expect("Could not parse using serde_wasm_bindgen");

        APIResult::Ok(out)
    }

    pub async fn transact(&mut self) -> u32 {
        self.ensureOpen().await;

        let table_dbs = (*TABLE_DBS).borrow();
        let Some(table_db) = table_dbs.get(&self.id) else {
            return 0;
        };
        let tdbt = table_db.clone().transact();
        let tdbtid = add_table_db_transaction(tdbt);
        return tdbtid;
    }

    // TODO: placeholders for transaction functions
    // pub async fn releaseTransaction(&mut self) {
    //     self.ensureOpen().await;
    // }

    // pub async fn commitTransaction(&mut self) {
    //     self.ensureOpen().await;
    // }

    // pub async fn rollbackTransaction(&mut self) {
    //     self.ensureOpen().await;
    // }

    // pub async fn storeTransaction(&mut self, tableId: u32, key: String, value: String) {
    //     self.ensureOpen().await;
    // }

    // pub async fn deleteTransaction(&mut self) {
    //     self.ensureOpen().await;
    // }
}
