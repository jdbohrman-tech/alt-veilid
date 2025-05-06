use super::*;

impl_veilid_log_facility!("net");

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BootV1Response {
    pub records: Vec<String>,
    pub peers: Vec<Arc<PeerInfo>>,
}

impl NetworkManager {
    /// Direct bootstrap v1 request handler
    /// This is a proxy mechanism to the TXT bootstrap mechanism
    /// that is intended for supporting WS/WSS nodes that can not perform DNS TXT lookups,
    /// however this does work over straight UDP and TCP protocols as well.
    #[instrument(level = "trace", target = "net", skip(self), ret, err)]
    pub async fn handle_boot_v1_request(&self, flow: Flow) -> EyreResult<NetworkResult<()>> {
        let bootstraps = self
            .config()
            .with(|c| c.network.routing_table.bootstrap.clone());

        // Don't bother if bootstraps aren't configured
        if bootstraps.is_empty() {
            return Ok(NetworkResult::service_unavailable(
                "no bootstraps configured",
            ));
        }

        // Extract only the TXT hostnames
        let dial_info_converter = BootstrapDialInfoConverter::default();
        let mut bootstrap_txt_names = Vec::<String>::new();
        for bootstrap in bootstraps {
            if dial_info_converter.try_vec_from_url(&bootstrap).is_ok() {
                // skip direct bootstraps here
            } else {
                bootstrap_txt_names.push(bootstrap);
            }
        }

        // Process bootstraps into TXT strings
        let mut unord = FuturesUnordered::<PinBoxFuture<EyreResult<Vec<String>>>>::new();

        for bstn in bootstrap_txt_names {
            // TXT bootstrap
            unord.push(pin_dyn_future!(async {
                let bstn = bstn;
                self.resolve_bootstrap_txt_strings(bstn).await
            }));
        }

        let mut txt_strings_set = HashSet::<String>::new();
        while let Some(res) = unord.next().await {
            match res {
                Ok(txt_strings) => {
                    for txt_string in txt_strings {
                        txt_strings_set.insert(txt_string);
                    }
                }
                Err(e) => {
                    veilid_log!(self debug "Direct bootstrap resolution error: {}", e);
                }
            }
        }

        let mut records = txt_strings_set.into_iter().collect::<Vec<_>>();
        records.sort();

        // Add peer infos if we have them, only for the peers present in the records
        let routing_table = self.routing_table();
        let routing_domain = RoutingDomain::PublicInternet;
        let bsrecs = self.parse_bootstrap_v1(&records)?;

        let peers: Vec<Arc<PeerInfo>> = bsrecs
            .into_iter()
            .filter_map(|bsrec| {
                if routing_table.matches_own_node_id(bsrec.node_ids()) {
                    routing_table.get_published_peer_info(routing_domain)
                } else if let Some(best_node_id) = bsrec.node_ids().best() {
                    if let Some(nr) = routing_table.lookup_node_ref(best_node_id).ok().flatten() {
                        nr.get_peer_info(routing_domain)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        // Serialize out bootstrap response
        let bootv1response = BootV1Response { records, peers };
        let json_bytes = serialize_json(bootv1response).as_bytes().to_vec();
        veilid_log!(self trace "B01T reponse: {}", String::from_utf8_lossy(&json_bytes));

        // Reply with bootstrap records
        let net = self.net();
        match pin_future_closure!(net.send_data_to_existing_flow(flow, json_bytes)).await? {
            SendDataToExistingFlowResult::Sent(_) => {
                // Bootstrap reply was sent
                Ok(NetworkResult::value(()))
            }
            SendDataToExistingFlowResult::NotSent(_) => Ok(NetworkResult::no_connection_other(
                "bootstrap reply could not be sent",
            )),
        }
    }
}
