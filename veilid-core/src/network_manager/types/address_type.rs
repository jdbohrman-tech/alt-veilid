#![allow(non_snake_case)]
use super::*;

#[allow(clippy::derived_hash_with_manual_eq)]
#[derive(Debug, PartialOrd, Ord, Hash, Serialize, Deserialize, EnumSetType)]
#[enumset(repr = "u8")]
pub enum AddressType {
    IPV6 = 0,
    IPV4 = 1,
}
pub type AddressTypeSet = EnumSet<AddressType>;
