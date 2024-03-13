use super::*;

/// The fully parsed descriptor
struct DescriptorInfo {
    /// The descriptor itself
    descriptor: Arc<SignedValueDescriptor>,

    /// The in-schema subkeys that overlap the inspected range
    subkeys: ValueSubkeyRangeSet,
}

impl DescriptorInfo {
    pub fn new(
        descriptor: Arc<SignedValueDescriptor>,
        subkeys: &ValueSubkeyRangeSet,
    ) -> VeilidAPIResult<Self> {
        let schema = descriptor.schema().map_err(RPCError::invalid_format)?;
        let subkeys = schema.truncate_subkeys(subkeys, Some(MAX_INSPECT_VALUE_A_SEQS_LEN));
        Ok(Self {
            descriptor,
            subkeys,
        })
    }
}

/// Info tracked per subkey
struct SubkeySeqCount {
    /// The newest sequence number found for a subkey
    pub seq: ValueSeqNum,
    /// The nodes that have returned the value so far (up to the consensus count)
    pub value_nodes: Vec<NodeRef>,
}

/// The context of the outbound_get_value operation
struct OutboundInspectValueContext {
    /// The combined sequence numbers and result counts so far
    pub seqcounts: Vec<SubkeySeqCount>,
    /// The descriptor if we got a fresh one or empty if no descriptor was needed
    pub opt_descriptor_info: Option<DescriptorInfo>,
}

/// The result of the outbound_get_value operation
pub(super) struct OutboundInspectValueResult {
    /// Fanout results for each subkey
    pub fanout_results: Vec<FanoutResult>,
    /// The inspection that was retrieved
    pub inspect_result: InspectResult,
}

