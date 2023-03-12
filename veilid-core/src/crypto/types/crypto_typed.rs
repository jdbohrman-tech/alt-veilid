use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, RkyvArchive, RkyvSerialize, RkyvDeserialize)]
#[archive_attr(repr(C), derive(CheckBytes, Hash, PartialEq, Eq))]
pub struct CryptoTyped<K>
where
    K: Clone
        + Copy
        + fmt::Debug
        + fmt::Display
        + FromStr
        + PartialEq
        + Eq
        + Ord
        + PartialOrd
        + Hash
        + RkyvArchive
        + Encodable,
    <K as RkyvArchive>::Archived: Hash + PartialEq + Eq,
{
    pub kind: CryptoKind,
    pub value: K,
}

impl<K> CryptoTyped<K>
where
    K: Clone
        + Copy
        + fmt::Debug
        + fmt::Display
        + FromStr
        + PartialEq
        + Eq
        + Ord
        + PartialOrd
        + Hash
        + RkyvArchive
        + Encodable,
    <K as RkyvArchive>::Archived: Hash + PartialEq + Eq,
{
    pub fn new(kind: CryptoKind, value: K) -> Self {
        Self { kind, value }
    }
}
impl<K> PartialOrd for CryptoTyped<K>
where
    K: Clone
        + Copy
        + fmt::Debug
        + fmt::Display
        + FromStr
        + PartialEq
        + Eq
        + Ord
        + PartialOrd
        + Hash
        + RkyvArchive
        + Encodable,
    <K as RkyvArchive>::Archived: Hash + PartialEq + Eq,
{
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<K> Ord for CryptoTyped<K>
where
    K: Clone
        + Copy
        + fmt::Debug
        + fmt::Display
        + FromStr
        + PartialEq
        + Eq
        + Ord
        + PartialOrd
        + Hash
        + RkyvArchive
        + Encodable,
    <K as RkyvArchive>::Archived: Hash + PartialEq + Eq,
{
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        let x = compare_crypto_kind(&self.kind, &other.kind);
        if x != cmp::Ordering::Equal {
            return x;
        }
        self.value.cmp(&other.value)
    }
}

impl<K> fmt::Display for CryptoTyped<K>
where
    K: Clone
        + Copy
        + fmt::Debug
        + fmt::Display
        + FromStr
        + PartialEq
        + Eq
        + Ord
        + PartialOrd
        + Hash
        + RkyvArchive
        + Encodable,
    <K as RkyvArchive>::Archived: Hash + PartialEq + Eq,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}:{}", self.kind, self.value)
    }
}
impl<K> FromStr for CryptoTyped<K>
where
    K: Clone
        + Copy
        + fmt::Debug
        + fmt::Display
        + FromStr
        + PartialEq
        + Eq
        + Ord
        + PartialOrd
        + Hash
        + RkyvArchive
        + Encodable,
    <K as RkyvArchive>::Archived: Hash + PartialEq + Eq,
{
    type Err = VeilidAPIError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let b = s.as_bytes();
        if b.len() != (5 + K::encoded_len()) || b[4..5] != b":"[..] {
            apibail_parse_error!("invalid typed key", s);
        }
        let kind: CryptoKind = b[0..4].try_into().expect("should not fail to convert");
        let value = K::try_decode_bytes(&b[5..])?;
        Ok(Self { kind, value })
    }
}
impl<'de, K> Deserialize<'de> for CryptoTyped<K>
where
    K: Clone
        + Copy
        + fmt::Debug
        + fmt::Display
        + FromStr
        + PartialEq
        + Eq
        + Ord
        + PartialOrd
        + Hash
        + RkyvArchive
        + Encodable,
    <K as RkyvArchive>::Archived: Hash + PartialEq + Eq,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as Deserialize>::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(serde::de::Error::custom)
    }
}
impl<K> Serialize for CryptoTyped<K>
where
    K: Clone
        + Copy
        + fmt::Debug
        + fmt::Display
        + FromStr
        + PartialEq
        + Eq
        + Ord
        + PartialOrd
        + Hash
        + RkyvArchive
        + Encodable,
    <K as RkyvArchive>::Archived: Hash + PartialEq + Eq,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(self)
    }
}
