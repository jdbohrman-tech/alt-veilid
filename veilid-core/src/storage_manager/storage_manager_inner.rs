use super::*;

const STORAGE_MANAGER_METADATA: &str = "storage_manager_metadata";
const OFFLINE_SUBKEY_WRITES: &[u8] = b"offline_subkey_writes";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(super) struct OfflineSubkeyWrite {
    pub safety_selection: SafetySelection,
    pub subkeys: ValueSubkeyRangeSet,
}

/// Locked structure for storage manager
pub(super) struct StorageManagerInner {
    unlocked_inner: Arc<StorageManagerUnlockedInner>,
    /// If we are started up
    pub initialized: bool,
    /// Records that have been 'opened' and are not yet closed
    pub opened_records: HashMap<TypedKey, OpenedRecord>,
    /// Records that have ever been 'created' or 'opened' by this node, things we care about that we must republish to keep alive
    pub local_record_store: Option<RecordStore<LocalRecordDetail>>,
    /// Records that have been pushed to this node for distribution by other nodes, that we make an effort to republish
    pub remote_record_store: Option<RecordStore<RemoteRecordDetail>>,
    /// Record subkeys that have not been pushed to the network because they were written to offline
    pub offline_subkey_writes: HashMap<TypedKey, OfflineSubkeyWrite>,
    /// Storage manager metadata that is persistent, including copy of offline subkey writes
    pub metadata_db: Option<TableDB>,
    /// RPC processor if it is available
    pub rpc_processor: Option<RPCProcessor>,
    /// Background processing task (not part of attachment manager tick tree so it happens when detached too)
    pub tick_future: Option<SendPinBoxFuture<()>>,
    /// Update callback to send ValueChanged notification to
    pub update_callback: Option<UpdateCallback>,
}

fn local_limits_from_config(config: VeilidConfig) -> RecordStoreLimits {
    let c = config.get();
    RecordStoreLimits {
        subkey_cache_size: c.network.dht.local_subkey_cache_size as usize,
        max_subkey_size: MAX_SUBKEY_SIZE,
        max_record_total_size: MAX_RECORD_DATA_SIZE,
        max_records: None,
        max_subkey_cache_memory_mb: Some(c.network.dht.local_max_subkey_cache_memory_mb as usize),
        max_storage_space_mb: None,
        public_watch_limit: c.network.dht.public_watch_limit as usize,
        member_watch_limit: c.network.dht.member_watch_limit as usize,
        max_watch_expiration: TimestampDuration::new(ms_to_us(
            c.network.dht.max_watch_expiration_ms,
        )),
        min_watch_expiration: TimestampDuration::new(ms_to_us(c.network.rpc.timeout_ms)),
    }
}

fn remote_limits_from_config(config: VeilidConfig) -> RecordStoreLimits {
    let c = config.get();
    RecordStoreLimits {
        subkey_cache_size: c.network.dht.remote_subkey_cache_size as usize,
        max_subkey_size: MAX_SUBKEY_SIZE,
        max_record_total_size: MAX_RECORD_DATA_SIZE,
        max_records: Some(c.network.dht.remote_max_records as usize),
        max_subkey_cache_memory_mb: Some(c.network.dht.remote_max_subkey_cache_memory_mb as usize),
        max_storage_space_mb: Some(c.network.dht.remote_max_storage_space_mb as usize),
        public_watch_limit: c.network.dht.public_watch_limit as usize,
        member_watch_limit: c.network.dht.member_watch_limit as usize,
        max_watch_expiration: TimestampDuration::new(ms_to_us(
            c.network.dht.max_watch_expiration_ms,
        )),
        min_watch_expiration: TimestampDuration::new(ms_to_us(c.network.rpc.timeout_ms)),
    }
}

impl StorageManagerInner {
    pub fn new(unlocked_inner: Arc<StorageManagerUnlockedInner>) -> Self {
        Self {
            unlocked_inner,
            initialized: false,
            opened_records: Default::default(),
            local_record_store: Default::default(),
            remote_record_store: Default::default(),
            offline_subkey_writes: Default::default(),
            metadata_db: Default::default(),
            rpc_processor: Default::default(),
            tick_future: Default::default(),
            update_callback: None,
        }
    }

