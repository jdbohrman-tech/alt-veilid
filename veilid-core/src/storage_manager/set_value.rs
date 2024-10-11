use super::*;

/// The context of the outbound_set_value operation
struct OutboundSetValueContext {
    /// The latest value of the subkey, may be the value passed in
    pub value: Arc<SignedValueData>,
    /// The nodes that have set the value so far (up to the consensus count)
    pub value_nodes: Vec<NodeRef>,
    /// The number of non-sets since the last set we have received
    pub missed_since_last_set: usize,
    /// The parsed schema from the descriptor if we have one
    pub schema: DHTSchema,
    /// If we should send a partial update with the current context
    pub send_partial_update: bool,
}

/// The result of the outbound_set_value operation
#[derive(Clone, Debug)]
pub(super) struct OutboundSetValueResult {
    /// Fanout result
    pub fanout_result: FanoutResult,
    /// The value that was set
    pub signed_value_data: Arc<SignedValueData>,
}

impl StorageManager {
    /// Perform a 'set value' query on the network
    #[instrument(level = "trace", target = "dht", skip_all, err)]
    pub(super) async fn outbound_set_value(
        &self,
        rpc_processor: RPCProcessor,
        key: TypedKey,
        subkey: ValueSubkey,
        safety_selection: SafetySelection,
        value: Arc<SignedValueData>,
        descriptor: Arc<SignedValueDescriptor>,
    ) -> VeilidAPIResult<flume::Receiver<VeilidAPIResult<OutboundSetValueResult>>> {
        let routing_table = rpc_processor.routing_table();
        let routing_domain = RoutingDomain::PublicInternet;

        // Get the DHT parameters for 'SetValue'
        let (key_count, get_consensus_count, set_consensus_count, fanout, timeout_us) = {
            let c = self.unlocked_inner.config.get();
            (
                c.network.dht.max_find_node_count as usize,
                c.network.dht.get_value_count as usize,
                c.network.dht.set_value_count as usize,
                c.network.dht.set_value_fanout as usize,
                TimestampDuration::from(ms_to_us(c.network.dht.set_value_timeout_ms)),
            )
        };

        // Get the nodes we know are caching this value to seed the fanout
        let init_fanout_queue = {
            let inner = self.inner.lock().await;
            inner
                .get_value_nodes(key)?
                .unwrap_or_default()
                .into_iter()
                .filter(|x| {
                    x.node_info(routing_domain)
                        .map(|ni| ni.has_all_capabilities(&[CAP_DHT]))
                        .unwrap_or_default()
                })
                .collect()
        };

        // Make the return channel
        let (out_tx, out_rx) = flume::unbounded::<VeilidAPIResult<OutboundSetValueResult>>();

        // Make do-set-value answer context
        let schema = descriptor.schema()?;
        let context = Arc::new(Mutex::new(OutboundSetValueContext {
            value,
            value_nodes: vec![],
            missed_since_last_set: 0,
            schema,
            send_partial_update: false,
        }));

        // Routine to call to generate fanout
        let call_routine = {
            let context = context.clone();
            let rpc_processor = rpc_processor.clone();

            move |next_node: NodeRef| {
                let rpc_processor = rpc_processor.clone();
                let context = context.clone();
                let descriptor = descriptor.clone();
                async move {
                    let send_descriptor = true; // xxx check if next_node needs the descriptor or not, see issue #203

                    // get most recent value to send
                    let value = {
                        let ctx = context.lock();
                        ctx.value.clone()
                    };

                    // send across the wire
                    let sva = network_result_try!(
                        rpc_processor
                            .clone()
                            .rpc_call_set_value(
                                Destination::direct(next_node.routing_domain_filtered(routing_domain))
                                    .with_safety(safety_selection),
                                key,
                                subkey,
                                (*value).clone(),
                                (*descriptor).clone(),
                                send_descriptor,
                            )
                            .await?
                    );

                    // If the node was close enough to possibly set the value
                    let mut ctx = context.lock();
                    if !sva.answer.set {
                        ctx.missed_since_last_set += 1;

                        // Return peers if we have some
                        log_network_result!(debug "SetValue missed: {}, fanout call returned peers {}", ctx.missed_since_last_set, sva.answer.peers.len());
                        return Ok(NetworkResult::value(FanoutCallOutput{peer_info_list:sva.answer.peers}));
                    }

                    // See if we got a value back
                    let Some(value) = sva.answer.value else {
                        // No newer value was found and returned, so increase our consensus count
                        ctx.value_nodes.push(next_node);
                        ctx.missed_since_last_set = 0;
                        // Send an update since it was set
                        if ctx.value_nodes.len() == 1 {
                            ctx.send_partial_update = true;
                        }

                        // Return peers if we have some
                        log_network_result!(debug "SetValue returned no value, fanout call returned peers {}", sva.answer.peers.len());
                        return Ok(NetworkResult::value(FanoutCallOutput{peer_info_list:sva.answer.peers}));
                    };

                    // Keep the value if we got one and it is newer and it passes schema validation
                    log_dht!(debug "SetValue got value back: len={} seq={}", value.value_data().data().len(), value.value_data().seq());

                    // Validate with schema
                    if !ctx.schema.check_subkey_value_data(
                        descriptor.owner(),
                        subkey,
                        value.value_data(),
                    ) {
                        // Validation failed, ignore this value and pretend we never saw this node
                        return Ok(NetworkResult::invalid_message(format!(
                            "Schema validation failed on subkey {}",
                            subkey
                        )));
                    }

                    // If we got a value back it should be different than the one we are setting
                    // But in the case of a benign bug, we can just move to the next node
                    if ctx.value.value_data() == value.value_data() {
                        ctx.value_nodes.push(next_node);
                        ctx.missed_since_last_set = 0;

                        // Send an update since it was set
                        if ctx.value_nodes.len() == 1 {
                            ctx.send_partial_update = true;
                        }

                        return Ok(NetworkResult::value(FanoutCallOutput{peer_info_list:sva.answer.peers}));
                    }

                    // We have a prior value, ensure this is a newer sequence number
                    let prior_seq = ctx.value.value_data().seq();
                    let new_seq = value.value_data().seq();
                    if new_seq < prior_seq {
                        // If the sequence number is older node should have not returned a value here.
                        // Skip this node and its closer list because it is misbehaving
                        // Ignore this value and pretend we never saw this node
                        return Ok(NetworkResult::invalid_message("Sequence number is older"));
                    }

                    // If the sequence number is greater or equal, keep it
                    // even if the sequence number is the same, accept all conflicts in an attempt to resolve them
                    ctx.value = Arc::new(value);
                    // One node has shown us this value so far
                    ctx.value_nodes = vec![next_node];
                    ctx.missed_since_last_set = 0;
                    // Send an update since the value changed
                    ctx.send_partial_update = true;

                    Ok(NetworkResult::value(FanoutCallOutput{peer_info_list:sva.answer.peers}))
                }.instrument(tracing::trace_span!("fanout call_routine"))
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
                    ctx.send_partial_update = false;

                    // return partial result
                    let fanout_result = FanoutResult {
                        kind: FanoutResultKind::Partial,
                        value_nodes: ctx.value_nodes.clone(),
                    };
                    let out = OutboundSetValueResult {
                        fanout_result,
                        signed_value_data: ctx.value.clone(),
                    };
                    log_dht!(debug "Sending partial SetValue result: {:?}", out);

                    if let Err(e) = out_tx.send(Ok(out)) {
                        log_dht!(debug "Sending partial SetValue result failed: {}", e);
                    }
                }

                // If we have reached set consensus (the max consensus we care about), return done
                if ctx.value_nodes.len() >= set_consensus_count {
                    return Some(());
                }

                // If we have missed get_consensus count (the minimum consensus we care about) or more since our last set, return done
                // This keeps the traversal from searching too many nodes when we aren't converging
                // Only do this if we have gotten at least the get_consensus (the minimum consensus we care about)
                if ctx.value_nodes.len() >= get_consensus_count
                    && ctx.missed_since_last_set >= get_consensus_count
                {
                    return Some(());
                }
                None
            }
        };

