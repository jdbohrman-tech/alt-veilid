mod tcp;
mod udp;
mod ws;
mod wss;

use super::*;

pub(crate) use tcp::*;
pub(crate) use udp::*;
pub(crate) use ws::*;
pub(crate) use wss::*;

// Keep member order appropriate for sorting < preference
// Must match ProtocolType order
#[derive(Clone, Debug, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub(crate) enum DialInfo {
    UDP(DialInfoUDP),
    TCP(DialInfoTCP),
    WS(DialInfoWS),
    WSS(DialInfoWSS),
}
impl Default for DialInfo {
    fn default() -> Self {
        DialInfo::UDP(DialInfoUDP::default())
    }
}

impl fmt::Display for DialInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            DialInfo::UDP(di) => write!(f, "udp|{}", di.socket_address),
            DialInfo::TCP(di) => write!(f, "tcp|{}", di.socket_address),
            DialInfo::WS(di) => {
                let url = format!("ws://{}", di.request);
                let split_url = SplitUrl::from_str(&url).unwrap();
                match split_url.host {
                    SplitUrlHost::Hostname(_) => {
                        write!(f, "ws|{}|{}", di.socket_address.ip_addr(), di.request)
                    }
                    SplitUrlHost::IpAddr(a) => {
                        if di.socket_address.ip_addr() == a {
                            write!(f, "ws|{}", di.request)
                        } else {
                            panic!("resolved address does not match url: {}", di.request);
                        }
                    }
                }
            }
            DialInfo::WSS(di) => {
                let url = format!("wss://{}", di.request);
                let split_url = SplitUrl::from_str(&url).unwrap();
                match split_url.host {
                    SplitUrlHost::Hostname(_) => {
                        write!(f, "wss|{}|{}", di.socket_address.ip_addr(), di.request)
                    }
                    SplitUrlHost::IpAddr(a) => {
                        if di.socket_address.ip_addr() == a {
                            write!(f, "wss|{}", di.request)
                        } else {
                            panic!("resolved address does not match url: {}", di.request);
                        }
                    }
                }
            }
        }
    }
}

impl FromStr for DialInfo {
    type Err = VeilidAPIError;
    fn from_str(s: &str) -> VeilidAPIResult<DialInfo> {
        let (proto, rest) = s.split_once('|').ok_or_else(|| {
            VeilidAPIError::parse_error("DialInfo::from_str missing protocol '|' separator", s)
        })?;
        match proto {
            "udp" => {
                let socket_address = SocketAddress::from_str(rest)?;
                Ok(DialInfo::udp(socket_address))
            }
            "tcp" => {
                let socket_address = SocketAddress::from_str(rest)?;
                Ok(DialInfo::tcp(socket_address))
            }
            "ws" => {
                let url = format!("ws://{}", rest);
                let split_url = SplitUrl::from_str(&url).map_err(|e| {
                    VeilidAPIError::parse_error(format!("unable to split WS url: {}", e), &url)
                })?;
                if split_url.scheme != "ws" || !url.starts_with("ws://") {
                    apibail_parse_error!("incorrect scheme for WS dialinfo", url);
                }
                let url_port = split_url.port.unwrap_or(80u16);

                match rest.split_once('|') {
                    Some((sa, rest)) => {
                        let address = Address::from_str(sa)?;

                        DialInfo::try_ws(
                            SocketAddress::new(address, url_port),
                            format!("ws://{}", rest),
                        )
                    }
                    None => {
                        let address = Address::from_str(&split_url.host.to_string())?;
                        DialInfo::try_ws(
                            SocketAddress::new(address, url_port),
                            format!("ws://{}", rest),
                        )
                    }
                }
            }
            "wss" => {
                let url = format!("wss://{}", rest);
                let split_url = SplitUrl::from_str(&url).map_err(|e| {
                    VeilidAPIError::parse_error(format!("unable to split WSS url: {}", e), &url)
                })?;
                if split_url.scheme != "wss" || !url.starts_with("wss://") {
                    apibail_parse_error!("incorrect scheme for WSS dialinfo", url);
                }
                let url_port = split_url.port.unwrap_or(443u16);

                match rest.split_once('|') {
                    Some((sa, rest)) => {
                        let address = Address::from_str(sa)?;

                        DialInfo::try_wss(
                            SocketAddress::new(address, url_port),
                            format!("wss://{}", rest),
                        )
                    }
                    None => {
                        let address = Address::from_str(&split_url.host.to_string())?;
                        DialInfo::try_wss(
                            SocketAddress::new(address, url_port),
                            format!("wss://{}", rest),
                        )
                    }
                }
            }
            _ => Err(VeilidAPIError::parse_error(
                "DialInfo::from_str has invalid scheme",
                s,
            )),
        }
    }
}

