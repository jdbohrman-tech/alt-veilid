use super::*;

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
pub struct ValueRecordData {
    data: ValueData,
    signature: Signature,
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
pub struct ValueRecord {
    secret: Option<SecretKey>,
    schema: DHTSchema,
    safety_selection: SafetySelection,
    total_size: usize,
    subkeys: Vec<ValueRecordData>,
}

impl ValueRecord {
    pub fn new(
        secret: Option<SecretKey>,
        schema: DHTSchema,
        safety_selection: SafetySelection,
    ) -> Self {
        // Get number of subkeys
        let subkey_count = schema.subkey_count();

        Self {
            secret,
            schema,
            safety_selection,
            subkeys: vec![Vec::new(); subkey_count],
        }
    }
}
