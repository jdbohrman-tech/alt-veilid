use super::*;

/// DHT Record Report
#[derive(
    Debug, Default, Clone, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize, JsonSchema,
)]
#[cfg_attr(
    target_arch = "wasm32",
    derive(Tsify),
    tsify(from_wasm_abi, into_wasm_abi)
)]
pub struct DHTRecordReport {
    /// The actual subkey range within the schema being reported on
    /// This may be a subset of the requested range if it exceeds the schema limits
    /// or has more than 512 subkeys
    subkeys: ValueSubkeyRangeSet,
    /// The sequence numbers of each subkey requested from a locally stored DHT Record
    local_seqs: Vec<ValueSeqNum>,
    /// The sequence numbers of each subkey requested from the DHT over the network
    network_seqs: Vec<ValueSeqNum>,
}
from_impl_to_jsvalue!(DHTRecordReport);

impl DHTRecordReport {
    pub fn new(
        subkeys: ValueSubkeyRangeSet,
        local_seqs: Vec<ValueSeqNum>,
        network_seqs: Vec<ValueSeqNum>,
    ) -> Self {
        Self {
            subkeys,
            local_seqs,
            network_seqs,
        }
    }

    pub fn subkeys(&self) -> &ValueSubkeyRangeSet {
        &self.subkeys
    }
    pub fn local_seqs(&self) -> &[ValueSeqNum] {
        &self.local_seqs
    }
    pub fn network_seqs(&self) -> &[ValueSeqNum] {
        &self.network_seqs
    }
}

/// DHT Record Report Scope
#[derive(
    Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, JsonSchema,
)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify), tsify(from_wasm_abi, namespace))]
pub enum DHTReportScope {
    /// Return only the local copy sequence numbers
    /// Useful for seeing what subkeys you have locally and which ones have not been retrieved
    Local = 0,
    /// Return the local sequence numbers and the network sequence numbers with GetValue fanout parameters
    /// Provides an independent view of both the local sequence numbers and the network sequence numbers for nodes that
    /// would be reached as if the local copy did not exist locally.
    /// Useful for determining if the current local copy should be updated from the network.
    SyncGet = 1,
    /// Return the local sequence numbers and the network sequence numbers with SetValue fanout parameters
    /// Provides an independent view of both the local sequence numbers and the network sequence numbers for nodes that
    /// would be reached as if the local copy did not exist locally.
    /// Useful for determining if the unchanged local copy should be pushed to the network.
    SyncSet = 2,
    /// Return the local sequence numbers and the network sequence numbers with GetValue fanout parameters
    /// Provides an view of both the local sequence numbers and the network sequence numbers for nodes that
    /// would be reached as if a GetValue operation were being performed, including accepting newer values from the network.
    /// Useful for determining which subkeys would change with a GetValue operation
    UpdateGet = 3,
    /// Return the local sequence numbers and the network sequence numbers with SetValue fanout parameters
    /// Provides an view of both the local sequence numbers and the network sequence numbers for nodes that
    /// would be reached as if a SetValue operation were being performed, including accepting newer values from the network.
    /// This simulates a SetValue with the initial sequence number incremented by 1, like a real SetValue would when updating.
    /// Useful for determine which subkeys would change on an SetValue operation
    UpdateSet = 4,
}
impl Default for DHTReportScope {
    fn default() -> Self {
        Self::Local
    }
}