    pub async fn init(
        &mut self,
        outer_self: StorageManager,
        update_callback: UpdateCallback,
    ) -> EyreResult<()> {
        let metadata_db = self
            .unlocked_inner
            .table_store
            .open(STORAGE_MANAGER_METADATA, 1)
            .await?;

        let local_limits = local_limits_from_config(self.unlocked_inner.config.clone());
        let remote_limits = remote_limits_from_config(self.unlocked_inner.config.clone());

        let mut local_record_store = RecordStore::new(
            self.unlocked_inner.table_store.clone(),
            "local",
            local_limits,
        );
        local_record_store.init().await?;

        let mut remote_record_store = RecordStore::new(
            self.unlocked_inner.table_store.clone(),
            "remote",
            remote_limits,
        );
        remote_record_store.init().await?;

        self.metadata_db = Some(metadata_db);
        self.local_record_store = Some(local_record_store);
        self.remote_record_store = Some(remote_record_store);

        self.load_metadata().await?;

        // Schedule tick
        let tick_future = interval(1000, move || {
            let this = outer_self.clone();
            async move {
                if let Err(e) = this.tick().await {
                    log_stor!(warn "storage manager tick failed: {}", e);
                }
            }
        });
        self.tick_future = Some(tick_future);
        self.update_callback = Some(update_callback);
        self.initialized = true;

        Ok(())
    }

    pub async fn terminate(&mut self) {
        self.update_callback = None;

        // Stop ticker
        let tick_future = self.tick_future.take();
        if let Some(f) = tick_future {
            f.await;
        }

        // Final flush on record stores
        if let Some(mut local_record_store) = self.local_record_store.take() {
            if let Err(e) = local_record_store.flush().await {
                log_stor!(error "termination local record store tick failed: {}", e);
            }
        }
        if let Some(mut remote_record_store) = self.remote_record_store.take() {
            if let Err(e) = remote_record_store.flush().await {
                log_stor!(error "termination remote record store tick failed: {}", e);
            }
        }

        // Save metadata
        if self.metadata_db.is_some() {
            if let Err(e) = self.save_metadata().await {
                log_stor!(error "termination metadata save failed: {}", e);
            }
            self.metadata_db = None;
        }
        self.offline_subkey_writes.clear();

        // Mark not initialized
        self.initialized = false;
    }

    async fn save_metadata(&mut self) -> EyreResult<()> {
        if let Some(metadata_db) = &self.metadata_db {
            let tx = metadata_db.transact();
            tx.store_json(0, OFFLINE_SUBKEY_WRITES, &self.offline_subkey_writes)?;
            tx.commit().await.wrap_err("failed to commit")?
        }
        Ok(())
    }

    async fn load_metadata(&mut self) -> EyreResult<()> {
        if let Some(metadata_db) = &self.metadata_db {
            self.offline_subkey_writes = match metadata_db.load_json(0, OFFLINE_SUBKEY_WRITES).await
            {
                Ok(v) => v.unwrap_or_default(),
                Err(_) => {
                    if let Err(e) = metadata_db.delete(0, OFFLINE_SUBKEY_WRITES).await {
                        debug!("offline_subkey_writes format changed, clearing: {}", e);
                    }
                    Default::default()
                }
            }
        }
        Ok(())
    }

    pub async fn create_new_owned_local_record(
        &mut self,
        kind: CryptoKind,
        schema: DHTSchema,
        safety_selection: SafetySelection,
    ) -> VeilidAPIResult<(TypedKey, KeyPair)> {
        // Get cryptosystem
        let Some(vcrypto) = self.unlocked_inner.crypto.get(kind) else {
            apibail_generic!("unsupported cryptosystem");
        };

        // Get local record store
        let Some(local_record_store) = self.local_record_store.as_mut() else {
            apibail_not_initialized!();
        };

        // Compile the dht schema
        let schema_data = schema.compile();

        // New values require a new owner key
        let owner = vcrypto.generate_keypair();

        // Make a signed value descriptor for this dht value
        let signed_value_descriptor = Arc::new(SignedValueDescriptor::make_signature(
            owner.key,
            schema_data,
            vcrypto.clone(),
            owner.secret,
        )?);

        // Add new local value record
        let cur_ts = get_aligned_timestamp();
        let local_record_detail = LocalRecordDetail {
            safety_selection,
            value_nodes: vec![],
        };
        let record =
            Record::<LocalRecordDetail>::new(cur_ts, signed_value_descriptor, local_record_detail)?;

        let dht_key = Self::get_key(vcrypto.clone(), &record);
        local_record_store.new_record(dht_key, record).await?;

        Ok((dht_key, owner))
    }

