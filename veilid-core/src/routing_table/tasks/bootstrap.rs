use super::*;

use futures_util::stream::{FuturesUnordered, StreamExt};
use stop_token::future::FutureExt as StopFutureExt;

impl_veilid_log_facility!("rtab");

impl RoutingTable {
    #[instrument(level = "trace", skip_all, err)]
    pub async fn bootstrap_task_routine(
        &self,
        stop_token: StopToken,
        _last_ts: Timestamp,
        _cur_ts: Timestamp,
    ) -> EyreResult<()> {
        let bootstraps = self
            .config()
            .with(|c| c.network.routing_table.bootstrap.clone());

        // Don't bother if bootstraps aren't configured
        if bootstraps.is_empty() {
            return Ok(());
        }

        veilid_log!(self debug "--- bootstrap_task");

        // See if we are specifying a direct dialinfo for bootstrap, if so use the direct mechanism
        // Otherwise treat them as txt names for the normal dns-based bootstrap mechanism
        let dial_info_converter = BootstrapDialInfoConverter::default();

        let mut bootstrap_dialinfos = Vec::<DialInfo>::new();
        let mut bootstrap_txt_names = Vec::<String>::new();
        for bootstrap in bootstraps {
            if let Ok(bootstrap_di_vec) = dial_info_converter.try_vec_from_url(&bootstrap) {
                for bootstrap_di in bootstrap_di_vec {
                    bootstrap_dialinfos.push(bootstrap_di);
                }
            } else {
                bootstrap_txt_names.push(bootstrap);
            }
        }

        // Process bootstraps into peers lists
        let network_manager = self.network_manager();
        let mut unord = FuturesUnordered::<PinBoxFuture<EyreResult<Vec<Arc<PeerInfo>>>>>::new();

        // Get a peer list from bootstraps to process
        for bsdi in bootstrap_dialinfos {
            // Direct bootstrap
            unord.push(pin_dyn_future!(async {
                let bsdi = bsdi;
                veilid_log!(self debug "Direct bootstrap with: {}", bsdi);
                pin_future!(network_manager.direct_bootstrap(bsdi)).await
            }));
        }
        for bstn in bootstrap_txt_names {
            // TXT bootstrap
            unord.push(pin_dyn_future!(async {
                let bstn = bstn;
                veilid_log!(self debug "TXT bootstrap with: {}", bstn);
                pin_future!(network_manager.txt_bootstrap(bstn)).await
            }));
        }

        let mut bootstrapped_peer_id_set = HashSet::<TypedNodeId>::new();
        let mut bootstrapped_peers = vec![];
        loop {
            match unord.next().timeout_at(stop_token.clone()).await {
                Ok(Some(res)) => match res {
                    Ok(peers) => {
                        for peer in peers {
                            let peer_node_ids = peer.node_ids();
                            if !peer_node_ids
                                .iter()
                                .any(|x| bootstrapped_peer_id_set.contains(x))
                            {
                                if self.matches_own_node_id(peer_node_ids) {
                                    veilid_log!(self debug "Ignoring own node in bootstrap response");
                                } else {
                                    for nid in peer.node_ids().iter().copied() {
                                        bootstrapped_peer_id_set.insert(nid);
                                    }
                                    bootstrapped_peers.push(peer);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        veilid_log!(self debug "Bootstrap error: {}", e);
                    }
                },
                Ok(None) => {
                    // Done
                    break;
                }
                Err(_) => {
                    // Cancelled
                    return Ok(());
                }
            }
        }

        // Get list of routing domains to bootstrap
        // NOTE: someday we may want to boot other domains than PublicInternet
        // so we are writing this code as generically as possible
        let routing_domains: RoutingDomainSet = bootstrapped_peers
            .iter()
            .map(|pi| pi.routing_domain())
            .collect();

        // For each routing domain, bootstrap it if any node in the bootstrap list
        // from TXT or BOOT mechanism has peer info in the routing domain
        // When a routing domain is bootstrapped, its current list of bootstrap nodes is cleared
        // and replaced with the latest peer info we are bootstrapping with.
        //
        // We do not use these stored bootstrap peers for anything right now but keeping track of them
        // will be useful for eventual tasks like configuration distribution and automatic updates.
        for rd in routing_domains {
            self.inner.read().with_routing_domain(rd, |rdd| {
                rdd.clear_bootstrap_peers();
            });
        }

        let valid_bootstraps = network_manager
            .bootstrap_with_peer_list(bootstrapped_peers.clone(), stop_token)
            .await?;
        if valid_bootstraps.is_empty() {
            veilid_log!(self debug "No external bootstrap peers");
        } else {
            veilid_log!(self debug "External bootstrap peers: {:#?}", valid_bootstraps);

            for rd in routing_domains {
                // Get all node ids that have a peerinfo in this routing domain
                let mut rd_peer_ids = BTreeSet::new();
                for peer in bootstrapped_peers.iter() {
                    if peer.routing_domain() == rd {
                        for nid in peer.node_ids().iter().copied() {
                            rd_peer_ids.insert(nid);
                        }
                    }
                }

                // Add valid bootstrap peers to routing domain
                self.inner.read().with_routing_domain(rd, |rdd| {
                    for bootstrap_peer in valid_bootstraps.iter().cloned() {
                        let mut add = false;
                        if let Some(pi) = bootstrap_peer.get_peer_info(rd) {
                            for nid in pi.node_ids().iter() {
                                if rd_peer_ids.contains(nid) {
                                    add = true;
                                    break;
                                }
                            }
                        }
                        if add {
                            rdd.add_bootstrap_peer(bootstrap_peer);
                        }
                    }
                });
            }
        }

        self.flush().await;

        Ok(())
    }
}
