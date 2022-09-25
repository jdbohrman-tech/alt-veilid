use super::*;

impl RPCProcessor {
    //////////////////////////////////////////////////////////////////////
    fn compile_safety_route(
        &self,
        safety_route_spec: Arc<SafetyRouteSpec>,
        private_route: PrivateRoute,
    ) -> Result<SafetyRoute, RPCError> {
        // Ensure the total hop count isn't too long for our config
        let pr_hopcount = private_route.hop_count as usize;
        let sr_hopcount = safety_route_spec.hops.len();
        let hopcount = 1 + sr_hopcount + pr_hopcount;
        if hopcount > self.unlocked_inner.max_route_hop_count {
            return Err(RPCError::internal("hop count too long for route"));
        }

        // Create hops
        let hops = if sr_hopcount == 0 {
            SafetyRouteHops::Private(private_route)
        } else {
            // start last blob-to-encrypt data off as private route
            let mut blob_data = {
                let mut pr_message = ::capnp::message::Builder::new_default();
                let mut pr_builder = pr_message.init_root::<veilid_capnp::private_route::Builder>();
                encode_private_route(&private_route, &mut pr_builder)?;
                let mut blob_data = builder_to_vec(pr_message)?;

                // append the private route tag so we know how to decode it later
                blob_data.push(1u8);
                blob_data
            };

            // Encode each hop from inside to outside
            // skips the outermost hop since that's entering the
            // safety route and does not include the dialInfo
            // (outer hop is a RouteHopData, not a RouteHop).
            // Each loop mutates 'nonce', and 'blob_data'
            let mut nonce = Crypto::get_random_nonce();
            for h in (1..sr_hopcount).rev() {
                // Get blob to encrypt for next hop
                blob_data = {
                    // Encrypt the previous blob ENC(nonce, DH(PKhop,SKsr))
                    let dh_secret = self
                        .crypto
                        .cached_dh(
                            &safety_route_spec.hops[h].dial_info.node_id.key,
                            &safety_route_spec.secret_key,
                        )
                        .map_err(RPCError::map_internal("dh failed"))?;
                    let enc_msg_data =
                        Crypto::encrypt_aead(blob_data.as_slice(), &nonce, &dh_secret, None)
                            .map_err(RPCError::map_internal("encryption failed"))?;

                    // Make route hop data
                    let route_hop_data = RouteHopData {
                        nonce,
                        blob: enc_msg_data,
                    };

                    // Make route hop
                    let route_hop = RouteHop {
                        dial_info: safety_route_spec.hops[h].dial_info.clone(),
                        next_hop: Some(route_hop_data),
                    };

                    // Make next blob from route hop
                    let mut rh_message = ::capnp::message::Builder::new_default();
                    let mut rh_builder = rh_message.init_root::<veilid_capnp::route_hop::Builder>();
                    encode_route_hop(&route_hop, &mut rh_builder)?;
                    let mut blob_data = builder_to_vec(rh_message)?;

                    // Append the route hop tag so we know how to decode it later
                    blob_data.push(0u8);
                    blob_data
                };

                // Make another nonce for the next hop
                nonce = Crypto::get_random_nonce();
            }

            // Encode first RouteHopData
            let dh_secret = self
                .crypto
                .cached_dh(
                    &safety_route_spec.hops[0].dial_info.node_id.key,
                    &safety_route_spec.secret_key,
                )
                .map_err(RPCError::map_internal("dh failed"))?;
            let enc_msg_data = Crypto::encrypt_aead(blob_data.as_slice(), &nonce, &dh_secret, None)
                .map_err(RPCError::map_internal("encryption failed"))?;

            let route_hop_data = RouteHopData {
                nonce,
                blob: enc_msg_data,
            };

            SafetyRouteHops::Data(route_hop_data)
        };

        // Build safety route
        let safety_route = SafetyRoute {
            public_key: safety_route_spec.public_key,
            hop_count: safety_route_spec.hops.len() as u8,
            hops,
        };

        Ok(safety_route)
    }

    // Wrap an operation inside a route
    pub(super) fn wrap_with_route(
        &self,
        safety_route_spec: Option<Arc<SafetyRouteSpec>>,
        private_route: PrivateRoute,
        message_data: Vec<u8>,
    ) -> Result<Vec<u8>, RPCError> {
        // Encrypt routed operation
        // Xmsg + ENC(Xmsg, DH(PKapr, SKbsr))
        let nonce = Crypto::get_random_nonce();
        let safety_route_spec =
            safety_route_spec.unwrap_or_else(|| Arc::new(SafetyRouteSpec::new()));
        let dh_secret = self
            .crypto
            .cached_dh(&private_route.public_key, &safety_route_spec.secret_key)
            .map_err(RPCError::map_internal("dh failed"))?;
        let enc_msg_data = Crypto::encrypt_aead(&message_data, &nonce, &dh_secret, None)
            .map_err(RPCError::map_internal("encryption failed"))?;

        // Compile the safety route with the private route
        let safety_route = self.compile_safety_route(safety_route_spec, private_route)?;

        // Make the routed operation
        let operation = RoutedOperation::new(nonce, enc_msg_data);

        // Prepare route operation
        let route = RPCOperationRoute {
            safety_route,
            operation,
        };
        let operation =
            RPCOperation::new_statement(RPCStatement::new(RPCStatementDetail::Route(route)), None);

        // Convert message to bytes and return it
        let mut route_msg = ::capnp::message::Builder::new_default();
        let mut route_operation = route_msg.init_root::<veilid_capnp::operation::Builder>();
        operation.encode(&mut route_operation)?;
        let out = builder_to_vec(route_msg)?;
        Ok(out)
    }
}
