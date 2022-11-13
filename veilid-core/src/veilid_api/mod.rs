#![allow(dead_code)]

mod debug;
mod privacy;
mod routing_context;
mod serialize_helpers;

pub use debug::*;
pub use privacy::*;
pub use routing_context::*;
pub use serialize_helpers::*;

use crate::*;

pub use crate::xx::{
    IpAddr, Ipv4Addr, Ipv6Addr, SendPinBoxFuture, SocketAddr, SocketAddrV4, SocketAddrV6,
    ToSocketAddrs,
};
pub use alloc::string::ToString;
pub use attachment_manager::AttachmentManager;
pub use core::str::FromStr;
pub use crypto::Crypto;
pub use crypto::{generate_secret, sign, verify, DHTKey, DHTKeySecret, DHTSignature, Nonce};
pub use intf::BlockStore;
pub use intf::ProtectedStore;
pub use intf::TableStore;
pub use network_manager::NetworkManager;
pub use routing_table::{NodeRef, NodeRefBase, RoutingTable};

use core::fmt;
use core_context::{api_shutdown, VeilidCoreContext};
use enumset::*;
use rkyv::{Archive as RkyvArchive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use rpc_processor::*;
use serde::*;
use xx::*;

/////////////////////////////////////////////////////////////////////////////////////////////////////

#[allow(unused_macros)]
#[macro_export]
macro_rules! apibail_generic {
    ($x:expr) => {
        return Err(VeilidAPIError::generic($x))
    };
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! apibail_internal {
    ($x:expr) => {
        return Err(VeilidAPIError::internal($x))
    };
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! apibail_parse {
    ($x:expr, $y:expr) => {
        return Err(VeilidAPIError::parse_error($x, $y))
    };
}

#[derive(
    ThisError,
    Clone,
    Debug,
    PartialOrd,
    PartialEq,
    Eq,
    Ord,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive_attr(repr(u8), derive(CheckBytes))]
#[serde(tag = "kind")]
pub enum VeilidAPIError {
    #[error("Not initialized")]
    NotInitialized,
    #[error("Already initialized")]
    AlreadyInitialized,
    #[error("Timeout")]
    Timeout,
    #[error("Shutdown")]
    Shutdown,
    #[error("Node not found: {node_id}")]
    NodeNotFound { node_id: NodeId },
    #[error("No connection: {message}")]
    NoConnection { message: String },
    #[error("No peer info: {node_id}")]
    NoPeerInfo { node_id: NodeId },
    #[error("Internal: {message}")]
    Internal { message: String },
    #[error("Unimplemented: {message}")]
    Unimplemented { message: String },
    #[error("Parse error: '{message}' with value '{value}'")]
    ParseError { message: String, value: String },
    #[error("Invalid argument: '{argument}' for '{context}' with value '{value}'")]
    InvalidArgument {
        context: String,
        argument: String,
        value: String,
    },
    #[error("Missing argument: '{argument}' for '{context}'")]
    MissingArgument { context: String, argument: String },
    #[error("Generic: {message}")]
    Generic { message: String },
}

impl VeilidAPIError {
    pub fn not_initialized() -> Self {
        Self::NotInitialized
    }
    pub fn already_initialized() -> Self {
        Self::AlreadyInitialized
    }
    pub fn timeout() -> Self {
        Self::Timeout
    }
    pub fn shutdown() -> Self {
        Self::Shutdown
    }
    pub fn node_not_found(node_id: NodeId) -> Self {
        Self::NodeNotFound { node_id }
    }
    pub fn no_connection(message: String) -> Self {
        Self::NoConnection { message }
    }
    pub fn no_peer_info(node_id: NodeId) -> Self {
        Self::NoPeerInfo { node_id }
    }
    pub fn internal<T: ToString>(msg: T) -> Self {
        Self::Internal {
            message: msg.to_string(),
        }
    }
    pub fn unimplemented<T: ToString>(msg: T) -> Self {
        Self::Unimplemented {
            message: msg.to_string(),
        }
    }
    pub fn parse_error<T: ToString, S: ToString>(msg: T, value: S) -> Self {
        Self::ParseError {
            message: msg.to_string(),
            value: value.to_string(),
        }
    }
    pub fn invalid_argument<T: ToString, S: ToString, R: ToString>(
        context: T,
        argument: S,
        value: R,
    ) -> Self {
        Self::InvalidArgument {
            context: context.to_string(),
            argument: argument.to_string(),
            value: value.to_string(),
        }
    }
    pub fn missing_argument<T: ToString, S: ToString>(context: T, argument: S) -> Self {
        Self::MissingArgument {
            context: context.to_string(),
            argument: argument.to_string(),
        }
    }
    pub fn generic<T: ToString>(msg: T) -> Self {
        Self::Generic {
            message: msg.to_string(),
        }
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Copy,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive_attr(repr(u8), derive(CheckBytes))]
pub enum VeilidLogLevel {
    Error = 1,
    Warn,
    Info,
    Debug,
    Trace,
}

impl VeilidLogLevel {
    pub fn from_tracing_level(level: tracing::Level) -> VeilidLogLevel {
        match level {
            tracing::Level::ERROR => VeilidLogLevel::Error,
            tracing::Level::WARN => VeilidLogLevel::Warn,
            tracing::Level::INFO => VeilidLogLevel::Info,
            tracing::Level::DEBUG => VeilidLogLevel::Debug,
            tracing::Level::TRACE => VeilidLogLevel::Trace,
        }
    }
    pub fn from_log_level(level: log::Level) -> VeilidLogLevel {
        match level {
            log::Level::Error => VeilidLogLevel::Error,
            log::Level::Warn => VeilidLogLevel::Warn,
            log::Level::Info => VeilidLogLevel::Info,
            log::Level::Debug => VeilidLogLevel::Debug,
            log::Level::Trace => VeilidLogLevel::Trace,
        }
    }
    pub fn to_tracing_level(&self) -> tracing::Level {
        match self {
            Self::Error => tracing::Level::ERROR,
            Self::Warn => tracing::Level::WARN,
            Self::Info => tracing::Level::INFO,
            Self::Debug => tracing::Level::DEBUG,
            Self::Trace => tracing::Level::TRACE,
        }
    }
    pub fn to_log_level(&self) -> log::Level {
        match self {
            Self::Error => log::Level::Error,
            Self::Warn => log::Level::Warn,
            Self::Info => log::Level::Info,
            Self::Debug => log::Level::Debug,
            Self::Trace => log::Level::Trace,
        }
    }
}

impl fmt::Display for VeilidLogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let text = match self {
            Self::Error => "ERROR",
            Self::Warn => "WARN",
            Self::Info => "INFO",
            Self::Debug => "DEBUG",
            Self::Trace => "TRACE",
        };
        write!(f, "{}", text)
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, RkyvArchive, RkyvSerialize, RkyvDeserialize,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct VeilidLog {
    pub log_level: VeilidLogLevel,
    pub message: String,
    pub backtrace: Option<String>,
}

#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, RkyvArchive, RkyvSerialize, RkyvDeserialize,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct VeilidAppMessage {
    /// Some(sender) if the message was sent directly, None if received via a private/safety route
    #[serde(with = "opt_json_as_string")]
    pub sender: Option<NodeId>,
    /// The content of the message to deliver to the application
    #[serde(with = "json_as_base64")]
    pub message: Vec<u8>,
}

#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, RkyvArchive, RkyvSerialize, RkyvDeserialize,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct VeilidAppCall {
    /// Some(sender) if the request was sent directly, None if received via a private/safety route
    #[serde(with = "opt_json_as_string")]
    pub sender: Option<NodeId>,
    /// The content of the request to deliver to the application
    #[serde(with = "json_as_base64")]
    pub message: Vec<u8>,
    /// The id to reply to
    #[serde(with = "json_as_string")]
    pub id: u64,
}

#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, RkyvArchive, RkyvSerialize, RkyvDeserialize,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct VeilidStateAttachment {
    pub state: AttachmentState,
}

#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, RkyvArchive, RkyvSerialize, RkyvDeserialize,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct PeerTableData {
    pub node_id: DHTKey,
    pub peer_address: PeerAddress,
    pub peer_stats: PeerStats,
}

#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, RkyvArchive, RkyvSerialize, RkyvDeserialize,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct VeilidStateNetwork {
    pub started: bool,
    #[serde(with = "json_as_string")]
    pub bps_down: u64,
    #[serde(with = "json_as_string")]
    pub bps_up: u64,
    pub peers: Vec<PeerTableData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, RkyvArchive, RkyvSerialize, RkyvDeserialize)]
#[archive_attr(repr(u8), derive(CheckBytes))]
#[serde(tag = "kind")]
pub enum VeilidUpdate {
    Log(VeilidLog),
    AppMessage(VeilidAppMessage),
    AppCall(VeilidAppCall),
    Attachment(VeilidStateAttachment),
    Network(VeilidStateNetwork),
    Shutdown,
}

