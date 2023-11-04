use super::*;

impl NetworkManager {
    // Direct bootstrap request handler (separate fallback mechanism from cheaper TXT bootstrap mechanism)
    #[instrument(level = "trace", skip(self), ret, err)]
    pub(crate) async fn handle_boot_request(&self, flow: Flow) -> EyreResult<NetworkResult<()>> {
        let routing_table = self.routing_table();

        // Get a bunch of nodes with the various
        let bootstrap_nodes = routing_table.find_bootstrap_nodes_filtered(2);

        // Serialize out peer info
        let bootstrap_peerinfo: Vec<PeerInfo> = bootstrap_nodes
            .iter()
            .filter_map(|nr| nr.make_peer_info(RoutingDomain::PublicInternet))
            .collect();
        let json_bytes = serialize_json(bootstrap_peerinfo).as_bytes().to_vec();

        // Reply with a chunk of signed routing table
        match self
            .net()
            .send_data_to_existing_flow(flow, json_bytes)
            .await?
        {
            SendDataToExistingFlowResult::Sent(_) => {
                // Bootstrap reply was sent
                Ok(NetworkResult::value(()))
            }
            SendDataToExistingFlowResult::NotSent(_) => Ok(NetworkResult::no_connection_other(
                "bootstrap reply could not be sent",
            )),
        }
    }

    // Direct bootstrap request
    #[instrument(level = "trace", err, skip(self))]
    pub async fn boot_request(&self, dial_info: DialInfo) -> EyreResult<Vec<PeerInfo>> {
        let timeout_ms = self.with_config(|c| c.network.rpc.timeout_ms);
        // Send boot magic to requested peer address
        let data = BOOT_MAGIC.to_vec();

        let out_data: Vec<u8> = network_result_value_or_log!(self
            .net()
            .send_recv_data_unbound_to_dial_info(dial_info, data, timeout_ms)
            .await? => [ format!(": dial_info={}, data.len={}", dial_info, data.len()) ]
        {
            return Ok(Vec::new());
        });

        let bootstrap_peerinfo: Vec<PeerInfo> =
            deserialize_json(std::str::from_utf8(&out_data).wrap_err("bad utf8 in boot peerinfo")?)
                .wrap_err("failed to deserialize boot peerinfo")?;

        Ok(bootstrap_peerinfo)
    }
}
