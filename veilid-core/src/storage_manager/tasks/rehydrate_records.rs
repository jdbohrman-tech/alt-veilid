use super::*;

impl_veilid_log_facility!("stor");

impl StorageManager {
    /// Process background rehydration requests
    #[instrument(level = "trace", target = "stor", skip_all, err)]
    pub(super) async fn rehydrate_records_task_routine(
        &self,
        stop_token: StopToken,
        _last_ts: Timestamp,
        _cur_ts: Timestamp,
    ) -> EyreResult<()> {
        let reqs = {
            let mut inner = self.inner.lock().await;
            core::mem::take(&mut inner.rehydration_requests)
        };

        let mut futs = Vec::new();
        for req in reqs {
            futs.push(async move {
                let res = self
                    .rehydrate_record(req.0, req.1.subkeys.clone(), req.1.consensus_count)
                    .await;

                let _report = match res {
                    Ok(v) => v,
                    Err(e) => {
                        veilid_log!(self debug "Rehydration request failed: {}", e);
                        if matches!(e, VeilidAPIError::TryAgain { message: _ }) {
                            // Try again later
                            self.add_rehydration_request(
                                req.0,
                                req.1.subkeys,
                                req.1.consensus_count,
                            )
                            .await;
                        }
                        return;
                    }
                };
            });
        }

        process_batched_future_queue_void(futs, REHYDRATE_BATCH_SIZE, stop_token).await;

        Ok(())
    }
}
