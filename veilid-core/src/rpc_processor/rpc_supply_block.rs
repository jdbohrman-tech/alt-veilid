use super::*;

impl RPCProcessor {
    #[cfg_attr(feature="verbose-tracing", instrument(level = "trace", skip(self, msg), fields(msg.operation.op_id), ret, err))]
    pub(crate) async fn process_supply_block_q(&self, msg: RPCMessage) -> RPCNetworkResult<()> {
        // Ignore if disabled
        #[cfg(feature = "unstable-blockstore")]
        {
            let routing_table = self.routing_table();
            {
                if let Some(opi) = routing_table.get_own_peer_info(detail.routing_domain) {
                    if !opi.signed_node_info().node_info().can_blockstore() {
                        return Ok(NetworkResult::service_unavailable(
                            "block store is not available",
                        ));
                    }
                }
            }
        }
        Err(RPCError::unimplemented("process_supply_block_q"))
    }
}
