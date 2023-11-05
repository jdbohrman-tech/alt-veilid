use super::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Compiled Privacy Objects

/// An encrypted private/safety route hop
#[derive(Clone, Debug)]
pub(crate) struct RouteHopData {
    /// The nonce used in the encryption ENC(Xn,DH(PKn,SKapr))
    pub nonce: Nonce,
    /// The encrypted blob
    pub blob: Vec<u8>,
}

/// How to find a route node
#[derive(Clone, Debug)]
pub(crate) enum RouteNode {
    /// Route node is optimized, no contact method information as this node id has been seen before
    NodeId(PublicKey),
    /// Route node with full contact method information to ensure the peer is reachable
    PeerInfo(Box<PeerInfo>),
}

impl RouteNode {
    pub fn validate(&self, crypto: Crypto) -> VeilidAPIResult<()> {
        match self {
            RouteNode::NodeId(_) => Ok(()),
            RouteNode::PeerInfo(pi) => pi.validate(crypto),
        }
    }

    pub fn node_ref(
        &self,
        routing_table: RoutingTable,
        crypto_kind: CryptoKind,
    ) -> Option<NodeRef> {
        match self {
            RouteNode::NodeId(id) => {
                //
                match routing_table.lookup_node_ref(TypedKey::new(crypto_kind, *id)) {
                    Ok(nr) => nr,
                    Err(e) => {
                        log_rtab!(debug "failed to look up route node: {}", e);
                        None
                    }
                }
            }
            RouteNode::PeerInfo(pi) => {
                //
                match routing_table.register_node_with_peer_info(
                    RoutingDomain::PublicInternet,
                    *pi.clone(),
                    false,
                ) {
                    Ok(nr) => Some(nr),
                    Err(e) => {
                        log_rtab!(debug "failed to register route node: {}", e);
                        None
                    }
                }
            }
        }
    }

    pub fn describe(&self, crypto_kind: CryptoKind) -> String {
        match self {
            RouteNode::NodeId(id) => {
                format!("{}", TypedKey::new(crypto_kind, *id))
            }
            RouteNode::PeerInfo(pi) => match pi.node_ids().get(crypto_kind) {
                Some(id) => format!("{}", id),
                None => {
                    format!("({})?{}", crypto_kind, pi.node_ids())
                }
            },
        }
    }
}

/// An unencrypted private/safety route hop
#[derive(Clone, Debug)]
pub(crate) struct RouteHop {
    /// The location of the hop
    pub node: RouteNode,
    /// The encrypted blob to pass to the next hop as its data (None for stubs)
    pub next_hop: Option<RouteHopData>,
}
impl RouteHop {
    pub fn validate(&self, crypto: Crypto) -> VeilidAPIResult<()> {
        self.node.validate(crypto)
    }
}

/// The kind of hops a private route can have
#[derive(Clone, Debug)]
pub(crate) enum PrivateRouteHops {
    /// The first hop of a private route, unencrypted, route_hops == total hop count
    FirstHop(Box<RouteHop>),
    /// Private route internal node. Has > 0 private route hops left but < total hop count
    Data(RouteHopData),
    /// Private route has ended (hop count = 0)
    Empty,
}

impl PrivateRouteHops {
    pub fn validate(&self, crypto: Crypto) -> VeilidAPIResult<()> {
        match self {
            PrivateRouteHops::FirstHop(rh) => rh.validate(crypto),
            PrivateRouteHops::Data(_) => Ok(()),
            PrivateRouteHops::Empty => Ok(()),
        }
    }
}
/// A private route for receiver privacy
#[derive(Clone, Debug)]
pub(crate) struct PrivateRoute {
    /// The public key used for the entire route
    pub public_key: TypedKey,
    pub hop_count: u8,
    pub hops: PrivateRouteHops,
}

impl PrivateRoute {
    /// Stub route is the form used when no privacy is required, but you need to specify the destination for a safety route
    pub fn new_stub(public_key: TypedKey, node: RouteNode) -> Self {
        Self {
            public_key,
            hop_count: 1,
            hops: PrivateRouteHops::FirstHop(Box::new(RouteHop {
                node,
                next_hop: None,
            })),
        }
    }

    pub fn validate(&self, crypto: Crypto) -> VeilidAPIResult<()> {
        self.hops.validate(crypto)
    }

    /// Check if this is a stub route
    pub fn is_stub(&self) -> bool {
        if let PrivateRouteHops::FirstHop(first_hop) = &self.hops {
            return first_hop.next_hop.is_none();
        }
        false
    }

    /// Get the crypto kind in use for this route
    pub fn crypto_kind(&self) -> CryptoKind {
        self.public_key.kind
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

                Some(first_hop_node)
            }
            PrivateRouteHops::Data(_) => None,
            PrivateRouteHops::Empty => None,
        }
    }

    pub fn first_hop_node_id(&self) -> Option<TypedKey> {
        let PrivateRouteHops::FirstHop(pr_first_hop) = &self.hops else {
            return None;
        };

        // Get the safety route to use from the spec
        Some(match &pr_first_hop.node {
            RouteNode::NodeId(n) => TypedKey::new(self.public_key.kind, *n),
            RouteNode::PeerInfo(p) => p.node_ids().get(self.public_key.kind).unwrap(),
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
                PrivateRouteHops::FirstHop(_) => {
                    format!(
                        "->{}",
                        self.first_hop_node_id()
                            .map(|n| n.to_string())
                            .unwrap_or_else(|| "None".to_owned())
                    )
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
pub(crate) enum SafetyRouteHops {
    /// Has >= 1 safety route hops
    Data(RouteHopData),
    /// Has 0 safety route hops
    Private(PrivateRoute),
}

#[derive(Clone, Debug)]
pub(crate) struct SafetyRoute {
    pub public_key: TypedKey,
    pub hop_count: u8,
    pub hops: SafetyRouteHops,
}

impl SafetyRoute {
    /// Stub route is the form used when no privacy is required, but you need to directly contact a private route
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

    /// Check if this is a stub route
    pub fn is_stub(&self) -> bool {
        matches!(self.hops, SafetyRouteHops::Private(_))
    }

    /// Get the crypto kind in use for this route
    pub fn crypto_kind(&self) -> CryptoKind {
        self.public_key.kind
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
