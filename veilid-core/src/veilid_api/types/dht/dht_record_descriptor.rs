use super::*;

/// DHT Record Descriptor
#[derive(
    Debug,
    Clone,
    PartialOrd,
    Ord,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct DHTRecordDescriptor {
    owner: PublicKey,
    schema: DHTSchema,
}

impl DHTRecordDescriptor {
    pub fn new(owner: PublicKey, schema: DHTSchema) -> Self {
        Self { owner, schema }
    }

    pub fn owner(&self) -> PublicKey {
        self.owner
    }

    pub fn schema(&self) -> DHTSchema {
        self.schema
    }
}
