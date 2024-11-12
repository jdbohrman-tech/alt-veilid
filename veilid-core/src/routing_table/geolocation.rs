use crate::CountryCode;
use maxminddb::MaxMindDBError;
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::net::IpAddr;
use tracing::error;

const IPV4_MMDB: &[u8] =
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../target/ipv4.mmdb"));
const IPV6_MMDB: &[u8] =
    include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../target/ipv6.mmdb"));

static IPV4: Lazy<Option<maxminddb::Reader<&'static [u8]>>> =
    Lazy::new(|| match maxminddb::Reader::from_source(IPV4_MMDB) {
        Ok(reader) => Some(reader),
        Err(err) => {
            error!("Unable to open embedded IPv4 geolocation database: {}", err);
            None
        }
    });

static IPV6: Lazy<Option<maxminddb::Reader<&'static [u8]>>> =
    Lazy::new(|| match maxminddb::Reader::from_source(IPV6_MMDB) {
        Ok(reader) => Some(reader),
        Err(err) => {
            error!("Unable to open embedded IPv6 geolocation database: {}", err);
            None
        }
    });

#[derive(Deserialize)]
struct Country {
    country_code: CountryCode,
}

pub fn query_country_code(addr: IpAddr) -> Option<CountryCode> {
    let db = match addr {
        IpAddr::V4(_) => &*IPV4,
        IpAddr::V6(_) => &*IPV6,
    };

    let Some(db) = db else {
        return None;
    };

    let result: Country = match db.lookup(addr) {
        Ok(result) => result,
        Err(MaxMindDBError::AddressNotFoundError(_)) => return None,
        Err(err) => {
            // We only expect AddressNotFoundError as possible error,
            // anything else means there's a problem
            error!("Unable to query country code: {}", err);
            return None;
        }
    };

    Some(result.country_code)
}

#[cfg(test)]
mod tests {
    use crate::CountryCode;
    use core::str::FromStr;
    use maxminddb::WithinItem;

    #[test]
    fn test_query_country_code() {
        let test_cases = [
            ("1.2.3.4", "AU"),
            ("18.103.1.1", "US"),
            ("100.128.1.1", "US"),
            ("198.3.123.4", "US"),
            ("2001:2a0::1", "JP"),
        ];

        for (ip_str, expected_country) in test_cases {
            let ip = ip_str.parse().unwrap();
            let expected_country_code = CountryCode::from_str(expected_country).unwrap();

            let country_code = super::query_country_code(ip).unwrap();
            assert_eq!(
                country_code, expected_country_code,
                "Wrong country for {ip_str}",
            );

            eprintln!("{ip_str} -> {country_code}");
        }

        assert!(super::query_country_code("127.0.0.1".parse().unwrap()).is_none());
        assert!(super::query_country_code("10.0.0.1".parse().unwrap()).is_none());
        assert!(super::query_country_code("::1".parse().unwrap()).is_none());
    }

    #[test]
    fn test_iter_over_ipv4_mmdb() {
        let db = super::IPV4.as_ref().unwrap();

        let count = db
            .within("0.0.0.0/0".parse().unwrap())
            .unwrap()
            .map(|item: Result<WithinItem<super::Country>, _>| item.unwrap())
            .count();

        assert!(count > 100, "Expecting some IPv4 subnets in IPv4 MMDB");
    }

    #[test]
    fn test_iter_over_ipv6_mmdb() {
        let db = super::IPV6.as_ref().unwrap();

        let count = db
            .within("::/0".parse().unwrap())
            .unwrap()
            .map(|item: Result<WithinItem<super::Country>, _>| item.unwrap())
            .count();

        assert!(count > 100, "Expecting some IPv6 subnets in IPv6 MMDB");
    }
}
