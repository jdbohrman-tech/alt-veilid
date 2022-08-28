use super::*;

impl RPCProcessor {
    // Send StatusQ RPC request, receive StatusA answer
    // Can be sent via relays, but not via routes
    #[instrument(level = "trace", skip(self), ret, err)]
    pub async fn rpc_call_status(
        self,
        peer: NodeRef,
    ) -> Result<NetworkResult<Answer<SenderInfo>>, RPCError> {
        let node_status = self.network_manager().generate_node_status();
        let status_q = RPCOperationStatusQ { node_status };
        let respond_to = self.make_respond_to_sender(peer.clone());
        let question = RPCQuestion::new(respond_to, RPCQuestionDetail::StatusQ(status_q));

        // Send the info request
        let waitable_reply = network_result_try!(
            self.question(Destination::Direct(peer.clone()), question, None)
                .await?
        );

        // Note what kind of ping this was and to what peer scope
        let send_data_kind = waitable_reply.send_data_kind;

        // Wait for reply
        let (msg, latency) = match self.wait_for_reply(waitable_reply).await? {
            TimeoutOr::Timeout => return Ok(NetworkResult::Timeout),
            TimeoutOr::Value(v) => v,
        };

        // Get the right answer type
        let status_a = match msg.operation.into_kind() {
            RPCOperationKind::Answer(a) => match a.into_detail() {
                RPCAnswerDetail::StatusA(a) => a,
                _ => return Err(RPCError::invalid_format("not a status answer")),
            },
            _ => return Err(RPCError::invalid_format("not an answer")),
        };

        // Update latest node status in routing table
        peer.operate_mut(|e| {
            e.update_node_status(status_a.node_status.clone());
        });

        // Report sender_info IP addresses to network manager
        if let Some(socket_address) = status_a.sender_info.socket_address {
            match send_data_kind {
                SendDataKind::Direct(connection_descriptor) => {
                    match connection_descriptor.peer_scope() {
                        PeerScope::Global => self.network_manager().report_global_socket_address(
                            socket_address,
                            connection_descriptor,
                            peer,
                        ),
                        PeerScope::Local => self.network_manager().report_local_socket_address(
                            socket_address,
                            connection_descriptor,
                            peer,
                        ),
                    }
                }
                SendDataKind::Indirect => {
                    // Do nothing in this case, as the socket address returned here would be for any node other than ours
                }
                SendDataKind::Existing(_) => {
                    // Do nothing in this case, as an existing connection could not have a different public address or it would have been reset
                }
            }
        }

        Ok(NetworkResult::value(Answer::new(
            latency,
            status_a.sender_info,
        )))
    }

    #[instrument(level = "trace", skip(self, msg), fields(msg.operation.op_id, res), err)]
    pub(crate) async fn process_status_q(&self, msg: RPCMessage) -> Result<(), RPCError> {
        let connection_descriptor = msg.header.connection_descriptor;

        // Get the question
        let status_q = match msg.operation.kind() {
            RPCOperationKind::Question(q) => match q.detail() {
                RPCQuestionDetail::StatusQ(q) => q,
                _ => panic!("not a status question"),
            },
            _ => panic!("not a question"),
        };

        // update node status for the requesting node to our routing table
        if let Some(sender_nr) = msg.opt_sender_nr.clone() {
            // Update latest node status in routing table for the statusq sender
            sender_nr.operate_mut(|e| {
                e.update_node_status(status_q.node_status.clone());
            });
        }

        // Make status answer
        let node_status = self.network_manager().generate_node_status();

        // Get the peer address in the returned sender info
        let sender_info = SenderInfo {
            socket_address: Some(*connection_descriptor.remote_address()),
        };

        let status_a = RPCOperationStatusA {
            node_status,
            sender_info,
        };

        // Send status answer
        let res = self
            .answer(
                msg,
                RPCAnswer::new(RPCAnswerDetail::StatusA(status_a)),
                None,
            )
            .await?;
        tracing::Span::current().record("res", &tracing::field::display(res));
        Ok(())
    }
}
