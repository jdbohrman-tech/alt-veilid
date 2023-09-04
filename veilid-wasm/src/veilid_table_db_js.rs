#![allow(non_snake_case)]
use super::*;

#[wasm_bindgen()]
pub struct VeilidTableDB {
    id: u32,
    tableName: String,
    columnCount: u32,
}

#[wasm_bindgen()]
impl VeilidTableDB {
    /// If the column count is greater than an existing TableDB's column count,
    /// the database will be upgraded to add the missing columns.
    #[wasm_bindgen(constructor)]
    pub fn new(tableName: String, columnCount: u32) -> Self {
        Self {
            id: 0,
            tableName,
            columnCount,
        }
    }

    fn getTableDB(&self) -> APIResult<TableDB> {
        let table_dbs = (*TABLE_DBS).borrow();
        let Some(table_db) = table_dbs.get(&self.id) else {
            return APIResult::Err(veilid_core::VeilidAPIError::invalid_argument("getTableDB", "id", self.id));
        };
        APIResult::Ok(table_db.clone())
    }

    /// Get or create the TableDB database table.
    /// This is called automatically when performing actions on the TableDB.
    pub async fn openTable(&mut self) -> APIResult<u32> {
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

    /// Release the TableDB instance from memory.
    pub fn releaseTable(&mut self) -> bool {
        let mut tdbs = (*TABLE_DBS).borrow_mut();
        let status = tdbs.remove(&self.id);
        self.id = 0;
        if status.is_none() {
            return false;
        }
        return true;
    }

    /// Delete this TableDB.
    pub async fn deleteTable(&mut self) -> APIResult<bool> {
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

    /// Read a key from a column in the TableDB immediately.
    pub async fn load(&mut self, columnId: u32, key: String) -> APIResult<Option<String>> {
        self.ensureOpen().await;
        let key = unmarshall(key);
        let table_db = self.getTableDB()?;

        let out = table_db.load(columnId, &key).await?.unwrap();
        let out = Some(marshall(&out));
        APIResult::Ok(out)
    }

    /// Store a key with a value in a column in the TableDB.
    /// Performs a single transaction immediately.
    pub async fn store(&mut self, columnId: u32, key: String, value: String) -> APIResult<()> {
        self.ensureOpen().await;
        let key = unmarshall(key);
        let value = unmarshall(value);
        let table_db = self.getTableDB()?;

        table_db.store(columnId, &key, &value).await?;
        APIRESULT_UNDEFINED
    }

    /// Delete key with from a column in the TableDB.
    pub async fn delete(&mut self, columnId: u32, key: String) -> APIResult<Option<String>> {
        self.ensureOpen().await;
        let key = unmarshall(key);
        let table_db = self.getTableDB()?;

        let out = table_db.delete(columnId, &key).await?.unwrap();
        let out = Some(marshall(&out));
        APIResult::Ok(out)
    }

    /// Get the list of keys in a column of the TableDB.
    ///
    /// Returns an array of base64Url encoded keys.
    pub async fn getKeys(&mut self, columnId: u32) -> APIResult<StringArray> {
        self.ensureOpen().await;
        let table_db = self.getTableDB()?;

        let keys = table_db.clone().get_keys(columnId).await?;
        let out: Vec<String> = keys.into_iter().map(|k| marshall(&k)).collect();
        let out = into_unchecked_string_array(out);

        APIResult::Ok(out)
    }

    /// Start a TableDB write transaction.
    /// The transaction object must be committed or rolled back before dropping.
    pub async fn createTransaction(&mut self) -> APIResult<VeilidTableDBTransaction> {
        self.ensureOpen().await;
        let table_db = self.getTableDB()?;

        let transaction = table_db.transact();
        let transaction_id = add_table_db_transaction(transaction);
        APIResult::Ok(VeilidTableDBTransaction { id: transaction_id })
    }
}

#[wasm_bindgen]
pub struct VeilidTableDBTransaction {
    id: u32,
}

#[wasm_bindgen]
impl VeilidTableDBTransaction {
    /// Don't use this constructor directly.
    /// Use `.createTransaction()` on an instance of `VeilidTableDB` instead.
    /// @deprecated
    #[wasm_bindgen(constructor)]
    pub fn new(id: u32) -> Self {
        Self { id }
    }

    fn getTransaction(&self) -> APIResult<TableDBTransaction> {
        let transactions = (*TABLE_DB_TRANSACTIONS).borrow();
        let Some(transaction) = transactions.get(&self.id) else {
            return APIResult::Err(veilid_core::VeilidAPIError::invalid_argument("getTransaction", "id", &self.id));
        };
        APIResult::Ok(transaction.clone())
    }

    /// Releases the transaction from memory.
    pub fn releaseTransaction(&mut self) -> bool {
        let mut transactions = (*TABLE_DB_TRANSACTIONS).borrow_mut();
        self.id = 0;
        if transactions.remove(&self.id).is_none() {
            return false;
        }
        return true;
    }

    /// Commit the transaction. Performs all actions atomically.
    pub async fn commit(&self) -> APIResult<()> {
        let transaction = self.getTransaction()?;
        transaction.commit().await
    }

    /// Rollback the transaction. Does nothing to the TableDB.
    pub fn rollback(&self) -> APIResult<()> {
        let transaction = self.getTransaction()?;
        transaction.rollback();
        APIRESULT_UNDEFINED
    }

    /// Store a key with a value in a column in the TableDB.
    /// Does not modify TableDB until `.commit()` is called.
    pub fn store(&self, col: u32, key: String, value: String) -> APIResult<()> {
        let key = unmarshall(key);
        let value = unmarshall(value);
        let transaction = self.getTransaction()?;
        transaction.store(col, &key, &value)
    }

    /// Delete key with from a column in the TableDB
    pub fn deleteKey(&self, col: u32, key: String) -> APIResult<()> {
        let key = unmarshall(key);
        let transaction = self.getTransaction()?;
        transaction.delete(col, &key)
    }
}
