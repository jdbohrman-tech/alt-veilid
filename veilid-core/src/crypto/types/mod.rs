use super::*;

use core::cmp::{Eq, Ord, PartialEq, PartialOrd};
use core::convert::TryInto;
use core::fmt;
use core::hash::Hash;

/// Cryptography version fourcc code
#[declare]
pub type CryptoKind = FourCC;

/// Sort best crypto kinds first
/// Better crypto kinds are 'less', ordered toward the front of a list
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

#[declare]
pub type TypedKey = CryptoTyped<PublicKey>;
#[declare]
pub type TypedSecret = CryptoTyped<SecretKey>;
#[declare]
pub type TypedKeyPair = CryptoTyped<KeyPair>;
#[declare]
pub type TypedSignature = CryptoTyped<Signature>;
#[declare]
pub type TypedSharedSecret = CryptoTyped<SharedSecret>;

#[declare]
pub type TypedKeyGroup = CryptoTypedGroup<PublicKey>;
#[declare]
pub type TypedSecretGroup = CryptoTypedGroup<SecretKey>;
#[declare]
pub type TypedKeyPairGroup = CryptoTypedGroup<KeyPair>;
#[declare]
pub type TypedSignatureGroup = CryptoTypedGroup<Signature>;
#[declare]
pub type TypedSharedSecretGroup = CryptoTypedGroup<SharedSecret>;
