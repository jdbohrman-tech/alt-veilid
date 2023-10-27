mod tcp;
mod udp;
mod ws;
mod wss;

use super::*;

pub use tcp::*;
pub use udp::*;
pub use ws::*;
pub use wss::*;

// Keep member order appropriate for sorting < preference
// Must match ProtocolType order
#[derive(Clone, Debug, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum DialInfo {
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
    #[allow(dead_code)]
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
    pub fn port(&self) -> u16 {
        match self {
            Self::UDP(di) => di.socket_address.port(),
            Self::TCP(di) => di.socket_address.port(),
            Self::WS(di) => di.socket_address.port(),
            Self::WSS(di) => di.socket_address.port(),
        }
    }
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

    pub fn try_vec_from_short<S: AsRef<str>, H: AsRef<str>>(
        short: S,
        hostname: H,
    ) -> VeilidAPIResult<Vec<Self>> {
        let short = short.as_ref();
        let hostname = hostname.as_ref();

        if short.len() < 2 {
            apibail_parse_error!("invalid short url length", short);
        }
        let url = match &short[0..1] {
            "U" => {
                format!("udp://{}:{}", hostname, &short[1..])
            }
            "T" => {
                format!("tcp://{}:{}", hostname, &short[1..])
            }
            "W" => {
                format!("ws://{}:{}", hostname, &short[1..])
            }
            "S" => {
                format!("wss://{}:{}", hostname, &short[1..])
            }
            _ => {
                apibail_parse_error!("invalid short url type", short);
            }
        };
        Self::try_vec_from_url(url)
    }

    pub fn try_vec_from_url<S: AsRef<str>>(url: S) -> VeilidAPIResult<Vec<Self>> {
        let url = url.as_ref();
        let split_url = SplitUrl::from_str(url)
            .map_err(|e| VeilidAPIError::parse_error(format!("unable to split url: {}", e), url))?;

        let port = match split_url.scheme.as_str() {
            "udp" | "tcp" => split_url
                .port
                .ok_or_else(|| VeilidAPIError::parse_error("Missing port in udp url", url))?,
            "ws" => split_url.port.unwrap_or(80u16),
            "wss" => split_url.port.unwrap_or(443u16),
            _ => {
                apibail_parse_error!("Invalid dial info url scheme", split_url.scheme);
            }
        };

        let socket_addrs = {
            // Resolve if possible, WASM doesn't support resolution and doesn't need it to connect to the dialinfo
            // This will not be used on signed dialinfo, only for bootstrapping, so we don't need to worry about
            // the '0.0.0.0' address being propagated across the routing table
            cfg_if::cfg_if! {
                if #[cfg(target_arch = "wasm32")] {
                    vec![SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0,0,0,0)), port)]
                } else {
                    match split_url.host {
                        SplitUrlHost::Hostname(_) => split_url
                            .host_port(port)
                            .to_socket_addrs()
                            .map_err(|_| VeilidAPIError::parse_error("couldn't resolve hostname in url", url))?
                            .collect(),
                        SplitUrlHost::IpAddr(a) => vec![SocketAddr::new(a, port)],
                    }
                }
            }
        };

        let mut out = Vec::new();
        for sa in socket_addrs {
            out.push(match split_url.scheme.as_str() {
                "udp" => Self::udp_from_socketaddr(sa),
                "tcp" => Self::tcp_from_socketaddr(sa),
                "ws" => Self::try_ws(
                    SocketAddress::from_socket_addr(sa).canonical(),
                    url.to_string(),
                )?,
                "wss" => Self::try_wss(
                    SocketAddress::from_socket_addr(sa).canonical(),
                    url.to_string(),
                )?,
                _ => {
                    unreachable!("Invalid dial info url scheme")
                }
            });
        }
        Ok(out)
    }

    pub async fn to_short(&self) -> (String, String) {
        match self {
            DialInfo::UDP(di) => (
                format!("U{}", di.socket_address.port()),
                intf::ptr_lookup(di.socket_address.ip_addr())
                    .await
                    .unwrap_or_else(|_| di.socket_address.to_string()),
            ),
            DialInfo::TCP(di) => (
                format!("T{}", di.socket_address.port()),
                intf::ptr_lookup(di.socket_address.ip_addr())
                    .await
                    .unwrap_or_else(|_| di.socket_address.to_string()),
            ),
            DialInfo::WS(di) => {
                let mut split_url = SplitUrl::from_str(&format!("ws://{}", di.request)).unwrap();
                if let SplitUrlHost::IpAddr(a) = split_url.host {
                    if let Ok(host) = intf::ptr_lookup(a).await {
                        split_url.host = SplitUrlHost::Hostname(host);
                    }
                }
                (
                    format!(
                        "W{}{}",
                        split_url.port.unwrap_or(80),
                        split_url
                            .path
                            .map(|p| format!("/{}", p))
                            .unwrap_or_default()
                    ),
                    split_url.host.to_string(),
                )
            }
            DialInfo::WSS(di) => {
                let mut split_url = SplitUrl::from_str(&format!("wss://{}", di.request)).unwrap();
                if let SplitUrlHost::IpAddr(a) = split_url.host {
                    if let Ok(host) = intf::ptr_lookup(a).await {
                        split_url.host = SplitUrlHost::Hostname(host);
                    }
                }
                (
                    format!(
                        "S{}{}",
                        split_url.port.unwrap_or(443),
                        split_url
                            .path
                            .map(|p| format!("/{}", p))
                            .unwrap_or_default()
                    ),
                    split_url.host.to_string(),
                )
            }
        }
    }
    #[allow(dead_code)]
    pub async fn to_url(&self) -> String {
        match self {
            DialInfo::UDP(di) => intf::ptr_lookup(di.socket_address.ip_addr())
                .await
                .map(|h| format!("udp://{}:{}", h, di.socket_address.port()))
                .unwrap_or_else(|_| format!("udp://{}", di.socket_address)),
            DialInfo::TCP(di) => intf::ptr_lookup(di.socket_address.ip_addr())
                .await
                .map(|h| format!("tcp://{}:{}", h, di.socket_address.port()))
                .unwrap_or_else(|_| format!("tcp://{}", di.socket_address)),
            DialInfo::WS(di) => {
                let mut split_url = SplitUrl::from_str(&format!("ws://{}", di.request)).unwrap();
                if let SplitUrlHost::IpAddr(a) = split_url.host {
                    if let Ok(host) = intf::ptr_lookup(a).await {
                        split_url.host = SplitUrlHost::Hostname(host);
                    }
                }
                split_url.to_string()
            }
            DialInfo::WSS(di) => {
                let mut split_url = SplitUrl::from_str(&format!("wss://{}", di.request)).unwrap();
                if let SplitUrlHost::IpAddr(a) = split_url.host {
                    if let Ok(host) = intf::ptr_lookup(a).await {
                        split_url.host = SplitUrlHost::Hostname(host);
                    }
                }
                split_url.to_string()
            }
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
