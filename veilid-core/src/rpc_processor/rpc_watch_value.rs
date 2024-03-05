use super::*;

#[derive(Clone, Debug)]
pub struct WatchValueAnswer {
    pub accepted: bool,
    pub expiration_ts: Timestamp,
    pub peers: Vec<PeerInfo>,
    pub watch_id: u64,
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
    #[allow(clippy::too_many_arguments)]
    pub async fn rpc_call_watch_value(
        self,
        dest: Destination,
        key: TypedKey,
        subkeys: ValueSubkeyRangeSet,
        expiration: Timestamp,
        count: u32,
        watcher: KeyPair,
        watch_id: Option<u64>,
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
            "OUT ==> WatchValueQ({} {} {}@{}+{}) => {} (watcher={}) ",
            if let Some(watch_id) = watch_id {
                format!("id={} ", watch_id)
            } else {
                "".to_owned()
            },
            key,
            subkeys,
            expiration,
            count,
            dest,
            watcher.key
        );

        // Send the watchvalue question
        let watch_value_q = RPCOperationWatchValueQ::new(
            key,
            subkeys.clone(),
            expiration.as_u64(),
            count,
            watch_id,
            watcher,
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
        let reply_private_route = waitable_reply.reply_private_route;

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
        let question_watch_id = watch_id;
        let (accepted, expiration, peers, watch_id) = watch_value_a.destructure();
        #[cfg(feature = "debug-dht")]
        {
            let debug_string_answer = format!(
                "OUT <== WatchValueA({}id={} {} #{:?}@{} peers={}) <= {}",
                if accepted { "+accept " } else { "" },
                watch_id,
                key,
                subkeys,
                expiration,
                peers.len(),
                dest
            );

            log_rpc!(debug "{}", debug_string_answer);

            let peer_ids: Vec<String> = peers
                .iter()
                .filter_map(|p| p.node_ids().get(key.kind).map(|k| k.to_string()))
                .collect();
            log_rpc!(debug "Peers: {:#?}", peer_ids);
        }

        // Validate accepted requests
        if accepted {
            xxx does this make sense?
            
            // Verify returned answer watch id is the same as the question watch id if it exists
            if let Some(question_watch_id) = question_watch_id {
                if question_watch_id != watch_id {
                    return Ok(NetworkResult::invalid_message(format!(
                        "answer watch id={} doesn't match question watch id={}",
                        watch_id, question_watch_id,
                    )));
                }
            }
            // Validate if a watch is created/updated, that it has a nonzero id
            if expiration != 0 && watch_id == 0 {
                return Ok(NetworkResult::invalid_message(
                    "zero watch id returned on accepted or cancelled watch",
                ));
            }
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
                accepted,
                expiration_ts: Timestamp::new(expiration),
                peers,
                watch_id,
            },
        )))
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////

    #[cfg_attr(feature="verbose-tracing", instrument(level = "trace", skip(self, msg), fields(msg.operation.op_id), ret, err))]
    pub(crate) async fn process_watch_value_q(&self, msg: RPCMessage) -> RPCNetworkResult<()> {
        let routing_table = self.routing_table();
        let rss = routing_table.route_spec_store();

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
        if !opi
            .signed_node_info()
            .node_info()
            .has_capability(CAP_DHT_WATCH)
        {
            return Ok(NetworkResult::service_unavailable(
                "dht watch is not available",
            ));
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
        let (key, subkeys, expiration, count, watch_id, watcher, _signature) =
            watch_value_q.destructure();

        // Get target for ValueChanged notifications
        let dest = network_result_try!(self.get_respond_to_destination(&msg));
        let target = dest.get_target(rss)?;

        #[cfg(feature = "debug-dht")]
        {
            let debug_string = format!(
                "IN <=== WatchValueQ({}{} {}@{}+{}) <== {} (watcher={})",
                if let Some(watch_id) = watch_id {
                    format!("id={} ", watch_id)
                } else {
                    "".to_owned()
                },
                key,
                subkeys,
                expiration,
                count,
                msg.header.direct_sender_node_id(),
                watcher
            );

            log_rpc!(debug "{}", debug_string);
        }

        // Get the nodes that we know about that are closer to the the key than our own node
        let closer_to_key_peers = network_result_try!(
            routing_table.find_preferred_peers_closer_to_key(key, vec![CAP_DHT, CAP_DHT_WATCH])
        );

        // See if we would have accepted this as a set, same set_value_count for watches
        let set_value_count = {
            let c = self.config.get();
            c.network.dht.set_value_count as usize
        };
        let (ret_accepted, ret_expiration, ret_watch_id) =
            if closer_to_key_peers.len() >= set_value_count {
                // Not close enough, not accepted

                #[cfg(feature = "debug-dht")]
                log_rpc!(debug "Not close enough for watch value");

                (false, Timestamp::default(), watch_id.unwrap_or_default())
            } else {
                // Accepted, lets try to watch or cancel it

                // See if we have this record ourselves, if so, accept the watch
                let storage_manager = self.storage_manager();
                let (ret_expiration, ret_watch_id) = network_result_try!(storage_manager
                    .inbound_watch_value(
                        key,
                        subkeys.clone(),
                        Timestamp::new(expiration),
                        count,
                        watch_id,
                        target,
                        watcher
                    )
                    .await
                    .map_err(RPCError::internal)?);
                (true, ret_expiration, ret_watch_id)
            };

        #[cfg(feature = "debug-dht")]
        {
            let debug_string_answer = format!(
                "IN ===> WatchValueA({}id={} {} #{} expiration={} peers={}) ==> {}",
                if ret_accepted { "+accept " } else { "" },
                ret_watch_id,
                key,
                subkeys,
                ret_expiration,
                closer_to_key_peers.len(),
                msg.header.direct_sender_node_id()
            );

            log_rpc!(debug "{}", debug_string_answer);
        }

        // Make WatchValue answer
        let watch_value_a = RPCOperationWatchValueA::new(
            ret_accepted,
            ret_expiration.as_u64(),
            closer_to_key_peers,
            ret_watch_id,
        )?;

        // Send GetValue answer
        self.answer(
            msg,
            RPCAnswer::new(RPCAnswerDetail::WatchValueA(Box::new(watch_value_a))),
        )
        .await
    }
}
