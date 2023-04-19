use super::*;

/// Signed NodeInfo that can be passed around amongst peers and verifiable
#[derive(Clone, Debug, Serialize, Deserialize, RkyvArchive, RkyvSerialize, RkyvDeserialize)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct SignedDirectNodeInfo {
    pub node_info: NodeInfo,
    pub timestamp: Timestamp,
    pub signatures: Vec<TypedSignature>,
}
impl SignedDirectNodeInfo {
    /// Returns a new SignedDirectNodeInfo that has its signatures validated.
    /// On success, this will modify the node_ids set to only include node_ids whose signatures validate.
    /// All signatures are stored however, as this can be passed to other nodes that may be able to validate those signatures.
    pub fn new(
        crypto: Crypto,
        node_ids: &mut TypedKeySet,
        node_info: NodeInfo,
        timestamp: Timestamp,
        typed_signatures: Vec<TypedSignature>,
    ) -> Result<Self, VeilidAPIError> {
        let node_info_bytes = Self::make_signature_bytes(&node_info, timestamp)?;

        // Verify the signatures that we can
        let validated_node_ids =
            crypto.verify_signatures(node_ids, &node_info_bytes, &typed_signatures)?;
        *node_ids = validated_node_ids;
        if node_ids.len() == 0 {
            apibail_generic!("no valid node ids in direct node info");
        }

        Ok(Self {
            node_info,
            timestamp,
            signatures: typed_signatures,
        })
    }

    pub fn make_signatures(
        crypto: Crypto,
        typed_key_pairs: Vec<TypedKeyPair>,
        node_info: NodeInfo,
    ) -> Result<Self, VeilidAPIError> {
        let timestamp = get_aligned_timestamp();
        let node_info_bytes = Self::make_signature_bytes(&node_info, timestamp)?;
        let typed_signatures =
            crypto.generate_signatures(&node_info_bytes, &typed_key_pairs, |kp, s| {
                TypedSignature::new(kp.kind, s)
            })?;
        Ok(Self {
            node_info,
            timestamp,
            signatures: typed_signatures,
        })
    }

    fn make_signature_bytes(
        node_info: &NodeInfo,
        timestamp: Timestamp,
    ) -> Result<Vec<u8>, VeilidAPIError> {
        let mut node_info_bytes = Vec::new();

        // Add nodeinfo to signature
        let mut ni_msg = ::capnp::message::Builder::new_default();
        let mut ni_builder = ni_msg.init_root::<veilid_capnp::node_info::Builder>();
        encode_node_info(node_info, &mut ni_builder).map_err(VeilidAPIError::internal)?;
        node_info_bytes.append(&mut builder_to_vec(ni_msg).map_err(VeilidAPIError::internal)?);

        // Add timestamp to signature
        node_info_bytes.append(&mut timestamp.as_u64().to_le_bytes().to_vec());

        Ok(node_info_bytes)
    }

    pub fn with_no_signature(node_info: NodeInfo) -> Self {
        Self {
            node_info,
            timestamp: get_aligned_timestamp(),
            signatures: Vec::new(),
        }
    }

    pub fn has_any_signature(&self) -> bool {
        !self.signatures.is_empty()
    }
}
