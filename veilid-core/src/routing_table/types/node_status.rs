use super::*;

/// RoutingDomain-specific status for each node
/// is returned by the StatusA call

pub type Capability = FourCC;
pub const CAP_WILL_ROUTE: Capability = FourCC(*b"ROUT");
pub const CAP_WILL_TUNNEL: Capability = FourCC(*b"TUNL");
pub const CAP_WILL_SIGNAL: Capability = FourCC(*b"SGNL");
pub const CAP_WILL_RELAY: Capability = FourCC(*b"RLAY");
pub const CAP_WILL_VALIDATE_DIAL_INFO: Capability = FourCC(*b"DIAL");
pub const CAP_WILL_DHT: Capability = FourCC(*b"DHTV");
pub const CAP_WILL_APPMESSAGE: Capability = FourCC(*b"APPM");
pub const MAX_CAPABILITIES: usize = 64;

/// PublicInternet RoutingDomain Status
#[derive(
    Clone, Debug, Default, Serialize, Deserialize, RkyvArchive, RkyvSerialize, RkyvDeserialize,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct PublicInternetNodeStatus {
    pub capabilities: Vec<Capability>,
}

#[derive(
    Clone, Debug, Default, Serialize, Deserialize, RkyvArchive, RkyvSerialize, RkyvDeserialize,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct LocalNetworkNodeStatus {
    pub capabilities: Vec<Capability>,
}

#[derive(Clone, Debug, Serialize, Deserialize, RkyvArchive, RkyvSerialize, RkyvDeserialize)]
#[archive_attr(repr(u8), derive(CheckBytes))]
pub enum NodeStatus {
    PublicInternet(PublicInternetNodeStatus),
    LocalNetwork(LocalNetworkNodeStatus),
}

impl NodeStatus {
    pub fn has_capability(&self, cap: Capability) -> bool {
        match self {
            NodeStatus::PublicInternet(pi) => pi.capabilities.contains(&cap),
            NodeStatus::LocalNetwork(ln) => ln.capabilities.contains(&cap),
        }
    }
}
