use super::*;

/// Tunnel identifier
pub type TunnelId = AlignedU64;

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

#[derive(
    Clone, Debug, PartialEq, Eq, Serialize, Deserialize, RkyvArchive, RkyvSerialize, RkyvDeserialize,
)]
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
pub struct FullTunnel {
    pub id: TunnelId,
    pub timeout: TimestampDuration,
    pub local: TunnelEndpoint,
    pub remote: TunnelEndpoint,
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
pub struct PartialTunnel {
    pub id: TunnelId,
    pub timeout: TimestampDuration,
    pub local: TunnelEndpoint,
}
