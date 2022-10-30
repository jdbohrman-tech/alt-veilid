use super::*;

impl RPCProcessor {
    #[instrument(level = "trace", skip_all, err)]
    async fn process_route_safety_route_hop(
        &self,
        route: RPCOperationRoute,
        route_hop: RouteHop,
    ) -> Result<(), RPCError> {
        // Make sure hop count makes sense
        if route.safety_route.hop_count as usize > self.unlocked_inner.max_route_hop_count {
            return Err(RPCError::protocol(
                "Safety route hop count too high to process",
            ));
        }
        if route.safety_route.hop_count == 0 {
            return Err(RPCError::protocol(
                "Safety route hop count should not be zero if there are more hops",
            ));
        }
        if route_hop.next_hop.is_none() {
            return Err(RPCError::protocol("Safety route hop must have next hop"));
        }

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
                hops: SafetyRouteHops::Data(route_hop.next_hop.unwrap()),
            },
            operation: route.operation,
        };
        let next_hop_route_stmt = RPCStatement::new(RPCStatementDetail::Route(next_hop_route));

        // Send the next route statement
        network_result_value_or_log!(debug
            self.statement(Destination::direct(next_hop_nr), next_hop_route_stmt)
                .await? => {
                    return Err(RPCError::network("unable to send route statement for next safety route hop"));
                }
        );
        Ok(())
    }

    #[instrument(level = "trace", skip_all, err)]
    async fn process_route_private_route_hop(
        &self,
        route: RPCOperationRoute,
        private_route: PrivateRoute,
    ) -> Result<(), RPCError> {
        // Make sure hop count makes sense
        if route.safety_route.hop_count != 0 {
            return Err(RPCError::protocol(
                "Safety hop count should be zero if switched to private route",
            ));
        }
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

        // Get private route first hop (this is validated to not be None before calling this function)
        let first_hop = private_route.first_hop.as_ref().unwrap();

        // Get next hop node ref
        let next_hop_nr = match &first_hop.node {
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
                        pi.signed_node_info.clone(),
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
                hop_count: 0,
                hops: SafetyRouteHops::Private(private_route),
            },
            operation: route.operation,
        };
        let next_hop_route_stmt = RPCStatement::new(RPCStatementDetail::Route(next_hop_route));

        // Send the next route statement
        network_result_value_or_log!(debug
            self.statement(Destination::direct(next_hop_nr), next_hop_route_stmt)
                .await? => {
                    return Err(RPCError::network("unable to send route statement for private route hop"));
                }
        );

        Ok(())
    }

    #[instrument(level = "trace", skip_all, err)]
    async fn process_routed_operation(
        &self,
        sender_id: DHTKey,
        route: RPCOperationRoute,
        private_route: &PrivateRoute,
    ) -> Result<(), RPCError> {
        // Make sure hop count makes sense
        if route.safety_route.hop_count != 0 {
            return Err(RPCError::protocol(
                "Safety hop count should be zero if switched to private route",
            ));
        }
        if private_route.hop_count != 0 {
            return Err(RPCError::protocol(
                "Private route hop count should be zero if we are at the end",
            ));
        }

        let routed_operation = &route.operation;

        // Get sequencing preference
        if route.

        // If the private route public key is our node id, then this was sent via safety route to our node directly
        // so there will be no signatures to validate
        let opt_pr_info = if private_route.public_key == self.routing_table.node_id() {
            // the private route was a stub to our own node's secret
            // return our secret key
            Some((
                self.routing_table.node_id_secret(), // Figure out how we'd reply to this if it were a question
                SafetySelection::Unsafe(sequencing),
            ))
        } else {
            // Look up the private route and ensure it's one in our spec store
            let opt_signatures_valid = self.routing_table.with_route_spec_store(|rss, rti| {
                rss.with_route_spec_detail(&private_route.public_key, |rsd| {
                    // Ensure we have the right number of signatures
                    if routed_operation.signatures.len() != rsd.hops.len() - 1 {
                        // Wrong number of signatures
                        log_rpc!(debug "wrong number of signatures ({} should be {}) for routed operation on private route {}", routed_operation.signatures.len(), rsd.hops.len() - 1, private_route.public_key);    
                        return None;
                    }
                    // Validate signatures to ensure the route was handled by the nodes and not messed with
                    for (hop_n, hop_public_key) in rsd.hops.iter().enumerate() {
                        // The last hop is not signed, as the whole packet is signed
                        if hop_n == routed_operation.signatures.len() {
                            // Verify the node we received the routed operation from is the last hop in our route
                            if *hop_public_key != sender_id {
                                log_rpc!(debug "received routed operation from the wrong hop ({} should be {}) on private route {}", hop_public_key.encode(), sender_id.encode(), private_route.public_key);    
                                return None;
                            }
                        } else {
                            // Verify a signature for a hop node along the route
                            if let Err(e) = verify(
                                hop_public_key,
                                &routed_operation.data,
                                &routed_operation.signatures[hop_n],
                            ) {
                                log_rpc!(debug "failed to verify signature for hop {} at {} on private route {}", hop_n, hop_public_key, private_route.public_key);
                                return None;
                            }
                        }
                    }
                    // Correct signatures
                    Some((
                        rsd.secret_key,
                        SafetySelection::Safe(SafetySpec { preferred_route: todo!(), hop_count: todo!(), stability: todo!(), sequencing: todo!() })
                    ))
                })
            });
            opt_signatures_valid.ok_or_else(|| {
                RPCError::protocol("routed operation received on unallocated private route")
            })?
        };
        if opt_pr_info.is_none() {
            return Err(RPCError::protocol(
                "signatures did not validate for private route",
            ));
        }
        let (secret_key, safety_selection) = opt_pr_info.unwrap();

        // Now that things are valid, decrypt the routed operation with DEC(nonce, DH(the SR's public key, the PR's (or node's) secret)
        // xxx: punish nodes that send messages that fail to decrypt eventually
        let dh_secret = self
            .crypto
            .cached_dh(&route.safety_route.public_key, &secret_key)
            .map_err(RPCError::protocol)?;
        let body = Crypto::decrypt_aead(
            &routed_operation.data,
            &routed_operation.nonce,
            &dh_secret,
            None,
        )
        .map_err(RPCError::map_internal(
            "decryption of routed operation failed",
        ))?;

        // Pass message to RPC system
        self.enqueue_private_route_message(private_route.public_key, safety_selection, body)
            .map_err(RPCError::internal)?;

        Ok(())
    }

    #[instrument(level = "trace", skip(self, msg), err)]
    pub(crate) async fn process_route(&self, msg: RPCMessage) -> Result<(), RPCError> {
        // xxx do not process latency for routed messages
        
        // Get header detail, must be direct and not inside a route itself
        let (envelope, peer_noderef, connection_descriptor, routing_domain) = match msg.header.detail {
            RPCMessageHeaderDetail::Direct { envelope, peer_noderef, connection_descriptor, routing_domain } => (envelope, peer_noderef, connection_descriptor, routing_domain),
            RPCMessageHeaderDetail::PrivateRoute { private_route, safety_selection } => { return Err(RPCError::protocol("route operation can not be inside route")) },
        };

        // Get the statement
        let route = match msg.operation.into_kind() {
            RPCOperationKind::Statement(s) => match s.into_detail() {
                RPCStatementDetail::Route(s) => s,
                _ => panic!("not a route statement"),
            },
            _ => panic!("not a statement"),
        };

        // See what kind of safety route we have going on here
        match route.safety_route.hops {
            // There is a safety route hop
            SafetyRouteHops::Data(ref d) => {
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
                    .map_err(RPCError::map_internal(
                        "decryption of safety route hop failed",
                    ))?;
                let dec_blob_reader = capnp::message::Reader::new(
                    RPCMessageData {
                        contents: dec_blob_data,
                    },
                    Default::default(),
                );

                // Decode the blob appropriately
                if blob_tag == 1 {
                    // PrivateRoute
                    let private_route = {
                        let pr_reader = dec_blob_reader
                            .get_root::<veilid_capnp::private_route::Reader>()
                            .map_err(RPCError::protocol)?;
                        decode_private_route(&pr_reader)?
                    };

                    // Get the next hop node ref
                    if private_route.first_hop.is_some() {
                        // Switching to private route from safety route
                        self.process_route_private_route_hop(route, private_route)
                            .await?;
                    } else {
                        // Private route is empty, process routed operation
                        self.process_routed_operation(
                            envelope.get_sender_id(),
                            route,
                            &private_route,
                        )
                        .await?;
                    }
                } else if blob_tag == 0 {
                    // RouteHop
                    let route_hop = {
                        let rh_reader = dec_blob_reader
                            .get_root::<veilid_capnp::route_hop::Reader>()
                            .map_err(RPCError::protocol)?;
                        decode_route_hop(&rh_reader)?
                    };

                    self.process_route_safety_route_hop(route, route_hop)
                        .await?;
                } else {
                    return Err(RPCError::protocol("invalid blob tag"));
                }
            }
            // No safety route left, now doing private route
            SafetyRouteHops::Private(ref private_route) => {
                if let Some(first_hop) = &private_route.first_hop {
                    // See if we have a next hop to send to
                    let opt_next_first_hop = if let Some(next_hop) = &first_hop.next_hop {
                        // Decrypt the blob with DEC(nonce, DH(the PR's public key, this hop's secret)
                        let node_id_secret = self.routing_table.node_id_secret();
                        let dh_secret = self
                            .crypto
                            .cached_dh(&private_route.public_key, &node_id_secret)
                            .map_err(RPCError::protocol)?;
                        let dec_blob_data =
                            Crypto::decrypt_aead(&next_hop.blob, &next_hop.nonce, &dh_secret, None)
                                .map_err(RPCError::map_internal(
                                    "decryption of private route hop failed",
                                ))?;
                        let dec_blob_reader = capnp::message::Reader::new(
                            RPCMessageData {
                                contents: dec_blob_data,
                            },
                            Default::default(),
                        );

                        // Decode next RouteHop
                        let route_hop = {
                            let rh_reader = dec_blob_reader
                                .get_root::<veilid_capnp::route_hop::Reader>()
                                .map_err(RPCError::protocol)?;
                            decode_route_hop(&rh_reader)?
                        };
                        Some(route_hop)
                    } else {
                        // If the first hop has no RouteHopData, then this is a stub private route
                        // and we should just pass the operation to its final destination with
                        // an empty safety and private route
                        None
                    };

                    // Make next PrivateRoute and pass it on
                    let private_route = PrivateRoute {
                        public_key: private_route.public_key,
                        hop_count: private_route.hop_count - 1,
                        first_hop: opt_next_first_hop,
                    };
                    self.process_route_private_route_hop(route, private_route)
                        .await?;
                } else {
                    // No hops left, time to process the routed operation
                    self.process_routed_operation(
                        msg.header.envelope.get_sender_id(),
                        route,
                        private_route,
                    )
                    .await?;
                }
            }
        }

        Ok(())
    }
}
