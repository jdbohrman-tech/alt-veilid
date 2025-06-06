use super::*;

/// Mechanism required to contact another node
#[derive(Clone, Debug)]
pub enum ContactMethod {
    /// Node is not reachable by any means
    Unreachable,
    /// Connection should have already existed
    Existing,
    /// Contact the node directly
    Direct(DialInfo),
    /// Request via signal the node connect back directly (relay, target)
    SignalReverse(TypedNodeId, TypedNodeId),
    /// Request via signal the node negotiate a hole punch (relay, target)
    SignalHolePunch(TypedNodeId, TypedNodeId),
    /// Must use an inbound relay to reach the node
    InboundRelay(TypedNodeId),
    /// Must use outbound relay to reach the node
    OutboundRelay(TypedNodeId),
}
