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
    /// If we should send a partial update with the current context
    pub send_partial_update: bool,
}

/// The result of the outbound_get_value operation
#[derive(Clone, Debug)]
pub(super) struct OutboundGetValueResult {
    /// Fanout result
    pub fanout_result: FanoutResult,
    /// The subkey that was retrieved
    pub get_result: GetResult,
}

impl StorageManager {
    /// Perform a 'get value' query on the network
    #[instrument(level = "trace", target = "dht", skip_all, err)]
    pub(super) async fn outbound_get_value(
        &self,
        rpc_processor: RPCProcessor,
        key: TypedKey,
        subkey: ValueSubkey,
        safety_selection: SafetySelection,
        last_get_result: GetResult,
    ) -> VeilidAPIResult<flume::Receiver<VeilidAPIResult<OutboundGetValueResult>>> {
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

        // Get the nodes we know are caching this value to seed the fanout
        let init_fanout_queue = {
            let inner = self.inner.lock().await;
            inner.get_value_nodes(key)?.unwrap_or_default()
        };

        // Parse the schema
        let schema = if let Some(d) = &last_get_result.opt_descriptor {
            Some(d.schema()?)
        } else {
            None
        };

        // Make the return channel
        let (out_tx, out_rx) = flume::unbounded::<VeilidAPIResult<OutboundGetValueResult>>();

        // Make do-get-value answer context
        let context = Arc::new(Mutex::new(OutboundGetValueContext {
            value: last_get_result.opt_value,
            value_nodes: vec![],
            descriptor: last_get_result.opt_descriptor.clone(),
            schema,
            send_partial_update: false,
        }));

        // Routine to call to generate fanout
        let call_routine = {
            let context = context.clone();
            let rpc_processor = rpc_processor.clone();
            move |next_node: NodeRef| {
                let context = context.clone();
                let rpc_processor = rpc_processor.clone();
                let last_descriptor = last_get_result.opt_descriptor.clone();
                async move {
                    let gva = network_result_try!(
                        rpc_processor
                            .clone()
                            .rpc_call_get_value(
                                Destination::direct(next_node.clone())
                                    .with_safety(safety_selection),
                                key,
                                subkey,
                                last_descriptor.map(|x| (*x).clone()),
                            )
                            .await?
                    );
                    let mut ctx = context.lock();

                    // Keep the descriptor if we got one. If we had a last_descriptor it will
                    // already be validated by rpc_call_get_value
                    if let Some(descriptor) = gva.answer.descriptor {
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
                    let Some(value) = gva.answer.value else {
                        // Return peers if we have some
                        log_network_result!(debug "GetValue returned no value, fanout call returned peers {}", gva.answer.peers.len());

                        return Ok(NetworkResult::value(gva.answer.peers))
                    };

                    log_dht!(debug "GetValue got value back: len={} seq={}", value.value_data().data().len(), value.value_data().seq());

                    // Ensure we have a schema and descriptor
                    let (Some(descriptor), Some(schema)) = (&ctx.descriptor, &ctx.schema)
                    else {
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
                                return Ok(NetworkResult::invalid_message(
                                    "value data mismatch",
                                ));
                            }
                            // Increase the consensus count for the existing value
                            ctx.value_nodes.push(next_node);
                        } else if new_seq > prior_seq {
                            // If the sequence number is greater, start over with the new value
                            ctx.value = Some(Arc::new(value));
                            // One node has shown us this value so far
                            ctx.value_nodes = vec![next_node];
                            // Send an update since the value changed
                            ctx.send_partial_update = true;
                        } else {
                            // If the sequence number is older, ignore it
                        }
                    } else {
                        // If we have no prior value, keep it
                        ctx.value = Some(Arc::new(value));
                        // One node has shown us this value so far
                        ctx.value_nodes = vec![next_node];
                        // Send an update since the value changed
                        ctx.send_partial_update = true;
                    }
                    
                    // Return peers if we have some
                    log_network_result!(debug "GetValue fanout call returned peers {}", gva.answer.peers.len());

                    Ok(NetworkResult::value(gva.answer.peers))
                }.instrument(tracing::trace_span!("outbound_get_value fanout routine"))
            }
        };

        // Routine to call to check if we're done at each step
        let check_done = {
            let context = context.clone();
            let out_tx = out_tx.clone();
            move |_closest_nodes: &[NodeRef]| {
                let mut ctx = context.lock();

                // send partial update if desired
                if ctx.send_partial_update {
                    ctx.send_partial_update=false;

                    // return partial result
                    let fanout_result = FanoutResult {
                        kind: FanoutResultKind::Partial,
                        value_nodes: ctx.value_nodes.clone(),
                    };
                    if let Err(e) = out_tx.send(Ok(OutboundGetValueResult {
                        fanout_result,
                        get_result: GetResult {
                            opt_value: ctx.value.clone(),
                            opt_descriptor: ctx.descriptor.clone(),
                        },
                    })) {
                        log_dht!(debug "Sending partial GetValue result failed: {}", e);
                    }
                }

                // If we have reached sufficient consensus, return done
                if ctx.value.is_some()
                    && ctx.descriptor.is_some()
                    && ctx.value_nodes.len() >= consensus_count
                {
                    return Some(());
                }
                None
            }
        };