#[derive(Debug, Clone, Serialize, Deserialize, RkyvArchive, RkyvSerialize, RkyvDeserialize)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct VeilidState {
    pub attachment: VeilidStateAttachment,
    pub network: VeilidStateNetwork,
}

/////////////////////////////////////////////////////////////////////////////////////////////////////
///
#[derive(
    Clone,
    Debug,
    Default,
    PartialOrd,
    PartialEq,
    Eq,
    Ord,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct NodeId {
    pub key: DHTKey,
}
impl NodeId {
    pub fn new(key: DHTKey) -> Self {
        Self { key }
    }
}
impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.key.encode())
    }
}
impl FromStr for NodeId {
    type Err = VeilidAPIError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            key: DHTKey::try_decode(s)?,
        })
    }
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialOrd,
    PartialEq,
    Eq,
    Ord,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct ValueKey {
    pub key: DHTKey,
    pub subkey: Option<String>,
}
impl ValueKey {
    pub fn new(key: DHTKey) -> Self {
        Self { key, subkey: None }
    }
    pub fn new_subkey(key: DHTKey, subkey: String) -> Self {
        Self {
            key,
            subkey: if subkey.is_empty() {
                None
            } else {
                Some(subkey)
            },
        }
    }
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialOrd,
    PartialEq,
    Eq,
    Ord,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct ValueData {
    pub data: Vec<u8>,
    pub seq: u32,
}
impl ValueData {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data, seq: 0 }
    }
    pub fn new_with_seq(data: Vec<u8>, seq: u32) -> Self {
        Self { data, seq }
    }
    pub fn change(&mut self, data: Vec<u8>) {
        self.data = data;
        self.seq += 1;
    }
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialOrd,
    PartialEq,
    Eq,
    Ord,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct BlockId {
    pub key: DHTKey,
}
impl BlockId {
    pub fn new(key: DHTKey) -> Self {
        Self { key }
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////

// Keep member order appropriate for sorting < preference
#[derive(
    Copy,
    Clone,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive_attr(repr(u8), derive(CheckBytes))]
pub enum DialInfoClass {
    Direct = 0, // D = Directly reachable with public IP and no firewall, with statically configured port
    Mapped = 1, // M = Directly reachable with via portmap behind any NAT or firewalled with dynamically negotiated port
    FullConeNAT = 2, // F = Directly reachable device without portmap behind full-cone NAT
    Blocked = 3, // B = Inbound blocked at firewall but may hole punch with public address
    AddressRestrictedNAT = 4, // A = Device without portmap behind address-only restricted NAT
    PortRestrictedNAT = 5, // P = Device without portmap behind address-and-port restricted NAT
}

impl DialInfoClass {
    // Is a signal required to do an inbound hole-punch?
    pub fn requires_signal(&self) -> bool {
        matches!(
            self,
            Self::Blocked | Self::AddressRestrictedNAT | Self::PortRestrictedNAT
        )
    }

    // Does a relay node need to be allocated for this dial info?
    // For full cone NAT, the relay itself may not be used but the keepalive sent to it
    // is required to keep the NAT mapping valid in the router state table
    pub fn requires_relay(&self) -> bool {
        matches!(
            self,
            Self::FullConeNAT
                | Self::Blocked
                | Self::AddressRestrictedNAT
                | Self::PortRestrictedNAT
        )
    }
}

// Ordering here matters, >= is used to check strength of sequencing requirement
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive_attr(repr(u8), derive(CheckBytes))]
pub enum Sequencing {
    NoPreference,
    PreferOrdered,
    EnsureOrdered,
}

// Ordering here matters, >= is used to check strength of stability requirement
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive_attr(repr(u8), derive(CheckBytes))]
pub enum Stability {
    LowLatency,
    Reliable,
}

/// The choice of safety route to include in compiled routes
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive_attr(repr(u8), derive(CheckBytes))]
pub enum SafetySelection {
    /// Don't use a safety route, only specify the sequencing preference
    Unsafe(Sequencing),
    /// Use a safety route and parameters specified by a SafetySpec
    Safe(SafetySpec),
}

/// Options for safety routes (sender privacy)
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct SafetySpec {
    /// preferred safety route if it still exists
    pub preferred_route: Option<DHTKey>,
    /// 0 = no safety route, just use node's node id, more hops is safer but slower
    pub hop_count: usize,
    /// prefer reliability over speed
    pub stability: Stability,
    /// prefer connection-oriented sequenced protocols
    pub sequencing: Sequencing,
}

// Keep member order appropriate for sorting < preference
#[derive(
    Debug,
    Clone,
    PartialEq,
    PartialOrd,
    Ord,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct DialInfoDetail {
    pub class: DialInfoClass,
    pub dial_info: DialInfo,
}

impl MatchesDialInfoFilter for DialInfoDetail {
    fn matches_filter(&self, filter: &DialInfoFilter) -> bool {
        self.dial_info.matches_filter(filter)
    }
}

impl DialInfoDetail {
    pub fn ordered_sequencing_sort(a: &DialInfoDetail, b: &DialInfoDetail) -> core::cmp::Ordering {
        if a.class < b.class {
            return core::cmp::Ordering::Less;
        }
        if a.class > b.class {
            return core::cmp::Ordering::Greater;
        }
        DialInfo::ordered_sequencing_sort(&a.dial_info, &b.dial_info)
    }
    pub const NO_SORT: std::option::Option<
        for<'r, 's> fn(
            &'r veilid_api::DialInfoDetail,
            &'s veilid_api::DialInfoDetail,
        ) -> std::cmp::Ordering,
    > = None::<fn(&DialInfoDetail, &DialInfoDetail) -> core::cmp::Ordering>;
}

#[derive(
    Copy,
    Clone,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive_attr(repr(u8), derive(CheckBytes))]
pub enum NetworkClass {
    InboundCapable = 0, // I = Inbound capable without relay, may require signal
    OutboundOnly = 1, // O = Outbound only, inbound relay required except with reverse connect signal
    WebApp = 2,       // W = PWA, outbound relay is required in most cases
    Invalid = 3,      // X = Invalid network class, we don't know how to reach this node
}

impl Default for NetworkClass {
    fn default() -> Self {
        Self::Invalid
    }
}

impl NetworkClass {
    // Should an outbound relay be kept available?
    pub fn outbound_wants_relay(&self) -> bool {
        matches!(self, Self::WebApp)
    }
}

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

#[derive(Clone, Debug, Serialize, Deserialize, RkyvArchive, RkyvSerialize, RkyvDeserialize)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct NodeInfo {
    pub network_class: NetworkClass,
    #[with(RkyvEnumSet)]
    pub outbound_protocols: ProtocolTypeSet,
    #[with(RkyvEnumSet)]
    pub address_types: AddressTypeSet,
    pub min_version: u8,
    pub max_version: u8,
    pub dial_info_detail_list: Vec<DialInfoDetail>,
}

impl NodeInfo {
    pub fn first_filtered_dial_info_detail<S, F>(
        &self,
        sort: Option<S>,
        filter: F,
    ) -> Option<DialInfoDetail>
    where
        S: Fn(&DialInfoDetail, &DialInfoDetail) -> std::cmp::Ordering,
        F: Fn(&DialInfoDetail) -> bool,
    {
        if let Some(sort) = sort {
            let mut dids = self.dial_info_detail_list.clone();
            dids.sort_by(sort);
            for did in dids {
                if filter(&did) {
                    return Some(did);
                }
            }
        } else {
            for did in &self.dial_info_detail_list {
                if filter(did) {
                    return Some(did.clone());
                }
            }
        };
        None
    }

    pub fn all_filtered_dial_info_details<S, F>(
        &self,
        sort: Option<S>,
        filter: F,
    ) -> Vec<DialInfoDetail>
    where
        S: Fn(&DialInfoDetail, &DialInfoDetail) -> std::cmp::Ordering,
        F: Fn(&DialInfoDetail) -> bool,
    {
        let mut dial_info_detail_list = Vec::new();

        if let Some(sort) = sort {
            let mut dids = self.dial_info_detail_list.clone();
            dids.sort_by(sort);
            for did in dids {
                if filter(&did) {
                    dial_info_detail_list.push(did);
                }
            }
        } else {
            for did in &self.dial_info_detail_list {
                if filter(did) {
                    dial_info_detail_list.push(did.clone());
                }
            }
        };
        dial_info_detail_list
    }

    /// Does this node has some dial info
    pub fn has_dial_info(&self) -> bool {
        !self.dial_info_detail_list.is_empty()
    }

    /// Is some relay required either for signal or inbound relay or outbound relay?
    pub fn requires_relay(&self) -> bool {
        match self.network_class {
            NetworkClass::InboundCapable => {
                for did in &self.dial_info_detail_list {
                    if did.class.requires_relay() {
                        return true;
                    }
                }
            }
            NetworkClass::OutboundOnly => {
                return true;
            }
            NetworkClass::WebApp => {
                return true;
            }
            NetworkClass::Invalid => {}
        }
        false
    }