    async fn move_remote_record_to_local(
        &mut self,
        key: TypedKey,
        safety_selection: SafetySelection,
    ) -> VeilidAPIResult<Option<(PublicKey, DHTSchema)>> {
        // Get local record store
        let Some(local_record_store) = self.local_record_store.as_mut() else {
            apibail_not_initialized!();
        };

        // Get remote record store
        let Some(remote_record_store) = self.remote_record_store.as_mut() else {
            apibail_not_initialized!();
        };

        let rcb = |r: &Record<RemoteRecordDetail>| {
            // Return record details
            r.clone()
        };
        let Some(remote_record) = remote_record_store.with_record(key, rcb) else {
            // No local or remote record found, return None
            return Ok(None);
        };

        // Make local record
        let cur_ts = get_aligned_timestamp();
        let local_record = Record::new(
            cur_ts,
            remote_record.descriptor().clone(),
            LocalRecordDetail {
                safety_selection,
                value_nodes: vec![],
            },
        )?;
        local_record_store.new_record(key, local_record).await?;

        // Move copy subkey data from remote to local store
        for subkey in remote_record.stored_subkeys().iter() {
            let Some(subkey_result) = remote_record_store.get_subkey(key, subkey, false).await?
            else {
                // Subkey was missing
                warn!("Subkey was missing: {} #{}", key, subkey);
                continue;
            };
            let Some(subkey_data) = subkey_result.value else {
                // Subkey was missing
                warn!("Subkey data was missing: {} #{}", key, subkey);
                continue;
            };
            local_record_store
                .set_subkey(key, subkey, subkey_data)
                .await?;
        }

        // Delete remote record from store
        remote_record_store.delete_record(key).await?;

        // Return record information as transferred to local record
        Ok(Some((*remote_record.owner(), remote_record.schema())))
    }

    pub async fn open_existing_record(
        &mut self,
        key: TypedKey,
        writer: Option<KeyPair>,
        safety_selection: SafetySelection,
    ) -> VeilidAPIResult<Option<DHTRecordDescriptor>> {
        // Ensure the record is closed
        if self.opened_records.contains_key(&key) {
            apibail_generic!("record is already open and should be closed first");
        }

        // Get local record store
        let Some(local_record_store) = self.local_record_store.as_mut() else {
            apibail_not_initialized!();
        };

        // See if we have a local record already or not
        let cb = |r: &mut Record<LocalRecordDetail>| {
            // Process local record

            // Keep the safety selection we opened the record with
            r.detail_mut().safety_selection = safety_selection;

            // Return record details
            (*r.owner(), r.schema())
        };
        let (owner, schema) = match local_record_store.with_record_mut(key, cb) {
            Some(v) => v,
            None => {
                // If we don't have a local record yet, check to see if we have a remote record
                // if so, migrate it to a local record
                let Some(v) = self
                    .move_remote_record_to_local(key, safety_selection)
                    .await?
                else {
                    // No remote record either
                    return Ok(None);
                };
                v
            }
        };
        // Had local record

        // If the writer we chose is also the owner, we have the owner secret
        // Otherwise this is just another subkey writer
        let owner_secret = if let Some(writer) = writer {
            if writer.key == owner {
                Some(writer.secret)
            } else {
                None
            }
        } else {
            None
        };

        // Write open record
        self.opened_records
            .insert(key, OpenedRecord::new(writer, safety_selection));

        // Make DHT Record Descriptor to return
        let descriptor = DHTRecordDescriptor::new(key, owner, owner_secret, schema);
        Ok(Some(descriptor))
    }

