use super::*;
use futures_util::*;
use stop_token::future::FutureExt as _;

#[derive(Debug)]
enum OfflineSubkeyWriteResult {
    Finished(set_value::OutboundSetValueResult),
    Cancelled,
    Dropped,
}

#[derive(Debug)]
struct WorkItem {
    key: TypedKey,
    safety_selection: SafetySelection,
    subkeys: ValueSubkeyRangeSet,
}

#[derive(Debug)]
struct WorkItemResult {
    key: TypedKey,
    written_subkeys: ValueSubkeyRangeSet,
    fanout_results: Vec<(ValueSubkey, FanoutResult)>,
}

impl StorageManager {
    // Write a single offline subkey
    #[instrument(level = "trace", target = "stor", skip_all, err)]
    async fn write_single_offline_subkey(
        self,
        stop_token: StopToken,
        key: TypedKey,
        subkey: ValueSubkey,
        safety_selection: SafetySelection,
    ) -> EyreResult<OfflineSubkeyWriteResult> {
        let Some(rpc_processor) = self.online_writes_ready().await? else {
            // Cancel this operation because we're offline
            return Ok(OfflineSubkeyWriteResult::Cancelled);
        };
        let get_result = {
            let mut inner = self.lock().await?;
            inner.handle_get_local_value(key, subkey, true).await
        };
        let Ok(get_result) = get_result else {
            log_stor!(debug "Offline subkey write had no subkey result: {}:{}", key, subkey);
            // drop this one
            return Ok(OfflineSubkeyWriteResult::Dropped);
        };
        let Some(value) = get_result.opt_value else {
            log_stor!(debug "Offline subkey write had no subkey value: {}:{}", key, subkey);
            // drop this one
            return Ok(OfflineSubkeyWriteResult::Dropped);
        };
        let Some(descriptor) = get_result.opt_descriptor else {
            log_stor!(debug "Offline subkey write had no descriptor: {}:{}", key, subkey);
            return Ok(OfflineSubkeyWriteResult::Dropped);
        };
        log_stor!(debug "Offline subkey write: {}:{} len={}", key, subkey, value.value_data().data().len());
        let osvres = self
            .outbound_set_value(
                rpc_processor,
                key,
                subkey,
                safety_selection,
                value.clone(),
                descriptor,
            )
            .await;
        match osvres {
            Ok(res_rx) => {
                while let Ok(Ok(res)) = res_rx.recv_async().timeout_at(stop_token.clone()).await {
                    match res {
                        Ok(result) => {
                            let partial = result.fanout_result.kind.is_partial();
                            // Skip partial results in offline subkey write mode
                            if partial {
                                continue;
                            }

                            // Set the new value if it differs from what was asked to set
                            if result.signed_value_data.value_data() != value.value_data() {
                                // Record the newer value and send and update since it is different than what we just set
                                let mut inner = self.lock().await?;
                                inner
                                    .handle_set_local_value(
                                        key,
                                        subkey,
                                        result.signed_value_data.clone(),
                                        WatchUpdateMode::UpdateAll,
                                    )
                                    .await?;
                            }

                            return Ok(OfflineSubkeyWriteResult::Finished(result));
                        }
                        Err(e) => {
                            log_stor!(debug "failed to get offline subkey write result: {}:{} {}", key, subkey, e);
                            return Ok(OfflineSubkeyWriteResult::Cancelled);
                        }
                    }
                }
                log_stor!(debug "writing offline subkey did not complete {}:{}", key, subkey);
                return Ok(OfflineSubkeyWriteResult::Cancelled);
            }
            Err(e) => {
                log_stor!(debug "failed to write offline subkey: {}:{} {}", key, subkey, e);
                return Ok(OfflineSubkeyWriteResult::Cancelled);
            }
        }
    }

