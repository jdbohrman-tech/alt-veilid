#![allow(non_snake_case)]

use super::*;

// Routing domain here is listed in order of preference, keep in order
#[allow(clippy::derive_hash_xor_eq)]
#[derive(
    Debug,
    Ord,
    PartialOrd,
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
pub enum RoutingDomain {
    LocalNetwork = 0,
    PublicInternet = 1,
}
impl RoutingDomain {
    pub const fn count() -> usize {
        2
    }
    pub const fn all() -> [RoutingDomain; RoutingDomain::count()] {
        // Routing domain here is listed in order of preference, keep in order
        [RoutingDomain::LocalNetwork, RoutingDomain::PublicInternet]
    }
}
pub type RoutingDomainSet = EnumSet<RoutingDomain>;
