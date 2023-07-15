use super::*;

/// Signed NodeInfo with a relay that can be passed around amongst peers and verifiable
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignedRelayedNodeInfo {
    node_info: NodeInfo,
    relay_ids: TypedKeyGroup,
    relay_info: SignedDirectNodeInfo,
    timestamp: Timestamp,
    signatures: Vec<TypedSignature>,
}

impl SignedRelayedNodeInfo {
    /// Returns a new SignedRelayedNodeInfo that has its signatures validated.
    /// On success, this will modify the node_ids set to only include node_ids whose signatures validate.
    /// All signatures are stored however, as this can be passed to other nodes that may be able to validate those signatures.
    pub fn new(
        node_info: NodeInfo,
        relay_ids: TypedKeyGroup,
        relay_info: SignedDirectNodeInfo,
        timestamp: Timestamp,
        signatures: Vec<TypedSignature>,
    ) -> Self {
        Self {
            node_info,
            relay_ids,
            relay_info,
            timestamp,
            signatures,
        }
    }

    pub fn validate(
        &self,
        node_ids: &TypedKeyGroup,
        crypto: Crypto,
    ) -> VeilidAPIResult<TypedKeyGroup> {
        // Ensure the relay info for the node has a superset of the crypto kinds of the node it is relaying
        if common_crypto_kinds(
            self.node_info.crypto_support(),
            self.relay_info.node_info().crypto_support(),
        )
        .len()
            != self.node_info.crypto_support().len()
        {
            apibail_generic!("relay should have superset of node crypto kinds");
        }

        // Verify signatures
        let node_info_bytes = Self::make_signature_bytes(
            &self.node_info,
            &self.relay_ids,
            &self.relay_info,
            self.timestamp,
        )?;
        let validated_node_ids =
            crypto.verify_signatures(node_ids, &node_info_bytes, &self.signatures)?;
        if validated_node_ids.len() == 0 {
            apibail_generic!("no valid node ids in relayed node info");
        }
        Ok(validated_node_ids)
    }

    pub fn make_signatures(
        crypto: Crypto,
        typed_key_pairs: Vec<TypedKeyPair>,
        node_info: NodeInfo,
        relay_ids: TypedKeyGroup,
        relay_info: SignedDirectNodeInfo,
    ) -> VeilidAPIResult<Self> {
        let timestamp = get_aligned_timestamp();
        let node_info_bytes =
            Self::make_signature_bytes(&node_info, &relay_ids, &relay_info, timestamp)?;
        let typed_signatures =
            crypto.generate_signatures(&node_info_bytes, &typed_key_pairs, |kp, s| {
                TypedSignature::new(kp.kind, s)
            })?;
        Ok(Self {
            node_info,
            relay_ids,
            relay_info,
            timestamp,
            signatures: typed_signatures,
        })
    }

    fn make_signature_bytes(
        node_info: &NodeInfo,
        relay_ids: &[TypedKey],
        relay_info: &SignedDirectNodeInfo,
        timestamp: Timestamp,
    ) -> VeilidAPIResult<Vec<u8>> {
        let mut sig_bytes = Vec::new();

        // Add nodeinfo to signature
        let mut ni_msg = ::capnp::message::Builder::new_default();
        let mut ni_builder = ni_msg.init_root::<veilid_capnp::node_info::Builder>();
        encode_node_info(node_info, &mut ni_builder).map_err(VeilidAPIError::internal)?;
        sig_bytes.append(&mut builder_to_vec(ni_msg).map_err(VeilidAPIError::internal)?);

        // Add relay ids to signature
        for relay_id in relay_ids {
            let mut rid_msg = ::capnp::message::Builder::new_default();
            let mut rid_builder = rid_msg.init_root::<veilid_capnp::typed_key::Builder>();
            encode_typed_key(relay_id, &mut rid_builder);
            sig_bytes.append(&mut builder_to_vec(rid_msg).map_err(VeilidAPIError::internal)?);
        }

        // Add relay info to signature
        let mut ri_msg = ::capnp::message::Builder::new_default();
        let mut ri_builder = ri_msg.init_root::<veilid_capnp::signed_direct_node_info::Builder>();
        encode_signed_direct_node_info(relay_info, &mut ri_builder)
            .map_err(VeilidAPIError::internal)?;
        sig_bytes.append(&mut builder_to_vec(ri_msg).map_err(VeilidAPIError::internal)?);

        // Add timestamp to signature
        sig_bytes.append(&mut timestamp.as_u64().to_le_bytes().to_vec());

        Ok(sig_bytes)
    }

    pub fn has_any_signature(&self) -> bool {
        !self.signatures.is_empty()
    }

    pub fn node_info(&self) -> &NodeInfo {
        &self.node_info
    }
    pub fn timestamp(&self) -> Timestamp {
        self.timestamp
    }
    pub fn relay_ids(&self) -> &TypedKeyGroup {
        &self.relay_ids
    }
    pub fn relay_info(&self) -> &SignedDirectNodeInfo {
        &self.relay_info
    }
    pub fn signatures(&self) -> &[TypedSignature] {
        &self.signatures
    }
}
