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
        target: NodeRef,
        /// Require safety route or not
        safety_selection: SafetySelection,
    },
    /// Send to private route
    PrivateRoute {
        /// A private route to send to
        private_route: PrivateRoute,
        /// Require safety route or not
        safety_selection: SafetySelection,
    },
}

impl Destination {
    pub fn target(&self) -> Option<NodeRef> {
        match self {
            Destination::Direct {
                target,
                safety_selection: _,
            } => Some(target.clone()),
            Destination::Relay {
                relay: _,
                target,
                safety_selection: _,
            } => Some(target.clone()),
            Destination::PrivateRoute {
                private_route: _,
                safety_selection: _,
            } => None,
        }
    }
    pub fn direct(target: NodeRef) -> Self {
        let sequencing = target.sequencing();
        Self::Direct {
            target,
            safety_selection: SafetySelection::Unsafe(sequencing),
        }
    }
    pub fn relay(relay: NodeRef, target: NodeRef) -> Self {
        let sequencing = relay.sequencing().max(target.sequencing());
        Self::Relay {
            relay,
            target,
            safety_selection: SafetySelection::Unsafe(sequencing),
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

                write!(f, "{}@{}{}", target, relay, sr)
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

                write!(f, "{}{}", private_route.public_key, sr)
            }
        }
    }
}

impl RPCProcessor {
    /// Convert the 'Destination' into a 'RespondTo' for a response
    pub(super) fn get_destination_respond_to(
        &self,
        dest: &Destination,
    ) -> Result<NetworkResult<RespondTo>, RPCError> {
        let routing_table = self.routing_table();
        let rss = routing_table.route_spec_store();

        match dest {
            Destination::Direct {
                target,
                safety_selection,
            } => match safety_selection {
                SafetySelection::Unsafe(_) => {
                    // Sent directly with no safety route, can respond directly
                    Ok(NetworkResult::value(RespondTo::Sender))
                }
                SafetySelection::Safe(safety_spec) => {
                    // Sent directly but with a safety route, respond to private route
                    let crypto_kind = target.best_node_id().kind;
                    let Some(pr_key) = rss
                        .get_private_route_for_safety_spec(
                            crypto_kind,
                            safety_spec,
                            &target.node_ids(),
                        )
                        .map_err(RPCError::internal)?
                    else {
                        return Ok(NetworkResult::no_connection_other(
                            "no private route for response at this time",
                        ));
                    };

                    // Get the assembled route for response
                    let private_route = rss
                        .assemble_private_route(&pr_key, None)
                        .map_err(RPCError::internal)?;

                    Ok(NetworkResult::Value(RespondTo::PrivateRoute(private_route)))
                }
            },
            Destination::Relay {
                relay,
                target,
                safety_selection,
            } => match safety_selection {
                SafetySelection::Unsafe(_) => {
                    // Sent via a relay with no safety route, can respond directly
                    Ok(NetworkResult::value(RespondTo::Sender))
                }
                SafetySelection::Safe(safety_spec) => {
                    // Sent via a relay but with a safety route, respond to private route
                    let crypto_kind = target.best_node_id().kind;

                    let mut avoid_nodes = relay.node_ids();
                    avoid_nodes.add_all(&target.node_ids());
                    let Some(pr_key) = rss
                        .get_private_route_for_safety_spec(crypto_kind, safety_spec, &avoid_nodes)
                        .map_err(RPCError::internal)?
                    else {
                        return Ok(NetworkResult::no_connection_other(
                            "no private route for response at this time",
                        ));
                    };

                    // Get the assembled route for response
                    let private_route = rss
                        .assemble_private_route(&pr_key, None)
                        .map_err(RPCError::internal)?;

                    Ok(NetworkResult::Value(RespondTo::PrivateRoute(private_route)))
                }
            },
            Destination::PrivateRoute {
                private_route,
                safety_selection,
            } => {
                let Some(avoid_node_id) = private_route.first_hop_node_id() else {
                    return Err(RPCError::internal(
                        "destination private route must have first hop",
                    ));
                };

                let crypto_kind = private_route.public_key.kind;

                match safety_selection {
                    SafetySelection::Unsafe(_) => {
                        // Sent to a private route with no safety route, use a stub safety route for the response
                        if !routing_table.has_valid_network_class(RoutingDomain::PublicInternet) {
                            return Ok(NetworkResult::no_connection_other(
                                "Own node info must be valid to use private route",
                            ));
                        }

                        // Determine if we can use optimized nodeinfo
                        let route_node = if rss.has_remote_private_route_seen_our_node_info(
                            &private_route.public_key.value,
                        ) {
                            RouteNode::NodeId(routing_table.node_id(crypto_kind).value)
                        } else {
                            let own_peer_info =
                                routing_table.get_own_peer_info(RoutingDomain::PublicInternet);
                            RouteNode::PeerInfo(Box::new(own_peer_info))
                        };

                        Ok(NetworkResult::value(RespondTo::PrivateRoute(
                            PrivateRoute::new_stub(routing_table.node_id(crypto_kind), route_node),
                        )))
                    }
                    SafetySelection::Safe(safety_spec) => {
                        // Sent to a private route via a safety route, respond to private route

                        // Check for loopback test
                        let opt_private_route_id =
                            rss.get_route_id_for_key(&private_route.public_key.value);
                        let pr_key = if opt_private_route_id.is_some()
                            && safety_spec.preferred_route == opt_private_route_id
                        {
                            // Private route is also safety route during loopback test
                            private_route.public_key.value
                        } else {
                            // Get the private route to respond to that matches the safety route spec we sent the request with
                            let Some(pr_key) = rss
                                .get_private_route_for_safety_spec(
                                    crypto_kind,
                                    safety_spec,
                                    &[avoid_node_id],
                                )
                                .map_err(RPCError::internal)?
                            else {
                                return Ok(NetworkResult::no_connection_other(
                                    "no private route for response at this time",
                                ));
                            };
                            pr_key
                        };

                        // Get the assembled route for response
                        let private_route = rss
                            .assemble_private_route(&pr_key, None)
                            .map_err(RPCError::internal)?;

                        Ok(NetworkResult::Value(RespondTo::PrivateRoute(private_route)))
                    }
                }
            }
        }
    }

