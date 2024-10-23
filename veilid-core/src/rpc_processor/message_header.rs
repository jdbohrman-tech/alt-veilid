use super::*;

#[derive(Debug, Clone)]
pub(in crate::rpc_processor) struct RPCMessageHeaderDetailDirect {
    /// The decoded header of the envelope
    pub envelope: Envelope,
    /// The noderef of the peer that sent the message (not the original sender).
    /// Ensures node doesn't get evicted from routing table until we're done with it
    /// Should be filtered to the routing domain of the peer that we received from
    pub peer_noderef: FilteredNodeRef,
    /// The flow from the peer sent the message (not the original sender)
    pub flow: Flow,
    /// The routing domain of the peer that we received from
    pub routing_domain: RoutingDomain,
}

/// Header details for rpc messages received over only a safety route but not a private route
#[derive(Debug, Clone)]
pub(in crate::rpc_processor) struct RPCMessageHeaderDetailSafetyRouted {
    /// Direct header
    pub direct: RPCMessageHeaderDetailDirect,
    /// Remote safety route used
    pub remote_safety_route: PublicKey,
    /// The sequencing used for this route
    pub sequencing: Sequencing,
}

/// Header details for rpc messages received over a private route
#[derive(Debug, Clone)]
pub(in crate::rpc_processor) struct RPCMessageHeaderDetailPrivateRouted {
    /// Direct header
    pub direct: RPCMessageHeaderDetailDirect,
    /// Remote safety route used (or possibly node id the case of no safety route)
    pub remote_safety_route: PublicKey,
    /// The private route we received the rpc over
    pub private_route: PublicKey,
    // The safety spec for replying to this private routed rpc
    pub safety_spec: SafetySpec,
}

#[derive(Debug, Clone)]
pub(in crate::rpc_processor) enum RPCMessageHeaderDetail {
    Direct(RPCMessageHeaderDetailDirect),
    SafetyRouted(RPCMessageHeaderDetailSafetyRouted),
    PrivateRouted(RPCMessageHeaderDetailPrivateRouted),
}

/// The decoded header of an RPC message
#[derive(Debug, Clone)]
pub(in crate::rpc_processor) struct MessageHeader {
    /// Time the message was received, not sent
    pub timestamp: Timestamp,
    /// The length in bytes of the rpc message body
    pub body_len: ByteCount,
    /// The header detail depending on which way the message was received
    pub detail: RPCMessageHeaderDetail,
}

impl MessageHeader {
    /// The crypto kind used on the RPC
    pub fn crypto_kind(&self) -> CryptoKind {
        match &self.detail {
            RPCMessageHeaderDetail::Direct(d) => d.envelope.get_crypto_kind(),
            RPCMessageHeaderDetail::SafetyRouted(s) => s.direct.envelope.get_crypto_kind(),
            RPCMessageHeaderDetail::PrivateRouted(p) => p.direct.envelope.get_crypto_kind(),
        }
    }
    // pub fn direct_peer_noderef(&self) -> NodeRef {
    //     match &self.detail {
    //         RPCMessageHeaderDetail::Direct(d) => d.peer_noderef.clone(),
    //         RPCMessageHeaderDetail::SafetyRouted(s) => s.direct.peer_noderef.clone(),
    //         RPCMessageHeaderDetail::PrivateRouted(p) => p.direct.peer_noderef.clone(),
    //     }
    // }
    pub fn routing_domain(&self) -> RoutingDomain {
        match &self.detail {
            RPCMessageHeaderDetail::Direct(d) => d.routing_domain,
            RPCMessageHeaderDetail::SafetyRouted(s) => s.direct.routing_domain,
            RPCMessageHeaderDetail::PrivateRouted(p) => p.direct.routing_domain,
        }
    }
    pub fn direct_sender_node_id(&self) -> TypedKey {
        match &self.detail {
            RPCMessageHeaderDetail::Direct(d) => d.envelope.get_sender_typed_id(),
            RPCMessageHeaderDetail::SafetyRouted(s) => s.direct.envelope.get_sender_typed_id(),
            RPCMessageHeaderDetail::PrivateRouted(p) => p.direct.envelope.get_sender_typed_id(),
        }
    }
}
