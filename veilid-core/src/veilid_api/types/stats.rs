use super::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
pub struct LatencyStats {
    pub fastest: TimestampDuration, // fastest latency in the ROLLING_LATENCIES_SIZE last latencies
    pub average: TimestampDuration, // average latency over the ROLLING_LATENCIES_SIZE last latencies
    pub slowest: TimestampDuration, // slowest latency in the ROLLING_LATENCIES_SIZE last latencies
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
pub struct TransferStats {
    pub total: ByteCount,   // total amount transferred ever
    pub maximum: ByteCount, // maximum rate over the ROLLING_TRANSFERS_SIZE last amounts
    pub average: ByteCount, // average rate over the ROLLING_TRANSFERS_SIZE last amounts
    pub minimum: ByteCount, // minimum rate over the ROLLING_TRANSFERS_SIZE last amounts
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
pub struct TransferStatsDownUp {
    pub down: TransferStats,
    pub up: TransferStats,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
pub struct RPCStats {
    pub messages_sent: u32, // number of rpcs that have been sent in the total_time range
    pub messages_rcvd: u32, // number of rpcs that have been received in the total_time range
    pub questions_in_flight: u32, // number of questions issued that have yet to be answered
    pub last_question_ts: Option<Timestamp>, // when the peer was last questioned (either successfully or not) and we wanted an answer
    pub last_seen_ts: Option<Timestamp>, // when the peer was last seen for any reason, including when we first attempted to reach out to it
    pub first_consecutive_seen_ts: Option<Timestamp>, // the timestamp of the first consecutive proof-of-life for this node (an answer or received question)
    pub recent_lost_answers: u32, // number of answers that have been lost since we lost reliability
    pub failed_to_send: u32, // number of messages that have failed to send since we last successfully sent one
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
pub struct PeerStats {
    pub time_added: Timestamp, // when the peer was added to the routing table
    pub rpc_stats: RPCStats,   // information about RPCs
    pub latency: Option<LatencyStats>, // latencies for communications with the peer
    pub transfer: TransferStatsDownUp, // Stats for communications with the peer
}
