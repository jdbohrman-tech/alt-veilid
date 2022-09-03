use super::*;

impl RPCProcessor {
    // Send StatusQ RPC request, receive StatusA answer
    // Can be sent via relays, but not via routes
    #[instrument(level = "trace", skip(self), ret, err)]
    pub async fn rpc_call_status(
        self,
        peer: NodeRef,
    ) -> Result<NetworkResult<Answer<SenderInfo>>, RPCError> {
        let routing_domain = match peer.best_routing_domain() {
            Some(rd) => rd,
            None => {
                return Ok(NetworkResult::no_connection_other(
                    "no routing domain for peer",
                ))
            }
        };
        let node_status = self.network_manager().generate_node_status(routing_domain);
        let status_q = RPCOperationStatusQ { node_status };
        let question = RPCQuestion::new(RespondTo::Sender, RPCQuestionDetail::StatusQ(status_q));

        // Send the info request
        let waitable_reply = network_result_try!(
            self.question(Destination::direct(peer.clone()), question)
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

        // Ensure the returned node status is the kind for the routing domain we asked for
        match routing_domain {
            RoutingDomain::PublicInternet => {
                if !matches!(status_a.node_status, NodeStatus::PublicInternet(_)) {
                    return Ok(NetworkResult::invalid_message(
                        "node status doesn't match PublicInternet routing domain",
                    ));
                }
            }
            RoutingDomain::LocalNetwork => {
                if !matches!(status_a.node_status, NodeStatus::LocalNetwork(_)) {
                    return Ok(NetworkResult::invalid_message(
                        "node status doesn't match LocalNetwork routing domain",
                    ));
                }
            }
        }

        // Update latest node status in routing table
        peer.update_node_status(status_a.node_status);

        // Report sender_info IP addresses to network manager
        // Don't need to validate these addresses for the current routing domain
        // the address itself is irrelevant, and the remote node can lie anyway
        if let Some(socket_address) = status_a.sender_info.socket_address {
            match send_data_kind {
                SendDataKind::Direct(connection_descriptor) => match routing_domain {
                    RoutingDomain::PublicInternet => self
                        .network_manager()
                        .report_public_internet_socket_address(
                            socket_address,
                            connection_descriptor,
                            peer,
                        ),
                    RoutingDomain::LocalNetwork => {
                        self.network_manager().report_local_network_socket_address(
                            socket_address,
                            connection_descriptor,
                            peer,
                        )
                    }
                },
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
        let routing_domain = msg.header.routing_domain;

        // Get the question
        let status_q = match msg.operation.kind() {
            RPCOperationKind::Question(q) => match q.detail() {
                RPCQuestionDetail::StatusQ(q) => q,
                _ => panic!("not a status question"),
            },
            _ => panic!("not a question"),
        };

        // Ensure the node status from the question is the kind for the routing domain we received the request in
        match routing_domain {
            RoutingDomain::PublicInternet => {
                if !matches!(status_a.node_status, NodeStatus::PublicInternet(_)) {
                    return Ok(NetworkResult::invalid_message(
                        "node status doesn't match PublicInternet routing domain",
                    ));
                }
            }
            RoutingDomain::LocalNetwork => {
                if !matches!(status_a.node_status, NodeStatus::LocalNetwork(_)) {
                    return Ok(NetworkResult::invalid_message(
                        "node status doesn't match LocalNetwork routing domain",
                    ));
                }
            }
        }

        // update node status for the requesting node to our routing table
        if let Some(sender_nr) = msg.opt_sender_nr.clone() {
            // Update latest node status in routing table for the statusq sender
            sender_nr.update_node_status(status_q.node_status.clone());
        }

        // Make status answer
        let node_status = self.network_manager().generate_node_status(routing_domain);

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
            .answer(msg, RPCAnswer::new(RPCAnswerDetail::StatusA(status_a)))
            .await?;
        tracing::Span::current().record("res", &tracing::field::display(res));
        Ok(())
    }
}
