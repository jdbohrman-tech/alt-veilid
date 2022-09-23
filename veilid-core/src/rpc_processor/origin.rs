use super::*;

#[derive(Debug, Clone)]
pub enum Origin {
    Sender,
    PrivateRoute(PrivateRoute),
}

impl Origin {
    pub fn sender() -> Self {
        Self::Sender
    }

    pub fn private_route(private_route: PrivateRoute) -> Self {
        Self::PrivateRoute(private_route)
    }

    pub fn into_respond_to(self, destination: &Destination) -> Result<RespondTo, RPCError> {
        match self {
            Self::Sender => {
                let peer = match destination {
                    Destination::Direct {
                        target,
                        safety_route_spec,
                    } => todo!(),
                    Destination::Relay {
                        relay,
                        target,
                        safety_route_spec,
                    } => todo!(),
                    Destination::PrivateRoute {
                        private_route,
                        safety_route_spec,
                    } => todo!(),
                };
                let routing_table = peer.routing_table();
                let routing_domain = peer.best_routing_domain();
                // Send some signed node info along with the question if this node needs to be replied to
                if routing_table.has_valid_own_node_info()
                    && !peer.has_seen_our_node_info(routing_domain)
                {
                    let our_sni = self
                        .routing_table()
                        .get_own_signed_node_info(routing_domain);
                    RespondTo::Sender(Some(our_sni))
                } else {
                    RespondTo::Sender(None)
                }
            }
            Self::PrivateRoute(pr) => RespondTo::PrivateRoute(pr),
        }
    }
}
