use super::*;
use futures_util::{stream::unfold, *};
use stop_token::future::FutureExt as _;

impl_veilid_log_facility!("stor");

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OfflineSubkeyWrite {
    pub safety_selection: SafetySelection,
    pub subkeys: ValueSubkeyRangeSet,
    #[serde(default)]
    pub subkeys_in_flight: ValueSubkeyRangeSet,
}

#[derive(Debug)]
enum OfflineSubkeyWriteResult {
    Finished(set_value::OutboundSetValueResult),
    Cancelled,
    Dropped,
}

#[derive(Debug)]
struct WorkItem {
    record_key: TypedRecordKey,
    safety_selection: SafetySelection,
    subkeys: ValueSubkeyRangeSet,
}

#[derive(Debug)]
struct WorkItemResult {
    work_item: WorkItem,
    written_subkeys: ValueSubkeyRangeSet,
    fanout_results: Vec<(ValueSubkeyRangeSet, FanoutResult)>,
}

impl StorageManager {
    // Write a single offline subkey
    #[instrument(level = "trace", target = "stor", skip_all, err)]
    async fn write_single_offline_subkey(
        &self,
        stop_token: StopToken,
        key: TypedRecordKey,
        subkey: ValueSubkey,
        safety_selection: SafetySelection,
    ) -> EyreResult<OfflineSubkeyWriteResult> {
        if !self.dht_is_online() {
            // Cancel this operation because we're offline
            return Ok(OfflineSubkeyWriteResult::Cancelled);
        };
        let get_result = {
            let mut inner = self.inner.lock().await;
            Self::handle_get_local_value_inner(&mut inner, key, subkey, true).await
        };
        let Ok(get_result) = get_result else {
            veilid_log!(self debug "Offline subkey write had no subkey result: {}:{}", key, subkey);
            // drop this one
            return Ok(OfflineSubkeyWriteResult::Dropped);
        };
        let Some(value) = get_result.opt_value else {
            veilid_log!(self debug "Offline subkey write had no subkey value: {}:{}", key, subkey);
            // drop this one
            return Ok(OfflineSubkeyWriteResult::Dropped);
        };
        let Some(descriptor) = get_result.opt_descriptor else {
            veilid_log!(self debug "Offline subkey write had no descriptor: {}:{}", key, subkey);
            return Ok(OfflineSubkeyWriteResult::Dropped);
        };
        veilid_log!(self debug "Offline subkey write: {}:{} len={}", key, subkey, value.value_data().data().len());
        let osvres = self
            .outbound_set_value(key, subkey, safety_selection, value.clone(), descriptor)
            .await;
        match osvres {
            Ok(res_rx) => {
                while let Ok(Ok(res)) = res_rx.recv_async().timeout_at(stop_token.clone()).await {
                    match res {
                        Ok(result) => {
                            let partial = result.fanout_result.kind.is_incomplete();
                            // Skip partial results in offline subkey write mode
                            if partial {
                                continue;
                            }

                            // Set the new value if it differs from what was asked to set
                            if result.signed_value_data.value_data() != value.value_data() {
                                // Record the newer value and send and update since it is different than what we just set
                                let mut inner = self.inner.lock().await;

                                Self::handle_set_local_value_inner(
                                    &mut inner,
                                    key,
                                    subkey,
                                    result.signed_value_data.clone(),
                                    InboundWatchUpdateMode::UpdateAll,
                                )
                                .await?;
                            }

                            return Ok(OfflineSubkeyWriteResult::Finished(result));
                        }
                        Err(e) => {
                            veilid_log!(self debug "failed to get offline subkey write result: {}:{} {}", key, subkey, e);
                            return Ok(OfflineSubkeyWriteResult::Cancelled);
                        }
                    }
                }
                veilid_log!(self debug "writing offline subkey did not complete {}:{}", key, subkey);
                return Ok(OfflineSubkeyWriteResult::Cancelled);
            }
            Err(e) => {
                veilid_log!(self debug "failed to write offline subkey: {}:{} {}", key, subkey, e);
                return Ok(OfflineSubkeyWriteResult::Cancelled);
            }
        }
    }

