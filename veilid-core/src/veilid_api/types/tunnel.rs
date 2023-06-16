#[cfg(feature = "unstable-tunnels")]
use super::*;

/// Tunnel identifier
#[cfg(feature = "unstable-tunnels")]
pub type TunnelId = AlignedU64;

#[cfg(feature = "unstable-tunnels")]
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
    JsonSchema,
)]
#[archive_attr(repr(u8), derive(CheckBytes))]
pub enum TunnelMode {
    Raw,
    Turn,
}

#[cfg(feature = "unstable-tunnels")]
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
    JsonSchema,
)]
#[archive_attr(repr(u8), derive(CheckBytes))]
pub enum TunnelError {
    BadId,        // Tunnel ID was rejected
    NoEndpoint,   // Endpoint was unreachable
    RejectedMode, // Endpoint couldn't provide mode
    NoCapacity,   // Endpoint is full
}

#[cfg(feature = "unstable-tunnels")]
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
    JsonSchema,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct TunnelEndpoint {
    pub mode: TunnelMode,
    pub description: String, // XXX: TODO
}

#[cfg(feature = "unstable-tunnels")]
impl Default for TunnelEndpoint {
    fn default() -> Self {
        Self {
            mode: TunnelMode::Raw,
            description: "".to_string(),
        }
    }
}

#[cfg(feature = "unstable-tunnels")]
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
    JsonSchema,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct FullTunnel {
    pub id: TunnelId,
    pub timeout: TimestampDuration,
    pub local: TunnelEndpoint,
    pub remote: TunnelEndpoint,
}

#[cfg(feature = "unstable-tunnels")]
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
    JsonSchema,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct PartialTunnel {
    pub id: TunnelId,
    pub timeout: TimestampDuration,
    pub local: TunnelEndpoint,
}
