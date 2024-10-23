/// Address checker - keep track of how other nodes are seeing our node's address on a per-protocol basis
/// Used to determine if our address has changed and if we should re-publish new PeerInfo
use super::*;

/// Number of 'existing dialinfo inconsistent' results in the cache during inbound-capable to trigger detection
pub const ADDRESS_INCONSISTENCY_DETECTION_COUNT: usize = 5;

/// Number of consistent results in the cache during outbound-only to trigger detection
pub const ADDRESS_CONSISTENCY_DETECTION_COUNT: usize = 5;

/// Length of consistent/inconsistent result cache for detection
pub const ADDRESS_CHECK_CACHE_SIZE: usize = 10;

/// Length of consistent/inconsistent result cache for detection
// pub const ADDRESS_CHECK_PEER_COUNT: usize = 256;
// /// Frequency of address checks
// pub const PUBLIC_ADDRESS_CHECK_TASK_INTERVAL_SECS: u32 = 60;
// /// Duration we leave nodes in the inconsistencies table
// pub const PUBLIC_ADDRESS_INCONSISTENCY_TIMEOUT_US: TimestampDuration =
//     TimestampDuration::new(300_000_000u64); // 5 minutes
// /// How long we punish nodes for lying about our address
// pub const PUBLIC_ADDRESS_INCONSISTENCY_PUNISHMENT_TIMEOUT_US: TimestampDuration =
//     TimestampDuration::new(3_600_000_000_u64); // 60 minutes

/// Address checker config
pub struct AddressCheckConfig {
    pub detect_address_changes: bool,
    pub ip6_prefix_size: usize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
struct AddressCheckCacheKey(RoutingDomain, ProtocolType, AddressType);

/// Address checker - keep track of how other nodes are seeing our node's address on a per-protocol basis
/// Used to determine if our address has changed and if we should re-publish new PeerInfo
pub struct AddressCheck {
    config: AddressCheckConfig,
    net: Network,
    current_network_class: BTreeMap<RoutingDomain, NetworkClass>,
    current_addresses: BTreeMap<AddressCheckCacheKey, HashSet<SocketAddress>>,
    // Used by InboundCapable to determine if we have changed our address or re-do our network class
    address_inconsistency_table: BTreeMap<AddressCheckCacheKey, usize>,
    // Used by OutboundOnly to determine if we should re-do our network class
    address_consistency_table: BTreeMap<AddressCheckCacheKey, LruCache<IpAddr, SocketAddress>>,
}

impl AddressCheck {
    pub fn new(config: AddressCheckConfig, net: Network) -> Self {
        Self {
            config,
            net,
            current_network_class: BTreeMap::new(),
            current_addresses: BTreeMap::new(),
            address_inconsistency_table: BTreeMap::new(),
            address_consistency_table: BTreeMap::new(),
        }
    }

    /// Accept a report of any peerinfo that has changed
    pub fn report_peer_info_change(&mut self, peer_info: Arc<PeerInfo>) {
        let routing_domain = peer_info.routing_domain();
        let network_class = peer_info.signed_node_info().node_info().network_class();

        self.current_network_class
            .insert(routing_domain, network_class);
        for protocol_type in ProtocolTypeSet::all() {
            for address_type in AddressTypeSet::all() {
                let acck = AddressCheckCacheKey(routing_domain, protocol_type, address_type);

                // Clear our current addresses so we can rebuild them for this routing domain
                self.current_addresses.remove(&acck);

                // Clear our history as well now so we start fresh when we get a new peer info
                self.address_inconsistency_table.remove(&acck);
                self.address_consistency_table.remove(&acck);
            }
        }

        for did in peer_info
            .signed_node_info()
            .node_info()
            .dial_info_detail_list()
        {
            // Strip port from direct and mapped addresses
            // as the incoming dialinfo may not match the outbound
            // connections' NAT mapping. In this case we only check for IP address changes.
            let socket_address =
                if did.class == DialInfoClass::Direct || did.class == DialInfoClass::Mapped {
                    did.dial_info.socket_address().with_port(0)
                } else {
                    did.dial_info.socket_address()
                };

            let address_type = did.dial_info.address_type();
            let protocol_type = did.dial_info.protocol_type();
            let acck = AddressCheckCacheKey(routing_domain, protocol_type, address_type);

            self.current_addresses
                .entry(acck)
                .or_default()
                .insert(socket_address);
        }
    }

