use super::*;

/// Locked structure for storage manager
#[derive(Default)]
pub(super) struct StorageManagerInner {
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
    pub fn open_record_check_existing(
        &mut self,
        key: TypedKey,
        writer: Option<KeyPair>,
        safety_selection: SafetySelection,
    ) -> Option<Result<DHTRecordDescriptor, VeilidAPIError>> {
        // Get local record store
        let Some(local_record_store) = self.local_record_store.as_mut() else {
            return Some(Err(VeilidAPIError::not_initialized()));
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
            return None;
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
        Some(Ok(descriptor))
    }

    pub async fn new_local_record(
        &mut self,
        key: TypedKey,
        subkey: ValueSubkey,
        signed_value_descriptor: SignedValueDescriptor,
        signed_value_data: Option<SignedValueData>,
        safety_selection: SafetySelection,
    ) -> Result<(), VeilidAPIError> {
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
        if let Some(signed_value_data) = signed_value_data {
            // Write subkey to local store
            local_record_store
                .set_subkey(key, subkey, signed_value_data)
                .await?;
        }
        // Write open record
        self.opened_records
            .insert(key, OpenedRecord::new(writer, safety_selection));

        Ok(())
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
}
