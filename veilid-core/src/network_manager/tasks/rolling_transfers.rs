use super::*;

impl NetworkManager {
    // Compute transfer statistics for the low level network
    #[instrument(level = "trace", skip(self), err)]
    pub async fn rolling_transfers_task_routine(
        &self,
        _stop_token: StopToken,
        last_ts: Timestamp,
        cur_ts: Timestamp,
    ) -> EyreResult<()> {
        // veilid_log!(self trace "--- network manager rolling_transfers task");
        {
            let inner = &mut *self.inner.lock();

            // Roll the low level network transfer stats for our address
            inner
                .stats
                .self_stats
                .transfer_stats_accounting
                .roll_transfers(last_ts, cur_ts, &mut inner.stats.self_stats.transfer_stats);

            // Roll all per-address transfers
            let mut dead_addrs: HashSet<PerAddressStatsKey> = HashSet::new();
            for (addr, stats) in &mut inner.stats.per_address_stats {
                stats.transfer_stats_accounting.roll_transfers(
                    last_ts,
                    cur_ts,
                    &mut stats.transfer_stats,
                );

                // While we're here, lets see if this address has timed out
                if cur_ts.saturating_sub(stats.last_seen_ts) >= IPADDR_MAX_INACTIVE_DURATION_US {
                    // it's dead, put it in the dead list
                    dead_addrs.insert(*addr);
                }
            }

            // Remove the dead addresses from our tables
            for da in &dead_addrs {
                inner.stats.per_address_stats.remove(da);
            }
        }

        // Send update
        self.send_network_update();

        Ok(())
    }
}
