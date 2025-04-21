use super::*;

impl_veilid_log_facility!("stor");

/// Requested parameters for watch
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OutboundWatchParameters {
    /// Requested expiration timestamp. A zero timestamp here indicates
    /// that the watch it to be renewed indefinitely
    pub expiration_ts: Timestamp,
    /// How many notifications the requestor asked for
    pub count: u32,
    /// Subkeys requested for this watch
    pub subkeys: ValueSubkeyRangeSet,
    /// What key to use to perform the watch
    pub opt_watcher: Option<KeyPair>,
    /// What safety selection to use on the network
    pub safety_selection: SafetySelection,
}

impl fmt::Display for OutboundWatchParameters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ expiration={}, count={}, subkeys={}, opt_watcher={}, safety_selection={:?} }}",
            self.expiration_ts,
            self.count,
            self.subkeys,
            if let Some(watcher) = &self.opt_watcher {
                watcher.to_string()
            } else {
                "None".to_owned()
            },
            self.safety_selection
        )
    }
}
