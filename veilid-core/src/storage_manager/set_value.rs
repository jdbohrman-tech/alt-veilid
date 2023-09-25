use super::*;

/// The context of the outbound_set_value operation
struct OutboundSetValueContext {
    /// The latest value of the subkey, may be the value passed in
    pub value: SignedValueData,
    /// The consensus count for the value we have received
    pub set_count: usize,
    /// The number of non-sets since the last set we have received
    pub missed_since_last_set: usize,
    /// The parsed schema from the descriptor if we have one
    pub schema: DHTSchema,
}

impl StorageManager {
    /// Perform a 'set value' query on the network
    pub async fn outbound_set_value(
        &self,
        rpc_processor: RPCProcessor,
        key: TypedKey,
        subkey: ValueSubkey,
        safety_selection: SafetySelection,
        value: SignedValueData,
        descriptor: SignedValueDescriptor,
    ) -> VeilidAPIResult<SignedValueData> {
        let routing_table = rpc_processor.routing_table();

        // Get the DHT parameters for 'SetValue'
        let (key_count, consensus_count, fanout, timeout_us) = {
            let c = self.unlocked_inner.config.get();
            (
                c.network.dht.max_find_node_count as usize,
                c.network.dht.set_value_count as usize,
                c.network.dht.set_value_fanout as usize,
                TimestampDuration::from(ms_to_us(c.network.dht.set_value_timeout_ms)),
            )
        };

        // Make do-set-value answer context
        let schema = descriptor.schema()?;
        let context = Arc::new(Mutex::new(OutboundSetValueContext {
            value,
            set_count: 0,
            missed_since_last_set: 0,
            schema,
        }));

        // Routine to call to generate fanout
        let call_routine = |next_node: NodeRef| {
            let rpc_processor = rpc_processor.clone();
            let context = context.clone();
            let descriptor = descriptor.clone();
            async move {
                let send_descriptor = true; // xxx check if next_node needs the descriptor or not

                // get most recent value to send
                let value = {
                    let ctx = context.lock();
                    ctx.value.clone()
                };

                // send across the wire
                let vres = rpc_processor
                    .clone()
                    .rpc_call_set_value(
                        Destination::direct(next_node.clone()).with_safety(safety_selection),
                        key,
                        subkey,
                        value,
                        descriptor.clone(),
                        send_descriptor,
                    )
                    .await?;
                let sva = network_result_value_or_log!(vres => [ format!(": next_node={} safety_selection={:?} key={} subkey={} send_descriptor={}", next_node, safety_selection, key, subkey, send_descriptor) ] {
                    // Any other failures, just try the next node and pretend this one never happened
                    return Ok(None);
                });

                // If the node was close enough to possibly set the value
                if sva.answer.set {
                    let mut ctx = context.lock();

                    // Keep the value if we got one and it is newer and it passes schema validation
                    if let Some(value) = sva.answer.value {
                        log_stor!(debug "Got value back: len={} seq={}", value.value_data().data().len(), value.value_data().seq());

                        // Validate with schema
                        if !ctx.schema.check_subkey_value_data(
                            descriptor.owner(),
                            subkey,
                            value.value_data(),
                        ) {
                            // Validation failed, ignore this value and pretend we never saw this node
                            return Ok(None);
                        }

                        // We have a prior value, ensure this is a newer sequence number
                        let prior_seq = ctx.value.value_data().seq();
                        let new_seq = value.value_data().seq();
                        if new_seq > prior_seq {
                            // If the sequence number is greater, keep it
                            ctx.value = value;
                            // One node has shown us this value so far
                            ctx.set_count = 1;
                            ctx.missed_since_last_set = 0;
                        } else {
                            // If the sequence number is older, or an equal sequence number,
                            // node should have not returned a value here.
                            // Skip this node and its closer list because it is misbehaving
                            return Ok(None);
                        }
                    } else {
                        // It was set on this node and no newer value was found and returned,
                        // so increase our consensus count
                        ctx.set_count += 1;
                        ctx.missed_since_last_set = 0;
                    }
                } else {
                    let mut ctx = context.lock();
                    ctx.missed_since_last_set += 1;
                }

                // Return peers if we have some
                #[cfg(feature = "network-result-extra")]
                log_stor!(debug "SetValue fanout call returned peers {}", sva.answer.peers.len());

                Ok(Some(sva.answer.peers))
            }
        };

        // Routine to call to check if we're done at each step
        let check_done = |_closest_nodes: &[NodeRef]| {
            let ctx = context.lock();

            // If we have reached sufficient consensus, return done
            if ctx.set_count >= consensus_count {
                return Some(());
            }
            // If we have missed more than our consensus count since our last set, return done
            // This keeps the traversal from searching too many nodes when we aren't converging
            // Only do this if we have gotten at least half our desired sets.
            if ctx.set_count >= ((consensus_count + 1) / 2)
                && ctx.missed_since_last_set >= consensus_count
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
                log_stor!(debug "SetValue Fanout Timeout");
                // Return the best answer we've got
                let ctx = context.lock();
                Ok(ctx.value.clone())
            }
            // If we finished with or without consensus (enough nodes returning the same value)
            TimeoutOr::Value(Ok(Some(()))) => {
                // Return the best answer we've got
                let ctx = context.lock();
                if ctx.set_count >= consensus_count {
                    log_stor!(debug "SetValue Fanout Consensus");
                } else {
                    log_stor!(debug "SetValue Fanout Non-Consensus: {}", ctx.set_count);
                }
                Ok(ctx.value.clone())
            }
            // If we ran out of nodes before getting consensus)
            TimeoutOr::Value(Ok(None)) => {
                // Return the best answer we've got
                let ctx = context.lock();
                if ctx.set_count >= consensus_count {
                    log_stor!(debug "SetValue Fanout Exhausted Consensus");
                } else {
                    log_stor!(debug "SetValue Fanout Exhausted Non-Consensus: {}", ctx.set_count);
                }
                Ok(ctx.value.clone())
            }
            // Failed
            TimeoutOr::Value(Err(e)) => {
                // If we finished with an error, return that
                log_stor!(debug "SetValue Fanout Error: {}", e);
                Err(e.into())
            }
        }
    }

    /// Handle a received 'Set Value' query
    /// Returns a None if the value passed in was set
    /// Returns a Some(current value) if the value was older and the current value was kept
    pub async fn inbound_set_value(
        &self,
        key: TypedKey,
        subkey: ValueSubkey,
        value: SignedValueData,
        descriptor: Option<SignedValueDescriptor>,
    ) -> VeilidAPIResult<NetworkResult<Option<SignedValueData>>> {
        let mut inner = self.lock().await?;

        // See if this is a remote or local value
        let (is_local, last_subkey_result) = {
            // See if the subkey we are modifying has a last known local value
            let last_subkey_result = inner.handle_get_local_value(key, subkey, true).await?;
            // If this is local, it must have a descriptor already
            if last_subkey_result.descriptor.is_some() {
                (true, last_subkey_result)
            } else {
                // See if the subkey we are modifying has a last known remote value
                let last_subkey_result = inner.handle_get_remote_value(key, subkey, true).await?;
                (false, last_subkey_result)
            }
        };

        // Make sure this value would actually be newer
        if let Some(last_value) = &last_subkey_result.value {
            if value.value_data().seq() <= last_value.value_data().seq() {
                // inbound value is older or equal sequence number than the one we have, just return the one we have
                return Ok(NetworkResult::value(Some(last_value.clone())));
            }
        }

        // Get the descriptor and schema for the key
        let actual_descriptor = match last_subkey_result.descriptor {
            Some(last_descriptor) => {
                if let Some(descriptor) = descriptor {
                    // Descriptor must match last one if it is provided
                    if descriptor.cmp_no_sig(&last_descriptor) != cmp::Ordering::Equal {
                        return Ok(NetworkResult::invalid_message(
                            "setvalue descriptor does not match last descriptor",
                        ));
                    }
                } else {
                    // Descriptor was not provided always go with last descriptor
                }
                last_descriptor
            }
            None => {
                if let Some(descriptor) = descriptor {
                    descriptor
                } else {
                    // No descriptor
                    return Ok(NetworkResult::invalid_message(
                        "descriptor must be provided",
                    ));
                }
            }
        };
        let Ok(schema) = actual_descriptor.schema() else {
            return Ok(NetworkResult::invalid_message("invalid schema"));
        };

        // Validate new value with schema
        if !schema.check_subkey_value_data(actual_descriptor.owner(), subkey, value.value_data()) {
            // Validation failed, ignore this value
            return Ok(NetworkResult::invalid_message("failed schema validation"));
        }

        // Do the set and return no new value
        let res = if is_local {
            inner.handle_set_local_value(key, subkey, value).await
        } else {
            inner
                .handle_set_remote_value(key, subkey, value, actual_descriptor)
                .await
        };
        match res {
            Ok(()) => {}
            Err(VeilidAPIError::Internal { message }) => {
                apibail_internal!(message);
            }
            Err(e) => {
                return Ok(NetworkResult::invalid_message(e));
            }
        }
        Ok(NetworkResult::value(None))
    }
}
