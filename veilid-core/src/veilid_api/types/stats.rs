use super::*;

/// Measurement of communications latency to this node over all RPC questions
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), derive(Tsify))]
pub struct LatencyStats {
    /// fastest latency in the ROLLING_LATENCIES_SIZE last latencies
    pub fastest: TimestampDuration,
    /// average latency over the ROLLING_LATENCIES_SIZE last latencies
    pub average: TimestampDuration,
    /// slowest latency in the ROLLING_LATENCIES_SIZE last latencies
    pub slowest: TimestampDuration,
}

impl fmt::Display for LatencyStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} slow / {} avg / {} fast",
            self.slowest, self.average, self.fastest
        )?;
        Ok(())
    }
}

/// Measurement of how much data has transferred to or from this node over a time span
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), derive(Tsify))]
pub struct TransferStats {
    /// total amount transferred ever
    pub total: ByteCount,
    /// maximum rate over the ROLLING_TRANSFERS_SIZE last amounts
    pub maximum: ByteCount,
    /// average rate over the ROLLING_TRANSFERS_SIZE last amounts
    pub average: ByteCount,
    /// minimum rate over the ROLLING_TRANSFERS_SIZE last amounts
    pub minimum: ByteCount,
}

impl fmt::Display for TransferStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} min / {} avg / {} max / {} total",
            self.minimum, self.average, self.maximum, self.total
        )?;
        Ok(())
    }
}

/// Transfer statistics from a node to our own (down) and
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), derive(Tsify))]
pub struct TransferStatsDownUp {
    pub down: TransferStats,
    pub up: TransferStats,
}

impl fmt::Display for TransferStatsDownUp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "down: {}", self.down)?;
        writeln!(f, "up:   {}", self.up)?;
        Ok(())
    }
}

/// Measurement of what states the node has been in over a time span
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), derive(Tsify))]
pub struct StateStats {
    /// total amount of time measured
    pub span: TimestampDuration,
    /// amount of time spent in a reliable state
    pub reliable: TimestampDuration,
    /// amount of time spent in an unreliable state
    pub unreliable: TimestampDuration,
    /// amount of time spent in a dead state
    pub dead: TimestampDuration,
    /// amount of time spent in a punished state
    pub punished: TimestampDuration,
    /// state reason stats for this peer
    #[serde(default)]
    pub reason: StateReasonStats,
}

impl fmt::Display for StateStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "span:       {}", self.span)?;
        writeln!(f, "reliable:   {}", self.reliable)?;
        writeln!(f, "unreliable: {}", self.unreliable)?;
        writeln!(f, "dead:       {}", self.dead)?;
        writeln!(f, "punished:   {}", self.punished)?;
        write!(f, "reason:\n{}", indent_all_string(&self.reason))?;
        Ok(())
    }
}

/// Measurement of what state reasons the node has been in over a time span
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), derive(Tsify))]
pub struct StateReasonStats {
    /// time spent dead due to being unable to send
    pub can_not_send: TimestampDuration,
    /// time spent dead because of too many lost answers
    pub too_many_lost_answers: TimestampDuration,
    /// time spent dead because of no ping response
    pub no_ping_response: TimestampDuration,
    /// time spent unreliable because of failures to send
    pub failed_to_send: TimestampDuration,
    /// time spent unreliable because of lost answers
    pub lost_answers: TimestampDuration,
    /// time spent unreliable because of not being seen consecutively
    pub not_seen_consecutively: TimestampDuration,
    /// time spent unreliable because we are in the unreliable ping span
    pub in_unreliable_ping_span: TimestampDuration,
}

impl fmt::Display for StateReasonStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "(dead) can_not_send:     {}", self.can_not_send)?;
        writeln!(f, "(dead) lost_answers:     {}", self.too_many_lost_answers)?;
        writeln!(f, "(dead) no_ping_response: {}", self.no_ping_response)?;
        writeln!(f, "(urel) failed_to_send:   {}", self.failed_to_send)?;
        writeln!(f, "(urel) lost_answers:     {}", self.lost_answers)?;
        writeln!(
            f,
            "(urel) not_consecutive:  {}",
            self.not_seen_consecutively
        )?;
        writeln!(
            f,
            "(urel) unreliable_ping:  {}",
            self.in_unreliable_ping_span
        )?;
        writeln!(f, "(urel) can_not_send:     {}", self.can_not_send)?;
        Ok(())
    }
}

