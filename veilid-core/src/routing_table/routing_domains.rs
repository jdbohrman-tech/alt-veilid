use super::*;

/// Mechanism required to contact another node
#[derive(Clone, Debug)]
pub(crate) enum ContactMethod {
    /// Node is not reachable by any means
    Unreachable,
    /// Connection should have already existed
    Existing,
    /// Contact the node directly
    Direct(DialInfo),
    /// Request via signal the node connect back directly (relay, target)
    SignalReverse(DHTKey, DHTKey),
    /// Request via signal the node negotiate a hole punch (relay, target_node)
    SignalHolePunch(DHTKey, DHTKey),
    /// Must use an inbound relay to reach the node
    InboundRelay(DHTKey),
    /// Must use outbound relay to reach the node
    OutboundRelay(DHTKey),
}

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

    /// Can this routing domain contain a particular address
    fn can_contain_address(&self, address: Address) -> bool;

    /// Get the contact method required for node A to reach node B in this routing domain
    /// Routing table must be locked for reading to use this function
    fn get_contact_method(
        &self,
        rti: &RoutingTableInner,
        node_a_id: &DHTKey,
        node_a: &NodeInfo,
        node_b_id: &DHTKey,
        node_b: &NodeInfo,
        dial_info_filter: DialInfoFilter,
        sequencing: Sequencing,
    ) -> ContactMethod;
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

