use super::*;

/// The context of the outbound_watch_value operation
struct OutboundWatchValueContext {
    /// The timestamp for the expiration of the watch we successfully got
    pub opt_expiration_ts: Option<Timestamp>,
}

impl StorageManager {
    /// Perform a 'watch value' query on the network
    pub async fn outbound_watch_value(
        &self,
        rpc_processor: RPCProcessor,
        key: TypedKey,
        subkeys: ValueSubkeyRangeSet,
        expiration: Timestamp,
        count: u32,
        safety_selection: SafetySelection,
        opt_watcher: Option<KeyPair>,
    ) -> VeilidAPIResult<Timestamp> {
        let routing_table = rpc_processor.routing_table();

        // Get the DHT parameters for 'WatchValue', some of which are the same for 'WatchValue' operations
        let (key_count, timeout_us, rpc_timeout_us) = {
            let c = self.unlocked_inner.config.get();
            (
                c.network.dht.max_find_node_count as usize,
                TimestampDuration::from(ms_to_us(c.network.dht.get_value_timeout_ms)),
                TimestampDuration::from(ms_to_us(c.network.rpc.timeout_ms)),
            )
        };

        // Get the minimum expiration timestamp we will accept
        let cur_ts = get_timestamp();
        let min_expiration_ts = cur_ts + rpc_timeout_us.as_u64();

        // Make do-watch-value answer context
        let context = Arc::new(Mutex::new(OutboundWatchValueContext {
            opt_expiration_ts: None,
        }));

        // Routine to call to generate fanout
        let call_routine = |next_node: NodeRef| {
            let rpc_processor = rpc_processor.clone();
            let context = context.clone();
            let subkeys = subkeys.clone();
            async move {
                let wva = network_result_try!(
                    rpc_processor
                        .clone()
                        .rpc_call_watch_value(
                            Destination::direct(next_node.clone()).with_safety(safety_selection),
                            key,
                            subkeys,
                            expiration,
                            count,
                            opt_watcher
                        )
                        .await?
                );

                // Keep the expiration_ts if we got one
                if wva.answer.expiration_ts.as_u64() >= min_expiration_ts {
                    log_stor!(debug "Got expiration back: expiration_ts={}", wva.answer.expiration_ts);
                    let mut ctx = context.lock();
                    ctx.opt_expiration_ts = Some(wva.answer.expiration_ts);
                }

                // Return peers if we have some
                #[cfg(feature = "network-result-extra")]
                log_stor!(debug "WatchValue fanout call returned peers {}", wva.answer.peers.len());

                Ok(NetworkResult::value(wva.answer.peers))
            }
        };

        // Routine to call to check if we're done at each step
        let check_done = |_closest_nodes: &[NodeRef]| {
            // If a watch has succeeded, return done
            let ctx = context.lock();
            if ctx.opt_expiration_ts.is_some() {
                return Some(());
            }
            None
        };

        // Call the fanout
        // Use a fixed fanout concurrency of 1 because we only want one watch
        let fanout_call = FanoutCall::new(
            routing_table.clone(),
            key,
            key_count,
            1,
            timeout_us,
            capability_fanout_node_info_filter(vec![CAP_DHT]),
            call_routine,
            check_done,
        );

        match fanout_call.run().await {
            // If we don't finish in the timeout (too much time passed without a successful watch)
            TimeoutOr::Timeout => {
                // Return the best answer we've got
                let ctx = context.lock();
                if ctx.opt_expiration_ts.is_some() {
                    log_stor!(debug "WatchValue Fanout Timeout Success");
                } else {
                    log_stor!(debug "WatchValue Fanout Timeout Failure");
                }
                Ok(ctx.opt_expiration_ts.unwrap_or_default())
            }
            // If we finished with done
            TimeoutOr::Value(Ok(Some(()))) => {
                // Return the best answer we've got
                let ctx = context.lock();
                if ctx.opt_expiration_ts.is_some() {
                    log_stor!(debug "WatchValue Fanout Success");
                } else {
                    log_stor!(debug "WatchValue Fanout Failure");
                }
                Ok(ctx.opt_expiration_ts.unwrap_or_default())
            }
            // If we ran out of nodes
            TimeoutOr::Value(Ok(None)) => {
                // Return the best answer we've got
                let ctx = context.lock();
                if ctx.opt_expiration_ts.is_some() {
                    log_stor!(debug "WatchValue Fanout Exhausted Success");
                } else {
                    log_stor!(debug "WatchValue Fanout Exhausted Failure");
                }
                Ok(ctx.opt_expiration_ts.unwrap_or_default())
            }
            // Failed
            TimeoutOr::Value(Err(e)) => {
                // If we finished with an error, return that
                log_stor!(debug "WatchValue Fanout Error: {}", e);
                Err(e.into())
            }
        }
    }

    /// Handle a received 'Watch Value' query
    pub async fn inbound_watch_value(
        &self,
        key: TypedKey,
        subkeys: ValueSubkeyRangeSet,
        expiration: Timestamp,
        count: u32,
        // xxx more here
    ) -> VeilidAPIResult<NetworkResult<SubkeyResult>> {
        let mut inner = self.lock().await?;
        let res = match inner
            .handle_watch_remote_value(key, subkeys, expiration, count)
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
