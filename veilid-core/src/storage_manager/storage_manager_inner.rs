use super::*;

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
    /// RPC processor if it is available
    pub rpc_processor: Option<RPCProcessor>,
    /// Background processing task (not part of attachment manager tick tree so it happens when detached too)
    pub tick_future: Option<SendPinBoxFuture<()>>,
}

impl StorageManagerInner {
    pub fn new(unlocked_inner: Arc<StorageManagerUnlockedInner>) -> Self {
        Self {
            unlocked_inner,
            initialized: false,
            opened_records: Default::default(),
            local_record_store: Default::default(),
            remote_record_store: Default::default(),
            rpc_processor: Default::default(),
            tick_future: Default::default(),
        }
    }

    pub async fn create_new_owned_local_record(
        &mut self,
        kind: CryptoKind,
        schema: DHTSchema,
        safety_selection: SafetySelection,
    ) -> Result<(TypedKey, KeyPair), VeilidAPIError> {
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
        let signed_value_descriptor = SignedValueDescriptor::make_signature(
            owner.key,
            schema_data,
            vcrypto.clone(),
            owner.secret,
        )?;

        // Add new local value record
        let cur_ts = get_aligned_timestamp();
        let local_record_detail = LocalRecordDetail { safety_selection };
        let record =
            Record::<LocalRecordDetail>::new(cur_ts, signed_value_descriptor, local_record_detail)?;

        let dht_key = Self::get_key(vcrypto.clone(), &record);
        local_record_store.new_record(dht_key, record).await?;

        Ok((dht_key, owner))
    }

    pub fn open_existing_record(
        &mut self,
        key: TypedKey,
        writer: Option<KeyPair>,
        safety_selection: SafetySelection,
    ) -> Result<Option<DHTRecordDescriptor>, VeilidAPIError> {
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
            (r.owner().clone(), r.schema())
        };
        let Some((owner, schema)) = local_record_store.with_record_mut(key, cb) else {
            return Ok(None);
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
    ) -> Result<DHTRecordDescriptor, VeilidAPIError> {
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
        let owner = signed_value_descriptor.owner().clone();

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
            LocalRecordDetail { safety_selection },
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

    pub fn close_record(&mut self, key: TypedKey) -> Result<(), VeilidAPIError> {
        let Some(_opened_record) = self.opened_records.remove(&key) else {
            apibail_generic!("record not open");
        };
        Ok(())
    }

    pub fn handle_get_local_value(
        &mut self,
        key: TypedKey,
        subkey: ValueSubkey,
        want_descriptor: bool,
    ) -> Result<SubkeyResult, VeilidAPIError> {
        // See if it's in the local record store
        let Some(local_record_store) = self.local_record_store.as_mut() else {
            apibail_not_initialized!();
        };
        if let Some(subkey_result) = local_record_store.get_subkey(key, subkey, want_descriptor)? {
            return Ok(subkey_result);
        }

        Ok(SubkeyResult {
            value: None,
            descriptor: None,
        })
    }

    pub async fn handle_set_local_value(
        &mut self,
        key: TypedKey,
        subkey: ValueSubkey,
        signed_value_data: SignedValueData,
    ) -> Result<(), VeilidAPIError> {
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

    pub fn handle_get_remote_value(
        &mut self,
        key: TypedKey,
        subkey: ValueSubkey,
        want_descriptor: bool,
    ) -> Result<SubkeyResult, VeilidAPIError> {
        // See if it's in the remote record store
        let Some(remote_record_store) = self.remote_record_store.as_mut() else {
            apibail_not_initialized!();
        };
        if let Some(subkey_result) = remote_record_store.get_subkey(key, subkey, want_descriptor)? {
            return Ok(subkey_result);
        }

        Ok(SubkeyResult {
            value: None,
            descriptor: None,
        })
    }

    pub async fn handle_set_remote_value(
        &mut self,
        key: TypedKey,
        subkey: ValueSubkey,
        signed_value_data: SignedValueData,
    ) -> Result<(), VeilidAPIError> {
        // See if it's in the remote record store
        let Some(remote_record_store) = self.remote_record_store.as_mut() else {
            apibail_not_initialized!();
        };

        // Write subkey to remote store
        remote_record_store
            .set_subkey(key, subkey, signed_value_data)
            .await?;

        Ok(())
    }

    /// # DHT Key = Hash(ownerKeyKind) of: [ ownerKeyValue, schema ]
    fn get_key<D>(vcrypto: CryptoSystemVersion, record: &Record<D>) -> TypedKey
    where
        D: Clone + RkyvArchive + RkyvSerialize<RkyvSerializer>,
        for<'t> <D as RkyvArchive>::Archived: CheckBytes<RkyvDefaultValidator<'t>>,
        <D as RkyvArchive>::Archived: RkyvDeserialize<D, SharedDeserializeMap>,
    {
        let compiled = record.descriptor().schema_data();
        let mut hash_data = Vec::<u8>::with_capacity(PUBLIC_KEY_LENGTH + 4 + compiled.len());
        hash_data.extend_from_slice(&vcrypto.kind().0);
        hash_data.extend_from_slice(&record.owner().bytes);
        hash_data.extend_from_slice(compiled);
        let hash = vcrypto.generate_hash(&hash_data);
        TypedKey::new(vcrypto.kind(), hash)
    }
}
