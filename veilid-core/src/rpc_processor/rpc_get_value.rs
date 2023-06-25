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
    #[instrument(level = "trace", skip(self, last_descriptor), 
        fields(ret.value.data.len, 
            ret.value.data.seq, 
            ret.value.data.writer, 
            ret.peers.len,
            ret.latency
        ),err)]
    pub async fn rpc_call_get_value(
        self,
        dest: Destination,
        key: TypedKey,
        subkey: ValueSubkey,
        last_descriptor: Option<SignedValueDescriptor>,
    ) -> Result<NetworkResult<Answer<GetValueAnswer>>, RPCError> {
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
            "GetValue(key={} subkey={} last_descriptor={}) => {}",
            key,
            subkey,
            if last_descriptor.is_some() {
                "Some"
            } else {
                "None"
            },
            dest
        );

        // Send the getvalue question
        let get_value_q = RPCOperationGetValueQ::new(key, subkey, last_descriptor.is_none());
        let question = RPCQuestion::new(
            network_result_try!(self.get_destination_respond_to(&dest)?),
            RPCQuestionDetail::GetValueQ(get_value_q),
        );

        let question_context = QuestionContext::GetValue(ValidateGetValueContext {
            last_descriptor,
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
        let get_value_a = match kind {
            RPCOperationKind::Answer(a) => match a.destructure() {
                RPCAnswerDetail::GetValueA(a) => a,
                _ => return Ok(NetworkResult::invalid_message("not a getvalue answer")),
            },
            _ => return Ok(NetworkResult::invalid_message("not an answer")),
        };

        let (value, peers, descriptor) = get_value_a.destructure();

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

        tracing::Span::current().record("ret.latency", latency.as_u64());
        if let Some(value) = &value {
            tracing::Span::current().record("ret.value.data.len", value.value_data().data().len());
            tracing::Span::current().record("ret.value.data.seq", value.value_data().seq());
            tracing::Span::current().record("ret.value.data.writer", value.value_data().writer().to_string());
        }
        tracing::Span::current().record("ret.peers.len", peers.len());

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

        // Get the nodes that we know about that are closer to the the key than our own node
        let routing_table = self.routing_table();
        let closer_to_key_peers = network_result_try!(routing_table.find_peers_closer_to_key(key));

        // See if we have this record ourselves
        let storage_manager = self.storage_manager();
        let subkey_result = network_result_try!(storage_manager
            .inbound_get_value(key, subkey, want_descriptor)
            .await
            .map_err(RPCError::internal)?);

        // Make GetValue answer
        let get_value_a = RPCOperationGetValueA::new(
            subkey_result.value,
            closer_to_key_peers,
            subkey_result.descriptor,
        )?;

        // Send GetValue answer
        self.answer(msg, RPCAnswer::new(RPCAnswerDetail::GetValueA(get_value_a)))
            .await
    }
}
