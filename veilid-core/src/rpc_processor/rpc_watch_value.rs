use super::*;

#[derive(Clone, Debug)]
pub struct WatchValueAnswer {
    pub expiration_ts: Option<Timestamp>,
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
            fields(ret.expiration_ts,
                ret.latency
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
        let Some(target) = dest.target() else {
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
            vcrypto,
        )?;
        let question = RPCQuestion::new(
            network_result_try!(self.get_destination_respond_to(&dest)?),
            RPCQuestionDetail::WatchValueQ(Box::new(watch_value_q)),
        );

        #[cfg(feature = "debug-dht")]
        log_rpc!(debug "{}", debug_string);

        let waitable_reply =
            network_result_try!(self.question(dest.clone(), question, None).await?);
xxxxx continue here
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
        #[cfg(feature = "debug-dht")]
        {
            let debug_string_value = value
                .as_ref()
                .map(|v| {
                    format!(
                        " len={} seq={} writer={}",
                        v.value_data().data().len(),
                        v.value_data().seq(),
                        v.value_data().writer(),
                    )
                })
                .unwrap_or_default();

            let debug_string_answer = format!(
                "OUT <== GetValueA({} #{}{}{} peers={}) <= {}",
                key,
                subkey,
                debug_string_value,
                if descriptor.is_some() { " +desc" } else { "" },
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
            tracing::Span::current().record(
                "ret.value.data.writer",
                value.value_data().writer().to_string(),
            );
        }
        #[cfg(feature = "verbose-tracing")]
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

    #[cfg_attr(feature="verbose-tracing", instrument(level = "trace", skip(self, msg), fields(msg.operation.op_id), ret, err))]
    pub(crate) async fn process_watch_value_q(&self, msg: RPCMessage) -> RPCNetworkResult<()> {
        // Ignore if disabled
        let routing_table = self.routing_table();
        let opi = routing_table.get_own_peer_info(msg.header.routing_domain());
        if !opi.signed_node_info().node_info().has_capability(CAP_DHT) {
            return Ok(NetworkResult::service_unavailable("dht is not available"));
        }
        Err(RPCError::unimplemented("process_watch_value_q"))
    }
}
