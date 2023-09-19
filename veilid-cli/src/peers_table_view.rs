use super::*;
use cursive_table_view::*;
use std::cmp::Ordering;

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

fn format_ts(ts: &json::JsonValue) -> String {
    if ts.is_null() {
        return "---".to_owned();
    }
    let ts = json_str_u64(ts);
    let secs = timestamp_to_secs(ts);
    if secs >= 1.0 {
        format!("{:.2}s", timestamp_to_secs(ts))
    } else {
        format!("{:.2}ms", timestamp_to_secs(ts) * 1000.0)
    }
}

fn format_bps(bps: &json::JsonValue) -> String {
    if bps.is_null() {
        return "---".to_owned();
    }
    let bps = json_str_u64(bps);
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

impl TableViewItem<PeerTableColumn> for json::JsonValue {
    fn to_column(&self, column: PeerTableColumn) -> String {
        match column {
            PeerTableColumn::NodeId => self["node_ids"][0].to_string(),
            PeerTableColumn::Address => self["peer_address"].to_string(),
            PeerTableColumn::LatencyAvg => {
                format_ts(&self["peer_stats"]["latency"]["average"]).to_string()
            }
            PeerTableColumn::TransferDownAvg => {
                format_bps(&self["peer_stats"]["transfer"]["down"]["average"])
            }
            PeerTableColumn::TransferUpAvg => {
                format_bps(&self["peer_stats"]["transfer"]["up"]["average"])
            }
        }
    }

    fn cmp(&self, other: &Self, column: PeerTableColumn) -> Ordering
    where
        Self: Sized,
    {
        match column {
            PeerTableColumn::NodeId => self
                .to_column(column)
                .to_ascii_lowercase()
                .cmp(&other.to_column(column).to_ascii_lowercase()),
            PeerTableColumn::Address => self
                .to_column(column)
                .to_ascii_lowercase()
                .cmp(&other.to_column(column).to_ascii_lowercase()),
            PeerTableColumn::LatencyAvg => json_str_u64(&self["peer_stats"]["latency"]["average"])
                .cmp(&json_str_u64(&other["peer_stats"]["latency"]["average"])),
            PeerTableColumn::TransferDownAvg => {
                json_str_u64(&self["peer_stats"]["transfer"]["down"]["average"]).cmp(&json_str_u64(
                    &other["peer_stats"]["transfer"]["down"]["average"],
                ))
            }
            PeerTableColumn::TransferUpAvg => {
                json_str_u64(&self["peer_stats"]["transfer"]["up"]["average"]).cmp(&json_str_u64(
                    &other["peer_stats"]["transfer"]["up"]["average"],
                ))
            }
        }
    }
}

pub type PeersTableView = TableView<json::JsonValue, PeerTableColumn>;
