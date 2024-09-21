use super::*;

/// Mechanism required to contact another node
#[derive(Clone, Debug)]
pub(crate) enum ContactMethod {
    /// Node is not reachable by any means
    Unreachable,
    /// Connection should have already existed
    Existing,
    /// Contact the node directly
    Direct(DialInfo),
    /// Request via signal the node connect back directly (relay, target)
    SignalReverse(TypedKey, TypedKey),
    /// Request via signal the node negotiate a hole punch (relay, target)
    SignalHolePunch(TypedKey, TypedKey),
    /// Must use an inbound relay to reach the node
    InboundRelay(TypedKey),
    /// Must use outbound relay to reach the node
    OutboundRelay(TypedKey),
}