    /// Can this node assist with signalling? Yes but only if it doesn't require signalling, itself.
    pub fn can_signal(&self) -> bool {
        // Must be inbound capable
        if !matches!(self.network_class, NetworkClass::InboundCapable) {
            return false;
        }
        // Do any of our dial info require signalling? if so, we can't offer signalling
        for did in &self.dial_info_detail_list {
            if did.class.requires_signal() {
                return false;
            }
        }
        true
    }

    /// Can this node relay be an inbound relay?
    pub fn can_inbound_relay(&self) -> bool {
        // For now this is the same
        self.can_signal()
    }

    /// Is this node capable of validating dial info
    pub fn can_validate_dial_info(&self) -> bool {
        // For now this is the same
        self.can_signal()
    }
}

#[allow(clippy::derive_hash_xor_eq)]
#[derive(
    Debug,
    PartialOrd,
    Ord,
    Hash,
    EnumSetType,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[enumset(repr = "u8")]
#[archive_attr(repr(u8), derive(CheckBytes))]
pub enum Direction {
    Inbound,
    Outbound,
}
pub type DirectionSet = EnumSet<Direction>;

// Keep member order appropriate for sorting < preference
// Must match DialInfo order
#[allow(clippy::derive_hash_xor_eq)]
#[derive(
    Debug,
    PartialOrd,
    Ord,
    Hash,
    EnumSetType,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[enumset(repr = "u8")]
#[archive_attr(repr(u8), derive(CheckBytes))]
pub enum LowLevelProtocolType {
    UDP,
    TCP,
}

impl LowLevelProtocolType {
    pub fn is_connection_oriented(&self) -> bool {
        matches!(self, LowLevelProtocolType::TCP)
    }
}
pub type LowLevelProtocolTypeSet = EnumSet<LowLevelProtocolType>;

// Keep member order appropriate for sorting < preference
// Must match DialInfo order
#[allow(clippy::derive_hash_xor_eq)]
#[derive(
    Debug,
    PartialOrd,
    Ord,
    Hash,
    EnumSetType,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[enumset(repr = "u8")]
#[archive_attr(repr(u8), derive(CheckBytes))]
pub enum ProtocolType {
    UDP,
    TCP,
    WS,
    WSS,
}

impl ProtocolType {
    pub fn is_connection_oriented(&self) -> bool {
        matches!(
            self,
            ProtocolType::TCP | ProtocolType::WS | ProtocolType::WSS
        )
    }
    pub fn low_level_protocol_type(&self) -> LowLevelProtocolType {
        match self {
            ProtocolType::UDP => LowLevelProtocolType::UDP,
            ProtocolType::TCP | ProtocolType::WS | ProtocolType::WSS => LowLevelProtocolType::TCP,
        }
    }
    pub fn sort_order(&self, sequencing: Sequencing) -> usize {
        match self {
            ProtocolType::UDP => {
                if sequencing != Sequencing::NoPreference {
                    3
                } else {
                    0
                }
            }
            ProtocolType::TCP => {
                if sequencing != Sequencing::NoPreference {
                    0
                } else {
                    1
                }
            }
            ProtocolType::WS => {
                if sequencing != Sequencing::NoPreference {
                    1
                } else {
                    2
                }
            }
            ProtocolType::WSS => {
                if sequencing != Sequencing::NoPreference {
                    2
                } else {
                    3
                }
            }
        }
    }
    pub fn all_ordered_set() -> ProtocolTypeSet {
        ProtocolType::TCP | ProtocolType::WS | ProtocolType::WSS
    }
}

pub type ProtocolTypeSet = EnumSet<ProtocolType>;

#[allow(clippy::derive_hash_xor_eq)]
#[derive(
    Debug,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
    EnumSetType,
)]
#[enumset(repr = "u8")]
#[archive_attr(repr(u8), derive(CheckBytes))]
pub enum AddressType {
    IPV4,
    IPV6,
}
pub type AddressTypeSet = EnumSet<AddressType>;

// Routing domain here is listed in order of preference, keep in order
#[allow(clippy::derive_hash_xor_eq)]
#[derive(
    Debug,
    Ord,
    PartialOrd,
    Hash,
    EnumSetType,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[enumset(repr = "u8")]
#[archive_attr(repr(u8), derive(CheckBytes))]
pub enum RoutingDomain {
    LocalNetwork = 0,
    PublicInternet = 1,
}
impl RoutingDomain {
    pub const fn count() -> usize {
        2
    }
    pub const fn all() -> [RoutingDomain; RoutingDomain::count()] {
        // Routing domain here is listed in order of preference, keep in order
        [RoutingDomain::LocalNetwork, RoutingDomain::PublicInternet]
    }
}
pub type RoutingDomainSet = EnumSet<RoutingDomain>;

#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    PartialOrd,
    Ord,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive_attr(repr(u8), derive(CheckBytes))]
pub enum Address {
    IPV4(Ipv4Addr),
    IPV6(Ipv6Addr),
}

impl Default for Address {
    fn default() -> Self {
        Address::IPV4(Ipv4Addr::new(0, 0, 0, 0))
    }
}

impl Address {
    pub fn from_socket_addr(sa: SocketAddr) -> Address {
        match sa {
            SocketAddr::V4(v4) => Address::IPV4(*v4.ip()),
            SocketAddr::V6(v6) => Address::IPV6(*v6.ip()),
        }
    }
    pub fn from_ip_addr(addr: IpAddr) -> Address {
        match addr {
            IpAddr::V4(v4) => Address::IPV4(v4),
            IpAddr::V6(v6) => Address::IPV6(v6),
        }
    }
    pub fn address_type(&self) -> AddressType {
        match self {
            Address::IPV4(_) => AddressType::IPV4,
            Address::IPV6(_) => AddressType::IPV6,
        }
    }
    pub fn address_string(&self) -> String {
        match self {
            Address::IPV4(v4) => v4.to_string(),
            Address::IPV6(v6) => v6.to_string(),
        }
    }
    pub fn address_string_with_port(&self, port: u16) -> String {
        match self {
            Address::IPV4(v4) => format!("{}:{}", v4, port),
            Address::IPV6(v6) => format!("[{}]:{}", v6, port),
        }
    }
    pub fn is_unspecified(&self) -> bool {
        match self {
            Address::IPV4(v4) => ipv4addr_is_unspecified(v4),
            Address::IPV6(v6) => ipv6addr_is_unspecified(v6),
        }
    }
    pub fn is_global(&self) -> bool {
        match self {
            Address::IPV4(v4) => ipv4addr_is_global(v4) && !ipv4addr_is_multicast(v4),
            Address::IPV6(v6) => ipv6addr_is_unicast_global(v6),
        }
    }
    pub fn is_local(&self) -> bool {
        match self {
            Address::IPV4(v4) => {
                ipv4addr_is_private(v4)
                    || ipv4addr_is_link_local(v4)
                    || ipv4addr_is_ietf_protocol_assignment(v4)
            }
            Address::IPV6(v6) => {
                ipv6addr_is_unicast_site_local(v6)
                    || ipv6addr_is_unicast_link_local(v6)
                    || ipv6addr_is_unique_local(v6)
            }
        }
    }
    pub fn to_ip_addr(&self) -> IpAddr {
        match self {
            Self::IPV4(a) => IpAddr::V4(*a),
            Self::IPV6(a) => IpAddr::V6(*a),
        }
    }
    pub fn to_socket_addr(&self, port: u16) -> SocketAddr {
        SocketAddr::new(self.to_ip_addr(), port)
    }
    pub fn to_canonical(&self) -> Address {
        match self {
            Address::IPV4(v4) => Address::IPV4(*v4),
            Address::IPV6(v6) => match v6.to_ipv4() {
                Some(v4) => Address::IPV4(v4),
                None => Address::IPV6(*v6),
            },
        }
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Address::IPV4(v4) => write!(f, "{}", v4),
            Address::IPV6(v6) => write!(f, "{}", v6),
        }
    }
}

impl FromStr for Address {
    type Err = VeilidAPIError;
    fn from_str(host: &str) -> Result<Address, VeilidAPIError> {
        if let Ok(addr) = Ipv4Addr::from_str(host) {
            Ok(Address::IPV4(addr))
        } else if let Ok(addr) = Ipv6Addr::from_str(host) {
            Ok(Address::IPV6(addr))
        } else {
            Err(VeilidAPIError::parse_error(
                "Address::from_str failed",
                host,
            ))
        }
    }
}

#[derive(
    Copy,
    Default,
    Clone,
    Debug,
    PartialEq,
    PartialOrd,
    Ord,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct SocketAddress {
    address: Address,
    port: u16,
}

