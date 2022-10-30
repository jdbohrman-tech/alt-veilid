use super::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Compiled Privacy Objects

#[derive(Clone, Debug)]
pub struct RouteHopData {
    pub nonce: Nonce,
    pub blob: Vec<u8>,
}

#[derive(Clone, Debug)]
pub enum RouteNode {
    NodeId(NodeId),
    PeerInfo(PeerInfo),
}
impl fmt::Display for RouteNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RouteNode::NodeId(x) => x.key.encode(),
                RouteNode::PeerInfo(pi) => pi.node_id.key.encode(),
            }
        )
    }
}

#[derive(Clone, Debug)]
pub struct RouteHop {
    pub node: RouteNode,
    pub next_hop: Option<RouteHopData>,
}

#[derive(Clone, Debug)]
pub struct PrivateRoute {
    pub public_key: DHTKey,
    pub hop_count: u8,
    pub first_hop: Option<RouteHop>,
}

impl PrivateRoute {
    /// Empty private route is the form used when receiving the last hop
    pub fn new_empty(public_key: DHTKey) -> Self {
        Self {
            public_key,
            hop_count: 0,
            first_hop: None,
        }
    }
    /// Stub route is the form used when no privacy is required, but you need to specify the destination for a safety route
    pub fn new_stub(public_key: DHTKey, node: RouteNode) -> Self {
        Self {
            public_key,
            hop_count: 1,
            first_hop: Some(RouteHop {
                node,
                next_hop: None,
            }),
        }
    }
    /// Switch from full node info to simple node id
    /// Only simplified single hop, primarily useful for stubs
    /// Published routes with >= 1 hops should be in NodeId form already, as they have
    /// already been connectivity-verified by the time the route is published
    pub fn simplify(self) -> Self {
        Self {
            public_key: self.public_key,
            hop_count: self.hop_count,
            first_hop: self.first_hop.map(|h| RouteHop {
                node: match h.node {
                    RouteNode::NodeId(ni) => RouteNode::NodeId(ni),
                    RouteNode::PeerInfo(pi) => RouteNode::NodeId(pi.node_id),
                },
                next_hop: h.next_hop,
            }),
        }
    }
}

impl fmt::Display for PrivateRoute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "PR({:?}+{}{})",
            self.public_key,
            self.hop_count,
            if let Some(first_hop) = &self.first_hop {
                format!("->{}", first_hop.node)
            } else {
                "".to_owned()
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
    pub public_key: DHTKey,
    pub hop_count: u8,
    pub hops: SafetyRouteHops,
}

impl SafetyRoute {
    pub fn new_stub(public_key: DHTKey, private_route: PrivateRoute) -> Self {
        Self {
            public_key,
            hop_count: 0,
            hops: SafetyRouteHops::Private(private_route),
        }
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
