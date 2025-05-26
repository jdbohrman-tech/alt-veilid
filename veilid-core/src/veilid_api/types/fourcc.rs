#[macro_export]
macro_rules! fourcc_type {
    ($name:ident) => {
        /// A four-character code
        #[derive(
            Copy,
            Default,
            Clone,
            Hash,
            PartialOrd,
            Ord,
            PartialEq,
            Eq,
            Serialize,
            Deserialize,
            JsonSchema,
        )]
        #[serde(try_from = "String")]
        #[serde(into = "String")]
        #[must_use]
        pub struct $name(pub [u8; 4]);

        cfg_if::cfg_if! {
            if #[cfg(all(target_arch = "wasm32", target_os = "unknown"))] {
                #[wasm_bindgen(typescript_custom_section)]
                const FOURCC_TYPE: &'static str = concat!(r#"
export type "#, stringify!($name), r#" = string;
"#);
            }
        }

        impl From<[u8; 4]> for $name {
            fn from(b: [u8; 4]) -> Self {
                Self(b)
            }
        }

        impl From<u32> for $name {
            fn from(u: u32) -> Self {
                Self(u.to_be_bytes())
            }
        }

        impl From<$name> for u32 {
            fn from(u: $name) -> Self {
                u32::from_be_bytes(u.0)
            }
        }

        impl From<$name> for String {
            fn from(u: $name) -> Self {
                String::from_utf8_lossy(&u.0).to_string()
            }
        }

        impl TryFrom<&[u8]> for $name {
            type Error = VeilidAPIError;
            fn try_from(b: &[u8]) -> Result<Self, Self::Error> {
                Ok(Self(b.try_into().map_err(VeilidAPIError::generic)?))
            }
        }

        impl TryFrom<String> for $name {
            type Error = VeilidAPIError;
            fn try_from(s: String) -> Result<Self, Self::Error> {
                Self::from_str(s.as_str())
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
                write!(f, "{}", String::from_utf8_lossy(&self.0))
            }
        }
        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
                write!(f, "{}", String::from_utf8_lossy(&self.0))
            }
        }

        impl FromStr for $name {
            type Err = VeilidAPIError;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(Self(
                    s.as_bytes().try_into().map_err(VeilidAPIError::generic)?,
                ))
            }
        }
    };
}
