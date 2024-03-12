use super::*;

/// Information about nodes that cache a local record remotely
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub(in crate::storage_manager) struct PerNodeRecordDetail {
    pub last_set: Timestamp,
    pub last_seen: Timestamp,
    pub subkeys: ValueSubkeyRangeSet,
}

/// Information required to handle locally opened records
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(in crate::storage_manager) struct LocalRecordDetail {
    /// The last 'safety selection' used when creating/opening this record.
    /// Even when closed, this safety selection applies to re-publication attempts by the system.
    pub safety_selection: SafetySelection,
    /// The nodes that we have seen this record cached on recently
    #[serde(default)]
    pub nodes: HashMap<PublicKey, PerNodeRecordDetail>,
}

impl LocalRecordDetail {
    pub fn new(safety_selection: SafetySelection) -> Self {
        Self {
            safety_selection,
            nodes: Default::default(),
        }
    }
}
