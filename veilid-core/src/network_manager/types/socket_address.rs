use super::*;

#[derive(
    Copy, Default, Clone, Debug, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize,
)]
pub(crate) struct SocketAddress {
    address: Address,
    port: u16,
}

impl SocketAddress {
    pub fn new(address: Address, port: u16) -> Self {
        Self { address, port }
    }
    pub fn from_socket_addr(sa: SocketAddr) -> SocketAddress {
        Self {
            address: Address::from_socket_addr(sa),
            port: sa.port(),
        }
    }
    pub fn address(&self) -> Address {
        self.address
    }
    pub fn set_address(&mut self, address: Address) {
        self.address = address;
    }
    pub fn address_type(&self) -> AddressType {
        self.address.address_type()
    }
    pub fn port(&self) -> u16 {
        self.port
    }
    pub fn set_port(&mut self, port: u16) {
        self.port = port
    }
    pub fn with_port(&self, port: u16) -> Self {
        let mut sa = *self;
        sa.port = port;
        sa
    }
    pub fn canonical(&self) -> SocketAddress {
        SocketAddress {
            address: self.address.canonical(),
            port: self.port,
        }
    }
    pub fn ip_addr(&self) -> IpAddr {
        self.address.ip_addr()
    }
    pub fn socket_addr(&self) -> SocketAddr {
        self.address.socket_addr(self.port)
    }
}

impl fmt::Display for SocketAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.socket_addr())
    }
}

impl FromStr for SocketAddress {
    type Err = VeilidAPIError;
    fn from_str(s: &str) -> VeilidAPIResult<SocketAddress> {
        let sa = SocketAddr::from_str(s)
            .map_err(|e| VeilidAPIError::parse_error("Failed to parse SocketAddress", e))?;
        Ok(SocketAddress::from_socket_addr(sa))
    }
}
