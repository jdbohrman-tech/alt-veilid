use super::*;

impl_veilid_log_facility!("rpc");

impl RPCProcessor {
    // Sends a high level app message
    // Can be sent via all methods including relays and routes
    #[instrument(level = "trace", target = "rpc", skip(self, message), fields(message.len = message.len()), err)]
    pub async fn rpc_call_app_message(
        &self,
        dest: Destination,
        message: Vec<u8>,
    ) -> RPCNetworkResult<()> {
        let _guard = self
            .startup_context
            .startup_lock
            .enter()
            .map_err(RPCError::map_try_again("not started up"))?;

        let app_message = RPCOperationAppMessage::new(message)?;
        let statement = RPCStatement::new(RPCStatementDetail::AppMessage(Box::new(app_message)));

        // Send the app message request
        self.statement(dest, statement).await
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////

    #[instrument(level = "trace", target = "rpc", skip(self, msg), fields(msg.operation.op_id), ret, err)]
    pub(super) async fn process_app_message(&self, msg: Message) -> RPCNetworkResult<()> {
        // Ignore if disabled
        let routing_table = self.routing_table();
        let has_capability_app_message = routing_table
            .get_published_peer_info(msg.header.routing_domain())
            .map(|ppi| {
                ppi.signed_node_info()
                    .node_info()
                    .has_capability(CAP_APPMESSAGE)
            })
            .unwrap_or(false);
        if !has_capability_app_message {
            return Ok(NetworkResult::service_unavailable(
                "app message is not available",
            ));
        }

        // Get the private route this came over
        let opt_pr_pubkey = match &msg.header.detail {
            RPCMessageHeaderDetail::Direct(_) | RPCMessageHeaderDetail::SafetyRouted(_) => None,
            RPCMessageHeaderDetail::PrivateRouted(pr) => Some(pr.private_route),
        };
        let route_id = if let Some(pr_pubkey) = opt_pr_pubkey {
            let rss = routing_table.route_spec_store();
            let Some(route_id) = rss.get_route_id_for_key(&pr_pubkey) else {
                return Ok(NetworkResult::invalid_message(format!(
                    "private route does not exist for key: {}",
                    pr_pubkey
                )));
            };
            Some(route_id)
        } else {
            None
        };

        // Get the statement
        let (_, _, kind) = msg.operation.destructure();
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

        #[cfg(not(feature = "footgun"))]
        {
            if sender.is_some() {
                return Ok(NetworkResult::invalid_message(
                    "Direct NodeId senders are not allowed for AppMessage when footgun is disabled",
                ));
            }
        }

        // Pass the message up through the update callback
        let message = app_message.destructure();
        (self.update_callback())(VeilidUpdate::AppMessage(Box::new(VeilidAppMessage::new(
            sender, route_id, message,
        ))));

        Ok(NetworkResult::value(()))
    }
}
