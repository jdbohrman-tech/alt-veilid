use super::*;

#[derive(Debug)]
pub struct RoutingDomainDetailCommon {
    routing_domain: RoutingDomain,
    node_id: DHTKey,
    node_id_secret: DHTKeySecret,
    network_class: Option<NetworkClass>,
    outbound_protocols: ProtocolTypeSet,
    inbound_protocols: ProtocolTypeSet,
    address_types: AddressTypeSet,
    relay_node: Option<NodeRef>,
    dial_info_details: Vec<DialInfoDetail>,
    // caches
    cached_peer_info: Mutex<Option<PeerInfo>>,
}

impl RoutingDomainDetailCommon {
    pub fn new(routing_domain: RoutingDomain) -> Self {
        Self {
            routing_domain,
            node_id: Default::default(),
            node_id_secret: Default::default(),
            network_class: Default::default(),
            outbound_protocols: Default::default(),
            inbound_protocols: Default::default(),
            address_types: Default::default(),
            relay_node: Default::default(),
            dial_info_details: Default::default(),
            cached_peer_info: Mutex::new(Default::default()),
        }
    }

    // Set from routing table
    pub(super) fn setup_node(&mut self, node_id: DHTKey, node_id_secret: DHTKeySecret) {
        self.node_id = node_id;
        self.node_id_secret = node_id_secret;
        self.clear_cache();
    }
    // Set from network manager
    pub(super) fn setup_network(
        &mut self,
        outbound_protocols: ProtocolTypeSet,
        inbound_protocols: ProtocolTypeSet,
        address_types: AddressTypeSet,
    ) {
        self.outbound_protocols = outbound_protocols;
        self.inbound_protocols = inbound_protocols;
        self.address_types = address_types;
    }

    pub fn node_id(&self) -> DHTKey {
        self.node_id
    }
    pub fn node_id_secret(&self) -> DHTKeySecret {
        self.node_id_secret
    }
    pub(super) fn set_network_class(&mut self, network_class: Option<NetworkClass>) {
        self.network_class = network_class;
    }
    pub fn network_class(&self) -> Option<NetworkClass> {
        self.network_class
    }
    pub fn outbound_protocols(&self) -> ProtocolTypeSet {
        self.outbound_protocols
    }
    pub fn inbound_protocols(&self) -> ProtocolTypeSet {
        self.inbound_protocols
    }
    pub fn address_types(&self) -> AddressTypeSet {
        self.address_types
    }
    pub fn relay_node(&self) -> Option<NodeRef> {
        self.relay_node.clone()
    }
    pub(super) fn set_relay_node(&mut self, opt_relay_node: Option<NodeRef>) {
        self.relay_node = opt_relay_node.map(|nr| {
            nr.filtered_clone(NodeRefFilter::new().with_routing_domain(self.routing_domain))
        })
    }
    pub fn dial_info_details(&self) -> &Vec<DialInfoDetail> {
        &self.dial_info_details
    }
    pub(super) fn clear_dial_info_details(&mut self) {
        self.dial_info_details.clear();
    }
    pub(super) fn add_dial_info_detail(&mut self, did: DialInfoDetail) {
        self.dial_info_details.push(did);
        self.dial_info_details.sort();
    }

    pub fn has_valid_own_node_info(&self) -> bool {
        self.network_class.unwrap_or(NetworkClass::Invalid) != NetworkClass::Invalid
    }

    pub fn with_peer_info<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&PeerInfo) -> R,
    {
        let cpi = self.cached_peer_info.lock();
        if cpi.is_none() {
            // Regenerate peer info
            let pi = PeerInfo::new(
                NodeId::new(self.node_id),
                SignedNodeInfo::with_secret(
                    NodeInfo {
                        network_class: self.network_class.unwrap_or(NetworkClass::Invalid),
                        outbound_protocols: self.outbound_protocols,
                        address_types: self.address_types,
                        min_version: MIN_VERSION,
                        max_version: MAX_VERSION,
                        dial_info_detail_list: self.dial_info_details.clone(),
                        relay_peer_info: self
                            .relay_node
                            .and_then(|rn| rn.make_peer_info(self.routing_domain).map(Box::new)),
                    },
                    NodeId::new(self.node_id),
                    &self.node_id_secret,
                )
                .unwrap(),
            );
            // Cache the peer info
            *cpi = Some(pi);
        }
        f(cpi.as_ref().unwrap())
    }

    pub fn inbound_dial_info_filter(&self) -> DialInfoFilter {
        DialInfoFilter::all()
            .with_protocol_type_set(self.inbound_protocols)
            .with_address_type_set(self.address_types)
    }
    pub fn outbound_dial_info_filter(&self) -> DialInfoFilter {
        DialInfoFilter::all()
            .with_protocol_type_set(self.outbound_protocols)
            .with_address_type_set(self.address_types)
    }

    pub(super) fn clear_cache(&self) {
        *self.cached_peer_info.lock() = None;
    }
}

/// General trait for all routing domains
pub trait RoutingDomainDetail {
    // Common accessors
    fn common(&self) -> &RoutingDomainDetailCommon;
    fn common_mut(&mut self) -> &mut RoutingDomainDetailCommon;

    // Per-domain accessors
    fn can_contain_address(&self, address: Address) -> bool;
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// Public Internet routing domain internals
#[derive(Debug)]
pub struct PublicInternetRoutingDomainDetail {
    /// Common implementation for all routing domains
    common: RoutingDomainDetailCommon,
}

impl Default for PublicInternetRoutingDomainDetail {
    fn default() -> Self {
        Self {
            common: RoutingDomainDetailCommon::new(RoutingDomain::PublicInternet),
        }
    }
}

impl RoutingDomainDetail for PublicInternetRoutingDomainDetail {
    fn common(&self) -> &RoutingDomainDetailCommon {
        &self.common
    }
    fn common_mut(&mut self) -> &mut RoutingDomainDetailCommon {
        &mut self.common
    }
    fn can_contain_address(&self, address: Address) -> bool {
        address.is_global()
    }
}

/// Local Network routing domain internals
#[derive(Debug)]
pub struct LocalNetworkRoutingDomainDetail {
    /// The local networks this domain will communicate with
    local_networks: Vec<(IpAddr, IpAddr)>,
    /// Common implementation for all routing domains
    common: RoutingDomainDetailCommon,
}

impl Default for LocalNetworkRoutingDomainDetail {
    fn default() -> Self {
        Self {
            local_networks: Default::default(),
            common: RoutingDomainDetailCommon::new(RoutingDomain::LocalNetwork),
        }
    }
}

impl LocalNetworkRoutingDomainDetail {
    pub fn set_local_networks(&mut self, mut local_networks: Vec<(IpAddr, IpAddr)>) -> bool {
        local_networks.sort();
        if local_networks == self.local_networks {
            return false;
        }
        self.local_networks = local_networks;
        true
    }
}

impl RoutingDomainDetail for LocalNetworkRoutingDomainDetail {
    fn common(&self) -> &RoutingDomainDetailCommon {
        &self.common
    }
    fn common_mut(&mut self) -> &mut RoutingDomainDetailCommon {
        &mut self.common
    }
    fn can_contain_address(&self, address: Address) -> bool {
        let ip = address.to_ip_addr();
        for localnet in &self.local_networks {
            if ipaddr_in_network(ip, localnet.0, localnet.1) {
                return true;
            }
        }
        false
    }
}
