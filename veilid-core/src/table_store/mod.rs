use super::*;

mod table_db;
pub use table_db::*;

pub mod tests;

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
mod wasm;
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use wasm::*;
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
mod native;
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
use native::*;

use keyvaluedb::*;

impl_veilid_log_facility!("tstore");

const ALL_TABLE_NAMES: &[u8] = b"all_table_names";

/// Description of column
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), derive(Tsify))]
#[must_use]
pub struct ColumnInfo {
    pub key_count: AlignedU64,
}

/// IO Stats for table
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), derive(Tsify))]
#[must_use]
pub struct IOStatsInfo {
    /// Number of transaction.
    pub transactions: AlignedU64,
    /// Number of read operations.
    pub reads: AlignedU64,
    /// Number of reads resulted in a read from cache.
    pub cache_reads: AlignedU64,
    /// Number of write operations.
    pub writes: AlignedU64,
    /// Number of bytes read
    pub bytes_read: ByteCount,
    /// Number of bytes read from cache
    pub cache_read_bytes: ByteCount,
    /// Number of bytes write
    pub bytes_written: ByteCount,
    /// Start of the statistic period.
    pub started: Timestamp,
    /// Total duration of the statistic period.
    pub span: TimestampDuration,
}

/// Description of table
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), derive(Tsify))]
#[must_use]
pub struct TableInfo {
    /// Internal table name
    pub table_name: String,
    /// IO statistics since previous query
    pub io_stats_since_previous: IOStatsInfo,
    /// IO statistics since database open
    pub io_stats_overall: IOStatsInfo,
    /// Total number of columns in the table
    pub column_count: u32,
    /// Column descriptions
    pub columns: Vec<ColumnInfo>,
}

#[must_use]
struct TableStoreInner {
    opened: BTreeMap<String, Weak<TableDBUnlockedInner>>,
    encryption_key: Option<TypedSharedSecret>,
    all_table_names: HashMap<String, String>,
    all_tables_db: Option<Database>,
}

impl fmt::Debug for TableStoreInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TableStoreInner")
            .field("opened", &self.opened)
            .field("encryption_key", &self.encryption_key)
            .field("all_table_names", &self.all_table_names)
            //.field("all_tables_db", &self.all_tables_db)
            .finish()
    }
}

/// Veilid Table Storage.
/// Database for storing key value pairs persistently and securely across runs.
#[must_use]
pub struct TableStore {
    registry: VeilidComponentRegistry,
    inner: Mutex<TableStoreInner>, // Sync mutex here because TableDB drops can happen at any time
    table_store_driver: TableStoreDriver,
    async_lock: Arc<AsyncMutex<()>>, // Async mutex for operations
}

impl fmt::Debug for TableStore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TableStore")
            .field("registry", &self.registry)
            .field("inner", &self.inner)
            //.field("table_store_driver", &self.table_store_driver)
            .field("async_lock", &self.async_lock)
            .finish()
    }
}

impl_veilid_component!(TableStore);

impl TableStore {
    fn new_inner() -> TableStoreInner {
        TableStoreInner {
            opened: BTreeMap::new(),
            encryption_key: None,
            all_table_names: HashMap::new(),
            all_tables_db: None,
        }
    }
    pub(crate) fn new(registry: VeilidComponentRegistry) -> Self {
        let inner = Self::new_inner();
        let table_store_driver = TableStoreDriver::new(registry.clone());

        Self {
            registry,
            inner: Mutex::new(inner),
            table_store_driver,
            async_lock: Arc::new(AsyncMutex::new(())),
        }
    }

