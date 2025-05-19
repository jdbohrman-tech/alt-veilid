use super::*;

pub struct PeerInfoChangeEvent {
    pub routing_domain: RoutingDomain,
    pub opt_old_peer_info: Option<Arc<PeerInfo>>,
    pub opt_new_peer_info: Option<Arc<PeerInfo>>,
}
