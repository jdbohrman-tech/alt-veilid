use super::*;

#[derive(
    Clone,
    Copy,
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
#[archive_attr(repr(C), derive(CheckBytes, Hash, PartialEq, Eq))]
pub struct KeyPair {
    pub key: PublicKey,
    pub secret: SecretKey,
}

impl KeyPair {
    pub fn new(key: PublicKey, secret: SecretKey) -> Self {
        Self { key, secret }
    }
}

impl Encodable for KeyPair {
    fn encode(&self) -> String {
        format!("{}:{}", self.key.encode(), self.secret.encode())
    }
    fn encoded_len() -> usize {
        PublicKey::encoded_len() + 1 + SecretKey::encoded_len()
    }
    fn try_decode_bytes(b: &[u8]) -> Result<Self, VeilidAPIError> {
        if b.len() != Self::encoded_len() {
            apibail_parse_error!("input has wrong encoded length", format!("len={}", b.len()));
        }
        let key = PublicKey::try_decode_bytes(&b[0..PublicKey::encoded_len()])?;
        let secret = SecretKey::try_decode_bytes(&b[(PublicKey::encoded_len() + 1)..])?;
        Ok(KeyPair { key, secret })
    }
}
impl fmt::Display for KeyPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.encode())
    }
}

impl fmt::Debug for KeyPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, concat!(stringify!($name), "("))?;
        write!(f, "{}", self.encode())?;
        write!(f, ")")
    }
}

impl From<&KeyPair> for String {
    fn from(value: &KeyPair) -> Self {
        value.encode()
    }
}

impl FromStr for KeyPair {
    type Err = VeilidAPIError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        KeyPair::try_from(s)
    }
}

impl TryFrom<String> for KeyPair {
    type Error = VeilidAPIError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        KeyPair::try_from(value.as_str())
    }
}

impl TryFrom<&str> for KeyPair {
    type Error = VeilidAPIError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_decode(value)
    }
}
