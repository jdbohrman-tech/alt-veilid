use super::*;

use core::cmp::{Eq, Ord, PartialEq, PartialOrd};
use core::convert::TryInto;
use core::fmt;
use core::hash::Hash;

use rkyv::{Archive as RkyvArchive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

/// Cryptography version fourcc code
pub type CryptoKind = FourCC;

/// Sort best crypto kinds first
pub fn compare_crypto_kind(a: CryptoKind, b: CryptoKind) -> cmp::Ordering {
    let a_idx = VALID_CRYPTO_KINDS.iter().position(|&k| k == a);
    let b_idx = VALID_CRYPTO_KINDS.iter().position(|&k| k == b);
    if let Some(a_idx) = a_idx {
        if let Some(b_idx) = b_idx {
            a_idx.cmp(&b_idx)
        } else {
            cmp::Ordering::Less
        }
    } else if let Some(b_idx) = b_idx {
        cmp::Ordering::Greater
    } else {
        a.cmp(&b)
    }
}

#[derive(
    Clone,
    Copy,
    Debug,
    Serialize,
    Deserialize,
    PartialOrd,
    Ord,
    PartialEq,
    Eq,
    Hash,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct KeyPair {
    pub key: PublicKey,
    pub secret: SecretKey,
}

impl KeyPair {
    pub fn new(key: PublicKey, secret: SecretKey) -> Self {
        Self { key, secret }
    }
}

#[derive(
    Clone,
    Copy,
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Hash,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct TypedKey {
    pub kind: CryptoKind,
    pub key: PublicKey,
}

impl TypedKey {
    pub fn new(kind: CryptoKind, key: PublicKey) -> Self {
        Self { kind, key }
    }
}
impl PartialOrd for TypedKey {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TypedKey {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        let x = compare_crypto_kind(self.kind, other.kind);
        if x != cmp::Ordering::Equal {
            return x;
        }
        self.key.cmp(&other.key)
    }
}

impl fmt::Display for TypedKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}:{}", self.kind, self.key.encode())
    }
}
impl FromStr for TypedKey {
    type Err = VeilidAPIError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let b = s.as_bytes();
        if b.len() != (5 + PUBLIC_KEY_LENGTH_ENCODED) || b[4..5] != b":"[..] {
            apibail_parse_error!("invalid typed key", s);
        }
        let kind: CryptoKind = b[0..4].try_into().expect("should not fail to convert");
        let key = PublicKey::try_decode_bytes(&b[5..])?;
        Ok(Self { kind, key })
    }
}

#[derive(
    Clone,
    Debug,
    Serialize,
    Deserialize,
    PartialOrd,
    Ord,
    PartialEq,
    Eq,
    Hash,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct TypedKeySet {
    items: Vec<TypedKey>,
}

impl TypedKeySet {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            items: Vec::with_capacity(cap),
        }
    }
    pub fn get(&self, kind: CryptoKind) -> Option<TypedKey> {
        self.items.iter().find(|x| x.kind == kind).copied()
    }
    pub fn add(&mut self, typed_key: TypedKey) {
        for x in &mut self.items {
            if x.kind == typed_key.kind {
                *x = typed_key;
                return;
            }
        }
        self.items.push(typed_key);
        self.items.sort()
    }
    pub fn remove(&self, kind: CryptoKind) {
        if let Some(idx) = self.items.iter().position(|x| x.kind == kind) {
            self.items.remove(idx);
        }
    }
    pub fn best(&self) -> Option<TypedKey> {
        self.items.first().copied()
    }
    pub fn len(&self) -> usize {
        self.items.len()
    }
    pub fn iter(&self) -> core::slice::Iter<'_, TypedKey> {
        self.items.iter()
    }
    pub fn contains(&self, typed_key: &TypedKey) -> bool {
        self.items.contains(typed_key)
    }
    pub fn contains_any(&self, typed_keys: &[TypedKey]) -> bool {
        for typed_key in typed_keys {
            if self.items.contains(typed_key) {
                return true;
            }
        }
        false
    }
}

impl core::ops::Deref for TypedKeySet {
    type Target = [TypedKey];

    #[inline]
    fn deref(&self) -> &[TypedKey] {
        &self.items
    }
}

impl fmt::Display for TypedKeySet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "[")?;
        let mut first = true;
        for x in &self.items {
            if !first {
                write!(f, ",")?;
                first = false;
            }
            write!(f, "{}", x)?;
        }
        write!(f, "]")
    }
}
impl FromStr for TypedKeySet {
    type Err = VeilidAPIError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut items = Vec::new();
        if s.len() < 2 {
            apibail_parse_error!("invalid length", s);
        }
        if &s[0..1] != "[" || &s[(s.len() - 1)..] != "]" {
            apibail_parse_error!("invalid format", s);
        }
        for x in s[1..s.len() - 1].split(",") {
            let tk = TypedKey::from_str(x.trim())?;
            items.push(tk);
        }