impl SocketAddress {
    pub fn new(address: Address, port: u16) -> Self {
        Self { address, port }
    }
    pub fn from_socket_addr(sa: SocketAddr) -> SocketAddress {
        Self {
            address: Address::from_socket_addr(sa),
            port: sa.port(),
        }
    }
    pub fn address(&self) -> Address {
        self.address
    }
    pub fn address_type(&self) -> AddressType {
        self.address.address_type()
    }
    pub fn port(&self) -> u16 {
        self.port
    }
    pub fn to_canonical(&self) -> SocketAddress {
        SocketAddress {
            address: self.address.to_canonical(),
            port: self.port,
        }
    }
    pub fn to_ip_addr(&self) -> IpAddr {
        self.address.to_ip_addr()
    }
    pub fn to_socket_addr(&self) -> SocketAddr {
        self.address.to_socket_addr(self.port)
    }
}

impl fmt::Display for SocketAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.to_socket_addr())
    }
}

impl FromStr for SocketAddress {
    type Err = VeilidAPIError;
    fn from_str(s: &str) -> Result<SocketAddress, VeilidAPIError> {
        let sa = SocketAddr::from_str(s)
            .map_err(|e| VeilidAPIError::parse_error("Failed to parse SocketAddress", e))?;
        Ok(SocketAddress::from_socket_addr(sa))
    }
}

//////////////////////////////////////////////////////////////////

#[derive(
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct DialInfoFilter {
    #[with(RkyvEnumSet)]
    pub protocol_type_set: ProtocolTypeSet,
    #[with(RkyvEnumSet)]
    pub address_type_set: AddressTypeSet,
}

impl Default for DialInfoFilter {
    fn default() -> Self {
        Self {
            protocol_type_set: ProtocolTypeSet::all(),
            address_type_set: AddressTypeSet::all(),
        }
    }
}

impl DialInfoFilter {
    pub fn all() -> Self {
        Self {
            protocol_type_set: ProtocolTypeSet::all(),
            address_type_set: AddressTypeSet::all(),
        }
    }
    pub fn with_protocol_type(mut self, protocol_type: ProtocolType) -> Self {
        self.protocol_type_set = ProtocolTypeSet::only(protocol_type);
        self
    }
    pub fn with_protocol_type_set(mut self, protocol_set: ProtocolTypeSet) -> Self {
        self.protocol_type_set = protocol_set;
        self
    }
    pub fn with_address_type(mut self, address_type: AddressType) -> Self {
        self.address_type_set = AddressTypeSet::only(address_type);
        self
    }
    pub fn with_address_type_set(mut self, address_set: AddressTypeSet) -> Self {
        self.address_type_set = address_set;
        self
    }
    pub fn filtered(mut self, other_dif: &DialInfoFilter) -> Self {
        self.protocol_type_set &= other_dif.protocol_type_set;
        self.address_type_set &= other_dif.address_type_set;
        self
    }
    pub fn is_dead(&self) -> bool {
        self.protocol_type_set.is_empty() || self.address_type_set.is_empty()
    }
}

impl fmt::Debug for DialInfoFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let mut out = String::new();
        if self.protocol_type_set != ProtocolTypeSet::all() {
            out += &format!("+{:?}", self.protocol_type_set);
        } else {
            out += "*";
        }
        if self.address_type_set != AddressTypeSet::all() {
            out += &format!("+{:?}", self.address_type_set);
        } else {
            out += "*";
        }
        write!(f, "[{}]", out)
    }
}

pub trait MatchesDialInfoFilter {
    fn matches_filter(&self, filter: &DialInfoFilter) -> bool;
}

#[derive(
    Clone,
    Default,
    Debug,
    PartialEq,
    PartialOrd,
    Ord,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct DialInfoUDP {
    pub socket_address: SocketAddress,
}

#[derive(
    Clone,
    Default,
    Debug,
    PartialEq,
    PartialOrd,
    Ord,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct DialInfoTCP {
    pub socket_address: SocketAddress,
}

#[derive(
    Clone,
    Default,
    Debug,
    PartialEq,
    PartialOrd,
    Ord,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct DialInfoWS {
    pub socket_address: SocketAddress,
    pub request: String,
}

#[derive(
    Clone,
    Default,
    Debug,
    PartialEq,
    PartialOrd,
    Ord,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct DialInfoWSS {
    pub socket_address: SocketAddress,
    pub request: String,
}

// Keep member order appropriate for sorting < preference
// Must match ProtocolType order
#[derive(
    Clone,
    Debug,
    PartialEq,
    PartialOrd,
    Ord,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive_attr(repr(u8), derive(CheckBytes))]
#[serde(tag = "kind")]
pub enum DialInfo {
    UDP(DialInfoUDP),
    TCP(DialInfoTCP),
    WS(DialInfoWS),
    WSS(DialInfoWSS),
}
impl Default for DialInfo {
    fn default() -> Self {
        DialInfo::UDP(DialInfoUDP::default())
    }
}

impl fmt::Display for DialInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            DialInfo::UDP(di) => write!(f, "udp|{}", di.socket_address),
            DialInfo::TCP(di) => write!(f, "tcp|{}", di.socket_address),
            DialInfo::WS(di) => {
                let url = format!("ws://{}", di.request);
                let split_url = SplitUrl::from_str(&url).unwrap();
                match split_url.host {
                    SplitUrlHost::Hostname(_) => {
                        write!(f, "ws|{}|{}", di.socket_address.to_ip_addr(), di.request)
                    }
                    SplitUrlHost::IpAddr(a) => {
                        if di.socket_address.to_ip_addr() == a {
                            write!(f, "ws|{}", di.request)
                        } else {
                            panic!("resolved address does not match url: {}", di.request);
                        }
                    }
                }
            }
            DialInfo::WSS(di) => {
                let url = format!("wss://{}", di.request);
                let split_url = SplitUrl::from_str(&url).unwrap();
                match split_url.host {
                    SplitUrlHost::Hostname(_) => {
                        write!(f, "wss|{}|{}", di.socket_address.to_ip_addr(), di.request)
                    }
                    SplitUrlHost::IpAddr(_) => {
                        panic!(
                            "secure websockets can not use ip address in request: {}",
                            di.request
                        );
                    }
                }
            }
        }
    }
}

impl FromStr for DialInfo {
    type Err = VeilidAPIError;
    fn from_str(s: &str) -> Result<DialInfo, VeilidAPIError> {
        let (proto, rest) = s.split_once('|').ok_or_else(|| {
            VeilidAPIError::parse_error("DialInfo::from_str missing protocol '|' separator", s)
        })?;
        match proto {
            "udp" => {
                let socket_address = SocketAddress::from_str(rest)?;
                Ok(DialInfo::udp(socket_address))
            }
            "tcp" => {
                let socket_address = SocketAddress::from_str(rest)?;
                Ok(DialInfo::tcp(socket_address))
            }
            "ws" => {
                let url = format!("ws://{}", rest);
                let split_url = SplitUrl::from_str(&url).map_err(|e| {
                    VeilidAPIError::parse_error(format!("unable to split WS url: {}", e), &url)
                })?;
                if split_url.scheme != "ws" || !url.starts_with("ws://") {
                    return Err(VeilidAPIError::parse_error(
                        "incorrect scheme for WS dialinfo",
                        url,
                    ));
                }
                let url_port = split_url.port.unwrap_or(80u16);

                match rest.split_once('|') {
                    Some((sa, rest)) => {
                        let address = Address::from_str(sa)?;

                        DialInfo::try_ws(
                            SocketAddress::new(address, url_port),
                            format!("ws://{}", rest),
                        )
                    }
                    None => {
                        let address = Address::from_str(&split_url.host.to_string())?;
                        DialInfo::try_ws(
                            SocketAddress::new(address, url_port),
                            format!("ws://{}", rest),
                        )
                    }
                }
            }
            "wss" => {
                let url = format!("wss://{}", rest);
                let split_url = SplitUrl::from_str(&url).map_err(|e| {
                    VeilidAPIError::parse_error(format!("unable to split WSS url: {}", e), &url)
                })?;
                if split_url.scheme != "wss" || !url.starts_with("wss://") {
                    return Err(VeilidAPIError::parse_error(
                        "incorrect scheme for WSS dialinfo",
                        url,
                    ));
                }
                let url_port = split_url.port.unwrap_or(443u16);

                let (a, rest) = rest.split_once('|').ok_or_else(|| {
                    VeilidAPIError::parse_error(
                        "DialInfo::from_str missing socket address '|' separator",
                        s,
                    )
                })?;

                let address = Address::from_str(a)?;
                DialInfo::try_wss(
                    SocketAddress::new(address, url_port),
                    format!("wss://{}", rest),
                )
            }
            _ => Err(VeilidAPIError::parse_error(
                "DialInfo::from_str has invalid scheme",
                s,
            )),
        }
    }
}

