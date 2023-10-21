use super::*;

impl RPCProcessor {
    // Can only be sent directly, not via relays or routes
    #[cfg_attr(
        feature = "verbose-tracing",
        instrument(level = "trace", skip(self), ret, err)
    )]
    pub async fn rpc_call_validate_dial_info(
        self,
        peer: NodeRef,
        dial_info: DialInfo,
        redirect: bool,
    ) -> Result<bool, RPCError> {
        let network_manager = self.network_manager();
        let receipt_time = ms_to_us(self.unlocked_inner.validate_dial_info_receipt_time_ms);

        // Generate receipt and waitable eventual so we can see if we get the receipt back
        let (receipt, eventual_value) = network_manager
            .generate_single_shot_receipt(receipt_time, [])
            .map_err(RPCError::internal)?;

        let validate_dial_info = RPCOperationValidateDialInfo::new(dial_info, receipt, redirect)?;
        let statement = RPCStatement::new(RPCStatementDetail::ValidateDialInfo(Box::new(
            validate_dial_info,
        )));

        // Send the validate_dial_info request
        // This can only be sent directly, as relays can not validate dial info
        network_result_value_or_log!(self.statement(Destination::direct(peer), statement)
            .await? => [ format!(": peer={} statement={:?}", peer, statement) ] {
                return Ok(false);
            }
        );

        // Wait for receipt
        match eventual_value.await.take_value().unwrap() {
            ReceiptEvent::ReturnedPrivate { private_route: _ }
            | ReceiptEvent::ReturnedInBand { inbound_noderef: _ }
            | ReceiptEvent::ReturnedSafety => {
                log_net!(debug "validate_dial_info receipt should be returned out-of-band");
                Ok(false)
            }
            ReceiptEvent::ReturnedOutOfBand => {
                log_net!(debug "validate_dial_info receipt returned");
                Ok(true)
            }
            ReceiptEvent::Expired => {
                log_net!(debug "validate_dial_info receipt expired");
                Ok(false)
            }
            ReceiptEvent::Cancelled => {
                Err(RPCError::internal("receipt was dropped before expiration"))
            }
        }
    }

    #[cfg_attr(feature="verbose-tracing", instrument(level = "trace", skip(self, msg), fields(msg.operation.op_id), ret, err))]
    pub(crate) async fn process_validate_dial_info(&self, msg: RPCMessage) -> RPCNetworkResult<()> {
        let routing_table = self.routing_table();
        if !routing_table.has_valid_network_class(msg.header.routing_domain()) {
            return Ok(NetworkResult::service_unavailable(
                "can't validate dial info without valid network class",
            ));
        }
        let opi = routing_table.get_own_peer_info(msg.header.routing_domain());

        let detail = match msg.header.detail {
            RPCMessageHeaderDetail::Direct(detail) => detail,
            RPCMessageHeaderDetail::SafetyRouted(_) | RPCMessageHeaderDetail::PrivateRouted(_) => {
                return Ok(NetworkResult::invalid_message(
                    "validate_dial_info must be direct",
                ));
            }
        };

        // Ignore if disabled
        let ni = opi.signed_node_info().node_info();
        if !opi
            .signed_node_info()
            .node_info()
            .has_capability(CAP_VALIDATE_DIAL_INFO)
            || !ni.is_fully_direct_inbound()
        {
            return Ok(NetworkResult::service_unavailable(
                "validate dial info is not available",
            ));
        }

        // Get the statement
        let (_, _, _, kind) = msg.operation.destructure();
        let (dial_info, receipt, redirect) = match kind {
            RPCOperationKind::Statement(s) => match s.destructure() {
                RPCStatementDetail::ValidateDialInfo(s) => s.destructure(),
                _ => panic!("not a validate dial info"),
            },
            _ => panic!("not a statement"),
        };

        // Redirect this request if we are asked to
        if redirect {
            // Find peers capable of validating this dial info
            // We filter on the -outgoing- protocol capability status not the node's dial info
            // Use the address type though, to ensure we reach an ipv6 capable node if this is
            // an ipv6 address
            let sender_node_id = detail.envelope.get_sender_typed_id();
            let routing_domain = detail.routing_domain;
            let node_count = {
                let c = self.config.get();
                c.network.dht.max_find_node_count as usize
            };

            // Filter on nodes that can validate dial info, and can reach a specific dial info
            let outbound_dial_info_entry_filter =
                RoutingTable::make_outbound_dial_info_entry_filter(
                    routing_domain,
                    dial_info.clone(),
                );
            let will_validate_dial_info_filter = Box::new(
                move |rti: &RoutingTableInner, v: Option<Arc<BucketEntry>>| {
                    let entry = v.unwrap();
                    entry.with(rti, move |_rti, e| {
                        e.node_info(routing_domain)
                            .map(|ni| {
                                ni.has_capability(CAP_VALIDATE_DIAL_INFO)
                                    && ni.is_fully_direct_inbound()
                            })
                            .unwrap_or(false)
                    })
                },
            ) as RoutingTableEntryFilter;

            let filters = VecDeque::from([
                outbound_dial_info_entry_filter,
                will_validate_dial_info_filter,
            ]);

            // Find nodes matching filter to redirect this to
            let peers = routing_table.find_fast_public_nodes_filtered(node_count, filters);
            if peers.is_empty() {
                return Ok(NetworkResult::no_connection_other(format!(
                    "no peers able to reach dialinfo '{:?}'",
                    dial_info
                )));
            }
            for peer in peers {
                // Ensure the peer is not the one asking for the validation
                if peer.node_ids().contains(&sender_node_id) {
                    continue;
                }

                // Make a copy of the request, without the redirect flag
                let validate_dial_info =
                    RPCOperationValidateDialInfo::new(dial_info.clone(), receipt.clone(), false)?;
                let statement = RPCStatement::new(RPCStatementDetail::ValidateDialInfo(Box::new(
                    validate_dial_info,
                )));

                // Send the validate_dial_info request
                // This can only be sent directly, as relays can not validate dial info
                network_result_value_or_log!(self.statement(Destination::direct(peer), statement)
                    .await? => [ format!(": peer={} statement={:?}", peer, statement) ] {
                        continue;
                    }
                );
                return Ok(NetworkResult::value(()));
            }

            return Ok(NetworkResult::no_connection_other(
                "could not redirect, no peers were reachable",
            ));
        };

        // Otherwise send a return receipt directly
        // Possibly from an alternate port
        let network_manager = self.network_manager();
        network_manager
            .send_out_of_band_receipt(dial_info.clone(), receipt)
            .await
            .map_err(RPCError::network)?;

        Ok(NetworkResult::value(()))
    }
}
