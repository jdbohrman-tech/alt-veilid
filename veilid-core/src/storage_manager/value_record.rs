use super::*;
use rkyv::{Archive as RkyvArchive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use serde::*;

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct RecordData {
    pub value_data: ValueData,
    pub signature: Signature,
}

impl RecordData {
    pub fn total_size(&self) -> usize {
        mem::size_of::<ValueData>() + self.value_data.data().len()
    }
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct Record {
    last_touched_ts: Timestamp,
    secret: Option<SecretKey>,
    schema: DHTSchema,
    safety_selection: SafetySelection,
    record_data_size: usize,
}

impl Record {
    pub fn new(
        cur_ts: Timestamp,
        secret: Option<SecretKey>,
        schema: DHTSchema,
        safety_selection: SafetySelection,
    ) -> Self {
        Self {
            last_touched_ts: cur_ts,
            secret,
            schema,
            safety_selection,
            record_data_size: 0,
        }
    }

    pub fn subkey_count(&self) -> usize {
        self.schema.subkey_count()
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

    pub fn total_size(&self) -> usize {
        mem::size_of::<Record>() + self.schema.data_size() + self.record_data_size
    }
}
