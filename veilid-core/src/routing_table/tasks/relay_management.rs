use super::*;

impl RoutingTable {
    // Check if a relay is desired or not
    fn public_internet_wants_relay(&self) -> Option<RelayKind> {
        let own_peer_info = self.get_own_peer_info(RoutingDomain::PublicInternet);
        let own_node_info = own_peer_info.signed_node_info().node_info();
        let network_class = own_node_info.network_class();

        // Never give a relay to something with an invalid network class
        if matches!(network_class, NetworkClass::Invalid) {
            return None;
        }

        // If we -need- a relay always request one
        if let Some(rk) = own_node_info.requires_relay() {
            return Some(rk);
        }

        // If we don't always need a relay, but we don't have support for
        // all the address types then we should request one anyway
        let mut address_types = AddressTypeSet::empty();
        for did in own_node_info.dial_info_detail_list() {
            address_types |= did.dial_info.address_type();
        }
        if address_types != AddressTypeSet::all() {
            return Some(RelayKind::Inbound);
        }

        // If we are behind some NAT, then we should get ourselves a relay just
        // in case we need to navigate hairpin NAT to our own network
        let mut inbound_addresses = HashSet::<SocketAddr>::new();
        for did in own_node_info.dial_info_detail_list() {
            inbound_addresses.insert(did.dial_info.to_socket_addr());
        }
        let own_local_peer_info = self.get_own_peer_info(RoutingDomain::LocalNetwork);
        let own_local_node_info = own_local_peer_info.signed_node_info().node_info();
        for ldid in own_local_node_info.dial_info_detail_list() {
            inbound_addresses.remove(&ldid.dial_info.to_socket_addr());
        }
        if !inbound_addresses.is_empty() {
            return Some(RelayKind::Inbound);
        }

        // No relay is desired
        None
    }

    // Keep relays assigned and accessible
    #[instrument(level = "trace", skip(self), err)]
    pub(crate) async fn relay_management_task_routine(
        self,
        _stop_token: StopToken,
        _last_ts: Timestamp,
        cur_ts: Timestamp,
    ) -> EyreResult<()> {
        let relay_node_filter = self.make_public_internet_relay_node_filter();
        let relay_desired = self.public_internet_wants_relay();

        // Get routing domain editor
        let mut editor = self.edit_routing_domain(RoutingDomain::PublicInternet);

        // If we already have a relay, see if it is dead, or if we don't need it any more
        let has_relay = {
            if let Some(relay_node) = self.relay_node(RoutingDomain::PublicInternet) {
                let state_reason = relay_node.state_reason(cur_ts);
                // Relay node is dead or no longer needed
                if matches!(
                    state_reason,
                    BucketEntryStateReason::Dead(_) | BucketEntryStateReason::Punished(_)
                ) {
                    log_rtab!(debug "Relay node is now {:?}, dropping relay {}", state_reason, relay_node);
                    editor.clear_relay_node();
                    false
                }
                // Relay node no longer can relay
                else if relay_node.operate(|_rti, e| !relay_node_filter(e)) {
                    log_rtab!(debug
                        "Relay node can no longer relay, dropping relay {}",
                        relay_node
                    );
                    editor.clear_relay_node();
                    false
                }
                // Relay node is no longer wanted
                else if relay_desired.is_none() {
                    log_rtab!(debug
                        "Relay node no longer desired, dropping relay {}",
                        relay_node
                    );
                    editor.clear_relay_node();
                    false
                } else {
                    true
                }
            } else {
                false
            }
        };

        // Do we want a relay?
        if !has_relay && relay_desired.is_some() {
            let relay_desired = relay_desired.unwrap();

            // Do we want an outbound relay?
            let mut got_outbound_relay = false;
            if matches!(relay_desired, RelayKind::Outbound) {
                // The outbound relay is the host of the PWA
                if let Some(outbound_relay_peerinfo) = intf::get_outbound_relay_peer().await {
                    // Register new outbound relay
                    match self.register_node_with_peer_info(
                        RoutingDomain::PublicInternet,
                        outbound_relay_peerinfo,
                        false,
                    ) {
                        Ok(nr) => {
                            log_rtab!(debug "Outbound relay node selected: {}", nr);
                            editor.set_relay_node(nr);
                            got_outbound_relay = true;
                        }
                        Err(e) => {
                            log_rtab!(error "failed to register node with peer info: {}", e);
                        }
                    }
                } else {
                    log_rtab!(debug "Outbound relay desired but not available");
                }
            }
            if !got_outbound_relay {
                // Find a node in our routing table that is an acceptable inbound relay
                if let Some(nr) = self.find_inbound_relay(
                    RoutingDomain::PublicInternet,
                    cur_ts,
                    relay_node_filter,
                ) {
                    log_rtab!(debug "Inbound relay node selected: {}", nr);
                    editor.set_relay_node(nr);
                }
            }
        }

        // Commit the changes
        editor.commit(false).await;

        Ok(())
    }

