use super::*;

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
pub enum Direction {
    Inbound,
    Outbound,
}
pub type DirectionSet = EnumSet<Direction>;
