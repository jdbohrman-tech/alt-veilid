use super::*;

// Keep member order appropriate for sorting < preference
// Must match DialInfo order
#[allow(clippy::derive_hash_xor_eq)]
#[derive(
    Debug,
    PartialOrd,
    Ord,
    Hash,
    EnumSetType,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[enumset(repr = "u8")]
#[archive_attr(repr(u8), derive(CheckBytes))]
pub enum LowLevelProtocolType {
    UDP,
    TCP,
}

impl LowLevelProtocolType {
    pub fn is_connection_oriented(&self) -> bool {
        matches!(self, LowLevelProtocolType::TCP)
    }
}

// pub type LowLevelProtocolTypeSet = EnumSet<LowLevelProtocolType>;