impl DialInfo {
    pub fn udp_from_socketaddr(socket_addr: SocketAddr) -> Self {
        Self::UDP(DialInfoUDP {
            socket_address: SocketAddress::from_socket_addr(socket_addr).canonical(),
        })
    }
    pub fn tcp_from_socketaddr(socket_addr: SocketAddr) -> Self {
        Self::TCP(DialInfoTCP {
            socket_address: SocketAddress::from_socket_addr(socket_addr).canonical(),
        })
    }
    pub fn udp(socket_address: SocketAddress) -> Self {
        Self::UDP(DialInfoUDP {
            socket_address: socket_address.canonical(),
        })
    }
    pub fn tcp(socket_address: SocketAddress) -> Self {
        Self::TCP(DialInfoTCP {
            socket_address: socket_address.canonical(),
        })
    }
    pub fn try_ws(socket_address: SocketAddress, url: String) -> VeilidAPIResult<Self> {
        let split_url = SplitUrl::from_str(&url).map_err(|e| {
            VeilidAPIError::parse_error(format!("unable to split WS url: {}", e), &url)
        })?;
        if split_url.scheme != "ws" || !url.starts_with("ws://") {
            apibail_parse_error!("incorrect scheme for WS dialinfo", url);
        }
        let url_port = split_url.port.unwrap_or(80u16);
        if url_port != socket_address.port() {
            apibail_parse_error!("socket address port doesn't match url port", url);
        }
        if let SplitUrlHost::IpAddr(a) = split_url.host {
            if socket_address.ip_addr() != a {
                apibail_parse_error!(
                    format!("request address does not match socket address: {}", a),
                    socket_address
                );
            }
        }
        Ok(Self::WS(DialInfoWS {
            socket_address: socket_address.canonical(),
            request: url[5..].to_string(),
        }))
    }
    pub fn try_wss(socket_address: SocketAddress, url: String) -> VeilidAPIResult<Self> {
        let split_url = SplitUrl::from_str(&url).map_err(|e| {
            VeilidAPIError::parse_error(format!("unable to split WSS url: {}", e), &url)
        })?;
        if split_url.scheme != "wss" || !url.starts_with("wss://") {
            apibail_parse_error!("incorrect scheme for WSS dialinfo", url);
        }
        let url_port = split_url.port.unwrap_or(443u16);
        if url_port != socket_address.port() {
            apibail_parse_error!("socket address port doesn't match url port", url);
        }
        if let SplitUrlHost::IpAddr(a) = split_url.host {
            if socket_address.ip_addr() != a {
                apibail_parse_error!(
                    format!("request address does not match socket address: {}", a),
                    socket_address
                );
            }
        }
        Ok(Self::WSS(DialInfoWSS {
            socket_address: socket_address.canonical(),
            request: url[6..].to_string(),
        }))
    }
    pub fn protocol_type(&self) -> ProtocolType {
        match self {
            Self::UDP(_) => ProtocolType::UDP,
            Self::TCP(_) => ProtocolType::TCP,
            Self::WS(_) => ProtocolType::WS,
            Self::WSS(_) => ProtocolType::WSS,
        }
    }
    pub fn address_type(&self) -> AddressType {
        self.socket_address().address_type()
    }
    pub fn address(&self) -> Address {
        match self {
            Self::UDP(di) => di.socket_address.address(),
            Self::TCP(di) => di.socket_address.address(),
            Self::WS(di) => di.socket_address.address(),
            Self::WSS(di) => di.socket_address.address(),
        }
    }
    #[expect(dead_code)]
    pub fn set_address(&mut self, address: Address) {
        match self {
            Self::UDP(di) => di.socket_address.set_address(address),
            Self::TCP(di) => di.socket_address.set_address(address),
            Self::WS(di) => di.socket_address.set_address(address),
            Self::WSS(di) => di.socket_address.set_address(address),
        }
    }
    pub fn socket_address(&self) -> SocketAddress {
        match self {
            Self::UDP(di) => di.socket_address,
            Self::TCP(di) => di.socket_address,
            Self::WS(di) => di.socket_address,
            Self::WSS(di) => di.socket_address,
        }
    }
    pub fn ip_addr(&self) -> IpAddr {
        match self {
            Self::UDP(di) => di.socket_address.ip_addr(),
            Self::TCP(di) => di.socket_address.ip_addr(),
            Self::WS(di) => di.socket_address.ip_addr(),
            Self::WSS(di) => di.socket_address.ip_addr(),
        }
    }
    #[expect(dead_code)]
    pub fn port(&self) -> u16 {
        match self {
            Self::UDP(di) => di.socket_address.port(),
            Self::TCP(di) => di.socket_address.port(),
            Self::WS(di) => di.socket_address.port(),
            Self::WSS(di) => di.socket_address.port(),
        }
    }
    #[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), expect(dead_code))]
    pub fn set_port(&mut self, port: u16) {
        match self {
            Self::UDP(di) => di.socket_address.set_port(port),
            Self::TCP(di) => di.socket_address.set_port(port),
            Self::WS(di) => di.socket_address.set_port(port),
            Self::WSS(di) => di.socket_address.set_port(port),
        }
    }
    pub fn to_socket_addr(&self) -> SocketAddr {
        match self {
            Self::UDP(di) => di.socket_address.socket_addr(),
            Self::TCP(di) => di.socket_address.socket_addr(),
            Self::WS(di) => di.socket_address.socket_addr(),
            Self::WSS(di) => di.socket_address.socket_addr(),
        }
    }
    pub fn peer_address(&self) -> PeerAddress {
        match self {
            Self::UDP(di) => PeerAddress::new(di.socket_address, ProtocolType::UDP),
            Self::TCP(di) => PeerAddress::new(di.socket_address, ProtocolType::TCP),
            Self::WS(di) => PeerAddress::new(di.socket_address, ProtocolType::WS),
            Self::WSS(di) => PeerAddress::new(di.socket_address, ProtocolType::WSS),
        }
    }
    pub fn request(&self) -> Option<String> {
        match self {
            Self::UDP(_) => None,
            Self::TCP(_) => None,
            Self::WS(di) => Some(format!("ws://{}", di.request)),
            Self::WSS(di) => Some(format!("wss://{}", di.request)),
        }
    }
    pub fn is_valid(&self) -> bool {
        let socket_address = self.socket_address();
        let address = socket_address.address();
        let port = socket_address.port();
        (address.is_global() || address.is_local()) && port > 0
    }