    // Write a set of subkeys of the same key
    #[instrument(level = "trace", target = "stor", skip_all, err)]
    async fn process_work_item(
        self,
        stop_token: StopToken,
        work_item: WorkItem,
    ) -> EyreResult<WorkItemResult> {
        let mut written_subkeys = ValueSubkeyRangeSet::new();
        let mut fanout_results = Vec::<(ValueSubkey, FanoutResult)>::new();

        for subkey in work_item.subkeys.iter() {
            if poll!(stop_token.clone()).is_ready() {
                break;
            }

            let result = match self
                .clone()
                .write_single_offline_subkey(
                    stop_token.clone(),
                    work_item.key,
                    subkey,
                    work_item.safety_selection,
                )
                .await?
            {
                OfflineSubkeyWriteResult::Finished(r) => r,
                OfflineSubkeyWriteResult::Cancelled => {
                    // Stop now and return what we have
                    break;
                }
                OfflineSubkeyWriteResult::Dropped => {
                    // Don't process this item any more but continue
                    written_subkeys.insert(subkey);
                    continue;
                }
            };

            // Process non-partial setvalue result
            let was_offline =
                self.check_fanout_set_offline(work_item.key, subkey, &result.fanout_result);
            if !was_offline {
                written_subkeys.insert(subkey);
            }
            fanout_results.push((subkey, result.fanout_result));
        }

        Ok(WorkItemResult {
            key: work_item.key,
            written_subkeys,
            fanout_results,
        })
    }

    // Process all results
    fn prepare_all_work(
        offline_subkey_writes: HashMap<TypedKey, OfflineSubkeyWrite>,
    ) -> VecDeque<WorkItem> {
        offline_subkey_writes
            .into_iter()
            .map(|(key, v)| WorkItem {
                key,
                safety_selection: v.safety_selection,
                subkeys: v.subkeys_in_flight,
            })
            .collect()
    }

    // Process all results
    #[instrument(level = "trace", target = "stor", skip_all)]
    fn process_single_result_inner(inner: &mut StorageManagerInner, result: WorkItemResult) {
        // Debug print the result
        log_stor!(debug "Offline write result: {:?}", result);

        // Get the offline subkey write record
        match inner.offline_subkey_writes.entry(result.key) {
            std::collections::hash_map::Entry::Occupied(mut o) => {
                let finished = {
                    let osw = o.get_mut();

                    // Mark in-flight subkeys as having been completed
                    let subkeys_still_offline =
                        osw.subkeys_in_flight.difference(&result.written_subkeys);
                    // Now any left over are still offline, so merge them with any subkeys that have been added while we were working
                    osw.subkeys = osw.subkeys.union(&subkeys_still_offline);
                    // And clear the subkeys in flight since we're done with this key for now
                    osw.subkeys_in_flight.clear();

                    osw.subkeys.is_empty()
                };
                if finished {
                    log_stor!(debug "Offline write finished key {}", result.key);
                    o.remove();
                }
            }
            std::collections::hash_map::Entry::Vacant(_) => {
                panic!(
                    "offline write work items should always be on offline_subkey_writes entries that exist"
                )
            }
        }

        // Keep the list of nodes that returned a value for later reference
        inner.process_fanout_results(
            result.key,
            result.fanout_results.iter().map(|x| (x.0, &x.1)),
            true,
        );
    }

    #[instrument(level = "trace", target = "stor", skip_all, err)]
    pub(crate) async fn process_offline_subkey_writes(
        self,
        stop_token: StopToken,
        work_items: Arc<Mutex<VecDeque<WorkItem>>>,
    ) -> EyreResult<()> {
        // Process all work items
        loop {
            let Some(work_item) = work_items.lock().pop_front() else {
                break;
            };
            let result = self
                .clone()
                .process_work_item(stop_token.clone(), work_item)
                .await?;
            {
                let mut inner = self.lock().await?;
                Self::process_single_result_inner(&mut inner, result);
            }
        }

        Ok(())
    }

    // Best-effort write subkeys to the network that were written offline
    #[instrument(level = "trace", target = "stor", skip_all, err)]
    pub(crate) async fn offline_subkey_writes_task_routine(
        self,
        stop_token: StopToken,
        _last_ts: Timestamp,
        _cur_ts: Timestamp,
    ) -> EyreResult<()> {
        // Operate on a copy of the offline subkey writes map
        let work_items = {
            let mut inner = self.lock().await?;
            // Move the current set of writes to 'in flight'
            for osw in &mut inner.offline_subkey_writes {
                osw.1.subkeys_in_flight = mem::take(&mut osw.1.subkeys);
            }

            // Prepare items to work on
            Arc::new(Mutex::new(Self::prepare_all_work(
                inner.offline_subkey_writes.clone(),
            )))
        };

        // Process everything
        let res = self
            .clone()
            .process_offline_subkey_writes(stop_token, work_items)
            .await;

        // Ensure nothing is left in-flight when returning even due to an error
        {
            let mut inner = self.lock().await?;
            for osw in &mut inner.offline_subkey_writes {
                osw.1.subkeys = osw
                    .1
                    .subkeys
                    .union(&mem::take(&mut osw.1.subkeys_in_flight));
            }
        }

        res
    }
}
