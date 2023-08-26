use super::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
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
    // Must an inbound relay be kept available?
    // In the case of InboundCapable, it is left up to the class of each DialInfo to determine if an inbound relay is required
    pub fn inbound_wants_relay(&self) -> bool {
        matches!(self, Self::OutboundOnly | Self::WebApp)
    }
    // Should an outbound relay be kept available?
    pub fn outbound_wants_relay(&self) -> bool {
        matches!(self, Self::WebApp)
    }
}