        // Call the fanout in a spawned task
        spawn(
            "outbound_set_value fanout",
            Box::pin(
                async move {
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
                            log_dht!(debug "SetValue fanout error: {}", e);
                            if let Err(e) = out_tx.send(Err(e.into())) {
                                log_dht!(debug "Sending SetValue fanout error failed: {}", e);
                            }
                            return;
                        }
                    };

                    let ctx = context.lock();
                    let fanout_result = FanoutResult {
                        kind,
                        value_nodes: ctx.value_nodes.clone(),
                    };
                    log_dht!(debug "SetValue Fanout: {:?}", fanout_result);

                    if let Err(e) = out_tx.send(Ok(OutboundSetValueResult {
                        fanout_result,
                        signed_value_data: ctx.value.clone(),
                    })) {
                        log_dht!(debug "Sending SetValue result failed: {}", e);
                    }
                }
                .instrument(tracing::trace_span!("outbound_set_value fanout routine")),
            ),
        )
        .detach();

        Ok(out_rx)
    }

    #[instrument(level = "trace", target = "dht", skip_all)]
    pub(super) fn process_deferred_outbound_set_value_result_inner(
        &self,
        inner: &mut StorageManagerInner,
        res_rx: flume::Receiver<Result<set_value::OutboundSetValueResult, VeilidAPIError>>,
        key: TypedKey,
        subkey: ValueSubkey,
        last_value_data: ValueData,
        safety_selection: SafetySelection,
    ) {
        let this = self.clone();
        let last_value_data = Arc::new(Mutex::new(last_value_data));
        inner.process_deferred_results(
            res_rx,
            Box::new(
                move |result: VeilidAPIResult<set_value::OutboundSetValueResult>| -> SendPinBoxFuture<bool> {
                    let this = this.clone();
                    let last_value_data = last_value_data.clone();
                    Box::pin(async move {
                        let result = match result {
                            Ok(v) => v,
                            Err(e) => {
                                log_rtab!(debug "Deferred fanout error: {}", e);
                                return false;
                            }
                        };
                        let is_partial = result.fanout_result.kind.is_partial();
                        let lvd = last_value_data.lock().clone();
                        let value_data = match this.process_outbound_set_value_result(key, subkey, lvd, safety_selection, result).await {
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
                        let changed = {
                            let mut lvd = last_value_data.lock();
                            if lvd.seq() != value_data.seq() {
                                *lvd = value_data.clone();
                                true
                            } else {
                                false
                            }
                        };
                        if changed {
                            if let Err(e) = this.update_callback_value_change(key,ValueSubkeyRangeSet::single(subkey), u32::MAX, Some(value_data)).await {
                                log_rtab!(debug "Failed sending deferred fanout value change: {}", e);
                            }
                        }

                        // Return done
                        false
                    }.instrument(tracing::trace_span!("outbound_set_value deferred results")))
                },
            ),
        );
    }

    #[instrument(level = "trace", target = "stor", skip_all, err)]
    pub(super) async fn process_outbound_set_value_result(
        &self,
        key: TypedKey,
        subkey: ValueSubkey,
        last_value_data: ValueData,
        safety_selection: SafetySelection,
        result: set_value::OutboundSetValueResult,
    ) -> Result<Option<ValueData>, VeilidAPIError> {
        // Regain the lock after network access
        let mut inner = self.lock().await?;

        // Report on fanout result offline
        let was_offline = self.check_fanout_set_offline(key, subkey, &result.fanout_result);
        if was_offline {
            // Failed to write, try again later
            inner.add_offline_subkey_write(key, subkey, safety_selection);
        }

        // Keep the list of nodes that returned a value for later reference
        inner.process_fanout_results(key, core::iter::once((subkey, &result.fanout_result)), true);

        // Return the new value if it differs from what was asked to set
        if result.signed_value_data.value_data() != &last_value_data {
            // Record the newer value and send and update since it is different than what we just set
            inner
                .handle_set_local_value(
                    key,
                    subkey,
                    result.signed_value_data.clone(),
                    WatchUpdateMode::UpdateAll,
                )
                .await?;

            return Ok(Some(result.signed_value_data.value_data().clone()));
        }

        // If the original value was set, return None
        Ok(None)
    }

    /// Handle a received 'Set Value' query
    /// Returns a None if the value passed in was set
    /// Returns a Some(current value) if the value was older and the current value was kept
    #[instrument(level = "trace", target = "dht", skip_all)]
    pub async fn inbound_set_value(
        &self,
        key: TypedKey,
        subkey: ValueSubkey,
        value: Arc<SignedValueData>,
        descriptor: Option<Arc<SignedValueDescriptor>>,
        target: Target,
    ) -> VeilidAPIResult<NetworkResult<Option<Arc<SignedValueData>>>> {
        let mut inner = self.lock().await?;

        // See if this is a remote or local value
        let (is_local, last_get_result) = {
            // See if the subkey we are modifying has a last known local value
            let last_get_result = inner.handle_get_local_value(key, subkey, true).await?;
            // If this is local, it must have a descriptor already
            if last_get_result.opt_descriptor.is_some() {
                (true, last_get_result)
            } else {
                // See if the subkey we are modifying has a last known remote value
                let last_get_result = inner.handle_get_remote_value(key, subkey, true).await?;
                (false, last_get_result)
            }
        };

        // Make sure this value would actually be newer
        if let Some(last_value) = &last_get_result.opt_value {
            if value.value_data().seq() < last_value.value_data().seq() {
                // inbound value is older than the sequence number that we have, just return the one we have
                return Ok(NetworkResult::value(Some(last_value.clone())));
            } else if value.value_data().seq() == last_value.value_data().seq() {
                // inbound value is equal to the sequence number that we have
                // if the value is the same including the writer, return nothing,
                // otherwise return the existing value because it was here first
                if value.value_data() == last_value.value_data() {
                    return Ok(NetworkResult::value(None));
                }
                // sequence number is the same but there's a value conflict, return what we have
                return Ok(NetworkResult::value(Some(last_value.clone())));
            }
        }

        // Get the descriptor and schema for the key
        let actual_descriptor = match last_get_result.opt_descriptor {
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
            inner
                .handle_set_local_value(key, subkey, value, WatchUpdateMode::ExcludeTarget(target))
                .await
        } else {
            inner
                .handle_set_remote_value(
                    key,
                    subkey,
                    value,
                    actual_descriptor,
                    WatchUpdateMode::ExcludeTarget(target),
                )
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
