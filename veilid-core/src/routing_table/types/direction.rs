#![allow(non_snake_case)]

use super::*;

#[allow(clippy::derive_hash_xor_eq)]
#[derive(Debug, PartialOrd, Ord, Hash, EnumSetType, Serialize, Deserialize)]
#[enumset(repr = "u8")]
pub enum Direction {
    Inbound = 0,
    Outbound = 1,
}
pub type DirectionSet = EnumSet<Direction>;