    /// Convert the 'RespondTo' into a 'Destination' for a response
    pub(super) fn get_respond_to_destination(
        &self,
        request: &RPCMessage,
    ) -> NetworkResult<Destination> {
        // Get the question 'respond to'
        let respond_to = match request.operation.kind() {
            RPCOperationKind::Question(q) => q.respond_to(),
            _ => {
                panic!("not a question");
            }
        };

        // To where should we respond?
        match respond_to {
            RespondTo::Sender => {
                // Parse out the header detail from the question
                let detail = match &request.header.detail {
                    RPCMessageHeaderDetail::Direct(detail) => detail,
                    RPCMessageHeaderDetail::SafetyRouted(_)
                    | RPCMessageHeaderDetail::PrivateRouted(_) => {
                        // If this was sent via a private route, we don't know what the sender was, so drop this
                        return NetworkResult::invalid_message(
                            "can't respond directly to non-direct question",
                        );
                    }
                };

                // Reply directly to the request's source
                let sender_node_id = detail.envelope.get_sender_typed_id();

                // This may be a different node's reference than the 'sender' in the case of a relay
                let peer_noderef = detail.peer_noderef.clone();

                // If the sender_id is that of the peer, then this is a direct reply
                // else it is a relayed reply through the peer
                if peer_noderef.node_ids().contains(&sender_node_id) {
                    NetworkResult::value(Destination::direct(peer_noderef))
                } else {
                    // Look up the sender node, we should have added it via senderNodeInfo before getting here.
                    let res = match self.routing_table.lookup_node_ref(sender_node_id) {
                        Ok(v) => v,
                        Err(e) => {
                            return NetworkResult::invalid_message(format!(
                                "failed to look up node info for respond to: {}",
                                e
                            ))
                        }
                    };
                    if let Some(sender_noderef) = res {
                        NetworkResult::value(Destination::relay(peer_noderef, sender_noderef))
                    } else {
                        NetworkResult::invalid_message(
                            "not responding to sender that has no node info",
                        )
                    }
                }
            }
            RespondTo::PrivateRoute(pr) => {
                match &request.header.detail {
                    RPCMessageHeaderDetail::Direct(_) => {
                        // If this was sent directly, we should only ever respond directly
                        NetworkResult::invalid_message(
                            "not responding to private route from direct question",
                        )
                    }
                    RPCMessageHeaderDetail::SafetyRouted(detail) => {
                        // If this was sent via a safety route, but not received over our private route, don't respond with a safety route,
                        // it would give away which safety routes belong to this node
                        NetworkResult::value(Destination::private_route(
                            pr.clone(),
                            SafetySelection::Unsafe(detail.sequencing),
                        ))
                    }
                    RPCMessageHeaderDetail::PrivateRouted(detail) => {
                        // If this was received over our private route, it's okay to respond to a private route via our safety route
                        NetworkResult::value(Destination::private_route(
                            pr.clone(),
                            SafetySelection::Safe(detail.safety_spec),
                        ))
                    }
                }
            }
        }
    }
}
