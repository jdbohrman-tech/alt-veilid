use super::*;

use core::cmp::{Eq, Ord, PartialEq, PartialOrd};
use core::convert::TryInto;
use core::fmt;
use core::hash::Hash;

/// Cryptography version fourcc code
#[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), declare)]
pub type CryptoKind = FourCC;

/// Sort best crypto kinds first
/// Better crypto kinds are 'less', ordered toward the front of a list
#[must_use]
pub fn compare_crypto_kind(a: &CryptoKind, b: &CryptoKind) -> cmp::Ordering {
    let a_idx = VALID_CRYPTO_KINDS.iter().position(|k| k == a);
    let b_idx = VALID_CRYPTO_KINDS.iter().position(|k| k == b);
    if let Some(a_idx) = a_idx {
        if let Some(b_idx) = b_idx {
            // Both are valid, prefer better crypto kind
            a_idx.cmp(&b_idx)
        } else {
            // A is valid, B is not
            cmp::Ordering::Less
        }
    } else if b_idx.is_some() {
        // B is valid, A is not
        cmp::Ordering::Greater
    } else {
        // Both are invalid, so use lex comparison
        a.cmp(b)
    }
}

/// Intersection of crypto kind vectors
#[must_use]
pub fn common_crypto_kinds(a: &[CryptoKind], b: &[CryptoKind]) -> Vec<CryptoKind> {
    let mut out = Vec::new();
    for ack in a {
        if b.contains(ack) {
            out.push(*ack);
        }
    }
    out
}

mod byte_array_types;
mod crypto_typed;
mod crypto_typed_group;
mod keypair;

pub use byte_array_types::*;
pub use crypto_typed::*;
pub use crypto_typed_group::*;
pub use keypair::*;

#[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), declare)]
pub type TypedPublicKey = CryptoTyped<PublicKey>;
#[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), declare)]
pub type TypedSecretKey = CryptoTyped<SecretKey>;
#[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), declare)]
pub type TypedKeyPair = CryptoTyped<KeyPair>;
#[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), declare)]
pub type TypedSignature = CryptoTyped<Signature>;
#[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), declare)]
pub type TypedSharedSecret = CryptoTyped<SharedSecret>;
#[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), declare)]
pub type TypedRouteId = CryptoTyped<RouteId>;
#[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), declare)]
pub type TypedRecordKey = CryptoTyped<RecordKey>;
#[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), declare)]
pub type TypedNodeId = CryptoTyped<NodeId>;
#[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), declare)]
pub type TypedHashDigest = CryptoTyped<HashDigest>;

#[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), declare)]
pub type TypedPublicKeyGroup = CryptoTypedGroup<PublicKey>;
#[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), declare)]
pub type TypedSecretKeyGroup = CryptoTypedGroup<SecretKey>;
#[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), declare)]
pub type TypedKeyPairGroup = CryptoTypedGroup<KeyPair>;
#[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), declare)]
pub type TypedSignatureGroup = CryptoTypedGroup<Signature>;
#[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), declare)]
pub type TypedSharedSecretGroup = CryptoTypedGroup<SharedSecret>;
#[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), declare)]
pub type TypedRouteIdGroup = CryptoTypedGroup<RouteId>;
#[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), declare)]
pub type TypedRecordKeyGroup = CryptoTypedGroup<RecordKey>;
#[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), declare)]
pub type TypedNodeIdGroup = CryptoTypedGroup<NodeId>;
#[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), declare)]
pub type TypedHashDigestGroup = CryptoTypedGroup<HashDigest>;

impl From<TypedPublicKey> for TypedHashDigest {
    fn from(value: TypedPublicKey) -> Self {
        TypedHashDigest::new(value.kind, value.value.into())
    }
}
impl From<TypedRecordKey> for TypedHashDigest {
    fn from(value: TypedRecordKey) -> Self {
        TypedHashDigest::new(value.kind, value.value.into())
    }
}
