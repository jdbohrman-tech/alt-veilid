use super::*;

/// Non-nodeinfo status for each node is returned by the StatusA call

#[derive(Clone, Debug, Serialize, Deserialize, RkyvArchive, RkyvSerialize, RkyvDeserialize)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct NodeStatus {
    // Reserved for expansion
}
