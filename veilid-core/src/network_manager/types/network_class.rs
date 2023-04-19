use super::*;

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
