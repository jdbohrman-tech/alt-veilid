mod editor;

pub use editor::*;

use super::*;

/// Public Internet routing domain internals
#[derive(Debug)]
pub struct PublicInternetRoutingDomainDetail {
    /// Common implementation for all routing domains
    common: RoutingDomainDetailCommon,
    /// Published peer info for this routing domain
    published_peer_info: Mutex<Option<Arc<PeerInfo>>>,
}

impl Default for PublicInternetRoutingDomainDetail {
    fn default() -> Self {
        Self {
            common: RoutingDomainDetailCommon::new(RoutingDomain::PublicInternet),
            published_peer_info: Default::default(),
        }
    }
}

impl RoutingDomainDetailCommonAccessors for PublicInternetRoutingDomainDetail {
    fn common(&self) -> &RoutingDomainDetailCommon {
        &self.common
    }
    fn common_mut(&mut self) -> &mut RoutingDomainDetailCommon {
        &mut self.common
    }
}

impl RoutingDomainDetail for PublicInternetRoutingDomainDetail {
    fn routing_domain(&self) -> RoutingDomain {
        RoutingDomain::PublicInternet
    }

    fn network_class(&self) -> Option<NetworkClass> {
        self.common.network_class()
    }
    fn outbound_protocols(&self) -> ProtocolTypeSet {
        self.common.outbound_protocols()
    }
    fn inbound_protocols(&self) -> ProtocolTypeSet {
        self.common.inbound_protocols()
    }
    fn address_types(&self) -> AddressTypeSet {
        self.common.address_types()
    }
    fn capabilities(&self) -> Vec<Capability> {
        self.common.capabilities()
    }
    fn requires_relay(&self) -> Option<RelayKind> {
        self.common.requires_relay()
    }
    fn relay_node(&self) -> Option<FilteredNodeRef> {
        self.common.relay_node()
    }
    fn relay_node_last_keepalive(&self) -> Option<Timestamp> {
        self.common.relay_node_last_keepalive()
    }
    fn dial_info_details(&self) -> &Vec<DialInfoDetail> {
        self.common.dial_info_details()
    }

    fn inbound_dial_info_filter(&self) -> DialInfoFilter {
        self.common.inbound_dial_info_filter()
    }
    fn outbound_dial_info_filter(&self) -> DialInfoFilter {
        self.common.outbound_dial_info_filter()
    }

    fn get_peer_info(&self, rti: &RoutingTableInner) -> Arc<PeerInfo> {
        self.common.get_peer_info(rti)
    }

    fn get_published_peer_info(&self) -> Option<Arc<PeerInfo>> {
        (*self.published_peer_info.lock()).clone()
    }

    ////////////////////////////////////////////////

    fn can_contain_address(&self, address: Address) -> bool {
        address.is_global()
    }

    fn refresh(&self) {
        self.common.clear_cache();
    }

    fn publish_peer_info(&self, rti: &RoutingTableInner) -> bool {
        let peer_info = {
            let pi = self.get_peer_info(rti);

            // If the network class is not yet determined, don't publish
            if pi.signed_node_info().node_info().network_class() == NetworkClass::Invalid {
                log_rtab!(debug "[PublicInternet] Not publishing peer info with invalid network class");
                return false;
            }

            // If we need a relay and we don't have one, don't publish yet
            if let Some(_relay_kind) = self.requires_relay() {
                if pi.signed_node_info().relay_ids().is_empty() {
                    log_rtab!(debug "[PublicInternet] Not publishing peer info that wants relay until we have a relay");
                    return false;
                }
            }

            // Don't publish if the peer info hasnt changed from our previous publication
            let mut ppi_lock = self.published_peer_info.lock();
            if let Some(old_peer_info) = &*ppi_lock {
                if pi.equivalent(old_peer_info) {
                    log_rtab!(debug "[PublicInternet] Not publishing peer info because it is equivalent");
                    return false;
                }
            }

            log_rtab!(debug "[PublicInternet] Published new peer info: {}", pi);
            *ppi_lock = Some(pi.clone());

            pi
        };

        rti.unlocked_inner
            .network_manager()
            .report_peer_info_change(peer_info);

        true
    }

