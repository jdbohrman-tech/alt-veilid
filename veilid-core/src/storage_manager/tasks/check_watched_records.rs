use super::*;

impl StorageManager {
    // Check if server-side watches have expired
    #[instrument(level = "trace", skip(self), err)]
    pub(super) async fn check_watched_records_task_routine(
        self,
        stop_token: StopToken,
        _last_ts: Timestamp,
        _cur_ts: Timestamp,
    ) -> EyreResult<()> {
        let mut inner = self.inner.lock().await;

        if let Some(local_record_store) = &mut inner.local_record_store {
            local_record_store.check_watched_records();
        }
        if let Some(remote_record_store) = &mut inner.remote_record_store {
            remote_record_store.check_watched_records();
        }

        Ok(())
    }
}
