use super::*;

use core::cmp::{Eq, Ord, PartialEq, PartialOrd};
use core::convert::{TryFrom, TryInto};
use core::fmt;
use core::hash::Hash;

use data_encoding::BASE64URL_NOPAD;

//////////////////////////////////////////////////////////////////////

/// Length of a crypto key in bytes
#[allow(dead_code)]
pub const CRYPTO_KEY_LENGTH: usize = 32;
/// Length of a crypto key in bytes after encoding to base64url
#[allow(dead_code)]
pub const CRYPTO_KEY_LENGTH_ENCODED: usize = 43;
/// Length of a crypto key in bytes
#[allow(dead_code)]
pub const PUBLIC_KEY_LENGTH: usize = CRYPTO_KEY_LENGTH;
/// Length of a crypto key in bytes after encoding to base64url
#[allow(dead_code)]
pub const PUBLIC_KEY_LENGTH_ENCODED: usize = CRYPTO_KEY_LENGTH_ENCODED;
/// Length of a secret key in bytes
#[allow(dead_code)]
pub const SECRET_KEY_LENGTH: usize = CRYPTO_KEY_LENGTH;
/// Length of a secret key in bytes after encoding to base64url
#[allow(dead_code)]
pub const SECRET_KEY_LENGTH_ENCODED: usize = CRYPTO_KEY_LENGTH_ENCODED;
/// Length of a signature in bytes
#[allow(dead_code)]
pub const SIGNATURE_LENGTH: usize = 64;
/// Length of a signature in bytes after encoding to base64url
#[allow(dead_code)]
pub const SIGNATURE_LENGTH_ENCODED: usize = 86;
/// Length of a nonce in bytes
#[allow(dead_code)]
pub const NONCE_LENGTH: usize = 24;
/// Length of a nonce in bytes after encoding to base64url
#[allow(dead_code)]
pub const NONCE_LENGTH_ENCODED: usize = 32;
/// Length of a shared secret in bytes
#[allow(dead_code)]
pub const SHARED_SECRET_LENGTH: usize = CRYPTO_KEY_LENGTH;
/// Length of a shared secret in bytes after encoding to base64url
#[allow(dead_code)]
pub const SHARED_SECRET_LENGTH_ENCODED: usize = CRYPTO_KEY_LENGTH_ENCODED;
/// Length of a route id in bytes
#[allow(dead_code)]
pub const ROUTE_ID_LENGTH: usize = CRYPTO_KEY_LENGTH;
/// Length of a route id in bytes after encoding to base64url
#[allow(dead_code)]
pub const ROUTE_ID_LENGTH_ENCODED: usize = CRYPTO_KEY_LENGTH_ENCODED;
/// Length of a hash digest in bytes
#[allow(dead_code)]
pub const HASH_DIGEST_LENGTH: usize = CRYPTO_KEY_LENGTH;
/// Length of a hash digest in bytes after encoding to base64url
#[allow(dead_code)]
pub const HASH_DIGEST_LENGTH_ENCODED: usize = CRYPTO_KEY_LENGTH_ENCODED;

//////////////////////////////////////////////////////////////////////

pub trait Encodable
where
    Self: Sized,
{
    fn encode(&self) -> String;
    fn encoded_len() -> usize;
    fn try_decode<S: AsRef<str>>(input: S) -> VeilidAPIResult<Self> {
        let b = input.as_ref().as_bytes();
        Self::try_decode_bytes(b)
    }
    fn try_decode_bytes(b: &[u8]) -> VeilidAPIResult<Self>;
}

//////////////////////////////////////////////////////////////////////

