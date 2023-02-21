use super::*;

use core::cmp::{Eq, Ord, PartialEq, PartialOrd};
use core::convert::{TryFrom, TryInto};
use core::fmt;
use core::hash::Hash;

use data_encoding::BASE64URL_NOPAD;

use rkyv::{Archive as RkyvArchive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

//////////////////////////////////////////////////////////////////////

/// Length of a public key in bytes
#[allow(dead_code)]
pub const PUBLIC_KEY_LENGTH: usize = 32;
/// Length of a public key in bytes after encoding to base64url
#[allow(dead_code)]
pub const PUBLIC_KEY_LENGTH_ENCODED: usize = 43;
/// Length of a secret key in bytes
#[allow(dead_code)]
pub const SECRET_KEY_LENGTH: usize = 32;
/// Length of a secret key in bytes after encoding to base64url
#[allow(dead_code)]
pub const SECRET_KEY_LENGTH_ENCODED: usize = 43;
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
pub const SHARED_SECRET_LENGTH: usize = 32;
/// Length of a shared secret in bytes after encoding to base64url
#[allow(dead_code)]
pub const SHARED_SECRET_LENGTH_ENCODED: usize = 43;

//////////////////////////////////////////////////////////////////////

pub trait Encodable {
    fn encode(&self) -> String;
}

//////////////////////////////////////////////////////////////////////

macro_rules! byte_array_type {
    ($name:ident, $size:expr) => {
        #[derive(
            Clone,
            Copy,
            Hash,
            Eq,
            PartialEq,
            PartialOrd,
            Ord,
            RkyvArchive,
            RkyvSerialize,
            RkyvDeserialize,
        )]
        #[archive_attr(repr(C), derive(CheckBytes, Hash, Eq, PartialEq, PartialOrd, Ord))]
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

            pub fn try_from_vec(v: Vec<u8>) -> Result<Self, VeilidAPIError> {
                let vl = v.len();
                Ok(Self {
                    bytes: v.try_into().map_err(|_| {
                        VeilidAPIError::generic(format!(
                            "Expected a Vec of length {} but it was {}",
                            $size, vl
                        ))
                    })?,
                })
            }

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

            pub fn try_decode<S: AsRef<str>>(input: S) -> Result<Self, VeilidAPIError> {
                let b = input.as_ref().as_bytes();
                Self::try_decode_bytes(b)
            }
            pub fn try_decode_bytes(b: &[u8]) -> Result<Self, VeilidAPIError> {
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

        impl Encodable for $name {
            fn encode(&self) -> String {
                BASE64URL_NOPAD.encode(&self.bytes)
            }
        }
        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                //write!(f, "{}", String::from(self))
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
                // let mut s = String::new();
                // for n in 0..($size / 8) {
                //     let b: [u8; 8] = value.bytes[n * 8..(n + 1) * 8].try_into().unwrap();
                //     s.push_str(hex::encode(b).as_str());
                // }
                // s
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
                // let mut out = $name::default();
                // if value == "" {
                //     return Ok(out);
                // }
                // if value.len() != ($size * 2) {
                //     apibail_generic!(concat!(stringify!($name), " is incorrect length"));
                // }
                // match hex::decode_to_slice(value, &mut out.bytes) {
                //     Ok(_) => Ok(out),
                //     Err(err) => Err(VeilidAPIError::generic(err)),
                // }
                Self::try_decode(value)
            }
        }
    };
}

/////////////////////////////////////////

byte_array_type!(PublicKey, PUBLIC_KEY_LENGTH);
byte_array_type!(SecretKey, SECRET_KEY_LENGTH);
byte_array_type!(Signature, SIGNATURE_LENGTH);
byte_array_type!(PublicKeyDistance, PUBLIC_KEY_LENGTH);
byte_array_type!(Nonce, NONCE_LENGTH);
byte_array_type!(SharedSecret, SHARED_SECRET_LENGTH);
