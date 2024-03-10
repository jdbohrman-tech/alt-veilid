use super::*;

/// The context of the outbound_get_value operation
struct OutboundInspectValueContext {
    /// The combined sequence map so far
    pub seqs: Vec<ValueSeqNum>,
    /// The nodes that have returned the value so far (up to the consensus count)
    pub value_nodes: Vec<NodeRef>,
    /// The descriptor if we got a fresh one or empty if no descriptor was needed
    pub descriptor: Option<Arc<SignedValueDescriptor>>,
    /// The parsed schema from the descriptor if we have one
    pub schema: Option<DHTSchema>,
}

/// The result of the outbound_get_value operation
pub(super) struct OutboundInspectValueResult {
    /// The subkey that was retrieved
    pub inspect_result: InspectResult,
    /// And where it was retrieved from
    pub value_nodes: Vec<NodeRef>,
}

impl StorageManager {
    /// Perform a 'inspect value' query on the network
    pub(super) async fn outbound_inspect_value(
        &self,
        rpc_processor: RPCProcessor,
        key: TypedKey,
        subkeys: ValueSubkeyRangeSet,
        safety_selection: SafetySelection,
        last_inspect_result: InspectResult,
    ) -> VeilidAPIResult<OutboundInspectValueResult> {
        let routing_table = rpc_processor.routing_table();

        // Get the DHT parameters for 'InspectValue' (the same as for 'GetValue')
        let (key_count, consensus_count, fanout, timeout_us) = {
            let c = self.unlocked_inner.config.get();
            (
                c.network.dht.max_find_node_count as usize,
                c.network.dht.get_value_count as usize,
                c.network.dht.get_value_fanout as usize,
                TimestampDuration::from(ms_to_us(c.network.dht.get_value_timeout_ms)),
            )
        };

        // Make do-inspect-value answer context
        let schema = if let Some(d) = &last_inspect_result.opt_descriptor {
            Some(d.schema()?)
        } else {
            None
        };
        let context = Arc::new(Mutex::new(OutboundInspectValueContext {
            seqs: last_inspect_result.seqs,
            value_nodes: vec![],
            descriptor: last_inspect_result.opt_descriptor.clone(),
            schema,
        }));

        // Routine to call to generate fanout
        let call_routine = |next_node: NodeRef| {
            let rpc_processor = rpc_processor.clone();
            let context = context.clone();
            let last_descriptor = last_inspect_result.opt_descriptor.clone();
            let subkeys = subkeys.clone();
            async move {
                let iva = network_result_try!(
                    rpc_processor
                        .clone()
                        .rpc_call_inspect_value(
                            Destination::direct(next_node.clone()).with_safety(safety_selection),
                            key,
                            subkeys.clone(),
                            last_descriptor.map(|x| (*x).clone()),
                        )
                        .await?
                );

                // Keep the descriptor if we got one. If we had a last_descriptor it will
                // already be validated by rpc_call_inspect_value
                if let Some(descriptor) = iva.answer.descriptor {
                    let mut ctx = context.lock();
                    if ctx.descriptor.is_none() && ctx.schema.is_none() {
                        ctx.schema = Some(descriptor.schema().map_err(RPCError::invalid_format)?);
                        ctx.descriptor = Some(Arc::new(descriptor));
                    }
                }

                // Keep the value if we got one and it is newer and it passes schema validation
                if !iva.answer.seqs.is_empty() {
                    log_stor!(debug "Got seqs back: len={}", iva.answer.seqs.len());
                    let mut ctx = context.lock();

                    // Ensure we have a schema and descriptor
                    let (Some(_descriptor), Some(schema)) = (&ctx.descriptor, &ctx.schema) else {
                        // Got a value but no descriptor for it
                        // Move to the next node
                        return Ok(NetworkResult::invalid_message(
                            "Got value with no descriptor",
                        ));
                    };

                    // Get number of subkeys from schema and ensure we are getting the
                    // right number of sequence numbers betwen that and what we asked for
                    let in_schema_subkeys = subkeys
                        .intersect(&ValueSubkeyRangeSet::single_range(0, schema.max_subkey()));
                    if iva.answer.seqs.len() != in_schema_subkeys.len() {
                        // Not the right number of sequence numbers
                        // Move to the next node
                        return Ok(NetworkResult::invalid_message(format!(
                            "wrong number of seqs returned {} (wanted {})",
                            iva.answer.seqs.len(),
                            in_schema_subkeys.len()
                        )));
                    }

                    // If we have a prior seqs list, merge in the new seqs
                    if ctx.seqs.len() == 0 {
                        ctx.seqs = iva.answer.seqs.clone();
                        // One node has shown us the newest sequence numbers so far
                        ctx.value_nodes = vec![next_node];
                    } else {
                        if ctx.seqs.len() != iva.answer.seqs.len() {
                            return Err(RPCError::internal(
                                "seqs list length should always be equal by now",
                            ));
                        }
                        let mut newer_seq = false;
                        for pair in ctx.seqs.iter_mut().zip(iva.answer.seqs.iter()) {
                            // If the new seq isn't undefined and is better than the old seq (either greater or old is undefined)
                            // Then take that sequence number and note that we have gotten newer sequence numbers so we keep
                            // looking for consensus
                            if *pair.1 != ValueSeqNum::MAX
                                && (*pair.0 == ValueSeqNum::MAX || pair.1 > pair.0)
                            {
                                newer_seq = true;
                                *pair.0 = *pair.1;
                            }
                        }
                        if newer_seq {
                            // One node has shown us the latest sequence numbers so far
                            ctx.value_nodes = vec![next_node];
                        } else {
                            // Increase the consensus count for the seqs list
                            ctx.value_nodes.push(next_node);
                        }
                    }
                }

                // Return peers if we have some
                log_network_result!(debug "InspectValue fanout call returned peers {}", iva.answer.peers.len());

                Ok(NetworkResult::value(iva.answer.peers))
            }
        };

        // Routine to call to check if we're done at each step
        let check_done = |_closest_nodes: &[NodeRef]| {
            // If we have reached sufficient consensus, return done
            let ctx = context.lock();
            if ctx.seqs.len() > 0
                && ctx.descriptor.is_some()
                && ctx.value_nodes.len() >= consensus_count
            {
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

        match fanout_call.run(vec![]).await {
            // If we don't finish in the timeout (too much time passed checking for consensus)
            TimeoutOr::Timeout => {
                // Return the best answer we've got
                let ctx = context.lock();
                if ctx.value_nodes.len() >= consensus_count {
                    log_stor!(debug "InspectValue Fanout Timeout Consensus");
                } else {
                    log_stor!(debug "InspectValue Fanout Timeout Non-Consensus: {}", ctx.value_nodes.len());
                }
                Ok(OutboundInspectValueResult {
                    inspect_result: InspectResult {
                        seqs: ctx.seqs.clone(),
                        opt_descriptor: ctx.descriptor.clone(),
                    },
                    value_nodes: ctx.value_nodes.clone(),
                })
            }
            // If we finished with consensus (enough nodes returning the same value)
            TimeoutOr::Value(Ok(Some(()))) => {
                // Return the best answer we've got
                let ctx = context.lock();
                if ctx.value_nodes.len() >= consensus_count {
                    log_stor!(debug "InspectValue Fanout Consensus");
                } else {
                    log_stor!(debug "InspectValue Fanout Non-Consensus: {}", ctx.value_nodes.len());
                }
                Ok(OutboundInspectValueResult {
                    inspect_result: InspectResult {
                        seqs: ctx.seqs.clone(),
                        opt_descriptor: ctx.descriptor.clone(),
                    },
                    value_nodes: ctx.value_nodes.clone(),
                })
            }
            // If we finished without consensus (ran out of nodes before getting consensus)
            TimeoutOr::Value(Ok(None)) => {
                // Return the best answer we've got
                let ctx = context.lock();
                if ctx.value_nodes.len() >= consensus_count {
                    log_stor!(debug "InspectValue Fanout Exhausted Consensus");
                } else {
                    log_stor!(debug "InspectValue Fanout Exhausted Non-Consensus: {}", ctx.value_nodes.len());
                }
                Ok(OutboundInspectValueResult {
                    inspect_result: InspectResult {
                        seqs: ctx.seqs.clone(),
                        opt_descriptor: ctx.descriptor.clone(),
                    },
                    value_nodes: ctx.value_nodes.clone(),
                })
            }
            // Failed
            TimeoutOr::Value(Err(e)) => {
                // If we finished with an error, return that
                log_stor!(debug "InspectValue Fanout Error: {}", e);
                Err(e.into())
            }
        }
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
        let (_is_local, last_get_result) = {
            // See if the subkey we are getting has a last known local value
            let mut last_inspect_result = inner
                .handle_inspect_local_value(key, subkeys.clone(), true)
                .await?;
            // If this is local, it must have a descriptor already
            if last_inspect_result.opt_descriptor.is_some() {
                if !want_descriptor {
                    last_inspect_result.opt_descriptor = None;
                }
                (true, last_inspect_result)
            } else {
                // See if the subkey we are getting has a last known remote value
                let last_inspect_result = inner
                    .handle_inspect_remote_value(key, subkeys, want_descriptor)
                    .await?;
                (false, last_inspect_result)
            }
        };

        Ok(NetworkResult::value(last_get_result))
    }
}