impl DialInfo {
    pub fn udp_from_socketaddr(socket_addr: SocketAddr) -> Self {
        Self::UDP(DialInfoUDP {
            socket_address: SocketAddress::from_socket_addr(socket_addr).to_canonical(),
        })
    }
    pub fn tcp_from_socketaddr(socket_addr: SocketAddr) -> Self {
        Self::TCP(DialInfoTCP {
            socket_address: SocketAddress::from_socket_addr(socket_addr).to_canonical(),
        })
    }
    pub fn udp(socket_address: SocketAddress) -> Self {
        Self::UDP(DialInfoUDP {
            socket_address: socket_address.to_canonical(),
        })
    }
    pub fn tcp(socket_address: SocketAddress) -> Self {
        Self::TCP(DialInfoTCP {
            socket_address: socket_address.to_canonical(),
        })
    }
    pub fn try_ws(socket_address: SocketAddress, url: String) -> Result<Self, VeilidAPIError> {
        let split_url = SplitUrl::from_str(&url).map_err(|e| {
            VeilidAPIError::parse_error(format!("unable to split WS url: {}", e), &url)
        })?;
        if split_url.scheme != "ws" || !url.starts_with("ws://") {
            return Err(VeilidAPIError::parse_error(
                "incorrect scheme for WS dialinfo",
                url,
            ));
        }
        let url_port = split_url.port.unwrap_or(80u16);
        if url_port != socket_address.port() {
            return Err(VeilidAPIError::parse_error(
                "socket address port doesn't match url port",
                url,
            ));
        }
        if let SplitUrlHost::IpAddr(a) = split_url.host {
            if socket_address.to_ip_addr() != a {
                return Err(VeilidAPIError::parse_error(
                    format!("request address does not match socket address: {}", a),
                    socket_address,
                ));
            }
        }
        Ok(Self::WS(DialInfoWS {
            socket_address: socket_address.to_canonical(),
            request: url[5..].to_string(),
        }))
    }
    pub fn try_wss(socket_address: SocketAddress, url: String) -> Result<Self, VeilidAPIError> {
        let split_url = SplitUrl::from_str(&url).map_err(|e| {
            VeilidAPIError::parse_error(format!("unable to split WSS url: {}", e), &url)
        })?;
        if split_url.scheme != "wss" || !url.starts_with("wss://") {
            return Err(VeilidAPIError::parse_error(
                "incorrect scheme for WSS dialinfo",
                url,
            ));
        }
        let url_port = split_url.port.unwrap_or(443u16);
        if url_port != socket_address.port() {
            return Err(VeilidAPIError::parse_error(
                "socket address port doesn't match url port",
                url,
            ));
        }
        if !matches!(split_url.host, SplitUrlHost::Hostname(_)) {
            return Err(VeilidAPIError::parse_error(
                "WSS url can not use address format, only hostname format",
                url,
            ));
        }
        Ok(Self::WSS(DialInfoWSS {
            socket_address: socket_address.to_canonical(),
            request: url[6..].to_string(),
        }))
    }
    pub fn protocol_type(&self) -> ProtocolType {
        match self {
            Self::UDP(_) => ProtocolType::UDP,
            Self::TCP(_) => ProtocolType::TCP,
            Self::WS(_) => ProtocolType::WS,
            Self::WSS(_) => ProtocolType::WSS,
        }
    }
    pub fn address_type(&self) -> AddressType {
        self.socket_address().address_type()
    }
    pub fn address(&self) -> Address {
        match self {
            Self::UDP(di) => di.socket_address.address,
            Self::TCP(di) => di.socket_address.address,
            Self::WS(di) => di.socket_address.address,
            Self::WSS(di) => di.socket_address.address,
        }
    }
    pub fn socket_address(&self) -> SocketAddress {
        match self {
            Self::UDP(di) => di.socket_address,
            Self::TCP(di) => di.socket_address,
            Self::WS(di) => di.socket_address,
            Self::WSS(di) => di.socket_address,
        }
    }
    pub fn to_ip_addr(&self) -> IpAddr {
        match self {
            Self::UDP(di) => di.socket_address.to_ip_addr(),
            Self::TCP(di) => di.socket_address.to_ip_addr(),
            Self::WS(di) => di.socket_address.to_ip_addr(),
            Self::WSS(di) => di.socket_address.to_ip_addr(),
        }
    }
    pub fn port(&self) -> u16 {
        match self {
            Self::UDP(di) => di.socket_address.port,
            Self::TCP(di) => di.socket_address.port,
            Self::WS(di) => di.socket_address.port,
            Self::WSS(di) => di.socket_address.port,
        }
    }
    pub fn set_port(&mut self, port: u16) {
        match self {
            Self::UDP(di) => di.socket_address.port = port,
            Self::TCP(di) => di.socket_address.port = port,
            Self::WS(di) => di.socket_address.port = port,
            Self::WSS(di) => di.socket_address.port = port,
        }
    }
    pub fn to_socket_addr(&self) -> SocketAddr {
        match self {
            Self::UDP(di) => di.socket_address.to_socket_addr(),
            Self::TCP(di) => di.socket_address.to_socket_addr(),
            Self::WS(di) => di.socket_address.to_socket_addr(),
            Self::WSS(di) => di.socket_address.to_socket_addr(),
        }
    }
    pub fn to_peer_address(&self) -> PeerAddress {
        match self {
            Self::UDP(di) => PeerAddress::new(di.socket_address, ProtocolType::UDP),
            Self::TCP(di) => PeerAddress::new(di.socket_address, ProtocolType::TCP),
            Self::WS(di) => PeerAddress::new(di.socket_address, ProtocolType::WS),
            Self::WSS(di) => PeerAddress::new(di.socket_address, ProtocolType::WSS),
        }
    }
    pub fn request(&self) -> Option<String> {
        match self {
            Self::UDP(_) => None,
            Self::TCP(_) => None,
            Self::WS(di) => Some(format!("ws://{}", di.request)),
            Self::WSS(di) => Some(format!("wss://{}", di.request)),
        }
    }
    pub fn is_valid(&self) -> bool {
        let socket_address = self.socket_address();
        let address = socket_address.address();
        let port = socket_address.port();
        (address.is_global() || address.is_local()) && port > 0
    }

    pub fn make_filter(&self) -> DialInfoFilter {
        DialInfoFilter {
            protocol_type_set: ProtocolTypeSet::only(self.protocol_type()),
            address_type_set: AddressTypeSet::only(self.address_type()),
        }
    }

    pub fn try_vec_from_short<S: AsRef<str>, H: AsRef<str>>(
        short: S,
        hostname: H,
    ) -> Result<Vec<Self>, VeilidAPIError> {
        let short = short.as_ref();
        let hostname = hostname.as_ref();

        if short.len() < 2 {
            return Err(VeilidAPIError::parse_error(
                "invalid short url length",
                short,
            ));
        }
        let url = match &short[0..1] {
            "U" => {
                format!("udp://{}:{}", hostname, &short[1..])
            }
            "T" => {
                format!("tcp://{}:{}", hostname, &short[1..])
            }
            "W" => {
                format!("ws://{}:{}", hostname, &short[1..])
            }
            "S" => {
                format!("wss://{}:{}", hostname, &short[1..])
            }
            _ => {
                return Err(VeilidAPIError::parse_error("invalid short url type", short));
            }
        };
        Self::try_vec_from_url(url)
    }

