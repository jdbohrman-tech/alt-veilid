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
    local_seqs: Vec<Option<ValueSeqNum>>,
    /// The sequence numbers of each subkey requested from the DHT over the network
    network_seqs: Vec<Option<ValueSeqNum>>,
}

impl DHTRecordReport {
    pub(crate) fn new(
        subkeys: ValueSubkeyRangeSet,
        offline_subkeys: ValueSubkeyRangeSet,
        local_seqs: Vec<Option<ValueSeqNum>>,
        network_seqs: Vec<Option<ValueSeqNum>>,
    ) -> VeilidAPIResult<Self> {
        if subkeys.is_full() {
            apibail_invalid_argument!("subkeys range should be exact", "subkeys", subkeys);
        }
        if subkeys.is_empty() {
            apibail_invalid_argument!("subkeys range should not be empty", "subkeys", subkeys);
        }
        if subkeys.len() > MAX_INSPECT_VALUE_A_SEQS_LEN as u64 {
            apibail_invalid_argument!("subkeys range is too large", "subkeys", subkeys);
        }
        if subkeys.len() != local_seqs.len() as u64 {
            apibail_invalid_argument!(
                "local seqs list does not match subkey length",
                "local_seqs",
                local_seqs.len()
            );
        }
        if subkeys.len() != network_seqs.len() as u64 {
            apibail_invalid_argument!(
                "network seqs list does not match subkey length",
                "network_seqs",
                network_seqs.len()
            );
        }
        if !offline_subkeys.is_subset(&subkeys) {
            apibail_invalid_argument!(
                "offline subkeys is not a subset of the whole subkey set",
                "offline_subkeys",
                offline_subkeys
            );
        }

        Ok(Self {
            subkeys,
            offline_subkeys,
            local_seqs,
            network_seqs,
        })
    }

    pub fn subkeys(&self) -> &ValueSubkeyRangeSet {
        &self.subkeys
    }
    pub fn offline_subkeys(&self) -> &ValueSubkeyRangeSet {
        &self.offline_subkeys
    }
    #[must_use]
    pub fn local_seqs(&self) -> &[Option<ValueSeqNum>] {
        &self.local_seqs
    }
    #[must_use]
    pub fn network_seqs(&self) -> &[Option<ValueSeqNum>] {
        &self.network_seqs
    }
    pub fn newer_online_subkeys(&self) -> ValueSubkeyRangeSet {
        let mut newer_online = ValueSubkeyRangeSet::new();
        for ((sk, lseq), nseq) in self
            .subkeys
            .iter()
            .zip(self.local_seqs.iter())
            .zip(self.network_seqs.iter())
        {
            if let Some(nseq) = nseq {
                if lseq.is_none() || *nseq > lseq.unwrap() {
                    newer_online.insert(sk);
                }
            }
        }
        newer_online
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
