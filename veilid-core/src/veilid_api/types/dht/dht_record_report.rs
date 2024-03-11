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
    /// The sequence numbers of each subkey requested from a DHT Record
    seqs: Vec<ValueSeqNum>,
}
from_impl_to_jsvalue!(DHTRecordReport);

impl DHTRecordReport {
    pub fn new(subkeys: ValueSubkeyRangeSet, seqs: Vec<ValueSeqNum>) -> Self {
        Self { subkeys, seqs }
    }

    pub fn seqs(&self) -> &[ValueSeqNum] {
        &self.seqs
    }
}

/// DHT Record Report Scope
#[derive(
    Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, JsonSchema,
)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify), tsify(from_wasm_abi, namespace))]
pub enum DHTReportScope {
    Local = 0,
    NetworkGet = 1,
    NetworkSet = 2,
}
impl Default for DHTReportScope {
    fn default() -> Self {
        Self::Local
    }
}
