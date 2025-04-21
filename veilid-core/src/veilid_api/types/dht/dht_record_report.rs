use super::*;

/// DHT Record Report
#[derive(Default, Clone, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(
    all(target_arch = "wasm32", target_os = "unknown"),
    derive(Tsify),
    tsify(from_wasm_abi, into_wasm_abi)
)]
#[must_use]
pub struct DHTRecordReport {
    /// The actual subkey range within the schema being reported on
    /// This may be a subset of the requested range if it exceeds the schema limits
    /// or has more than 512 subkeys
    subkeys: ValueSubkeyRangeSet,
    /// The subkeys that have been writen offline that still need to be flushed
    offline_subkeys: ValueSubkeyRangeSet,
    /// The sequence numbers of each subkey requested from a locally stored DHT Record
    local_seqs: Vec<ValueSeqNum>,
    /// The sequence numbers of each subkey requested from the DHT over the network
    network_seqs: Vec<ValueSeqNum>,
}
from_impl_to_jsvalue!(DHTRecordReport);

impl DHTRecordReport {
    pub fn new(
        subkeys: ValueSubkeyRangeSet,
        offline_subkeys: ValueSubkeyRangeSet,
        local_seqs: Vec<ValueSeqNum>,
        network_seqs: Vec<ValueSeqNum>,
    ) -> Self {
        Self {
            subkeys,
            offline_subkeys,
            local_seqs,
            network_seqs,
        }
    }

    pub fn subkeys(&self) -> &ValueSubkeyRangeSet {
        &self.subkeys
    }
    pub fn offline_subkeys(&self) -> &ValueSubkeyRangeSet {
        &self.offline_subkeys
    }
    #[must_use]
    pub fn local_seqs(&self) -> &[ValueSeqNum] {
        &self.local_seqs
    }
    #[must_use]
    pub fn network_seqs(&self) -> &[ValueSeqNum] {
        &self.network_seqs
    }
    pub fn changed_subkeys(&self) -> ValueSubkeyRangeSet {
        let mut changed = ValueSubkeyRangeSet::new();
        for ((sk, lseq), nseq) in self
            .subkeys
            .iter()
            .zip(self.local_seqs.iter())
            .zip(self.network_seqs.iter())
        {
            if nseq > lseq {
                changed.insert(sk);
            }
        }
        changed
    }
}

impl fmt::Debug for DHTRecordReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DHTRecordReport {{\n  subkeys: {:?}\n  offline_subkeys: {:?}\n  local_seqs:\n{}\n  remote_seqs:\n{}\n}}\n",
            &self.subkeys,
            &self.offline_subkeys,
            &debug_seqs(&self.local_seqs),
            &debug_seqs(&self.network_seqs)
        )
    }
}

/// DHT Record Report Scope
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    JsonSchema,
    Default,
)]
#[cfg_attr(
    all(target_arch = "wasm32", target_os = "unknown"),
    derive(Tsify),
    tsify(from_wasm_abi, into_wasm_abi, namespace)
)]
pub enum DHTReportScope {
    /// Return only the local copy sequence numbers
    /// Useful for seeing what subkeys you have locally and which ones have not been retrieved
    #[default]
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
