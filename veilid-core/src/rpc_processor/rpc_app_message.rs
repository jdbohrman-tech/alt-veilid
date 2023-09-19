use super::*;

impl RPCProcessor {
    // Sends a high level app message
    // Can be sent via all methods including relays and routes
    #[cfg_attr(
        feature = "verbose-tracing",
        instrument(level = "trace", skip(self, message), fields(message.len = message.len()), err)
    )]
    pub async fn rpc_call_app_message(
        self,
        dest: Destination,
        message: Vec<u8>,
    ) -> Result<NetworkResult<()>, RPCError> {
        let app_message = RPCOperationAppMessage::new(message)?;
        let statement = RPCStatement::new(RPCStatementDetail::AppMessage(Box::new(app_message)));

        // Send the app message request
        self.statement(dest, statement).await
    }

    #[cfg_attr(feature="verbose-tracing", instrument(level = "trace", skip(self, msg), fields(msg.operation.op_id), ret, err))]
    pub(crate) async fn process_app_message(
        &self,
        msg: RPCMessage,
    ) -> Result<NetworkResult<()>, RPCError> {
        // Ignore if disabled
        let routing_table = self.routing_table();
        let opi = routing_table.get_own_peer_info(msg.header.routing_domain());
        if !opi
            .signed_node_info()
            .node_info()
            .has_capability(CAP_APPMESSAGE)
        {
            return Ok(NetworkResult::service_unavailable(
                "app message is not available",
            ));
        }

        // Get the statement
        let (_, _, _, kind) = msg.operation.destructure();
        let app_message = match kind {
            RPCOperationKind::Statement(s) => match s.destructure() {
                RPCStatementDetail::AppMessage(s) => s,
                _ => panic!("not an app message"),
            },
            _ => panic!("not a statement"),
        };

        // Get the crypto kind used to send this question
        let crypto_kind = msg.header.crypto_kind();

        // Get the sender node id this came from
        let sender = msg
            .opt_sender_nr
            .as_ref()
            .map(|nr| nr.node_ids().get(crypto_kind).unwrap());

        // Pass the message up through the update callback
        let message = app_message.destructure();
        (self.unlocked_inner.update_callback)(VeilidUpdate::AppMessage(Box::new(
            VeilidAppMessage::new(sender, message),
        )));

        Ok(NetworkResult::value(()))
    }
}
