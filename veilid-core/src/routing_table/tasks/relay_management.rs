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
        let own_node_info = own_peer_info.signed_node_info.node_info();
        let network_class = own_node_info.network_class;

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
                } else if !own_node_info.requires_relay() {
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
                    if let Some(nr) = self.register_node_with_peer_info(
                        RoutingDomain::PublicInternet,
                        outbound_relay_peerinfo.node_id.key,
                        outbound_relay_peerinfo.signed_node_info,
                        false,
                    ) {
                        info!("Outbound relay node selected: {}", nr);
                        editor.set_relay_node(nr);
                        got_outbound_relay = true;
                    }
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
        editor.commit().await;

        Ok(())
    }
}