    pub async fn open_new_record(
        &mut self,
        key: TypedKey,
        writer: Option<KeyPair>,
        subkey: ValueSubkey,
        subkey_result: SubkeyResult,
        safety_selection: SafetySelection,
    ) -> VeilidAPIResult<DHTRecordDescriptor> {
        // Ensure the record is closed
        if self.opened_records.contains_key(&key) {
            panic!("new record should never be opened at this point");
        }

        // Must have descriptor
        let Some(signed_value_descriptor) = subkey_result.descriptor else {
            // No descriptor for new record, can't store this
            apibail_generic!("no descriptor");
        };
        // Get owner
        let owner = *signed_value_descriptor.owner();

        // If the writer we chose is also the owner, we have the owner secret
        // Otherwise this is just another subkey writer
        let owner_secret = if let Some(writer) = writer {
            if writer.key == owner {
                Some(writer.secret)
            } else {
                None
            }
        } else {
            None
        };
        let schema = signed_value_descriptor.schema()?;

        // Get local record store
        let Some(local_record_store) = self.local_record_store.as_mut() else {
            apibail_not_initialized!();
        };

        // Make and store a new record for this descriptor
        let record = Record::<LocalRecordDetail>::new(
            get_aligned_timestamp(),
            signed_value_descriptor,
            LocalRecordDetail {
                safety_selection,
                value_nodes: vec![],
            },
        )?;
        local_record_store.new_record(key, record).await?;

        // If we got a subkey with the getvalue, it has already been validated against the schema, so store it
        if let Some(signed_value_data) = subkey_result.value {
            // Write subkey to local store
            local_record_store
                .set_subkey(key, subkey, signed_value_data)
                .await?;
        }

        // Write open record
        self.opened_records
            .insert(key, OpenedRecord::new(writer, safety_selection));

        // Make DHT Record Descriptor to return
        let descriptor = DHTRecordDescriptor::new(key, owner, owner_secret, schema);
        Ok(descriptor)
    }

    pub fn get_value_nodes(&self, key: TypedKey) -> VeilidAPIResult<Option<Vec<NodeRef>>> {
        // Get local record store
        let Some(local_record_store) = self.local_record_store.as_ref() else {
            apibail_not_initialized!();
        };

        // Get routing table to see if we still know about these nodes
        let Some(routing_table) = self.rpc_processor.as_ref().map(|r| r.routing_table()) else {
            apibail_try_again!("offline, try again later");
        };

        let opt_value_nodes = local_record_store.peek_record(key, |r| {
            let d = r.detail();
            d.value_nodes
                .iter()
                .copied()
                .filter_map(|x| {
                    routing_table
                        .lookup_node_ref(TypedKey::new(key.kind, x))
                        .ok()
                        .flatten()
                })
                .collect()
        });

        Ok(opt_value_nodes)
    }

    pub fn set_value_nodes(
        &mut self,
        key: TypedKey,
        value_nodes: Vec<NodeRef>,
    ) -> VeilidAPIResult<()> {
        // Get local record store
        let Some(local_record_store) = self.local_record_store.as_mut() else {
            apibail_not_initialized!();
        };
        local_record_store.with_record_mut(key, |r| {
            let d = r.detail_mut();
            d.value_nodes = value_nodes
                .into_iter()
                .filter_map(|x| x.node_ids().get(key.kind).map(|k| k.value))
                .collect();
        });
        Ok(())
    }

    pub fn close_record(&mut self, key: TypedKey) -> VeilidAPIResult<OpenedRecord> {
        let Some(opened_record) = self.opened_records.remove(&key) else {
            apibail_generic!("record not open");
        };
        Ok(opened_record)
    }

    pub(super) async fn handle_get_local_value(
        &mut self,
        key: TypedKey,
        subkey: ValueSubkey,
        want_descriptor: bool,
    ) -> VeilidAPIResult<SubkeyResult> {
        // See if it's in the local record store
        let Some(local_record_store) = self.local_record_store.as_mut() else {
            apibail_not_initialized!();
        };
        if let Some(subkey_result) = local_record_store
            .get_subkey(key, subkey, want_descriptor)
            .await?
        {
            return Ok(subkey_result);
        }

        Ok(SubkeyResult {
            value: None,
            descriptor: None,
        })
    }

