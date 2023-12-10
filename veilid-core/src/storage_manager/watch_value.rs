use super::*;

/// The context of the outbound_watch_value operation
struct OutboundWatchValueContext {
    /// A successful watch
    pub opt_watch_value_result: Option<OutboundWatchValueResult>,
}

/// The result of the outbound_watch_value operation
#[derive(Debug, Clone)]
pub(super) struct OutboundWatchValueResult {
    /// The expiration of a successful watch
    pub expiration_ts: Timestamp,
    /// Which node accepted the watch
    pub watch_node: NodeRef,
    /// Which private route is responsible for receiving ValueChanged notifications
    pub opt_value_changed_route: Option<PublicKey>,
}

impl StorageManager {
    /// Perform a 'watch value' query on the network
    #[allow(clippy::too_many_arguments)]
    pub(super) async fn outbound_watch_value(
        &self,
        rpc_processor: RPCProcessor,
        key: TypedKey,
        subkeys: ValueSubkeyRangeSet,
        expiration: Timestamp,
        count: u32,
        safety_selection: SafetySelection,
        opt_watcher: Option<KeyPair>,
        opt_watch_node: Option<NodeRef>,
    ) -> VeilidAPIResult<Option<OutboundWatchValueResult>> {
        let routing_table = rpc_processor.routing_table();

        // Get the DHT parameters for 'WatchValue', some of which are the same for 'WatchValue' operations
        let (key_count, timeout_us) = {
            let c = self.unlocked_inner.config.get();
            (
                c.network.dht.max_find_node_count as usize,
                TimestampDuration::from(ms_to_us(c.network.dht.get_value_timeout_ms)),
            )
        };

        // Get the nodes we know are caching this value to seed the fanout
        let init_fanout_queue = if let Some(watch_node) = opt_watch_node {
            vec![watch_node]
        } else {
            let inner = self.inner.lock().await;
            inner.get_value_nodes(key)?.unwrap_or_default()
        };

        // Get the appropriate watcher key
        let watcher = opt_watcher.unwrap_or_else(|| {
            self.unlocked_inner
                .anonymous_watch_keys
                .get(key.kind)
                .unwrap()
                .value
        });

        // Make do-watch-value answer context
        let context = Arc::new(Mutex::new(OutboundWatchValueContext {
            opt_watch_value_result: None,
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
                            watcher
                        )
                        .await?
                );

                // Keep answer if we got one
                if wva.answer.expiration_ts.as_u64() > 0 {
                    if count > 0 {
                        // If we asked for a nonzero notification count, then this is an accepted watch
                        log_stor!(debug "Watch accepted: expiration_ts={}", wva.answer.expiration_ts);
                    } else {
                        // If we asked for a zero notification count, then this is a cancelled watch
                        log_stor!(debug "Watch cancelled");
                    }
                    let mut ctx = context.lock();
                    ctx.opt_watch_value_result = Some(OutboundWatchValueResult {
                        expiration_ts: wva.answer.expiration_ts,
                        watch_node: next_node.clone(),
                        opt_value_changed_route: wva.reply_private_route,
                    });
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
            if ctx.opt_watch_value_result.is_some() {
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

        match fanout_call.run(init_fanout_queue).await {
            // If we don't finish in the timeout (too much time passed without a successful watch)
            TimeoutOr::Timeout => {
                // Return the best answer we've got
                let ctx = context.lock();
                if ctx.opt_watch_value_result.is_some() {
                    log_stor!(debug "WatchValue Fanout Timeout Success");
                } else {
                    log_stor!(debug "WatchValue Fanout Timeout Failure");
                }
                Ok(ctx.opt_watch_value_result.clone())
            }
            // If we finished with done
            TimeoutOr::Value(Ok(Some(()))) => {
                // Return the best answer we've got
                let ctx = context.lock();
                if ctx.opt_watch_value_result.is_some() {
                    log_stor!(debug "WatchValue Fanout Success");
                } else {
                    log_stor!(debug "WatchValue Fanout Failure");
                }
                Ok(ctx.opt_watch_value_result.clone())
            }
            // If we ran out of nodes
            TimeoutOr::Value(Ok(None)) => {
                // Return the best answer we've got
                let ctx = context.lock();
                if ctx.opt_watch_value_result.is_some() {
                    log_stor!(debug "WatchValue Fanout Exhausted Success");
                } else {
                    log_stor!(debug "WatchValue Fanout Exhausted Failure");
                }
                Ok(ctx.opt_watch_value_result.clone())
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
        target: Target,
        watcher: CryptoKey,
    ) -> VeilidAPIResult<NetworkResult<Timestamp>> {
        let mut inner = self.lock().await?;

        // See if this is a remote or local value
        let (_is_local, opt_expiration_ts) = {
            // See if the subkey we are watching has a local value
            let opt_expiration_ts = inner
                .handle_watch_local_value(
                    key,
                    subkeys.clone(),
                    expiration,
                    count,
                    target.clone(),
                    watcher,
                )
                .await?;
            if opt_expiration_ts.is_some() {
                (true, opt_expiration_ts)
            } else {
                // See if the subkey we are watching is a remote value
                let opt_expiration_ts = inner
                    .handle_watch_remote_value(key, subkeys, expiration, count, target, watcher)
                    .await?;
                (false, opt_expiration_ts)
            }
        };

        Ok(NetworkResult::value(opt_expiration_ts.unwrap_or_default()))
    }

    /// Handle a received 'Value Changed' statement
    pub async fn inbound_value_changed(
        &self,
        key: TypedKey,
        subkeys: ValueSubkeyRangeSet,
        mut count: u32,
        value: Arc<SignedValueData>,
    ) -> VeilidAPIResult<()> {
        // Update local record store with new value
        let (res, opt_update_callback) = {
            let mut inner = self.lock().await?;

            let res = if let Some(first_subkey) = subkeys.first() {
                inner
                    .handle_set_local_value(
                        key,
                        first_subkey,
                        value.clone(),
                        WatchUpdateMode::NoUpdate,
                    )
                    .await
            } else {
                VeilidAPIResult::Ok(())
            };

            let Some(opened_record) = inner.opened_records.get_mut(&key) else {
                // Don't send update or update the ActiveWatch if this record is closed
                return res;
            };
            let Some(mut active_watch) = opened_record.active_watch() else {
                // No active watch means no callback
                return res;
            };

            if count > active_watch.count {
                // If count is greater than our requested count then this is invalid, cancel the watch
                log_stor!(debug "watch count went backward: {}: {}/{}", key, count, active_watch.count);
                // Force count to zero
                count = 0;
                opened_record.clear_active_watch();
            } else if count == 0 {
                // If count is zero, we're done, cancel the watch and the app can renew it if it wants
                log_stor!(debug "watch count finished: {}", key);
                opened_record.clear_active_watch();
            } else {
                log_stor!(
                    "watch count decremented: {}: {}/{}",
                    key,
                    count,
                    active_watch.count
                );
                active_watch.count = count;
                opened_record.set_active_watch(active_watch);
            }

            (res, inner.update_callback.clone())
        };

        // Announce ValueChanged VeilidUpdate
        if let Some(update_callback) = opt_update_callback {
            update_callback(VeilidUpdate::ValueChange(Box::new(VeilidValueChange {
                key,
                subkeys,
                count,
                value: value.value_data().clone(),
            })));
        }

        res
    }
}
