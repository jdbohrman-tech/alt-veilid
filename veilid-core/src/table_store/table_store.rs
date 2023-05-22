use super::*;
use keyvaluedb::*;

struct TableStoreInner {
    opened: BTreeMap<String, Weak<TableDBUnlockedInner>>,
    encryption_key: Option<TypedSharedSecret>,
    all_table_names: HashMap<String, String>,
    all_tables_db: Option<Database>,
}

/// Veilid Table Storage
/// Database for storing key value pairs persistently and securely across runs
#[derive(Clone)]
pub struct TableStore {
    config: VeilidConfig,
    protected_store: ProtectedStore,
    table_store_driver: TableStoreDriver,
    inner: Arc<Mutex<TableStoreInner>>, // Sync mutex here because TableDB drops can happen at any time
    async_lock: Arc<AsyncMutex<()>>,    // Async mutex for operations
}

impl TableStore {
    fn new_inner() -> TableStoreInner {
        TableStoreInner {
            opened: BTreeMap::new(),
            encryption_key: None,
            all_table_names: HashMap::new(),
            all_tables_db: None,
        }
    }
    pub(crate) fn new(config: VeilidConfig, protected_store: ProtectedStore) -> Self {
        let inner = Self::new_inner();
        let table_store_driver = TableStoreDriver::new(config.clone());

        Self {
            config,
            protected_store,
            inner: Arc::new(Mutex::new(inner)),
            table_store_driver,
            async_lock: Arc::new(AsyncMutex::new(())),
        }
    }

    // Flush internal control state
    async fn flush(&self) {
        let (all_table_names_value, all_tables_db) = {
            let inner = self.inner.lock();
            let all_table_names_value =
                to_rkyv(&inner.all_table_names).expect("failed to archive all_table_names");
            (all_table_names_value, inner.all_tables_db.clone().unwrap())
        };
        let mut dbt = DBTransaction::new();
        dbt.put(0, b"all_table_names", &all_table_names_value);
        if let Err(e) = all_tables_db.write(dbt).await {
            error!("failed to write all tables db: {}", e);
        }
    } xxx must from_rkyv the all_table_names

    // Internal naming support
    // Adds rename capability and ensures names of tables are totally unique and valid

    fn namespaced_name(&self, table: &str) -> VeilidAPIResult<String> {
        if !table
            .chars()
            .all(|c| char::is_alphanumeric(c) || c == '_' || c == '-')
        {
            apibail_invalid_argument!("table name is invalid", "table", table);
        }
        let c = self.config.get();
        let namespace = c.namespace.clone();
        Ok(if namespace.is_empty() {
            table.to_string()
        } else {
            format!("_ns_{}_{}", namespace, table)
        })
    }

    async fn name_get_or_create(&self, table: &str) -> VeilidAPIResult<String> {
        let name = self.namespaced_name(table)?;

        let mut inner = self.inner.lock();
        // Do we have this name yet?
        if let Some(real_name) = inner.all_table_names.get(&name) {
            return Ok(real_name.clone());
        }

        // If not, make a new low level name mapping
        let mut real_name_bytes = [0u8; 32];
        random_bytes(&mut real_name_bytes);
        let real_name = data_encoding::BASE64URL_NOPAD.encode(&real_name_bytes);

        if inner
            .all_table_names
            .insert(name.to_owned(), real_name.clone())
            .is_some()
        {
            panic!("should not have had some value");
        };

        Ok(real_name)
    }

    async fn name_delete(&self, table: &str) -> VeilidAPIResult<Option<String>> {
        let name = self.namespaced_name(table)?;
        let mut inner = self.inner.lock();
        let real_name = inner.all_table_names.remove(&name);
        Ok(real_name)
    }

    async fn name_get(&self, table: &str) -> VeilidAPIResult<Option<String>> {
        let name = self.namespaced_name(table)?;
        let inner = self.inner.lock();
        let real_name = inner.all_table_names.get(&name).cloned();
        Ok(real_name)
    }

    async fn name_rename(&self, old_table: &str, new_table: &str) -> VeilidAPIResult<()> {
        let old_name = self.namespaced_name(old_table)?;
        let new_name = self.namespaced_name(new_table)?;

        let mut inner = self.inner.lock();
        // Ensure new name doesn't exist
        if inner.all_table_names.contains_key(&new_name) {
            return Err(VeilidAPIError::generic("new table already exists"));
        }
        // Do we have this name yet?
        let Some(real_name) = inner.all_table_names.remove(&old_name) else {
            return Err(VeilidAPIError::generic("table does not exist"));
        };
        // Insert with new name
        inner.all_table_names.insert(new_name.to_owned(), real_name);

        Ok(())
    }