/// Measurement of round-trip RPC question/answer performance
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), derive(Tsify))]
pub struct AnswerStats {
    /// total amount of time measured
    pub span: TimestampDuration,
    /// number of questions sent in this span
    pub questions: u32,
    /// number of answers received in this span
    pub answers: u32,
    /// number of lost answers in this span
    pub lost_answers: u32,
    /// maximum number of received answers before a lost answer in this span
    pub consecutive_answers_maximum: u32,
    /// average number of received answers before a lost answer in this span
    pub consecutive_answers_average: u32,
    /// minimum number of received answers before a lost answer in this span
    pub consecutive_answers_minimum: u32,
    /// maximum number of timeouts before a received answer in this span
    pub consecutive_lost_answers_maximum: u32,
    /// average number of timeouts before a received answer in this span
    pub consecutive_lost_answers_average: u32,
    /// minimum number of timeouts before a received answer in this span
    pub consecutive_lost_answers_minimum: u32,
}

impl fmt::Display for AnswerStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "span: {}", self.span)?;
        writeln!(
            f,
            "questions/answers/lost: {} / {} / {}",
            self.questions, self.answers, self.lost_answers
        )?;
        writeln!(
            f,
            "consecutive answers min/avg/max: {} / {} / {}",
            self.consecutive_answers_minimum,
            self.consecutive_answers_average,
            self.consecutive_answers_maximum
        )?;
        writeln!(
            f,
            "consecutive lost min/avg/max: {} / {} / {}",
            self.consecutive_lost_answers_minimum,
            self.consecutive_lost_answers_average,
            self.consecutive_lost_answers_maximum
        )?;

        Ok(())
    }
}

/// Statistics for RPC operations performed on a node
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), derive(Tsify))]
pub struct RPCStats {
    /// number of rpcs that have been sent in the total entry time range
    pub messages_sent: u32,
    /// number of rpcs that have been received in the total entry time range
    pub messages_rcvd: u32,
    /// number of questions issued that have yet to be answered
    pub questions_in_flight: u32,
    /// when the peer was last questioned (either successfully or not) and we wanted an answer
    pub last_question_ts: Option<Timestamp>,
    /// when the peer was last seen for any reason, including when we first attempted to reach out to it
    pub last_seen_ts: Option<Timestamp>,
    /// the timestamp of the first consecutive proof-of-life for this node (an answer or received question)
    pub first_consecutive_seen_ts: Option<Timestamp>,
    /// number of answers that have been lost consecutively
    pub recent_lost_answers: u32,
    /// number of messages that have failed to send or connections dropped since we last successfully sent one
    pub failed_to_send: u32,
    /// rpc answer stats for this peer
    #[serde(default)]
    pub answer: AnswerStats,
}

impl fmt::Display for RPCStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "# sent/received/in-flight:      {} / {} / {}",
            self.messages_sent, self.messages_rcvd, self.questions_in_flight
        )?;
        writeln!(
            f,
            "# recently-lost/failed-to-send: {} / {}",
            self.recent_lost_answers, self.failed_to_send
        )?;
        writeln!(
            f,
            "last_question:     {}",
            if let Some(ts) = &self.last_question_ts {
                ts.to_string()
            } else {
                "None".to_owned()
            }
        )?;
        writeln!(
            f,
            "last_seen:         {}",
            if let Some(ts) = &self.last_seen_ts {
                ts.to_string()
            } else {
                "None".to_owned()
            }
        )?;
        writeln!(
            f,
            "first_consecutive: {}",
            if let Some(ts) = &self.first_consecutive_seen_ts {
                ts.to_string()
            } else {
                "None".to_owned()
            }
        )?;

        write!(f, "answers:\n{}", indent_all_string(&self.answer))?;

        Ok(())
    }
}

/// Statistics for a peer in the routing table
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), derive(Tsify))]
pub struct PeerStats {
    /// when the peer was added to the routing table
    pub time_added: Timestamp,
    #[serde(default)]
    /// information about RPCs
    pub rpc_stats: RPCStats,
    #[serde(default)]
    /// latency stats for this peer
    pub latency: Option<LatencyStats>,
    /// transfer stats for this peer
    #[serde(default)]
    pub transfer: TransferStatsDownUp,
    /// state stats for this peer
    #[serde(default)]
    pub state: StateStats,
}

impl fmt::Display for PeerStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "time_added: {}", self.time_added)?;
        write!(f, "rpc_stats:\n{}", indent_all_string(&self.rpc_stats))?;
        if let Some(ls) = &self.latency {
            writeln!(f, "latency: {}", ls)?;
        } else {
            writeln!(f, "latency: None")?;
        }
        write!(f, "transfer:\n{}", indent_all_string(&self.transfer))?;
        write!(f, "state:\n{}", indent_all_string(&self.state))?;

        Ok(())
    }
}