    // Write a set of subkeys of the same key
    #[instrument(level = "trace", target = "stor", skip_all, err)]
    async fn process_work_item(
        &self,
        stop_token: StopToken,
        work_item: WorkItem,
    ) -> EyreResult<WorkItemResult> {
        let mut written_subkeys = ValueSubkeyRangeSet::new();
        let mut fanout_results = Vec::<(ValueSubkeyRangeSet, FanoutResult)>::new();

        for subkey in work_item.subkeys.iter() {
            if poll!(stop_token.clone()).is_ready() {
                break;
            }

            let result = match self
                .write_single_offline_subkey(
                    stop_token.clone(),
                    work_item.record_key,
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
                self.check_fanout_set_offline(work_item.record_key, subkey, &result.fanout_result);
            if !was_offline {
                written_subkeys.insert(subkey);
            }
            fanout_results.push((ValueSubkeyRangeSet::single(subkey), result.fanout_result));
        }

        Ok(WorkItemResult {
            work_item,
            written_subkeys,
            fanout_results,
        })
    }

    // Process all results
    #[instrument(level = "trace", target = "stor", skip_all)]
    async fn process_single_result(&self, result: WorkItemResult) {
        let consensus_count = self
            .config()
            .with(|c| c.network.dht.set_value_count as usize);

        let mut inner = self.inner.lock().await;

        // Debug print the result
        veilid_log!(self debug "Offline write result: {:?}", result);

        // Get the offline subkey write record
        match inner
            .offline_subkey_writes
            .entry(result.work_item.record_key)
        {
            hashlink::linked_hash_map::Entry::Occupied(mut o) => {
                let finished = {
                    let osw = o.get_mut();

                    // Mark in-flight subkeys as having been completed
                    let subkeys_still_offline =
                        result.work_item.subkeys.difference(&result.written_subkeys);
                    // Now any left over are still offline, so merge them with any subkeys that have been added while we were working
                    osw.subkeys = osw.subkeys.union(&subkeys_still_offline);
                    // And clear the subkeys in flight since we're done with this key for now
                    osw.subkeys_in_flight =
                        osw.subkeys_in_flight.difference(&result.written_subkeys);

                    // If we have no new work to do, and not still doing work, then this record is done
                    osw.subkeys.is_empty() && osw.subkeys_in_flight.is_empty()
                };
                if finished {
                    veilid_log!(self debug "Offline write finished key {}", result.work_item.record_key);
                    o.remove();
                }
            }
            hashlink::linked_hash_map::Entry::Vacant(_) => {
                veilid_log!(self warn "offline write work items should always be on offline_subkey_writes entries that exist: ignoring key {}", result.work_item.record_key);
            }
        }

        // Keep the list of nodes that returned a value for later reference
        let crypto = self.crypto();
        let vcrypto = crypto.get(result.work_item.record_key.kind).unwrap();

        Self::process_fanout_results_inner(
            &mut inner,
            &vcrypto,
            result.work_item.record_key,
            result.fanout_results.into_iter().map(|x| (x.0, x.1)),
            true,
            consensus_count,
        );
    }

    // Get the next available work item
    async fn get_next_work_item(&self) -> Option<WorkItem> {
        let mut inner = self.inner.lock().await;

        // Find first offline subkey write record
        // That doesn't have the maximum number of concurrent
        // in-flight subkeys right now
        for (record_key, osw) in &mut inner.offline_subkey_writes {
            if osw.subkeys_in_flight.len() < OFFLINE_SUBKEY_WRITES_SUBKEY_CHUNK_SIZE {
                // Get first subkey to process that is not already in-flight
                for sk in osw.subkeys.iter() {
                    if !osw.subkeys_in_flight.contains(sk) {
                        // Found a not-yet-in-flight subkey, move it to in-flight
                        osw.subkeys.remove(sk);
                        osw.subkeys_in_flight.insert(sk);
                        // And return a work item for it
                        return Some(WorkItem {
                            record_key: *record_key,
                            safety_selection: osw.safety_selection,
                            subkeys: ValueSubkeyRangeSet::single(sk),
                        });
                    }
                }
            }
        }

        None
    }

    // Best-effort write subkeys to the network that were written offline
    //#[instrument(level = "trace", target = "stor", skip_all, err)]
    pub(super) async fn offline_subkey_writes_task_routine(
        &self,
        stop_token: StopToken,
        _last_ts: Timestamp,
        _cur_ts: Timestamp,
    ) -> EyreResult<()> {
        // Produce WorkItems
        let work_item_stream = unfold((), |_| {
            let registry = self.registry();
            {
                async move {
                    let storage_manager = registry.storage_manager();
                    storage_manager.get_next_work_item().await.map(|x| (x, ()))
                }
            }
        });

        // WorkItem -> Work Futures
        let work_future_stream = {
            let stop_token = stop_token.clone();
            work_item_stream.map(move |work_item| {
                let stop_token = stop_token.clone();
                async move {
                    let res = self.process_work_item(stop_token.clone(), work_item).await;
                    let result = match res {
                        Ok(v) => v,
                        Err(e) => {
                            veilid_log!(self debug "Offline subkey write failed: {}", e);
                            return;
                        }
                    };
                    self.process_single_result(result).await;
                }
            })
        };

        // Batched parallel processed Work Futures
        process_batched_future_stream_void(
            work_future_stream,
            OFFLINE_SUBKEY_WRITES_BATCH_SIZE,
            stop_token,
        )
        .await;

        // Ensure nothing is left in-flight when returning even due to an error
        {
            let mut inner = self.inner.lock().await;
            inner.offline_subkey_writes.retain(|_, v| {
                v.subkeys = v.subkeys.union(&mem::take(&mut v.subkeys_in_flight));
                !v.subkeys.is_empty()
            });
        }

        Ok(())
    }
}
