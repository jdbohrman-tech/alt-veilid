use super::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Compiled Privacy Objects

/// An encrypted private/safety route hop
#[derive(Clone, Debug)]
pub struct RouteHopData {
    /// The nonce used in the encryption ENC(Xn,DH(PKn,SKapr))
    pub nonce: Nonce,
    /// The encrypted blob
    pub blob: Vec<u8>,
}

/// How to find a route node
#[derive(Clone, Debug)]
pub enum RouteNode {
    /// Route node is optimized, no contact method information as this node id has been seen before
    NodeId(TypedKey),
    /// Route node with full contact method information to ensure the peer is reachable
    PeerInfo(PeerInfo),
}

/// An unencrypted private/safety route hop
#[derive(Clone, Debug)]
pub struct RouteHop {
    /// The location of the hop
    pub node: RouteNode,
    /// The encrypted blob to pass to the next hop as its data (None for stubs)
    pub next_hop: Option<RouteHopData>,
}

/// The kind of hops a private route can have
#[derive(Clone, Debug)]
pub enum PrivateRouteHops {
    /// The first hop of a private route, unencrypted, route_hops == total hop count
    FirstHop(RouteHop),
    /// Private route internal node. Has > 0 private route hops left but < total hop count
    Data(RouteHopData),
    /// Private route has ended (hop count = 0)
    Empty,
}

/// A private route for receiver privacy
#[derive(Clone, Debug)]
pub struct PrivateRoute {
    /// The public key used for the entire route
    pub public_key: TypedKey,
    pub hop_count: u8,
    pub hops: PrivateRouteHops,
}

impl PrivateRoute {
    /// Empty private route is the form used when receiving the last hop
    pub fn new_empty(public_key: TypedKey) -> Self {
        Self {
            public_key,
            hop_count: 0,
            hops: PrivateRouteHops::Empty,
        }
    }
    /// Stub route is the form used when no privacy is required, but you need to specify the destination for a safety route
    pub fn new_stub(public_key: TypedKey, node: RouteNode) -> Self {
        Self {
            public_key,
            hop_count: 1,
            hops: PrivateRouteHops::FirstHop(RouteHop {
                node,
                next_hop: None,
            }),
        }
    }

    /// Check if this is a stub route
    pub fn is_stub(&self) -> bool {
        if let PrivateRouteHops::FirstHop(first_hop) = &self.hops {
            return first_hop.next_hop.is_none();
        }
        false
    }

    /// Remove the first unencrypted hop if possible
    pub fn pop_first_hop(&mut self) -> Option<RouteNode> {
        match &mut self.hops {
            PrivateRouteHops::FirstHop(first_hop) => {
                let first_hop_node = first_hop.node.clone();

                // Reduce hop count
                if self.hop_count > 0 {
                    self.hop_count -= 1;
                } else {
                    error!("hop count should not be 0 for first hop");
                }

                // Go to next hop
                self.hops = match first_hop.next_hop.take() {
                    Some(rhd) => PrivateRouteHops::Data(rhd),
                    None => PrivateRouteHops::Empty,
                };

                return Some(first_hop_node);
            }
            PrivateRouteHops::Data(_) => return None,
            PrivateRouteHops::Empty => return None,
        }
    }

    pub fn first_hop_node_id(&self) -> Option<TypedKey> {
        let PrivateRouteHops::FirstHop(pr_first_hop) = &self.hops else {
            return None;
        };

        // Get the safety route to use from the spec
        Some(match &pr_first_hop.node {
            RouteNode::NodeId(n) => n,
            RouteNode::PeerInfo(p) => p.node_id.key,
        })
    }
}

impl fmt::Display for PrivateRoute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "PR({:?}+{}{})",
            self.public_key,
            self.hop_count,
            match &self.hops {
                PrivateRouteHops::FirstHop(fh) => {
                    format!("->{}", fh.node)
                }
                PrivateRouteHops::Data(_) => {
                    "->?".to_owned()
                }
                PrivateRouteHops::Empty => {
                    "".to_owned()
                }
            }
        )
    }
}

#[derive(Clone, Debug)]
pub enum SafetyRouteHops {
    /// Has >= 1 safety route hops
    Data(RouteHopData),
    /// Has 0 safety route hops
    Private(PrivateRoute),
}

#[derive(Clone, Debug)]
pub struct SafetyRoute {
    pub public_key: TypedKey,
    pub hop_count: u8,
    pub hops: SafetyRouteHops,
}

impl SafetyRoute {
    pub fn new_stub(public_key: TypedKey, private_route: PrivateRoute) -> Self {
        // First hop should have already been popped off for stubbed safety routes since
        // we are sending directly to the first hop
        assert!(matches!(private_route.hops, PrivateRouteHops::Data(_)));
        Self {
            public_key,
            hop_count: 0,
            hops: SafetyRouteHops::Private(private_route),
        }
    }
    pub fn is_stub(&self) -> bool {
        matches!(self.hops, SafetyRouteHops::Private(_))
    }
}

impl fmt::Display for SafetyRoute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SR({:?}+{}{})",
            self.public_key,
            self.hop_count,
            match &self.hops {
                SafetyRouteHops::Data(_) => "".to_owned(),
                SafetyRouteHops::Private(p) => format!("->{}", p),
            }
        )
    }
}
