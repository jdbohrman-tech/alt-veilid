#![allow(non_snake_case)]
use super::*;

#[wasm_bindgen()]
pub struct VeilidTableDB {
    inner_table_db: Option<TableDB>,
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
            inner_table_db: None,
            tableName,
            columnCount,
        }
    }

    fn getTableDB(&self) -> APIResult<TableDB> {
        let Some(table_db) = &self.inner_table_db else {
            return APIResult::Err(veilid_core::VeilidAPIError::generic("Unable to getTableDB instance. Ensure you've called openTable()."));
        };
        APIResult::Ok(table_db.clone())
    }

    /// Get or create the TableDB database table.
    /// This is called automatically when performing actions on the TableDB.
    pub async fn openTable(&mut self) -> APIResult<()> {
        let veilid_api = get_veilid_api()?;
        let tstore = veilid_api.table_store()?;
        let table_db = tstore
            .open(&self.tableName, self.columnCount)
            .await
            .map_err(veilid_core::VeilidAPIError::generic)?;
        self.inner_table_db = Some(table_db);
        APIRESULT_UNDEFINED
    }

    /// Delete this TableDB.
    pub async fn deleteTable(&mut self) -> APIResult<bool> {
        self.inner_table_db = None;

        let veilid_api = get_veilid_api()?;
        let tstore = veilid_api.table_store()?;
        let deleted = tstore
            .delete(&self.tableName)
            .await
            .map_err(veilid_core::VeilidAPIError::generic)?;
        APIResult::Ok(deleted)
    }

    async fn ensureOpen(&mut self) {
        if self.inner_table_db.is_none() {
            let _ = self.openTable().await;
        }
    }

    /// Read a key from a column in the TableDB immediately.
    pub async fn load(&mut self, columnId: u32, key: String) -> APIResult<Option<String>> {
        self.ensureOpen().await;
        let key = unmarshall(key)?;
        let table_db = self.getTableDB()?;

        let out = table_db.load(columnId, &key).await?;
        let out = out.map(|out| marshall(&out));
        APIResult::Ok(out)
    }

    /// Store a key with a value in a column in the TableDB.
    /// Performs a single transaction immediately.
    pub async fn store(&mut self, columnId: u32, key: String, value: String) -> APIResult<()> {
        self.ensureOpen().await;
        let key = unmarshall(key)?;
        let value = unmarshall(value)?;
        let table_db = self.getTableDB()?;

        table_db.store(columnId, &key, &value).await?;
        APIRESULT_UNDEFINED
    }

    /// Delete key with from a column in the TableDB.
    pub async fn delete(&mut self, columnId: u32, key: String) -> APIResult<Option<String>> {
        self.ensureOpen().await;
        let key = unmarshall(key)?;
        let table_db = self.getTableDB()?;

        let out = table_db.delete(columnId, &key).await?;
        let out = out.map(|out| marshall(&out));
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
        APIResult::Ok(VeilidTableDBTransaction {
            inner_transaction: Some(transaction),
        })
    }
}

#[wasm_bindgen]
pub struct VeilidTableDBTransaction {
    inner_transaction: Option<TableDBTransaction>,
}

#[wasm_bindgen]
impl VeilidTableDBTransaction {
    /// Don't use this constructor directly.
    /// Use `.createTransaction()` on an instance of `VeilidTableDB` instead.
    /// @deprecated
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner_transaction: None,
        }
    }

    fn getTransaction(&self) -> APIResult<TableDBTransaction> {
        let Some(transaction) = &self.inner_transaction else {
            return APIResult::Err(veilid_core::VeilidAPIError::generic("Unable to getTransaction instance. inner_transaction is None."));
        };
        APIResult::Ok(transaction.clone())
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
        let key = unmarshall(key)?;
        let value = unmarshall(value)?;
        let transaction = self.getTransaction()?;
        transaction.store(col, &key, &value)
    }

    /// Delete key with from a column in the TableDB
    pub fn deleteKey(&self, col: u32, key: String) -> APIResult<()> {
        let key = unmarshall(key)?;
        let transaction = self.getTransaction()?;
        transaction.delete(col, &key)
    }
}
