use super::*;

impl RPCProcessor {
    #[cfg_attr(feature="verbose-tracing", instrument(level = "trace", skip_all, err))]
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
        let Some(mut next_hop_nr) = route_hop.node.node_ref(self.routing_table.clone(), safety_route.public_key.kind) else {
            return Err(RPCError::network(format!(
                "could not get route node hop ref: {}",
                route_hop.node.describe(safety_route.public_key.kind)
            )));
        };

        // Apply sequencing preference
        next_hop_nr.set_sequencing(routed_operation.sequencing());

        // Pass along the route
        let next_hop_route = RPCOperationRoute::new(
            SafetyRoute {
                public_key: safety_route.public_key,
                hop_count: safety_route.hop_count - 1,
                hops: SafetyRouteHops::Data(route_hop.next_hop.unwrap()),
            },
            routed_operation,
        );
        let next_hop_route_stmt = RPCStatement::new(RPCStatementDetail::Route(next_hop_route));

        // Send the next route statement
        self.statement(Destination::direct(next_hop_nr), next_hop_route_stmt)
            .await
    }

    #[cfg_attr(feature="verbose-tracing", instrument(level = "trace", skip_all, err))]
    async fn process_route_private_route_hop(
        &self,
        routed_operation: RoutedOperation,
        next_route_node: RouteNode,
        safety_route_public_key: TypedKey,
        next_private_route: PrivateRoute,
    ) -> Result<NetworkResult<()>, RPCError> {
        // Make sure hop count makes sense
        if next_private_route.hop_count as usize > self.unlocked_inner.max_route_hop_count {
            return Ok(NetworkResult::invalid_message(
                "Private route hop count too high to process",
            ));
        }

        // Get next hop node ref
        let Some(mut next_hop_nr) = next_route_node.node_ref(self.routing_table.clone(), safety_route_public_key.kind) else {
            return Err(RPCError::network(format!(
                "could not get route node hop ref: {}",
                next_route_node.describe(safety_route_public_key.kind)
            )));
        };

        // Apply sequencing preference
        next_hop_nr.set_sequencing(routed_operation.sequencing());

        // Pass along the route
        let next_hop_route = RPCOperationRoute::new(
            SafetyRoute {
                public_key: safety_route_public_key,
                hop_count: 0,
                hops: SafetyRouteHops::Private(next_private_route),
            },
            routed_operation,
        );
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
    #[cfg_attr(feature="verbose-tracing", instrument(level = "trace", skip_all, err))]
    fn process_safety_routed_operation(
        &self,
        detail: RPCMessageHeaderDetailDirect,
        vcrypto: CryptoSystemVersion,
        routed_operation: RoutedOperation,
        remote_sr_pubkey: TypedKey,
    ) -> Result<NetworkResult<()>, RPCError> {
        // Now that things are valid, decrypt the routed operation with DEC(nonce, DH(the SR's public key, the PR's (or node's) secret)
        // xxx: punish nodes that send messages that fail to decrypt eventually? How to do this for safety routes?
        let node_id_secret = self.routing_table.node_id_secret_key(remote_sr_pubkey.kind);
        let Ok(dh_secret) = vcrypto.cached_dh(&remote_sr_pubkey.value, &node_id_secret) else {
            return Ok(NetworkResult::invalid_message("dh failed for remote safety route for safety routed operation"));
        };
        let body = match vcrypto.decrypt_aead(
            routed_operation.data(),
            routed_operation.nonce(),
            &dh_secret,
            None,
        ) {
            Ok(v) => v,
            Err(e) => {
                return Ok(NetworkResult::invalid_message(format!(
                    "decryption of routed operation failed: {}",
                    e
                )));
            }
        };

        // Pass message to RPC system
        self.enqueue_safety_routed_message(
            detail,
            remote_sr_pubkey.value,
            routed_operation.sequencing(),
            body,
        )
        .map_err(RPCError::internal)?;

        Ok(NetworkResult::value(()))
    }

    /// Process a routed operation that came in over both a safety route and a private route
    #[cfg_attr(feature="verbose-tracing", instrument(level = "trace", skip_all, err))]
    fn process_private_routed_operation(
        &self,
        detail: RPCMessageHeaderDetailDirect,
        vcrypto: CryptoSystemVersion,
        routed_operation: RoutedOperation,
        remote_sr_pubkey: TypedKey,
        pr_pubkey: TypedKey,
    ) -> Result<NetworkResult<()>, RPCError> {
        // Get sender id of the peer with the crypto kind of the route
        let Some(sender_id) = detail.peer_noderef.node_ids().get(pr_pubkey.kind) else {
            return Ok(NetworkResult::invalid_message("route node doesnt have a required crypto kind for routed operation"));
        };

        // Look up the private route and ensure it's one in our spec store
        // Ensure the route is validated, and construct a return safetyspec that matches the inbound preferences
        let rss = self.routing_table.route_spec_store();
        let preferred_route = rss.get_route_id_for_key(&pr_pubkey.value);
        let Some((secret_key, safety_spec)) = rss
            .with_signature_validated_route(
                &pr_pubkey,
                routed_operation.signatures(),
                routed_operation.data(),
                sender_id.value,
                |rssd, rsd| { 
                    (
                        rsd.secret_key,
                        SafetySpec {
                            preferred_route,
                            hop_count: rssd.hop_count(),
                            stability: rssd.get_stability(),
                            sequencing: routed_operation.sequencing(),
                        },
                    )
                }
            )
            else {
                return Ok(NetworkResult::invalid_message("signatures did not validate for private route"));
            };

        // Now that things are valid, decrypt the routed operation with DEC(nonce, DH(the SR's public key, the PR's (or node's) secret)
        // xxx: punish nodes that send messages that fail to decrypt eventually. How to do this for private routes?
        let Ok(dh_secret) = vcrypto.cached_dh(&remote_sr_pubkey.value, &secret_key) else {
            return Ok(NetworkResult::invalid_message("dh failed for remote safety route for private routed operation"));
        };
        let Ok(body) = vcrypto
            .decrypt_aead(
                routed_operation.data(),
                routed_operation.nonce(),
                &dh_secret,
                None,
            ) else {
                return Ok(NetworkResult::invalid_message("decryption of routed operation failed"));
            };

        // Pass message to RPC system
        self.enqueue_private_routed_message(
            detail,
            remote_sr_pubkey.value,
            pr_pubkey.value,
            safety_spec,
            body,
        )
        .map_err(RPCError::internal)?;

        Ok(NetworkResult::value(()))
    }

    #[cfg_attr(feature="verbose-tracing", instrument(level = "trace", skip_all, err))]
    fn process_routed_operation(
        &self,
        detail: RPCMessageHeaderDetailDirect,
        vcrypto: CryptoSystemVersion,
        routed_operation: RoutedOperation,
        remote_sr_pubkey: TypedKey,
        pr_pubkey: TypedKey,
    ) -> Result<NetworkResult<()>, RPCError> {
        // If the private route public key is our node id, then this was sent via safety route to our node directly
        // so there will be no signatures to validate
        if self.routing_table.node_ids().contains(&pr_pubkey) {
            // The private route was a stub
            self.process_safety_routed_operation(
                detail,
                vcrypto,
                routed_operation,
                remote_sr_pubkey,
            )
        } else {
            // Both safety and private routes used, should reply with a safety route
            self.process_private_routed_operation(
                detail,
                vcrypto,
                routed_operation,
                remote_sr_pubkey,
                pr_pubkey,
            )
        }
    }
    #[cfg_attr(feature="verbose-tracing", instrument(level = "trace", skip_all, err))]
    pub(crate) async fn process_private_route_first_hop(
        &self,
        mut routed_operation: RoutedOperation,
        sr_pubkey: TypedKey,
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
            let route_hop = network_result_try!(self.decrypt_private_route_hop_data(
                &route_hop_data,
                &private_route.public_key,
                &mut routed_operation
            )?);

            // Ensure hop count > 0
            if private_route.hop_count == 0 {
                return Ok(NetworkResult::invalid_message(
                    "route should not be at the end",
                ));
            }

            // Make next PrivateRoute and pass it on
            return self
                .process_route_private_route_hop(
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
    pub(crate) fn decrypt_private_route_hop_data(
        &self,
        route_hop_data: &RouteHopData,
        pr_pubkey: &TypedKey,
        route_operation: &mut RoutedOperation,
    ) -> Result<NetworkResult<RouteHop>, RPCError> {
        // Get crypto kind
        let crypto_kind = pr_pubkey.kind;
        let Some(vcrypto) = self.crypto.get(crypto_kind) else {
            return Ok(NetworkResult::invalid_message(
                "private route hop data crypto is not supported",
            ));
        };

        // Decrypt the blob with DEC(nonce, DH(the PR's public key, this hop's secret)
        let node_id_secret = self.routing_table.node_id_secret_key(crypto_kind);
        let dh_secret = vcrypto
            .cached_dh(&pr_pubkey.value, &node_id_secret)
            .map_err(RPCError::protocol)?;
        let dec_blob_data = match vcrypto.decrypt_aead(
            &route_hop_data.blob,
            &route_hop_data.nonce,
            &dh_secret,
            None,
        ) {
            Ok(v) => v,
            Err(e) => {
                return Ok(NetworkResult::invalid_message(format!(
                    "unable to decrypt private route hop data: {}",
                    e
                )));
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
        
        // Validate the RouteHop
        route_hop.validate(self.crypto.clone()).map_err(RPCError::protocol)?;

        // Sign the operation if this is not our last hop
        // as the last hop is already signed by the envelope
        if route_hop.next_hop.is_some() {
            let node_id = self.routing_table.node_id(crypto_kind);
            let node_id_secret = self.routing_table.node_id_secret_key(crypto_kind);
            let sig = vcrypto
                .sign(&node_id.value, &node_id_secret, route_operation.data())
                .map_err(RPCError::internal)?;
            route_operation.add_signature(sig);
        }

        Ok(NetworkResult::value(route_hop))
    }

    #[cfg_attr(feature="verbose-tracing", instrument(level = "trace", skip(self), ret, err))]
    pub(crate) async fn process_route(
        &self,
        msg: RPCMessage,
    ) -> Result<NetworkResult<()>, RPCError> {
        // Ignore if disabled
        let routing_table = self.routing_table();
        if !routing_table.has_valid_network_class(msg.header.routing_domain()) {
            return Ok(NetworkResult::service_unavailable(
                "can't route without valid network class",
            ));
        }

        let opi = routing_table.get_own_peer_info(msg.header.routing_domain());
        if !opi
            .signed_node_info()
            .node_info()
            .has_capability(CAP_ROUTE)
        {
            return Ok(NetworkResult::service_unavailable(
                "route is not available",
            ));
        }
        
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
        let (_,_,_,kind) = msg.operation.destructure();
        let route = match kind {
            RPCOperationKind::Statement(s) => match s.destructure() {
                RPCStatementDetail::Route(s) => s,
                _ => panic!("not a route statement"),
            },
            _ => panic!("not a statement"),
        };

        // Get crypto kind
        let crypto_kind = route.safety_route().crypto_kind();
        let Some(vcrypto) = self.crypto.get(crypto_kind) else {
            return Ok(NetworkResult::invalid_message(
                "routed operation crypto is not supported",
            ));
        };

        // See what kind of safety route we have going on here
        let (safety_route, mut routed_operation) = route.destructure();
        match safety_route.hops {
            // There is a safety route hop
            SafetyRouteHops::Data(ref route_hop_data) => {
                // Decrypt the blob with DEC(nonce, DH(the SR's public key, this hop's secret)
                let node_id_secret = self.routing_table.node_id_secret_key(crypto_kind);
                let Ok(dh_secret) = vcrypto
                    .cached_dh(&safety_route.public_key.value, &node_id_secret) else {
                    return Ok(NetworkResult::invalid_message("dh failed for safety route hop"));
                };
                let Ok(mut dec_blob_data) = vcrypto
                    .decrypt_aead(
                        &route_hop_data.blob,
                        &route_hop_data.nonce,
                        &dh_secret,
                        None,
                    )
                    else {
                    return Ok(NetworkResult::invalid_message("failed to decrypt route hop data for safety route hop"));
                };

                // See if this is last hop in safety route, if so, we're decoding a PrivateRoute not a RouteHop
                let Some(dec_blob_tag) = dec_blob_data.pop() else {
                    return Ok(NetworkResult::invalid_message("no bytes in blob"));
                };

                let Ok(dec_blob_reader) = RPCMessageData::new(dec_blob_data).get_reader() else {
                    return Ok(NetworkResult::invalid_message("Failed to decode RPCMessageData from blob"));
                };

                // Decode the blob appropriately
                if dec_blob_tag == 1 {
                    // PrivateRoute
                    let private_route = {
                        let Ok(pr_reader) = dec_blob_reader
                            .get_root::<veilid_capnp::private_route::Reader>() else {
                            return Ok(NetworkResult::invalid_message("failed to get private route reader for blob"));
                        };
                        let Ok(private_route) = decode_private_route(&pr_reader) else {
                            return Ok(NetworkResult::invalid_message("failed to decode private route"));
                        };
                        private_route
                    };
                    
                    // Validate the private route
                    if let Err(_) = private_route.validate(self.crypto.clone()) {
                        return Ok(NetworkResult::invalid_message("failed to validate private route"));
                    }

                    // Switching from full safety route to private route first hop
                    network_result_try!(
                        self.process_private_route_first_hop(
                            routed_operation,
                            safety_route.public_key,
                            private_route,
                        )
                        .await?
                    );
                } else if dec_blob_tag == 0 {
                    // RouteHop
                    let route_hop = {
                        let Ok(rh_reader) = dec_blob_reader
                            .get_root::<veilid_capnp::route_hop::Reader>() else {
                            return Ok(NetworkResult::invalid_message("failed to get route hop reader for blob"));
                        };
                        let Ok(route_hop) = decode_route_hop(&rh_reader) else {
                            return Ok(NetworkResult::invalid_message("failed to decode route hop"));
                        };
                        route_hop
                    };

                    // Validate the route hop
                    if let Err(_) = route_hop.validate(self.crypto.clone()) {
                        return Ok(NetworkResult::invalid_message("failed to validate route hop"));
                    }

                    // Continue the full safety route with another hop
                    network_result_try!(
                        self.process_route_safety_route_hop(
                            routed_operation,
                            route_hop,
                            safety_route
                        )
                        .await?
                    );
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
                        network_result_try!(
                            self.process_private_route_first_hop(
                                routed_operation,
                                safety_route.public_key,
                                private_route,
                            )
                            .await?
                        );
                    }
                    PrivateRouteHops::Data(route_hop_data) => {
                        // Decrypt route hop data
                        let route_hop = network_result_try!(self.decrypt_private_route_hop_data(
                            &route_hop_data,
                            &private_route.public_key,
                            &mut routed_operation
                        )?);

                        // Ensure hop count > 0
                        if private_route.hop_count == 0 {
                            return Ok(NetworkResult::invalid_message(
                                "route should not be at the end",
                            ));
                        }

                        // Make next PrivateRoute and pass it on
                        network_result_try!(
                            self.process_route_private_route_hop(
                                routed_operation,
                                route_hop.node,
                                safety_route.public_key,
                                PrivateRoute {
                                    public_key: private_route.public_key,
                                    hop_count: private_route.hop_count - 1,
                                    hops: route_hop
                                        .next_hop
                                        .map(|rhd| PrivateRouteHops::Data(rhd))
                                        .unwrap_or(PrivateRouteHops::Empty),
                                },
                            )
                            .await?
                        );
                    }
                    PrivateRouteHops::Empty => {
                        // Ensure hop count == 0
                        if private_route.hop_count != 0 {
                            return Ok(NetworkResult::invalid_message(
                                "route should be at the end",
                            ));
                        }
                        if safety_route.hop_count != 0 {
                            return Ok(NetworkResult::invalid_message(
                                "Safety hop count should be zero if switched to private route",
                            ));
                        }

                        // No hops left, time to process the routed operation
                        network_result_try!(self.process_routed_operation(
                            detail,
                            vcrypto,
                            routed_operation,
                            safety_route.public_key,
                            private_route.public_key,
                        )?);
                    }
                }
            }
        }

        Ok(NetworkResult::value(()))
    }
}
