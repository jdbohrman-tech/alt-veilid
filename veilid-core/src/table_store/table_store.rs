use super::*;
use keyvaluedb::*;

const ALL_TABLE_NAMES: &[u8] = b"all_table_names";

struct TableStoreInner {
    opened: BTreeMap<String, Weak<TableDBUnlockedInner>>,
    encryption_key: Option<TypedSharedSecret>,
    all_table_names: HashMap<String, String>,
    all_tables_db: Option<Database>,
    crypto: Option<Crypto>,
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
            crypto: None,
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

    pub(crate) fn set_crypto(&self, crypto: Crypto) {
        let mut inner = self.inner.lock();
        inner.crypto = Some(crypto);
    }

    // Flush internal control state (must not use crypto)
    async fn flush(&self) {
        let (all_table_names_value, all_tables_db) = {
            let inner = self.inner.lock();
            let all_table_names_value = serialize_json_bytes(&inner.all_table_names);
            (all_table_names_value, inner.all_tables_db.clone().unwrap())
        };
        let mut dbt = DBTransaction::new();
        dbt.put(0, ALL_TABLE_NAMES, &all_table_names_value);
        if let Err(e) = all_tables_db.write(dbt).await {
            error!("failed to write all tables db: {}", e);
        }
    }

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

    pub(crate) fn maybe_unprotect_device_encryption_key(
        &self,
        dek_bytes: &[u8],
        device_encryption_key_password: &str,
    ) -> EyreResult<TypedSharedSecret> {
        // Ensure the key is at least as long as necessary if unencrypted
        if dek_bytes.len() < (4 + SHARED_SECRET_LENGTH) {
            bail!("device encryption key is not valid");
        }

        // Get cryptosystem
        let kind = FourCC::try_from(&dek_bytes[0..4]).unwrap();
        let crypto = self.inner.lock().crypto.as_ref().unwrap().clone();
        let Some(vcrypto) = crypto.get(kind) else {
            bail!("unsupported cryptosystem");
        };

        if !device_encryption_key_password.is_empty() {
            if dek_bytes.len()
                != (4 + SHARED_SECRET_LENGTH + vcrypto.aead_overhead() + NONCE_LENGTH)
            {
                bail!("password protected device encryption key is not valid");
            }
            let protected_key = &dek_bytes[4..(4 + SHARED_SECRET_LENGTH + vcrypto.aead_overhead())];
            let nonce = &dek_bytes[(4 + SHARED_SECRET_LENGTH + vcrypto.aead_overhead())..];

            let shared_secret = vcrypto
                .derive_shared_secret(device_encryption_key_password.as_bytes(), &nonce)
                .wrap_err("failed to derive shared secret")?;
            let unprotected_key = vcrypto
                .decrypt_aead(
                    &protected_key,
                    &Nonce::try_from(nonce).wrap_err("invalid nonce")?,
                    &shared_secret,
                    None,
                )
                .wrap_err("failed to decrypt device encryption key")?;
            return Ok(TypedSharedSecret::new(
                kind,
                SharedSecret::try_from(unprotected_key.as_slice())
                    .wrap_err("invalid shared secret")?,
            ));
        }

        if dek_bytes.len() != (4 + SHARED_SECRET_LENGTH) {
            bail!("password protected device encryption key is not valid");
        }

        Ok(TypedSharedSecret::new(
            kind,
            SharedSecret::try_from(&dek_bytes[4..])?,
        ))
    }

    pub(crate) fn maybe_protect_device_encryption_key(
        &self,
        dek: TypedSharedSecret,
        device_encryption_key_password: &str,
    ) -> EyreResult<Vec<u8>> {
        // Check if we are to protect the key
        if device_encryption_key_password.is_empty() {
            debug!("no dek password");
            // Return the unprotected key bytes
            let mut out = Vec::with_capacity(4 + SHARED_SECRET_LENGTH);
            out.extend_from_slice(&dek.kind.0);
            out.extend_from_slice(&dek.value.bytes);
            return Ok(out);
        }

        // Get cryptosystem
        let crypto = self.inner.lock().crypto.as_ref().unwrap().clone();
        let Some(vcrypto) = crypto.get(dek.kind) else {
            bail!("unsupported cryptosystem");
        };

        let nonce = vcrypto.random_nonce();
        let shared_secret = vcrypto
            .derive_shared_secret(device_encryption_key_password.as_bytes(), &nonce.bytes)
            .wrap_err("failed to derive shared secret")?;
        let mut protected_key = vcrypto
            .encrypt_aead(
                &dek.value.bytes,
                &Nonce::try_from(nonce).wrap_err("invalid nonce")?,
                &shared_secret,
                None,
            )
            .wrap_err("failed to decrypt device encryption key")?;
        let mut out =
            Vec::with_capacity(4 + SHARED_SECRET_LENGTH + vcrypto.aead_overhead() + NONCE_LENGTH);
        out.extend_from_slice(&dek.kind.0);
        out.append(&mut protected_key);
        out.extend_from_slice(&nonce.bytes);
        assert!(out.len() == 4 + SHARED_SECRET_LENGTH + vcrypto.aead_overhead() + NONCE_LENGTH);
        Ok(out)
    }