    pub(super) async fn handle_set_local_value(
        &mut self,
        key: TypedKey,
        subkey: ValueSubkey,
        signed_value_data: Arc<SignedValueData>,
    ) -> VeilidAPIResult<()> {
        // See if it's in the local record store
        let Some(local_record_store) = self.local_record_store.as_mut() else {
            apibail_not_initialized!();
        };

        // Write subkey to local store
        local_record_store
            .set_subkey(key, subkey, signed_value_data)
            .await?;

        Ok(())
    }

    pub(super) async fn handle_watch_local_value(
        &mut self,
        key: TypedKey,
        subkeys: ValueSubkeyRangeSet,
        expiration: Timestamp,
        count: u32,
        target: Target,
        watcher: CryptoKey,
    ) -> VeilidAPIResult<Option<Timestamp>> {
        // See if it's in the local record store
        let Some(local_record_store) = self.local_record_store.as_mut() else {
            apibail_not_initialized!();
        };
        local_record_store
            .watch_record(key, subkeys, expiration, count, target, watcher)
            .await
    }

    pub(super) async fn handle_get_remote_value(
        &mut self,
        key: TypedKey,
        subkey: ValueSubkey,
        want_descriptor: bool,
    ) -> VeilidAPIResult<SubkeyResult> {
        // See if it's in the remote record store
        let Some(remote_record_store) = self.remote_record_store.as_mut() else {
            apibail_not_initialized!();
        };
        if let Some(subkey_result) = remote_record_store
            .get_subkey(key, subkey, want_descriptor)
            .await?
        {
            return Ok(subkey_result);
        }

        Ok(SubkeyResult {
            value: None,
            descriptor: None,
        })
    }

    pub(super) async fn handle_set_remote_value(
        &mut self,
        key: TypedKey,
        subkey: ValueSubkey,
        signed_value_data: Arc<SignedValueData>,
        signed_value_descriptor: Arc<SignedValueDescriptor>,
    ) -> VeilidAPIResult<()> {
        // See if it's in the remote record store
        let Some(remote_record_store) = self.remote_record_store.as_mut() else {
            apibail_not_initialized!();
        };

        // See if we have a remote record already or not
        if remote_record_store.with_record(key, |_| {}).is_none() {
            // record didn't exist, make it
            let cur_ts = get_aligned_timestamp();
            let remote_record_detail = RemoteRecordDetail {};
            let record = Record::<RemoteRecordDetail>::new(
                cur_ts,
                signed_value_descriptor,
                remote_record_detail,
            )?;
            remote_record_store.new_record(key, record).await?
        };

        // Write subkey to remote store
        remote_record_store
            .set_subkey(key, subkey, signed_value_data)
            .await?;

        Ok(())
    }

    pub(super) async fn handle_watch_remote_value(
        &mut self,
        key: TypedKey,
        subkeys: ValueSubkeyRangeSet,
        expiration: Timestamp,
        count: u32,
        target: Target,
        watcher: CryptoKey,
    ) -> VeilidAPIResult<Option<Timestamp>> {
        // See if it's in the remote record store
        let Some(remote_record_store) = self.remote_record_store.as_mut() else {
            apibail_not_initialized!();
        };
        remote_record_store
            .watch_record(key, subkeys, expiration, count, target, watcher)
            .await
    }

    /// # DHT Key = Hash(ownerKeyKind) of: [ ownerKeyValue, schema ]
    fn get_key<D>(vcrypto: CryptoSystemVersion, record: &Record<D>) -> TypedKey
    where
        D: fmt::Debug + Clone + Serialize,
    {
        let descriptor = record.descriptor();
        let compiled = descriptor.schema_data();
        let mut hash_data = Vec::<u8>::with_capacity(PUBLIC_KEY_LENGTH + 4 + compiled.len());
        hash_data.extend_from_slice(&vcrypto.kind().0);
        hash_data.extend_from_slice(&record.owner().bytes);
        hash_data.extend_from_slice(compiled);
        let hash = vcrypto.generate_hash(&hash_data);
        TypedKey::new(vcrypto.kind(), hash)
    }
}
