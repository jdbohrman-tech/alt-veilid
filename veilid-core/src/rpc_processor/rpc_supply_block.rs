use super::*;

impl RPCProcessor {
    #[cfg_attr(feature="verbose-tracing", instrument(level = "trace", skip(self, msg), fields(msg.operation.op_id), ret, err))]
    pub(crate) async fn process_supply_block_q(
        &self,
        msg: RPCMessage,
    ) -> Result<NetworkResult<()>, RPCError> {
        // Ignore if disabled
        {
            let c = self.config.get();
            if c.capabilities.disable.contains(&CAP_WILL_BLOCKSTORE) {
                return Ok(NetworkResult::service_unavailable(
                    "supply block is disabled",
                ));
            }
        }
        Err(RPCError::unimplemented("process_supply_block_q"))
    }
}
