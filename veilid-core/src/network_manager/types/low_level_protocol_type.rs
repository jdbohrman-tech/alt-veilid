#![allow(non_snake_case)]

use super::*;

// Keep member order appropriate for sorting < preference
// Must match DialInfo order
#[allow(clippy::derived_hash_with_manual_eq)]
#[derive(Debug, PartialOrd, Ord, Hash, EnumSetType, Serialize, Deserialize)]
#[enumset(repr = "u8")]
pub enum LowLevelProtocolType {
    UDP = 0,
    TCP = 1,
}

impl LowLevelProtocolType {
    pub fn is_connection_oriented(&self) -> bool {
        matches!(self, LowLevelProtocolType::TCP)
    }
}

// pub type LowLevelProtocolTypeSet = EnumSet<LowLevelProtocolType>;
