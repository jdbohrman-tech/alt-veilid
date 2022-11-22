use super::*;

impl RPCProcessor {
    #[instrument(level = "trace", skip(self, msg), fields(msg.operation.op_id), err)]
    pub(crate) async fn process_value_changed(
        &self,
        msg: RPCMessage,
    ) -> Result<NetworkResult<()>, RPCError> {
        Err(RPCError::unimplemented("process_value_changed"))
    }
}
