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
        mut route: RPCOperationRoute,
        next_private_route: PrivateRoute,
    ) -> Result<(), RPCError> {
        // Make sure hop count makes sense
        if route.safety_route.hop_count != 0 {
            return Err(RPCError::protocol(
                "Safety hop count should be zero if switched to private route",
            ));
        }
        if next_private_route.hop_count as usize > self.unlocked_inner.max_route_hop_count {
            return Err(RPCError::protocol(
                "Private route hop count too high to process",
            ));
        }

        // Get private route first hop (this is validated to not be None before calling this function)
        let first_hop = next_private_route.first_hop.as_ref().unwrap();

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

        // Sign the operation if this is not our last hop
        // as the last hop is already signed by the envelope
        if next_private_route.hop_count != 0 {
            let node_id = self.routing_table.node_id();
            let node_id_secret = self.routing_table.node_id_secret();
            let sig = sign(&node_id, &node_id_secret, &route.operation.data)
                .map_err(RPCError::internal)?;
            route.operation.signatures.push(sig);
        }

        // Pass along the route
        let next_hop_route = RPCOperationRoute {
            safety_route: SafetyRoute {
                public_key: route.safety_route.public_key,
                hop_count: 0,
                hops: SafetyRouteHops::Private(next_private_route),
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
        detail: RPCMessageHeaderDetailDirect,
        routed_operation: RoutedOperation,
        safety_route: &SafetyRoute,
        private_route: &PrivateRoute,
    ) -> Result<(), RPCError> {
        // Make sure hop count makes sense
        if safety_route.hop_count != 0 {
            return Err(RPCError::protocol(
                "Safety hop count should be zero if switched to private route",
            ));
        }
        if private_route.hop_count != 0 {
            return Err(RPCError::protocol(
                "Private route hop count should be zero if we are at the end",
            ));
        }

        // If the private route public key is our node id, then this was sent via safety route to our node directly
        // so there will be no signatures to validate
        let (secret_key, safety_selection) = if private_route.public_key
            == self.routing_table.node_id()
        {
            // The private route was a stub
            // Return our secret key and an appropriate safety selection
            //
            // Note: it is important that we never respond with a safety route to questions that come
            // in without a private route. Giving away a safety route when the node id is known is
            // a privacy violation!

            // Get sequencing preference
            let sequencing = if detail
                .connection_descriptor
                .protocol_type()
                .is_connection_oriented()
            {
                Sequencing::EnsureOrdered
            } else {
                Sequencing::NoPreference
            };
            (
                self.routing_table.node_id_secret(),
                SafetySelection::Unsafe(sequencing),
            )
        } else {
            // Get sender id
            let sender_id = detail.envelope.get_sender_id();

            // Look up the private route and ensure it's one in our spec store
            let rss = self.routing_table.route_spec_store();
            rss.validate_signatures(
                &private_route.public_key,
                &routed_operation.signatures,
                &routed_operation.data,
                sender_id,
            )
            .map_err(RPCError::protocol)?
            .ok_or_else(|| RPCError::protocol("signatures did not validate for private route"))?
        };

        // Now that things are valid, decrypt the routed operation with DEC(nonce, DH(the SR's public key, the PR's (or node's) secret)
        // xxx: punish nodes that send messages that fail to decrypt eventually
        let dh_secret = self
            .crypto
            .cached_dh(&safety_route.public_key, &secret_key)
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
        // Get header detail, must be direct and not inside a route itself
        let detail = match msg.header.detail {
            RPCMessageHeaderDetail::Direct(detail) => detail,
            RPCMessageHeaderDetail::PrivateRoute(_) => {
                return Err(RPCError::protocol(
                    "route operation can not be inside route",
                ))
            }
        };

        // Get the statement
        let route = match msg.operation.into_kind() {
            RPCOperationKind::Statement(s) => match s.into_detail() {
                RPCStatementDetail::Route(s) => s,
                _ => panic!("not a route statement"),
            },
            _ => panic!("not a statement"),
        };

        // Process routed operation version
        // xxx switch this to a Crypto trait factory method per issue#140
        if route.operation.version != MAX_CRYPTO_VERSION {
            return Err(RPCError::protocol(
                "routes operation crypto is not valid version",
            ));
        }

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
                            detail,
                            route.operation,
                            &route.safety_route,
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
                    let next_private_route = PrivateRoute {
                        public_key: private_route.public_key,
                        hop_count: private_route.hop_count - 1,
                        first_hop: opt_next_first_hop,
                    };
                    self.process_route_private_route_hop(route, next_private_route)
                        .await?;
                } else {
                    // No hops left, time to process the routed operation
                    self.process_routed_operation(
                        detail,
                        route.operation,
                        &route.safety_route,
                        private_route,
                    )
                    .await?;
                }
            }
        }

        Ok(())
    }
}
