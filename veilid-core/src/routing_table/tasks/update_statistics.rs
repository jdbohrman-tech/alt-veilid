use super::*;

impl RoutingTable {
    // Compute transfer statistics to determine how 'fast' a node is
    #[instrument(level = "trace", skip(self), err)]
    pub(crate) async fn rolling_transfers_task_routine(
        self,
        _stop_token: StopToken,
        last_ts: Timestamp,
        cur_ts: Timestamp,
    ) -> EyreResult<()> {
        {
            let inner = &mut *self.inner.write();

            // Roll our own node's transfers
            inner.self_transfer_stats_accounting.roll_transfers(
                last_ts,
                cur_ts,
                &mut inner.self_transfer_stats,
            );

            // Roll all bucket entry transfers
            let all_entries: Vec<Arc<BucketEntry>> = inner.all_entries.iter().collect();
            for entry in all_entries {
                entry.with_mut(inner, |_rti, e| e.roll_transfers(last_ts, cur_ts));
            }
        }

        // Roll all route transfers
        let rss = self.route_spec_store();
        rss.roll_transfers(last_ts, cur_ts);

        Ok(())
    }

    // Update state statistics in PeerStats
    #[instrument(level = "trace", skip(self), err)]
    pub(crate) async fn update_state_stats_task_routine(
        self,
        _stop_token: StopToken,
        _last_ts: Timestamp,
        _cur_ts: Timestamp,
    ) -> EyreResult<()> {
        {
            let inner = &mut *self.inner.write();

            // Roll all bucket entry transfers
            let all_entries: Vec<Arc<BucketEntry>> = inner.all_entries.iter().collect();
            for entry in all_entries {
                entry.with_mut(inner, |_rti, e| e.update_state_stats());
            }
        }

        Ok(())
    }

    // Update rolling answers in PeerStats
    #[instrument(level = "trace", skip(self), err)]
    pub(crate) async fn rolling_answers_task_routine(
        self,
        _stop_token: StopToken,
        _last_ts: Timestamp,
        cur_ts: Timestamp,
    ) -> EyreResult<()> {
        {
            let inner = &mut *self.inner.write();

            // Roll all bucket entry answers stats
            let all_entries: Vec<Arc<BucketEntry>> = inner.all_entries.iter().collect();
            for entry in all_entries {
                entry.with_mut(inner, |_rti, e| e.roll_answer_stats(cur_ts));
            }
        }

        // Roll all route answers
        let rss = self.route_spec_store();
        rss.roll_answers(cur_ts);

        Ok(())
    }
}
