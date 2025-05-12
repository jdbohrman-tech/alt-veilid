use super::{inspect_value::OutboundInspectValueResult, *};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RehydrateReport {
    /// The record key rehydrated
    record_key: TypedKey,
    /// The requested range of subkeys to rehydrate if necessary
    subkeys: ValueSubkeyRangeSet,
    /// The requested consensus count,
    consensus_count: usize,
    /// The range of subkeys that actually could be rehydrated
    rehydrated: ValueSubkeyRangeSet,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(super) struct RehydrationRequest {
    pub subkeys: ValueSubkeyRangeSet,
    pub consensus_count: usize,
}

impl StorageManager {
    /// Add a background rehydration request
    #[instrument(level = "trace", target = "stor", skip_all)]
    pub async fn add_rehydration_request(
        &self,
        record_key: TypedKey,
        subkeys: ValueSubkeyRangeSet,
        consensus_count: usize,
    ) {
        let req = RehydrationRequest {
            subkeys,
            consensus_count,
        };
        veilid_log!(self debug "Adding rehydration request: {} {:?}", record_key, req);
        let mut inner = self.inner.lock().await;
        inner
            .rehydration_requests
            .entry(record_key)
            .and_modify(|r| {
                r.subkeys = r.subkeys.union(&req.subkeys);
                r.consensus_count.max_assign(req.consensus_count);
            })
            .or_insert(req);
    }

    /// Sends the local copies of all of a record's subkeys back to the network
    /// Triggers a subkey update if the consensus on the subkey is less than
    /// the specified 'consensus_count'.
    /// The subkey updates are performed in the background if rehydration was
    /// determined to be necessary.
    /// If a newer copy of a subkey's data is available online, the background
    /// write will pick up the newest subkey data as it does the SetValue fanout
    /// and will drive the newest values to consensus.
    #[instrument(level = "trace", target = "stor", skip(self), ret, err)]
    pub(super) async fn rehydrate_record(
        &self,
        record_key: TypedKey,
        subkeys: ValueSubkeyRangeSet,
        consensus_count: usize,
    ) -> VeilidAPIResult<RehydrateReport> {
        veilid_log!(self debug "Checking for record rehydration: {} {} @ consensus {}", record_key, subkeys, consensus_count);
        // Get subkey range for consideration
        let subkeys = if subkeys.is_empty() {
            ValueSubkeyRangeSet::full()
        } else {
            subkeys
        };

        // Get safety selection
        let mut inner = self.inner.lock().await;
        let safety_selection = {
            if let Some(opened_record) = inner.opened_records.get(&record_key) {
                opened_record.safety_selection()
            } else {
                // See if it's in the local record store
                let Some(local_record_store) = inner.local_record_store.as_mut() else {
                    apibail_not_initialized!();
                };
                let Some(safety_selection) =
                    local_record_store.with_record(record_key, |rec| rec.detail().safety_selection)
                else {
                    apibail_key_not_found!(record_key);
                };
                safety_selection
            }
        };

        // See if the requested record is our local record store
        let local_inspect_result = self
            .handle_inspect_local_value_inner(&mut inner, record_key, subkeys.clone(), true)
            .await?;

        // Get rpc processor and drop mutex so we don't block while getting the value from the network
        if !self.dht_is_online() {
            apibail_try_again!("offline, try again later");
        };

        // Drop the lock for network access
        drop(inner);

        // Trim inspected subkey range to subkeys we have data for locally
        let local_inspect_result = local_inspect_result.strip_none_seqs();

        // Get the inspect record report from the network
        let result = self
            .outbound_inspect_value(
                record_key,
                local_inspect_result.subkeys().clone(),
                safety_selection,
                InspectResult::default(),
                true,
            )
            .await?;

        // If online result had no subkeys, then trigger writing the entire record in the background
        if result.inspect_result.subkeys().is_empty()
            || result.inspect_result.opt_descriptor().is_none()
        {
            return self
                .rehydrate_all_subkeys(
                    record_key,
                    subkeys,
                    consensus_count,
                    safety_selection,
                    local_inspect_result,
                )
                .await;
        }

        return self
            .rehydrate_required_subkeys(
                record_key,
                subkeys,
                consensus_count,
                safety_selection,
                local_inspect_result,
                result,
            )
            .await;
    }

    #[instrument(level = "trace", target = "stor", skip(self), ret, err)]
    pub(super) async fn rehydrate_all_subkeys(
        &self,
        record_key: TypedKey,
        subkeys: ValueSubkeyRangeSet,
        consensus_count: usize,
        safety_selection: SafetySelection,
        local_inspect_result: InspectResult,
    ) -> VeilidAPIResult<RehydrateReport> {
        let mut inner = self.inner.lock().await;

        veilid_log!(self debug "Rehydrating all subkeys: record={} subkeys={}", record_key, subkeys);

        let mut rehydrated = ValueSubkeyRangeSet::new();
        for (n, subkey) in local_inspect_result.subkeys().iter().enumerate() {
            if local_inspect_result.seqs()[n].is_some() {
                // Add to offline writes to flush
                veilid_log!(self debug "Rehydrating: record={} subkey={}", record_key, subkey);
                rehydrated.insert(subkey);
                Self::add_offline_subkey_write_inner(
                    &mut inner,
                    record_key,
                    subkey,
                    safety_selection,
                );
            }
        }

        if rehydrated.is_empty() {
            veilid_log!(self debug "Record wanted full rehydrating, but no subkey data available: record={} subkeys={}", record_key, subkeys);
        } else {
            veilid_log!(self debug "Record full rehydrating: record={} subkeys={} rehydrated={}", record_key, subkeys, rehydrated);
        }

        return Ok(RehydrateReport {
            record_key,
            subkeys,
            consensus_count,
            rehydrated,
        });
    }

    #[instrument(level = "trace", target = "stor", skip(self), ret, err)]
    pub(super) async fn rehydrate_required_subkeys(
        &self,
        record_key: TypedKey,
        subkeys: ValueSubkeyRangeSet,
        consensus_count: usize,
        safety_selection: SafetySelection,
        local_inspect_result: InspectResult,
        outbound_inspect_result: OutboundInspectValueResult,
    ) -> VeilidAPIResult<RehydrateReport> {
        let mut inner = self.inner.lock().await;

        // Get cryptosystem
        let crypto = self.crypto();
        let Some(vcrypto) = crypto.get(record_key.kind) else {
            apibail_generic!("unsupported cryptosystem");
        };

        // For each subkey, determine if we should rehydrate it
        let mut rehydrated = ValueSubkeyRangeSet::new();
        for (n, subkey) in local_inspect_result.subkeys().iter().enumerate() {
            if local_inspect_result.seqs()[n].is_none() {
                apibail_internal!(format!("None sequence number found in local inspect results. Should have been stripped by strip_none_seqs(): {:?}", local_inspect_result));
            }

            let sfr = outbound_inspect_result
                .subkey_fanout_results
                .get(n)
                .unwrap();
            // Does the online subkey have enough consensus?
            // If not, schedule it to be written in the background
            if sfr.consensus_nodes.len() < consensus_count {
                // Add to offline writes to flush
                veilid_log!(self debug "Rehydrating: record={} subkey={}", record_key, subkey);
                rehydrated.insert(subkey);
                Self::add_offline_subkey_write_inner(
                    &mut inner,
                    record_key,
                    subkey,
                    safety_selection,
                );
            }
        }

        if rehydrated.is_empty() {
            veilid_log!(self debug "Record did not need rehydrating: record={} local_subkeys={}", record_key, local_inspect_result.subkeys());
        } else {
            veilid_log!(self debug "Record rehydrating: record={} local_subkeys={} rehydrated={}", record_key, local_inspect_result.subkeys(), rehydrated);
        }

        // Keep the list of nodes that returned a value for later reference
        let results_iter = outbound_inspect_result
            .inspect_result
            .subkeys()
            .iter()
            .map(ValueSubkeyRangeSet::single)
            .zip(outbound_inspect_result.subkey_fanout_results.into_iter());

        Self::process_fanout_results_inner(
            &mut inner,
            &vcrypto,
            record_key,
            results_iter,
            false,
            self.config()
                .with(|c| c.network.dht.set_value_count as usize),
        );

        Ok(RehydrateReport {
            record_key,
            subkeys,
            consensus_count,
            rehydrated,
        })
    }
}
