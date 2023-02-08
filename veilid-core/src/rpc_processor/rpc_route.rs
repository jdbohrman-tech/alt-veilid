use super::*;

impl RPCProcessor {
    #[instrument(level = "trace", skip_all, err)]
    async fn process_route_safety_route_hop(
        &self,
        routed_operation: RoutedOperation,
        route_hop: RouteHop,
        safety_route: SafetyRoute,
    ) -> Result<NetworkResult<()>, RPCError> {
        // Make sure hop count makes sense
        if safety_route.hop_count as usize > self.unlocked_inner.max_route_hop_count {
            return Ok(NetworkResult::invalid_message(
                "Safety route hop count too high to process",
            ));
        }
        if safety_route.hop_count == 0 {
            return Ok(NetworkResult::invalid_message(
                "Safety route hop count should not be zero if there are more hops",
            ));
        }
        if route_hop.next_hop.is_none() {
            return Ok(NetworkResult::invalid_message(
                "Safety route hop must have next hop",
            ));
        }

        // Get next hop node ref
        let mut next_hop_nr = match route_hop.node {
            RouteNode::NodeId(id) => {
                //
                let Some(nr) = self.routing_table.lookup_node_ref(id.key) else {
                    return Ok(NetworkResult::invalid_message(format!("node hop {} not found", id.key)));
                };
                nr
            }
            RouteNode::PeerInfo(pi) => {
                //
                let Some(nr) = self.routing_table
                    .register_node_with_signed_node_info(
                        RoutingDomain::PublicInternet,
                        pi.node_id.key,
                        pi.signed_node_info,
                        false,
                    ) else 
                    {
                        return Ok(NetworkResult::invalid_message(format!(
                            "node hop {} could not be registered",
                            pi.node_id.key
                        )));
                    };
                nr
            }
        };

        // Apply sequencing preference
        next_hop_nr.set_sequencing(routed_operation.sequencing);

        // Pass along the route
        let next_hop_route = RPCOperationRoute {
            safety_route: SafetyRoute {
                public_key: safety_route.public_key,
                hop_count: safety_route.hop_count - 1,
                hops: SafetyRouteHops::Data(route_hop.next_hop.unwrap()),
            },
            operation: routed_operation,
        };
        let next_hop_route_stmt = RPCStatement::new(RPCStatementDetail::Route(next_hop_route));

        // Send the next route statement
        self.statement(Destination::direct(next_hop_nr), next_hop_route_stmt)
            .await
    }