    fn unpublish_peer_info(&self) {
        let mut ppi_lock = self.published_peer_info.lock();
        log_rtab!(debug "[PublicInternet] Unpublished peer info");
        *ppi_lock = None;
    }

    fn ensure_dial_info_is_valid(&self, dial_info: &DialInfo) -> bool {
        let address = dial_info.socket_address().address();
        let can_contain_address = self.can_contain_address(address);

        if !can_contain_address {
            log_rtab!(debug "[PublicInternet] can not add dial info to this routing domain: {:?}", dial_info);
            return false;
        }
        if !dial_info.is_valid() {
            log_rtab!(debug
                "shouldn't be registering invalid addresses: {:?}",
                dial_info
            );
            return false;
        }
        true
    }

    fn get_contact_method(
        &self,
        rti: &RoutingTableInner,
        peer_a: Arc<PeerInfo>,
        peer_b: Arc<PeerInfo>,
        dial_info_filter: DialInfoFilter,
        sequencing: Sequencing,
        dif_sort: Option<Arc<DialInfoDetailSort>>,
    ) -> ContactMethod {
        let ip6_prefix_size = rti
            .unlocked_inner
            .config
            .get()
            .network
            .max_connections_per_ip6_prefix_size as usize;

        // Get the nodeinfos for convenience
        let node_a = peer_a.signed_node_info().node_info();
        let node_b = peer_b.signed_node_info().node_info();

        // Check to see if these nodes are on the same network
        let same_ipblock = node_a.node_is_on_same_ipblock(node_b, ip6_prefix_size);

        // Get the node ids that would be used between these peers
        let cck = common_crypto_kinds(&peer_a.node_ids().kinds(), &peer_b.node_ids().kinds());
        let Some(best_ck) = cck.first().copied() else {
            // No common crypto kinds between these nodes, can't contact
            return ContactMethod::Unreachable;
        };

        //let node_a_id = peer_a.node_ids().get(best_ck).unwrap();
        let node_b_id = peer_b.node_ids().get(best_ck).unwrap();

        // Get the best match dial info for node B if we have it
        // Don't try direct inbound at all if the two nodes are on the same ipblock to avoid hairpin NAT issues
        // as well avoiding direct traffic between same-network nodes. This would be done in the LocalNetwork RoutingDomain.
        if let Some(target_did) = (!same_ipblock)
            .then(|| {
                first_filtered_dial_info_detail_between_nodes(
                    node_a,
                    node_b,
                    &dial_info_filter,
                    sequencing,
                    dif_sort.clone(),
                )
            })
            .flatten()
        {
            // Do we need to signal before going inbound?
            if !target_did.class.requires_signal() {
                // Go direct without signaling
                return ContactMethod::Direct(target_did.dial_info);
            }

            // Get the target's inbound relay, it must have one or it is not reachable
            if let Some(node_b_relay) = peer_b.signed_node_info().relay_info() {
                // Note that relay_peer_info could be node_a, in which case a connection already exists
                // and we only get here if the connection had dropped, in which case node_a is unreachable until
                // it gets a new relay connection up
                if peer_b
                    .signed_node_info()
                    .relay_ids()
                    .contains_any(peer_a.node_ids())
                {
                    return ContactMethod::Existing;
                }

                // Get best node id to contact relay with
                let Some(node_b_relay_id) = peer_b.signed_node_info().relay_ids().get(best_ck)
                else {
                    // No best relay id
                    return ContactMethod::Unreachable;
                };

                // Can node A reach the inbound relay directly?
                if first_filtered_dial_info_detail_between_nodes(
                    node_a,
                    node_b_relay,
                    &dial_info_filter,
                    sequencing,
                    dif_sort.clone(),
                )
                .is_some()
                {
                    // Can node A receive anything inbound ever?
                    if matches!(node_a.network_class(), NetworkClass::InboundCapable) {
                        ///////// Reverse connection

                        // Get the best match dial info for an reverse inbound connection from node B to node A
                        if let Some(reverse_did) = first_filtered_dial_info_detail_between_nodes(
                            node_b,
                            node_a,
                            &dial_info_filter,
                            sequencing,
                            dif_sort.clone(),
                        ) {
                            // Ensure we aren't on the same public IP address (no hairpin nat)
                            if reverse_did.dial_info.ip_addr() != target_did.dial_info.ip_addr() {
                                // Can we receive a direct reverse connection?
                                if !reverse_did.class.requires_signal() {
                                    return ContactMethod::SignalReverse(
                                        node_b_relay_id,
                                        node_b_id,
                                    );
                                }
                            }
                        }

                        ///////// UDP hole-punch

                        // Does node B have a direct udp dialinfo node A can reach?
                        let udp_dial_info_filter = dial_info_filter
                            .filtered(&DialInfoFilter::all().with_protocol_type(ProtocolType::UDP));
                        if let Some(target_udp_did) = first_filtered_dial_info_detail_between_nodes(
                            node_a,
                            node_b,
                            &udp_dial_info_filter,
                            sequencing,
                            dif_sort.clone(),
                        ) {
                            // Does node A have a direct udp dialinfo that node B can reach?
                            if let Some(reverse_udp_did) =
                                first_filtered_dial_info_detail_between_nodes(
                                    node_b,
                                    node_a,
                                    &udp_dial_info_filter,
                                    sequencing,
                                    dif_sort.clone(),
                                )
                            {
                                // Ensure we aren't on the same public IP address (no hairpin nat)
                                if reverse_udp_did.dial_info.ip_addr()
                                    != target_udp_did.dial_info.ip_addr()
                                {
                                    // The target and ourselves have a udp dialinfo that they can reach
                                    return ContactMethod::SignalHolePunch(
                                        node_b_relay_id,
                                        node_b_id,
                                    );
                                }
                            }
                        }
                        // Otherwise we have to inbound relay
                    }

                    return ContactMethod::InboundRelay(node_b_relay_id);
                }
            }
        }
        // If the node B has no direct dial info or is on the same ipblock, it needs to have an inbound relay
        else if let Some(node_b_relay) = peer_b.signed_node_info().relay_info() {
            // Note that relay_peer_info could be node_a, in which case a connection already exists
            // and we only get here if the connection had dropped, in which case node_b is unreachable until
            // it gets a new relay connection up
            if peer_b
                .signed_node_info()
                .relay_ids()
                .contains_any(peer_a.node_ids())
            {
                return ContactMethod::Existing;
            }

            // Get best node id to contact relay with
            let Some(node_b_relay_id) = peer_b.signed_node_info().relay_ids().get(best_ck) else {
                // No best relay id
                return ContactMethod::Unreachable;
            };

            // Can we reach the inbound relay?
            if first_filtered_dial_info_detail_between_nodes(
                node_a,
                node_b_relay,
                &dial_info_filter,
                sequencing,
                dif_sort.clone(),
            )
            .is_some()
            {
                ///////// Reverse connection

                // Get the best match dial info for an reverse inbound connection from node B to node A
                // unless both nodes are on the same ipblock
                if let Some(reverse_did) = (!same_ipblock)
                    .then(|| {
                        first_filtered_dial_info_detail_between_nodes(
                            node_b,
                            node_a,
                            &dial_info_filter,
                            sequencing,
                            dif_sort.clone(),
                        )
                    })
                    .flatten()
                {
                    // Can we receive a direct reverse connection?
                    if !reverse_did.class.requires_signal() {
                        return ContactMethod::SignalReverse(node_b_relay_id, node_b_id);
                    }
                }

                return ContactMethod::InboundRelay(node_b_relay_id);
            }
        }

        // If node A can't reach the node by other means, it may need to use its outbound relay
        if peer_a
            .signed_node_info()
            .node_info()
            .network_class()
            .outbound_wants_relay()
        {
            if let Some(node_a_relay_id) = peer_a.signed_node_info().relay_ids().get(best_ck) {
                // Ensure it's not our relay we're trying to reach
                if node_a_relay_id != node_b_id {
                    return ContactMethod::OutboundRelay(node_a_relay_id);
                }
            }
        }

        ContactMethod::Unreachable
    }

    fn set_relay_node_last_keepalive(&mut self, ts: Option<Timestamp>) {
        self.common.set_relay_node_last_keepalive(ts);
    }
}
