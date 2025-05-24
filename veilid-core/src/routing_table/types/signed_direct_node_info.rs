use super::*;

/// Signed NodeInfo that can be passed around amongst peers and verifiable
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignedDirectNodeInfo {
    node_info: NodeInfo,
    timestamp: Timestamp,
    signatures: Vec<TypedSignature>,
}

impl fmt::Display for SignedDirectNodeInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "node_info:")?;
        write!(f, "{}", indent_all_string(&self.node_info))?;
        writeln!(f, "timestamp: {}", self.timestamp)?;
        writeln!(f, "signatures:")?;
        for sig in &self.signatures {
            writeln!(f, "{}", indent_all_string(sig))?;
        }
        Ok(())
    }
}

impl SignedDirectNodeInfo {
    /// Returns a new SignedDirectNodeInfo that has its signatures validated.
    /// On success, this will modify the node_ids set to only include node_ids whose signatures validate.
    /// All signatures are stored however, as this can be passed to other nodes that may be able to validate those signatures.
    pub fn new(node_info: NodeInfo, timestamp: Timestamp, signatures: Vec<TypedSignature>) -> Self {
        Self {
            node_info,
            timestamp,
            signatures,
        }
    }

    pub fn validate(
        &self,
        node_ids: &TypedNodeIdGroup,
        crypto: &Crypto,
    ) -> VeilidAPIResult<TypedNodeIdGroup> {
        let node_info_bytes = Self::make_signature_bytes(&self.node_info, self.timestamp)?;

        let public_keys = TypedPublicKeyGroup::from(node_ids.clone());
        // Verify the signatures that we can
        let opt_validated_node_ids =
            crypto.verify_signatures(&public_keys, &node_info_bytes, &self.signatures)?;
        let Some(validated_node_ids) = opt_validated_node_ids else {
            apibail_generic!("verification error in direct node info");
        };
        if validated_node_ids.is_empty() {
            apibail_generic!("no valid node ids in direct node info");
        }

        Ok(validated_node_ids.into())
    }

    pub fn make_signatures(
        crypto: &Crypto,
        typed_key_pairs: Vec<TypedKeyPair>,
        node_info: NodeInfo,
    ) -> VeilidAPIResult<Self> {
        let timestamp = Timestamp::now();
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
    ) -> VeilidAPIResult<Vec<u8>> {
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
            timestamp: Timestamp::now(),
            signatures: Vec::new(),
        }
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
    pub fn signatures(&self) -> &[TypedSignature] {
        &self.signatures
    }

    /// Compare this SignedDirectNodeInfo to another one
    /// Exclude the signature and timestamp and any other fields that are not
    /// semantically valuable
    pub fn equivalent(&self, other: &SignedDirectNodeInfo) -> bool {
        let a = self.node_info();
        let b = other.node_info();
        a == b
    }
}
