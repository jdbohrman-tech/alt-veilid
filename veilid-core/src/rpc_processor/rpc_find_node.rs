use super::*;

impl RPCProcessor {
    /// Send FindNodeQ RPC request, receive FindNodeA answer
    /// Can be sent via all methods including relays
    /// Safety routes may be used, but never private routes.
    /// Because this leaks information about the identity of the node itself,
    /// replying to this request received over a private route will leak
    /// the identity of the node and defeat the private route.
    #[instrument(level = "trace", skip(self), ret, err)]
    pub async fn rpc_call_find_node(
        self,
        dest: Destination,
        key: DHTKey,
    ) -> Result<NetworkResult<Answer<Vec<PeerInfo>>>, RPCError> {
        // Ensure destination never has a private route
        if matches!(
            dest,
            Destination::PrivateRoute {
                private_route: _,
                safety_selection: _
            }
        ) {
            return Err(RPCError::internal(
                "Never send find node requests over private routes",
            ));
        }

        let find_node_q_detail =
            RPCQuestionDetail::FindNodeQ(RPCOperationFindNodeQ { node_id: key });
        let find_node_q = RPCQuestion::new(RespondTo::Sender, find_node_q_detail);

        // Send the find_node request
        let waitable_reply = network_result_try!(self.question(dest, find_node_q).await?);

        // Wait for reply
        let (msg, latency) = match self.wait_for_reply(waitable_reply).await? {
            TimeoutOr::Timeout => return Ok(NetworkResult::Timeout),
            TimeoutOr::Value(v) => v,
        };

        // Get the right answer type
        let find_node_a = match msg.operation.into_kind() {
            RPCOperationKind::Answer(a) => match a.into_detail() {
                RPCAnswerDetail::FindNodeA(a) => a,
                _ => return Err(RPCError::invalid_format("not a find_node answer")),
            },
            _ => return Err(RPCError::invalid_format("not an answer")),
        };

        // Verify peers are in the correct peer scope
        for peer_info in &find_node_a.peers {
            if !self.filter_node_info(
                RoutingDomain::PublicInternet,
                &peer_info.signed_node_info.node_info,
            ) {
                return Err(RPCError::invalid_format(
                    "find_node response has invalid peer scope",
                ));
            }
        }

        Ok(NetworkResult::value(Answer::new(
            latency,
            find_node_a.peers,
        )))
    }

    #[instrument(level = "trace", skip(self, msg), fields(msg.operation.op_id, res), err)]
    pub(crate) async fn process_find_node_q(&self, msg: RPCMessage) -> Result<(), RPCError> {
        // Ensure this never came over a private route
        match msg.header.detail {
            RPCMessageHeaderDetail::Direct(_) => todo!(),
            RPCMessageHeaderDetail::PrivateRouted(_) => todo!(),
        }
        if matches!(
            dest,
            Destination::PrivateRoute {
                private_route: _,
                safety_selection: _
            }
        ) {
            return Err(RPCError::internal(
                "Never send find node requests over private routes",
            ));
        }

        // Get the question
        let find_node_q = match msg.operation.kind() {
            RPCOperationKind::Question(q) => match q.detail() {
                RPCQuestionDetail::FindNodeQ(q) => q,
                _ => panic!("not a status question"),
            },
            _ => panic!("not a question"),
        };

        // add node information for the requesting node to our routing table
        let routing_table = self.routing_table();
        let has_valid_own_node_info =
            routing_table.has_valid_own_node_info(RoutingDomain::PublicInternet);
        let own_peer_info = routing_table.get_own_peer_info(RoutingDomain::PublicInternet);

        // find N nodes closest to the target node in our routing table

        let filter = Box::new(
            move |rti: &RoutingTableInner, _k: DHTKey, v: Option<Arc<BucketEntry>>| {
                rti.filter_has_valid_signed_node_info(
                    RoutingDomain::PublicInternet,
                    has_valid_own_node_info,
                    v,
                )
            },
        ) as RoutingTableEntryFilter;
        let filters = VecDeque::from([filter]);

        let closest_nodes = routing_table.find_closest_nodes(
            find_node_q.node_id,
            filters,
            // transform
            |rti, k, v| {
                rti.transform_to_peer_info(
                    RoutingDomain::PublicInternet,
                    own_peer_info.clone(),
                    k,
                    v,
                )
            },
        );

        // Make status answer
        let find_node_a = RPCOperationFindNodeA {
            peers: closest_nodes,
        };

        // Send status answer
        let res = self
            .answer(msg, RPCAnswer::new(RPCAnswerDetail::FindNodeA(find_node_a)))
            .await?;
        tracing::Span::current().record("res", &tracing::field::display(res));
        Ok(())
    }
}
