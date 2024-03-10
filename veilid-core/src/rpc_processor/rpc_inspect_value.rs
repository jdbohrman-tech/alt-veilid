use super::*;
use crate::storage_manager::SignedValueDescriptor;

#[derive(Clone, Debug)]
pub struct InspectValueAnswer {
    pub seqs: Vec<ValueSeqNum>,
    pub peers: Vec<PeerInfo>,
    pub descriptor: Option<SignedValueDescriptor>,
}

impl RPCProcessor {
    /// Sends an inspect value request and wait for response
    /// Can be sent via all methods including relays
    /// Safety routes may be used, but never private routes.
    /// Because this leaks information about the identity of the node itself,
    /// replying to this request received over a private route will leak
    /// the identity of the node and defeat the private route.
    
    #[cfg_attr(
        feature = "verbose-tracing",        
        instrument(level = "trace", skip(self, last_descriptor), 
            fields(ret.value.data.len, 
                ret.seqs, 
                ret.peers.len,
                ret.latency
            ),err)
    )]
    pub async fn rpc_call_inspect_value(
        self,
        dest: Destination,
        key: TypedKey,
        subkeys: ValueSubkeyRangeSet,
        last_descriptor: Option<SignedValueDescriptor>,
    ) -> RPCNetworkResult<Answer<InspectValueAnswer>> {
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
            "OUT ==> InspectValueQ({} #{}{}) => {}",
            key,
            &subkeys,
            if last_descriptor.is_some() {
                " +lastdesc"
            } else {
                ""
            },
            dest
        );

        // Send the inspectvalue question
        let inspect_value_q = RPCOperationInspectValueQ::new(key, subkeys.clone(), last_descriptor.is_none())?;
        let question = RPCQuestion::new(
            network_result_try!(self.get_destination_respond_to(&dest)?),
            RPCQuestionDetail::InspectValueQ(Box::new(inspect_value_q)),
        );

        let question_context = QuestionContext::InspectValue(ValidateInspectValueContext {
            last_descriptor,
            subkeys,
            vcrypto: vcrypto.clone(),
        });

        log_dht!(debug "{}", debug_string);

        let waitable_reply = network_result_try!(
            self.question(dest.clone(), question, Some(question_context))
                .await?
        );

        // Keep the reply private route that was used to return with the answer
        let reply_private_route = waitable_reply.reply_private_route;

        // Wait for reply
        let (msg, latency) = match self.wait_for_reply(waitable_reply, debug_string).await? {
            TimeoutOr::Timeout => return Ok(NetworkResult::Timeout),
            TimeoutOr::Value(v) => v,
        };

        // Get the right answer type
        let (_, _, _, kind) = msg.operation.destructure();
        let inspect_value_a = match kind {
            RPCOperationKind::Answer(a) => match a.destructure() {
                RPCAnswerDetail::InspectValueA(a) => a,
                _ => return Ok(NetworkResult::invalid_message("not an inspectvalue answer")),
            },
            _ => return Ok(NetworkResult::invalid_message("not an answer")),
        };

        let (seqs, peers, descriptor) = inspect_value_a.destructure();
        if debug_target_enabled!("dht") {
            let debug_string_answer = format!(
                "OUT <== InspectValueA({} {:?}{} peers={}) <= {}",
                key,
                seqs,
                if descriptor.is_some() {
                    " +desc"
                } else {
                    ""
                },
                peers.len(),
                dest
            );

            log_dht!(debug "{}", debug_string_answer);
            
            let peer_ids:Vec<String> = peers.iter().filter_map(|p| p.node_ids().get(key.kind).map(|k| k.to_string())).collect();
            log_dht!(debug "Peers: {:#?}", peer_ids);
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
        tracing::Span::current().record("ret.seqs", seqs);
        #[cfg(feature = "verbose-tracing")]
        tracing::Span::current().record("ret.peers.len", peers.len());

        Ok(NetworkResult::value(Answer::new(
            latency,
            reply_private_route,
            InspectValueAnswer {
                seqs,
                peers,
                descriptor,
            },
        )))
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////

    #[cfg_attr(feature="verbose-tracing", instrument(level = "trace", skip(self, msg), fields(msg.operation.op_id), ret, err))]
    pub(crate) async fn process_inspect_value_q(
        &self,
        msg: RPCMessage,
    ) -> RPCNetworkResult<()> {

        // Ensure this never came over a private route, safety route is okay though
        match &msg.header.detail {
            RPCMessageHeaderDetail::Direct(_) | RPCMessageHeaderDetail::SafetyRouted(_) => {}
            RPCMessageHeaderDetail::PrivateRouted(_) => {
                return Ok(NetworkResult::invalid_message(
                    "not processing inspect value request over private route",
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
        let inspect_value_q = match kind {
            RPCOperationKind::Question(q) => match q.destructure() {
                (_, RPCQuestionDetail::InspectValueQ(q)) => q,
                _ => panic!("not a inspectvalue question"),
            },
            _ => panic!("not a question"),
        };

        // Destructure
        let (key, subkeys, want_descriptor) = inspect_value_q.destructure();

        // Get the nodes that we know about that are closer to the the key than our own node
        let routing_table = self.routing_table();
        let closer_to_key_peers = network_result_try!(routing_table.find_preferred_peers_closer_to_key(key, vec![CAP_DHT, CAP_DHT_WATCH]));

        if debug_target_enabled!("dht") {
            let debug_string = format!(
                "IN <=== InspectValueQ({} {}{}) <== {}",
                key,
                subkeys,
                if want_descriptor {
                    " +wantdesc"
                } else {
                    ""
                },
                msg.header.direct_sender_node_id()
            );

            log_dht!(debug "{}", debug_string);
        }

        // See if we would have accepted this as a set
        let set_value_count = {
            let c = self.config.get();
            c.network.dht.set_value_count as usize
        };
        let (inspect_result_seqs, inspect_result_descriptor) = if closer_to_key_peers.len() >= set_value_count {
            // Not close enough
            (Vec::new(), None)
        } else {
            // Close enough, lets get it

            // See if we have this record ourselves
            let storage_manager = self.storage_manager();
            let inspect_result = network_result_try!(storage_manager
                .inbound_inspect_value(key, subkeys, want_descriptor)
                .await
                .map_err(RPCError::internal)?);
            (inspect_result.seqs, inspect_result.opt_descriptor)
        };

        if debug_target_enabled!("dht") {
            let debug_string_answer = format!(
                "IN ===> InspectValueA({} {:?}{} peers={}) ==> {}",
                key,
                inspect_result_seqs,
                if inspect_result_descriptor.is_some() {
                    " +desc"
                } else {
                    ""
                },
                closer_to_key_peers.len(),
                msg.header.direct_sender_node_id()
            );
        
            log_dht!(debug "{}", debug_string_answer);
        }
            
        // Make InspectValue answer
        let inspect_value_a = RPCOperationInspectValueA::new(
            inspect_result_seqs,
            closer_to_key_peers,
            inspect_result_descriptor.map(|x| (*x).clone()),
        )?;

        // Send InspectValue answer
        self.answer(msg, RPCAnswer::new(RPCAnswerDetail::InspectValueA(Box::new(inspect_value_a))))
            .await
    }
}
