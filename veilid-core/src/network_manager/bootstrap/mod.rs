mod bootstrap_record;
mod debug;
mod dial_info_converter;
mod direct_bootstrap;
mod txt_bootstrap;

use super::*;
use futures_util::StreamExt as _;
use stop_token::future::FutureExt as _;

pub use bootstrap_record::*;
pub use dial_info_converter::*;
pub use txt_bootstrap::*;

impl_veilid_log_facility!("net");

impl NetworkManager {
    //#[instrument(level = "trace", skip(self), err)]
    pub fn bootstrap_with_peer(
        &self,
        crypto_kinds: Vec<CryptoKind>,
        pi: Arc<PeerInfo>,
        unord: &FuturesUnordered<PinBoxFutureStatic<Option<NodeRef>>>,
    ) {
        veilid_log!(self trace
            "--- bootstrapping {} with {:?}",
            pi.node_ids(),
            pi.signed_node_info().node_info().dial_info_detail_list()
        );

        let routing_domain = pi.routing_domain();
        let routing_table = self.routing_table();

        let nr = match routing_table.register_node_with_peer_info(pi, true) {
            Ok(nr) => nr,
            Err(e) => {
                veilid_log!(self error "failed to register bootstrap peer info: {}", e);
                return;
            }
        };

        // Add this our futures to process in parallel
        for crypto_kind in crypto_kinds {
            // Bootstrap this crypto kind
            let nr = nr.unfiltered();
            unord.push(Box::pin(
                    async move {
                        let network_manager = nr.network_manager();
                        let routing_table = nr.routing_table();

                        // Get what contact method would be used for contacting the bootstrap
                        let bsdi = match network_manager
                            .get_node_contact_method(nr.sequencing_filtered(Sequencing::PreferOrdered))
                        {
                            Ok(Some(ncm)) if ncm.is_direct() => ncm.direct_dial_info().unwrap(),
                            Ok(v) => {
                                veilid_log!(nr debug "invalid contact method for bootstrap, ignoring peer: {:?}", v);
                                // let _ =
                                //     network_manager
                                //     .get_node_contact_method(nr.clone());
                                return None;
                            }
                            Err(e) => {
                                veilid_log!(nr warn "unable to bootstrap: {}", e);
                                return None;
                            }
                        };

                        // Need VALID signed peer info, so ask bootstrap to find_node of itself
                        // which will ensure it has the bootstrap's signed peer info as part of the response
                        let _ = routing_table.find_nodes_close_to_node_ref(crypto_kind, nr.sequencing_filtered(Sequencing::PreferOrdered), vec![]).await;

                        // Ensure we got the signed peer info
                        if !nr.signed_node_info_has_valid_signature(routing_domain) {
                            veilid_log!(nr info "bootstrap server is not responding for dialinfo: {}", bsdi);

                            // Try a different dialinfo next time
                            network_manager.address_filter().set_dial_info_failed(bsdi);
                            None
                        } else {
                            // otherwise this bootstrap is valid, lets ask it to find ourselves now
                            // We should prefer nodes that have relaying, signaling, routing and validation of dial info
                            // for our initial nodes in our routing table because we need these things in order to
                            // properly attach to the network
                            routing_table.reverse_find_node(crypto_kind, nr.clone(), true, CONNECTIVITY_CAPABILITIES.to_vec()).await;

                            veilid_log!(nr info "bootstrap of {} successful via {}", crypto_kind, nr);
                            Some(nr)
                        }
                    }
                    .instrument(Span::current()),
                ));
        }
    }

    /// Takes in a list of bootstrap peer info, and attempts bootstrapping
    /// A list of valid bootstrap peer noderefs is returned
    #[instrument(level = "trace", skip(self), err)]
    pub async fn bootstrap_with_peer_list(
        &self,
        bootstrap_peers: Vec<Arc<PeerInfo>>,
        stop_token: StopToken,
    ) -> EyreResult<Vec<NodeRef>> {
        if bootstrap_peers.is_empty() {
            veilid_log!(self debug "No peers suitable for bootstrap");
            return Ok(vec![]);
        }

        veilid_log!(self debug "Bootstrap peers: {:?}", &bootstrap_peers);

        // Get crypto kinds to bootstrap
        let crypto_kinds = self.get_bootstrap_crypto_kinds();
        veilid_log!(self debug "Bootstrap crypto kinds: {:?}", &crypto_kinds);

        // Run all bootstrap operations concurrently
        let mut unord = FuturesUnordered::<PinBoxFutureStatic<Option<NodeRef>>>::new();
        for peer in bootstrap_peers {
            // Validate bootstrap key for crypto kinds
            let mut peer_has_crypto_kind = false;
            for ck in crypto_kinds.iter().copied() {
                if peer.node_ids().get(ck).is_some() {
                    peer_has_crypto_kind = true;
                }
            }

            if peer_has_crypto_kind {
                veilid_log!(self info "Attempting bootstrap: {}", peer.node_ids());
                self.bootstrap_with_peer(crypto_kinds.clone(), peer, &unord);
            }
        }

        // Wait for all bootstrap operations to complete before we complete the singlefuture
        let mut valid_bootstraps = vec![];
        while let Ok(Some(res)) = unord.next().timeout_at(stop_token.clone()).await {
            if let Some(valid_bootstrap) = res {
                valid_bootstraps.push(valid_bootstrap);
            }
        }
        Ok(valid_bootstraps)
    }

    /// Get counts by crypto kind and figure out which crypto kinds need bootstrapping
    pub fn get_bootstrap_crypto_kinds(&self) -> Vec<CryptoKind> {
        let routing_table = self.routing_table();

        let live_entry_counts = routing_table.cached_live_entry_counts();

        let mut crypto_kinds = Vec::new();
        for rd in BOOTSTRAP_ROUTING_DOMAINS {
            for crypto_kind in VALID_CRYPTO_KINDS {
                // Do we need to bootstrap this crypto kind?
                let eckey = (rd, crypto_kind);
                let cnt = live_entry_counts
                    .connectivity_capabilities
                    .get(&eckey)
                    .copied()
                    .unwrap_or_default();
                if cnt < MIN_BOOTSTRAP_CONNECTIVITY_PEERS {
                    crypto_kinds.push(crypto_kind);
                }
            }
        }
        crypto_kinds
    }
}