    async fn load_device_encryption_key(&self) -> EyreResult<Option<TypedSharedSecret>> {
        let dek_bytes: Option<Vec<u8>> = self
            .protected_store
            .load_user_secret("device_encryption_key")
            .await?;
        let Some(dek_bytes) = dek_bytes else {
            debug!("no device encryption key");
            return Ok(None);
        };

        // Get device encryption key protection password if we have it
        let device_encryption_key_password = {
            let c = self.config.get();
            c.protected_store.device_encryption_key_password.clone()
        };

        Ok(Some(self.maybe_unprotect_device_encryption_key(
            &dek_bytes,
            &device_encryption_key_password,
        )?))
    }
    async fn save_device_encryption_key(
        &self,
        device_encryption_key: Option<TypedSharedSecret>,
    ) -> EyreResult<()> {
        let Some(device_encryption_key) = device_encryption_key else {
            // Remove the device encryption key
            let existed = self
                .protected_store
                .remove_user_secret("device_encryption_key")
                .await?;
            debug!("removed device encryption key. existed: {}", existed);
            return Ok(());
        };

        // Get new device encryption key protection password if we are changing it
        let new_device_encryption_key_password = {
            let c = self.config.get();
            c.protected_store.new_device_encryption_key_password.clone()
        };
        let device_encryption_key_password =
            if let Some(new_device_encryption_key_password) = new_device_encryption_key_password {
                // Change password
                debug!("changing dek password");
                self.config
                    .with_mut(|c| {
                        c.protected_store.device_encryption_key_password =
                            new_device_encryption_key_password.clone();
                        Ok(new_device_encryption_key_password)
                    })
                    .unwrap()
            } else {
                // Get device encryption key protection password if we have it
                debug!("saving with existing dek password");
                let c = self.config.get();
                c.protected_store.device_encryption_key_password.clone()
            };

        let dek_bytes = self.maybe_protect_device_encryption_key(
            device_encryption_key,
            &device_encryption_key_password,
        )?;

        // Save the new device encryption key
        let existed = self
            .protected_store
            .save_user_secret("device_encryption_key", &dek_bytes)
            .await?;
        debug!("saving device encryption key. existed: {}", existed);
        Ok(())
    }

