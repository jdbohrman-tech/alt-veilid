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
    
    #[cfg_attr(
        feature = "verbose-tracing",        
        instrument(level = "trace", skip(self, last_descriptor), 
            fields(ret.value.data.len, 
                ret.value.data.seq, 
                ret.value.data.writer, 
                ret.peers.len,
                ret.latency
            ),err)
    )]
    pub async fn rpc_call_get_value(
        self,
        dest: Destination,
        key: TypedKey,
        subkey: ValueSubkey,
        last_descriptor: Option<SignedValueDescriptor>,
    ) ->RPCNetworkResult<Answer<GetValueAnswer>> {
        // Ensure destination never has a private route
        // and get the target noderef so we can validate the response
        let Some(target) = dest.node() else {
            return Err(RPCError::internal(
                "Never send get value requests over private routes",
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
            "OUT ==> GetValueQ({} #{}{}) => {}",
            key,
            subkey,
            if last_descriptor.is_some() {
                " +lastdesc"
            } else {
                ""
            },
            dest
        );

        // Send the getvalue question
        let get_value_q = RPCOperationGetValueQ::new(key, subkey, last_descriptor.is_none());
        let question = RPCQuestion::new(
            network_result_try!(self.get_destination_respond_to(&dest)?),
            RPCQuestionDetail::GetValueQ(Box::new(get_value_q)),
        );

        let question_context = QuestionContext::GetValue(ValidateGetValueContext {
            last_descriptor,
            subkey,
            vcrypto: vcrypto.clone(),
        });

        #[cfg(feature="debug-dht")]
        log_rpc!(debug "{}", debug_string);

        let waitable_reply = network_result_try!(
            self.question(dest.clone(), question, Some(question_context))
                .await?
        );

        // Keep the reply private route that was used to return with the answer
        let reply_private_route = waitable_reply.reply_private_route.clone();

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
        #[cfg(feature="debug-dht")]
        {   
            let debug_string_value = value.as_ref().map(|v| {
                format!(" len={} seq={} writer={}",
                    v.value_data().data().len(),
                    v.value_data().seq(),
                    v.value_data().writer(),
                )
            }).unwrap_or_default();
            
            let debug_string_answer = format!(
                "OUT <== GetValueA({} #{}{}{} peers={}) <= {}",
                key,
                subkey,
                debug_string_value,
                if descriptor.is_some() {
                    " +desc"
                } else {
                    ""
                },
                peers.len(),
                dest
            );

            log_rpc!(debug "{}", debug_string_answer);
            
            let peer_ids:Vec<String> = peers.iter().filter_map(|p| p.node_ids().get(key.kind).map(|k| k.to_string())).collect();
            log_rpc!(debug "Peers: {:#?}", peer_ids);
        }

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

        #[cfg(feature = "verbose-tracing")]
        tracing::Span::current().record("ret.latency", latency.as_u64());
        #[cfg(feature = "verbose-tracing")]
        if let Some(value) = &value {
            tracing::Span::current().record("ret.value.data.len", value.value_data().data().len());
            tracing::Span::current().record("ret.value.data.seq", value.value_data().seq());
            tracing::Span::current().record("ret.value.data.writer", value.value_data().writer().to_string());
        }
        #[cfg(feature = "verbose-tracing")]
        tracing::Span::current().record("ret.peers.len", peers.len());

        Ok(NetworkResult::value(Answer::new(
            latency,
            reply_private_route,
            GetValueAnswer {
                value,
                peers,
                descriptor,
            },
        )))
    }

    #[cfg_attr(feature="verbose-tracing", instrument(level = "trace", skip(self, msg), fields(msg.operation.op_id), ret, err))]
    pub(crate) async fn process_get_value_q(
        &self,
        msg: RPCMessage,
    ) ->RPCNetworkResult<()> {

        // Ensure this never came over a private route, safety route is okay though
        match &msg.header.detail {
            RPCMessageHeaderDetail::Direct(_) | RPCMessageHeaderDetail::SafetyRouted(_) => {}
            RPCMessageHeaderDetail::PrivateRouted(_) => {
                return Ok(NetworkResult::invalid_message(
                    "not processing get value request over private route",
                ))
            }
        }
        // Ignore if disabled
        let routing_table = self.routing_table();
        let opi = routing_table.get_own_peer_info(msg.header.routing_domain());
        if !opi
            .signed_node_info()
            .node_info()
            .has_capability(CAP_DHT)
        {
            return Ok(NetworkResult::service_unavailable(
                "dht is not available",
            ));
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
        let closer_to_key_peers = network_result_try!(routing_table.find_preferred_peers_closer_to_key(key, vec![CAP_DHT]));

        #[cfg(feature="debug-dht")]
        {   
            let debug_string = format!(
                "IN <=== GetValueQ({} #{}{}) <== {}",
                key,
                subkey,
                if want_descriptor {
                    " +wantdesc"
                } else {
                    ""
                },
                msg.header.direct_sender_node_id()
            );

            log_rpc!(debug "{}", debug_string);
        }

        // See if we have this record ourselves
        let storage_manager = self.storage_manager();
        let subkey_result = network_result_try!(storage_manager
            .inbound_get_value(key, subkey, want_descriptor)
            .await
            .map_err(RPCError::internal)?);
        
        #[cfg(feature="debug-dht")]
        {
            let debug_string_value = subkey_result.value.as_ref().map(|v| {
                format!(" len={} seq={} writer={}",
                    v.value_data().data().len(),
                    v.value_data().seq(),
                    v.value_data().writer(),
                )
            }).unwrap_or_default();

            let debug_string_answer = format!(
                "IN ===> GetValueA({} #{}{}{} peers={}) ==> {}",
                key,
                subkey,
                debug_string_value,
                if subkey_result.descriptor.is_some() {
                    " +desc"
                } else {
                    ""
                },
                closer_to_key_peers.len(),
                msg.header.direct_sender_node_id()
            );
        
            log_rpc!(debug "{}", debug_string_answer);
        }
            
        // Make GetValue answer
        let get_value_a = RPCOperationGetValueA::new(
            subkey_result.value,
            closer_to_key_peers,
            subkey_result.descriptor,
        )?;

        // Send GetValue answer
        self.answer(msg, RPCAnswer::new(RPCAnswerDetail::GetValueA(Box::new(get_value_a))))
            .await
    }
}