    pub fn try_vec_from_url<S: AsRef<str>>(url: S) -> Result<Vec<Self>, VeilidAPIError> {
        let url = url.as_ref();
        let split_url = SplitUrl::from_str(url)
            .map_err(|e| VeilidAPIError::parse_error(format!("unable to split url: {}", e), url))?;

        let port = match split_url.scheme.as_str() {
            "udp" | "tcp" => split_url
                .port
                .ok_or_else(|| VeilidAPIError::parse_error("Missing port in udp url", url))?,
            "ws" => split_url.port.unwrap_or(80u16),
            "wss" => split_url.port.unwrap_or(443u16),
            _ => {
                return Err(VeilidAPIError::parse_error(
                    "Invalid dial info url scheme",
                    split_url.scheme,
                ));
            }
        };

        let socket_addrs = {
            // Resolve if possible, WASM doesn't support resolution and doesn't need it to connect to the dialinfo
            // This will not be used on signed dialinfo, only for bootstrapping, so we don't need to worry about
            // the '0.0.0.0' address being propagated across the routing table
            cfg_if::cfg_if! {
                if #[cfg(target_arch = "wasm32")] {
                    vec![SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0,0,0,0)), port)]
                } else {
                    match split_url.host {
                        SplitUrlHost::Hostname(_) => split_url
                            .host_port(port)
                            .to_socket_addrs()
                            .map_err(|_| VeilidAPIError::parse_error("couldn't resolve hostname in url", url))?
                            .collect(),
                        SplitUrlHost::IpAddr(a) => vec![SocketAddr::new(a, port)],
                    }
                }
            }
        };

        let mut out = Vec::new();
        for sa in socket_addrs {
            out.push(match split_url.scheme.as_str() {
                "udp" => Self::udp_from_socketaddr(sa),
                "tcp" => Self::tcp_from_socketaddr(sa),
                "ws" => Self::try_ws(
                    SocketAddress::from_socket_addr(sa).to_canonical(),
                    url.to_string(),
                )?,
                "wss" => Self::try_wss(
                    SocketAddress::from_socket_addr(sa).to_canonical(),
                    url.to_string(),
                )?,
                _ => {
                    unreachable!("Invalid dial info url scheme")
                }
            });
        }
        Ok(out)
    }

    pub async fn to_short(&self) -> (String, String) {
        match self {
            DialInfo::UDP(di) => (
                format!("U{}", di.socket_address.port()),
                intf::ptr_lookup(di.socket_address.to_ip_addr())
                    .await
                    .unwrap_or_else(|_| di.socket_address.to_string()),
            ),
            DialInfo::TCP(di) => (
                format!("T{}", di.socket_address.port()),
                intf::ptr_lookup(di.socket_address.to_ip_addr())
                    .await
                    .unwrap_or_else(|_| di.socket_address.to_string()),
            ),
            DialInfo::WS(di) => {
                let mut split_url = SplitUrl::from_str(&format!("ws://{}", di.request)).unwrap();
                if let SplitUrlHost::IpAddr(a) = split_url.host {
                    if let Ok(host) = intf::ptr_lookup(a).await {
                        split_url.host = SplitUrlHost::Hostname(host);
                    }
                }
                (
                    format!(
                        "W{}{}",
                        split_url.port.unwrap_or(80),
                        split_url
                            .path
                            .map(|p| format!("/{}", p))
                            .unwrap_or_default()
                    ),
                    split_url.host.to_string(),
                )
            }
            DialInfo::WSS(di) => {
                let mut split_url = SplitUrl::from_str(&format!("wss://{}", di.request)).unwrap();
                if let SplitUrlHost::IpAddr(a) = split_url.host {
                    if let Ok(host) = intf::ptr_lookup(a).await {
                        split_url.host = SplitUrlHost::Hostname(host);
                    }
                }
                (
                    format!(
                        "S{}{}",
                        split_url.port.unwrap_or(443),
                        split_url
                            .path
                            .map(|p| format!("/{}", p))
                            .unwrap_or_default()
                    ),
                    split_url.host.to_string(),
                )
            }
        }
    }
    pub async fn to_url(&self) -> String {
        match self {
            DialInfo::UDP(di) => intf::ptr_lookup(di.socket_address.to_ip_addr())
                .await
                .map(|h| format!("udp://{}:{}", h, di.socket_address.port()))
                .unwrap_or_else(|_| format!("udp://{}", di.socket_address)),
            DialInfo::TCP(di) => intf::ptr_lookup(di.socket_address.to_ip_addr())
                .await
                .map(|h| format!("tcp://{}:{}", h, di.socket_address.port()))
                .unwrap_or_else(|_| format!("tcp://{}", di.socket_address)),
            DialInfo::WS(di) => {
                let mut split_url = SplitUrl::from_str(&format!("ws://{}", di.request)).unwrap();
                if let SplitUrlHost::IpAddr(a) = split_url.host {
                    if let Ok(host) = intf::ptr_lookup(a).await {
                        split_url.host = SplitUrlHost::Hostname(host);
                    }
                }
                split_url.to_string()
            }
            DialInfo::WSS(di) => {
                let mut split_url = SplitUrl::from_str(&format!("wss://{}", di.request)).unwrap();
                if let SplitUrlHost::IpAddr(a) = split_url.host {
                    if let Ok(host) = intf::ptr_lookup(a).await {
                        split_url.host = SplitUrlHost::Hostname(host);
                    }
                }
                split_url.to_string()
            }
        }
    }

    pub fn ordered_sequencing_sort(a: &DialInfo, b: &DialInfo) -> core::cmp::Ordering {
        let ca = a.protocol_type().sort_order(Sequencing::EnsureOrdered);
        let cb = b.protocol_type().sort_order(Sequencing::EnsureOrdered);
        if ca < cb {
            return core::cmp::Ordering::Less;
        }
        if ca > cb {
            return core::cmp::Ordering::Greater;
        }
        match (a, b) {
            (DialInfo::UDP(a), DialInfo::UDP(b)) => a.cmp(b),
            (DialInfo::TCP(a), DialInfo::TCP(b)) => a.cmp(b),
            (DialInfo::WS(a), DialInfo::WS(b)) => a.cmp(b),
            (DialInfo::WSS(a), DialInfo::WSS(b)) => a.cmp(b),
            _ => unreachable!(),
        }
    }
}

impl MatchesDialInfoFilter for DialInfo {
    fn matches_filter(&self, filter: &DialInfoFilter) -> bool {
        if !filter.protocol_type_set.contains(self.protocol_type()) {
            return false;
        }
        if !filter.address_type_set.contains(self.address_type()) {
            return false;
        }
        true
    }
}

//////////////////////////////////////////////////////////////////////////

// Signed NodeInfo that can be passed around amongst peers and verifiable
#[derive(Clone, Debug, Serialize, Deserialize, RkyvArchive, RkyvSerialize, RkyvDeserialize)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct SignedDirectNodeInfo {
    pub node_info: NodeInfo,
    pub timestamp: u64,
    pub signature: Option<DHTSignature>,
}

impl SignedDirectNodeInfo {
    pub fn new(
        node_id: NodeId,
        node_info: NodeInfo,
        timestamp: u64,
        signature: DHTSignature,
    ) -> Result<Self, VeilidAPIError> {
        let node_info_bytes = Self::make_signature_bytes(&node_info, timestamp)?;
        verify(&node_id.key, &node_info_bytes, &signature)?;
        Ok(Self {
            node_info,
            timestamp,
            signature: Some(signature),
        })
    }

    pub fn with_secret(
        node_id: NodeId,
        node_info: NodeInfo,
        secret: &DHTKeySecret,
    ) -> Result<Self, VeilidAPIError> {
        let timestamp = intf::get_timestamp();
        let node_info_bytes = Self::make_signature_bytes(&node_info, timestamp)?;
        let signature = sign(&node_id.key, secret, &node_info_bytes)?;
        Ok(Self {
            node_info,
            timestamp,
            signature: Some(signature),
        })
    }

    fn make_signature_bytes(
        node_info: &NodeInfo,
        timestamp: u64,
    ) -> Result<Vec<u8>, VeilidAPIError> {
        let mut node_info_bytes = Vec::new();

        // Add nodeinfo to signature
        let mut ni_msg = ::capnp::message::Builder::new_default();
        let mut ni_builder = ni_msg.init_root::<veilid_capnp::node_info::Builder>();
        encode_node_info(node_info, &mut ni_builder).map_err(VeilidAPIError::internal)?;
        node_info_bytes.append(&mut builder_to_vec(ni_msg).map_err(VeilidAPIError::internal)?);

        // Add timestamp to signature
        node_info_bytes.append(&mut timestamp.to_le_bytes().to_vec());

        Ok(node_info_bytes)
    }

    pub fn with_no_signature(node_info: NodeInfo) -> Self {
        Self {
            node_info,
            signature: None,
            timestamp: intf::get_timestamp(),
        }
    }

    pub fn has_valid_signature(&self) -> bool {
        self.signature.is_some()
    }
}

/// Signed NodeInfo with a relay that can be passed around amongst peers and verifiable
#[derive(Clone, Debug, Serialize, Deserialize, RkyvArchive, RkyvSerialize, RkyvDeserialize)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct SignedRelayedNodeInfo {
    pub node_info: NodeInfo,
    pub relay_id: NodeId,
    pub relay_info: SignedDirectNodeInfo,
    pub timestamp: u64,
    pub signature: DHTSignature,
}

impl SignedRelayedNodeInfo {
    pub fn new(
        node_id: NodeId,
        node_info: NodeInfo,
        relay_id: NodeId,
        relay_info: SignedDirectNodeInfo,
        timestamp: u64,
        signature: DHTSignature,
    ) -> Result<Self, VeilidAPIError> {
        let node_info_bytes =
            Self::make_signature_bytes(&node_info, &relay_id, &relay_info, timestamp)?;
        verify(&node_id.key, &node_info_bytes, &signature)?;
        Ok(Self {
            node_info,
            relay_id,
            relay_info,
            signature,
            timestamp,
        })
    }

    pub fn with_secret(
        node_id: NodeId,
        node_info: NodeInfo,
        relay_id: NodeId,
        relay_info: SignedDirectNodeInfo,
        secret: &DHTKeySecret,
    ) -> Result<Self, VeilidAPIError> {
        let timestamp = intf::get_timestamp();
        let node_info_bytes =
            Self::make_signature_bytes(&node_info, &relay_id, &relay_info, timestamp)?;
        let signature = sign(&node_id.key, secret, &node_info_bytes)?;
        Ok(Self {
            node_info,
            relay_id,
            relay_info,
            signature,
            timestamp,
        })
    }

