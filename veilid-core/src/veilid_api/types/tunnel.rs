#[cfg(feature = "unstable-tunnels")]
use super::*;

/// Tunnel identifier
#[cfg(feature = "unstable-tunnels")]
pub type TunnelId = AlignedU64;

#[cfg(feature = "unstable-tunnels")]
#[derive(
    Copy, Clone, Debug, PartialOrd, PartialEq, Eq, Ord, Serialize, Deserialize, JsonSchema,
)]
pub enum TunnelMode {
    Raw = 0,
    Turn = 1,
}

#[cfg(feature = "unstable-tunnels")]
#[derive(
    Copy, Clone, Debug, PartialOrd, PartialEq, Eq, Ord, Serialize, Deserialize, JsonSchema,
)]
pub enum TunnelError {
    BadId = 0,        // Tunnel ID was rejected
    NoEndpoint = 1,   // Endpoint was unreachable
    RejectedMode = 2, // Endpoint couldn't provide mode
    NoCapacity = 3,   // Endpoint is full
}

#[cfg(feature = "unstable-tunnels")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
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
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct FullTunnel {
    pub id: TunnelId,
    pub timeout: TimestampDuration,
    pub local: TunnelEndpoint,
    pub remote: TunnelEndpoint,
}

#[cfg(feature = "unstable-tunnels")]
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct PartialTunnel {
    pub id: TunnelId,
    pub timeout: TimestampDuration,
    pub local: TunnelEndpoint,
}
