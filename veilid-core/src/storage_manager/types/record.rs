use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(in crate::storage_manager) struct Record<D>
where
    D: fmt::Debug + Serialize + Clone,
{
    descriptor: Arc<SignedValueDescriptor>,
    subkey_count: usize,
    stored_subkeys: ValueSubkeyRangeSet,
    last_touched_ts: Timestamp,
    record_data_size: usize,
    detail: D,
}

impl<D> Record<D>
where
    D: fmt::Debug + Serialize + Clone,
{
    pub fn new(
        cur_ts: Timestamp,
        descriptor: Arc<SignedValueDescriptor>,
        detail: D,
    ) -> VeilidAPIResult<Self> {
        let schema = descriptor.schema()?;
        let subkey_count = schema.subkey_count();
        Ok(Self {
            descriptor,
            subkey_count,
            stored_subkeys: ValueSubkeyRangeSet::new(),
            last_touched_ts: cur_ts,
            record_data_size: 0,
            detail,
        })
    }

    pub fn descriptor(&self) -> Arc<SignedValueDescriptor> {
        self.descriptor.clone()
    }
    pub fn owner(&self) -> &PublicKey {
        self.descriptor.owner()
    }

    pub fn subkey_count(&self) -> usize {
        self.subkey_count
    }

    pub fn stored_subkeys(&self) -> &ValueSubkeyRangeSet {
        &self.stored_subkeys
    }
    pub fn store_subkey(&mut self, subkey: ValueSubkey) {
        self.stored_subkeys.insert(subkey);
    }

    pub fn touch(&mut self, cur_ts: Timestamp) {
        self.last_touched_ts = cur_ts
    }

    pub fn last_touched(&self) -> Timestamp {
        self.last_touched_ts
    }

    pub fn set_record_data_size(&mut self, size: usize) {
        self.record_data_size = size;
    }

    pub fn record_data_size(&self) -> usize {
        self.record_data_size
    }

    pub fn schema(&self) -> DHTSchema {
        // unwrap is safe here because descriptor is immutable and set in new()
        self.descriptor.schema().unwrap()
    }

    pub fn total_size(&self) -> usize {
        (mem::size_of::<Self>() - mem::size_of::<Arc<SignedValueDescriptor>>())
            + self.descriptor.total_size()
            + self.record_data_size
    }

    pub fn detail(&self) -> &D {
        &self.detail
    }
    pub fn detail_mut(&mut self) -> &mut D {
        &mut self.detail
    }
}
