use super::*;

pub struct ShortDialInfo {
    pub short_url: String,
    pub hostname: String,
}

trait DialInfoConverterResolver: Send + Sync {
    fn ptr_lookup(&self, ip_addr: IpAddr) -> PinBoxFuture<'_, EyreResult<String>>;
    fn to_socket_addrs(
        &self,
        host: &str,
        default: SocketAddr,
    ) -> std::io::Result<std::vec::IntoIter<SocketAddr>>;
}

pub trait DialInfoConverter: Send + Sync {
    fn try_vec_from_short(&self, short_dial_info: &ShortDialInfo)
        -> VeilidAPIResult<Vec<DialInfo>>;
    fn try_vec_from_url(&self, url: &str) -> VeilidAPIResult<Vec<DialInfo>>;
    fn to_short(&self, dial_info: DialInfo) -> PinBoxFuture<'_, ShortDialInfo>;
    #[expect(dead_code)]
    fn to_url(&self, dial_info: DialInfo) -> PinBoxFuture<'_, String>;
}

impl<C> DialInfoConverter for C
where
    C: DialInfoConverterResolver,
{
    fn try_vec_from_short(
        &self,
        short_dial_info: &ShortDialInfo,
    ) -> VeilidAPIResult<Vec<DialInfo>> {
        let short = &short_dial_info.short_url;
        let hostname = &short_dial_info.hostname;

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
        self.try_vec_from_url(&url)
    }

    fn try_vec_from_url(&self, url: &str) -> VeilidAPIResult<Vec<DialInfo>> {
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

        let socket_addrs = match split_url.host {
            SplitUrlHost::Hostname(_) => self
                .to_socket_addrs(
                    &split_url.host_port(port),
                    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port),
                )
                .map_err(|_| VeilidAPIError::parse_error("couldn't resolve hostname in url", url))?
                .collect(),
            SplitUrlHost::IpAddr(a) => vec![SocketAddr::new(a, port)],
        };

        let mut out = Vec::new();
        for sa in socket_addrs {
            out.push(match split_url.scheme.as_str() {
                "udp" => DialInfo::udp_from_socketaddr(sa),
                "tcp" => DialInfo::tcp_from_socketaddr(sa),
                "ws" => DialInfo::try_ws(
                    SocketAddress::from_socket_addr(sa).canonical(),
                    url.to_string(),
                )?,
                "wss" => DialInfo::try_wss(
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

    fn to_short(&self, dial_info: DialInfo) -> PinBoxFuture<'_, ShortDialInfo> {
        pin_dyn_future!(async move {
            match dial_info {
                DialInfo::UDP(di) => ShortDialInfo {
                    short_url: format!("U{}", di.socket_address.port()),
                    hostname: self
                        .ptr_lookup(di.socket_address.ip_addr())
                        .await
                        .unwrap_or_else(|_| di.socket_address.to_string()),
                },
                DialInfo::TCP(di) => ShortDialInfo {
                    short_url: format!("T{}", di.socket_address.port()),
                    hostname: self
                        .ptr_lookup(di.socket_address.ip_addr())
                        .await
                        .unwrap_or_else(|_| di.socket_address.to_string()),
                },
                DialInfo::WS(di) => {
                    let mut split_url =
                        SplitUrl::from_str(&format!("ws://{}", di.request)).unwrap();
                    if let SplitUrlHost::IpAddr(a) = split_url.host {
                        if let Ok(host) = self.ptr_lookup(a).await {
                            split_url.host = SplitUrlHost::Hostname(host);
                        }
                    }
                    ShortDialInfo {
                        short_url: format!(
                            "W{}{}",
                            split_url.port.unwrap_or(80),
                            split_url
                                .path
                                .map(|p| format!("/{}", p))
                                .unwrap_or_default()
                        ),
                        hostname: split_url.host.to_string(),
                    }
                }
                DialInfo::WSS(di) => {
                    let mut split_url =
                        SplitUrl::from_str(&format!("wss://{}", di.request)).unwrap();
                    if let SplitUrlHost::IpAddr(a) = split_url.host {
                        if let Ok(host) = self.ptr_lookup(a).await {
                            split_url.host = SplitUrlHost::Hostname(host);
                        }
                    }
                    ShortDialInfo {
                        short_url: format!(
                            "S{}{}",
                            split_url.port.unwrap_or(443),
                            split_url
                                .path
                                .map(|p| format!("/{}", p))
                                .unwrap_or_default()
                        ),
                        hostname: split_url.host.to_string(),
                    }
                }
            }
        })
    }

    fn to_url(&self, dial_info: DialInfo) -> PinBoxFuture<'_, String> {
        pin_dyn_future!(async move {
            match dial_info {
                DialInfo::UDP(di) => self
                    .ptr_lookup(di.socket_address.ip_addr())
                    .await
                    .map(|h| format!("udp://{}:{}", h, di.socket_address.port()))
                    .unwrap_or_else(|_| format!("udp://{}", di.socket_address)),
                DialInfo::TCP(di) => self
                    .ptr_lookup(di.socket_address.ip_addr())
                    .await
                    .map(|h| format!("tcp://{}:{}", h, di.socket_address.port()))
                    .unwrap_or_else(|_| format!("tcp://{}", di.socket_address)),
                DialInfo::WS(di) => {
                    let mut split_url =
                        SplitUrl::from_str(&format!("ws://{}", di.request)).unwrap();
                    if let SplitUrlHost::IpAddr(a) = split_url.host {
                        if let Ok(host) = self.ptr_lookup(a).await {
                            split_url.host = SplitUrlHost::Hostname(host);
                        }
                    }
                    split_url.to_string()
                }
                DialInfo::WSS(di) => {
                    let mut split_url =
                        SplitUrl::from_str(&format!("wss://{}", di.request)).unwrap();
                    if let SplitUrlHost::IpAddr(a) = split_url.host {
                        if let Ok(host) = self.ptr_lookup(a).await {
                            split_url.host = SplitUrlHost::Hostname(host);
                        }
                    }
                    split_url.to_string()
                }
            }
        })
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BootstrapDialInfoConverter {}

impl DialInfoConverterResolver for BootstrapDialInfoConverter {
    fn ptr_lookup(&self, ip_addr: IpAddr) -> PinBoxFuture<'_, EyreResult<String>> {
        pin_dyn_future!(async move { intf::ptr_lookup(ip_addr).await })
    }

    #[allow(unused_variables)]
    fn to_socket_addrs(
        &self,
        host: &str,
        default: SocketAddr,
    ) -> std::io::Result<std::vec::IntoIter<SocketAddr>> {
        // Resolve if possible, WASM doesn't support resolution and doesn't need it to connect to the dialinfo
        // This will not be used on signed dialinfo, only for bootstrapping, so we don't need to worry about
        // the '0.0.0.0' address being propagated across the routing table

        cfg_if::cfg_if! {
            if #[cfg(all(target_arch = "wasm32", target_os = "unknown"))] {
                Ok(vec![default].into_iter())
            } else {
                host.to_socket_addrs()
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct MockDialInfoConverter {}

impl DialInfoConverterResolver for MockDialInfoConverter {
    fn ptr_lookup(&self, _ip_addr: IpAddr) -> PinBoxFuture<'_, EyreResult<String>> {
        pin_dyn_future!(async move { Ok("fake_hostname".to_string()) })
    }

    fn to_socket_addrs(
        &self,
        _host: &str,
        default: SocketAddr,
    ) -> std::io::Result<std::vec::IntoIter<SocketAddr>> {
        Ok(vec![default].into_iter())
    }
}
