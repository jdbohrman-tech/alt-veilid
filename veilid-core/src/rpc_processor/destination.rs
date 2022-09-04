use super::*;

/// Where to send an RPC message
#[derive(Debug, Clone)]
pub enum Destination {
    /// Send to node directly
    Direct {
        /// The node to send to
        target: NodeRef,
        /// An optional safety route specification to send from for sender privacy
        safety_route_spec: Option<Arc<SafetyRouteSpec>>,
    },
    /// Send to node for relay purposes
    Relay {
        /// The relay to send to
        relay: NodeRef,
        /// The final destination the relay should send to
        target: DHTKey,
        /// An optional safety route specification to send from for sender privacy
        safety_route_spec: Option<Arc<SafetyRouteSpec>>,
    },
    /// Send to private route (privateroute)
    PrivateRoute {
        /// A private route to send to
        private_route: PrivateRoute,
        /// An optional safety route specification to send from for sender privacy
        safety_route_spec: Option<Arc<SafetyRouteSpec>>,
    },
}

impl Destination {
    pub fn direct(target: NodeRef) -> Self {
        Self::Direct {
            target,
            safety_route_spec: None,
        }
    }
    pub fn relay(relay: NodeRef, target: DHTKey) -> Self {
        Self::Relay {
            relay,
            target,
            safety_route_spec: None,
        }
    }
    pub fn private_route(private_route: PrivateRoute) -> Self {
        Self::PrivateRoute {
            private_route,
            safety_route_spec: None,
        }
    }
    // pub fn target_id(&self) -> DHTKey {
    //     match self {
    //         Destination::Direct {
    //             target,
    //             safety_route_spec,
    //         } => target.node_id(),
    //         Destination::Relay {
    //             relay,
    //             target,
    //             safety_route_spec,
    //         } => *target,
    //         Destination::PrivateRoute {
    //             private_route,
    //             safety_route_spec,
    //         } => {}
    //     }
    // }

    // pub fn best_routing_domain(&self) -> RoutingDomain {
    //     match self {
    //         Destination::Direct {
    //             target,
    //             safety_route_spec,
    //         } => {
    //             if safety_route_spec.is_some() {
    //                 RoutingDomain::PublicInternet
    //             } else {
    //                 target
    //                     .best_routing_domain()
    //                     .unwrap_or(RoutingDomain::PublicInternet)
    //             }
    //         }
    //         Destination::Relay {
    //             relay,
    //             target,
    //             safety_route_spec,
    //         } => {
    //             if safety_route_spec.is_some() {
    //                 RoutingDomain::PublicInternet
    //             } else {
    //                 relay
    //                     .best_routing_domain()
    //                     .unwrap_or(RoutingDomain::PublicInternet)
    //             }
    //         }
    //         Destination::PrivateRoute {
    //             private_route: _,
    //             safety_route_spec: _,
    //         } => RoutingDomain::PublicInternet,
    //     }
    // }

    pub fn safety_route_spec(&self) -> Option<Arc<SafetyRouteSpec>> {
        match self {
            Destination::Direct {
                target,
                safety_route_spec,
            } => safety_route_spec.clone(),
            Destination::Relay {
                relay,
                target,
                safety_route_spec,
            } => safety_route_spec.clone(),
            Destination::PrivateRoute {
                private_route,
                safety_route_spec,
            } => safety_route_spec.clone(),
        }
    }
    pub fn with_safety_route_spec(self, safety_route_spec: Arc<SafetyRouteSpec>) -> Self {
        match self {
            Destination::Direct {
                target,
                safety_route_spec: _,
            } => Self::Direct {
                target,
                safety_route_spec: Some(safety_route_spec),
            },
            Destination::Relay {
                relay,
                target,
                safety_route_spec: _,
            } => Self::Relay {
                relay,
                target,
                safety_route_spec: Some(safety_route_spec),
            },
            Destination::PrivateRoute {
                private_route,
                safety_route_spec: _,
            } => Self::PrivateRoute {
                private_route,
                safety_route_spec: Some(safety_route_spec),
            },
        }
    }
}

impl fmt::Display for Destination {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Destination::Direct {
                target,
                safety_route_spec,
            } => {
                let sr = safety_route_spec
                    .map(|_sr| "+SR".to_owned())
                    .unwrap_or_default();

                write!(f, "{:?}{}", target, sr)
            }
            Destination::Relay {
                relay,
                target,
                safety_route_spec,
            } => {
                let sr = safety_route_spec
                    .map(|_sr| "+SR".to_owned())
                    .unwrap_or_default();

                write!(f, "{:?}@{:?}{}", target.encode(), relay, sr)
            }
            Destination::PrivateRoute {
                private_route,
                safety_route_spec,
            } => {
                let sr = safety_route_spec
                    .map(|_sr| "+SR".to_owned())
                    .unwrap_or_default();

                write!(f, "{}{}", private_route, sr)
            }
        }
    }
}
