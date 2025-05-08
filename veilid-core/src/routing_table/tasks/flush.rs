use super::*;

impl RoutingTable {
    // Save routing table to disk
    #[instrument(level = "trace", skip(self), err)]
    pub async fn flush_task_routine(
        &self,
        _stop_token: StopToken,
        last_ts: Timestamp,
        cur_ts: Timestamp,
    ) -> EyreResult<()> {
        // Simple task, just writes everything to the tablestore
        self.flush().await;

        Ok(())
    }
}
