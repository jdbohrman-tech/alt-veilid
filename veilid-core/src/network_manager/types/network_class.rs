use super::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub(crate) enum NetworkClass {
    InboundCapable = 0, // I = Has inbound-capable dialinfo, including direct or holepunch-able NAT'd dialinfo
    OutboundOnly = 1, // O = Has no dialinfo but may be reachable via inbound relay or reverse connections.
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