        // Call the fanout in a spawned task
        spawn("outbound_get_value fanout", Box::pin(async move {
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

            let kind = match fanout_call.run(init_fanout_queue).await {
                // If we don't finish in the timeout (too much time passed checking for consensus)
                TimeoutOr::Timeout => FanoutResultKind::Timeout,
                // If we finished with or without consensus (enough nodes returning the same value)
                TimeoutOr::Value(Ok(Some(()))) => FanoutResultKind::Finished,
                // If we ran out of nodes before getting consensus)
                TimeoutOr::Value(Ok(None)) => FanoutResultKind::Exhausted,
                // Failed
                TimeoutOr::Value(Err(e)) => {
                    // If we finished with an error, return that
                    log_dht!(debug "GetValue fanout error: {}", e);
                    if let Err(e) = out_tx.send(Err(e.into())) {
                        log_dht!(debug "Sending GetValue fanout error failed: {}", e);
                    }
                    return;
                }
            };

            let ctx = context.lock();
            let fanout_result = FanoutResult {
                kind,
                value_nodes: ctx.value_nodes.clone(),
            };
            log_network_result!(debug "GetValue Fanout: {:?}", fanout_result);

            if let Err(e) = out_tx.send(Ok(OutboundGetValueResult {
                fanout_result,
                get_result: GetResult {
                    opt_value: ctx.value.clone(),
                    opt_descriptor: ctx.descriptor.clone(),
                },
            })) {
                log_dht!(debug "Sending GetValue result failed: {}", e);
            }
        }.instrument(tracing::trace_span!("outbound_get_value result"))))
        .detach();

        Ok(out_rx)
    }

    #[instrument(level = "trace", target = "dht", skip_all)]
    pub(super) fn process_deferred_outbound_get_value_result_inner(&self, inner: &mut StorageManagerInner, res_rx: flume::Receiver<Result<get_value::OutboundGetValueResult, VeilidAPIError>>, key: TypedKey, subkey: ValueSubkey, last_seq: ValueSeqNum) {
        let this = self.clone();
        inner.process_deferred_results(
            res_rx,
            Box::new(
                move |result: VeilidAPIResult<get_value::OutboundGetValueResult>| -> SendPinBoxFuture<bool> {
                    let this = this.clone();
                    Box::pin(async move { 
                        let result = match result {
                            Ok(v) => v,
                            Err(e) => {
                                log_rtab!(debug "Deferred fanout error: {}", e);
                                return false;
                            }
                        };
                        let is_partial = result.fanout_result.kind.is_partial();
                        let value_data = match this.process_outbound_get_value_result(key, subkey, Some(last_seq), result).await {
                            Ok(Some(v)) => v,
                            Ok(None) => {
                                return is_partial;
                            }
                            Err(e) => {
                                log_rtab!(debug "Deferred fanout error: {}", e);
                                return false;
                            }
                        };
                        if is_partial {
                            // If more partial results show up, don't send an update until we're done
                            return true;
                        }
                        // If we processed the final result, possibly send an update 
                        // if the sequence number changed since our first partial update
                        // Send with a max count as this is not attached to any watch
                        if last_seq != value_data.seq() {
                            if let Err(e) = this.update_callback_value_change(key,ValueSubkeyRangeSet::single(subkey), u32::MAX, Some(value_data)).await {
                                log_rtab!(debug "Failed sending deferred fanout value change: {}", e);
                            }
                        }

                        // Return done
                        false
                    }.instrument(tracing::trace_span!("outbound_get_value deferred results")))
                },
            ),
        );
    }

    #[instrument(level = "trace", target = "dht", skip_all)]
    pub(super) async fn process_outbound_get_value_result(&self, key: TypedKey, subkey: ValueSubkey, opt_last_seq: Option<u32>, result: get_value::OutboundGetValueResult) -> Result<Option<ValueData>, VeilidAPIError> {
        // See if we got a value back
        let Some(get_result_value) = result.get_result.opt_value else {
            // If we got nothing back then we also had nothing beforehand, return nothing
            return Ok(None);
        };

        // Keep the list of nodes that returned a value for later reference
        let mut inner = self.lock().await?;
        
        inner.process_fanout_results(
            key,
            core::iter::once((subkey, &result.fanout_result)),
            false,
        );

        // If we got a new value back then write it to the opened record
        if Some(get_result_value.value_data().seq()) != opt_last_seq {
            inner
                .handle_set_local_value(
                    key,
                    subkey,
                    get_result_value.clone(),
                    WatchUpdateMode::UpdateAll,
                )
                .await?;
        }
        Ok(Some(get_result_value.value_data().clone()))   
    }

    /// Handle a received 'Get Value' query
    #[instrument(level = "trace", target = "dht", skip_all)]
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
