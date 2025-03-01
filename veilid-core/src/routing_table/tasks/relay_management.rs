use super::*;

impl_veilid_log_facility!("rtab");

impl RoutingTable {
    // Check if a relay is desired or not
    #[instrument(level = "trace", skip_all)]
    fn public_internet_wants_relay(&self) -> Option<RelayKind> {
        let own_peer_info = self.get_current_peer_info(RoutingDomain::PublicInternet);
        let own_node_info = own_peer_info.signed_node_info().node_info();
        let network_class = own_node_info.network_class();

        // Never give a relay to something with an invalid network class
        if matches!(network_class, NetworkClass::Invalid) {
            return None;
        }

        // If we -need- a relay always request one
        let requires_relay = self
            .inner
            .read()
            .with_routing_domain(RoutingDomain::PublicInternet, |rdd| rdd.requires_relay());
        if let Some(rk) = requires_relay {
            return Some(rk);
        }

        // If we are behind some NAT, then we should get ourselves a relay just
        // in case we need to navigate hairpin NAT to our own network
        let mut inbound_addresses = HashSet::<SocketAddr>::new();
        for did in own_node_info.dial_info_detail_list() {
            inbound_addresses.insert(did.dial_info.to_socket_addr());
        }
        let own_local_peer_info = self.get_current_peer_info(RoutingDomain::LocalNetwork);
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
    #[instrument(level = "trace", skip_all, err)]
    pub async fn relay_management_task_routine(
        &self,
        _stop_token: StopToken,
        _last_ts: Timestamp,
        cur_ts: Timestamp,
    ) -> EyreResult<()> {
        let relay_node_filter = self.make_public_internet_relay_node_filter();
        let relay_desired = self.public_internet_wants_relay();

        // Get routing domain editor
        let mut editor = self.edit_public_internet_routing_domain();

        // If we already have a relay, see if it is dead, or if we don't need it any more
        let has_relay = {
            if let Some(relay_node) = self.relay_node(RoutingDomain::PublicInternet) {
                let state_reason = relay_node.state_reason(cur_ts);
                // Relay node is dead or no longer needed
                if matches!(
                    state_reason,
                    BucketEntryStateReason::Dead(_) | BucketEntryStateReason::Punished(_)
                ) {
                    veilid_log!(self debug "Relay node is now {:?}, dropping relay {}", state_reason, relay_node);
                    editor.set_relay_node(None);
                    false
                }
                // Relay node no longer can relay
                else if relay_node.operate(|_rti, e| !&relay_node_filter(e)) {
                    veilid_log!(self debug
                        "Relay node can no longer relay, dropping relay {}",
                        relay_node
                    );
                    editor.set_relay_node(None);
                    false
                }
                // Relay node is no longer wanted
                else if relay_desired.is_none() {
                    veilid_log!(self debug
                        "Relay node no longer desired, dropping relay {}",
                        relay_node
                    );
                    editor.set_relay_node(None);
                    false
                } else {
                    // See if our relay was optimized last long enough ago to consider getting a new one
                    // if it is no longer fast enough
                    let mut has_relay = true;
                    let mut inner = self.inner.upgradable_read();
                    if let Some(last_optimized) =
                        inner.relay_node_last_optimized(RoutingDomain::PublicInternet)
                    {
                        let last_optimized_duration = cur_ts - last_optimized;
                        if last_optimized_duration
                            > TimestampDuration::new_secs(RELAY_OPTIMIZATION_INTERVAL_SECS)
                        {
                            // See what our relay's current percentile is
                            let relay_node_id = relay_node.best_node_id();
                            if let Some(relay_relative_performance) = inner
                                .get_node_relative_performance(
                                    relay_node_id,
                                    cur_ts,
                                    &relay_node_filter,
                                    |ls| ls.tm90,
                                )
                            {
                                // Get latency numbers
                                let latency_stats =
                                    if let Some(latency) = relay_node.peer_stats().latency {
                                        latency.to_string()
                                    } else {
                                        "[no stats]".to_owned()
                                    };

                                // Get current relay reliability
                                let state_reason = relay_node.state_reason(cur_ts);

                                if relay_relative_performance.percentile
                                    < RELAY_OPTIMIZATION_PERCENTILE
                                {
                                    // Drop the current relay so we can get the best new one
                                    veilid_log!(self debug
                                        "Relay tm90 is ({:.2}% < {:.2}%) ({} out of {}) (latency {}, {:?}) optimizing relay {}",
                                        relay_relative_performance.percentile,
                                        RELAY_OPTIMIZATION_PERCENTILE,
                                        relay_relative_performance.node_index,
                                        relay_relative_performance.node_count,
                                        latency_stats,
                                        state_reason,
                                        relay_node
                                    );
                                    editor.set_relay_node(None);
                                    has_relay = false;
                                } else {
                                    // Note that we tried to optimize the relay but it was good
                                    veilid_log!(self debug
                                        "Relay tm90 is ({:.2}% >= {:.2}%) ({} out of {}) (latency {}, {:?}) keeping {}",
                                        relay_relative_performance.percentile,
                                        RELAY_OPTIMIZATION_PERCENTILE,
                                        relay_relative_performance.node_index,
                                        relay_relative_performance.node_count,
                                        latency_stats,
                                        state_reason,
                                        relay_node
                                    );
                                    inner.with_upgraded(|inner| {
                                        inner.set_relay_node_last_optimized(
                                            RoutingDomain::PublicInternet,
                                            cur_ts,
                                        )
                                    });
                                }
                            } else {
                                // Drop the current relay because it could not be measured
                                veilid_log!(self debug
                                    "Relay relative performance not found {}",
                                    relay_node
                                );
                                editor.set_relay_node(None);
                                has_relay = false;
                            }
                        }
                    }

                    has_relay
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
                if let Some(outbound_relay_peerinfo) =
                    intf::get_outbound_relay_peer(RoutingDomain::PublicInternet).await
                {
                    // Register new outbound relay
                    match self.register_node_with_peer_info(outbound_relay_peerinfo, false) {
                        Ok(nr) => {
                            veilid_log!(self debug "Outbound relay node selected: {}", nr);
                            editor.set_relay_node(Some(nr.unfiltered()));
                            got_outbound_relay = true;
                        }
                        Err(e) => {
                            veilid_log!(self error "failed to register node with peer info: {}", e);
                        }
                    }
                } else {
                    veilid_log!(self debug "Outbound relay desired but not available");
                }
            }
            if !got_outbound_relay {
                // Find a node in our routing table that is an acceptable inbound relay
                if let Some(nr) = self.find_random_fast_node(
                    cur_ts,
                    &relay_node_filter,
                    RELAY_SELECTION_PERCENTILE,
                    |ls| ls.tm90,
                ) {
                    veilid_log!(self debug "Inbound relay node selected: {}", nr);
                    editor.set_relay_node(Some(nr));
                }
            }
        }

        // Commit the changes
        if editor.commit(false).await {
            // Try to publish the peer info
            editor.publish();
        }

        Ok(())
    }

    #[instrument(level = "trace", skip_all)]
    pub fn make_public_internet_relay_node_filter(&self) -> impl Fn(&BucketEntryInner) -> bool {
        // Get all our outbound protocol/address types
        let outbound_dif = self.get_outbound_dial_info_filter(RoutingDomain::PublicInternet);
        let mapped_port_info = self.get_low_level_port_info();
        let own_node_info = self
            .get_current_peer_info(RoutingDomain::PublicInternet)
            .signed_node_info()
            .node_info()
            .clone();
        let ip6_prefix_size = self
            .config()
            .with(|c| c.network.max_connections_per_ip6_prefix_size as usize);

        move |e: &BucketEntryInner| {
            // Ensure this node is not on the local network and is on the public internet
            if e.has_node_info(RoutingDomain::LocalNetwork.into()) {
                return false;
            }
            let Some(signed_node_info) = e.signed_node_info(RoutingDomain::PublicInternet) else {
                return false;
            };

            // Exclude any nodes that have 'failed to send' state indicating a
            // connection drop or inability to reach the node
            if e.peer_stats().rpc_stats.failed_to_send > 0 {
                return false;
            }

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
            let dids = node_info.filtered_dial_info_details(DialInfoDetail::NO_SORT, &|did| {
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
}
