use super::*;

impl RPCProcessor {
    // Sends a high level app request and wait for response
    // Can be sent via all methods including relays and routes
    #[instrument(level = "trace", skip(self), ret, err)]
    pub async fn rpc_call_app_call(
        self,
        dest: Destination,
        message: Vec<u8>,
    ) -> Result<NetworkResult<Answer<Vec<u8>>>, RPCError> {
        let app_call_q = RPCOperationAppCallQ { message };
        let question = RPCQuestion::new(
            network_result_try!(self.get_destination_respond_to(&dest)?),
            RPCQuestionDetail::AppCallQ(app_call_q),
        );

        // Send the app call question
        let waitable_reply = network_result_try!(self.question(dest, question).await?);

        // Wait for reply
        let (msg, latency) = match self.wait_for_reply(waitable_reply).await? {
            TimeoutOr::Timeout => return Ok(NetworkResult::Timeout),
            TimeoutOr::Value(v) => v,
        };

        // Get the right answer type
        let app_call_a = match msg.operation.into_kind() {
            RPCOperationKind::Answer(a) => match a.into_detail() {
                RPCAnswerDetail::AppCallA(a) => a,
                _ => return Err(RPCError::invalid_format("not an appcall answer")),
            },
            _ => return Err(RPCError::invalid_format("not an answer")),
        };

        Ok(NetworkResult::value(Answer::new(
            latency,
            app_call_a.message,
        )))
    }

    #[instrument(level = "trace", skip(self, msg), fields(msg.operation.op_id), ret, err)]
    pub(crate) async fn process_app_call_q(
        &self,
        msg: RPCMessage,
    ) -> Result<NetworkResult<()>, RPCError> {
        // Get the question
        let app_call_q = match msg.operation.kind() {
            RPCOperationKind::Question(q) => match q.detail() {
                RPCQuestionDetail::AppCallQ(q) => q,
                _ => panic!("not an appcall question"),
            },
            _ => panic!("not a question"),
        };

        // Register a waiter for this app call
        let id = msg.operation.op_id();
        let handle = self.unlocked_inner.waiting_app_call_table.add_op_waiter(id);

        // Pass the call up through the update callback
        let sender = msg
            .opt_sender_nr
            .as_ref()
            .map(|nr| NodeId::new(nr.node_id()));
        let message = app_call_q.message.clone();
        (self.unlocked_inner.update_callback)(VeilidUpdate::AppCall(VeilidAppCall {
            sender,
            message,
            id,
        }));

        // Wait for an app call answer to come back from the app
        let res = self
            .unlocked_inner
            .waiting_app_call_table
            .wait_for_op(handle, self.unlocked_inner.timeout)
            .await?;
        let (message, _latency) = match res {
            TimeoutOr::Timeout => {
                // No message sent on timeout, but this isn't an error
                log_rpc!(debug "App call timed out for id {}", id);
                return Ok(NetworkResult::timeout());
            }
            TimeoutOr::Value(v) => v,
        };

        // Return the appcall answer
        let app_call_a = RPCOperationAppCallA { message };

        // Send status answer
        let res = self
            .answer(msg, RPCAnswer::new(RPCAnswerDetail::AppCallA(app_call_a)))
            .await?;
        tracing::Span::current().record("res", &tracing::field::display(res));
        Ok(res)
    }

    /// Exposed to API for apps to return app call answers
    pub async fn app_call_reply(&self, id: u64, message: Vec<u8>) -> Result<(), RPCError> {
        self.unlocked_inner
            .waiting_app_call_table
            .complete_op_waiter(id, message)
            .await
    }
}
