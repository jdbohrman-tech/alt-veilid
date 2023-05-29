use super::*;

#[derive(Clone, Debug, Serialize, Deserialize, RkyvArchive, RkyvSerialize, RkyvDeserialize)]
#[archive_attr(repr(u8), derive(CheckBytes))]
pub enum SignedNodeInfo {
    Direct(SignedDirectNodeInfo),
    Relayed(SignedRelayedNodeInfo),
}

impl SignedNodeInfo {
    pub fn validate(&self, node_ids: &TypedKeySet, crypto: Crypto) -> VeilidAPIResult<TypedKeySet> {
        match self {
            SignedNodeInfo::Direct(d) => d.validate(node_ids, crypto),
            SignedNodeInfo::Relayed(r) => r.validate(node_ids, crypto),
        }
    }

    pub fn has_any_signature(&self) -> bool {
        match self {
            SignedNodeInfo::Direct(d) => d.has_any_signature(),
            SignedNodeInfo::Relayed(r) => r.has_any_signature(),
        }
    }

    pub fn timestamp(&self) -> Timestamp {
        match self {
            SignedNodeInfo::Direct(d) => d.timestamp(),
            SignedNodeInfo::Relayed(r) => r.timestamp(),
        }
    }
    pub fn node_info(&self) -> &NodeInfo {
        match self {
            SignedNodeInfo::Direct(d) => &d.node_info(),
            SignedNodeInfo::Relayed(r) => &r.node_info(),
        }
    }
    pub fn relay_ids(&self) -> TypedKeySet {
        match self {
            SignedNodeInfo::Direct(_) => TypedKeySet::new(),
            SignedNodeInfo::Relayed(r) => r.relay_ids().clone(),
        }
    }
    pub fn relay_info(&self) -> Option<&NodeInfo> {
        match self {
            SignedNodeInfo::Direct(_) => None,
            SignedNodeInfo::Relayed(r) => Some(r.relay_info().node_info()),
        }
    }
    pub fn relay_peer_info(&self) -> Option<PeerInfo> {
        match self {
            SignedNodeInfo::Direct(_) => None,
            SignedNodeInfo::Relayed(r) => Some(PeerInfo::new(
                r.relay_ids().clone(),
                SignedNodeInfo::Direct(r.relay_info().clone()),
            )),
        }
    }
    pub fn has_any_dial_info(&self) -> bool {
        self.node_info().has_dial_info()
            || self
                .relay_info()
                .map(|relay_ni| relay_ni.has_dial_info())
                .unwrap_or_default()
    }

    pub fn has_sequencing_matched_dial_info(&self, sequencing: Sequencing) -> bool {
        // Check our dial info
        for did in self.node_info().dial_info_detail_list() {
            match sequencing {
                Sequencing::NoPreference | Sequencing::PreferOrdered => return true,
                Sequencing::EnsureOrdered => {
                    if did.dial_info.protocol_type().is_connection_oriented() {
                        return true;
                    }
                }
            }
        }
        // Check our relay if we have one
        return self
            .relay_info()
            .map(|relay_ni| {
                for did in relay_ni.dial_info_detail_list() {
                    match sequencing {
                        Sequencing::NoPreference | Sequencing::PreferOrdered => return true,
                        Sequencing::EnsureOrdered => {
                            if did.dial_info.protocol_type().is_connection_oriented() {
                                return true;
                            }
                        }
                    }
                }
                false
            })
            .unwrap_or_default();
    }
}
