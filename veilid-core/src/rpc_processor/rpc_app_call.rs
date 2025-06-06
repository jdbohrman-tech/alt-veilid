use super::*;

impl_veilid_log_facility!("rpc");

impl RPCProcessor {
    // Sends a high level app request and wait for response
    // Can be sent via all methods including relays and routes
    #[instrument(level = "trace", target = "rpc", skip(self, message), fields(message.len = message.len(), ret.latency, ret.len), err)]
    pub async fn rpc_call_app_call(
        &self,
        dest: Destination,
        message: Vec<u8>,
    ) -> RPCNetworkResult<Answer<Vec<u8>>> {
        let _guard = self
            .startup_context
            .startup_lock
            .enter()
            .map_err(RPCError::map_try_again("not started up"))?;

        let debug_string = format!("AppCall(message(len)={}) => {}", message.len(), dest);

        let app_call_q = RPCOperationAppCallQ::new(message)?;
        let question = RPCQuestion::new(
            network_result_try!(self.get_destination_respond_to(&dest)?),
            RPCQuestionDetail::AppCallQ(Box::new(app_call_q)),
        );

        // Send the app call question
        let waitable_reply = network_result_try!(self.question(dest, question, None).await?);

        // Keep the reply private route that was used to return with the answer
        let reply_private_route = waitable_reply.context.reply_private_route;

        // Wait for reply
        let (msg, latency) = match self.wait_for_reply(waitable_reply, debug_string).await? {
            TimeoutOr::Timeout => return Ok(NetworkResult::Timeout),
            TimeoutOr::Value(v) => v,
        };

        // Get the right answer type
        let (_, _, kind) = msg.operation.destructure();
        let app_call_a = match kind {
            RPCOperationKind::Answer(a) => match a.destructure() {
                RPCAnswerDetail::AppCallA(a) => a,
                _ => return Ok(NetworkResult::invalid_message("not an appcall answer")),
            },
            _ => return Ok(NetworkResult::invalid_message("not an answer")),
        };

        let a_message = app_call_a.destructure();

        #[cfg(feature = "verbose-tracing")]
        tracing::Span::current().record("ret.latency", latency.as_u64());
        #[cfg(feature = "verbose-tracing")]
        tracing::Span::current().record("ret.len", a_message.len());
        Ok(NetworkResult::value(Answer::new(
            latency,
            reply_private_route,
            a_message,
        )))
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////

    #[instrument(level = "trace", target = "rpc", skip(self, msg), fields(msg.operation.op_id), ret, err)]
    pub(super) async fn process_app_call_q(&self, msg: Message) -> RPCNetworkResult<()> {
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
                "app call is not available",
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

        // Get the question
        let (op_id, _, kind) = msg.operation.clone().destructure();
        let app_call_q = match kind {
            RPCOperationKind::Question(q) => match q.destructure() {
                (_, RPCQuestionDetail::AppCallQ(q)) => q,
                _ => panic!("not an appcall question"),
            },
            _ => panic!("not a question"),
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
                    "Direct NodeId senders are not allowed for AppCall when footgun is disabled",
                ));
            }
        }

        // Register a waiter for this app call
        let handle = self.waiting_app_call_table.add_op_waiter(op_id, ());

        // Pass the call up through the update callback
        let message_q = app_call_q.destructure();
        (self.update_callback())(VeilidUpdate::AppCall(Box::new(VeilidAppCall::new(
            sender, route_id, message_q, op_id,
        ))));

        // Wait for an app call answer to come back from the app
        let res = self
            .waiting_app_call_table
            .wait_for_op(handle, self.timeout_us)
            .await?;
        let (message_a, _latency) = match res {
            TimeoutOr::Timeout => {
                // No message sent on timeout, but this isn't an error
                veilid_log!(self debug "App call timed out for id {}", op_id);
                return Ok(NetworkResult::timeout());
            }
            TimeoutOr::Value(v) => v,
        };

        // Return the appcall answer
        let app_call_a = RPCOperationAppCallA::new(message_a)?;

        // Send status answer
        self.answer(
            msg,
            RPCAnswer::new(RPCAnswerDetail::AppCallA(Box::new(app_call_a))),
        )
        .await
    }

    /// Exposed to API for apps to return app call answers
    #[instrument(level = "trace", target = "rpc", skip_all)]
    pub fn app_call_reply(&self, call_id: OperationId, message: Vec<u8>) -> Result<(), RPCError> {
        let _guard = self
            .startup_context
            .startup_lock
            .enter()
            .map_err(RPCError::map_try_again("not started up"))?;
        self.waiting_app_call_table
            .complete_op_waiter(call_id, message)
            .map_err(RPCError::ignore)
    }
}
