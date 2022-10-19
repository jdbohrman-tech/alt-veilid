use super::*;

/// Where to send an RPC message
#[derive(Debug, Clone)]
pub enum Destination {
    /// Send to node directly
    Direct {
        /// The node to send to
        target: NodeRef,
        /// Require safety route or not
        safety_spec: Option<SafetySpec>,
    },
    /// Send to node for relay purposes
    Relay {
        /// The relay to send to
        relay: NodeRef,
        /// The final destination the relay should send to
        target: DHTKey,
        /// Require safety route or not
        safety_spec: Option<SafetySpec>,
    },
    /// Send to private route (privateroute)
    PrivateRoute {
        /// A private route to send to
        private_route: PrivateRoute,
        /// Require safety route or not
        safety_spec: Option<SafetySpec>,
        /// Prefer reliability or not
        reliable: bool,
    },
}

impl Destination {
    pub fn direct(target: NodeRef) -> Self {
        Self::Direct {
            target,
            safety_spec: None,
        }
    }
    pub fn relay(relay: NodeRef, target: DHTKey) -> Self {
        Self::Relay {
            relay,
            target,
            safety_spec: None,
        }
    }
    pub fn private_route(private_route: PrivateRoute, reliable: bool) -> Self {
        Self::PrivateRoute {
            private_route,
            safety_spec: None,
            reliable,
        }
    }

    pub fn with_safety(self, safety_spec: SafetySpec) -> Self {
        match self {
            Destination::Direct {
                target,
                safety_spec: _,
            } => Self::Direct {
                target,
                safety_spec: Some(safety_spec),
            },
            Destination::Relay {
                relay,
                target,
                safety_spec: _,
            } => Self::Relay {
                relay,
                target,
                safety_spec: Some(safety_spec),
            },
            Destination::PrivateRoute {
                private_route,
                safety_spec: _,
                reliable,
            } => Self::PrivateRoute {
                private_route,
                safety_spec: Some(safety_spec),
                reliable,
            },
        }
    }
}

impl fmt::Display for Destination {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Destination::Direct {
                target,
                safety_spec,
            } => {
                let sr = if safety_spec.is_some() { "+SR" } else { "" };

                write!(f, "{}{}", target, sr)
            }
            Destination::Relay {
                relay,
                target,
                safety_spec,
            } => {
                let sr = if safety_spec.is_some() { "+SR" } else { "" };

                write!(f, "{}@{}{}", target.encode(), relay, sr)
            }
            Destination::PrivateRoute {
                private_route,
                safety_spec,
                reliable,
            } => {
                let sr = if safety_spec.is_some() { "+SR" } else { "" };
                let rl = if *reliable { "+RL" } else { "" };

                write!(f, "{}{}{}", private_route, sr, rl)
            }
        }
    }
}
