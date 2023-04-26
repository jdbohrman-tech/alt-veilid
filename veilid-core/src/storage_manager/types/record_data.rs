use super::*;
use rkyv::{Archive as RkyvArchive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use serde::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct RecordData {
    signed_value_data: SignedValueData,
}

impl RecordData {
    pub fn new(signed_value_data: SignedValueData) -> Self {
        Self { signed_value_data }
    }
    pub fn signed_value_data(&self) -> &SignedValueData {
        &self.signed_value_data
    }
    pub fn total_size(&self) -> usize {
        mem::size_of::<RecordData>() + self.signed_value_data.value_data().data().len()
    }
}
