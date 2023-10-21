use super::*;

/// The context of the outbound_get_value operation
struct OutboundGetValueContext {
    /// The latest value of the subkey, may be the value passed in
    pub value: Option<SignedValueData>,
    /// The consensus count for the value we have received
    pub value_count: usize,
    /// The descriptor if we got a fresh one or empty if no descriptor was needed
    pub descriptor: Option<SignedValueDescriptor>,
    /// The parsed schema from the descriptor if we have one
    pub schema: Option<DHTSchema>,
}

impl StorageManager {
    /// Perform a 'get value' query on the network
    pub async fn outbound_get_value(
        &self,
        rpc_processor: RPCProcessor,
        key: TypedKey,
        subkey: ValueSubkey,
        safety_selection: SafetySelection,
        last_subkey_result: SubkeyResult,
    ) -> VeilidAPIResult<SubkeyResult> {
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
        let schema = if let Some(d) = &last_subkey_result.descriptor {
            Some(d.schema()?)
        } else {
            None
        };
        let context = Arc::new(Mutex::new(OutboundGetValueContext {
            value: last_subkey_result.value,
            value_count: 0,
            descriptor: last_subkey_result.descriptor.clone(),
            schema,
        }));

        // Routine to call to generate fanout
        let call_routine = |next_node: NodeRef| {
            let rpc_processor = rpc_processor.clone();
            let context = context.clone();
            let last_descriptor = last_subkey_result.descriptor.clone();
            async move {
                let gva = network_result_try!(
                    rpc_processor
                        .clone()
                        .rpc_call_get_value(
                            Destination::direct(next_node.clone()).with_safety(safety_selection),
                            key,
                            subkey,
                            last_descriptor,
                        )
                        .await?
                );

                // Keep the descriptor if we got one. If we had a last_descriptor it will
                // already be validated by rpc_call_get_value
                if let Some(descriptor) = gva.answer.descriptor {
                    let mut ctx = context.lock();
                    if ctx.descriptor.is_none() && ctx.schema.is_none() {
                        ctx.schema = Some(descriptor.schema().map_err(RPCError::invalid_format)?);
                        ctx.descriptor = Some(descriptor);
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
                            ctx.value_count += 1;
                        } else if new_seq > prior_seq {
                            // If the sequence number is greater, start over with the new value
                            ctx.value = Some(value);
                            // One node has shown us this value so far
                            ctx.value_count = 1;
                        } else {
                            // If the sequence number is older, ignore it
                        }
                    } else {
                        // If we have no prior value, keep it
                        ctx.value = Some(value);
                        // One node has shown us this value so far
                        ctx.value_count = 1;
                    }
                }

                // Return peers if we have some
                #[cfg(feature = "network-result-extra")]
                log_stor!(debug "GetValue fanout call returned peers {}", gva.answer.peers.len());

                Ok(NetworkResult::value(gva.answer.peers))
            }
        };

        // Routine to call to check if we're done at each step
        let check_done = |_closest_nodes: &[NodeRef]| {
            // If we have reached sufficient consensus, return done
            let ctx = context.lock();
            if ctx.value.is_some() && ctx.descriptor.is_some() && ctx.value_count >= consensus_count
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
            capability_fanout_node_info_filter(vec![CAP_DHT]),
            call_routine,
            check_done,
        );

        match fanout_call.run().await {
            // If we don't finish in the timeout (too much time passed checking for consensus)
            TimeoutOr::Timeout => {
                // Return the best answer we've got
                let ctx = context.lock();
                if ctx.value_count >= consensus_count {
                    log_stor!(debug "GetValue Fanout Timeout Consensus");
                } else {
                    log_stor!(debug "GetValue Fanout Timeout Non-Consensus: {}", ctx.value_count);
                }
                Ok(SubkeyResult {
                    value: ctx.value.clone(),
                    descriptor: ctx.descriptor.clone(),
                })
            }
            // If we finished with consensus (enough nodes returning the same value)
            TimeoutOr::Value(Ok(Some(()))) => {
                // Return the best answer we've got
                let ctx = context.lock();
                if ctx.value_count >= consensus_count {
                    log_stor!(debug "GetValue Fanout Consensus");
                } else {
                    log_stor!(debug "GetValue Fanout Non-Consensus: {}", ctx.value_count);
                }
                Ok(SubkeyResult {
                    value: ctx.value.clone(),
                    descriptor: ctx.descriptor.clone(),
                })
            }
            // If we finished without consensus (ran out of nodes before getting consensus)
            TimeoutOr::Value(Ok(None)) => {
                // Return the best answer we've got
                let ctx = context.lock();
                if ctx.value_count >= consensus_count {
                    log_stor!(debug "GetValue Fanout Exhausted Consensus");
                } else {
                    log_stor!(debug "GetValue Fanout Exhausted Non-Consensus: {}", ctx.value_count);
                }
                Ok(SubkeyResult {
                    value: ctx.value.clone(),
                    descriptor: ctx.descriptor.clone(),
                })
            }
            // Failed
            TimeoutOr::Value(Err(e)) => {
                // If we finished with an error, return that
                log_stor!(debug "GetValue Fanout Error: {}", e);
                Err(e.into())
            }
        }
    }

    /// Handle a received 'Get Value' query
    pub async fn inbound_get_value(
        &self,
        key: TypedKey,
        subkey: ValueSubkey,
        want_descriptor: bool,
    ) -> VeilidAPIResult<NetworkResult<SubkeyResult>> {
        let mut inner = self.lock().await?;
        let res = match inner
            .handle_get_remote_value(key, subkey, want_descriptor)
            .await
        {
            Ok(res) => res,
            Err(VeilidAPIError::Internal { message }) => {
                apibail_internal!(message);
            }
            Err(e) => {
                return Ok(NetworkResult::invalid_message(e));
            }
        };
        Ok(NetworkResult::value(res))
    }
}
