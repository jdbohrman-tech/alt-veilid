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
    #[instrument(level = "trace", skip(self), ret, err)]
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
        if matches!(
            dest,
            Destination::PrivateRoute {
                private_route: _,
                safety_selection: _
            }
        ) {
            return Err(RPCError::internal(
                "Never send set value requests over private routes",
            ));
        }

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
        let Some(vcrypto) = self.crypto.get(key.kind) else {
            return Err(RPCError::internal("unsupported cryptosystem"));
        };

        // Send the setvalue question
        let question_context = QuestionContext::SetValue(ValidateSetValueContext {
            descriptor,
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
        let set_value_a = match kind {
            RPCOperationKind::Answer(a) => match a.destructure() {
                RPCAnswerDetail::SetValueA(a) => a,
                _ => return Err(RPCError::invalid_format("not a setvalue answer")),
            },
            _ => return Err(RPCError::invalid_format("not an answer")),
        };

        let (set, value, peers) = set_value_a.destructure();

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
