mod editor;
mod local_network;
mod public_internet;

use super::*;

pub use editor::*;
pub use local_network::*;
pub use public_internet::*;

/// General trait for all routing domains
pub(crate) trait RoutingDomainDetail {
    // Common accessors
    #[expect(dead_code)]
    fn routing_domain(&self) -> RoutingDomain;
    fn network_class(&self) -> Option<NetworkClass>;
    fn outbound_protocols(&self) -> ProtocolTypeSet;
    fn inbound_protocols(&self) -> ProtocolTypeSet;
    fn address_types(&self) -> AddressTypeSet;
    fn capabilities(&self) -> Vec<Capability>;
    fn relay_node(&self) -> Option<FilteredNodeRef>;
    fn relay_node_last_keepalive(&self) -> Option<Timestamp>;
    fn dial_info_details(&self) -> &Vec<DialInfoDetail>;
    fn has_valid_network_class(&self) -> bool;
    fn get_published_peer_info(&self) -> Option<Arc<PeerInfo>>;
    fn inbound_dial_info_filter(&self) -> DialInfoFilter;
    fn outbound_dial_info_filter(&self) -> DialInfoFilter;
    fn get_peer_info(&self, rti: &RoutingTableInner) -> Arc<PeerInfo>;

    /// Can this routing domain contain a particular address
    fn can_contain_address(&self, address: Address) -> bool;
    fn ensure_dial_info_is_valid(&self, dial_info: &DialInfo) -> bool;

    /// Refresh caches if external data changes
    fn refresh(&self);

    /// Publish current peer info to the world
    fn publish_peer_info(&self, rti: &RoutingTableInner) -> bool;
    fn unpublish_peer_info(&self);

    /// Get the contact method required for node A to reach node B in this routing domain
    /// Routing table must be locked for reading to use this function
    fn get_contact_method(
        &self,
        rti: &RoutingTableInner,
        peer_a: Arc<PeerInfo>,
        peer_b: Arc<PeerInfo>,
        dial_info_filter: DialInfoFilter,
        sequencing: Sequencing,
        dif_sort: Option<Arc<DialInfoDetailSort>>,
    ) -> ContactMethod;
}

trait RoutingDomainDetailCommonAccessors: RoutingDomainDetail {
    #[expect(dead_code)]
    fn common(&self) -> &RoutingDomainDetailCommon;
    fn common_mut(&mut self) -> &mut RoutingDomainDetailCommon;
}

fn first_filtered_dial_info_detail_between_nodes(
    from_node: &NodeInfo,
    to_node: &NodeInfo,
    dial_info_filter: &DialInfoFilter,
    sequencing: Sequencing,
    dif_sort: Option<Arc<DialInfoDetailSort>>,
) -> Option<DialInfoDetail> {
    // Consider outbound capabilities
    let dial_info_filter = (*dial_info_filter).filtered(
        &DialInfoFilter::all()
            .with_address_type_set(from_node.address_types())
            .with_protocol_type_set(from_node.outbound_protocols()),
    );

    // Apply sequencing and get sort
    // Include sorting by external dial info sort for rotating through dialinfo
    // based on an external preference table, for example the one kept by
    // AddressFilter to deprioritize dialinfo that have recently failed to connect
    let (ordered, dial_info_filter) = dial_info_filter.apply_sequencing(sequencing);
    let sort: Option<Box<DialInfoDetailSort>> = if ordered {
        if let Some(dif_sort) = dif_sort {
            Some(Box::new(move |a, b| {
                let mut ord = dif_sort(a, b);
                if ord == core::cmp::Ordering::Equal {
                    ord = DialInfoDetail::ordered_sequencing_sort(a, b);
                }
                ord
            }))
        } else {
            Some(Box::new(move |a, b| {
                DialInfoDetail::ordered_sequencing_sort(a, b)
            }))
        }
    } else if let Some(dif_sort) = dif_sort {
        Some(Box::new(move |a, b| dif_sort(a, b)))
    } else {
        None
    };

    // If the filter is dead then we won't be able to connect
    if dial_info_filter.is_dead() {
        return None;
    }

    // Get the best match dial info for node B if we have it
    let direct_filter = |did: &DialInfoDetail| did.matches_filter(&dial_info_filter);
    to_node.first_filtered_dial_info_detail(sort, direct_filter)
}

#[derive(Debug)]
struct RoutingDomainDetailCommon {
    routing_domain: RoutingDomain,
    network_class: Option<NetworkClass>,
    outbound_protocols: ProtocolTypeSet,
    inbound_protocols: ProtocolTypeSet,
    address_types: AddressTypeSet,
    relay_node: Option<NodeRef>,
    relay_node_last_keepalive: Option<Timestamp>,
    capabilities: Vec<Capability>,
    dial_info_details: Vec<DialInfoDetail>,
    // caches
    cached_peer_info: Mutex<Option<Arc<PeerInfo>>>,
}

impl RoutingDomainDetailCommon {
    pub fn new(routing_domain: RoutingDomain) -> Self {
        Self {
            routing_domain,
            network_class: Default::default(),
            outbound_protocols: Default::default(),
            inbound_protocols: Default::default(),
            address_types: Default::default(),
            relay_node: Default::default(),
            relay_node_last_keepalive: Default::default(),
            capabilities: Default::default(),
            dial_info_details: Default::default(),
            cached_peer_info: Mutex::new(Default::default()),
        }
    }

