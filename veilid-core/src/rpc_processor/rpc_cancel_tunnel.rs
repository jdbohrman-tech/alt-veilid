use super::*;

impl RPCProcessor {
    #[cfg_attr(feature="verbose-tracing", instrument(level = "trace", skip(self, msg), fields(msg.operation.op_id), ret, err))]
    pub(crate) async fn process_cancel_tunnel_q(&self, msg: RPCMessage) -> RPCNetworkResult<()> {
        // Ignore if disabled
        #[cfg(feature = "unstable-tunnels")]
        {
            let routing_table = self.routing_table();
            {
                if let Some(opi) = routing_table.get_own_peer_info(msg.header.routing_domain()) {
                    if !opi
                        .signed_node_info()
                        .node_info()
                        .has_capability(CAP_TUNNEL)
                    {
                        return Ok(NetworkResult::service_unavailable(
                            "tunnel is not available",
                        ));
                    }
                }
            }
        }

        Err(RPCError::unimplemented("process_cancel_tunnel_q"))
    }
}
