use super::*;

// Latency entry is per round-trip packet (ping or data)
// - Size is number of entries
const ROLLING_LATENCIES_SIZE: usize = 50;

// Transfers entries are in bytes total for the interval
// - Size is number of entries
// - Interval is number of seconds in each entry
const ROLLING_TRANSFERS_SIZE: usize = 10;
pub const ROLLING_TRANSFERS_INTERVAL_SECS: u32 = 1;

// State entry is per state reason change
// - Size is number of entries
const ROLLING_STATE_REASON_SPAN_SIZE: usize = 32;
pub const UPDATE_STATE_STATS_INTERVAL_SECS: u32 = 1;

// Answer entries are in counts per interval
// - Size is number of entries
// - Interval is number of seconds in each entry
const ROLLING_ANSWERS_SIZE: usize = 10;
pub const ROLLING_ANSWER_INTERVAL_SECS: u32 = 60;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TransferCount {
    down: ByteCount,
    up: ByteCount,
}

#[derive(Debug, Clone, Default)]
pub struct TransferStatsAccounting {
    rolling_transfers: VecDeque<TransferCount>,
    current_transfer: TransferCount,
}

impl TransferStatsAccounting {
    pub fn new() -> Self {
        Self {
            rolling_transfers: VecDeque::new(),
            current_transfer: TransferCount::default(),
        }
    }

    pub fn add_down(&mut self, bytes: ByteCount) {
        self.current_transfer.down += bytes;
    }

    pub fn add_up(&mut self, bytes: ByteCount) {
        self.current_transfer.up += bytes;
    }

