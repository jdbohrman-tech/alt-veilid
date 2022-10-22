use super::*;

impl RPCProcessor {
    // xxx do not process latency for routed messages

    #[instrument(level = "trace", skip(self, msg), fields(msg.operation.op_id), err)]
    pub(crate) async fn process_route(&self, msg: RPCMessage) -> Result<(), RPCError> {
        // xxx do not process latency for routed messages
        // tracing::Span::current().record("res", &tracing::field::display(res));

        xxx continue here

        Err(RPCError::unimplemented("process_route"))
    }
}
