use super::*;

/// Where to send an RPC message
#[derive(Debug, Clone)]
pub enum Destination {
    /// Send to node directly
    Direct {
        /// The node to send to
        target: NodeRef,
        /// Require safety route or not
        safety_selection: SafetySelection,
    },
    /// Send to node for relay purposes
    Relay {
        /// The relay to send to
        relay: NodeRef,
        /// The final destination the relay should send to
        target: DHTKey,
        /// Require safety route or not
        safety_selection: SafetySelection,
    },
    /// Send to private route (privateroute)
    PrivateRoute {
        /// A private route to send to
        private_route: PrivateRoute,
        /// Require safety route or not
        safety_selection: SafetySelection,
    },
}

impl Destination {
    pub fn direct(target: NodeRef) -> Self {
        Self::Direct {
            target,
            safety_selection: SafetySelection::Unsafe(target.sequencing()),
        }
    }
    pub fn relay(relay: NodeRef, target: DHTKey) -> Self {
        Self::Relay {
            relay,
            target,
            safety_selection: SafetySelection::Unsafe(relay.sequencing()),
        }
    }
    pub fn private_route(private_route: PrivateRoute, safety_selection: SafetySelection) -> Self {
        Self::PrivateRoute {
            private_route,
            safety_selection,
        }
    }

    pub fn with_safety(self, safety_selection: SafetySelection) -> Self {
        match self {
            Destination::Direct {
                target,
                safety_selection: _,
            } => Self::Direct {
                target,
                safety_selection,
            },
            Destination::Relay {
                relay,
                target,
                safety_selection: _,
            } => Self::Relay {
                relay,
                target,
                safety_selection,
            },
            Destination::PrivateRoute {
                private_route,
                safety_selection: _,
            } => Self::PrivateRoute {
                private_route,
                safety_selection,
            },
        }
    }

    pub fn get_safety_selection(&self) -> &SafetySelection {
        match self {
            Destination::Direct {
                target: _,
                safety_selection,
            } => safety_selection,
            Destination::Relay {
                relay: _,
                target: _,
                safety_selection,
            } => safety_selection,
            Destination::PrivateRoute {
                private_route: _,
                safety_selection,
            } => safety_selection,
        }
    }
}

impl fmt::Display for Destination {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Destination::Direct {
                target,
                safety_selection,
            } => {
                let sr = if matches!(safety_selection, SafetySelection::Safe(_)) {
                    "+SR"
                } else {
                    ""
                };

                write!(f, "{}{}", target, sr)
            }
            Destination::Relay {
                relay,
                target,
                safety_selection,
            } => {
                let sr = if matches!(safety_selection, SafetySelection::Safe(_)) {
                    "+SR"
                } else {
                    ""
                };

                write!(f, "{}@{}{}", target.encode(), relay, sr)
            }
            Destination::PrivateRoute {
                private_route,
                safety_selection,
            } => {
                let sr = if matches!(safety_selection, SafetySelection::Safe(_)) {
                    "+SR"
                } else {
                    ""
                };

                write!(f, "{}{}", private_route, sr)
            }
        }
    }
}
