use super::*;
use cursive_table_view::*;
use std::cmp::Ordering;
use veilid_core::*;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum PeerTableColumn {
    NodeId,
    Address,
    LatencyAvg,
    TransferDownAvg,
    TransferUpAvg,
}

// impl PeerTableColumn {
//     fn as_str(&self) -> &str {
//         match self {
//             PeerTableColumn::NodeId => "Node Id",
//             PeerTableColumn::Address => "Address",
//             PeerTableColumn::LatencyAvg => "Latency",
//             PeerTableColumn::TransferDownAvg => "Down",
//             PeerTableColumn::TransferUpAvg => "Up",
//         }
//     }
// }

fn format_ts(ts: Timestamp) -> String {
    let ts = ts.as_u64();
    let secs = timestamp_to_secs(ts);
    if secs >= 1.0 {
        format!("{:.2}s", timestamp_to_secs(ts))
    } else {
        format!("{:.2}ms", timestamp_to_secs(ts) * 1000.0)
    }
}

fn format_bps(bps: ByteCount) -> String {
    let bps = bps.as_u64();
    if bps >= 1024u64 * 1024u64 * 1024u64 {
        format!("{:.2}GB/s", (bps / (1024u64 * 1024u64)) as f64 / 1024.0)
    } else if bps >= 1024u64 * 1024u64 {
        format!("{:.2}MB/s", (bps / 1024u64) as f64 / 1024.0)
    } else if bps >= 1024u64 {
        format!("{:.2}KB/s", bps as f64 / 1024.0)
    } else {
        format!("{:.2}B/s", bps as f64)
    }
}

impl TableViewItem<PeerTableColumn> for PeerTableData {
    fn to_column(&self, column: PeerTableColumn) -> String {
        match column {
            PeerTableColumn::NodeId => self
                .node_ids
                .first()
                .cloned()
                .unwrap_or_else(|| "???".to_owned()),
            PeerTableColumn::Address => self.peer_address.clone(),
            PeerTableColumn::LatencyAvg => format!(
                "{}",
                self.peer_stats
                    .latency
                    .as_ref()
                    .map(|l| format_ts(l.average))
                    .unwrap_or("---".to_owned())
            ),
            PeerTableColumn::TransferDownAvg => format_bps(self.peer_stats.transfer.down.average),
            PeerTableColumn::TransferUpAvg => format_bps(self.peer_stats.transfer.up.average),
        }
    }

    fn cmp(&self, other: &Self, column: PeerTableColumn) -> Ordering
    where
        Self: Sized,
    {
        match column {
            PeerTableColumn::NodeId => self.to_column(column).cmp(&other.to_column(column)),
            PeerTableColumn::Address => self.to_column(column).cmp(&other.to_column(column)),
            PeerTableColumn::LatencyAvg => self
                .peer_stats
                .latency
                .as_ref()
                .map(|l| l.average)
                .cmp(&other.peer_stats.latency.as_ref().map(|l| l.average)),
            PeerTableColumn::TransferDownAvg => self
                .peer_stats
                .transfer
                .down
                .average
                .cmp(&other.peer_stats.transfer.down.average),
            PeerTableColumn::TransferUpAvg => self
                .peer_stats
                .transfer
                .up
                .average
                .cmp(&other.peer_stats.transfer.up.average),
        }
    }
}

pub type PeersTableView = TableView<PeerTableData, PeerTableColumn>;
