use super::*;

#[derive(Debug, Default)]
pub struct GeolocationInfo {
    country_code: Option<CountryCode>,
    relay_country_codes: Vec<Option<CountryCode>>,
}

impl GeolocationInfo {
    pub fn new(
        country_code: Option<CountryCode>,
        relay_country_codes: Vec<Option<CountryCode>>,
    ) -> Self {
        GeolocationInfo {
            country_code,
            relay_country_codes,
        }
    }

    /// Get node country code. Might be `None` if unable to determine.
    pub fn country_code(&self) -> Option<CountryCode> {
        self.country_code
    }

    /// Get country codes of relays used by the node.
    /// There will be exactly one entry for each relay.
    /// Empty if no relays are used.
    pub fn relay_country_codes(&self) -> &[Option<CountryCode>] {
        &self.relay_country_codes
    }
}
