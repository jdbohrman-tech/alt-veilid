use super::*;

impl RPCProcessor {
    #[cfg_attr(feature="verbose-tracing", instrument(level = "trace", skip(self, msg), fields(msg.operation.op_id), ret, err))]
    pub(crate) async fn process_watch_value_q(
        &self,
        msg: RPCMessage,
    ) -> Result<NetworkResult<()>, RPCError> {
        // Ignore if disabled
        let routing_table = self.routing_table();
        let opi = routing_table.get_own_peer_info(msg.header.routing_domain());
        if !opi.signed_node_info().node_info().has_capability(CAP_DHT) {
            return Ok(NetworkResult::service_unavailable("dht is not available"));
        }
        Err(RPCError::unimplemented("process_watch_value_q"))
    }
}
