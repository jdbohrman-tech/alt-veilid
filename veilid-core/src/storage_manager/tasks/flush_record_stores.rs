use super::*;

impl StorageManager {
    // Flush records stores to disk and remove dead records
    #[instrument(level = "trace", target = "stor", skip_all, err)]
    pub(super) async fn flush_record_stores_task_routine(
        &self,
        _stop_token: StopToken,
        _last_ts: Timestamp,
        _cur_ts: Timestamp,
    ) -> EyreResult<()> {
        let mut inner = self.inner.lock().await;
        if let Some(local_record_store) = &mut inner.local_record_store {
            local_record_store.flush().await?;
        }
        if let Some(remote_record_store) = &mut inner.remote_record_store {
            remote_record_store.flush().await?;
        }
        Ok(())
    }
}
