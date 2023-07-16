use super::*;

/// Information required to handle locally opened records
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalRecordDetail {
    /// The last 'safety selection' used when creating/opening this record.
    /// Even when closed, this safety selection applies to re-publication attempts by the system.
    pub safety_selection: SafetySelection,
}
