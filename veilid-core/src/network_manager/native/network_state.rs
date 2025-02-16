use super::*;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ProtocolConfig {
    pub outbound: ProtocolTypeSet,
    pub inbound: ProtocolTypeSet,
    pub family_global: AddressTypeSet,
    pub family_local: AddressTypeSet,
    pub public_internet_capabilities: Vec<FourCC>,
    pub local_network_capabilities: Vec<FourCC>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct NetworkState {
    /// the calculated protocol configuration for inbound/outbound protocols
    pub protocol_config: ProtocolConfig,
    /// does our network have ipv4 on any interface?
    pub enable_ipv4: bool,
    /// does our network have ipv6 on any interface?
    pub enable_ipv6: bool,
    /// The list of stable interface addresses we have last seen
    pub stable_interface_addresses: Vec<IpAddr>,
    /// The local networks (network+mask) most recently seen
    pub local_networks: Vec<(IpAddr, IpAddr)>,
}

impl Network {
    fn make_stable_interface_addresses(&self) -> Vec<IpAddr> {
        let addrs = self.interfaces.stable_addresses();
        let mut addrs: Vec<IpAddr> = addrs
            .into_iter()
            .filter(|addr| {
                let address = Address::from_ip_addr(*addr);
                address.is_local() || address.is_global()
            })
            .collect();
        addrs.sort();
        addrs.dedup();
        addrs
    }

    pub(super) fn last_network_state(&self) -> Option<NetworkState> {
        self.inner.lock().network_state.clone()
    }

    pub(super) fn is_stable_interface_address(&self, addr: IpAddr) -> bool {
        self.inner
            .lock()
            .network_state
            .as_ref()
            .unwrap()
            .stable_interface_addresses
            .contains(&addr)
    }

    pub(super) async fn make_network_state(&self) -> EyreResult<NetworkState> {
        // refresh network interfaces
        self.interfaces
            .refresh()
            .await
            .wrap_err("failed to refresh network interfaces")?;

        // build the set of networks we should consider for the 'LocalNetwork' routing domain
        let mut local_networks: HashSet<(IpAddr, IpAddr)> = HashSet::new();

        self.interfaces.with_interfaces(|interfaces| {
            for intf in interfaces.values() {
                // Skip networks that we should never encounter
                if intf.is_loopback() || !intf.is_running() {
                    continue;
                }
                // Add network to local networks table
                for addr in &intf.addrs {
                    let netmask = addr.if_addr().netmask();
                    let network_ip = ipaddr_apply_netmask(addr.if_addr().ip(), netmask);
                    local_networks.insert((network_ip, netmask));
                }
            }
        });
        let mut local_networks: Vec<(IpAddr, IpAddr)> = local_networks.into_iter().collect();
        local_networks.sort();

        // determine if we have ipv4/ipv6 addresses
        let mut enable_ipv4 = false;
        let mut enable_ipv6 = false;

        let stable_interface_addresses = self.make_stable_interface_addresses();

        for addr in stable_interface_addresses.iter() {
            if addr.is_ipv4() {
                enable_ipv4 = true;
            } else if addr.is_ipv6() {
                enable_ipv6 = true;
            }
        }

        // Get protocol config
        let protocol_config = {
            let config = self.config();
            let c = config.get();
            let mut inbound = ProtocolTypeSet::new();

            if c.network.protocol.udp.enabled {
                inbound.insert(ProtocolType::UDP);
            }
            if c.network.protocol.tcp.listen {
                inbound.insert(ProtocolType::TCP);
            }
            if c.network.protocol.ws.listen {
                inbound.insert(ProtocolType::WS);
            }
            if c.network.protocol.wss.listen {
                inbound.insert(ProtocolType::WSS);
            }

            let mut outbound = ProtocolTypeSet::new();
            if c.network.protocol.udp.enabled {
                outbound.insert(ProtocolType::UDP);
            }
            if c.network.protocol.tcp.connect {
                outbound.insert(ProtocolType::TCP);
            }
            if c.network.protocol.ws.connect {
                outbound.insert(ProtocolType::WS);
            }
            if c.network.protocol.wss.connect {
                outbound.insert(ProtocolType::WSS);
            }

            let mut family_global = AddressTypeSet::new();
            let mut family_local = AddressTypeSet::new();
            if enable_ipv4 {
                family_global.insert(AddressType::IPV4);
                family_local.insert(AddressType::IPV4);
            }
            if enable_ipv6 {
                family_global.insert(AddressType::IPV6);
                family_local.insert(AddressType::IPV6);
            }

            // set up the routing table's network config
            // if we have static public dialinfo, upgrade our network class
            let public_internet_capabilities = {
                PUBLIC_INTERNET_CAPABILITIES
                    .iter()
                    .copied()
                    .filter(|cap| !c.capabilities.disable.contains(cap))
                    .collect::<Vec<Capability>>()
            };
            let local_network_capabilities = {
                LOCAL_NETWORK_CAPABILITIES
                    .iter()
                    .copied()
                    .filter(|cap| !c.capabilities.disable.contains(cap))
                    .collect::<Vec<Capability>>()
            };

            ProtocolConfig {
                outbound,
                inbound,
                family_global,
                family_local,
                public_internet_capabilities,
                local_network_capabilities,
            }
        };

        Ok(NetworkState {
            protocol_config,
            enable_ipv4,
            enable_ipv6,
            stable_interface_addresses,
            local_networks,
        })
    }
}