    pub fn make_public_internet_relay_node_filter(&self) -> impl Fn(&BucketEntryInner) -> bool {
        // Get all our outbound protocol/address types
        let outbound_dif = self.get_outbound_dial_info_filter(RoutingDomain::PublicInternet);
        let mapped_port_info = self.get_low_level_port_info();
        let own_node_info = self
            .get_own_peer_info(RoutingDomain::PublicInternet)
            .signed_node_info()
            .node_info()
            .clone();
        let ip6_prefix_size = self
            .unlocked_inner
            .config
            .get()
            .network
            .max_connections_per_ip6_prefix_size as usize;

        move |e: &BucketEntryInner| {
            // Ensure this node is not on the local network and is on the public internet
            if e.has_node_info(RoutingDomain::LocalNetwork.into()) {
                return false;
            }
            let Some(signed_node_info) = e.signed_node_info(RoutingDomain::PublicInternet) else {
                return false;
            };

            // Until we have a way of reducing a SignedRelayedNodeInfo to a SignedDirectNodeInfo
            // See https://gitlab.com/veilid/veilid/-/issues/381
            // We should consider nodes with allocated relays as disqualified from being a relay themselves
            // due to limitations in representing the PeerInfo for relays that also have relays.
            let node_info = match signed_node_info {
                SignedNodeInfo::Direct(d) => d.node_info(),
                SignedNodeInfo::Relayed(_) => {
                    return false;
                }
            };

            // Disqualify nodes that don't have relay capability or require a relay themselves
            if !(node_info.has_capability(CAP_RELAY) && node_info.is_fully_direct_inbound()) {
                // Needs to be able to accept packets to relay directly
                return false;
            }

            // Disqualify nodes that don't cover all our inbound ports for tcp and udp
            // as we need to be able to use the relay for keepalives for all nat mappings
            let mut low_level_protocol_ports = mapped_port_info.low_level_protocol_ports.clone();
            let dids = node_info.all_filtered_dial_info_details(DialInfoDetail::NO_SORT, |did| {
                did.matches_filter(&outbound_dif)
            });
            for did in &dids {
                let pt = did.dial_info.protocol_type();
                let at = did.dial_info.address_type();
                if let Some((llpt, port)) = mapped_port_info.protocol_to_port.get(&(pt, at)) {
                    low_level_protocol_ports.remove(&(*llpt, at, *port));
                }
            }
            if !low_level_protocol_ports.is_empty() {
                return false;
            }

            // For all protocol types we could connect to the relay by, ensure the relay supports all address types
            let mut address_type_mappings = HashMap::<ProtocolType, AddressTypeSet>::new();
            let dids = node_info.dial_info_detail_list();
            for did in dids {
                address_type_mappings
                    .entry(did.dial_info.protocol_type())
                    .and_modify(|x| {
                        x.insert(did.dial_info.address_type());
                    })
                    .or_insert_with(|| did.dial_info.address_type().into());
            }
            for pt in outbound_dif.protocol_type_set.iter() {
                if let Some(ats) = address_type_mappings.get(&pt) {
                    if *ats != AddressTypeSet::all() {
                        return false;
                    }
                }
            }

            // Exclude any nodes that have our same network block
            if own_node_info.node_is_on_same_ipblock(node_info, ip6_prefix_size) {
                return false;
            }

            true
        }
    }

    #[instrument(level = "trace", skip(self, relay_node_filter), ret)]
    pub fn find_inbound_relay(
        &self,
        routing_domain: RoutingDomain,
        cur_ts: Timestamp,
        relay_node_filter: impl Fn(&BucketEntryInner) -> bool,
    ) -> Option<NodeRef> {
        // Go through all entries and find fastest entry that matches filter function
        let inner = self.inner.read();
        let inner = &*inner;
        let mut best_inbound_relay: Option<Arc<BucketEntry>> = None;

        // Iterate all known nodes for candidates
        inner.with_entries(cur_ts, BucketEntryState::Unreliable, |rti, entry| {
            let entry2 = entry.clone();
            entry.with(rti, |rti, e| {
                // Filter this node
                if relay_node_filter(e) {
                    // Compare against previous candidate
                    if let Some(best_inbound_relay) = best_inbound_relay.as_mut() {
                        // Less is faster
                        let better = best_inbound_relay.with(rti, |_rti, best| {
                            // choose low latency stability for relays
                            BucketEntryInner::cmp_fastest_reliable(cur_ts, e, best)
                                == std::cmp::Ordering::Less
                        });
                        // Now apply filter function and see if this node should be included
                        if better {
                            *best_inbound_relay = entry2;
                        }
                    } else {
                        // Always store the first candidate
                        best_inbound_relay = Some(entry2);
                    }
                }
            });
            // Don't end early, iterate through all entries
            Option::<()>::None
        });
        // Return the best inbound relay noderef
        best_inbound_relay.map(|e| NodeRef::new(self.clone(), e, None))
    }
}
