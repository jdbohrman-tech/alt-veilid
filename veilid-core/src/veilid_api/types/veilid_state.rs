use super::*;

/// Attachment abstraction for network 'signal strength'.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(
    target_arch = "wasm32",
    derive(Tsify),
    tsify(namespace, from_wasm_abi, into_wasm_abi)
)]
pub enum AttachmentState {
    Detached = 0,
    Attaching = 1,
    AttachedWeak = 2,
    AttachedGood = 3,
    AttachedStrong = 4,
    FullyAttached = 5,
    OverAttached = 6,
    Detaching = 7,
}
impl AttachmentState {
    pub fn is_detached(&self) -> bool {
        matches!(self, Self::Detached)
    }
    pub fn is_attached(&self) -> bool {
        matches!(
            self,
            Self::AttachedWeak
                | Self::AttachedGood
                | Self::AttachedStrong
                | Self::FullyAttached
                | Self::OverAttached
        )
    }
}

impl fmt::Display for AttachmentState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let out = match self {
            AttachmentState::Attaching => "attaching".to_owned(),
            AttachmentState::AttachedWeak => "attached_weak".to_owned(),
            AttachmentState::AttachedGood => "attached_good".to_owned(),
            AttachmentState::AttachedStrong => "attached_strong".to_owned(),
            AttachmentState::FullyAttached => "fully_attached".to_owned(),
            AttachmentState::OverAttached => "over_attached".to_owned(),
            AttachmentState::Detaching => "detaching".to_owned(),
            AttachmentState::Detached => "detached".to_owned(),
        };
        write!(f, "{}", out)
    }
}

impl TryFrom<String> for AttachmentState {
    type Error = ();

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Ok(match s.as_str() {
            "attaching" => AttachmentState::Attaching,
            "attached_weak" => AttachmentState::AttachedWeak,
            "attached_good" => AttachmentState::AttachedGood,
            "attached_strong" => AttachmentState::AttachedStrong,
            "fully_attached" => AttachmentState::FullyAttached,
            "over_attached" => AttachmentState::OverAttached,
            "detaching" => AttachmentState::Detaching,
            "detached" => AttachmentState::Detached,
            _ => return Err(()),
        })
    }
}

/// Describe the attachment state of the Veilid node
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
pub struct VeilidStateAttachment {
    /// The overall quality of the routing table if attached, or the current state the attachment state machine.
    pub state: AttachmentState,
    /// If attached and there are enough eachable nodes in the routing table to perform all the actions of the PublicInternet RoutingDomain,
    /// including things like private/safety route allocation and DHT operations.
    pub public_internet_ready: bool,
    /// If attached and there are enough eachable nodes in the routing table to perform all the actions of the LocalNetwork RoutingDomain.
    pub local_network_ready: bool,
}

/// Describe a recently accessed peer
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
pub struct PeerTableData {
    /// The node ids used by this peer
    #[schemars(with = "Vec<String>")]
    #[cfg_attr(target_arch = "wasm32", tsify(type = "string[]"))]
    pub node_ids: Vec<TypedKey>,
    /// The peer's human readable address.
    pub peer_address: String,
    /// Statistics we have collected on this peer.
    pub peer_stats: PeerStats,
}

/// Describe the current network state of the Veilid node
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
pub struct VeilidStateNetwork {
    /// If the network has been started or not.
    pub started: bool,
    /// The total number of bytes per second used by Veilid currently in the download direction.
    pub bps_down: ByteCount,
    /// The total number of bytes per second used by Veilid currently in the upload direction.
    pub bps_up: ByteCount,
    /// The list of most recently accessed peers.
    /// This is not an active connection table, nor is representative of the entire routing table.
    pub peers: Vec<PeerTableData>,
}

/// Describe a private route change that has happened
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
pub struct VeilidRouteChange {
    /// If a private route that was allocated has died, it is listed here.
    #[schemars(with = "Vec<String>")]
    pub dead_routes: Vec<RouteId>,
    /// If a private route that was imported has died, it is listed here.
    #[schemars(with = "Vec<String>")]
    pub dead_remote_routes: Vec<RouteId>,
}

/// Describe changes to the Veilid node configuration
/// Currently this is only ever emitted once, however we reserve the right to
/// add the ability to change the configuration or have it changed by the Veilid node
/// itself during runtime.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
pub struct VeilidStateConfig {
    /// If the Veilid node configuration has changed the full new config will be here.
    pub config: VeilidConfigInner,
}

/// Describe when DHT records have subkey values changed
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
pub struct VeilidValueChange {
    /// The DHT Record key that changed
    #[schemars(with = "String")]
    pub key: TypedKey,
    /// The portion of the DHT Record's subkeys that have changed
    /// If the subkey range is empty, any watch present on the value has died.
    pub subkeys: ValueSubkeyRangeSet,
    /// The count remaining on the watch that triggered this value change
    /// If there is no watch and this is received, it will be set to u32::MAX
    /// If this value is zero, any watch present on the value has died.
    pub count: u32,
    /// The (optional) value data for the first subkey in the subkeys range
    /// If 'subkeys' is not a single value, other values than the first value
    /// must be retrieved with RoutingContext::get_dht_value().
    pub value: Option<ValueData>,
}

/// An update from the veilid-core to the host application describing a change
/// to the internal state of the Veilid node.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify), tsify(into_wasm_abi))]
#[serde(tag = "kind")]
pub enum VeilidUpdate {
    Log(Box<VeilidLog>),
    AppMessage(Box<VeilidAppMessage>),
    AppCall(Box<VeilidAppCall>),
    Attachment(Box<VeilidStateAttachment>),
    Network(Box<VeilidStateNetwork>),
    Config(Box<VeilidStateConfig>),
    RouteChange(Box<VeilidRouteChange>),
    ValueChange(Box<VeilidValueChange>),
    Shutdown,
}
from_impl_to_jsvalue!(VeilidUpdate);

/// A queriable state of the internals of veilid-core.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify), tsify(into_wasm_abi))]
pub struct VeilidState {
    pub attachment: Box<VeilidStateAttachment>,
    pub network: Box<VeilidStateNetwork>,
    pub config: Box<VeilidStateConfig>,
}
from_impl_to_jsvalue!(VeilidState);
