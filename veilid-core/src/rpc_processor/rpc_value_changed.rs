use super::*;

impl RPCProcessor {
    #[cfg_attr(feature="verbose-tracing", instrument(level = "trace", skip(self, msg), fields(msg.operation.op_id), err))]
    pub(crate) async fn process_value_changed(
        &self,
        msg: RPCMessage,
    ) -> Result<NetworkResult<()>, RPCError> {
        // Ignore if disabled
        {
            let c = self.config.get();
            if c.capabilities.disable.contains(&CAP_WILL_DHT) {
                return Ok(NetworkResult::service_unavailable(
                    "value changed is disabled",
                ));
            }
        }
        Err(RPCError::unimplemented("process_value_changed"))
    }
}
