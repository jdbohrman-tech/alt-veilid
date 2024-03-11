use super::*;

/// The context of the outbound_get_value operation
struct OutboundGetValueContext {
    /// The latest value of the subkey, may be the value passed in
    pub value: Option<Arc<SignedValueData>>,
    /// The nodes that have returned the value so far (up to the consensus count)
    pub value_nodes: Vec<NodeRef>,
    /// The descriptor if we got a fresh one or empty if no descriptor was needed
    pub descriptor: Option<Arc<SignedValueDescriptor>>,
    /// The parsed schema from the descriptor if we have one
    pub schema: Option<DHTSchema>,
}

/// The result of the outbound_get_value operation
pub(super) struct OutboundGetValueResult {
    /// Fanout result
    pub fanout_result: FanoutResult,
    /// Consensus count for this operation,
    pub consensus_count: usize,
    /// The subkey that was retrieved
    pub get_result: GetResult,
}

impl StorageManager {
    /// Perform a 'get value' query on the network
    pub(super) async fn outbound_get_value(
        &self,
        rpc_processor: RPCProcessor,
        key: TypedKey,
        subkey: ValueSubkey,
        safety_selection: SafetySelection,
        last_get_result: GetResult,
    ) -> VeilidAPIResult<OutboundGetValueResult> {
        let routing_table = rpc_processor.routing_table();

        // Get the DHT parameters for 'GetValue'
        let (key_count, consensus_count, fanout, timeout_us) = {
            let c = self.unlocked_inner.config.get();
            (
                c.network.dht.max_find_node_count as usize,
                c.network.dht.get_value_count as usize,
                c.network.dht.get_value_fanout as usize,
                TimestampDuration::from(ms_to_us(c.network.dht.get_value_timeout_ms)),
            )
        };

        // Make do-get-value answer context
        let schema = if let Some(d) = &last_get_result.opt_descriptor {
            Some(d.schema()?)
        } else {
            None
        };
        let context = Arc::new(Mutex::new(OutboundGetValueContext {
            value: last_get_result.opt_value,
            value_nodes: vec![],
            descriptor: last_get_result.opt_descriptor.clone(),
            schema,
        }));

        // Routine to call to generate fanout
        let call_routine = |next_node: NodeRef| {
            let rpc_processor = rpc_processor.clone();
            let context = context.clone();
            let last_descriptor = last_get_result.opt_descriptor.clone();
            async move {
                let gva = network_result_try!(
                    rpc_processor
                        .clone()
                        .rpc_call_get_value(
                            Destination::direct(next_node.clone()).with_safety(safety_selection),
                            key,
                            subkey,
                            last_descriptor.map(|x| (*x).clone()),
                        )
                        .await?
                );

                // Keep the descriptor if we got one. If we had a last_descriptor it will
                // already be validated by rpc_call_get_value
                if let Some(descriptor) = gva.answer.descriptor {
                    let mut ctx = context.lock();
                    if ctx.descriptor.is_none() && ctx.schema.is_none() {
                        let schema = match descriptor.schema() {
                            Ok(v) => v,
                            Err(e) => {
                                return Ok(NetworkResult::invalid_message(e));
                            }
                        };
                        ctx.schema = Some(schema);
                        ctx.descriptor = Some(Arc::new(descriptor));
                    }
                }

                // Keep the value if we got one and it is newer and it passes schema validation
                if let Some(value) = gva.answer.value {
                    log_stor!(debug "Got value back: len={} seq={}", value.value_data().data().len(), value.value_data().seq());
                    let mut ctx = context.lock();

                    // Ensure we have a schema and descriptor
                    let (Some(descriptor), Some(schema)) = (&ctx.descriptor, &ctx.schema) else {
                        // Got a value but no descriptor for it
                        // Move to the next node
                        return Ok(NetworkResult::invalid_message(
                            "Got value with no descriptor",
                        ));
                    };

                    // Validate with schema
                    if !schema.check_subkey_value_data(
                        descriptor.owner(),
                        subkey,
                        value.value_data(),
                    ) {
                        // Validation failed, ignore this value
                        // Move to the next node
                        return Ok(NetworkResult::invalid_message(format!(
                            "Schema validation failed on subkey {}",
                            subkey
                        )));
                    }

                    // If we have a prior value, see if this is a newer sequence number
                    if let Some(prior_value) = &ctx.value {
                        let prior_seq = prior_value.value_data().seq();
                        let new_seq = value.value_data().seq();

                        if new_seq == prior_seq {
                            // If sequence number is the same, the data should be the same
                            if prior_value.value_data() != value.value_data() {
                                // Move to the next node
                                return Ok(NetworkResult::invalid_message("value data mismatch"));
                            }
                            // Increase the consensus count for the existing value
                            ctx.value_nodes.push(next_node);
                        } else if new_seq > prior_seq {
                            // If the sequence number is greater, start over with the new value
                            ctx.value = Some(Arc::new(value));
                            // One node has shown us this value so far
                            ctx.value_nodes = vec![next_node];
                        } else {
                            // If the sequence number is older, ignore it
                        }
                    } else {
                        // If we have no prior value, keep it
                        ctx.value = Some(Arc::new(value));
                        // One node has shown us this value so far
                        ctx.value_nodes = vec![next_node];
                    }
                }

                // Return peers if we have some
                log_network_result!(debug "GetValue fanout call returned peers {}", gva.answer.peers.len());

                Ok(NetworkResult::value(gva.answer.peers))
            }
        };

        // Routine to call to check if we're done at each step
        let check_done = |_closest_nodes: &[NodeRef]| {
            // If we have reached sufficient consensus, return done
            let ctx = context.lock();
            if ctx.value.is_some()
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
                log_stor!(debug "GetValue Fanout Error: {}", e);
                return Err(e.into());
            }
        };

        let ctx = context.lock();
        let fanout_result = FanoutResult {
            kind,
            value_nodes: ctx.value_nodes.clone(),
        };
        log_stor!(debug "GetValue Fanout: {:?}", fanout_result);

        Ok(OutboundGetValueResult {
            fanout_result,
            consensus_count,
            get_result: GetResult {
                opt_value: ctx.value.clone(),
                opt_descriptor: ctx.descriptor.clone(),
            },
        })
    }

    /// Handle a received 'Get Value' query
    pub async fn inbound_get_value(
        &self,
        key: TypedKey,
        subkey: ValueSubkey,
        want_descriptor: bool,
    ) -> VeilidAPIResult<NetworkResult<GetResult>> {
        let mut inner = self.lock().await?;

        // See if this is a remote or local value
        let (_is_local, last_get_result) = {
            // See if the subkey we are getting has a last known local value
            let mut last_get_result = inner.handle_get_local_value(key, subkey, true).await?;
            // If this is local, it must have a descriptor already
            if last_get_result.opt_descriptor.is_some() {
                if !want_descriptor {
                    last_get_result.opt_descriptor = None;
                }
                (true, last_get_result)
            } else {
                // See if the subkey we are getting has a last known remote value
                let last_get_result = inner
                    .handle_get_remote_value(key, subkey, want_descriptor)
                    .await?;
                (false, last_get_result)
            }
        };

        Ok(NetworkResult::value(last_get_result))
    }
}