        Ok(Self { items })
    }
}

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct TypedKeyPair {
    pub kind: CryptoKind,
    pub key: PublicKey,
    pub secret: SecretKey,
}

impl TypedKeyPair {
    pub fn new(kind: CryptoKind, key: PublicKey, secret: SecretKey) -> Self {
        Self { kind, key, secret }
    }
}

impl PartialOrd for TypedKeyPair {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TypedKeyPair {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        let x = compare_crypto_kind(self.kind, other.kind);
        if x != cmp::Ordering::Equal {
            return x;
        }
        let x = self.key.cmp(&other.key);
        if x != cmp::Ordering::Equal {
            return x;
        }
        self.secret.cmp(&other.secret)
    }
}

impl fmt::Display for TypedKeyPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}:{}:{}",
            self.kind,
            self.key.encode(),
            self.secret.encode()
        )
    }
}
impl FromStr for TypedKeyPair {
    type Err = VeilidAPIError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let b = s.as_bytes();
        if b.len() != (5 + PUBLIC_KEY_LENGTH_ENCODED + 1 + SECRET_KEY_LENGTH_ENCODED)
            || b[4..5] != b":"[..]
            || b[5 + PUBLIC_KEY_LENGTH_ENCODED..6 + PUBLIC_KEY_LENGTH_ENCODED] != b":"[..]
        {
            apibail_parse_error!("invalid typed key pair", s);
        }
        let kind: CryptoKind = b[0..4].try_into().expect("should not fail to convert");
        let key = PublicKey::try_decode_bytes(&b[5..5 + PUBLIC_KEY_LENGTH_ENCODED])?;
        let secret = SecretKey::try_decode_bytes(&b[5 + PUBLIC_KEY_LENGTH_ENCODED + 1..])?;
        Ok(Self { kind, key, secret })
    }
}

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct TypedSignature {
    pub kind: CryptoKind,
    pub signature: Signature,
}
impl TypedSignature {
    pub fn new(kind: CryptoKind, signature: Signature) -> Self {
        Self { kind, signature }
    }
    pub fn from_keyed(tks: &TypedKeySignature) -> Self {
        Self {
            kind: tks.kind,
            signature: tks.signature,
        }
    }
    pub fn from_pair_sig(tkp: &TypedKeyPair, sig: Signature) -> Self {
        Self {
            kind: tkp.kind,
            signature: sig,
        }
    }
}

impl PartialOrd for TypedSignature {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TypedSignature {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        let x = compare_crypto_kind(self.kind, other.kind);
        if x != cmp::Ordering::Equal {
            return x;
        }
        self.signature.cmp(&other.signature)
    }
}

impl fmt::Display for TypedSignature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}:{}", self.kind, self.signature.encode())
    }
}
impl FromStr for TypedSignature {
    type Err = VeilidAPIError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let b = s.as_bytes();
        if b.len() != (5 + SIGNATURE_LENGTH_ENCODED) || b[4..5] != b":"[..] {
            apibail_parse_error!("invalid typed signature", s);
        }
        let kind: CryptoKind = b[0..4].try_into()?;
        let signature = Signature::try_decode_bytes(&b[5..])?;
        Ok(Self { kind, signature })
    }
}

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct TypedKeySignature {
    pub kind: CryptoKind,
    pub key: PublicKey,
    pub signature: Signature,
}

impl TypedKeySignature {
    pub fn new(kind: CryptoKind, key: PublicKey, signature: Signature) -> Self {
        Self {
            kind,
            key,
            signature,
        }
    }
    pub fn as_typed_signature(&self) -> TypedSignature {
        TypedSignature {
            kind: self.kind,
            signature: self.signature,
        }
    }
}

impl PartialOrd for TypedKeySignature {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TypedKeySignature {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        let x = compare_crypto_kind(self.kind, other.kind);
        if x != cmp::Ordering::Equal {
            return x;
        }
        let x = self.key.cmp(&other.key);
        if x != cmp::Ordering::Equal {
            return x;
        }
        self.signature.cmp(&other.signature)
    }
}

impl fmt::Display for TypedKeySignature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}:{}:{}",
            self.kind,
            self.key.encode(),
            self.signature.encode()
        )
    }
}
impl FromStr for TypedKeySignature {
    type Err = VeilidAPIError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let b = s.as_bytes();
        if b.len() != (5 + PUBLIC_KEY_LENGTH_ENCODED + 1 + SIGNATURE_LENGTH_ENCODED)
            || b[4] != b':'
            || b[5 + PUBLIC_KEY_LENGTH_ENCODED] != b':'
        {
            apibail_parse_error!("invalid typed key signature", s);
        }
        let kind: CryptoKind = b[0..4].try_into().expect("should not fail to convert");
        let key = PublicKey::try_decode_bytes(&b[5..5 + PUBLIC_KEY_LENGTH_ENCODED])?;
        let signature = Signature::try_decode_bytes(&b[5 + PUBLIC_KEY_LENGTH_ENCODED + 1..])?;
        Ok(Self {
            kind,
            key,
            signature,
        })
    }
}
