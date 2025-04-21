use super::*;

impl_veilid_log_facility!("rpc");

impl RPCProcessor {
    // Can only be sent directly, not via relays or routes
    #[instrument(level = "trace", target = "rpc", skip(self), ret, err)]
    pub async fn rpc_call_validate_dial_info(
        &self,
        peer: NodeRef,
        dial_info: DialInfo,
        redirect: bool,
    ) -> Result<bool, RPCError> {
        let _guard = self
            .startup_context
            .startup_lock
            .enter()
            .map_err(RPCError::map_try_again("not started up"))?;
        let stop_token = self
            .startup_context
            .startup_lock
            .stop_token()
            .ok_or(RPCError::try_again("not started up"))?;

        let network_manager = self.network_manager();

        let validate_dial_info_receipt_time_ms = self
            .config()
            .with(|c| c.network.dht.validate_dial_info_receipt_time_ms as u64);

        let receipt_time = TimestampDuration::new_ms(validate_dial_info_receipt_time_ms);

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
        network_result_value_or_log!(self self.statement(Destination::direct(peer.default_filtered()), statement)
            .await? => [ format!(": peer={} statement={:?}", peer, statement) ] {
                return Ok(false);
            }
        );

        // Wait for receipt
        match eventual_value
            .timeout_at(stop_token)
            .in_current_span()
            .await
        {
            Err(_) => {
                return Err(RPCError::try_again("not started up"));
            }
            Ok(v) => {
                let receipt_event = v.take_value().unwrap();
                match receipt_event {
                    ReceiptEvent::ReturnedPrivate { private_route: _ }
                    | ReceiptEvent::ReturnedInBand { inbound_noderef: _ }
                    | ReceiptEvent::ReturnedSafety => {
                        veilid_log!(self debug "validate_dial_info receipt should be returned out-of-band");
                        Ok(false)
                    }
                    ReceiptEvent::ReturnedOutOfBand => {
                        veilid_log!(self debug "validate_dial_info receipt returned");
                        Ok(true)
                    }
                    ReceiptEvent::Expired => {
                        veilid_log!(self debug "validate_dial_info receipt expired");
                        Ok(false)
                    }
                    ReceiptEvent::Cancelled => {
                        Err(RPCError::internal("receipt was dropped before expiration"))
                    }
                }
            }
        }
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////

    //#[instrument(level = "trace", target = "rpc", skip(self, msg), fields(msg.operation.op_id), ret, err)]
    pub(super) async fn process_validate_dial_info(&self, msg: Message) -> RPCNetworkResult<()> {
        // Ensure this never came over a private route, safety route is okay though
        let detail = match msg.header.detail {
            RPCMessageHeaderDetail::Direct(detail) => detail,
            RPCMessageHeaderDetail::SafetyRouted(_) | RPCMessageHeaderDetail::PrivateRouted(_) => {
                return Ok(NetworkResult::invalid_message(
                    "validate_dial_info must be direct",
                ));
            }
        };

        // Ignore if disabled
        let routing_table = self.routing_table();
        let routing_domain = detail.routing_domain;

        let has_capability_validate_dial_info = routing_table
            .get_published_peer_info(routing_domain)
            .map(|ppi| {
                ppi.signed_node_info()
                    .node_info()
                    .has_capability(CAP_VALIDATE_DIAL_INFO)
                    && ppi.signed_node_info().node_info().is_fully_direct_inbound()
            })
            .unwrap_or(false);
        if !has_capability_validate_dial_info {
            return Ok(NetworkResult::service_unavailable(
                "validate dial info is not available",
            ));
        }

        // Get the statement
        let (_, _, kind) = msg.operation.destructure();
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
            let node_count = self
                .config()
                .with(|c| c.network.dht.max_find_node_count as usize);

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
            let peers = routing_table.find_fast_non_local_nodes_filtered(
                routing_domain,
                node_count,
                filters,
            );
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
                network_result_value_or_log!(self pin_future_closure!(self.statement(Destination::direct(peer.default_filtered()), statement))
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

        // Otherwise send a return receipt directly from a system-allocated random port
        let network_manager = self.network_manager();
        network_manager
            .send_out_of_band_receipt(dial_info.clone(), receipt)
            .await
            .map_err(RPCError::network)?;

        Ok(NetworkResult::value(()))
    }
}
