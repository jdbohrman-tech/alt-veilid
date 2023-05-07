use super::*;

/// The result of the do_get_value_operation
pub struct DoGetValueResult {
    /// The subkey value if we got one
    pub value: Option<SignedValueData>,
    /// The descriptor if we got a fresh one or empty if no descriptor was needed
    pub descriptor: Option<SignedValueDescriptor>,
}

/// The context of the do_get_value operation
struct DoGetValueContext {
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

    pub async fn do_get_value(
        &self,
        rpc_processor: RPCProcessor,
        key: TypedKey,
        subkey: ValueSubkey,
        last_value: Option<SignedValueData>,
        last_descriptor: Option<SignedValueDescriptor>,
        safety_selection: SafetySelection,
    ) -> Result<DoGetValueResult, VeilidAPIError> {
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
        let schema = if let Some(d) = &last_descriptor {
            Some(d.schema()?)
        } else {
            None
        };
        let context = Arc::new(Mutex::new(DoGetValueContext {
            value: last_value,
            value_count: 0,
            descriptor: last_descriptor.clone(),
            schema,
        }));

        // Routine to call to generate fanout
        let call_routine = |next_node: NodeRef| {
            let rpc_processor = rpc_processor.clone();
            let context = context.clone();
            let last_descriptor = last_descriptor.clone();
            async move {
                match rpc_processor
                    .clone()
                    .rpc_call_get_value(
                        Destination::direct(next_node).with_safety(safety_selection),
                        key,
                        subkey,
                        last_descriptor,
                    )
                    .await
                {
                    Ok(v) => {
                        let v = network_result_value_or_log!(v => {
                            // Any other failures, just try the next node
                            return Ok(None);
                        });

                        // Keep the descriptor if we got one. If we had a last_descriptor it will
                        // already be validated by rpc_call_get_value
                        if let Some(descriptor) = v.answer.descriptor {
                            let mut ctx = context.lock();
                            if ctx.descriptor.is_none() && ctx.schema.is_none() {
                                ctx.schema =
                                    Some(descriptor.schema().map_err(RPCError::invalid_format)?);
                                ctx.descriptor = Some(descriptor);
                            }
                        }

                        // Keep the value if we got one and it is newer and it passes schema validation
                        if let Some(value) = v.answer.value {
                            let mut ctx = context.lock();

                            // Ensure we have a schema and descriptor
                            let (Some(descriptor), Some(schema)) = (&ctx.descriptor, &ctx.schema) else {
                                // Got a value but no descriptor for it
                                // Move to the next node
                                return Ok(None);
                            };

                            // Validate with schema
                            if !schema.check_subkey_value_data(
                                descriptor.owner(),
                                subkey,
                                value.value_data(),
                            ) {
                                // Validation failed, ignore this value
                                // Move to the next node
                                return Ok(None);
                            }

                            // If we have a prior value, see if this is a newer sequence number
                            if let Some(prior_value) = &ctx.value {
                                let prior_seq = prior_value.value_data().seq();
                                let new_seq = value.value_data().seq();

                                if new_seq == prior_seq {
                                    // If sequence number is the same, the data should be the same
                                    if prior_value.value_data() != value.value_data() {
                                        // Move to the next node
                                        return Ok(None);
                                    }
                                    // Increase the consensus count for the existing value
                                    ctx.value_count += 1;
                                } else if new_seq > prior_seq {
                                    // If the sequence number is greater, go with it
                                    ctx.value = Some(value);
                                    ctx.value_count = 1;
                                } else {
                                    // If the sequence number is older, ignore it
                                }
                            }
                        }

                        // Return peers if we have some
                        Ok(Some(v.answer.peers))
                    }
                    Err(e) => Err(e),
                }
            }
        };

        // Routine to call to check if we're done at each step
        let check_done = |_closest_nodes: &[NodeRef]| {
            // If we have reached sufficient consensus, return done
            let ctx = context.lock();
            if ctx.value.is_some() && ctx.descriptor.is_some() && ctx.value_count >= consensus_count {
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
            call_routine,
            check_done,
        );

        match fanout_call.run().await {
            // If we don't finish in the timeout (too much time passed checking for consensus)
            TimeoutOr::Timeout | 
            // If we finished with consensus (enough nodes returning the same value)
            TimeoutOr::Value(Ok(Some(()))) | 
            // If we finished without consensus (ran out of nodes before getting consensus)
            TimeoutOr::Value(Ok(None)) => {
                // Return the best answer we've got
                let ctx = context.lock();
                Ok(DoGetValueResult{
                    value: ctx.value.clone(),
                    descriptor: ctx.descriptor.clone(),
                })
            }
            // Failed
            TimeoutOr::Value(Err(e)) => {
                // If we finished with an error, return that
                Err(e.into())
            }
        }
    }
}
