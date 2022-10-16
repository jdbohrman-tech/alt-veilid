use super::*;

/// Where to send an RPC message
#[derive(Debug, Clone)]
pub enum Destination {
    /// Send to node directly
    Direct {
        /// The node to send to
        target: NodeRef,
        /// Require safety route or not
        safety: Option<SafetySpec>,
    },
    /// Send to node for relay purposes
    Relay {
        /// The relay to send to
        relay: NodeRef,
        /// The final destination the relay should send to
        target: DHTKey,
        /// Require safety route or not
        safety: Option<SafetySpec>,
    },
    /// Send to private route (privateroute)
    PrivateRoute {
        /// A private route to send to
        private_route: PrivateRoute,
        /// Require safety route or not
        safety: Option<SafetySpec>,
        /// Prefer reliability or not
        reliable: bool,
    },
}

impl Destination {
    pub fn direct(target: NodeRef) -> Self {
        Self::Direct {
            target,
            safety: None,
        }
    }
    pub fn relay(relay: NodeRef, target: DHTKey) -> Self {
        Self::Relay {
            relay,
            target,
            safety: None,
        }
    }
    pub fn private_route(private_route: PrivateRoute, reliable: bool) -> Self {
        Self::PrivateRoute {
            private_route,
            safety: None,
            reliable,
        }
    }

    pub fn with_safety(self, spec: SafetySpec) -> Self {
        match self {
            Destination::Direct { target, safety: _ } => Self::Direct {
                target,
                safety: Some(spec),
            },
            Destination::Relay {
                relay,
                target,
                safety: _,
            } => Self::Relay {
                relay,
                target,
                safety: Some(spec),
            },
            Destination::PrivateRoute {
                private_route,
                safety: _,
                reliable,
            } => Self::PrivateRoute {
                private_route,
                safety: Some(spec),
                reliable,
            },
        }
    }
}

impl fmt::Display for Destination {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Destination::Direct { target, safety } => {
                let sr = if safety.is_some() { "+SR" } else { "" };

                write!(f, "{}{}", target, sr)
            }
            Destination::Relay {
                relay,
                target,
                safety,
            } => {
                let sr = if safety.is_some() { "+SR" } else { "" };

                write!(f, "{}@{}{}", target.encode(), relay, sr)
            }
            Destination::PrivateRoute {
                private_route,
                safety,
                reliable,
            } => {
                let sr = if safety.is_some() { "+SR" } else { "" };
                let rl = if *reliable { "+RL" } else { "" };

                write!(f, "{}{}{}", private_route, sr, rl)
            }
        }
    }
}
