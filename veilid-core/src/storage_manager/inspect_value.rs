use super::*;

impl_veilid_log_facility!("stor");

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
    pub seq: Option<ValueSeqNum>,
    /// The set of nodes that had the most recent value for this subkey
    pub consensus_nodes: Vec<NodeRef>,
    /// The set of nodes that had any value for this subkey
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
#[derive(Debug, Clone)]
pub(super) struct OutboundInspectValueResult {
    /// Fanout results for each subkey
    pub subkey_fanout_results: Vec<FanoutResult>,
    /// The inspection that was retrieved
    pub inspect_result: InspectResult,
}

impl StorageManager {
    /// Perform a 'inspect value' query on the network
    #[instrument(level = "trace", target = "dht", skip_all, err)]
    pub(super) async fn outbound_inspect_value(
        &self,
        record_key: TypedKey,
        subkeys: ValueSubkeyRangeSet,
        safety_selection: SafetySelection,
        local_inspect_result: InspectResult,
        use_set_scope: bool,
    ) -> VeilidAPIResult<OutboundInspectValueResult> {
        let routing_domain = RoutingDomain::PublicInternet;
        let requested_subkeys = subkeys.clone();

        // Get the DHT parameters for 'InspectValue'
        // Can use either 'get scope' or 'set scope' depending on the purpose of the inspection
        let (key_count, consensus_count, fanout, timeout_us) = self.config().with(|c| {
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
        });

        // Get the nodes we know are caching this value to seed the fanout
        let init_fanout_queue = {
            self.get_value_nodes(record_key)
                .await?
                .unwrap_or_default()
                .into_iter()
                .filter(|x| {
                    x.node_info(routing_domain)
                        .map(|ni| ni.has_all_capabilities(&[CAP_DHT]))
                        .unwrap_or_default()
                })
                .collect()
        };

        // Make do-inspect-value answer context
        let opt_descriptor_info = if let Some(descriptor) = local_inspect_result.opt_descriptor() {
            // Get the descriptor info. This also truncates the subkeys list to what can be returned from the network.
            Some(DescriptorInfo::new(descriptor, &subkeys)?)
        } else {
            None
        };

        let context = Arc::new(Mutex::new(OutboundInspectValueContext {
            seqcounts: local_inspect_result
                .seqs()
                .iter()
                .map(|s| SubkeySeqCount {
                    seq: *s,
                    consensus_nodes: vec![],
                    value_nodes: vec![],
                })
                .collect(),
            opt_descriptor_info,
        }));

        // Routine to call to generate fanout
        let call_routine = {
            let context = context.clone();
            let registry = self.registry();
            Arc::new(
                move |next_node: NodeRef| -> PinBoxFutureStatic<FanoutCallResult> {
                    let context = context.clone();
                    let registry = registry.clone();
                    let opt_descriptor = local_inspect_result.opt_descriptor();
                    let subkeys = subkeys.clone();
                    Box::pin(async move {
                        let rpc_processor = registry.rpc_processor();

                        let iva = match
                            rpc_processor
                                .rpc_call_inspect_value(
                                    Destination::direct(next_node.routing_domain_filtered(routing_domain)).with_safety(safety_selection),
                                    record_key,
                                    subkeys.clone(),
                                    opt_descriptor.map(|x| (*x).clone()),
                                )
                                .await? {
                            NetworkResult::Timeout => {
                                return Ok(FanoutCallOutput{peer_info_list: vec![], disposition: FanoutCallDisposition::Timeout});
                            }
                            NetworkResult::ServiceUnavailable(_) |
                            NetworkResult::NoConnection(_)  |
                            NetworkResult::AlreadyExists(_) |
                            NetworkResult::InvalidMessage(_) => {
                                return Ok(FanoutCallOutput{peer_info_list: vec![], disposition: FanoutCallDisposition::Invalid});
                            }
                            NetworkResult::Value(v) => v
                        };

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
                                            veilid_log!(registry debug target:"network_result", "InspectValue returned an invalid descriptor: {}", e);
                                            return Ok(FanoutCallOutput{peer_info_list: vec![], disposition: FanoutCallDisposition::Invalid});
                                        }
                                    };
                                ctx.opt_descriptor_info = Some(descriptor_info);
                            }
                        }

                        // Keep the value if we got one and it is newer and it passes schema validation
                        if answer.seqs.is_empty() {
                            veilid_log!(registry debug target:"network_result", "InspectValue returned no seq, fanout call returned peers {}", answer.peers.len());
                            return Ok(FanoutCallOutput{peer_info_list: answer.peers, disposition: FanoutCallDisposition::Rejected});
                        }

                        veilid_log!(registry debug target:"network_result", "Got seqs back: len={}", answer.seqs.len());
                        let mut ctx = context.lock();

                        // Ensure we have a schema and descriptor etc
                        let Some(descriptor_info) = &ctx.opt_descriptor_info else {
                            // Got a value but no descriptor for it
                            // Move to the next node
                            veilid_log!(registry debug target:"network_result", "InspectValue returned a value with no descriptor invalid descriptor");
                            return Ok(FanoutCallOutput{peer_info_list: vec![], disposition: FanoutCallDisposition::Invalid});
                        };

                        // Get number of subkeys from schema and ensure we are getting the
                        // right number of sequence numbers betwen that and what we asked for
                        #[allow(clippy::unnecessary_cast)]
                        if answer.seqs.len() as u64 != descriptor_info.subkeys.len() as u64 {
                            // Not the right number of sequence numbers
                            // Move to the next node
                            veilid_log!(registry debug target:"network_result", "wrong number of seqs returned {} (wanted {})",
                                answer.seqs.len(),
                                descriptor_info.subkeys.len());
                            return Ok(FanoutCallOutput{peer_info_list: vec![], disposition: FanoutCallDisposition::Invalid});
                        }

                        // If we have a prior seqs list, merge in the new seqs
                        if ctx.seqcounts.is_empty() {
                            ctx.seqcounts = answer
                                .seqs
                                .iter()
                                .map(|s| SubkeySeqCount {
                                    seq: *s,
                                    // One node has shown us the newest sequence numbers so far
                                    consensus_nodes: vec![next_node.clone()],
                                    value_nodes: vec![next_node.clone()],
                                })
                                .collect();
                        } else {
                            if ctx.seqcounts.len() != answer.seqs.len() {
                                veilid_log!(registry debug target:"network_result", "seqs list length should always be equal by now: {} (wanted {})",
                                    answer.seqs.len(),
                                    ctx.seqcounts.len());
                                return Ok(FanoutCallOutput{peer_info_list: vec![], disposition: FanoutCallDisposition::Invalid});

                            }
                            for pair in ctx.seqcounts.iter_mut().zip(answer.seqs.iter()) {
                                let ctx_seqcnt = pair.0;
                                let answer_seq = *pair.1;

                                // If we already have consensus for this subkey, don't bother updating it any more
                                // While we may find a better sequence number if we keep looking, this does not mimic the behavior
                                // of get and set unless we stop here
                                if ctx_seqcnt.consensus_nodes.len() >= consensus_count {
                                    continue;
                                }

                                // If the new seq isn't undefined and is better than the old seq (either greater or old is undefined)
                                // Then take that sequence number and note that we have gotten newer sequence numbers so we keep
                                // looking for consensus
                                // If the sequence number matches the old sequence number, then we keep the value node for reference later
                                if let Some(answer_seq) = answer_seq {
                                    if ctx_seqcnt.seq.is_none() || answer_seq > ctx_seqcnt.seq.unwrap()
                                    {
                                        // One node has shown us the latest sequence numbers so far
                                        ctx_seqcnt.seq = Some(answer_seq);
                                        ctx_seqcnt.consensus_nodes = vec![next_node.clone()];
                                    } else if answer_seq == ctx_seqcnt.seq.unwrap() {
                                        // Keep the nodes that showed us the latest values
                                        ctx_seqcnt.consensus_nodes.push(next_node.clone());
                                    }
                                }
                                ctx_seqcnt.value_nodes.push(next_node.clone());
                            }
                        }


                        // Return peers if we have some
                        veilid_log!(registry debug target:"network_result", "InspectValue fanout call returned peers {}", answer.peers.len());

                        // Inspect doesn't actually use the fanout queue consensus tracker
                        Ok(FanoutCallOutput { peer_info_list: answer.peers, disposition: FanoutCallDisposition::Accepted})
                    }.instrument(tracing::trace_span!("outbound_inspect_value fanout call"))) as PinBoxFuture<FanoutCallResult>
                },
            )
        };

        // Routine to call to check if we're done at each step
        // For inspect, we are tracking consensus externally from the FanoutCall,
        // for each subkey, rather than a single consensus, so the single fanoutresult
        // that is passed in here is ignored in favor of our own per-subkey tracking
        let check_done = {
            let context = context.clone();
            Arc::new(move |_: &FanoutResult| {
                // If we have reached sufficient consensus on all subkeys, return done
                let ctx = context.lock();
                let mut has_consensus = true;
                for cs in ctx.seqcounts.iter() {
                    if cs.consensus_nodes.len() < consensus_count {
                        has_consensus = false;
                        break;
                    }
                }

                !ctx.seqcounts.is_empty() && ctx.opt_descriptor_info.is_some() && has_consensus
            })
        };

        // Call the fanout
        let routing_table = self.routing_table();
        let fanout_call = FanoutCall::new(
            &routing_table,
            record_key,
            key_count,
            fanout,
            consensus_count,
            timeout_us,
            capability_fanout_node_info_filter(vec![CAP_DHT]),
            call_routine,
            check_done,
        );

        let fanout_result = fanout_call.run(init_fanout_queue).await?;

        let ctx = context.lock();
        let mut subkey_fanout_results = vec![];
        for cs in &ctx.seqcounts {
            let has_consensus = cs.consensus_nodes.len() >= consensus_count;
            let subkey_fanout_result = FanoutResult {
                kind: if has_consensus {
                    FanoutResultKind::Consensus
                } else {
                    fanout_result.kind
                },
                consensus_nodes: cs.consensus_nodes.clone(),
                value_nodes: cs.value_nodes.clone(),
            };
            subkey_fanout_results.push(subkey_fanout_result);
        }

        if subkey_fanout_results.len() == 1 {
            veilid_log!(self debug "InspectValue Fanout: {:#}\n{:#}", fanout_result, subkey_fanout_results.first().unwrap());
        } else {
            veilid_log!(self debug "InspectValue Fanout: {:#}:\n{}", fanout_result, debug_fanout_results(&subkey_fanout_results));
        }

        let result = OutboundInspectValueResult {
            subkey_fanout_results,
            inspect_result: InspectResult::new(
                self,
                requested_subkeys,
                "outbound_inspect_value",
                ctx.opt_descriptor_info
                    .as_ref()
                    .map(|d| d.subkeys.clone())
                    .unwrap_or_default(),
                ctx.seqcounts.iter().map(|cs| cs.seq).collect(),
                ctx.opt_descriptor_info
                    .as_ref()
                    .map(|d| d.descriptor.clone()),
            )?,
        };

        #[allow(clippy::unnecessary_cast)]
        {
            if result.inspect_result.subkeys().len() as u64
                != result.subkey_fanout_results.len() as u64
            {
                veilid_log!(self error "mismatch between subkeys returned and fanout results returned: {}!={}", result.inspect_result.subkeys().len(), result.subkey_fanout_results.len());
                apibail_internal!("subkey and fanout list length mismatched");
            }
        }

        Ok(result)
    }

    /// Handle a received 'Inspect Value' query
    #[instrument(level = "trace", target = "dht", skip_all)]
    pub async fn inbound_inspect_value(
        &self,
        record_key: TypedKey,
        subkeys: ValueSubkeyRangeSet,
        want_descriptor: bool,
    ) -> VeilidAPIResult<NetworkResult<InspectResult>> {
        let mut inner = self.inner.lock().await;

        // See if this is a remote or local value
        let (_is_local, inspect_result) = {
            // See if the subkey we are getting has a last known local value
            let mut local_inspect_result = self
                .handle_inspect_local_value_inner(&mut inner, record_key, subkeys.clone(), true)
                .await?;
            // If this is local, it must have a descriptor already
            if local_inspect_result.opt_descriptor().is_some() {
                if !want_descriptor {
                    local_inspect_result.drop_descriptor();
                }
                (true, local_inspect_result)
            } else {
                // See if the subkey we are getting has a last known remote value
                let remote_inspect_result = self
                    .handle_inspect_remote_value_inner(
                        &mut inner,
                        record_key,
                        subkeys,
                        want_descriptor,
                    )
                    .await?;
                (false, remote_inspect_result)
            }
        };

        Ok(NetworkResult::value(inspect_result))
    }
}