    /// Delete all known tables
    async fn delete_all(&self) {
        // Get all tables
        let real_names = {
            let mut inner = self.inner.lock();
            let real_names = inner
                .all_table_names
                .values()
                .cloned()
                .collect::<Vec<String>>();
            inner.all_table_names.clear();
            real_names
        };

        // Delete all tables
        for table_name in real_names {
            if let Err(e) = self.table_store_driver.delete(&table_name).await {
                error!("error deleting table: {}", e);
            }
        }
        self.flush().await;
    }

    pub(crate) async fn init(&self) -> EyreResult<()> {
        let _async_guard = self.async_lock.lock().await;

        let encryption_key: Option<TypedSharedSecret> = self
            .protected_store
            .load_user_secret_rkyv("device_encryption_key")
            .await?;

        let all_tables_db = self
            .table_store_driver
            .open("__veilid_all_tables", 1)
            .await
            .wrap_err("failed to create all tables table")?;

        {
            let mut inner = self.inner.lock();
            inner.encryption_key = encryption_key;
            inner.all_tables_db = Some(all_tables_db);
        }

        let do_delete = {
            let c = self.config.get();
            c.table_store.delete
        };

        if do_delete {
            self.delete_all().await;
        }

        Ok(())
    }

    pub(crate) async fn terminate(&self) {
        let _async_guard = self.async_lock.lock().await;
        let mut inner = self.inner.lock();
        if !inner.opened.is_empty() {
            panic!(
                "all open databases should have been closed: {:?}",
                inner.opened
            );
        }
        inner.all_tables_db = None;
        inner.encryption_key = None;
    }

    pub(crate) fn on_table_db_drop(&self, table: String) {
        let mut inner = self.inner.lock();
        if inner.opened.remove(&table).is_none() {
            unreachable!("should have removed an item");
        }
    }

    /// Get or create a TableDB database table. If the column count is greater than an
    /// existing TableDB's column count, the database will be upgraded to add the missing columns
    pub async fn open(&self, name: &str, column_count: u32) -> VeilidAPIResult<TableDB> {
        let _async_guard = self.async_lock.lock().await;
        let table_name = self.name_get_or_create(name).await?;

        // See if this table is already opened
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

        // Open table db using platform-specific driver
        let db = match self
            .table_store_driver
            .open(&table_name, column_count)
            .await
        {
            Ok(db) => db,
            Err(e) => {
                self.name_delete(name).await.expect("cleanup failed");
                self.flush().await;
                return Err(e);
            }
        };

        // Flush table names to disk
        self.flush().await;

        // Wrap low-level Database in TableDB object
        let mut inner = self.inner.lock();
        let table_db = TableDB::new(
            table_name.clone(),
            self.clone(),
            db,
            inner.encryption_key.clone(),
        );

        // Keep track of opened DBs
        inner
            .opened
            .insert(table_name.clone(), table_db.weak_inner());

        Ok(table_db)
    }

    /// Delete a TableDB table by name
    pub async fn delete(&self, name: &str) -> VeilidAPIResult<bool> {
        let _async_guard = self.async_lock.lock().await;
        let Some(table_name) = self.name_get(name).await? else {
            // Did not exist in name table
            return Ok(false);
        };

        // See if this table is opened
        {
            let inner = self.inner.lock();
            if inner.opened.contains_key(&table_name) {
                apibail_generic!("Not deleting table that is still opened");
            }
        }

        // Delete table db using platform-specific driver
        let deleted = self.table_store_driver.delete(&table_name).await?;
        if !deleted {
            // Table missing? Just remove name
            self.name_delete(&name)
                .await
                .expect("failed to delete name");
            warn!(
                "table existed in name table but not in storage: {} : {}",
                name, table_name
            );
            return Ok(false);
        }

        Ok(true)
    }

    /// Rename a TableDB table
    pub async fn rename(&self, old_name: &str, new_name: &str) -> VeilidAPIResult<()> {
        let _async_guard = self.async_lock.lock().await;
        trace!("TableStore::rename {} -> {}", old_name, new_name);
        self.name_rename(old_name, new_name).await
    }
}
