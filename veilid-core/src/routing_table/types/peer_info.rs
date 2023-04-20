use super::*;

#[derive(Clone, Debug, Serialize, Deserialize, RkyvArchive, RkyvSerialize, RkyvDeserialize)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct PeerInfo {
    node_ids: TypedKeySet,
    signed_node_info: SignedNodeInfo,
}

impl PeerInfo {
    pub fn new(node_ids: TypedKeySet, signed_node_info: SignedNodeInfo) -> Self {
        assert!(node_ids.len() > 0 && node_ids.len() <= MAX_CRYPTO_KINDS);
        Self {
            node_ids,
            signed_node_info,
        }
    }

    pub fn validate(&self, crypto: Crypto) -> Result<TypedKeySet, VeilidAPIError> {
        self.signed_node_info.validate(&self.node_ids, crypto)
    }

    pub fn node_ids(&self) -> &TypedKeySet {
        &self.node_ids
    }
    pub fn signed_node_info(&self) -> &SignedNodeInfo {
        &self.signed_node_info
    }
    pub fn into_signed_node_info(self) -> SignedNodeInfo {
        self.signed_node_info
    }
    pub fn into_fields(self) -> (TypedKeySet, SignedNodeInfo) {
        (self.node_ids, self.signed_node_info)
    }
}
