use super::*;

/// Where to send an RPC message
#[derive(Debug, Clone)]
pub enum Destination {
    /// Send to node directly
    Direct {
        /// The node to send to
        target: NodeRef,
        /// Require safety route or not
        xxx convert back to safety spec, bubble up to api
        safety: bool,
    },
    /// Send to node for relay purposes
    Relay {
        /// The relay to send to
        relay: NodeRef,
        /// The final destination the relay should send to
        target: DHTKey,
        /// Require safety route or not
        safety: bool,
    },
    /// Send to private route (privateroute)
    PrivateRoute {
        /// A private route to send to
        private_route: PrivateRoute,
        /// Require safety route or not
        safety: bool,
        /// Prefer reliability or not
        reliable: bool,
    },
}

impl Destination {
    pub fn direct(target: NodeRef) -> Self {
        Self::Direct {
            target,
            safety: false,
        }
    }
    pub fn relay(relay: NodeRef, target: DHTKey) -> Self {
        Self::Relay {
            relay,
            target,
            safety: false,
        }
    }
    pub fn private_route(private_route: PrivateRoute, reliable: bool) -> Self {
        Self::PrivateRoute {
            private_route,
            safety: false,
            reliable,
        }
    }

    pub fn with_safety(self) -> Self {
        match self {
            Destination::Direct { target, safety: _ } => Self::Direct {
                target,
                safety: true,
            },
            Destination::Relay {
                relay,
                target,
                safety: _,
            } => Self::Relay {
                relay,
                target,
                safety: true,
            },
            Destination::PrivateRoute {
                private_route,
                safety: _,
                reliable,
            } => Self::PrivateRoute {
                private_route,
                safety: true,
                reliable,
            },
        }
    }
}

impl fmt::Display for Destination {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Destination::Direct { target, safety } => {
                let sr = if *safety { "+SR" } else { "" };

                write!(f, "{}{}", target, sr)
            }
            Destination::Relay {
                relay,
                target,
                safety,
            } => {
                let sr = if *safety { "+SR" } else { "" };

                write!(f, "{}@{}{}", target.encode(), relay, sr)
            }
            Destination::PrivateRoute {
                private_route,
                safety,
                reliable,
            } => {
                let sr = if *safety { "+SR" } else { "" };
                let rl = if *reliable { "+RL" } else { "" };

                write!(f, "{}{}{}", private_route, sr, rl)
            }
        }
    }
}
