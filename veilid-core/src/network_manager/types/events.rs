use super::*;

pub(crate) struct PeerInfoChangeEvent {
    pub peer_info: Arc<PeerInfo>,
}

pub(crate) struct SocketAddressChangeEvent {
    pub routing_domain: RoutingDomain, // the routing domain this flow is over
    pub socket_address: SocketAddress, // the socket address as seen by the remote peer
    pub old_socket_address: Option<SocketAddress>, // the socket address previously for this peer
    pub flow: Flow,                    // the flow used
    pub reporting_peer: NodeRef,       // the peer's noderef reporting the socket address
}
