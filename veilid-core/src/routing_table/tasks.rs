use super::*;
use crate::xx::*;

impl RoutingTable {
    // Compute transfer statistics to determine how 'fast' a node is
    #[instrument(level = "trace", skip(self), err)]
    pub(super) async fn rolling_transfers_task_routine(
        self,
        _stop_token: StopToken,
        last_ts: u64,
        cur_ts: u64,
    ) -> EyreResult<()> {
        // log_rtab!("--- rolling_transfers task");
        let mut inner = self.inner.write();
        let inner = &mut *inner;

        // Roll our own node's transfers
        inner.self_transfer_stats_accounting.roll_transfers(
            last_ts,
            cur_ts,
            &mut inner.self_transfer_stats,
        );

        // Roll all bucket entry transfers
        let entries: Vec<Arc<BucketEntry>> = inner
            .buckets
            .iter()
            .flat_map(|b| b.entries().map(|(_k, v)| v.clone()))
            .collect();
        for v in entries {
            v.with_mut(inner, |_rti, e| e.roll_transfers(last_ts, cur_ts));
        }
        Ok(())
    }

    // Kick the queued buckets in the routing table to free dead nodes if necessary
    // Attempts to keep the size of the routing table down to the bucket depth
    #[instrument(level = "trace", skip(self), err)]
    pub(super) async fn kick_buckets_task_routine(
        self,
        _stop_token: StopToken,
        _last_ts: u64,
        cur_ts: u64,
    ) -> EyreResult<()> {
        let kick_queue: Vec<usize> = core::mem::take(&mut *self.unlocked_inner.kick_queue.lock())
            .into_iter()
            .collect();
        let mut inner = self.inner.write();
        for idx in kick_queue {
            Self::kick_bucket(&mut *inner, idx)
        }
        Ok(())
    }
}