    /// Accept a report of our address as seen by the other end of a flow, such
    /// as the StatusA response from a StatusQ
    pub fn report_socket_address_change(
        &mut self,
        routing_domain: RoutingDomain, // the routing domain used by this flow
        socket_address: SocketAddress, // the socket address as seen by the remote peer
        old_socket_address: Option<SocketAddress>, // the socket address previously for this peer
        flow: Flow,                    // the flow used
        reporting_peer: NodeRef,       // the peer's noderef reporting the socket address
    ) {
        // Don't accept any reports if we're already in the middle of a public dial info check
        if self.net.needs_public_dial_info_check() {
            return;
        }

        // Ignore the LocalNetwork routing domain because we know if our local addresses change
        // from our interfaces
        if matches!(routing_domain, RoutingDomain::LocalNetwork) {
            return;
        }

        // Ignore flows that do not start from our listening port (unbound connections etc),
        // because a router is going to map these differently
        let Some(pla) = self
            .net
            .get_preferred_local_address_by_key(flow.protocol_type(), flow.address_type())
        else {
            return;
        };
        let Some(local) = flow.local() else {
            return;
        };
        if local.port() != pla.port() {
            log_network_result!(debug "ignoring address report because local port did not match listener: {} != {}", local.port(), pla.port());
            return;
        }

        // Get the ip(block) this report is coming from
        let reporting_ipblock =
            ip_to_ipblock(self.config.ip6_prefix_size, flow.remote_address().ip_addr());

        // Reject public address reports from nodes that we know are behind symmetric nat or
        // nodes that must be using a relay for everything
        let Some(reporting_node_info) = reporting_peer.node_info(routing_domain) else {
            return;
        };
        if reporting_node_info.network_class() != NetworkClass::InboundCapable {
            return;
        }

        // If the socket address reported is the same as the reporter, then this is coming through a relay
        // or it should be ignored due to local proximity (nodes on the same network block should not be trusted as
        // public ip address reporters, only disinterested parties)
        if reporting_ipblock == ip_to_ipblock(self.config.ip6_prefix_size, socket_address.ip_addr())
        {
            return;
        }

        // Get current network class / dial info
        // If we haven't gotten our own network class yet we're done for now
        let Some(network_class) = self.current_network_class.get(&routing_domain) else {
            return;
        };

        // Process the state of the address checker and see if we need to
        // perform a full address check for this routing domain
        let needs_address_detection = match network_class {
            NetworkClass::InboundCapable => self.detect_for_inbound_capable(
                routing_domain,
                socket_address,
                old_socket_address,
                flow,
                reporting_peer,
            ),
            NetworkClass::OutboundOnly => self.detect_for_outbound_only(
                routing_domain,
                socket_address,
                flow,
                reporting_ipblock,
            ),
            NetworkClass::WebApp | NetworkClass::Invalid => {
                return;
            }
        };

        if needs_address_detection {
            if self.config.detect_address_changes {
                // Reset the address check cache now so we can start detecting fresh
                info!(
                    "{:?} address has changed, detecting dial info",
                    routing_domain
                );

                // Re-detect the public dialinfo
                self.net.set_needs_public_dial_info_check(None);
            } else {
                warn!(
                    "{:?} address may have changed. Restarting the server may be required.",
                    routing_domain
                );
            }
        }
    }