    pub(crate) async fn init(&self) -> EyreResult<()> {
        let _async_guard = self.async_lock.lock().await;

        // Get device encryption key from protected store
        let mut device_encryption_key = self.load_device_encryption_key().await?;
        let mut device_encryption_key_changed = false;
        if let Some(device_encryption_key) = device_encryption_key {
            // If encryption in current use is not the best encryption, then run table migration
            let best_kind = best_crypto_kind();
            if device_encryption_key.kind != best_kind {
                // XXX: Run migration. See issue #209
            }
        } else {
            // If we don't have an encryption key yet, then make one with the best cryptography and save it
            let best_kind = best_crypto_kind();
            let mut shared_secret = SharedSecret::default();
            random_bytes(&mut shared_secret.bytes);

            device_encryption_key = Some(TypedSharedSecret::new(best_kind, shared_secret));
            device_encryption_key_changed = true;
        }

        // Check for password change
        let changing_password = self
            .config
            .get()
            .protected_store
            .new_device_encryption_key_password
            .is_some();

        // Save encryption key if it has changed or if the protecting password wants to change
        if device_encryption_key_changed || changing_password {
            self.save_device_encryption_key(device_encryption_key)
                .await?;
        }

        // Deserialize all table names
        let all_tables_db = self
            .table_store_driver
            .open("__veilid_all_tables", 1)
            .await
            .wrap_err("failed to create all tables table")?;
        match all_tables_db.get(0, ALL_TABLE_NAMES).await {
            Ok(Some(v)) => match deserialize_json_bytes::<HashMap<String, String>>(&v) {
                Ok(all_table_names) => {
                    let mut inner = self.inner.lock();
                    inner.all_table_names = all_table_names;
                }
                Err(e) => {
                    error!("could not deserialize __veilid_all_tables: {}", e);
                }
            },
            Ok(None) => {
                // No table names yet, that's okay
                trace!("__veilid_all_tables is empty");
            }
            Err(e) => {
                error!("could not get __veilid_all_tables: {}", e);
            }
        };

        {
            let mut inner = self.inner.lock();
            inner.encryption_key = device_encryption_key;
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

        self.flush().await;

        let mut inner = self.inner.lock();
        if !inner.opened.is_empty() {
            panic!(
                "all open databases should have been closed: {:?}",
                inner.opened
            );
        }
        inner.all_tables_db = None;
        inner.all_table_names.clear();
        inner.encryption_key = None;
    }

    pub(crate) fn on_table_db_drop(&self, table: String) {
        log_rtab!("dropping table db: {}", table);
        let mut inner = self.inner.lock();
        if inner.opened.remove(&table).is_none() {
            unreachable!("should have removed an item");
        }
    }

    /// Get or create a TableDB database table. If the column count is greater than an
    /// existing TableDB's column count, the database will be upgraded to add the missing columns
    pub async fn open(&self, name: &str, column_count: u32) -> VeilidAPIResult<TableDB> {
        let _async_guard = self.async_lock.lock().await;

        // If we aren't initialized yet, bail
        {
            let inner = self.inner.lock();
            if inner.all_tables_db.is_none() {
                apibail_not_initialized!();
            }
        }

        let table_name = self.name_get_or_create(name).await?;

        // See if this table is already opened, if so the column count must be the same
        {
            let mut inner = self.inner.lock();
            if let Some(table_db_weak_inner) = inner.opened.get(&table_name) {
                match TableDB::try_new_from_weak_inner(table_db_weak_inner.clone(), column_count) {
                    Some(tdb) => {
                        // Ensure column count isnt bigger
                        let existing_col_count = tdb.get_column_count()?;
                        if column_count > existing_col_count {
                            return Err(VeilidAPIError::generic(format!(
                                "database must be closed before increasing column count {} -> {}",
                                existing_col_count, column_count,
                            )));
                        }

                        return Ok(tdb);
                    }
                    None => {
                        inner.opened.remove(&table_name);
                    }
                };
            }
        }

        // Open table db using platform-specific driver
        let mut db = match self
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

        // If more columns are available, open the low level db with the max column count but restrict the tabledb object to the number requested
        let existing_col_count = db.num_columns().map_err(VeilidAPIError::from)?;
        if existing_col_count > column_count {
            drop(db);
            db = match self
                .table_store_driver
                .open(&table_name, existing_col_count)
                .await
            {
                Ok(db) => db,
                Err(e) => {
                    self.name_delete(name).await.expect("cleanup failed");
                    self.flush().await;
                    return Err(e);
                }
            };
        }

        // Wrap low-level Database in TableDB object
        let mut inner = self.inner.lock();
        let table_db = TableDB::new(
            table_name.clone(),
            self.clone(),
            inner.crypto.as_ref().unwrap().clone(),
            db,
            inner.encryption_key.clone(),
            inner.encryption_key.clone(),
            column_count,
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
        // If we aren't initialized yet, bail
        {
            let inner = self.inner.lock();
            if inner.all_tables_db.is_none() {
                apibail_not_initialized!();
            }
        }

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
            warn!(
                "table existed in name table but not in storage: {} : {}",
                name, table_name
            );
        }
        self.name_delete(&name)
            .await
            .expect("failed to delete name");
        self.flush().await;

        Ok(true)
    }

    /// Rename a TableDB table
    pub async fn rename(&self, old_name: &str, new_name: &str) -> VeilidAPIResult<()> {
        let _async_guard = self.async_lock.lock().await;
        // If we aren't initialized yet, bail
        {
            let inner = self.inner.lock();
            if inner.all_tables_db.is_none() {
                apibail_not_initialized!();
            }
        }
        trace!("TableStore::rename {} -> {}", old_name, new_name);
        self.name_rename(old_name, new_name).await?;
        self.flush().await;
        Ok(())
    }
}