fn first_filtered_dial_info_detail(
    from_node: &NodeInfo,
    to_node: &NodeInfo,
    dial_info_filter: &DialInfoFilter,
    sequencing: Sequencing,
) -> Option<DialInfoDetail> {
    let dial_info_filter = dial_info_filter.clone().filtered(
        &DialInfoFilter::all()
            .with_address_type_set(from_node.address_types)
            .with_protocol_type_set(from_node.outbound_protocols),
    );

    // Get first filtered dialinfo
    let (sort, dial_info_filter) = match sequencing {
        Sequencing::NoPreference => (None, dial_info_filter),
        Sequencing::PreferOrdered => (
            Some(DialInfoDetail::ordered_sequencing_sort),
            dial_info_filter,
        ),
        Sequencing::EnsureOrdered => (
            Some(DialInfoDetail::ordered_sequencing_sort),
            dial_info_filter.filtered(
                &DialInfoFilter::all().with_protocol_type_set(ProtocolType::all_ordered_set()),
            ),
        ),
    };
    // If the filter is dead then we won't be able to connect
    if dial_info_filter.is_dead() {
        return None;
    }

    let direct_filter = |did: &DialInfoDetail| did.matches_filter(&dial_info_filter);

    // Get the best match dial info for node B if we have it
    to_node.first_filtered_dial_info_detail(sort, direct_filter)
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
    fn get_contact_method(
        &self,
        _rti: &RoutingTableInner,
        node_a_id: &DHTKey,
        node_a: &NodeInfo,
        node_b_id: &DHTKey,
        node_b: &NodeInfo,
        dial_info_filter: DialInfoFilter,
        sequencing: Sequencing,
    ) -> ContactMethod {
        // Get the best match dial info for node B if we have it
        if let Some(target_did) =
            first_filtered_dial_info_detail(node_a, node_b, &dial_info_filter, sequencing)
        {
            // Do we need to signal before going inbound?
            if !target_did.class.requires_signal() {
                // Go direct without signaling
                return ContactMethod::Direct(target_did.dial_info);
            }

            // Get the target's inbound relay, it must have one or it is not reachable
            if let Some(inbound_relay) = node_b.relay_peer_info {
                // Note that relay_peer_info could be node_a, in which case a connection already exists
                // and we shouldn't have even gotten here
                if inbound_relay.node_id.key == *node_a_id {
                    return ContactMethod::Existing;
                }

                // Can node A reach the inbound relay directly?
                if first_filtered_dial_info_detail(
                    node_a,
                    &inbound_relay.signed_node_info.node_info,
                    &dial_info_filter,
                    sequencing,
                )
                .is_some()
                {
                    // Can node A receive anything inbound ever?
                    if matches!(node_a.network_class, NetworkClass::InboundCapable) {
                        ///////// Reverse connection

                        // Get the best match dial info for an reverse inbound connection from node B to node A
                        if let Some(reverse_did) = first_filtered_dial_info_detail(
                            node_b,
                            node_a,
                            &dial_info_filter,
                            sequencing,
                        ) {
                            // Ensure we aren't on the same public IP address (no hairpin nat)
                            if reverse_did.dial_info.to_ip_addr()
                                != target_did.dial_info.to_ip_addr()
                            {
                                // Can we receive a direct reverse connection?
                                if !reverse_did.class.requires_signal() {
                                    return ContactMethod::SignalReverse(
                                        inbound_relay.node_id.key,
                                        *node_b_id,
                                    );
                                }
                            }
                        }

                        ///////// UDP hole-punch

                        // Does node B have a direct udp dialinfo node A can reach?
                        let udp_dial_info_filter = dial_info_filter
                            .clone()
                            .filtered(&DialInfoFilter::all().with_protocol_type(ProtocolType::UDP));
                        if let Some(target_udp_did) = first_filtered_dial_info_detail(
                            node_a,
                            node_b,
                            &udp_dial_info_filter,
                            sequencing,
                        ) {
                            // Does node A have a direct udp dialinfo that node B can reach?
                            if let Some(reverse_udp_did) = first_filtered_dial_info_detail(
                                node_b,
                                node_a,
                                &udp_dial_info_filter,
                                sequencing,
                            ) {
                                // Ensure we aren't on the same public IP address (no hairpin nat)
                                if reverse_udp_did.dial_info.to_ip_addr()
                                    != target_udp_did.dial_info.to_ip_addr()
                                {
                                    // The target and ourselves have a udp dialinfo that they can reach
                                    return ContactMethod::SignalHolePunch(
                                        inbound_relay.node_id.key,
                                        *node_b_id,
                                    );
                                }
                            }
                        }
                        // Otherwise we have to inbound relay
                    }

                    return ContactMethod::InboundRelay(inbound_relay.node_id.key);
                }
            }
        }
        // If the node B has no direct dial info, it needs to have an inbound relay
        else if let Some(inbound_relay) = node_b.relay_peer_info {
            // Can we reach the full relay?
            if first_filtered_dial_info_detail(
                node_a,
                &inbound_relay.signed_node_info.node_info,
                &dial_info_filter,
                sequencing,
            )
            .is_some()
            {
                return ContactMethod::InboundRelay(inbound_relay.node_id.key);
            }
        }

        // If node A can't reach the node by other means, it may need to use its own relay
        if let Some(outbound_relay) = node_a.relay_peer_info {
            return ContactMethod::OutboundRelay(outbound_relay.node_id.key);
        }

        ContactMethod::Unreachable
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

    fn get_contact_method(
        &self,
        _rti: &RoutingTableInner,
        _node_a_id: &DHTKey,
        node_a: &NodeInfo,
        _node_b_id: &DHTKey,
        node_b: &NodeInfo,
        dial_info_filter: DialInfoFilter,
        sequencing: Sequencing,
    ) -> ContactMethod {
        // Scope the filter down to protocols node A can do outbound
        let dial_info_filter = dial_info_filter.filtered(
            &DialInfoFilter::all()
                .with_address_type_set(node_a.address_types)
                .with_protocol_type_set(node_a.outbound_protocols),
        );

        // Get first filtered dialinfo
        let (sort, dial_info_filter) = match sequencing {
            Sequencing::NoPreference => (None, dial_info_filter),
            Sequencing::PreferOrdered => (
                Some(DialInfoDetail::ordered_sequencing_sort),
                dial_info_filter,
            ),
            Sequencing::EnsureOrdered => (
                Some(DialInfoDetail::ordered_sequencing_sort),
                dial_info_filter.filtered(
                    &DialInfoFilter::all().with_protocol_type_set(ProtocolType::all_ordered_set()),
                ),
            ),
        };
        // If the filter is dead then we won't be able to connect
        if dial_info_filter.is_dead() {
            return ContactMethod::Unreachable;
        }

        let filter = |did: &DialInfoDetail| did.matches_filter(&dial_info_filter);

        let opt_target_did = node_b.first_filtered_dial_info_detail(sort, filter);
        if let Some(target_did) = opt_target_did {
            return ContactMethod::Direct(target_did.dial_info);
        }

        ContactMethod::Unreachable
    }
}