macro_rules! byte_array_type {
    ($name:ident, $size:expr, $encoded_size:expr) => {
        #[derive(Clone, Copy, Hash, PartialOrd, Ord, PartialEq, Eq)]
        #[cfg_attr(target_arch = "wasm32", derive(Tsify), tsify(into_wasm_abi))]
        pub struct $name {
            pub bytes: [u8; $size],
        }

        impl Default for $name {
            fn default() -> Self {
                Self {
                    bytes: [0u8; $size],
                }
            }
        }

        impl serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let s = self.encode();
                serde::Serialize::serialize(&s, serializer)
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let s = <String as serde::Deserialize>::deserialize(deserializer)?;
                if s == "" {
                    return Ok($name::default());
                }
                $name::try_decode(s.as_str()).map_err(serde::de::Error::custom)
            }
        }

        impl $name {
            pub fn new(bytes: [u8; $size]) -> Self {
                Self { bytes }
            }

            // Big endian bit ordering
            pub fn bit(&self, index: usize) -> bool {
                assert!(index < ($size * 8));
                let bi = index / 8;
                let ti = 7 - (index % 8);
                ((self.bytes[bi] >> ti) & 1) != 0
            }

            pub fn first_nonzero_bit(&self) -> Option<usize> {
                for i in 0..$size {
                    let b = self.bytes[i];
                    if b != 0 {
                        for n in 0..8 {
                            if ((b >> (7 - n)) & 1u8) != 0u8 {
                                return Some((i * 8) + n);
                            }
                        }
                        panic!("wtf")
                    }
                }
                None
            }

            // Big endian nibble ordering
            pub fn nibble(&self, index: usize) -> u8 {
                assert!(index < ($size * 2));
                let bi = index / 2;
                if index & 1 == 0 {
                    (self.bytes[bi] >> 4) & 0xFu8
                } else {
                    self.bytes[bi] & 0xFu8
                }
            }

            pub fn first_nonzero_nibble(&self) -> Option<(usize, u8)> {
                for i in 0..($size * 2) {
                    let n = self.nibble(i);
                    if n != 0 {
                        return Some((i, n));
                    }
                }
                None
            }
        }

        impl Encodable for $name {
            fn encode(&self) -> String {
                BASE64URL_NOPAD.encode(&self.bytes)
            }
            fn encoded_len() -> usize {
                $encoded_size
            }
            fn try_decode_bytes(b: &[u8]) -> VeilidAPIResult<Self> {
                let mut bytes = [0u8; $size];
                let res = BASE64URL_NOPAD.decode_len(b.len());
                match res {
                    Ok(v) => {
                        if v != $size {
                            apibail_generic!("Incorrect length in decode");
                        }
                    }
                    Err(_) => {
                        apibail_generic!("Failed to decode");
                    }
                }

                let res = BASE64URL_NOPAD.decode_mut(b, &mut bytes);
                match res {
                    Ok(_) => Ok(Self::new(bytes)),
                    Err(_) => apibail_generic!("Failed to decode"),
                }
            }
        }
        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.encode())
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, concat!(stringify!($name), "("))?;
                write!(f, "{}", self.encode())?;
                write!(f, ")")
            }
        }

        impl From<&$name> for String {
            fn from(value: &$name) -> Self {
                value.encode()
            }
        }

        impl FromStr for $name {
            type Err = VeilidAPIError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                $name::try_from(s)
            }
        }

        impl TryFrom<String> for $name {
            type Error = VeilidAPIError;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                $name::try_from(value.as_str())
            }
        }

        impl TryFrom<&str> for $name {
            type Error = VeilidAPIError;
            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Self::try_decode(value)
            }
        }
        impl TryFrom<&[u8]> for $name {
            type Error = VeilidAPIError;
            fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
                let vl = v.len();
                Ok(Self {
                    bytes: v.try_into().map_err(|_| {
                        VeilidAPIError::generic(format!(
                            "Expected a slice of length {} but it was {}",
                            $size, vl
                        ))
                    })?,
                })
            }
        }

        impl core::ops::Deref for $name {
            type Target = [u8; $size];

            fn deref(&self) -> &Self::Target {
                &self.bytes
            }
        }

        impl core::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.bytes
            }
        }

        impl From<[u8; $size]> for $name {
            fn from(value: [u8; $size]) -> Self {
                Self::new(value)
            }
        }
        
        impl From<$name> for [u8; $size] {
            fn from(value: $name) -> Self {
                value.bytes
            }
        }
    };
}

/////////////////////////////////////////

byte_array_type!(CryptoKey, CRYPTO_KEY_LENGTH, CRYPTO_KEY_LENGTH_ENCODED);

#[cfg_attr(target_arch = "wasm32", declare)]
pub type PublicKey = CryptoKey;
#[cfg_attr(target_arch = "wasm32", declare)]
pub type SecretKey = CryptoKey;
#[cfg_attr(target_arch = "wasm32", declare)]
pub type HashDigest = CryptoKey;
#[cfg_attr(target_arch = "wasm32", declare)]
pub type SharedSecret = CryptoKey;
#[cfg_attr(target_arch = "wasm32", declare)]
pub type RouteId = CryptoKey;
#[cfg_attr(target_arch = "wasm32", declare)]
pub type CryptoKeyDistance = CryptoKey;

byte_array_type!(Signature, SIGNATURE_LENGTH, SIGNATURE_LENGTH_ENCODED);
byte_array_type!(Nonce, NONCE_LENGTH, NONCE_LENGTH_ENCODED);


// impl From<[u8; SIGNATURE_LENGTH]> for Signature {
//     fn from(value: [u8; SIGNATURE_LENGTH]) -> Self {
//         Self::new(value)
//     }
// }

// impl From<Signature> for [u8; SIGNATURE_LENGTH] {
//     fn from(value: Signature) -> Self {
//         value.bytes
//     }
// }
