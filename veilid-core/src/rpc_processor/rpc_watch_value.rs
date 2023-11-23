use super::*;

#[derive(Clone, Debug)]
pub struct WatchValueAnswer {
    pub expiration_ts: Timestamp,
    pub peers: Vec<PeerInfo>,
}

impl RPCProcessor {
    /// Sends a watch value request and wait for response
    /// Can be sent via all methods including relays
    /// Safety routes may be used, but never private routes.
    /// Because this leaks information about the identity of the node itself,
    /// replying to this request received over a private route will leak
    /// the identity of the node and defeat the private route.

    #[cfg_attr(
        feature = "verbose-tracing",        
        instrument(level = "trace", skip(self), 
            fields(ret.expiration,
                ret.latency,
                ret.peers.len
            ),err)
    )]
    pub async fn rpc_call_watch_value(
        self,
        dest: Destination,
        key: TypedKey,
        subkeys: ValueSubkeyRangeSet,
        expiration: Timestamp,
        count: u32,
        opt_watcher: Option<KeyPair>,
    ) -> RPCNetworkResult<Answer<WatchValueAnswer>> {
        // Ensure destination never has a private route
        // and get the target noderef so we can validate the response
        let Some(target) = dest.node() else {
            return Err(RPCError::internal(
                "Never send watch value requests over private routes",
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
            "OUT ==> WatchValueQ({} {}#{:?}@{}+{}) => {}",
            key,
            if opt_watcher.is_some() { "+W " } else { "" },
            subkeys,
            expiration,
            count,
            dest
        );

        // Send the watchvalue question
        let watch_value_q = RPCOperationWatchValueQ::new(
            key,
            subkeys,
            expiration.as_u64(),
            count,
            opt_watcher,
            vcrypto.clone(),
        )?;
        let question = RPCQuestion::new(
            network_result_try!(self.get_destination_respond_to(&dest)?),
            RPCQuestionDetail::WatchValueQ(Box::new(watch_value_q)),
        );

        #[cfg(feature = "debug-dht")]
        log_rpc!(debug "{}", debug_string);

        let waitable_reply =
            network_result_try!(self.question(dest.clone(), question, None).await?);

        // Keep the reply private route that was used to return with the answer
        let reply_private_route = waitable_reply.reply_private_route.clone();

        // Wait for reply
        let (msg, latency) = match self.wait_for_reply(waitable_reply, debug_string).await? {
            TimeoutOr::Timeout => return Ok(NetworkResult::Timeout),
            TimeoutOr::Value(v) => v,
        };

        // Get the right answer type
        let (_, _, _, kind) = msg.operation.destructure();
        let watch_value_a = match kind {
            RPCOperationKind::Answer(a) => match a.destructure() {
                RPCAnswerDetail::WatchValueA(a) => a,
                _ => return Ok(NetworkResult::invalid_message("not a watchvalue answer")),
            },
            _ => return Ok(NetworkResult::invalid_message("not an answer")),
        };

        let (expiration, peers) = watch_value_a.destructure();
        #[cfg(feature = "debug-dht")]
        {
            let debug_string_answer = format!(
                "OUT <== WatchValueA({} {}#{:?}@{} peers={}) <= {}",
                key,
                if opt_watcher.is_some() { "+W " } else { "" },
                subkeys,
                expiration,
                peer.len()
                dest
            );

            log_rpc!(debug "{}", debug_string_answer);

            let peer_ids: Vec<String> = peers
                .iter()
                .filter_map(|p| p.node_ids().get(key.kind).map(|k| k.to_string()))
                .collect();
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
        tracing::Span::current().record("ret.expiration", latency.as_u64());
        #[cfg(feature = "verbose-tracing")]
        tracing::Span::current().record("ret.peers.len", peers.len());

        Ok(NetworkResult::value(Answer::new(
            latency,
            reply_private_route,
            WatchValueAnswer {
                expiration_ts: Timestamp::new(expiration),
                peers,
            },
        )))
    }

    #[cfg_attr(feature="verbose-tracing", instrument(level = "trace", skip(self, msg), fields(msg.operation.op_id), ret, err))]
    pub(crate) async fn process_watch_value_q(&self, msg: RPCMessage) -> RPCNetworkResult<()> {
        // Ensure this never came over a private route, safety route is okay though
        match &msg.header.detail {
            RPCMessageHeaderDetail::Direct(_) | RPCMessageHeaderDetail::SafetyRouted(_) => {}
            RPCMessageHeaderDetail::PrivateRouted(_) => {
                return Ok(NetworkResult::invalid_message(
                    "not processing watch value request over private route",
                ))
            }
        }

        // Ignore if disabled
        let routing_table = self.routing_table();
        let opi = routing_table.get_own_peer_info(msg.header.routing_domain());
        if !opi.signed_node_info().node_info().has_capability(CAP_DHT) {
            return Ok(NetworkResult::service_unavailable("dht is not available"));
        }

        // Get the question
        let kind = msg.operation.kind().clone();
        let watch_value_q = match kind {
            RPCOperationKind::Question(q) => match q.destructure() {
                (_, RPCQuestionDetail::WatchValueQ(q)) => q,
                _ => panic!("not a watchvalue question"),
            },
            _ => panic!("not a question"),
        };

        // Destructure
        let (key, subkeys, expiration, count, opt_watch_signature) = watch_value_q.destructure();
        let opt_watcher = opt_watch_signature.map(|ws| ws.0);

        // Get target for ValueChanged notifications
        let dest = network_result_try!(self.get_respond_to_destination(&msg));
        let target = dest.get_target();

        #[cfg(feature = "debug-dht")]
        {
            let debug_string = format!(
                "IN <=== WatchValueQ({} {}#{:?}@{}+{}) <== {}",
                key,
                if opt_watcher.is_some() { "+W " } else { "" },
                subkeys,
                expiration,
                count,
                msg.header.direct_sender_node_id()
            );

            log_rpc!(debug "{}", debug_string);
        }

        // See if we have this record ourselves, if so, accept the watch
        let storage_manager = self.storage_manager();
        let ret_expiration = network_result_try!(storage_manager
            .inbound_watch_value(
                key,
                subkeys,
                Timestamp::new(expiration),
                count,
                target,
                opt_watcher
            )
            .await
            .map_err(RPCError::internal)?);

        // Get the nodes that we know about that are closer to the the key than our own node
        let routing_table = self.routing_table();
        let closer_to_key_peers = if ret_expiration.as_u64() == 0 {
            network_result_try!(routing_table.find_preferred_peers_closer_to_key(key, vec![CAP_DHT]))
        } else {
            vec![]
        };

        #[cfg(feature = "debug-dht")]
        {
            let debug_string_answer = format!(
                "IN ===> WatchValueA({} #{} expiration={} peers={}) ==> {}",
                key,
                subkeys,
                ret_expiration,
                closer_to_key_peers.len(),
                msg.header.direct_sender_node_id()
            );

            log_rpc!(debug "{}", debug_string_answer);
        }

        // Make WatchValue answer
        let watch_value_a =
            RPCOperationWatchValueA::new(ret_expiration.as_u64(), closer_to_key_peers)?;

        // Send GetValue answer
        self.answer(
            msg,
            RPCAnswer::new(RPCAnswerDetail::WatchValueA(Box::new(watch_value_a))),
        )
        .await
    }
}
