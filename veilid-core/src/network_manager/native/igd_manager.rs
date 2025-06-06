use super::*;
use igd::*;
use std::net::UdpSocket;

impl_veilid_log_facility!("net");

const UPNP_GATEWAY_DETECT_TIMEOUT_MS: u32 = 5_000;
const UPNP_MAPPING_LIFETIME_MS: u32 = 120_000;
const UPNP_MAPPING_ATTEMPTS: u32 = 3;
const UPNP_MAPPING_LIFETIME_US: u64 = UPNP_MAPPING_LIFETIME_MS as u64 * 1000u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct PortMapKey {
    protocol_type: IGDProtocolType,
    address_type: IGDAddressType,
    local_port: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct PortMapValue {
    ext_ip: IpAddr,
    mapped_port: u16,
    timestamp: u64,
    renewal_lifetime: u64,
    renewal_attempts: u32,
}

struct IGDManagerInner {
    local_ip_addrs: BTreeMap<IGDAddressType, IpAddr>,
    gateways: BTreeMap<IpAddr, Arc<Gateway>>,
    port_maps: BTreeMap<PortMapKey, PortMapValue>,
}

#[derive(Clone)]
pub struct IGDManager {
    registry: VeilidComponentRegistry,
    inner: Arc<Mutex<IGDManagerInner>>,
}

impl_veilid_component_registry_accessor!(IGDManager);

fn convert_protocol_type(igdpt: IGDProtocolType) -> PortMappingProtocol {
    match igdpt {
        IGDProtocolType::UDP => PortMappingProtocol::UDP,
        IGDProtocolType::TCP => PortMappingProtocol::TCP,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IGDAddressType {
    IPV6,
    IPV4,
}

impl fmt::Display for IGDAddressType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IGDAddressType::IPV6 => write!(f, "IPV6"),
            IGDAddressType::IPV4 => write!(f, "IPV4"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IGDProtocolType {
    UDP,
    TCP,
}

impl fmt::Display for IGDProtocolType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IGDProtocolType::UDP => write!(f, "UDP"),
            IGDProtocolType::TCP => write!(f, "TCP"),
        }
    }
}

impl IGDManager {
    /////////////////////////////////////////////////////////////////////
    // Public Interface

    pub fn new(registry: VeilidComponentRegistry) -> Self {
        Self {
            registry,
            inner: Arc::new(Mutex::new(IGDManagerInner {
                local_ip_addrs: BTreeMap::new(),
                gateways: BTreeMap::new(),
                port_maps: BTreeMap::new(),
            })),
        }
    }

    #[instrument(level = "trace", target = "net", skip_all)]
    pub async fn unmap_port(
        &self,
        protocol_type: IGDProtocolType,
        address_type: IGDAddressType,
        mapped_port: u16,
    ) -> Option<()> {
        let this = self.clone();
        blocking_wrapper(
            "igd unmap_port",
            move || {
                let mut inner = this.inner.lock();

                // If we already have this port mapped, just return the existing portmap
                let mut found = None;
                for (pmk, pmv) in &inner.port_maps {
                    if pmk.protocol_type == protocol_type
                        && pmk.address_type == address_type
                        && pmv.mapped_port == mapped_port
                    {
                        found = Some(*pmk);
                        break;
                    }
                }
                let pmk = found?;
                let _pmv = inner
                    .port_maps
                    .remove(&pmk)
                    .expect("key found but remove failed");

                // Get local ip address
                let local_ip = this.find_local_ip_inner(&mut inner, address_type)?;

                // Find gateway
                let gw = this.find_gateway_inner(&mut inner, local_ip)?;

                // Unmap port
                match gw.remove_port(convert_protocol_type(protocol_type), mapped_port) {
                    Ok(()) => (),
                    Err(e) => {
                        // Failed to map external port
                        veilid_log!(this debug "upnp failed to remove external port: {}", e);
                        return None;
                    }
                };
                Some(())
            },
            None,
        )
        .await
    }

    #[instrument(level = "trace", target = "net", skip_all)]
    pub async fn map_any_port(
        &self,
        protocol_type: IGDProtocolType,
        address_type: IGDAddressType,
        local_port: u16,
        expected_external_address: Option<IpAddr>,
    ) -> Option<SocketAddr> {
        let this = self.clone();
        blocking_wrapper("igd map_any_port", move || {
            let mut inner = this.inner.lock();

            // If we already have this port mapped, just return the existing portmap
            let pmkey = PortMapKey {
                protocol_type,
                address_type,
                local_port,
            };
            if let Some(pmval) = inner.port_maps.get(&pmkey) {
                return Some(SocketAddr::new(pmval.ext_ip, pmval.mapped_port));
            }

            // Get local ip address
            let local_ip = this.find_local_ip_inner(&mut inner, address_type)?;

            // Find gateway
            let gw = this.find_gateway_inner(&mut inner, local_ip)?;

            // Get external address
            let ext_ip = match gw.get_external_ip() {
                Ok(ip) => ip,
                Err(e) => {
                    veilid_log!(this debug "couldn't get external ip from igd: {}", e);
                    return None;
                }
            };

            // Ensure external IP matches address type
            if ext_ip.is_ipv4() && address_type != IGDAddressType::IPV4 {
                veilid_log!(this debug "mismatched ip address type from igd, wanted v4, got v6");
                return None;
            } else if ext_ip.is_ipv6() && address_type != IGDAddressType::IPV6 {
                veilid_log!(this debug "mismatched ip address type from igd, wanted v6, got v4");
                return None;
            }

            if let Some(expected_external_address) = expected_external_address {
                if ext_ip != expected_external_address {
                    veilid_log!(this debug "gateway external address does not match calculated external address: expected={} vs gateway={}", expected_external_address, ext_ip);
                    return None;
                }
            }

            // Map any port
            let desc = this.get_description(protocol_type, local_port);
            let mapped_port = match gw.add_any_port(convert_protocol_type(protocol_type), SocketAddr::new(local_ip, local_port), UPNP_MAPPING_LIFETIME_MS.div_ceil(1000), &desc) {
                Ok(mapped_port) => mapped_port,
                Err(e) => {
                    // Failed to map external port
                    veilid_log!(this debug "upnp failed to map external port: {}", e);
                    return None;
                }
            };

            // Add to mapping list to keep alive
            let timestamp = get_timestamp();
            inner.port_maps.insert(PortMapKey {
                protocol_type,
                address_type,
                local_port,
            }, PortMapValue {
                ext_ip,
                mapped_port,
                timestamp,
                renewal_lifetime: (UPNP_MAPPING_LIFETIME_MS / 2) as u64 * 1000u64,
                renewal_attempts: 0,
            });

            // Succeeded, return the externally mapped port
            Some(SocketAddr::new(ext_ip, mapped_port))
        }, None)
        .await
    }

    #[instrument(
        level = "trace",
        target = "net",
        name = "IGDManager::tick",
        skip_all,
        err
    )]
    pub async fn tick(&self) -> EyreResult<bool> {
        // Refresh mappings if we have them
        // If an error is received, then return false to restart the local network
        let mut full_renews: Vec<(PortMapKey, PortMapValue)> = Vec::new();
        let mut renews: Vec<(PortMapKey, PortMapValue)> = Vec::new();
        {
            let inner = self.inner.lock();
            let now = get_timestamp();

            for (k, v) in &inner.port_maps {
                let mapping_lifetime = now.saturating_sub(v.timestamp);
                if mapping_lifetime >= UPNP_MAPPING_LIFETIME_US
                    || v.renewal_attempts >= UPNP_MAPPING_ATTEMPTS
                {
                    // Past expiration time or tried N times, do a full renew and fail out if we can't
                    full_renews.push((*k, *v));
                } else if mapping_lifetime >= v.renewal_lifetime {
                    // Attempt a normal renewal
                    renews.push((*k, *v));
                }
            }

            // See if we need to do some blocking operations
            if full_renews.is_empty() && renews.is_empty() {
                // Just return now since there's nothing to renew
                return Ok(true);
            }
        }

        let this = self.clone();
        blocking_wrapper(
            "igd tick",
            move || {
                let mut inner = this.inner.lock();

                // Process full renewals
                for (k, v) in full_renews {
                    // Get local ip for address type
                    let local_ip = match this.get_local_ip_inner(&mut inner, k.address_type) {
                        Some(ip) => ip,
                        None => {
                            return Err(eyre!("local ip missing for address type"));
                        }
                    };

                    // Get gateway for interface
                    let gw = match Self::get_gateway_inner(&mut inner, local_ip) {
                        Some(gw) => gw,
                        None => {
                            return Err(eyre!("gateway missing for interface"));
                        }
                    };

                    // Delete the mapping if it exists, ignore any errors here
                    let _ = gw.remove_port(convert_protocol_type(k.protocol_type), v.mapped_port);
                    inner.port_maps.remove(&k);

                    let desc = this.get_description(k.protocol_type, k.local_port);
                    match gw.add_any_port(
                        convert_protocol_type(k.protocol_type),
                        SocketAddr::new(local_ip, k.local_port),
                        UPNP_MAPPING_LIFETIME_MS.div_ceil(1000),
                        &desc,
                    ) {
                        Ok(mapped_port) => {
                            veilid_log!(this debug "full-renewed mapped port {:?} -> {:?}", v, k);
                            inner.port_maps.insert(
                                k,
                                PortMapValue {
                                    ext_ip: v.ext_ip,
                                    mapped_port,
                                    timestamp: get_timestamp(),
                                    renewal_lifetime: (UPNP_MAPPING_LIFETIME_MS / 2) as u64
                                        * 1000u64,
                                    renewal_attempts: 0,
                                },
                            );
                        }
                        Err(e) => {
                            veilid_log!(this info "failed to full-renew mapped port {:?} -> {:?}: {}", v, k, e);

                            // Must restart network now :(
                            return Ok(false);
                        }
                    };
                }
                // Process normal renewals
                for (k, mut v) in renews {
                    // Get local ip for address type
                    let local_ip = match this.get_local_ip_inner(&mut inner, k.address_type) {
                        Some(ip) => ip,
                        None => {
                            return Err(eyre!("local ip missing for address type"));
                        }
                    };

                    // Get gateway for interface
                    let gw = match Self::get_gateway_inner(&mut inner, local_ip) {
                        Some(gw) => gw,
                        None => {
                            return Err(eyre!("gateway missing for address type"));
                        }
                    };

                    let desc = this.get_description(k.protocol_type, k.local_port);
                    match gw.add_port(
                        convert_protocol_type(k.protocol_type),
                        v.mapped_port,
                        SocketAddr::new(local_ip, k.local_port),
                        UPNP_MAPPING_LIFETIME_MS.div_ceil(1000),
                        &desc,
                    ) {
                        Ok(()) => {
                            veilid_log!(this trace "renewed mapped port {:?} -> {:?}", v, k);

                            inner.port_maps.insert(
                                k,
                                PortMapValue {
                                    ext_ip: v.ext_ip,
                                    mapped_port: v.mapped_port,
                                    timestamp: get_timestamp(),
                                    renewal_lifetime: (UPNP_MAPPING_LIFETIME_MS / 2) as u64
                                        * 1000u64,
                                    renewal_attempts: 0,
                                },
                            );
                        }
                        Err(e) => {
                            veilid_log!(this debug "failed to renew mapped port {:?} -> {:?}: {}", v, k, e);

                            // Get closer to the maximum renewal timeline by a factor of two each time
                            v.renewal_lifetime =
                                (v.renewal_lifetime + UPNP_MAPPING_LIFETIME_US) / 2u64;
                            v.renewal_attempts += 1;

                            // Store new value to try again
                            inner.port_maps.insert(k, v);
                        }
                    };
                }

                // Normal exit, no restart
                Ok(true)
            },
            Err(eyre!("failed to process blocking task")),
        )
        .instrument(tracing::trace_span!("igd tick fut"))
        .await
    }

    /////////////////////////////////////////////////////////////////////
    // Private Implementation

    #[instrument(level = "trace", target = "net", skip_all)]
    fn get_routed_local_ip_address(&self, address_type: IGDAddressType) -> Option<IpAddr> {
        let socket = match UdpSocket::bind(match address_type {
            IGDAddressType::IPV4 => SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0),
            IGDAddressType::IPV6 => SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), 0),
        }) {
            Ok(s) => s,
            Err(e) => {
                veilid_log!(self debug "failed to bind to unspecified address: {}", e);
                return None;
            }
        };

        // can be any routable ip address,
        // this is just to make the system routing table calculate the appropriate local ip address
        // using google's dns, but it wont actually send any packets to it
        socket
            .connect(match address_type {
                IGDAddressType::IPV4 => SocketAddr::new(IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)), 80),
                IGDAddressType::IPV6 => SocketAddr::new(
                    IpAddr::V6(Ipv6Addr::new(0x2001, 0x4860, 0x4860, 0, 0, 0, 0, 0x8888)),
                    80,
                ),
            })
            .map_err(|e| {
                veilid_log!(self debug "failed to connect to dummy address: {}", e);
                e
            })
            .ok()?;

        Some(socket.local_addr().ok()?.ip())
    }

    #[instrument(level = "trace", target = "net", skip_all)]
    fn find_local_ip_inner(
        &self,
        inner: &mut IGDManagerInner,
        address_type: IGDAddressType,
    ) -> Option<IpAddr> {
        if let Some(ip) = inner.local_ip_addrs.get(&address_type) {
            return Some(*ip);
        }

        let ip = match self.get_routed_local_ip_address(address_type) {
            Some(x) => x,
            None => {
                veilid_log!(self debug "failed to get local ip address: address_type={:?}", address_type);
                return None;
            }
        };

        inner.local_ip_addrs.insert(address_type, ip);
        Some(ip)
    }

    #[instrument(level = "trace", target = "net", skip_all)]
    fn get_local_ip_inner(
        &self,
        inner: &mut IGDManagerInner,
        address_type: IGDAddressType,
    ) -> Option<IpAddr> {
        if let Some(ip) = inner.local_ip_addrs.get(&address_type) {
            return Some(*ip);
        }
        None
    }

    #[instrument(level = "trace", target = "net", skip_all)]
    fn find_gateway_inner(
        &self,
        inner: &mut IGDManagerInner,
        local_ip: IpAddr,
    ) -> Option<Arc<Gateway>> {
        if let Some(gw) = inner.gateways.get(&local_ip) {
            return Some(gw.clone());
        }

        let gateway = match local_ip {
            IpAddr::V4(v4) => {
                let mut opts = SearchOptions::new_v4(UPNP_GATEWAY_DETECT_TIMEOUT_MS as u64);
                opts.bind_addr = SocketAddr::V4(SocketAddrV4::new(v4, 0));

                match igd::search_gateway(opts) {
                    Ok(v) => v,
                    Err(e) => {
                        veilid_log!(self debug "couldn't find ipv4 igd: {}", e);
                        return None;
                    }
                }
            }
            IpAddr::V6(v6) => {
                let mut opts = SearchOptions::new_v6(
                    Ipv6SearchScope::LinkLocal,
                    UPNP_GATEWAY_DETECT_TIMEOUT_MS as u64,
                );
                opts.bind_addr = SocketAddr::V6(SocketAddrV6::new(v6, 0, 0, 0));

                match igd::search_gateway(opts) {
                    Ok(v) => v,
                    Err(e) => {
                        veilid_log!(self debug "couldn't find ipv6 igd: {}", e);
                        return None;
                    }
                }
            }
        };
        let gw = Arc::new(gateway);
        inner.gateways.insert(local_ip, gw.clone());
        Some(gw)
    }

    #[instrument(level = "trace", target = "net", skip_all)]
    fn get_gateway_inner(inner: &mut IGDManagerInner, local_ip: IpAddr) -> Option<Arc<Gateway>> {
        if let Some(gw) = inner.gateways.get(&local_ip) {
            return Some(gw.clone());
        }
        None
    }

    fn get_description(&self, protocol_type: IGDProtocolType, local_port: u16) -> String {
        format!(
            "{} map {} for port {}",
            self.registry.program_name(),
            protocol_type,
            local_port
        )
    }
}