    fn make_signature_bytes(
        node_info: &NodeInfo,
        relay_id: &NodeId,
        relay_info: &SignedDirectNodeInfo,
        timestamp: u64,
    ) -> Result<Vec<u8>, VeilidAPIError> {
        let mut sig_bytes = Vec::new();

        // Add nodeinfo to signature
        let mut ni_msg = ::capnp::message::Builder::new_default();
        let mut ni_builder = ni_msg.init_root::<veilid_capnp::node_info::Builder>();
        encode_node_info(node_info, &mut ni_builder).map_err(VeilidAPIError::internal)?;
        sig_bytes.append(&mut builder_to_vec(ni_msg).map_err(VeilidAPIError::internal)?);

        // Add relay id to signature
        let mut rid_msg = ::capnp::message::Builder::new_default();
        let mut rid_builder = rid_msg.init_root::<veilid_capnp::curve25519_public_key::Builder>();
        encode_public_key(&relay_id.key, &mut rid_builder).map_err(VeilidAPIError::internal)?;
        sig_bytes.append(&mut builder_to_vec(rid_msg).map_err(VeilidAPIError::internal)?);

        // Add relay info to signature
        let mut ri_msg = ::capnp::message::Builder::new_default();
        let mut ri_builder = ri_msg.init_root::<veilid_capnp::signed_direct_node_info::Builder>();
        encode_signed_direct_node_info(relay_info, &mut ri_builder)
            .map_err(VeilidAPIError::internal)?;
        sig_bytes.append(&mut builder_to_vec(ri_msg).map_err(VeilidAPIError::internal)?);

        // Add timestamp to signature
        sig_bytes.append(&mut timestamp.to_le_bytes().to_vec());

        Ok(sig_bytes)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, RkyvArchive, RkyvSerialize, RkyvDeserialize)]
#[archive_attr(repr(u8), derive(CheckBytes))]
pub enum SignedNodeInfo {
    Direct(SignedDirectNodeInfo),
    Relayed(SignedRelayedNodeInfo),
}

impl SignedNodeInfo {
    pub fn has_valid_signature(&self) -> bool {
        match self {
            SignedNodeInfo::Direct(d) => d.has_valid_signature(),
            SignedNodeInfo::Relayed(_) => true,
        }
    }

    pub fn timestamp(&self) -> u64 {
        match self {
            SignedNodeInfo::Direct(d) => d.timestamp,
            SignedNodeInfo::Relayed(r) => r.timestamp,
        }
    }