    pub fn roll_transfers(
        &mut self,
        last_ts: Timestamp,
        cur_ts: Timestamp,
        transfer_stats: &mut TransferStatsDownUp,
    ) {
        let dur_ms = cur_ts.saturating_sub(last_ts) / 1000u64;
        while self.rolling_transfers.len() >= ROLLING_TRANSFERS_SIZE {
            self.rolling_transfers.pop_front();
        }
        self.rolling_transfers.push_back(self.current_transfer);

        transfer_stats.down.total += self.current_transfer.down;
        transfer_stats.up.total += self.current_transfer.up;

        self.current_transfer = TransferCount::default();

        transfer_stats.down.maximum = 0.into();
        transfer_stats.up.maximum = 0.into();
        transfer_stats.down.minimum = u64::MAX.into();
        transfer_stats.up.minimum = u64::MAX.into();
        transfer_stats.down.average = 0.into();
        transfer_stats.up.average = 0.into();
        for xfer in &self.rolling_transfers {
            let bpsd = xfer.down * 1000u64 / dur_ms;
            let bpsu = xfer.up * 1000u64 / dur_ms;
            transfer_stats.down.maximum.max_assign(bpsd);
            transfer_stats.up.maximum.max_assign(bpsu);
            transfer_stats.down.minimum.min_assign(bpsd);
            transfer_stats.up.minimum.min_assign(bpsu);
            transfer_stats.down.average += bpsd;
            transfer_stats.up.average += bpsu;
        }
        let len = self.rolling_transfers.len() as u64;
        if len > 0 {
            transfer_stats.down.average /= len;
            transfer_stats.up.average /= len;
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct LatencyStatsAccounting {
    rolling_latencies: VecDeque<TimestampDuration>,
}

impl LatencyStatsAccounting {
    pub fn new() -> Self {
        Self {
            rolling_latencies: VecDeque::new(),
        }
    }

    fn get_tm_n(sorted_latencies: &[TimestampDuration], n: usize) -> Option<TimestampDuration> {
        let tmcount = sorted_latencies.len() * n / 100;
        if tmcount == 0 {
            None
        } else {
            let mut tm = TimestampDuration::new(0);
            for l in &sorted_latencies[..tmcount] {
                tm += *l;
            }
            tm /= tmcount as u64;
            Some(tm)
        }
    }

    fn get_p_n(sorted_latencies: &[TimestampDuration], n: usize) -> TimestampDuration {
        let pindex = (sorted_latencies.len() * n / 100).saturating_sub(1);
        sorted_latencies[pindex]
    }

    pub fn record_latency(&mut self, latency: TimestampDuration) -> LatencyStats {
        while self.rolling_latencies.len() >= ROLLING_LATENCIES_SIZE {
            self.rolling_latencies.pop_front();
        }
        self.rolling_latencies.push_back(latency);

        // Calculate latency stats

        let mut fastest = TimestampDuration::new(u64::MAX);
        let mut slowest = TimestampDuration::new(0u64);
        let mut average = TimestampDuration::new(0u64);

        for rl in &self.rolling_latencies {
            fastest.min_assign(*rl);
            slowest.max_assign(*rl);
            average += *rl;
        }
        let len = self.rolling_latencies.len() as u64;
        if len > 0 {
            average /= len;
        }

        let mut sorted_latencies: Vec<_> = self.rolling_latencies.iter().copied().collect();
        sorted_latencies.sort();

        let tm90 = Self::get_tm_n(&sorted_latencies, 90).unwrap_or(average);
        let tm75 = Self::get_tm_n(&sorted_latencies, 75).unwrap_or(average);
        let p90 = Self::get_p_n(&sorted_latencies, 90);
        let p75 = Self::get_p_n(&sorted_latencies, 75);

        LatencyStats {
            fastest,
            average,
            slowest,
            tm90,
            tm75,
            p90,
            p75,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StateReasonSpan {
    state_reason: BucketEntryStateReason,
    enter_ts: Timestamp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StateSpan {
    state: BucketEntryState,
    enter_ts: Timestamp,
}

#[derive(Debug, Clone, Default)]
pub struct StateStatsAccounting {
    rolling_state_reason_spans: VecDeque<StateReasonSpan>,
    last_stats: Option<StateStats>,
}

impl StateStatsAccounting {
    pub fn new() -> Self {
        Self {
            rolling_state_reason_spans: VecDeque::new(),
            last_stats: None,
        }
    }

    fn make_stats(&self, cur_ts: Timestamp) -> StateStats {
        let mut ss = StateStats::default();
        let srs = &mut ss.reason;

        let mut last_ts = cur_ts;
        for rss in self.rolling_state_reason_spans.iter().rev() {
            let span_dur = last_ts.saturating_sub(rss.enter_ts);

            match BucketEntryState::from(rss.state_reason) {
                BucketEntryState::Punished => ss.punished += span_dur,
                BucketEntryState::Dead => ss.dead += span_dur,
                BucketEntryState::Unreliable => ss.unreliable += span_dur,
                BucketEntryState::Reliable => ss.reliable += span_dur,
            }
            match rss.state_reason {
                BucketEntryStateReason::Punished(_) => {
                    // Ignore punished nodes for now
                }
                BucketEntryStateReason::Dead(bucket_entry_dead_reason) => {
                    match bucket_entry_dead_reason {
                        BucketEntryDeadReason::CanNotSend => srs.can_not_send += span_dur,
                        BucketEntryDeadReason::TooManyLostAnswers => {
                            srs.too_many_lost_answers += span_dur
                        }
                        BucketEntryDeadReason::NoPingResponse => srs.no_ping_response += span_dur,
                    }
                }
                BucketEntryStateReason::Unreliable(bucket_entry_unreliable_reason) => {
                    match bucket_entry_unreliable_reason {
                        BucketEntryUnreliableReason::FailedToSend => srs.failed_to_send += span_dur,
                        BucketEntryUnreliableReason::LostAnswers => srs.lost_answers += span_dur,
                        BucketEntryUnreliableReason::NotSeenConsecutively => {
                            srs.not_seen_consecutively += span_dur
                        }
                        BucketEntryUnreliableReason::InUnreliablePingSpan => {
                            srs.in_unreliable_ping_span += span_dur
                        }
                    }
                }
                BucketEntryStateReason::Reliable => {
                    // Reliable nodes don't have a reason other than lack of unreliability
                }
            }

            last_ts = rss.enter_ts;
        }
        ss.span = cur_ts.saturating_sub(last_ts);
        ss
    }

    pub fn take_stats(&mut self) -> Option<StateStats> {
        self.last_stats.take()
    }

    pub fn record_state_reason(&mut self, cur_ts: Timestamp, state_reason: BucketEntryStateReason) {
        let new_span = if let Some(cur_span) = self.rolling_state_reason_spans.back() {
            if state_reason != cur_span.state_reason {
                while self.rolling_state_reason_spans.len() >= ROLLING_STATE_REASON_SPAN_SIZE {
                    self.rolling_state_reason_spans.pop_front();
                }
                true
            } else {
                false
            }
        } else {
            true
        };
        if new_span {
            self.last_stats = Some(self.make_stats(cur_ts));
            self.rolling_state_reason_spans.push_back(StateReasonSpan {
                state_reason,
                enter_ts: cur_ts,
            });
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnswerSpan {
    enter_ts: Timestamp,
    questions: u32,
    answers: u32,
    lost_answers: u32,
    current_consecutive_answers: u32,
    current_consecutive_lost_answers: u32,
    consecutive_answers_maximum: u32,
    consecutive_answers_total: u32,
    consecutive_answers_count: u32,
    consecutive_answers_minimum: u32,
    consecutive_lost_answers_maximum: u32,
    consecutive_lost_answers_total: u32,
    consecutive_lost_answers_count: u32,
    consecutive_lost_answers_minimum: u32,
}

impl AnswerSpan {
    pub fn new(cur_ts: Timestamp) -> Self {
        AnswerSpan {
            enter_ts: cur_ts,
            questions: 0,
            answers: 0,
            lost_answers: 0,
            current_consecutive_answers: 0,
            current_consecutive_lost_answers: 0,
            consecutive_answers_maximum: 0,
            consecutive_answers_total: 0,
            consecutive_answers_count: 0,
            consecutive_answers_minimum: 0,
            consecutive_lost_answers_maximum: 0,
            consecutive_lost_answers_total: 0,
            consecutive_lost_answers_count: 0,
            consecutive_lost_answers_minimum: 0,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct AnswerStatsAccounting {
    rolling_answer_spans: VecDeque<AnswerSpan>,
}

impl AnswerStatsAccounting {
    pub fn new() -> Self {
        Self {
            rolling_answer_spans: VecDeque::new(),
        }
    }

    fn current_span(&mut self, cur_ts: Timestamp) -> &mut AnswerSpan {
        if self.rolling_answer_spans.is_empty() {
            self.rolling_answer_spans.push_back(AnswerSpan::new(cur_ts));
        }
        self.rolling_answer_spans.front_mut().unwrap()
    }

    fn make_stats(&self, cur_ts: Timestamp) -> AnswerStats {
        let mut questions = 0u32;
        let mut answers = 0u32;
        let mut lost_answers = 0u32;
        let mut consecutive_answers_maximum = 0u32;
        let mut consecutive_answers_average = 0u32;
        let mut consecutive_answers_minimum = u32::MAX;
        let mut consecutive_lost_answers_maximum = 0u32;
        let mut consecutive_lost_answers_average = 0u32;
        let mut consecutive_lost_answers_minimum = u32::MAX;

        let mut last_ts = cur_ts;
        for ras in self.rolling_answer_spans.iter().rev() {
            questions += ras.questions;
            answers += ras.answers;
            lost_answers += ras.lost_answers;

            consecutive_answers_maximum.max_assign(ras.consecutive_answers_maximum);
            consecutive_answers_minimum.min_assign(ras.consecutive_answers_minimum);
            consecutive_answers_average += if ras.consecutive_answers_total > 0 {
                ras.consecutive_answers_count / ras.consecutive_answers_total
            } else {
                0
            };

            consecutive_lost_answers_maximum.max_assign(ras.consecutive_lost_answers_maximum);
            consecutive_lost_answers_minimum.min_assign(ras.consecutive_lost_answers_minimum);
            consecutive_lost_answers_average += if ras.consecutive_lost_answers_total > 0 {
                ras.consecutive_lost_answers_count / ras.consecutive_lost_answers_total
            } else {
                0
            };

            last_ts = ras.enter_ts;
        }

        let len = self.rolling_answer_spans.len() as u32;
        if len > 0 {
            consecutive_answers_average /= len;
            consecutive_lost_answers_average /= len;
        }

        let span = cur_ts.saturating_sub(last_ts);

        AnswerStats {
            span,
            questions,
            answers,
            lost_answers,
            consecutive_answers_maximum,
            consecutive_answers_average,
            consecutive_answers_minimum,
            consecutive_lost_answers_maximum,
            consecutive_lost_answers_average,
            consecutive_lost_answers_minimum,
        }
    }

    pub fn roll_answers(&mut self, cur_ts: Timestamp) -> AnswerStats {
        let stats = self.make_stats(cur_ts);

        while self.rolling_answer_spans.len() >= ROLLING_ANSWERS_SIZE {
            self.rolling_answer_spans.pop_front();
        }
        self.rolling_answer_spans.push_back(AnswerSpan::new(cur_ts));

        stats
    }

    pub fn record_question(&mut self, cur_ts: Timestamp) {
        let cas = self.current_span(cur_ts);
        cas.questions += 1;
    }
    pub fn record_answer(&mut self, cur_ts: Timestamp) {
        let cas = self.current_span(cur_ts);
        cas.answers += 1;
        if cas.current_consecutive_lost_answers > 0 {
            cas.consecutive_lost_answers_maximum
                .max_assign(cas.current_consecutive_lost_answers);
            cas.consecutive_lost_answers_minimum
                .min_assign(cas.current_consecutive_lost_answers);
            cas.consecutive_lost_answers_total += cas.current_consecutive_lost_answers;
            cas.consecutive_lost_answers_count += 1;
            cas.current_consecutive_lost_answers = 0;
        }
        cas.current_consecutive_answers = 1;
    }
    pub fn record_lost_answer(&mut self, cur_ts: Timestamp) {
        let cas = self.current_span(cur_ts);
        cas.lost_answers += 1;
        if cas.current_consecutive_answers > 0 {
            cas.consecutive_answers_maximum
                .max_assign(cas.current_consecutive_answers);
            cas.consecutive_answers_minimum
                .min_assign(cas.current_consecutive_answers);
            cas.consecutive_answers_total += cas.current_consecutive_answers;
            cas.consecutive_answers_count += 1;
            cas.current_consecutive_answers = 0;
        }
        cas.current_consecutive_lost_answers = 1;
    }
}
