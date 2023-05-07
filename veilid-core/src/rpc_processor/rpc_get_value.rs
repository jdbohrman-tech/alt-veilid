use super::*;
use crate::storage_manager::{SignedValueData, SignedValueDescriptor};

#[derive(Clone, Debug)]
pub struct GetValueAnswer {
    pub value: Option<SignedValueData>,
    pub peers: Vec<PeerInfo>,
    pub descriptor: Option<SignedValueDescriptor>,
}

impl RPCProcessor {
    /// Sends a get value request and wait for response
    /// Can be sent via all methods including relays
    /// Safety routes may be used, but never private routes.
    /// Because this leaks information about the identity of the node itself,
    /// replying to this request received over a private route will leak
    /// the identity of the node and defeat the private route.
    #[instrument(level = "trace", skip(self), ret, err)]
    pub async fn rpc_call_get_value(
        self,
        dest: Destination,
        key: TypedKey,
        subkey: ValueSubkey,
        last_descriptor: Option<SignedValueDescriptor>,
    ) -> Result<NetworkResult<Answer<GetValueAnswer>>, RPCError> {
        // Ensure destination never has a private route
        if matches!(
            dest,
            Destination::PrivateRoute {
                private_route: _,
                safety_selection: _
            }
        ) {
            return Err(RPCError::internal(
                "Never send get value requests over private routes",
            ));
        }

        let get_value_q = RPCOperationGetValueQ::new(key, subkey, last_descriptor.is_none());
        let question = RPCQuestion::new(
            network_result_try!(self.get_destination_respond_to(&dest)?),
            RPCQuestionDetail::GetValueQ(get_value_q),
        );
        let Some(vcrypto) = self.crypto.get(key.kind) else {
            return Err(RPCError::internal("unsupported cryptosystem"));
        };

        // Send the app call question
        let question_context = QuestionContext::GetValue(ValidateGetValueContext {
            last_descriptor,
            subkey,
            vcrypto,
        });

        let waitable_reply = network_result_try!(
            self.question(dest, question, Some(question_context))
                .await?
        );

        // Wait for reply
        let (msg, latency) = match self.wait_for_reply(waitable_reply).await? {
            TimeoutOr::Timeout => return Ok(NetworkResult::Timeout),
            TimeoutOr::Value(v) => v,
        };

        // Get the right answer type
        let (_, _, _, kind) = msg.operation.destructure();
        let get_value_a = match kind {
            RPCOperationKind::Answer(a) => match a.destructure() {
                RPCAnswerDetail::GetValueA(a) => a,
                _ => return Err(RPCError::invalid_format("not a getvalue answer")),
            },
            _ => return Err(RPCError::invalid_format("not an answer")),
        };

        let (value, peers, descriptor) = get_value_a.destructure();

        Ok(NetworkResult::value(Answer::new(
            latency,
            GetValueAnswer {
                value,
                peers,
                descriptor,
            },
        )))
    }

    #[instrument(level = "trace", skip(self, msg), fields(msg.operation.op_id), ret, err)]
    pub(crate) async fn process_get_value_q(
        &self,
        msg: RPCMessage,
    ) -> Result<NetworkResult<()>, RPCError> {
        // Ensure this never came over a private route, safety route is okay though
        match &msg.header.detail {
            RPCMessageHeaderDetail::Direct(_) | RPCMessageHeaderDetail::SafetyRouted(_) => {}
            RPCMessageHeaderDetail::PrivateRouted(_) => {
                return Ok(NetworkResult::invalid_message(
                    "not processing get value request over private route",
                ))
            }
        }

        // Get the question
        let kind = msg.operation.kind().clone();
        let get_value_q = match kind {
            RPCOperationKind::Question(q) => match q.destructure() {
                (_, RPCQuestionDetail::GetValueQ(q)) => q,
                _ => panic!("not a getvalue question"),
            },
            _ => panic!("not a question"),
        };

        // Destructure
        let (key, subkey, want_descriptor) = get_value_q.destructure();

        // add node information for the requesting node to our routing table
        let crypto_kind = key.kind;
        let routing_table = self.routing_table();
        let own_node_id = routing_table.node_id(crypto_kind);

        // find N nodes closest to the target node in our routing table
        // ensure the nodes returned are only the ones closer to the target node than ourself
        let Some(vcrypto) = self.crypto.get(crypto_kind) else {
            return Ok(NetworkResult::invalid_message("unsupported cryptosystem"));
        };
        let own_distance = vcrypto.distance(&own_node_id.value, &key.value);

        let filter = Box::new(
            move |rti: &RoutingTableInner, opt_entry: Option<Arc<BucketEntry>>| {
                // Exclude our own node
                let Some(entry) = opt_entry else {
                    return false;
                };
                // Ensure only things that are valid/signed in the PublicInternet domain are returned
                if !rti.filter_has_valid_signed_node_info(
                    RoutingDomain::PublicInternet,
                    true,
                    Some(entry.clone()),
                ) {
                    return false;
                }
                // Ensure things further from the key than our own node are not included
                let Some(entry_node_id) = entry.with(rti, |_rti, e| e.node_ids().get(crypto_kind)) else {
                    return false;
                };
                let entry_distance = vcrypto.distance(&entry_node_id.value, &key.value);
                if entry_distance >= own_distance {
                    return false;
                }

                true
            },
        ) as RoutingTableEntryFilter;
        let filters = VecDeque::from([filter]);

        let node_count = {
            let c = self.config.get();
            c.network.dht.max_find_node_count as usize
        };

        //
        let closest_nodes = routing_table.find_closest_nodes(
            node_count,
            key,
            filters,
            // transform
            |rti, entry| {
                entry.unwrap().with(rti, |_rti, e| {
                    e.make_peer_info(RoutingDomain::PublicInternet).unwrap()
                })
            },
        );

        // Make status answer
        let find_node_a = RPCOperationFindNodeA::new(closest_nodes)?;

        // Send status answer
        self.answer(msg, RPCAnswer::new(RPCAnswerDetail::FindNodeA(find_node_a)))
            .await
    }
}
