use super::*;
use std::hash::{Hash, Hasher};

/// Two-letter country code. Case-insensitive when comparing.
#[derive(Copy, Default, Clone, Ord, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(try_from = "String")]
#[serde(into = "String")]
pub struct CountryCode(pub [u8; 2]);

impl From<[u8; 2]> for CountryCode {
    fn from(b: [u8; 2]) -> Self {
        Self(b)
    }
}

impl From<CountryCode> for String {
    fn from(u: CountryCode) -> Self {
        String::from_utf8_lossy(&u.0).to_string()
    }
}

impl TryFrom<&[u8]> for CountryCode {
    type Error = VeilidAPIError;
    fn try_from(b: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self(b.try_into().map_err(VeilidAPIError::generic)?))
    }
}

impl TryFrom<String> for CountryCode {
    type Error = VeilidAPIError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::from_str(s.as_str())
    }
}

impl fmt::Display for CountryCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", String::from_utf8_lossy(&self.0))
    }
}

impl fmt::Debug for CountryCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", String::from_utf8_lossy(&self.0))
    }
}

impl FromStr for CountryCode {
    type Err = VeilidAPIError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.as_bytes().try_into().map_err(VeilidAPIError::generic)?,
        ))
    }
}

impl Hash for CountryCode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let this = [
            self.0[0].to_ascii_uppercase(),
            self.0[1].to_ascii_uppercase(),
        ];

        state.write(&this[..]);
    }
}

impl PartialEq for CountryCode {
    fn eq(&self, other: &Self) -> bool {
        self.0[0].to_ascii_uppercase() == other.0[0].to_ascii_uppercase()
            && self.0[1].to_ascii_uppercase() == other.0[1].to_ascii_uppercase()
    }
}

impl PartialOrd for CountryCode {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        let this = [
            self.0[0].to_ascii_uppercase(),
            self.0[1].to_ascii_uppercase(),
        ];
        let other = [
            other.0[0].to_ascii_uppercase(),
            other.0[1].to_ascii_uppercase(),
        ];

        this.partial_cmp(&other)
    }
}

#[cfg(test)]
mod tests {
    use crate::CountryCode;
    use core::str::FromStr;
    use std::collections::HashSet;

    #[test]
    fn test_hash_country_code() {
        let mut set = HashSet::new();

        set.insert(CountryCode::from_str("aa").unwrap());

        assert!(set.get(&CountryCode::from_str("AA").unwrap()).is_some());
    }

    #[test]
    fn test_compare_country_code() {
        assert_eq!(
            CountryCode::from_str("aa").unwrap(),
            CountryCode::from_str("AA").unwrap(),
        );

        assert!(CountryCode::from_str("aa").unwrap() < CountryCode::from_str("Ab").unwrap());

        assert!(CountryCode::from_str("Bc").unwrap() > CountryCode::from_str("bb").unwrap());
    }
}