    #[instrument(level = "trace", skip_all, err)]
    async fn process_route_private_route_hop(
        &self,
        routed_operation: RoutedOperation,
        next_route_node: RouteNode,
        safety_route_public_key: PublicKey,
        next_private_route: PrivateRoute,
    ) -> Result<NetworkResult<()>, RPCError> {
        // Make sure hop count makes sense
        if next_private_route.hop_count as usize > self.unlocked_inner.max_route_hop_count {
            return Err(RPCError::protocol(
                "Private route hop count too high to process",
            ));
        }

        // Get next hop node ref
        let mut next_hop_nr = match &next_route_node {
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

        // Apply sequencing preference
        next_hop_nr.set_sequencing(routed_operation.sequencing);

        // Pass along the route
        let next_hop_route = RPCOperationRoute {
            safety_route: SafetyRoute {
                public_key: safety_route_public_key,
                hop_count: 0,
                hops: SafetyRouteHops::Private(next_private_route),
            },
            operation: routed_operation,
        };
        let next_hop_route_stmt = RPCStatement::new(RPCStatementDetail::Route(next_hop_route));

        // Send the next route statement
        self.statement(Destination::direct(next_hop_nr), next_hop_route_stmt)
            .await
    }

    /// Process a routed operation that came in over a safety route but no private route
    ///
    /// Note: it is important that we never respond with a safety route to questions that come
    /// in without a private route. Giving away a safety route when the node id is known is
    /// a privacy violation!
    #[instrument(level = "trace", skip_all, err)]
    fn process_safety_routed_operation(
        &self,
        _detail: RPCMessageHeaderDetailDirect,
        routed_operation: RoutedOperation,
        remote_sr_pubkey: PublicKey,
    ) -> Result<NetworkResult<()>, RPCError> {

        // Now that things are valid, decrypt the routed operation with DEC(nonce, DH(the SR's public key, the PR's (or node's) secret)
        // xxx: punish nodes that send messages that fail to decrypt eventually? How to do this for safety routes?
        let node_id_secret = self.routing_table.node_id_secret();
        let dh_secret = self
            .crypto
            .cached_dh(&remote_sr_pubkey, &node_id_secret)
            .map_err(RPCError::protocol)?;
        let body = match Crypto::decrypt_aead(
            &routed_operation.data,
            &routed_operation.nonce,
            &dh_secret,
            None,
        ) {
            Ok(v) => v,
            Err(e) => {
                return Ok(NetworkResult::invalid_message(format!("decryption of routed operation failed: {}", e)));
            }
        };
        
        // Pass message to RPC system
        self.enqueue_safety_routed_message(remote_sr_pubkey, routed_operation.sequencing, body)
            .map_err(RPCError::internal)?;

        Ok(NetworkResult::value(()))
    }

    /// Process a routed operation that came in over both a safety route and a private route
    #[instrument(level = "trace", skip_all, err)]
    fn process_private_routed_operation(
        &self,
        detail: RPCMessageHeaderDetailDirect,
        routed_operation: RoutedOperation,
        remote_sr_pubkey: PublicKey,
        pr_pubkey: PublicKey,
    ) -> Result<NetworkResult<()>, RPCError> {
        // Get sender id
        let sender_id = detail.envelope.get_sender_id();

        // Look up the private route and ensure it's one in our spec store
        // Ensure the route is validated, and construct a return safetyspec that matches the inbound preferences
        let rss = self.routing_table.route_spec_store();
        let Some((secret_key, safety_spec)) = rss
            .with_signature_validated_route(
                &pr_pubkey,
                &routed_operation.signatures,
                &routed_operation.data,
                sender_id,
                |rsd| {
                    (
                        rsd.get_secret_key(),
                        SafetySpec {
                            preferred_route: Some(pr_pubkey),
                            hop_count: rsd.hop_count(),
                            stability: rsd.get_stability(),
                            sequencing: routed_operation.sequencing,
                        },
                    )
                }
            )
            else {
                return Ok(NetworkResult::invalid_message("signatures did not validate for private route"));
            };


        // Now that things are valid, decrypt the routed operation with DEC(nonce, DH(the SR's public key, the PR's (or node's) secret)
        // xxx: punish nodes that send messages that fail to decrypt eventually. How to do this for private routes?
        let dh_secret = self
            .crypto
            .cached_dh(&remote_sr_pubkey, &secret_key)
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
        self.enqueue_private_routed_message(remote_sr_pubkey, pr_pubkey, safety_spec, body)
            .map_err(RPCError::internal)?;

        Ok(NetworkResult::value(()))
    }

    #[instrument(level = "trace", skip_all, err)]
    fn process_routed_operation(
        &self,
        detail: RPCMessageHeaderDetailDirect,
        routed_operation: RoutedOperation,
        remote_sr_pubkey: PublicKey,
        pr_pubkey: PublicKey,
    ) -> Result<NetworkResult<()>, RPCError> {

        // If the private route public key is our node id, then this was sent via safety route to our node directly
        // so there will be no signatures to validate
        if pr_pubkey == self.routing_table.node_id() {
            // The private route was a stub
            self.process_safety_routed_operation(detail, routed_operation, remote_sr_pubkey)
        } else {
            // Both safety and private routes used, should reply with a safety route
            self.process_private_routed_operation(
                detail,
                routed_operation,
                remote_sr_pubkey,
                pr_pubkey,
            )
        }
    }
    #[instrument(level = "trace", skip_all, err)]
    pub(crate) async fn process_private_route_first_hop(
        &self,
        mut routed_operation: RoutedOperation,
        sr_pubkey: PublicKey,
        mut private_route: PrivateRoute,
    ) -> Result<NetworkResult<()>, RPCError> {
        let Some(pr_first_hop) = private_route.pop_first_hop() else {
            return Ok(NetworkResult::invalid_message("switching from safety route to private route requires first hop"));
        };

        // Check for loopback test where private route is the same as safety route
        if sr_pubkey == private_route.public_key {
            // If so, we're going to turn this thing right around without transiting the network
            let PrivateRouteHops::Data(route_hop_data) = private_route.hops else {
                return Ok(NetworkResult::invalid_message("Loopback test requires hops"));
            };

            // Decrypt route hop data
            let route_hop = network_result_try!(self.decrypt_private_route_hop_data(&route_hop_data, &private_route.public_key, &mut routed_operation)?);
                    
            // Ensure hop count > 0
            if private_route.hop_count == 0 {
                return Ok(NetworkResult::invalid_message(
                    "route should not be at the end",
                ));
            }

            // Make next PrivateRoute and pass it on
            return self.process_route_private_route_hop(
                routed_operation,
                route_hop.node,
                sr_pubkey,
                PrivateRoute {
                    public_key: private_route.public_key,
                    hop_count: private_route.hop_count - 1,
                    hops: route_hop
                        .next_hop
                        .map(|rhd| PrivateRouteHops::Data(rhd))
                        .unwrap_or(PrivateRouteHops::Empty),
                },
            )
            .await;
        }

        // Switching to private route from safety route
        self.process_route_private_route_hop(
            routed_operation,
            pr_first_hop,
            sr_pubkey,
            private_route,
        )
        .await
    }

    /// Decrypt route hop data and sign routed operation
    pub(crate) fn decrypt_private_route_hop_data(&self, route_hop_data: &RouteHopData, pr_pubkey: &PublicKey, route_operation: &mut RoutedOperation) -> Result<NetworkResult<RouteHop>, RPCError>
    {
        // Decrypt the blob with DEC(nonce, DH(the PR's public key, this hop's secret)
        let node_id_secret = self.routing_table.node_id_secret();
        let dh_secret = self
            .crypto
            .cached_dh(&pr_pubkey, &node_id_secret)
            .map_err(RPCError::protocol)?;
        let dec_blob_data = match Crypto::decrypt_aead(
            &route_hop_data.blob,
            &route_hop_data.nonce,
            &dh_secret,
            None,
        ) {
            Ok(v) => v,
            Err(e) => {
                return Ok(NetworkResult::invalid_message(format!("unable to decrypt private route hop data: {}", e)));
            }
        };
        let dec_blob_reader = RPCMessageData::new(dec_blob_data).get_reader()?;

        // Decode next RouteHop
        let route_hop = {
            let rh_reader = dec_blob_reader
                .get_root::<veilid_capnp::route_hop::Reader>()
                .map_err(RPCError::protocol)?;
            decode_route_hop(&rh_reader)?
        };

        // Sign the operation if this is not our last hop
        // as the last hop is already signed by the envelope
        if route_hop.next_hop.is_some() {
            let node_id = self.routing_table.node_id();
            let node_id_secret = self.routing_table.node_id_secret();
            let sig = sign(&node_id, &node_id_secret, &route_operation.data)
                .map_err(RPCError::internal)?;
            route_operation.signatures.push(sig);
        }

        Ok(NetworkResult::value(route_hop))
    }

    #[instrument(level = "trace", skip(self, msg), ret, err)]
    pub(crate) async fn process_route(
        &self,
        msg: RPCMessage,
    ) -> Result<NetworkResult<()>, RPCError> {
        // Get header detail, must be direct and not inside a route itself
        let detail = match msg.header.detail {
            RPCMessageHeaderDetail::Direct(detail) => detail,
            RPCMessageHeaderDetail::SafetyRouted(_) | RPCMessageHeaderDetail::PrivateRouted(_) => {
                return Ok(NetworkResult::invalid_message(
                    "route operation can not be inside route",
                ))
            }
        };

        // Get the statement
        let mut route = match msg.operation.into_kind() {
            RPCOperationKind::Statement(s) => match s.into_detail() {
                RPCStatementDetail::Route(s) => s,
                _ => panic!("not a route statement"),
            },
            _ => panic!("not a statement"),
        };

        // Process routed operation version
        // xxx switch this to a Crypto trait factory method per issue#140
        if route.operation.version != MAX_CRYPTO_VERSION {
            return Ok(NetworkResult::invalid_message(
                "routes operation crypto is not valid version",
            ));
        }

        // See what kind of safety route we have going on here
        match route.safety_route.hops {
            // There is a safety route hop
            SafetyRouteHops::Data(ref route_hop_data) => {
                // Decrypt the blob with DEC(nonce, DH(the SR's public key, this hop's secret)
                let node_id_secret = self.routing_table.node_id_secret();
                let dh_secret = self
                    .crypto
                    .cached_dh(&route.safety_route.public_key, &node_id_secret)
                    .map_err(RPCError::protocol)?;
                let mut dec_blob_data = Crypto::decrypt_aead(&route_hop_data.blob, &route_hop_data.nonce, &dh_secret, None)
                    .map_err(RPCError::protocol)?;

                // See if this is last hop in safety route, if so, we're decoding a PrivateRoute not a RouteHop
                let Some(dec_blob_tag) = dec_blob_data.pop() else {
                    return Ok(NetworkResult::invalid_message("no bytes in blob"));
                };

                let dec_blob_reader = RPCMessageData::new(dec_blob_data).get_reader()?;

                // Decode the blob appropriately
                if dec_blob_tag == 1 {
                    // PrivateRoute
                    let private_route = {
                        let pr_reader = dec_blob_reader
                            .get_root::<veilid_capnp::private_route::Reader>()
                            .map_err(RPCError::protocol)?;
                        decode_private_route(&pr_reader)?
                    };

                    // Switching from full safety route to private route first hop
                    network_result_try!(self.process_private_route_first_hop(
                        route.operation,
                        route.safety_route.public_key,
                        private_route,
                    )
                    .await?);
                } else if dec_blob_tag == 0 {
                    // RouteHop
                    let route_hop = {
                        let rh_reader = dec_blob_reader
                            .get_root::<veilid_capnp::route_hop::Reader>()
                            .map_err(RPCError::protocol)?;
                        decode_route_hop(&rh_reader)?
                    };

                    // Continue the full safety route with another hop
                    network_result_try!(self.process_route_safety_route_hop(route.operation, route_hop, route.safety_route)
                        .await?);
                } else {
                    return Ok(NetworkResult::invalid_message("invalid blob tag"));
                }
            }
            // No safety route left, now doing private route
            SafetyRouteHops::Private(private_route) => {
                // See if we have a hop, if not, we are at the end of the private route
                match private_route.hops {
                    PrivateRouteHops::FirstHop(_) => {
                        // Safety route was a stub, start with the beginning of the private route
                        network_result_try!(self.process_private_route_first_hop(
                            route.operation,
                            route.safety_route.public_key,
                            private_route,
                        )
                        .await?);
                    }
                    PrivateRouteHops::Data(route_hop_data) => {
                        
                        // Decrypt route hop data
                        let route_hop = network_result_try!(self.decrypt_private_route_hop_data(&route_hop_data, &private_route.public_key, &mut route.operation)?);
                        
                        // Ensure hop count > 0
                        if private_route.hop_count == 0 {
                            return Ok(NetworkResult::invalid_message(
                                "route should not be at the end",
                            ));
                        }

                        // Make next PrivateRoute and pass it on
                        network_result_try!(self.process_route_private_route_hop(
                            route.operation,
                            route_hop.node,
                            route.safety_route.public_key,
                            PrivateRoute {
                                public_key: private_route.public_key,
                                hop_count: private_route.hop_count - 1,
                                hops: route_hop
                                    .next_hop
                                    .map(|rhd| PrivateRouteHops::Data(rhd))
                                    .unwrap_or(PrivateRouteHops::Empty),
                            },
                        )
                        .await?);
                    }
                    PrivateRouteHops::Empty => {
                        // Ensure hop count == 0
                        if private_route.hop_count != 0 {
                            return Ok(NetworkResult::invalid_message(
                                "route should be at the end",
                            ));
                        }
                        if route.safety_route.hop_count != 0 {
                            return Ok(NetworkResult::invalid_message(
                                "Safety hop count should be zero if switched to private route",
                            ));
                        }

                        // No hops left, time to process the routed operation
                        network_result_try!(self.process_routed_operation(
                            detail,
                            route.operation,
                            route.safety_route.public_key,
                            private_route.public_key,
                        )?);
                    }
                }
            }
        }

        Ok(NetworkResult::value(()))
    }
}
