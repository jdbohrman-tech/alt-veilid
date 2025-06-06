use super::*;

// State entry is per state reason change
// - Size is number of entries
const ROLLING_STATE_REASON_SPAN_SIZE: usize = 32;
pub const UPDATE_STATE_STATS_INTERVAL_SECS: u32 = 1;

// Answer entries are in counts per interval
// - Size is number of entries
// - Interval is number of seconds in each entry
const ROLLING_ANSWERS_SIZE: usize = 10;
pub const ROLLING_ANSWER_INTERVAL_SECS: u32 = 60;
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
