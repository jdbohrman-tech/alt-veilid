use super::*;

/// RoutingDomain-specific status for each node
/// is returned by the StatusA call

/// PublicInternet RoutingDomain Status
#[derive(
    Clone, Debug, Default, Serialize, Deserialize, RkyvArchive, RkyvSerialize, RkyvDeserialize,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct PublicInternetNodeStatus {
    pub will_route: bool,
    pub will_tunnel: bool,
    pub will_signal: bool,
    pub will_relay: bool,
    pub will_validate_dial_info: bool,
}

#[derive(
    Clone, Debug, Default, Serialize, Deserialize, RkyvArchive, RkyvSerialize, RkyvDeserialize,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct LocalNetworkNodeStatus {
    pub will_relay: bool,
    pub will_validate_dial_info: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, RkyvArchive, RkyvSerialize, RkyvDeserialize)]
#[archive_attr(repr(u8), derive(CheckBytes))]
pub enum NodeStatus {
    PublicInternet(PublicInternetNodeStatus),
    LocalNetwork(LocalNetworkNodeStatus),
}

impl NodeStatus {
    pub fn will_route(&self) -> bool {
        match self {
            NodeStatus::PublicInternet(pi) => pi.will_route,
            NodeStatus::LocalNetwork(_) => false,
        }
    }
    pub fn will_tunnel(&self) -> bool {
        match self {
            NodeStatus::PublicInternet(pi) => pi.will_tunnel,
            NodeStatus::LocalNetwork(_) => false,
        }
    }
    pub fn will_signal(&self) -> bool {
        match self {
            NodeStatus::PublicInternet(pi) => pi.will_signal,
            NodeStatus::LocalNetwork(_) => false,
        }
    }
    pub fn will_relay(&self) -> bool {
        match self {
            NodeStatus::PublicInternet(pi) => pi.will_relay,
            NodeStatus::LocalNetwork(ln) => ln.will_relay,
        }
    }
    pub fn will_validate_dial_info(&self) -> bool {
        match self {
            NodeStatus::PublicInternet(pi) => pi.will_validate_dial_info,
            NodeStatus::LocalNetwork(ln) => ln.will_validate_dial_info,
        }
    }
}
