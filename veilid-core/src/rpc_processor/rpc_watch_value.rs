use super::*;

impl RPCProcessor {
    #[cfg_attr(feature="verbose-tracing", instrument(level = "trace", skip(self, msg), fields(msg.operation.op_id), ret, err))]
    pub(crate) async fn process_watch_value_q(
        &self,
        msg: RPCMessage,
    ) -> Result<NetworkResult<()>, RPCError> {
        // Ignore if disabled
        {
            let c = self.config.get();
            if c.capabilities.disable.contains(&CAP_WILL_DHT) {
                return Ok(NetworkResult::service_unavailable(
                    "watch value is disabled",
                ));
            }
        }

        Err(RPCError::unimplemented("process_watch_value_q"))
    }
}
