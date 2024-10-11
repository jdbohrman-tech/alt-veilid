use super::*;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub(crate) struct RouteStats {
    /// Consecutive failed to send count
    #[serde(skip)]
    pub failed_to_send: u32,
    /// Consecutive questions that didn't get an answer
    #[serde(skip)]
    pub recent_lost_answers: u32,
    /// Timestamp of when the route was created
    pub created_ts: Timestamp,
    /// Timestamp of when the route was last checked for validity or received traffic
    #[serde(skip)]
    pub last_known_valid_ts: Option<Timestamp>,
    /// Timestamp of when the route was last sent to
    #[serde(skip)]
    pub last_sent_ts: Option<Timestamp>,
    /// Timestamp of when the route last received a question or statement
    #[serde(skip)]
    pub last_rcvd_question_ts: Option<Timestamp>,
    /// Timestamp of when the route last received an answer
    #[serde(skip)]
    pub last_rcvd_answer_ts: Option<Timestamp>,
    /// Transfers up and down
    pub transfer: TransferStatsDownUp,
    /// Latency stats
    pub latency: LatencyStats,
    /// Answer stats
    pub answer: AnswerStats,
    /// Accounting mechanism for this route's RPC latency
    #[serde(skip)]
    latency_stats_accounting: LatencyStatsAccounting,
    /// Accounting mechanism for the bandwidth across this route
    #[serde(skip)]
    transfer_stats_accounting: TransferStatsAccounting,
    /// Accounting mechanism for this route's RPC answers
    #[serde(skip)]
    answer_stats_accounting: AnswerStatsAccounting,
}

impl fmt::Display for RouteStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "created: {}", self.created_ts)?;
        writeln!(
            f,
            "# recently-lost/failed-to-send: {} / {}",
            self.recent_lost_answers, self.failed_to_send
        )?;
        writeln!(
            f,
            "last_known_valid:   {}",
            if let Some(ts) = &self.last_known_valid_ts {
                ts.to_string()
            } else {
                "None".to_owned()
            }
        )?;
        writeln!(
            f,
            "last_sent:          {}",
            if let Some(ts) = &self.last_sent_ts {
                ts.to_string()
            } else {
                "None".to_owned()
            }
        )?;
        writeln!(
            f,
            "last_rcvd_question: {}",
            if let Some(ts) = &self.last_rcvd_question_ts {
                ts.to_string()
            } else {
                "None".to_owned()
            }
        )?;
        write!(f, "transfer:\n{}", indent_all_string(&self.transfer))?;
        write!(f, "latency: {}", self.latency)?;
        write!(f, "answer:\n{}", indent_all_string(&self.answer))?;

        Ok(())
    }
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

    /// Mark a route as having lost an answer
    pub fn record_lost_answer(&mut self) {
        let cur_ts = Timestamp::now();
        self.recent_lost_answers += 1;
        self.answer_stats_accounting.record_lost_answer(cur_ts);
    }

    /// Mark a route as having received a question or statement
    pub fn record_question_received(&mut self, cur_ts: Timestamp, bytes: ByteCount) {
        self.last_rcvd_question_ts = Some(cur_ts);
        self.last_known_valid_ts = Some(cur_ts);
        self.transfer_stats_accounting.add_down(bytes);
        self.answer_stats_accounting.record_question(cur_ts);
    }

    /// Mark a route as having received an answer
    pub fn record_answer_received(&mut self, cur_ts: Timestamp, bytes: ByteCount) {
        self.last_rcvd_answer_ts = Some(cur_ts);
        self.last_known_valid_ts = Some(cur_ts);
        self.recent_lost_answers = 0;
        self.transfer_stats_accounting.add_down(bytes);
        self.answer_stats_accounting.record_answer(cur_ts);
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
        self.latency = self.latency_stats_accounting.record_latency(latency);
    }

    /// Roll transfers for these route stats
    pub fn roll_transfers(&mut self, last_ts: Timestamp, cur_ts: Timestamp) {
        self.transfer_stats_accounting
            .roll_transfers(last_ts, cur_ts, &mut self.transfer);
    }
    pub fn roll_answers(&mut self, cur_ts: Timestamp) {
        self.answer = self.answer_stats_accounting.roll_answers(cur_ts);
    }

    /// Get the latency stats
    pub fn latency_stats(&self) -> &LatencyStats {
        &self.latency
    }

    /// Get the transfer stats
    #[expect(dead_code)]
    pub fn transfer_stats(&self) -> &TransferStatsDownUp {
        &self.transfer
    }

    /// Reset stats when network restarts
    pub fn reset(&mut self) {
        self.last_known_valid_ts = None;
        self.last_sent_ts = None;
        self.last_rcvd_question_ts = None;
        self.last_rcvd_answer_ts = None;
        self.failed_to_send = 0;
        self.recent_lost_answers = 0;
    }

    /// Check if a route needs testing
    pub fn needs_testing(&self, cur_ts: Timestamp) -> bool {
        // Has the route had any failures lately?
        if self.recent_lost_answers > 0 || self.failed_to_send > 0 {
            // If so, always test
            return true;
        }

        // Has the route been tested within the idle time we'd want to check things?
        // (also if we've received successfully over the route, this will get set)
        if let Some(last_tested_ts) = self.last_known_valid_ts {
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
