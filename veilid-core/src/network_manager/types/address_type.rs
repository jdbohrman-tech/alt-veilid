use super::*;

#[allow(clippy::derive_hash_xor_eq)]
#[derive(
    Debug,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
    EnumSetType,
)]
#[enumset(repr = "u8")]
#[archive_attr(repr(u8), derive(CheckBytes))]
pub enum AddressType {
    IPV4,
    IPV6,
}
pub type AddressTypeSet = EnumSet<AddressType>;