    fn matches_current_address(
        &self,
        acckey: AddressCheckCacheKey,
        socket_address: SocketAddress,
    ) -> bool {
        self.current_addresses
            .get(&acckey)
            .map(|current_addresses| {
                current_addresses.contains(&socket_address)
                    || current_addresses.contains(&socket_address.with_port(0))
            })
            .unwrap_or(false)
    }

    // If we are inbound capable, but start to see places where our sender info used to match our dial info
    // but no longer matches our dial info (count up the number of changes -away- from our dial info)
    // then trigger a detection of dial info and network class
    fn detect_for_inbound_capable(
        &mut self,
        routing_domain: RoutingDomain, // the routing domain used by this flow
        socket_address: SocketAddress, // the socket address as seen by the remote peer
        old_socket_address: Option<SocketAddress>, // the socket address previously for this peer
        flow: Flow,                    // the flow used
        reporting_peer: NodeRef,       // the peer's noderef reporting the socket address
    ) -> bool {
        let acckey =
            AddressCheckCacheKey(routing_domain, flow.protocol_type(), flow.address_type());

        // Check the current socket address and see if it matches our current dial info
        let new_matches_current = self.matches_current_address(acckey, socket_address);

        // If we have something that matches our current dial info at all, consider it a validation
        if new_matches_current {
            self.address_inconsistency_table
                .entry(acckey)
                .and_modify(|ait| {
                    if *ait != 0 {
                        log_net!(debug "Resetting address inconsistency for {:?} due to match on flow {:?} from {}", acckey, flow, reporting_peer);
                    }
                    *ait = 0;
                })
                .or_insert(0);
            return false;
        }

        // See if we have a case of switching away from our dial info
        let old_matches_current = old_socket_address
            .map(|osa| self.matches_current_address(acckey, osa))
            .unwrap_or(false);

        if old_matches_current {
            let val = *self
                .address_inconsistency_table
                .entry(acckey)
                .and_modify(|ait| {
                    *ait += 1;
                })
                .or_insert(1);
            log_net!(debug "Adding address inconsistency ({}) for {:?} due to address {} on flow {:?} from {}", val, acckey, socket_address, flow, reporting_peer);
            return val >= ADDRESS_INCONSISTENCY_DETECTION_COUNT;
        }

        false
    }

    // If we are currently outbound only, we don't have any public dial info
    // but if we are starting to see consistent socket address from multiple reporting peers
    // then we may be become inbound capable, so zap the network class so we can re-detect it and any public dial info
    // lru the addresses we're seeing and if they all match (same ip only?) then trigger
    fn detect_for_outbound_only(
        &mut self,
        routing_domain: RoutingDomain, // the routing domain used by this flow
        socket_address: SocketAddress, // the socket address as seen by the remote peer
        flow: Flow,                    // the flow used
        reporting_ipblock: IpAddr,     // the IP block this report came from
    ) -> bool {
        let acckey =
            AddressCheckCacheKey(routing_domain, flow.protocol_type(), flow.address_type());

        // Add the currently seen socket address into the consistency table
        let cache = self
            .address_consistency_table
            .entry(acckey)
            .and_modify(|act| {
                act.insert(reporting_ipblock, socket_address);
            })
            .or_insert_with(|| {
                let mut lruc = LruCache::new(ADDRESS_CHECK_CACHE_SIZE);
                lruc.insert(reporting_ipblock, socket_address);
                lruc
            });

        // If we have at least N consistencies then trigger a detect
        let mut consistencies = HashMap::<SocketAddress, usize>::new();
        for (_k, v) in cache.iter() {
            let count = *consistencies.entry(*v).and_modify(|e| *e += 1).or_insert(1);
            if count >= ADDRESS_CONSISTENCY_DETECTION_COUNT {
                log_net!(debug "Address consistency detected for {:?}: {}", acckey, v);
                return true;
            }
        }

        false
    }
}
