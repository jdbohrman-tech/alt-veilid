use super::*;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PeerInfo {
    node_ids: TypedKeyGroup,
    signed_node_info: SignedNodeInfo,
}

impl PeerInfo {
    pub fn new(node_ids: TypedKeyGroup, signed_node_info: SignedNodeInfo) -> Self {
        assert!(node_ids.len() > 0 && node_ids.len() <= MAX_CRYPTO_KINDS);
        Self {
            node_ids,
            signed_node_info,
        }
    }

    pub fn validate(&self, crypto: Crypto) -> VeilidAPIResult<()> {
        let validated_node_ids = self.signed_node_info.validate(&self.node_ids, crypto)?;
        if validated_node_ids.is_empty() {
            // Shouldn't get here because signed node info validation also checks this
            apibail_generic!("no validated node ids");
        }
        Ok(())
    }

    pub fn node_ids(&self) -> &TypedKeyGroup {
        &self.node_ids
    }
    pub fn signed_node_info(&self) -> &SignedNodeInfo {
        &self.signed_node_info
    }
    pub fn destructure(self) -> (TypedKeyGroup, SignedNodeInfo) {
        (self.node_ids, self.signed_node_info)
    }

    pub fn validate_vec(peer_info_vec: &mut Vec<PeerInfo>, crypto: Crypto) {
        let mut n = 0usize;
        while n < peer_info_vec.len() {
            let pi = peer_info_vec.get(n).unwrap();
            if pi.validate(crypto.clone()).is_err() {
                peer_info_vec.remove(n);
            } else {
                n += 1;
            }
        }
    }
}
