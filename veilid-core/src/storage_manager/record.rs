use super::*;
use rkyv::{Archive as RkyvArchive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use serde::*;

#[derive(
    Clone, Debug, PartialEq, Eq, Serialize, Deserialize, RkyvArchive, RkyvSerialize, RkyvDeserialize,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct Record {
    last_touched_ts: Timestamp,
    descriptor: SignedValueDescriptor,
    subkey_count: usize,

    owner_secret: Option<SecretKey>,
    safety_selection: SafetySelection,
    record_data_size: usize,
}

impl Record {
    pub fn new(
        cur_ts: Timestamp,
        descriptor: SignedValueDescriptor,
        owner_secret: Option<SecretKey>,
        safety_selection: SafetySelection,
    ) -> Result<Self, VeilidAPIError> {
        let schema = descriptor.schema()?;
        let subkey_count = schema.subkey_count();
        Ok(Self {
            last_touched_ts: cur_ts,
            descriptor,
            subkey_count,
            owner_secret,
            safety_selection,
            record_data_size: 0,
        })
    }

    pub fn descriptor(&self) -> &SignedValueDescriptor {
        &self.descriptor
    }
    pub fn owner(&self) -> &PublicKey {
        self.descriptor.owner()
    }

    pub fn subkey_count(&self) -> usize {
        self.subkey_count
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
        mem::size_of::<Record>() + self.descriptor.total_size() + self.record_data_size
    }
}
