use super::*;
use rkyv::{Archive as RkyvArchive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use serde::*;

/////////////////////////////////////////////////////////////////////////////////////////////////////
///

#[derive(
    Clone,
    Debug,
    PartialOrd,
    PartialEq,
    Eq,
    Ord,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct ValueDetail {
    signed_value_data: SignedValueData,
    descriptor: Option<SignedValueDescriptor>,
}

impl ValueDetail {
    pub fn new(
        signed_value_data: SignedValueData,
        descriptor: Option<SignedValueDescriptor>,
    ) -> Self {
        Self {
            signed_value_data,
            descriptor,
        }
    }
    pub fn signed_value_data(&self) -> &SignedValueData {
        &self.signed_value_data
    }
    pub fn descriptor(&self) -> Option<&SignedValueDescriptor> {
        self.descriptor.as_ref()
    }
}
