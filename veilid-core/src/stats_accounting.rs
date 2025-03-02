use super::*;

// Latency entry is per round-trip packet (ping or data)
// - Size is number of entries
const ROLLING_LATENCIES_SIZE: usize = 50;

// Transfers entries are in bytes total for the interval
// - Size is number of entries
// - Interval is number of seconds in each entry
const ROLLING_TRANSFERS_SIZE: usize = 10;
pub const ROLLING_TRANSFERS_INTERVAL_SECS: u32 = 1;

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