    pub fn node_info(&self) -> &NodeInfo {
        match self {
            SignedNodeInfo::Direct(d) => &d.node_info,
            SignedNodeInfo::Relayed(r) => &r.node_info,
        }
    }
    pub fn relay_id(&self) -> Option<NodeId> {
        match self {
            SignedNodeInfo::Direct(_) => None,
            SignedNodeInfo::Relayed(r) => Some(r.relay_id.clone()),
        }
    }
    pub fn relay_info(&self) -> Option<&NodeInfo> {
        match self {
            SignedNodeInfo::Direct(_) => None,
            SignedNodeInfo::Relayed(r) => Some(&r.relay_info.node_info),
        }
    }
    pub fn relay_peer_info(&self) -> Option<PeerInfo> {
        match self {
            SignedNodeInfo::Direct(_) => None,
            SignedNodeInfo::Relayed(r) => Some(PeerInfo::new(
                r.relay_id.clone(),
                SignedNodeInfo::Direct(r.relay_info.clone()),
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
        for did in &self.node_info().dial_info_detail_list {
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
                for did in &relay_ni.dial_info_detail_list {
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

#[derive(Clone, Debug, Serialize, Deserialize, RkyvArchive, RkyvSerialize, RkyvDeserialize)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct PeerInfo {
    pub node_id: NodeId,
    pub signed_node_info: SignedNodeInfo,
}

impl PeerInfo {
    pub fn new(node_id: NodeId, signed_node_info: SignedNodeInfo) -> Self {
        Self {
            node_id,
            signed_node_info,
        }
    }
}

#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct PeerAddress {
    protocol_type: ProtocolType,
    #[serde(with = "json_as_string")]
    socket_address: SocketAddress,
}

impl PeerAddress {
    pub fn new(socket_address: SocketAddress, protocol_type: ProtocolType) -> Self {
        Self {
            socket_address: socket_address.to_canonical(),
            protocol_type,
        }
    }

    pub fn socket_address(&self) -> &SocketAddress {
        &self.socket_address
    }

    pub fn protocol_type(&self) -> ProtocolType {
        self.protocol_type
    }

    pub fn to_socket_addr(&self) -> SocketAddr {
        self.socket_address.to_socket_addr()
    }

    pub fn address_type(&self) -> AddressType {
        self.socket_address.address_type()
    }
}

/// Represents the 5-tuple of an established connection
/// Not used to specify connections to create, that is reserved for DialInfo
///
/// ConnectionDescriptors should never be from unspecified local addresses for connection oriented protocols
/// If the medium does not allow local addresses, None should have been used or 'new_no_local'
/// If we are specifying only a port, then the socket's 'local_address()' should have been used, since an
/// established connection is always from a real address to another real address.
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct ConnectionDescriptor {
    remote: PeerAddress,
    local: Option<SocketAddress>,
}

impl ConnectionDescriptor {
    pub fn new(remote: PeerAddress, local: SocketAddress) -> Self {
        assert!(
            !remote.protocol_type().is_connection_oriented() || !local.address().is_unspecified()
        );

        Self {
            remote,
            local: Some(local),
        }
    }
    pub fn new_no_local(remote: PeerAddress) -> Self {
        Self {
            remote,
            local: None,
        }
    }
    pub fn remote(&self) -> PeerAddress {
        self.remote
    }
    pub fn remote_address(&self) -> &SocketAddress {
        self.remote.socket_address()
    }
    pub fn local(&self) -> Option<SocketAddress> {
        self.local
    }
    pub fn protocol_type(&self) -> ProtocolType {
        self.remote.protocol_type
    }
    pub fn address_type(&self) -> AddressType {
        self.remote.address_type()
    }
    pub fn make_dial_info_filter(&self) -> DialInfoFilter {
        DialInfoFilter::all()
            .with_protocol_type(self.protocol_type())
            .with_address_type(self.address_type())
    }
}

impl MatchesDialInfoFilter for ConnectionDescriptor {
    fn matches_filter(&self, filter: &DialInfoFilter) -> bool {
        if !filter.protocol_type_set.contains(self.protocol_type()) {
            return false;
        }
        if !filter.address_type_set.contains(self.address_type()) {
            return false;
        }
        true
    }
}

//////////////////////////////////////////////////////////////////////////

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct LatencyStats {
    #[serde(with = "json_as_string")]
    pub fastest: u64, // fastest latency in the ROLLING_LATENCIES_SIZE last latencies
    #[serde(with = "json_as_string")]
    pub average: u64, // average latency over the ROLLING_LATENCIES_SIZE last latencies
    #[serde(with = "json_as_string")]
    pub slowest: u64, // slowest latency in the ROLLING_LATENCIES_SIZE last latencies
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct TransferStats {
    #[serde(with = "json_as_string")]
    pub total: u64, // total amount transferred ever
    #[serde(with = "json_as_string")]
    pub maximum: u64, // maximum rate over the ROLLING_TRANSFERS_SIZE last amounts
    #[serde(with = "json_as_string")]
    pub average: u64, // average rate over the ROLLING_TRANSFERS_SIZE last amounts
    #[serde(with = "json_as_string")]
    pub minimum: u64, // minimum rate over the ROLLING_TRANSFERS_SIZE last amounts
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct TransferStatsDownUp {
    pub down: TransferStats,
    pub up: TransferStats,
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct RPCStats {
    pub messages_sent: u32, // number of rpcs that have been sent in the total_time range
    pub messages_rcvd: u32, // number of rpcs that have been received in the total_time range
    pub questions_in_flight: u32, // number of questions issued that have yet to be answered
    #[serde(with = "opt_json_as_string")]
    pub last_question: Option<u64>, // when the peer was last questioned (either successfully or not) and we wanted an answer
    #[serde(with = "opt_json_as_string")]
    pub last_seen_ts: Option<u64>, // when the peer was last seen for any reason, including when we first attempted to reach out to it
    #[serde(with = "opt_json_as_string")]
    pub first_consecutive_seen_ts: Option<u64>, // the timestamp of the first consecutive proof-of-life for this node (an answer or received question)
    pub recent_lost_answers: u32, // number of answers that have been lost since we lost reliability
    pub failed_to_send: u32, // number of messages that have failed to send since we last successfully sent one
}

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct PeerStats {
    #[serde(with = "json_as_string")]
    pub time_added: u64, // when the peer was added to the routing table
    pub rpc_stats: RPCStats,           // information about RPCs
    pub latency: Option<LatencyStats>, // latencies for communications with the peer
    pub transfer: TransferStatsDownUp, // Stats for communications with the peer
}

pub type ValueChangeCallback =
    Arc<dyn Fn(ValueKey, Vec<u8>) -> SendPinBoxFuture<()> + Send + Sync + 'static>;

/////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Serialize, Deserialize, RkyvArchive, RkyvSerialize, RkyvDeserialize)]
#[archive_attr(repr(u8), derive(CheckBytes))]
pub enum SignalInfo {
    HolePunch {
        // UDP Hole Punch Request
        receipt: Vec<u8>,    // Receipt to be returned after the hole punch
        peer_info: PeerInfo, // Sender's peer info
    },
    ReverseConnect {
        // Reverse Connection Request
        receipt: Vec<u8>,    // Receipt to be returned by the reverse connection
        peer_info: PeerInfo, // Sender's peer info
    },
    // XXX: WebRTC
    // XXX: App-level signalling
}

/////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(
    Copy,
    Clone,
    Debug,
    PartialOrd,
    PartialEq,
    Eq,
    Ord,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive_attr(repr(u8), derive(CheckBytes))]
pub enum TunnelMode {
    Raw,
    Turn,
}

#[derive(
    Copy,
    Clone,
    Debug,
    PartialOrd,
    PartialEq,
    Eq,
    Ord,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive_attr(repr(u8), derive(CheckBytes))]
pub enum TunnelError {
    BadId,        // Tunnel ID was rejected
    NoEndpoint,   // Endpoint was unreachable
    RejectedMode, // Endpoint couldn't provide mode
    NoCapacity,   // Endpoint is full
}

pub type TunnelId = u64;

#[derive(Clone, Debug, Serialize, Deserialize, RkyvArchive, RkyvSerialize, RkyvDeserialize)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct TunnelEndpoint {
    pub mode: TunnelMode,
    pub description: String, // XXX: TODO
}

impl Default for TunnelEndpoint {
    fn default() -> Self {
        Self {
            mode: TunnelMode::Raw,
            description: "".to_string(),
        }
    }
}

#[derive(
    Clone, Debug, Default, Serialize, Deserialize, RkyvArchive, RkyvSerialize, RkyvDeserialize,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct FullTunnel {
    pub id: TunnelId,
    pub timeout: u64,
    pub local: TunnelEndpoint,
    pub remote: TunnelEndpoint,
}

#[derive(
    Clone, Debug, Default, Serialize, Deserialize, RkyvArchive, RkyvSerialize, RkyvDeserialize,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct PartialTunnel {
    pub id: TunnelId,
    pub timeout: u64,
    pub local: TunnelEndpoint,
}

/////////////////////////////////////////////////////////////////////////////////////////////////////

struct VeilidAPIInner {
    context: Option<VeilidCoreContext>,
}

impl fmt::Debug for VeilidAPIInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VeilidAPIInner")
    }
}

impl Drop for VeilidAPIInner {
    fn drop(&mut self) {
        if let Some(context) = self.context.take() {
            intf::spawn_detached(api_shutdown(context));
        }
    }
}

#[derive(Clone, Debug)]
pub struct VeilidAPI {
    inner: Arc<Mutex<VeilidAPIInner>>,
}

impl VeilidAPI {
    #[instrument(skip_all)]
    pub(crate) fn new(context: VeilidCoreContext) -> Self {
        Self {
            inner: Arc::new(Mutex::new(VeilidAPIInner {
                context: Some(context),
            })),
        }
    }

    #[instrument(skip_all)]
    pub async fn shutdown(self) {
        let context = { self.inner.lock().context.take() };
        if let Some(context) = context {
            api_shutdown(context).await;
        }
    }

    pub fn is_shutdown(&self) -> bool {
        self.inner.lock().context.is_none()
    }

    ////////////////////////////////////////////////////////////////
    // Accessors
    pub fn config(&self) -> Result<VeilidConfig, VeilidAPIError> {
        let inner = self.inner.lock();
        if let Some(context) = &inner.context {
            return Ok(context.config.clone());
        }
        Err(VeilidAPIError::NotInitialized)
    }
    pub fn crypto(&self) -> Result<Crypto, VeilidAPIError> {
        let inner = self.inner.lock();
        if let Some(context) = &inner.context {
            return Ok(context.crypto.clone());
        }
        Err(VeilidAPIError::NotInitialized)
    }
    pub fn table_store(&self) -> Result<TableStore, VeilidAPIError> {
        let inner = self.inner.lock();
        if let Some(context) = &inner.context {
            return Ok(context.table_store.clone());
        }
        Err(VeilidAPIError::not_initialized())
    }
    pub fn block_store(&self) -> Result<BlockStore, VeilidAPIError> {
        let inner = self.inner.lock();
        if let Some(context) = &inner.context {
            return Ok(context.block_store.clone());
        }
        Err(VeilidAPIError::not_initialized())
    }
    pub fn protected_store(&self) -> Result<ProtectedStore, VeilidAPIError> {
        let inner = self.inner.lock();
        if let Some(context) = &inner.context {
            return Ok(context.protected_store.clone());
        }
        Err(VeilidAPIError::not_initialized())
    }
    pub fn attachment_manager(&self) -> Result<AttachmentManager, VeilidAPIError> {
        let inner = self.inner.lock();
        if let Some(context) = &inner.context {
            return Ok(context.attachment_manager.clone());
        }
        Err(VeilidAPIError::not_initialized())
    }
    pub fn network_manager(&self) -> Result<NetworkManager, VeilidAPIError> {
        let inner = self.inner.lock();
        if let Some(context) = &inner.context {
            return Ok(context.attachment_manager.network_manager());
        }
        Err(VeilidAPIError::not_initialized())
    }
    pub fn rpc_processor(&self) -> Result<RPCProcessor, VeilidAPIError> {
        let inner = self.inner.lock();
        if let Some(context) = &inner.context {
            return Ok(context.attachment_manager.network_manager().rpc_processor());
        }
        Err(VeilidAPIError::NotInitialized)
    }

    ////////////////////////////////////////////////////////////////
    // Attach/Detach

    // get a full copy of the current state
    pub async fn get_state(&self) -> Result<VeilidState, VeilidAPIError> {
        let attachment_manager = self.attachment_manager()?;
        let network_manager = attachment_manager.network_manager();

        let attachment = attachment_manager.get_veilid_state();
        let network = network_manager.get_veilid_state();

        Ok(VeilidState {
            attachment,
            network,
        })
    }

    // get network connectedness

    // connect to the network
    #[instrument(level = "debug", err, skip_all)]
    pub async fn attach(&self) -> Result<(), VeilidAPIError> {
        let attachment_manager = self.attachment_manager()?;
        attachment_manager
            .request_attach()
            .await
            .map_err(|e| VeilidAPIError::internal(e))
    }

    // disconnect from the network
    #[instrument(level = "debug", err, skip_all)]
    pub async fn detach(&self) -> Result<(), VeilidAPIError> {
        let attachment_manager = self.attachment_manager()?;
        attachment_manager
            .request_detach()
            .await
            .map_err(|e| VeilidAPIError::internal(e))
    }

    ////////////////////////////////////////////////////////////////
    // Routing Context

    #[instrument(level = "debug", skip(self))]
    pub fn routing_context(&self) -> RoutingContext {
        RoutingContext::new(self.clone())
    }

    ////////////////////////////////////////////////////////////////
    // App Calls

    #[instrument(level = "debug", skip(self))]
    pub async fn app_call_reply(&self, id: u64, message: Vec<u8>) -> Result<(), VeilidAPIError> {
        let rpc_processor = self.rpc_processor()?;
        rpc_processor
            .app_call_reply(id, message)
            .await
            .map_err(|e| e.into())
    }

    ////////////////////////////////////////////////////////////////
    // Tunnel Building

    #[instrument(level = "debug", err, skip(self))]
    pub async fn start_tunnel(
        &self,
        _endpoint_mode: TunnelMode,
        _depth: u8,
    ) -> Result<PartialTunnel, VeilidAPIError> {
        panic!("unimplemented");
    }

    #[instrument(level = "debug", err, skip(self))]
    pub async fn complete_tunnel(
        &self,
        _endpoint_mode: TunnelMode,
        _depth: u8,
        _partial_tunnel: PartialTunnel,
    ) -> Result<FullTunnel, VeilidAPIError> {
        panic!("unimplemented");
    }

    #[instrument(level = "debug", err, skip(self))]
    pub async fn cancel_tunnel(&self, _tunnel_id: TunnelId) -> Result<bool, VeilidAPIError> {
        panic!("unimplemented");
    }
}
