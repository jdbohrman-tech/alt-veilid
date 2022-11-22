use super::*;

impl RPCProcessor {
    // Sends a high level app message
    // Can be sent via all methods including relays and routes
    #[instrument(level = "trace", skip(self), ret, err)]
    pub async fn rpc_call_app_message(
        self,
        dest: Destination,
        message: Vec<u8>,
    ) -> Result<NetworkResult<()>, RPCError> {
        let app_message = RPCOperationAppMessage { message };
        let statement = RPCStatement::new(RPCStatementDetail::AppMessage(app_message));

        // Send the app message request
        self.statement(dest, statement).await
    }

    #[instrument(level = "trace", skip(self, msg), fields(msg.operation.op_id), ret, err)]
    pub(crate) async fn process_app_message(
        &self,
        msg: RPCMessage,
    ) -> Result<NetworkResult<()>, RPCError> {
        // Get the statement
        let app_message = match msg.operation.into_kind() {
            RPCOperationKind::Statement(s) => match s.into_detail() {
                RPCStatementDetail::AppMessage(s) => s,
                _ => panic!("not an app message"),
            },
            _ => panic!("not a statement"),
        };

        // Pass the message up through the update callback
        let sender = msg.opt_sender_nr.map(|nr| NodeId::new(nr.node_id()));
        let message = app_message.message;
        (self.unlocked_inner.update_callback)(VeilidUpdate::AppMessage(VeilidAppMessage {
            sender,
            message,
        }));

        Ok(NetworkResult::value(()))
    }
}
