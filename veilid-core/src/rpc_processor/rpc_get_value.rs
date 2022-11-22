use super::*;

impl RPCProcessor {
    #[instrument(level = "trace", skip(self, msg), fields(msg.operation.op_id), ret, err)]
    pub(crate) async fn process_get_value_q(
        &self,
        msg: RPCMessage,
    ) -> Result<NetworkResult<()>, RPCError> {
        Err(RPCError::unimplemented("process_get_value_q"))
    }
}
