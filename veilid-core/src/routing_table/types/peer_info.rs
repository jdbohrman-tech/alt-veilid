use super::*;

#[derive(Clone, Debug, Serialize, Deserialize, RkyvArchive, RkyvSerialize, RkyvDeserialize)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct PeerInfo {
    pub node_ids: TypedKeySet,
    pub signed_node_info: SignedNodeInfo,
}

impl PeerInfo {
    pub fn new(node_ids: TypedKeySet, signed_node_info: SignedNodeInfo) -> Self {
        assert!(node_ids.len() > 0 && node_ids.len() <= MAX_CRYPTO_KINDS);
        Self {
            node_ids,
            signed_node_info,
        }
    }
}
