use super::*;

/// Two-letter country code. Stored in upper case internally.
#[derive(
    Copy, Default, Clone, Hash, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize, JsonSchema,
)]
#[serde(try_from = "String")]
#[serde(into = "String")]
pub struct CountryCode([u8; 2]);

impl From<CountryCode> for String {
    fn from(u: CountryCode) -> Self {
        String::from_utf8_lossy(&u.0).to_string()
    }
}

impl TryFrom<&[u8]> for CountryCode {
    type Error = VeilidAPIError;
    fn try_from(b: &[u8]) -> Result<Self, Self::Error> {
        let cc: [u8; 2] = b.try_into().map_err(VeilidAPIError::generic)?;

        if !cc[0].is_ascii_alphabetic() || !cc[1].is_ascii_alphabetic() {
            return Err(VeilidAPIError::generic(
                "country code must only contain alphabetic chars",
            ));
        }

        Ok(Self([
            cc[0].to_ascii_uppercase(),
            cc[1].to_ascii_uppercase(),
        ]))
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
        // self.0 is guaranteed to be a valid ASCII string, checked in Self::try_from(&[u8])
        write!(f, "{}{}", self.0[0] as char, self.0[1] as char)
    }
}

impl fmt::Debug for CountryCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        <Self as fmt::Display>::fmt(self, f)
    }
}

impl FromStr for CountryCode {
    type Err = VeilidAPIError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::try_from(s.as_bytes())?)
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
