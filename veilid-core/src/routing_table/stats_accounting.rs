use crate::*;
use alloc::collections::VecDeque;

// Latency entry is per round-trip packet (ping or data)
// - Size is number of entries
const ROLLING_LATENCIES_SIZE: usize = 10;

// Transfers entries are in bytes total for the interval
// - Size is number of entries
// - Interval is number of seconds in each entry
const ROLLING_TRANSFERS_SIZE: usize = 10;
pub const ROLLING_TRANSFERS_INTERVAL_SECS: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TransferCount {
    down: u64,
    up: u64,
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

    pub fn add_down(&mut self, bytes: u64) {
        self.current_transfer.down += bytes;
    }

    pub fn add_up(&mut self, bytes: u64) {
        self.current_transfer.up += bytes;
    }

    pub fn roll_transfers(
        &mut self,
        last_ts: u64,
        cur_ts: u64,
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

        transfer_stats.down.maximum = 0;
        transfer_stats.up.maximum = 0;
        transfer_stats.down.minimum = u64::MAX;
        transfer_stats.up.minimum = u64::MAX;
        transfer_stats.down.average = 0;
        transfer_stats.up.average = 0;
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
        transfer_stats.down.average /= len;
        transfer_stats.up.average /= len;
    }
}

#[derive(Debug, Clone, Default)]
pub struct LatencyStatsAccounting {
    rolling_latencies: VecDeque<u64>,
}

impl LatencyStatsAccounting {
    pub fn new() -> Self {
        Self {
            rolling_latencies: VecDeque::new(),
        }
    }

    pub fn record_latency(&mut self, latency: u64) -> veilid_api::LatencyStats {
        while self.rolling_latencies.len() >= ROLLING_LATENCIES_SIZE {
            self.rolling_latencies.pop_front();
        }
        self.rolling_latencies.push_back(latency);

        let mut ls = LatencyStats {
            fastest: u64::MAX,
            average: 0,
            slowest: 0,
        };
        for rl in &self.rolling_latencies {
            ls.fastest.min_assign(*rl);
            ls.slowest.max_assign(*rl);
            ls.average += *rl;
        }
        let len = self.rolling_latencies.len() as u64;
        ls.average /= len;

        ls
    }
}
