use super::*;

/// FOURCC code
#[derive(
    Copy, Default, Clone, Hash, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize, JsonSchema,
)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
#[serde(try_from = "String")]
#[serde(into = "String")]
pub struct FourCC(pub [u8; 4]);

impl From<[u8; 4]> for FourCC {
    fn from(b: [u8; 4]) -> Self {
        Self(b)
    }
}

impl From<u32> for FourCC {
    fn from(u: u32) -> Self {
        Self(u.to_be_bytes())
    }
}

impl From<FourCC> for u32 {
    fn from(u: FourCC) -> Self {
        u32::from_be_bytes(u.0)
    }
}

impl From<FourCC> for String {
    fn from(u: FourCC) -> Self {
        String::from_utf8_lossy(&u.0).to_string()
    }
}

impl TryFrom<&[u8]> for FourCC {
    type Error = VeilidAPIError;
    fn try_from(b: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self(b.try_into().map_err(VeilidAPIError::generic)?))
    }
}

impl TryFrom<String> for FourCC {
    type Error = VeilidAPIError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::from_str(s.as_str())
    }
}

impl fmt::Display for FourCC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", String::from_utf8_lossy(&self.0))
    }
}
impl fmt::Debug for FourCC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", String::from_utf8_lossy(&self.0))
    }
}

impl FromStr for FourCC {
    type Err = VeilidAPIError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.as_bytes().try_into().map_err(VeilidAPIError::generic)?,
        ))
    }
}