impl StorageManager {
    /// Perform a 'inspect value' query on the network
    pub(super) async fn outbound_inspect_value(
        &self,
        rpc_processor: RPCProcessor,
        key: TypedKey,
        subkeys: ValueSubkeyRangeSet,
        safety_selection: SafetySelection,
        local_inspect_result: InspectResult,
        use_set_scope: bool,
    ) -> VeilidAPIResult<OutboundInspectValueResult> {
        let routing_table = rpc_processor.routing_table();

        // Get the DHT parameters for 'InspectValue'
        // Can use either 'get scope' or 'set scope' depending on the purpose of the inspection
        let (key_count, consensus_count, fanout, timeout_us) = {
            let c = self.unlocked_inner.config.get();

            if use_set_scope {
                (
                    c.network.dht.max_find_node_count as usize,
                    c.network.dht.set_value_count as usize,
                    c.network.dht.set_value_fanout as usize,
                    TimestampDuration::from(ms_to_us(c.network.dht.set_value_timeout_ms)),
                )
            } else {
                (
                    c.network.dht.max_find_node_count as usize,
                    c.network.dht.get_value_count as usize,
                    c.network.dht.get_value_fanout as usize,
                    TimestampDuration::from(ms_to_us(c.network.dht.get_value_timeout_ms)),
                )
            }
        };

        // Make do-inspect-value answer context
        let opt_descriptor_info = if let Some(descriptor) = &local_inspect_result.opt_descriptor {
            // Get the descriptor info. This also truncates the subkeys list to what can be returned from the network.
            Some(DescriptorInfo::new(descriptor.clone(), &subkeys)?)
        } else {
            None
        };

        let context = Arc::new(Mutex::new(OutboundInspectValueContext {
            seqcounts: local_inspect_result
                .seqs
                .iter()
                .map(|s| SubkeySeqCount {
                    seq: *s,
                    value_nodes: vec![],
                })
                .collect(),
            opt_descriptor_info,
        }));

        // Routine to call to generate fanout
        let call_routine = |next_node: NodeRef| {
            let rpc_processor = rpc_processor.clone();
            let context = context.clone();
            let opt_descriptor = local_inspect_result.opt_descriptor.clone();
            let subkeys = subkeys.clone();
            async move {
                let iva = network_result_try!(
                    rpc_processor
                        .clone()
                        .rpc_call_inspect_value(
                            Destination::direct(next_node.clone()).with_safety(safety_selection),
                            key,
                            subkeys.clone(),
                            opt_descriptor.map(|x| (*x).clone()),
                        )
                        .await?
                );
                let answer = iva.answer;

                // Keep the descriptor if we got one. If we had a last_descriptor it will
                // already be validated by rpc_call_inspect_value
                if let Some(descriptor) = answer.descriptor {
                    let mut ctx = context.lock();
                    if ctx.opt_descriptor_info.is_none() {
                        // Get the descriptor info. This also truncates the subkeys list to what can be returned from the network.
                        let descriptor_info =
                            match DescriptorInfo::new(Arc::new(descriptor.clone()), &subkeys) {
                                Ok(v) => v,
                                Err(e) => {
                                    return Ok(NetworkResult::invalid_message(e));
                                }
                            };
                        ctx.opt_descriptor_info = Some(descriptor_info);
                    }
                }

                // Keep the value if we got one and it is newer and it passes schema validation
                if !answer.seqs.is_empty() {
                    log_dht!(debug "Got seqs back: len={}", answer.seqs.len());
                    let mut ctx = context.lock();

                    // Ensure we have a schema and descriptor etc
                    let Some(descriptor_info) = &ctx.opt_descriptor_info else {
                        // Got a value but no descriptor for it
                        // Move to the next node
                        return Ok(NetworkResult::invalid_message(
                            "Got inspection with no descriptor",
                        ));
                    };

                    // Get number of subkeys from schema and ensure we are getting the
                    // right number of sequence numbers betwen that and what we asked for
                    if answer.seqs.len() != descriptor_info.subkeys.len() {
                        // Not the right number of sequence numbers
                        // Move to the next node
                        return Ok(NetworkResult::invalid_message(format!(
                            "wrong number of seqs returned {} (wanted {})",
                            answer.seqs.len(),
                            descriptor_info.subkeys.len()
                        )));
                    }

                    // If we have a prior seqs list, merge in the new seqs
                    if ctx.seqcounts.is_empty() {
                        ctx.seqcounts = answer
                            .seqs
                            .iter()
                            .map(|s| SubkeySeqCount {
                                seq: *s,
                                // One node has shown us the newest sequence numbers so far
                                value_nodes: if *s == ValueSeqNum::MAX {
                                    vec![]
                                } else {
                                    vec![next_node.clone()]
                                },
                            })
                            .collect();
                    } else {
                        if ctx.seqcounts.len() != answer.seqs.len() {
                            return Err(RPCError::internal(
                                "seqs list length should always be equal by now",
                            ));
                        }
                        for pair in ctx.seqcounts.iter_mut().zip(answer.seqs.iter()) {
                            let ctx_seqcnt = pair.0;
                            let answer_seq = *pair.1;

                            // If we already have consensus for this subkey, don't bother updating it any more
                            // While we may find a better sequence number if we keep looking, this does not mimic the behavior
                            // of get and set unless we stop here
                            if ctx_seqcnt.value_nodes.len() >= consensus_count {
                                continue;
                            }

                            // If the new seq isn't undefined and is better than the old seq (either greater or old is undefined)
                            // Then take that sequence number and note that we have gotten newer sequence numbers so we keep
                            // looking for consensus
                            // If the sequence number matches the old sequence number, then we keep the value node for reference later
                            if answer_seq != ValueSeqNum::MAX {
                                if ctx_seqcnt.seq == ValueSeqNum::MAX || answer_seq > ctx_seqcnt.seq
                                {
                                    // One node has shown us the latest sequence numbers so far
                                    ctx_seqcnt.seq = answer_seq;
                                    ctx_seqcnt.value_nodes = vec![next_node.clone()];
                                } else if answer_seq == ctx_seqcnt.seq {
                                    // Keep the nodes that showed us the latest values
                                    ctx_seqcnt.value_nodes.push(next_node.clone());
                                }
                            }
                        }
                    }
                }

                // Return peers if we have some
                log_network_result!(debug "InspectValue fanout call returned peers {}", answer.peers.len());

                Ok(NetworkResult::value(answer.peers))
            }
        };

        // Routine to call to check if we're done at each step
        let check_done = |_closest_nodes: &[NodeRef]| {
            // If we have reached sufficient consensus on all subkeys, return done
            let ctx = context.lock();
            let mut has_consensus = true;
            for cs in ctx.seqcounts.iter() {
                if cs.value_nodes.len() < consensus_count {
                    has_consensus = false;
                    break;
                }
            }
            if !ctx.seqcounts.is_empty() && ctx.opt_descriptor_info.is_some() && has_consensus {
                return Some(());
            }
            None
        };

        // Call the fanout
        let fanout_call = FanoutCall::new(
            routing_table.clone(),
            key,
            key_count,
            fanout,
            timeout_us,
            capability_fanout_node_info_filter(vec![CAP_DHT, CAP_DHT_WATCH]),
            call_routine,
            check_done,
        );

        let kind = match fanout_call.run(vec![]).await {
            // If we don't finish in the timeout (too much time passed checking for consensus)
            TimeoutOr::Timeout => FanoutResultKind::Timeout,
            // If we finished with or without consensus (enough nodes returning the same value)
            TimeoutOr::Value(Ok(Some(()))) => FanoutResultKind::Finished,
            // If we ran out of nodes before getting consensus)
            TimeoutOr::Value(Ok(None)) => FanoutResultKind::Exhausted,
            // Failed
            TimeoutOr::Value(Err(e)) => {
                // If we finished with an error, return that
                log_dht!(debug "InspectValue Fanout Error: {}", e);
                return Err(e.into());
            }
        };

        let ctx = context.lock();
        let mut fanout_results = vec![];
        for cs in &ctx.seqcounts {
            let has_consensus = cs.value_nodes.len() >= consensus_count;
            let fanout_result = FanoutResult {
                kind: if has_consensus {
                    FanoutResultKind::Finished
                } else {
                    kind
                },
                value_nodes: cs.value_nodes.clone(),
            };
            fanout_results.push(fanout_result);
        }

        log_network_result!(debug "InspectValue Fanout ({:?}):\n{}", kind, debug_fanout_results(&fanout_results));

        Ok(OutboundInspectValueResult {
            fanout_results,
            inspect_result: InspectResult {
                subkeys: ctx
                    .opt_descriptor_info
                    .as_ref()
                    .map(|d| d.subkeys.clone())
                    .unwrap_or_default(),
                seqs: ctx.seqcounts.iter().map(|cs| cs.seq).collect(),
                opt_descriptor: ctx
                    .opt_descriptor_info
                    .as_ref()
                    .map(|d| d.descriptor.clone()),
            },
        })
    }

    /// Handle a received 'Inspect Value' query
    pub async fn inbound_inspect_value(
        &self,
        key: TypedKey,
        subkeys: ValueSubkeyRangeSet,
        want_descriptor: bool,
    ) -> VeilidAPIResult<NetworkResult<InspectResult>> {
        let mut inner = self.lock().await?;

        // See if this is a remote or local value
        let (_is_local, inspect_result) = {
            // See if the subkey we are getting has a last known local value
            let mut local_inspect_result = inner
                .handle_inspect_local_value(key, subkeys.clone(), true)
                .await?;
            // If this is local, it must have a descriptor already
            if local_inspect_result.opt_descriptor.is_some() {
                if !want_descriptor {
                    local_inspect_result.opt_descriptor = None;
                }
                (true, local_inspect_result)
            } else {
                // See if the subkey we are getting has a last known remote value
                let remote_inspect_result = inner
                    .handle_inspect_remote_value(key, subkeys, want_descriptor)
                    .await?;
                (false, remote_inspect_result)
            }
        };

        Ok(NetworkResult::value(inspect_result))
    }
}
