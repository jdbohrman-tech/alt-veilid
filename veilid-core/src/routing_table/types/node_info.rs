use super::*;

fourcc_type!(VeilidCapability);
pub const CAP_ROUTE: VeilidCapability = VeilidCapability(*b"ROUT");
#[cfg(feature = "unstable-tunnels")]
pub const CAP_TUNNEL: VeilidCapability = VeilidCapability(*b"TUNL");
pub const CAP_SIGNAL: VeilidCapability = VeilidCapability(*b"SGNL");
pub const CAP_RELAY: VeilidCapability = VeilidCapability(*b"RLAY");
pub const CAP_VALIDATE_DIAL_INFO: VeilidCapability = VeilidCapability(*b"DIAL");
pub const CAP_DHT: VeilidCapability = VeilidCapability(*b"DHTV");
pub const CAP_DHT_WATCH: VeilidCapability = VeilidCapability(*b"DHTW");
pub const CAP_APPMESSAGE: VeilidCapability = VeilidCapability(*b"APPM");
#[cfg(feature = "unstable-blockstore")]
pub const CAP_BLOCKSTORE: VeilidCapability = VeilidCapability(*b"BLOC");

pub const DISTANCE_METRIC_CAPABILITIES: &[VeilidCapability] = &[CAP_DHT, CAP_DHT_WATCH];
pub const CONNECTIVITY_CAPABILITIES: &[VeilidCapability] =
    &[CAP_RELAY, CAP_SIGNAL, CAP_ROUTE, CAP_VALIDATE_DIAL_INFO];

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct NodeInfo {
    network_class: NetworkClass,
    outbound_protocols: ProtocolTypeSet,
    address_types: AddressTypeSet,
    envelope_support: Vec<u8>,
    crypto_support: Vec<CryptoKind>,
    capabilities: Vec<VeilidCapability>,
    dial_info_detail_list: Vec<DialInfoDetail>,
}

impl fmt::Display for NodeInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "network_class:      {:?}", self.network_class)?;
        writeln!(f, "outbound_protocols: {:?}", self.outbound_protocols)?;
        writeln!(f, "address_types:      {:?}", self.address_types)?;
        writeln!(f, "envelope_support:   {:?}", self.envelope_support)?;
        writeln!(f, "crypto_support:     {:?}", self.crypto_support)?;
        writeln!(f, "capabilities:       {:?}", self.capabilities)?;
        writeln!(f, "dial_info_detail_list:")?;
        for did in &self.dial_info_detail_list {
            writeln!(f, "    {}", did)?;
        }
        Ok(())
    }
}

impl NodeInfo {
    pub fn new(
        network_class: NetworkClass,
        outbound_protocols: ProtocolTypeSet,
        address_types: AddressTypeSet,
        envelope_support: Vec<u8>,
        crypto_support: Vec<CryptoKind>,
        capabilities: Vec<VeilidCapability>,
        dial_info_detail_list: Vec<DialInfoDetail>,
    ) -> Self {
        Self {
            network_class,
            outbound_protocols,
            address_types,
            envelope_support,
            crypto_support,
            capabilities,
            dial_info_detail_list,
        }
    }

    pub fn network_class(&self) -> NetworkClass {
        self.network_class
    }
    pub fn outbound_protocols(&self) -> ProtocolTypeSet {
        self.outbound_protocols
    }
    pub fn address_types(&self) -> AddressTypeSet {
        self.address_types
    }
    pub fn envelope_support(&self) -> &[u8] {
        &self.envelope_support
    }
    pub fn crypto_support(&self) -> &[CryptoKind] {
        &self.crypto_support
    }
    pub fn capabilities(&self) -> &[VeilidCapability] {
        &self.capabilities
    }
    pub fn dial_info_detail_list(&self) -> &[DialInfoDetail] {
        &self.dial_info_detail_list
    }

    pub fn first_filtered_dial_info_detail<'a, S, F>(
        &self,
        sort: Option<&'a S>,
        filter: &'a F,
    ) -> Option<DialInfoDetail>
    where
        S: Fn(&DialInfoDetail, &DialInfoDetail) -> std::cmp::Ordering,
        F: Fn(&DialInfoDetail) -> bool,
    {
        if let Some(sort) = sort {
            let mut dids = self.dial_info_detail_list.clone();
            dids.sort_by(sort);
            for did in dids {
                if filter(&did) {
                    return Some(did);
                }
            }
        } else {
            for did in &self.dial_info_detail_list {
                if filter(did) {
                    return Some(did.clone());
                }
            }
        };
        None
    }

    pub fn filtered_dial_info_details<S, F>(
        &self,
        sort: Option<&S>,
        filter: &F,
    ) -> Vec<DialInfoDetail>
    where
        S: Fn(&DialInfoDetail, &DialInfoDetail) -> std::cmp::Ordering,
        F: Fn(&DialInfoDetail) -> bool,
    {
        let mut dial_info_detail_list = Vec::new();

        if let Some(sort) = sort {
            let mut dids = self.dial_info_detail_list.clone();
            dids.sort_by(sort);
            for did in dids {
                if filter(&did) {
                    dial_info_detail_list.push(did);
                }
            }
        } else {
            for did in &self.dial_info_detail_list {
                if filter(did) {
                    dial_info_detail_list.push(did.clone());
                }
            }
        };
        dial_info_detail_list
    }

    /// Does this node has some dial info
    pub fn has_dial_info(&self) -> bool {
        !self.dial_info_detail_list.is_empty()
    }

    pub fn has_capability(&self, cap: VeilidCapability) -> bool {
        self.capabilities.contains(&cap)
    }
    pub fn has_all_capabilities(&self, capabilities: &[VeilidCapability]) -> bool {
        for cap in capabilities {
            if !self.has_capability(*cap) {
                return false;
            }
        }
        true
    }
    pub fn has_any_capabilities(&self, capabilities: &[VeilidCapability]) -> bool {
        if capabilities.is_empty() {
            return true;
        }
        for cap in capabilities {
            if self.has_capability(*cap) {
                return true;
            }
        }
        false
    }

    /// Can direct connections be made
    pub fn is_fully_direct_inbound(&self) -> bool {
        // Must be inbound capable
        if !matches!(self.network_class, NetworkClass::InboundCapable) {
            return false;
        }
        // Do any of our dial info require signalling? if so, we can't offer signalling
        for did in &self.dial_info_detail_list {
            if did.class.requires_signal() {
                return false;
            }
        }
        true
    }

    /// Does this appear on the same network within the routing domain?
    /// The notion of 'ipblock' is a single external IP address for ipv4, and a fixed prefix for ipv6.
    /// If a NAT is present, this detects if two public peerinfo would share the same router and be
    /// subject to hairpin NAT (for ipv4 typically). This is also overloaded for the concept
    /// of rate-limiting the number of nodes coming from the same ip 'block' within a specific amount of
    /// time for the address filter.
    pub fn node_is_on_same_ipblock(&self, node_b: &NodeInfo, ip6_prefix_size: usize) -> bool {
        let our_ip_blocks = self
            .dial_info_detail_list()
            .iter()
            .map(|did| ip_to_ipblock(ip6_prefix_size, did.dial_info.to_socket_addr().ip()))
            .collect::<HashSet<_>>();

        for did in node_b.dial_info_detail_list() {
            let ipblock = ip_to_ipblock(ip6_prefix_size, did.dial_info.to_socket_addr().ip());
            if our_ip_blocks.contains(&ipblock) {
                return true;
            }
        }
        false
    }
}
