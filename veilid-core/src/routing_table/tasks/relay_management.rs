use super::*;

impl RoutingTable {
    // Keep relays assigned and accessible
    #[instrument(level = "trace", skip(self), err)]
    pub(crate) async fn relay_management_task_routine(
        self,
        _stop_token: StopToken,
        _last_ts: Timestamp,
        cur_ts: Timestamp,
    ) -> EyreResult<()> {
        // Get our node's current node info and network class and do the right thing
        let Some(own_peer_info) = self.get_own_peer_info(RoutingDomain::PublicInternet) else {
            return Ok(());
        };
        let own_node_info = own_peer_info.signed_node_info().node_info();
        let network_class = own_node_info.network_class();
        let relay_node_filter = self.make_public_internet_relay_node_filter();

        // Get routing domain editor
        let mut editor = self.edit_routing_domain(RoutingDomain::PublicInternet);

        // If we already have a relay, see if it is dead, or if we don't need it any more
        let has_relay = {
            if let Some(relay_node) = self.relay_node(RoutingDomain::PublicInternet) {
                let state = relay_node.state(cur_ts);
                // Relay node is dead or no longer needed
                if matches!(state, BucketEntryState::Dead) {
                    info!("Relay node died, dropping relay {}", relay_node);
                    editor.clear_relay_node();
                    false
                }
                // Relay node no longer can relay
                else if relay_node.operate(|_rti, e| !relay_node_filter(e)) {
                    info!(
                        "Relay node can no longer relay, dropping relay {}",
                        relay_node
                    );
                    editor.clear_relay_node();
                    false
                }
                // Relay node is no longer required
                else if !own_node_info.requires_relay() {
                    info!(
                        "Relay node no longer required, dropping relay {}",
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

        // Do we need a relay?
        if !has_relay && own_node_info.requires_relay() {
            // Do we want an outbound relay?
            let mut got_outbound_relay = false;
            if network_class.outbound_wants_relay() {
                // The outbound relay is the host of the PWA
                if let Some(outbound_relay_peerinfo) = intf::get_outbound_relay_peer().await {
                    // Register new outbound relay
                    match self.register_node_with_peer_info(
                        RoutingDomain::PublicInternet,
                        outbound_relay_peerinfo,
                        false,
                    ) {
                        Ok(nr) => {
                            info!("Outbound relay node selected: {}", nr);
                            editor.set_relay_node(nr);
                            got_outbound_relay = true;
                        }
                        Err(e) => {
                            log_rtab!(error "failed to register node with peer info: {}", e);
                        }
                    }
                } else {
                    info!("Outbound relay desired but not available");
                }
            }
            if !got_outbound_relay {
                // Find a node in our routing table that is an acceptable inbound relay
                if let Some(nr) = self.find_inbound_relay(RoutingDomain::PublicInternet, cur_ts) {
                    info!("Inbound relay node selected: {}", nr);
                    editor.set_relay_node(nr);
                }
            }
        }

        // Commit the changes
        editor.commit();

        Ok(())
    }

    pub fn make_public_internet_relay_node_filter(&self) -> impl Fn(&BucketEntryInner) -> bool {
        // Get all our outbound protocol/address types
        let outbound_dif = self.get_outbound_dial_info_filter(RoutingDomain::PublicInternet);
        let mapped_port_info = self.get_low_level_port_info();

        move |e: &BucketEntryInner| {
            // Ensure this node is not on the local network
            if e.has_node_info(RoutingDomain::LocalNetwork.into()) {
                return false;
            }

            // Disqualify nodes that don't cover all our inbound ports for tcp and udp
            // as we need to be able to use the relay for keepalives for all nat mappings
            let mut low_level_protocol_ports = mapped_port_info.low_level_protocol_ports.clone();

            let can_serve_as_relay = e
                .node_info(RoutingDomain::PublicInternet)
                .map(|n| {
                    if !(n.has_capability(CAP_RELAY) && n.is_fully_direct_inbound()) {
                        // Needs to be able to accept packets to relay directly
                        return false;
                    }

                    let dids = n.all_filtered_dial_info_details(DialInfoDetail::NO_SORT, |did| {
                        did.matches_filter(&outbound_dif)
                    });
                    for did in &dids {
                        let pt = did.dial_info.protocol_type();
                        let at = did.dial_info.address_type();
                        if let Some((llpt, port)) = mapped_port_info.protocol_to_port.get(&(pt, at))
                        {
                            low_level_protocol_ports.remove(&(*llpt, at, *port));
                        }
                    }
                    low_level_protocol_ports.is_empty()
                })
                .unwrap_or(false);
            if !can_serve_as_relay {
                return false;
            }

            true
        }
    }

    #[instrument(level = "trace", skip(self), ret)]
    pub fn find_inbound_relay(
        &self,
        routing_domain: RoutingDomain,
        cur_ts: Timestamp,
    ) -> Option<NodeRef> {
        // Get relay filter function
        let relay_node_filter = match routing_domain {
            RoutingDomain::PublicInternet => self.make_public_internet_relay_node_filter(),
            RoutingDomain::LocalNetwork => {
                unimplemented!();
            }
        };

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
