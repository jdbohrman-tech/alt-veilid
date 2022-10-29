use super::*;

impl RPCProcessor {
    async fn process_route_safety_route_hop(
        &self,
        route: &RPCOperationRoute,
        route_hop: RouteHop,
    ) -> Result<(), RPCError> {
        // Get next hop node ref
        let next_hop_nr = match route_hop.node {
            RouteNode::NodeId(id) => {
                //
                self.routing_table
                    .lookup_node_ref(id.key)
                    .ok_or_else(|| RPCError::network(format!("node hop {} not found", id.key)))
            }
            RouteNode::PeerInfo(pi) => {
                //
                self.routing_table
                    .register_node_with_signed_node_info(
                        RoutingDomain::PublicInternet,
                        pi.node_id.key,
                        pi.signed_node_info,
                        false,
                    )
                    .ok_or_else(|| {
                        RPCError::network(format!(
                            "node hop {} could not be registered",
                            pi.node_id.key
                        ))
                    })
            }
        }?;

        // Pass along the route
        let next_hop_route = RPCOperationRoute {
            safety_route: SafetyRoute {
                public_key: route.safety_route.public_key,
                hop_count: route.safety_route.hop_count - 1,
                hops: SafetyRouteHops::Data(route_hop.next_hop),
            },
            operation: route.operation,
        };
        let next_hop_route_stmt = RPCStatement::new(RPCStatementDetail::Route(next_hop_route));

        // Send the next route statement
        network_result_try!(
            self.statement(Destination::direct(next_hop_nr), next_hop_route_stmt)
                .await?
        );

        Ok(())
    }

    async fn process_route_safety_route_private_route_hop(
        &self,
        route: &RPCOperationRoute,
        private_route: &PrivateRoute,
    ) -> Result<(), RPCError> {
        //
        let route_hop = private_route.first_hop.unwrap();

        // Pass along the route
        let next_hop_route = RPCOperationRoute {
            safety_route: SafetyRoute {
                public_key: route.safety_route.public_key,
                hop_count: 0,
                hops: SafetyRouteHops::PrivateRoute(Private),
            },
            operation: route.operation,
        };
        let next_hop_route_stmt = RPCStatement::new(RPCStatementDetail::Route(next_hop_route));

        // Send the next route statement
        network_result_try!(
            self.statement(Destination::direct(next_hop_nr), next_hop_route_stmt)
                .await?
        );

        Ok(())
    }
    async fn process_routed_operation(
        &self,
        route: &RPCOperationRoute,
        private_route: &PrivateRoute,
    ) -> Result<(), RPCError> {
        //
        Ok(())
    }

    #[instrument(level = "trace", skip(self, msg), fields(msg.operation.op_id), err)]
    pub(crate) async fn process_route(&self, msg: RPCMessage) -> Result<(), RPCError> {
        // xxx do not process latency for routed messages
        // tracing::Span::current().record("res", &tracing::field::display(res));

        // Get the statement
        let route = match msg.operation.kind() {
            RPCOperationKind::Statement(s) => match s.detail() {
                RPCStatementDetail::Route(s) => s,
                _ => panic!("not a route statement"),
            },
            _ => panic!("not a statement"),
        };

        // See what kind of safety route we have going on here
        match &route.safety_route.hops {
            // There is a safety route hop
            SafetyRouteHops::Data(d) => {
                // See if this is last hop in safety route, if so, we're decoding a PrivateRoute not a RouteHop
                let (blob_tag, blob_data) = if let Some(b) = d.blob.last() {
                    (*b, &d.blob[0..d.blob.len() - 1])
                } else {
                    return Err(RPCError::protocol("no bytes in blob"));
                };

                // Decrypt the blob with DEC(nonce, DH(the SR's public key, this hop's secret)
                let node_id_secret = self.routing_table.node_id_secret();
                let dh_secret = self
                    .crypto
                    .cached_dh(&route.safety_route.public_key, &node_id_secret)
                    .map_err(RPCError::protocol)?;
                let dec_blob_data = Crypto::decrypt_aead(blob_data, &d.nonce, &dh_secret, None)
                    .map_err(RPCError::map_internal("encryption failed"))?;
                let dec_blob_reader = capnp::message::Reader::new(
                    RPCMessageData {
                        contents: dec_blob_data,
                    },
                    Default::default(),
                );

                // Decode the blob appropriately
                if blob_tag == 0 {
                    // PrivateRoute
                    let private_route = {
                        let pr_reader = dec_blob_reader
                            .get_root::<veilid_capnp::private_route::Reader>()
                            .map_err(RPCError::protocol)?;
                        decode_private_route(&pr_reader)?
                    };

                    // Make sure hop count makes sense
                    if route.safety_route.hop_count as usize != 0 {
                        return Err(RPCError::protocol(
                            "Safety hop count should be zero if switched to private route",
                        ));
                    }

                    // Get the next hop node ref
                    if private_route.first_hop.is_some() {
                        // Make sure hop count makes sense
                        if private_route.hop_count as usize
                            > self.unlocked_inner.max_route_hop_count
                        {
                            return Err(RPCError::protocol(
                                "Private route hop count too high to process",
                            ));
                        }
                        if private_route.hop_count == 0 {
                            return Err(RPCError::protocol(
                                "Private route hop count should not be zero if there are more hops",
                            ));
                        }

                        // Switching to private route from safety route
                        self.process_route_safety_route_private_route_hop(route, &private_route)
                            .await?;
                    } else {
                        // Make sure hop count makes sense
                        if private_route.hop_count != 0 {
                            return Err(RPCError::protocol(
                                "Private route hop count should be zero if we are at the end",
                            ));
                        }

                        // Private route was a stub, process routed operation
                        self.process_routed_operation(route, &private_route).await?;
                    }
                } else if blob_tag == 1 {
                    // RouteHop
                    let route_hop = {
                        let rh_reader = dec_blob_reader
                            .get_root::<veilid_capnp::route_hop::Reader>()
                            .map_err(RPCError::protocol)?;
                        decode_route_hop(&rh_reader)?
                    };

                    // Make sure hop count makes sense
                    if route.safety_route.hop_count as usize
                        > self.unlocked_inner.max_route_hop_count
                    {
                        return Err(RPCError::protocol(
                            "Safety route hop count too high to process",
                        ));
                    }
                    if route.safety_route.hop_count == 0 {
                        return Err(RPCError::protocol(
                            "Safety route hop count should not be zero if there are more hops",
                        ));
                    }

                    self.process_route_safety_route_hop(route, route_hop)
                        .await?;
                } else {
                    return Err(RPCError::protocol("invalid blob tag"));
                }
            }
            // Safety route has ended, now do private route
            SafetyRouteHops::Private(private_route) => {
                if private_route.first_hop.is_some() {
                    // Make sure hop count makes sense
                    if private_route.hop_count as usize > self.unlocked_inner.max_route_hop_count {
                        return Err(RPCError::protocol(
                            "Private route hop count too high to process",
                        ));
                    }
                    if private_route.hop_count == 0 {
                        return Err(RPCError::protocol(
                            "Private route hop count should not be zero if there are more hops",
                        ));
                    }

                    // There are some hops left
                    self.process_route_safety_route_private_route_hop(route, private_route)
                        .await?;
                } else {
                    // Make sure hop count makes sense
                    if private_route.hop_count != 0 {
                        return Err(RPCError::protocol(
                            "Private route hop count should be zero if we are at the end",
                        ));
                    }

                    // No hops left, time to process the routed operation
                    self.process_routed_operation(route, private_route).await?;
                }
            }
        }

        Ok(())
    }
}