    // Flush internal control state
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
        let namespace = self.config().with(|c| c.namespace.clone());
        Ok(if namespace.is_empty() {
            table.to_string()
        } else {
            format!("_ns_{}_{}", namespace, table)
        })
    }

    fn name_get_or_create(&self, table: &str) -> VeilidAPIResult<String> {
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

    #[instrument(level = "trace", target = "tstore", skip_all)]
    async fn name_delete(&self, table: &str) -> VeilidAPIResult<Option<String>> {
        let name = self.namespaced_name(table)?;
        let mut inner = self.inner.lock();
        let real_name = inner.all_table_names.remove(&name);
        Ok(real_name)
    }

    #[instrument(level = "trace", target = "tstore", skip_all)]
    async fn name_get(&self, table: &str) -> VeilidAPIResult<Option<String>> {
        let name = self.namespaced_name(table)?;
        let inner = self.inner.lock();
        let real_name = inner.all_table_names.get(&name).cloned();
        Ok(real_name)
    }

    #[instrument(level = "trace", target = "tstore", skip_all)]
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

    /// List all known tables
    #[instrument(level = "trace", target = "tstore", skip_all)]
    pub fn list_all(&self) -> Vec<(String, String)> {
        let inner = self.inner.lock();
        inner
            .all_table_names
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect::<Vec<(String, String)>>()
    }

    /// Delete all known tables
    #[instrument(level = "trace", target = "tstore", skip_all)]
    pub async fn delete_all(&self) {
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

    #[instrument(level = "trace", target = "tstore", skip_all)]
    pub(crate) async fn maybe_unprotect_device_encryption_key(
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
        let crypto = self.crypto();
        let Some(vcrypto) = crypto.get_async(kind) else {
            bail!("unsupported cryptosystem '{kind}'");
        };

        if !device_encryption_key_password.is_empty() {
            if dek_bytes.len()
                != (4 + SHARED_SECRET_LENGTH + vcrypto.aead_overhead() + NONCE_LENGTH)
            {
                bail!("password protected device encryption key is not valid");
            }
            let protected_key = &dek_bytes[4..(4 + SHARED_SECRET_LENGTH + vcrypto.aead_overhead())];
            let nonce =
                Nonce::try_from(&dek_bytes[(4 + SHARED_SECRET_LENGTH + vcrypto.aead_overhead())..])
                    .wrap_err("invalid nonce")?;

            let shared_secret = vcrypto
                .derive_shared_secret(device_encryption_key_password.as_bytes(), &nonce.bytes)
                .await
                .wrap_err("failed to derive shared secret")?;

            let unprotected_key = vcrypto
                .decrypt_aead(protected_key, &nonce, &shared_secret, None)
                .await
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

    #[instrument(level = "trace", target = "tstore", skip_all)]
    pub(crate) async fn maybe_protect_device_encryption_key(
        &self,
        dek: TypedSharedSecret,
        device_encryption_key_password: &str,
    ) -> EyreResult<Vec<u8>> {
        // Check if we are to protect the key
        if device_encryption_key_password.is_empty() {
            veilid_log!(self debug "no dek password");
            // Return the unprotected key bytes
            let mut out = Vec::with_capacity(4 + SHARED_SECRET_LENGTH);
            out.extend_from_slice(&dek.kind.0);
            out.extend_from_slice(&dek.value.bytes);
            return Ok(out);
        }

        // Get cryptosystem
        let crypto = self.crypto();
        let Some(vcrypto) = crypto.get_async(dek.kind) else {
            bail!("unsupported cryptosystem '{}'", dek.kind);
        };

        let nonce = vcrypto.random_nonce().await;
        let shared_secret = vcrypto
            .derive_shared_secret(device_encryption_key_password.as_bytes(), &nonce.bytes)
            .await
            .wrap_err("failed to derive shared secret")?;
        let mut protected_key = vcrypto
            .encrypt_aead(&dek.value.bytes, &nonce, &shared_secret, None)
            .await
            .wrap_err("failed to decrypt device encryption key")?;
        let mut out =
            Vec::with_capacity(4 + SHARED_SECRET_LENGTH + vcrypto.aead_overhead() + NONCE_LENGTH);
        out.extend_from_slice(&dek.kind.0);
        out.append(&mut protected_key);
        out.extend_from_slice(&nonce.bytes);
        assert!(out.len() == 4 + SHARED_SECRET_LENGTH + vcrypto.aead_overhead() + NONCE_LENGTH);
        Ok(out)
    }

    #[instrument(level = "trace", target = "tstore", skip_all)]
    async fn load_device_encryption_key(&self) -> EyreResult<Option<TypedSharedSecret>> {
        let dek_bytes: Option<Vec<u8>> = self
            .protected_store()
            .load_user_secret("device_encryption_key")?;
        let Some(dek_bytes) = dek_bytes else {
            veilid_log!(self debug "no device encryption key");
            return Ok(None);
        };

        // Get device encryption key protection password if we have it
        let device_encryption_key_password = self
            .config()
            .with(|c| c.protected_store.device_encryption_key_password.clone());

        Ok(Some(
            self.maybe_unprotect_device_encryption_key(&dek_bytes, &device_encryption_key_password)
                .await?,
        ))
    }

    #[instrument(level = "trace", target = "tstore", skip_all)]
    async fn save_device_encryption_key(
        &self,
        device_encryption_key: Option<TypedSharedSecret>,
    ) -> EyreResult<()> {
        let Some(device_encryption_key) = device_encryption_key else {
            // Remove the device encryption key
            let existed = self
                .protected_store()
                .remove_user_secret("device_encryption_key")?;
            veilid_log!(self debug "removed device encryption key. existed: {}", existed);
            return Ok(());
        };

        // Get new device encryption key protection password if we are changing it
        let new_device_encryption_key_password = {
            self.config()
                .with(|c| c.protected_store.new_device_encryption_key_password.clone())
        };
        let device_encryption_key_password =
            if let Some(new_device_encryption_key_password) = new_device_encryption_key_password {
                // Change password
                veilid_log!(self debug "changing dek password");
                self.config()
                    .try_with_mut(|c| {
                        c.protected_store
                            .device_encryption_key_password
                            .clone_from(&new_device_encryption_key_password);
                        Ok(new_device_encryption_key_password)
                    })
                    .unwrap()
            } else {
                // Get device encryption key protection password if we have it
                veilid_log!(self debug "saving with existing dek password");
                self.config()
                    .with(|c| c.protected_store.device_encryption_key_password.clone())
            };

        let dek_bytes = self
            .maybe_protect_device_encryption_key(
                device_encryption_key,
                &device_encryption_key_password,
            )
            .await?;

        // Save the new device encryption key
        let existed = self
            .protected_store()
            .save_user_secret("device_encryption_key", &dek_bytes)?;
        veilid_log!(self debug "saving device encryption key. existed: {}", existed);
        Ok(())
    }

    #[instrument(level = "trace", target = "tstore", skip_all)]
    async fn init_async(&self) -> EyreResult<()> {
        {
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
            let changing_password = self.config().with(|c| {
                c.protected_store
                    .new_device_encryption_key_password
                    .is_some()
            });

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
                    veilid_log!(self trace "__veilid_all_tables is empty");
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

            let do_delete = self.config().with(|c| c.table_store.delete);

            if do_delete {
                self.delete_all().await;
            }
        }

        // Set up crypto
        let crypto = self.crypto();
        crypto.table_store_setup(self).await?;

        Ok(())
    }

    #[instrument(level = "trace", target = "tstore", skip_all)]
    async fn post_init_async(&self) -> EyreResult<()> {
        Ok(())
    }

    #[instrument(level = "trace", target = "tstore", skip_all)]
    async fn pre_terminate_async(&self) {}

    #[instrument(level = "trace", target = "tstore", skip_all)]
    async fn terminate_async(&self) {
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

    #[instrument(level = "trace", target = "tstore", skip_all)]
    pub(crate) fn on_table_db_drop(&self, table: String) {
        veilid_log!(self trace "dropping table db: {}", table);
        let mut inner = self.inner.lock();
        if inner.opened.remove(&table).is_none() {
            unreachable!("should have removed an item");
        }
    }

    /// Get or create a TableDB database table. If the column count is greater than an
    /// existing TableDB's column count, the database will be upgraded to add the missing columns.
    #[instrument(level = "trace", target = "tstore", skip_all)]
    pub async fn open(&self, name: &str, column_count: u32) -> VeilidAPIResult<TableDB> {
        let _async_guard = self.async_lock.lock().await;

        // If we aren't initialized yet, bail
        {
            let inner = self.inner.lock();
            if inner.all_tables_db.is_none() {
                apibail_not_initialized!();
            }
        }

        let table_name = self.name_get_or_create(name)?;

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
            self.registry(),
            db,
            inner.encryption_key,
            inner.encryption_key,
            column_count,
        );

        // Keep track of opened DBs
        inner
            .opened
            .insert(table_name.clone(), table_db.weak_unlocked_inner());

        Ok(table_db)
    }

    /// Delete a TableDB table by name
    #[instrument(level = "trace", target = "tstore", skip_all)]
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
            veilid_log!(self warn
                "table existed in name table but not in storage: {} : {}",
                name, table_name
            );
        }
        self.name_delete(name).await.expect("failed to delete name");
        self.flush().await;

        Ok(true)
    }

    /// Get the description of a TableDB table
    #[instrument(level = "trace", target = "tstore", skip_all)]
    pub async fn info(&self, name: &str) -> VeilidAPIResult<Option<TableInfo>> {
        // Open with the default number of columns
        let tdb = self.open(name, 0).await?;
        let internal_name = tdb.table_name();
        let io_stats_since_previous = tdb.io_stats(IoStatsKind::SincePrevious);
        let io_stats_overall = tdb.io_stats(IoStatsKind::Overall);
        let column_count = tdb.get_column_count()?;
        let mut columns = Vec::<ColumnInfo>::with_capacity(column_count as usize);
        for col in 0..column_count {
            let key_count = tdb.get_key_count(col).await?;
            columns.push(ColumnInfo {
                key_count: AlignedU64::new(key_count),
            })
        }
        Ok(Some(TableInfo {
            table_name: internal_name,
            io_stats_since_previous: IOStatsInfo {
                transactions: AlignedU64::new(io_stats_since_previous.transactions),
                reads: AlignedU64::new(io_stats_since_previous.reads),
                cache_reads: AlignedU64::new(io_stats_since_previous.cache_reads),
                writes: AlignedU64::new(io_stats_since_previous.writes),
                bytes_read: ByteCount::new(io_stats_since_previous.bytes_read),
                cache_read_bytes: ByteCount::new(io_stats_since_previous.cache_read_bytes),
                bytes_written: ByteCount::new(io_stats_since_previous.bytes_written),
                started: Timestamp::new(
                    io_stats_since_previous
                        .started
                        .duration_since(std::time::SystemTime::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_micros() as u64,
                ),
                span: TimestampDuration::new(io_stats_since_previous.span.as_micros() as u64),
            },
            io_stats_overall: IOStatsInfo {
                transactions: AlignedU64::new(io_stats_overall.transactions),
                reads: AlignedU64::new(io_stats_overall.reads),
                cache_reads: AlignedU64::new(io_stats_overall.cache_reads),
                writes: AlignedU64::new(io_stats_overall.writes),
                bytes_read: ByteCount::new(io_stats_overall.bytes_read),
                cache_read_bytes: ByteCount::new(io_stats_overall.cache_read_bytes),
                bytes_written: ByteCount::new(io_stats_overall.bytes_written),
                started: Timestamp::new(
                    io_stats_overall
                        .started
                        .duration_since(std::time::SystemTime::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_micros() as u64,
                ),
                span: TimestampDuration::new(io_stats_overall.span.as_micros() as u64),
            },
            column_count,
            columns,
        }))
    }

    /// Rename a TableDB table
    #[instrument(level = "trace", target = "tstore", skip_all)]
    pub async fn rename(&self, old_name: &str, new_name: &str) -> VeilidAPIResult<()> {
        let _async_guard = self.async_lock.lock().await;
        // If we aren't initialized yet, bail
        {
            let inner = self.inner.lock();
            if inner.all_tables_db.is_none() {
                apibail_not_initialized!();
            }
        }
        veilid_log!(self debug "TableStore::rename {} -> {}", old_name, new_name);
        self.name_rename(old_name, new_name).await?;
        self.flush().await;
        Ok(())
    }
}
