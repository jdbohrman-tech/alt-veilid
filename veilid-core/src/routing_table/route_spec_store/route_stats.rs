use super::*;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RouteStats {
    /// Consecutive failed to send count
    #[serde(skip)]
    pub failed_to_send: u32,
    /// Questions lost
    #[serde(skip)]
    pub questions_lost: u32,
    /// Timestamp of when the route was created
    pub created_ts: Timestamp,
    /// Timestamp of when the route was last checked for validity
    #[serde(skip)]
    pub last_tested_ts: Option<Timestamp>,
    /// Timestamp of when the route was last sent to
    #[serde(skip)]
    pub last_sent_ts: Option<Timestamp>,
    /// Timestamp of when the route was last received over
    #[serde(skip)]
    pub last_received_ts: Option<Timestamp>,
    /// Transfers up and down
    pub transfer_stats_down_up: TransferStatsDownUp,
    /// Latency stats
    pub latency_stats: LatencyStats,
    /// Accounting mechanism for this route's RPC latency
    #[serde(skip)]
    latency_stats_accounting: LatencyStatsAccounting,
    /// Accounting mechanism for the bandwidth across this route
    #[serde(skip)]
    transfer_stats_accounting: TransferStatsAccounting,
}

impl RouteStats {
    /// Make new route stats
    pub fn new(created_ts: Timestamp) -> Self {
        Self {
            created_ts,
            ..Default::default()
        }
    }
    /// Mark a route as having failed to send
    pub fn record_send_failed(&mut self) {
        self.failed_to_send += 1;
    }

    /// Mark a route as having lost a question
    pub fn record_question_lost(&mut self) {
        self.questions_lost += 1;
    }

    /// Mark a route as having received something
    pub fn record_received(&mut self, cur_ts: Timestamp, bytes: ByteCount) {
        self.last_received_ts = Some(cur_ts);
        self.last_tested_ts = Some(cur_ts);
        self.transfer_stats_accounting.add_down(bytes);
    }

    /// Mark a route as having been sent to
    pub fn record_sent(&mut self, cur_ts: Timestamp, bytes: ByteCount) {
        self.last_sent_ts = Some(cur_ts);
        self.transfer_stats_accounting.add_up(bytes);

        // If we sent successfully, then reset 'failed_to_send'
        self.failed_to_send = 0;
    }

    /// Mark a route as having been sent to
    pub fn record_latency(&mut self, latency: TimestampDuration) {
        self.latency_stats = self.latency_stats_accounting.record_latency(latency);
    }

    /// Mark a route as having been tested
    pub fn record_tested(&mut self, cur_ts: Timestamp) {
        self.last_tested_ts = Some(cur_ts);

        // Reset question_lost and failed_to_send if we test clean
        self.failed_to_send = 0;
        self.questions_lost = 0;
    }

    /// Roll transfers for these route stats
    pub fn roll_transfers(&mut self, last_ts: Timestamp, cur_ts: Timestamp) {
        self.transfer_stats_accounting.roll_transfers(
            last_ts,
            cur_ts,
            &mut self.transfer_stats_down_up,
        )
    }

    /// Get the latency stats
    pub fn latency_stats(&self) -> &LatencyStats {
        &self.latency_stats
    }

    /// Get the transfer stats
    pub fn transfer_stats(&self) -> &TransferStatsDownUp {
        &self.transfer_stats_down_up
    }

    /// Reset stats when network restarts
    pub fn reset(&mut self) {
        self.last_tested_ts = None;
        self.last_sent_ts = None;
        self.last_received_ts = None;
        self.failed_to_send = 0;
        self.questions_lost = 0;
    }

    /// Check if a route needs testing
    pub fn needs_testing(&self, cur_ts: Timestamp) -> bool {
        // Has the route had any failures lately?
        if self.questions_lost > 0 || self.failed_to_send > 0 {
            // If so, always test
            return true;
        }

        // Has the route been tested within the idle time we'd want to check things?
        // (also if we've received successfully over the route, this will get set)
        if let Some(last_tested_ts) = self.last_tested_ts {
            if cur_ts.saturating_sub(last_tested_ts)
                > TimestampDuration::new(ROUTE_MIN_IDLE_TIME_MS as u64 * 1000u64)
            {
                return true;
            }
        } else {
            // If this route has never been tested, it needs to be
            return true;
        }

        false
    }
}