    pub fn make_filter(&self) -> DialInfoFilter {
        DialInfoFilter {
            protocol_type_set: ProtocolTypeSet::only(self.protocol_type()),
            address_type_set: AddressTypeSet::only(self.address_type()),
        }
    }

    pub fn ordered_sequencing_sort(a: &DialInfo, b: &DialInfo) -> core::cmp::Ordering {
        let s = ProtocolType::ordered_sequencing_sort(a.protocol_type(), b.protocol_type());
        if s != core::cmp::Ordering::Equal {
            return s;
        }
        match (a, b) {
            (DialInfo::UDP(a), DialInfo::UDP(b)) => a.cmp(b),
            (DialInfo::TCP(a), DialInfo::TCP(b)) => a.cmp(b),
            (DialInfo::WS(a), DialInfo::WS(b)) => a.cmp(b),
            (DialInfo::WSS(a), DialInfo::WSS(b)) => a.cmp(b),
            _ => unreachable!(),
        }
    }
}

impl MatchesDialInfoFilter for DialInfo {
    fn matches_filter(&self, filter: &DialInfoFilter) -> bool {
        if !filter.protocol_type_set.contains(self.protocol_type()) {
            return false;
        }
        if !filter.address_type_set.contains(self.address_type()) {
            return false;
        }
        true
    }
}
