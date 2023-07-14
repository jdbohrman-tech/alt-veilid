use super::*;

// Ordering here matters, IPV6 is preferred to IPV4 in dial info sorts
// See issue #236 for eventual resolution of this unfortunate implementation
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    PartialOrd,
    Ord,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive_attr(repr(u8), derive(CheckBytes))]
pub enum Address {
    IPV6(Ipv6Addr),
    IPV4(Ipv4Addr),
}

impl Default for Address {
    fn default() -> Self {
        Address::IPV4(Ipv4Addr::new(0, 0, 0, 0))
    }
}

impl Address {
    pub fn from_socket_addr(sa: SocketAddr) -> Address {
        match sa {
            SocketAddr::V4(v4) => Address::IPV4(*v4.ip()),
            SocketAddr::V6(v6) => Address::IPV6(*v6.ip()),
        }
    }
    pub fn from_ip_addr(addr: IpAddr) -> Address {
        match addr {
            IpAddr::V4(v4) => Address::IPV4(v4),
            IpAddr::V6(v6) => Address::IPV6(v6),
        }
    }
    pub fn address_type(&self) -> AddressType {
        match self {
            Address::IPV4(_) => AddressType::IPV4,
            Address::IPV6(_) => AddressType::IPV6,
        }
    }
    pub fn address_string(&self) -> String {
        match self {
            Address::IPV4(v4) => v4.to_string(),
            Address::IPV6(v6) => v6.to_string(),
        }
    }
    pub fn address_string_with_port(&self, port: u16) -> String {
        match self {
            Address::IPV4(v4) => format!("{}:{}", v4, port),
            Address::IPV6(v6) => format!("[{}]:{}", v6, port),
        }
    }
    pub fn is_unspecified(&self) -> bool {
        match self {
            Address::IPV4(v4) => ipv4addr_is_unspecified(v4),
            Address::IPV6(v6) => ipv6addr_is_unspecified(v6),
        }
    }
    pub fn is_global(&self) -> bool {
        match self {
            Address::IPV4(v4) => ipv4addr_is_global(v4) && !ipv4addr_is_multicast(v4),
            Address::IPV6(v6) => ipv6addr_is_unicast_global(v6),
        }
    }
    pub fn is_local(&self) -> bool {
        match self {
            Address::IPV4(v4) => {
                ipv4addr_is_private(v4)
                    || ipv4addr_is_link_local(v4)
                    || ipv4addr_is_ietf_protocol_assignment(v4)
            }
            Address::IPV6(v6) => {
                ipv6addr_is_unicast_site_local(v6)
                    || ipv6addr_is_unicast_link_local(v6)
                    || ipv6addr_is_unique_local(v6)
            }
        }
    }
    pub fn to_ip_addr(&self) -> IpAddr {
        match self {
            Self::IPV4(a) => IpAddr::V4(*a),
            Self::IPV6(a) => IpAddr::V6(*a),
        }
    }
    pub fn to_socket_addr(&self, port: u16) -> SocketAddr {
        SocketAddr::new(self.to_ip_addr(), port)
    }
    pub fn to_canonical(&self) -> Address {
        match self {
            Address::IPV4(v4) => Address::IPV4(*v4),
            Address::IPV6(v6) => match v6.to_ipv4() {
                Some(v4) => Address::IPV4(v4),
                None => Address::IPV6(*v6),
            },
        }
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Address::IPV4(v4) => write!(f, "{}", v4),
            Address::IPV6(v6) => write!(f, "{}", v6),
        }
    }
}

impl FromStr for Address {
    type Err = VeilidAPIError;
    fn from_str(host: &str) -> VeilidAPIResult<Address> {
        if let Ok(addr) = Ipv4Addr::from_str(host) {
            Ok(Address::IPV4(addr))
        } else if let Ok(addr) = Ipv6Addr::from_str(host) {
            Ok(Address::IPV6(addr))
        } else {
            Err(VeilidAPIError::parse_error(
                "Address::from_str failed",
                host,
            ))
        }
    }
}
