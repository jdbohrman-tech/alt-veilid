use super::*;

#[derive(Clone, Debug)]
pub struct SetValueAnswer {
    pub set: bool,
    pub value: Option<SignedValueData>,
    pub peers: Vec<PeerInfo>,
}

impl RPCProcessor {
    /// Sends a set value request and wait for response
    /// Can be sent via all methods including relays
    /// Safety routes may be used, but never private routes.
    /// Because this leaks information about the identity of the node itself,
    /// replying to this request received over a private route will leak
    /// the identity of the node and defeat the private route.
    pub async fn rpc_call_set_value(
        self,
        dest: Destination,
        key: TypedKey,
        subkey: ValueSubkey,
        value: SignedValueData,
        descriptor: SignedValueDescriptor,
        send_descriptor: bool,
    ) -> Result<NetworkResult<Answer<SetValueAnswer>>, RPCError> {
        // Ensure destination never has a private route
        // and get the target noderef so we can validate the response
        let Some(target) = dest.target() else {
            return Err(RPCError::internal(
                "Never send set value requests over private routes",
            ));
        };

        // Get the target node id
        let Some(vcrypto) = self.crypto.get(key.kind) else {
            return Err(RPCError::internal("unsupported cryptosystem"));
        };
        let Some(target_node_id) = target.node_ids().get(key.kind) else {
            return Err(RPCError::internal("No node id for crypto kind"));
        };

        let debug_string = format!(
            "SetValue(key={} subkey={} value_data(writer)={} value_data(len)={} send_descriptor={}) => {}",
            key,
            subkey,
            value.value_data().writer(),
            value.value_data().data().len(),
            send_descriptor,
            dest
        );

        // Send the setvalue question
        let set_value_q = RPCOperationSetValueQ::new(
            key,
            subkey,
            value,
            if send_descriptor {
                Some(descriptor.clone())
            } else {
                None
            },
        );
        let question = RPCQuestion::new(
            network_result_try!(self.get_destination_respond_to(&dest)?),
            RPCQuestionDetail::SetValueQ(set_value_q),
        );
        let question_context = QuestionContext::SetValue(ValidateSetValueContext {
            descriptor,
            subkey,
            vcrypto: vcrypto.clone(),
        });

        let waitable_reply = network_result_try!(
            self.question(dest, question, Some(question_context))
                .await?
        );

        // Wait for reply
        let (msg, latency) = match self.wait_for_reply(waitable_reply, debug_string).await? {
            TimeoutOr::Timeout => return Ok(NetworkResult::Timeout),
            TimeoutOr::Value(v) => v,
        };

        // Get the right answer type
        let (_, _, _, kind) = msg.operation.destructure();
        let set_value_a = match kind {
            RPCOperationKind::Answer(a) => match a.destructure() {
                RPCAnswerDetail::SetValueA(a) => a,
                _ => return Ok(NetworkResult::invalid_message("not a setvalue answer")),
            },
            _ => return Ok(NetworkResult::invalid_message("not an answer")),
        };

        let (set, value, peers) = set_value_a.destructure();

        // Validate peers returned are, in fact, closer to the key than the node we sent this to
        let valid = match RoutingTable::verify_peers_closer(vcrypto, target_node_id, key, &peers) {
            Ok(v) => v,
            Err(e) => {
                return Ok(NetworkResult::invalid_message(format!(
                    "missing cryptosystem in peers node ids: {}",
                    e
                )));
            }
        };
        if !valid {
            return Ok(NetworkResult::invalid_message("non-closer peers returned"));
        }

        Ok(NetworkResult::value(Answer::new(
            latency,
            SetValueAnswer { set, value, peers },
        )))
    }

    #[instrument(level = "trace", skip(self, msg), fields(msg.operation.op_id), ret, err)]
    pub(crate) async fn process_set_value_q(
        &self,
        msg: RPCMessage,
    ) -> Result<NetworkResult<()>, RPCError> {
        // Ensure this never came over a private route, safety route is okay though
        match &msg.header.detail {
            RPCMessageHeaderDetail::Direct(_) | RPCMessageHeaderDetail::SafetyRouted(_) => {}
            RPCMessageHeaderDetail::PrivateRouted(_) => {
                return Ok(NetworkResult::invalid_message(
                    "not processing set value request over private route",
                ))
            }
        }

        // Get the question
        let kind = msg.operation.kind().clone();
        let set_value_q = match kind {
            RPCOperationKind::Question(q) => match q.destructure() {
                (_, RPCQuestionDetail::SetValueQ(q)) => q,
                _ => panic!("not a setvalue question"),
            },
            _ => panic!("not a question"),
        };

        // Destructure
        let (key, subkey, value, descriptor) = set_value_q.destructure();

        // Get the nodes that we know about that are closer to the the key than our own node
        let routing_table = self.routing_table();
        let closer_to_key_peers = network_result_try!(routing_table.find_peers_closer_to_key(key));

        // If there are less than 'set_value_count' peers that are closer, then store here too
        let set_value_count = {
            let c = self.config.get();
            c.network.dht.set_value_fanout as usize
        };
        let (set, new_value) = if closer_to_key_peers.len() >= set_value_count {
            // Not close enough
            (false, None)
        } else {
            // Close enough, lets set it

            // Save the subkey, creating a new record if necessary
            let storage_manager = self.storage_manager();
            let new_value = network_result_try!(storage_manager
                .inbound_set_value(key, subkey, value, descriptor)
                .await
                .map_err(RPCError::internal)?);

            (true, new_value)
        };

        // Make SetValue answer
        let set_value_a = RPCOperationSetValueA::new(set, new_value, closer_to_key_peers)?;

        // Send SetValue answer
        self.answer(msg, RPCAnswer::new(RPCAnswerDetail::SetValueA(set_value_a)))
            .await
    }
}
