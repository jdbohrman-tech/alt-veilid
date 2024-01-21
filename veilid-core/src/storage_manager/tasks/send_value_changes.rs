use super::*;
use futures_util::StreamExt;
use stop_token::future::FutureExt;

impl StorageManager {
    // Send value change notifications across the network
    #[instrument(level = "trace", skip(self), err)]
    pub(super) async fn send_value_changes_task_routine(
        self,
        stop_token: StopToken,
        _last_ts: Timestamp,
        _cur_ts: Timestamp,
    ) -> EyreResult<()> {
        let mut value_changes: Vec<ValueChangedInfo> = vec![];

        {
            let mut inner = self.inner.lock().await;
            if let Some(local_record_store) = &mut inner.local_record_store {
                local_record_store
                    .take_value_changes(&mut value_changes)
                    .await;
            }
            if let Some(remote_record_store) = &mut inner.remote_record_store {
                remote_record_store
                    .take_value_changes(&mut value_changes)
                    .await;
            }
        }
        // Send all value changes in parallel
        let mut unord = FuturesUnordered::new();

        // Add a future for each value change
        for vc in value_changes {
            let this = self.clone();
            unord.push(async move {
                if let Err(e) = this.send_value_change(vc).await {
                    log_stor!(debug "Failed to send value change: {}", e);
                }
            });
        }

        while !unord.is_empty() {
            match unord.next().timeout_at(stop_token.clone()).await {
                Ok(Some(_)) => {
                    // Some ValueChanged completed
                }
                Ok(None) => {
                    // We're empty
                }
                Err(_) => {
                    // Timeout means we drop the rest because we were asked to stop
                    return Ok(());
                }
            }
        }

        Ok(())
    }
}
