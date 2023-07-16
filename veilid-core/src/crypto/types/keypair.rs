use super::*;

#[derive(Clone, Copy, Default, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct KeyPair {
    pub key: PublicKey,
    pub secret: SecretKey,
}

impl KeyPair {
    pub fn new(key: PublicKey, secret: SecretKey) -> Self {
        Self { key, secret }
    }
    pub fn split(&self) -> (PublicKey, SecretKey) {
        (self.key, self.secret)
    }
    pub fn into_split(self) -> (PublicKey, SecretKey) {
        (self.key, self.secret)
    }
}

impl Encodable for KeyPair {
    fn encode(&self) -> String {
        format!("{}:{}", self.key.encode(), self.secret.encode())
    }
    fn encoded_len() -> usize {
        PublicKey::encoded_len() + 1 + SecretKey::encoded_len()
    }
    fn try_decode_bytes(b: &[u8]) -> VeilidAPIResult<Self> {
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
        write!(f, "KeyPair({})", self.encode())
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

impl serde::Serialize for KeyPair {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = self.encode();
        serde::Serialize::serialize(&s, serializer)
    }
}

impl<'de> serde::Deserialize<'de> for KeyPair {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        if s == "" {
            return Ok(KeyPair::default());
        }
        KeyPair::try_decode(s.as_str()).map_err(serde::de::Error::custom)
    }
}