    ///////////////////////////////////////////////////////////////////////
    // Accessors

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

    pub fn capabilities(&self) -> Vec<Capability> {
        self.capabilities.clone()
    }

    pub fn relay_node(&self) -> Option<FilteredNodeRef> {
        self.relay_node.as_ref().map(|nr| {
            nr.custom_filtered(NodeRefFilter::new().with_routing_domain(self.routing_domain))
        })
    }

    pub fn relay_node_last_keepalive(&self) -> Option<Timestamp> {
        self.relay_node_last_keepalive
    }

    pub fn dial_info_details(&self) -> &Vec<DialInfoDetail> {
        &self.dial_info_details
    }

    pub fn has_valid_network_class(&self) -> bool {
        self.network_class.unwrap_or(NetworkClass::Invalid) != NetworkClass::Invalid
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

    pub fn get_peer_info(&self, rti: &RoutingTableInner) -> Arc<PeerInfo> {
        let mut cpi = self.cached_peer_info.lock();
        if cpi.is_none() {
            // Regenerate peer info
            let pi = self.make_peer_info(rti);

            // Cache the peer info
            *cpi = Some(Arc::new(pi));
        }
        cpi.as_ref().unwrap().clone()
    }

    ///////////////////////////////////////////////////////////////////////
    // Mutators

    fn setup_network(
        &mut self,
        outbound_protocols: ProtocolTypeSet,
        inbound_protocols: ProtocolTypeSet,
        address_types: AddressTypeSet,
        capabilities: Vec<Capability>,
    ) {
        self.outbound_protocols = outbound_protocols;
        self.inbound_protocols = inbound_protocols;
        self.address_types = address_types;
        self.capabilities = capabilities;
        self.clear_cache();
    }

    fn set_network_class(&mut self, network_class: Option<NetworkClass>) {
        self.network_class = network_class;
        self.clear_cache();
    }

    fn set_relay_node(&mut self, opt_relay_node: Option<NodeRef>) {
        self.relay_node = opt_relay_node;
        self.relay_node_last_keepalive = None;
        self.clear_cache();
    }
    fn set_relay_node_last_keepalive(&mut self, ts: Option<Timestamp>) {
        self.relay_node_last_keepalive = ts;
    }

    fn clear_dial_info_details(
        &mut self,
        address_type: Option<AddressType>,
        protocol_type: Option<ProtocolType>,
    ) {
        self.dial_info_details.retain_mut(|e| {
            let mut remove = true;
            if let Some(pt) = protocol_type {
                if pt != e.dial_info.protocol_type() {
                    remove = false;
                }
            }
            if let Some(at) = address_type {
                if at != e.dial_info.address_type() {
                    remove = false;
                }
            }
            !remove
        });
        self.clear_cache();
    }
    fn add_dial_info_detail(&mut self, did: DialInfoDetail) {
        self.dial_info_details.push(did);
        self.dial_info_details.sort();
        self.dial_info_details.dedup();
        self.clear_cache();
    }
    // fn remove_dial_info_detail(&mut self, did: DialInfoDetail) {
    //     if let Some(index) = self.dial_info_details.iter().position(|x| *x == did) {
    //         self.dial_info_details.remove(index);
    //     }
    //     self.clear_cache();
    // }

    //////////////////////////////////////////////////////////////////////////////
    // Internal functions

    fn make_peer_info(&self, rti: &RoutingTableInner) -> PeerInfo {
        let node_info = NodeInfo::new(
            self.network_class.unwrap_or(NetworkClass::Invalid),
            self.outbound_protocols,
            self.address_types,
            VALID_ENVELOPE_VERSIONS.to_vec(),
            VALID_CRYPTO_KINDS.to_vec(),
            self.capabilities.clone(),
            self.dial_info_details.clone(),
        );

        let relay_info = if let Some(rn) = &self.relay_node {
            let opt_relay_pi = rn.locked(rti).make_peer_info(self.routing_domain);
            if let Some(relay_pi) = opt_relay_pi {
                let (_routing_domain, relay_ids, relay_sni) = relay_pi.destructure();
                match relay_sni {
                    SignedNodeInfo::Direct(d) => Some((relay_ids, d)),
                    SignedNodeInfo::Relayed(_) => {
                        warn!("relay node should not have a relay itself! if this happens, a relay updated its signed node info and became a relay, which should cause the relay to be dropped");
                        None
                    }
                }
            } else {
                None
            }
        } else {
            None
        };

        let signed_node_info = match relay_info {
            Some((relay_ids, relay_sdni)) => SignedNodeInfo::Relayed(
                SignedRelayedNodeInfo::make_signatures(
                    rti.unlocked_inner.crypto(),
                    rti.unlocked_inner.node_id_typed_key_pairs(),
                    node_info,
                    relay_ids,
                    relay_sdni,
                )
                .unwrap(),
            ),
            None => SignedNodeInfo::Direct(
                SignedDirectNodeInfo::make_signatures(
                    rti.unlocked_inner.crypto(),
                    rti.unlocked_inner.node_id_typed_key_pairs(),
                    node_info,
                )
                .unwrap(),
            ),
        };

        PeerInfo::new(
            self.routing_domain,
            rti.unlocked_inner.node_ids(),
            signed_node_info,
        )
    }

    fn clear_cache(&self) {
        *self.cached_peer_info.lock() = None;
    }
}
