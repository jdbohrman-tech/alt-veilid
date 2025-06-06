use super::*;
use stop_token::future::FutureExt as _;

impl NetworkManager {
    /// Send raw data to a node
    ///
    /// Sending to a node requires determining a NodeContactMethod.
    /// NodeContactMethod is how to reach a node given the context of our current node, which may
    /// include information about the existing connections and network state of our node.
    /// NodeContactMethod calculation requires first calculating the per-RoutingDomain ContactMethod
    /// between the source and destination PeerInfo, which is a stateless operation.
    #[instrument(level = "trace", target = "net", skip_all, err)]
    pub async fn send_data(
        &self,
        destination_node_ref: FilteredNodeRef,
        data: Vec<u8>,
    ) -> EyreResult<NetworkResult<SendDataResult>> {
        // Get the best way to contact this node
        let mut opt_node_contact_method =
            self.get_node_contact_method(destination_node_ref.clone())?;

        // Retry loop
        loop {
            // Boxed because calling rpc_call_signal() is recursive to send_data()
            let nres = pin_future_closure!(self.try_node_contact_method(
                opt_node_contact_method.clone(),
                destination_node_ref.clone(),
                data.clone(),
            ))
            .await?;

            match &nres {
                NetworkResult::Timeout => {
                    // Record contact method failure statistics
                    self.inner
                        .lock()
                        .node_contact_method_cache
                        .record_contact_method_failure(
                            opt_node_contact_method.as_ref().map(|x| &x.ncm_kind),
                        );

                    // Timeouts may retry with a different method
                    match opt_node_contact_method {
                        Some(NodeContactMethod {
                            ncm_key,
                            ncm_kind:
                                NodeContactMethodKind::SignalReverse(relay_nr, _target_node_ref),
                        }) => {
                            // Try again with a different method
                            opt_node_contact_method = Some(NodeContactMethod {
                                ncm_key,
                                ncm_kind: NodeContactMethodKind::InboundRelay(relay_nr),
                            });
                            continue;
                        }
                        Some(NodeContactMethod {
                            ncm_key,
                            ncm_kind:
                                NodeContactMethodKind::SignalHolePunch(relay_nr, _target_node_ref),
                        }) => {
                            // Try again with a different method
                            opt_node_contact_method = Some(NodeContactMethod {
                                ncm_key,
                                ncm_kind: NodeContactMethodKind::InboundRelay(relay_nr),
                            });
                            continue;
                        }
                        _ => {
                            // Don't retry any other contact methods, and don't cache a timeout
                            break Ok(nres);
                        }
                    }
                }
                NetworkResult::ServiceUnavailable(_)
                | NetworkResult::NoConnection(_)
                | NetworkResult::AlreadyExists(_)
                | NetworkResult::InvalidMessage(_) => {
                    // Record contact method failure statistics
                    self.inner
                        .lock()
                        .node_contact_method_cache
                        .record_contact_method_failure(
                            opt_node_contact_method.as_ref().map(|x| &x.ncm_kind),
                        );

                    // Other network results don't cache, just directly return the result
                    break Ok(nres);
                }
                NetworkResult::Value(v) => {
                    // Successful network result gets to cache the node contact method
                    if let Some(ncm) = &v.opt_contact_method {
                        // Cache the contact method
                        self.cache_node_contact_method(ncm.clone());
                    }
                    if let Some(ncm) = &v.opt_relayed_contact_method {
                        // Cache the relayed contact method
                        self.cache_node_contact_method(ncm.clone());
                    }

                    // Record cache insertion as a success
                    self.inner
                        .lock()
                        .node_contact_method_cache
                        .record_contact_method_success(
                            v.opt_contact_method.as_ref().map(|x| &x.ncm_kind),
                        );
                    // If relayed contact method was specified, then it wasn't unreachable
                    // (must have been relay type or it wouldnt be here, and if this is None
                    // then the contact method was not relayed)
                    if v.opt_relayed_contact_method.is_some() {
                        self.inner
                            .lock()
                            .node_contact_method_cache
                            .record_contact_method_success(
                                v.opt_relayed_contact_method.as_ref().map(|x| &x.ncm_kind),
                            );
                    }

                    break Ok(nres);
                }
            }
        }
    }

    #[instrument(level = "trace", target = "net", skip_all)]
    async fn try_node_contact_method(
        &self,
        opt_node_contact_method: Option<NodeContactMethod>,
        destination_node_ref: FilteredNodeRef,
        data: Vec<u8>,
    ) -> EyreResult<NetworkResult<SendDataResult>> {
        // If we need to relay, do it
        let (opt_contact_method, target_node_ref, opt_relayed_contact_method) =
            match opt_node_contact_method.clone().map(|x| x.ncm_kind) {
                Some(NodeContactMethodKind::OutboundRelay(relay_nr))
                | Some(NodeContactMethodKind::InboundRelay(relay_nr)) => {
                    let opt_contact_method = self.get_node_contact_method(relay_nr.clone())?;
                    (opt_contact_method, relay_nr, opt_node_contact_method)
                }
                _ => (opt_node_contact_method, destination_node_ref.clone(), None),
            };

        #[cfg(feature = "verbose-tracing")]
        veilid_log!(self debug
            "ContactMethod: {:?} for {:?}",
            opt_contact_method, destination_node_ref
        );

        // Try the contact method
        let unique_flow = match &opt_contact_method {
            None => {
                // If a node is unreachable it may still have an existing inbound connection
                // Try that, but don't cache anything
                network_result_try!(
                    pin_future_closure!(self.send_data_unreachable(target_node_ref, data)).await?
                )
            }
            Some(NodeContactMethod {
                ncm_key: _,
                ncm_kind: NodeContactMethodKind::Existing,
            }) => {
                // The node must have an existing connection, for example connecting to your own
                // relay is something that must always have a connection already
                network_result_try!(
                    pin_future_closure!(self.send_data_ncm_existing(target_node_ref, data)).await?
                )
            }
            Some(NodeContactMethod {
                ncm_key: _,
                ncm_kind: NodeContactMethodKind::OutboundRelay(relay_nr),
            }) => {
                // Relay loop or multiple relays
                veilid_log!(self debug
                "Outbound relay loop or multiple relays detected: destination {} resolved to target {} via extraneous relay {}",
                    destination_node_ref,
                    target_node_ref,
                    relay_nr
                );
                return Ok(NetworkResult::no_connection_other("outbound relay loop"));
            }
            Some(NodeContactMethod {
                ncm_key: _,
                ncm_kind: NodeContactMethodKind::InboundRelay(relay_nr),
            }) => {
                // Relay loop or multiple relays
                veilid_log!(self debug
                "Inbound relay loop or multiple relays detected: destination {} resolved to target {} via extraneous relay {}",
                    destination_node_ref,
                    target_node_ref,
                    relay_nr
                );
                return Ok(NetworkResult::no_connection_other("inbound relay loop"));
            }
            Some(NodeContactMethod {
                ncm_key: _,
                ncm_kind: NodeContactMethodKind::Direct(dial_info),
            }) => {
                network_result_try!(
                    pin_future_closure!(self.send_data_ncm_direct(
                        target_node_ref,
                        dial_info.clone(),
                        data
                    ))
                    .await?
                )
            }
            Some(NodeContactMethod {
                ncm_key: _,
                ncm_kind: NodeContactMethodKind::SignalReverse(relay_nr, target_node_ref),
            }) => {
                network_result_try!(
                    pin_future_closure!(self.send_data_ncm_signal_reverse(
                        relay_nr.clone(),
                        target_node_ref.clone(),
                        data.clone()
                    ))
                    .await?
                )
            }
            Some(NodeContactMethod {
                ncm_key: _,
                ncm_kind: NodeContactMethodKind::SignalHolePunch(relay_nr, target_node_ref),
            }) => {
                network_result_try!(
                    pin_future_closure!(self.send_data_ncm_signal_hole_punch(
                        relay_nr.clone(),
                        target_node_ref.clone(),
                        data.clone()
                    ))
                    .await?
                )
            }
        };

        Ok(NetworkResult::value(SendDataResult {
            opt_contact_method,
            opt_relayed_contact_method,
            unique_flow,
        }))
    }

    /// Send data to unreachable node
    #[instrument(level = "trace", target = "net", skip_all, err)]
    async fn send_data_unreachable(
        &self,
        target_node_ref: FilteredNodeRef,
        data: Vec<u8>,
    ) -> EyreResult<NetworkResult<UniqueFlow>> {
        // First try to send data to the last connection we've seen this peer on
        let Some(flow) = target_node_ref.last_flow() else {
            return Ok(NetworkResult::no_connection_other(format!(
                "node was unreachable: {}",
                target_node_ref
            )));
        };

        let net = self.net();
        let unique_flow = match pin_future!(debug_duration(
            || { net.send_data_to_existing_flow(flow, data) },
            Some(1_000_000)
        ))
        .await?
        {
            SendDataToExistingFlowResult::Sent(unique_flow) => unique_flow,
            SendDataToExistingFlowResult::NotSent(_) => {
                return Ok(NetworkResult::no_connection_other(
                    "failed to send to existing flow",
                ));
            }
        };

        // Update timestamp for this last connection since we just sent to it
        self.set_last_flow(target_node_ref.unfiltered(), flow, Timestamp::now());

        Ok(NetworkResult::value(unique_flow))
    }

    /// Send data using NodeContactMethod::Existing
    #[instrument(level = "trace", target = "net", skip_all, err)]
    async fn send_data_ncm_existing(
        &self,
        target_node_ref: FilteredNodeRef,
        data: Vec<u8>,
    ) -> EyreResult<NetworkResult<UniqueFlow>> {
        // First try to send data to the last connection we've seen this peer on
        let Some(flow) = target_node_ref.last_flow() else {
            return Ok(NetworkResult::no_connection_other(format!(
                "should have found an existing connection: {}",
                target_node_ref
            )));
        };

        let net = self.net();
        let unique_flow = match pin_future!(debug_duration(
            || { net.send_data_to_existing_flow(flow, data) },
            Some(1_000_000)
        ))
        .await?
        {
            SendDataToExistingFlowResult::Sent(unique_flow) => unique_flow,
            SendDataToExistingFlowResult::NotSent(_) => {
                return Ok(NetworkResult::no_connection_other(
                    "failed to send to existing flow",
                ));
            }
        };

        // Update timestamp for this last connection since we just sent to it
        self.set_last_flow(target_node_ref.unfiltered(), flow, Timestamp::now());

        Ok(NetworkResult::value(unique_flow))
    }

    /// Send data using NodeContactMethod::SignalReverse
    #[instrument(level = "trace", target = "net", skip_all, err)]
    async fn send_data_ncm_signal_reverse(
        &self,
        relay_nr: FilteredNodeRef,
        target_node_ref: FilteredNodeRef,
        data: Vec<u8>,
    ) -> EyreResult<NetworkResult<UniqueFlow>> {
        // Make a noderef that meets the sequencing requirements
        // But is not protocol-specific, or address-family-specific
        // as a signalled node gets to choose its own dial info for the reverse connection.
        let (_sorted, seq_dif) = target_node_ref
            .dial_info_filter()
            .apply_sequencing(target_node_ref.sequencing());
        let seq_target_node_ref = if seq_dif.is_ordered_only() {
            target_node_ref
                .unfiltered()
                .sequencing_filtered(Sequencing::EnsureOrdered)
        } else {
            target_node_ref
                .unfiltered()
                .sequencing_filtered(Sequencing::NoPreference)
        };

        // First try to send data to the last flow we've seen this peer on
        let data = if let Some(flow) = seq_target_node_ref.last_flow() {
            let net = self.net();
            match pin_future!(debug_duration(
                || { net.send_data_to_existing_flow(flow, data) },
                Some(1_000_000)
            ))
            .await?
            {
                SendDataToExistingFlowResult::Sent(unique_flow) => {
                    // Update timestamp for this last connection since we just sent to it
                    self.set_last_flow(target_node_ref.unfiltered(), flow, Timestamp::now());

                    return Ok(NetworkResult::value(unique_flow));
                }
                SendDataToExistingFlowResult::NotSent(data) => {
                    // Couldn't send data to existing connection
                    // so pass the data back out
                    data
                }
            }
        } else {
            // No last connection
            #[cfg(feature = "verbose-tracing")]
            veilid_log!(self debug
                "No last flow in reverse connect for {:?}",
                target_node_ref
            );

            data
        };

        let excessive_reverse_connect_duration_us = self.config().with(|c| {
            (c.network.connection_initial_timeout_ms * 2
                + c.network.reverse_connection_receipt_time_ms) as u64
                * 1000
        });

        let unique_flow = network_result_try!(
            pin_future!(debug_duration(
                || { self.do_reverse_connect(relay_nr.clone(), target_node_ref.clone(), data) },
                Some(excessive_reverse_connect_duration_us)
            ))
            .await?
        );
        Ok(NetworkResult::value(unique_flow))
    }

    /// Send data using NodeContactMethod::SignalHolePunch
    #[instrument(level = "trace", target = "net", skip_all, err)]
    async fn send_data_ncm_signal_hole_punch(
        &self,
        relay_nr: FilteredNodeRef,
        target_node_ref: FilteredNodeRef,
        data: Vec<u8>,
    ) -> EyreResult<NetworkResult<UniqueFlow>> {
        // First try to send data to the last flow we've seen this peer on
        let data = if let Some(flow) = target_node_ref.last_flow() {
            let net = self.net();
            match pin_future!(debug_duration(
                || { net.send_data_to_existing_flow(flow, data) },
                Some(1_000_000)
            ))
            .await?
            {
                SendDataToExistingFlowResult::Sent(unique_flow) => {
                    // Update timestamp for this last connection since we just sent to it
                    self.set_last_flow(target_node_ref.unfiltered(), flow, Timestamp::now());

                    return Ok(NetworkResult::value(unique_flow));
                }
                SendDataToExistingFlowResult::NotSent(data) => {
                    // Couldn't send data to existing connection
                    // so pass the data back out
                    data
                }
            }
        } else {
            // No last connection
            #[cfg(feature = "verbose-tracing")]
            veilid_log!(self debug
                "No last flow in hole punch for {:?}",
                target_node_ref
            );

            data
        };

        let hole_punch_receipt_time_us = self
            .config()
            .with(|c| c.network.hole_punch_receipt_time_ms as u64 * 1000);

        let unique_flow = network_result_try!(
            pin_future!(debug_duration(
                || { self.do_hole_punch(relay_nr.clone(), target_node_ref.clone(), data) },
                Some(hole_punch_receipt_time_us * 2)
            ))
            .await?
        );

        Ok(NetworkResult::value(unique_flow))
    }

    /// Send data using NodeContactMethod::Direct
    #[instrument(level = "trace", target = "net", skip_all, err)]
    async fn send_data_ncm_direct(
        &self,
        node_ref: FilteredNodeRef,
        dial_info: DialInfo,
        data: Vec<u8>,
    ) -> EyreResult<NetworkResult<UniqueFlow>> {
        // Since we have the best dial info already, we can find a connection to use by protocol type
        let node_ref = node_ref.filtered_clone(NodeRefFilter::from(dial_info.make_filter()));

        // First try to send data to the last flow we've seen this peer on
        let data = if let Some(flow) = node_ref.last_flow() {
            #[cfg(feature = "verbose-tracing")]
            veilid_log!(self debug
                "ExistingConnection: {:?} for {:?}",
                flow, node_ref
            );

            let net = self.net();
            match pin_future!(debug_duration(
                || { net.send_data_to_existing_flow(flow, data) },
                Some(1_000_000)
            ))
            .await?
            {
                SendDataToExistingFlowResult::Sent(unique_flow) => {
                    // Update timestamp for this last connection since we just sent to it
                    self.set_last_flow(node_ref.unfiltered(), flow, Timestamp::now());

                    return Ok(NetworkResult::value(unique_flow));
                }
                SendDataToExistingFlowResult::NotSent(d) => {
                    // Connection couldn't send, kill it
                    node_ref.clear_last_flow(flow);
                    d
                }
            }
        } else {
            data
        };

        // New direct connection was necessary for this dial info
        let net = self.net();
        let unique_flow = network_result_try!(
            pin_future!(net.send_data_to_dial_info(dial_info.clone(), data)).await?
        );

        // If we connected to this node directly, save off the last connection so we can use it again
        self.set_last_flow(node_ref.unfiltered(), unique_flow.flow, Timestamp::now());

        Ok(NetworkResult::value(unique_flow))
    }

    #[instrument(level = "trace", target = "net", skip(self), err)]
    pub fn get_node_contact_method(
        &self,
        target_node_ref: FilteredNodeRef,
    ) -> EyreResult<Option<NodeContactMethod>> {
        let routing_table = self.routing_table();

        // If a node is punished, then don't try to contact it
        if target_node_ref
            .node_ids()
            .iter()
            .any(|nid| self.address_filter().is_node_id_punished(*nid))
        {
            return Ok(None);
        }

        // Figure out the best routing domain to get the contact method over
        let routing_domain = match target_node_ref.best_routing_domain() {
            Some(rd) => rd,
            None => {
                veilid_log!(self trace "no routing domain for node {:?}", target_node_ref);
                return Ok(None);
            }
        };

        // Peer A is our own node
        // Use whatever node info we've calculated so far
        let peer_a = routing_table.get_current_peer_info(routing_domain);
        let own_node_info_ts = peer_a.signed_node_info().timestamp();

        // Peer B is the target node, get the whole peer info now
        let Some(peer_b) = target_node_ref.get_peer_info(routing_domain) else {
            veilid_log!(self trace "no node info for node {:?}", target_node_ref);
            return Ok(None);
        };

        // Calculate the dial info failures map
        let address_filter = self.address_filter();
        let dial_info_failures_map = {
            let mut dial_info_failures_map = BTreeMap::<DialInfo, Timestamp>::new();
            for did in peer_b
                .signed_node_info()
                .node_info()
                .filtered_dial_info_details(DialInfoDetail::NO_SORT, &|_| true)
            {
                if let Some(ts) = address_filter.get_dial_info_failed_ts(&did.dial_info) {
                    dial_info_failures_map.insert(did.dial_info, ts);
                }
            }
            dial_info_failures_map
        };

        // Get cache key
        let ncm_key = NodeContactMethodCacheKey {
            node_ids: target_node_ref.node_ids(),
            own_node_info_ts,
            target_node_info_ts: peer_b.signed_node_info().timestamp(),
            target_node_ref_filter: target_node_ref.filter(),
            target_node_ref_sequencing: target_node_ref.sequencing(),
            dial_info_failures_map,
        };
        if let Some(ncm_kind) = self.inner.lock().node_contact_method_cache.get(&ncm_key) {
            return Ok(Some(NodeContactMethod { ncm_key, ncm_kind }));
        }

        // Calculate the node contact method
        let routing_table = self.routing_table();
        let Some(ncm_kind) = Self::get_node_contact_method_kind(
            &routing_table,
            routing_domain,
            target_node_ref,
            peer_a,
            peer_b,
            &ncm_key,
        )?
        else {
            return Ok(None);
        };

        Ok(Some(NodeContactMethod { ncm_key, ncm_kind }))
    }

    fn cache_node_contact_method(&self, ncm: NodeContactMethod) {
        // Cache this
        self.inner
            .lock()
            .node_contact_method_cache
            .insert(ncm.ncm_key, ncm.ncm_kind);
    }

    /// Figure out how to reach a node from our own node over the best routing domain and reference the nodes we want to access
    /// Uses NodeRefs to ensure nodes are referenced, this is not a part of 'RoutingTable' because RoutingTable is not
    /// allowed to use NodeRefs due to recursive locking
    #[instrument(level = "trace", target = "net", skip_all, err)]
    fn get_node_contact_method_kind(
        routing_table: &RoutingTable,
        routing_domain: RoutingDomain,
        target_node_ref: FilteredNodeRef,
        peer_a: Arc<PeerInfo>,
        peer_b: Arc<PeerInfo>,
        ncm_key: &NodeContactMethodCacheKey,
    ) -> EyreResult<Option<NodeContactMethodKind>> {
        // Dial info filter comes from the target node ref but must be filtered by this node's outbound capabilities
        let dial_info_filter = target_node_ref.dial_info_filter().filtered(
            DialInfoFilter::all()
                .with_address_type_set(peer_a.signed_node_info().node_info().address_types())
                .with_protocol_type_set(peer_a.signed_node_info().node_info().outbound_protocols()),
        );
        let sequencing = target_node_ref.sequencing();

        // If the node has had lost questions or failures to send, prefer sequencing
        // to improve reliability. The node may be experiencing UDP fragmentation drops
        // or other firewalling issues and may perform better with TCP.
        // let unreliable = target_node_ref.peer_stats().rpc_stats.failed_to_send > 2 || target_node_ref.peer_stats().rpc_stats.recent_lost_answers > 2;
        // if unreliable && sequencing < Sequencing::PreferOrdered {
        //     veilid_log!(self debug "Node contact failing over to Ordered for {}", target_node_ref.to_string().cyan());
        //     sequencing = Sequencing::PreferOrdered;
        // }

        // Deprioritize dial info that have recently failed
        let dif_sort: Option<DialInfoDetailSort> = if ncm_key.dial_info_failures_map.is_empty() {
            None
        } else {
            Some(Box::new(|a: &DialInfoDetail, b: &DialInfoDetail| {
                let ats = ncm_key
                    .dial_info_failures_map
                    .get(&a.dial_info)
                    .copied()
                    .unwrap_or_default();
                let bts = ncm_key
                    .dial_info_failures_map
                    .get(&b.dial_info)
                    .copied()
                    .unwrap_or_default();
                ats.cmp(&bts)
            }))
        };

        // Get the best contact method with these parameters from the routing domain
        let cm = routing_table.get_contact_method(
            routing_domain,
            peer_a.clone(),
            peer_b.clone(),
            dial_info_filter,
            sequencing,
            dif_sort.as_ref(),
        );

        // Translate the raw contact method to a referenced contact method
        let ncm = match cm {
            ContactMethod::Unreachable => None,
            ContactMethod::Existing => Some(NodeContactMethodKind::Existing),
            ContactMethod::Direct(di) => Some(NodeContactMethodKind::Direct(di)),
            ContactMethod::SignalReverse(relay_key, target_key) => {
                let mut relay_nr = routing_table
                    .lookup_and_filter_noderef(relay_key, routing_domain.into(), dial_info_filter)?
                    .ok_or_else(|| {
                        eyre!(
                            "couldn't look up relay for signal reverse: {} with filter {:?}",
                            relay_key,
                            dial_info_filter
                        )
                    })?;
                if !target_node_ref.node_ids().contains(&target_key) {
                    bail!("signalreverse target noderef didn't match target key: {:?} != {} for relay {}", target_node_ref, target_key, relay_key );
                }
                // Set sequencing requirement for the relay
                relay_nr.set_sequencing(sequencing);

                // Tighten sequencing for the target to the best reverse connection flow we can get
                let tighten = peer_a
                    .signed_node_info()
                    .node_info()
                    .filtered_dial_info_details(DialInfoDetail::NO_SORT, &|did| {
                        did.matches_filter(&dial_info_filter)
                    })
                    .iter()
                    .find_map(|did| {
                        if peer_b
                            .signed_node_info()
                            .node_info()
                            .address_types()
                            .contains(did.dial_info.address_type())
                            && peer_b
                                .signed_node_info()
                                .node_info()
                                .outbound_protocols()
                                .contains(did.dial_info.protocol_type())
                            && did.dial_info.protocol_type().is_ordered()
                        {
                            Some(true)
                        } else {
                            None
                        }
                    })
                    .unwrap_or(false);

                let mut target_node_ref = target_node_ref.filtered_clone(
                    NodeRefFilter::from(dial_info_filter).with_routing_domain(routing_domain),
                );
                if tighten {
                    target_node_ref.set_sequencing(Sequencing::EnsureOrdered);
                }
                Some(NodeContactMethodKind::SignalReverse(
                    relay_nr,
                    target_node_ref,
                ))
            }
            ContactMethod::SignalHolePunch(relay_key, target_key) => {
                let mut relay_nr = routing_table
                    .lookup_and_filter_noderef(relay_key, routing_domain.into(), dial_info_filter)?
                    .ok_or_else(|| {
                        eyre!(
                            "couldn't look up relay for hole punch: {} with filter {:?}",
                            relay_key,
                            dial_info_filter
                        )
                    })?;
                if !target_node_ref.node_ids().contains(&target_key) {
                    bail!("signalholepunch target noderef didn't match target key: {:?} != {} for relay {}", target_node_ref, target_key, relay_key );
                }
                // Set sequencing requirement for the relay
                relay_nr.set_sequencing(sequencing);

                // if any other protocol were possible here we could update this and do_hole_punch
                // but tcp hole punch is very very unreliable it seems
                let udp_target_node_ref = target_node_ref.filtered_clone(
                    NodeRefFilter::new()
                        .with_routing_domain(routing_domain)
                        .with_dial_info_filter(dial_info_filter)
                        .with_protocol_type(ProtocolType::UDP),
                );

                Some(NodeContactMethodKind::SignalHolePunch(
                    relay_nr,
                    udp_target_node_ref,
                ))
            }
            ContactMethod::InboundRelay(relay_key) => {
                let mut relay_nr = routing_table
                    .lookup_and_filter_noderef(relay_key, routing_domain.into(), dial_info_filter)?
                    .ok_or_else(|| {
                        eyre!(
                            "couldn't look up relay for inbound relay: {} with filter {:?}",
                            relay_key,
                            dial_info_filter
                        )
                    })?;
                relay_nr.set_sequencing(sequencing);
                Some(NodeContactMethodKind::InboundRelay(relay_nr))
            }
            ContactMethod::OutboundRelay(relay_key) => {
                let mut relay_nr = routing_table
                    .lookup_and_filter_noderef(relay_key, routing_domain.into(), dial_info_filter)?
                    .ok_or_else(|| {
                        eyre!(
                            "couldn't look up relay for outbound relay: {} with filter {:?}",
                            relay_key,
                            dial_info_filter
                        )
                    })?;
                relay_nr.set_sequencing(sequencing);
                Some(NodeContactMethodKind::OutboundRelay(relay_nr))
            }
        };

        Ok(ncm)
    }

    /// Send a reverse connection signal and wait for the return receipt over it
    /// Then send the data across the new connection
    /// Only usable for PublicInternet routing domain
    #[instrument(level = "trace", target = "net", skip_all, err)]
    async fn do_reverse_connect(
        &self,
        relay_nr: FilteredNodeRef,
        target_nr: FilteredNodeRef,
        data: Vec<u8>,
    ) -> EyreResult<NetworkResult<UniqueFlow>> {
        // Detect if network is stopping so we can break out of this
        let Some(stop_token) = self.startup_context.startup_lock.stop_token() else {
            return Ok(NetworkResult::service_unavailable("network is stopping"));
        };

        // Build a return receipt for the signal
        let receipt_timeout = TimestampDuration::new_ms(
            self.config()
                .with(|c| c.network.reverse_connection_receipt_time_ms as u64),
        );
        let (receipt, eventual_value) = self.generate_single_shot_receipt(receipt_timeout, [])?;

        // Get target routing domain
        let Some(routing_domain) = target_nr.best_routing_domain() else {
            return Ok(NetworkResult::no_connection_other(
                "No routing domain for target for reverse connect",
            ));
        };

        // Get our published peer info
        let Some(published_peer_info) =
            self.routing_table().get_published_peer_info(routing_domain)
        else {
            return Ok(NetworkResult::no_connection_other(
                "Network class not yet valid for reverse connect",
            ));
        };

        // Issue the signal
        let rpc = self.rpc_processor();
        network_result_try!(pin_future!(rpc.rpc_call_signal(
            Destination::relay(relay_nr.clone(), target_nr.unfiltered()),
            SignalInfo::ReverseConnect {
                receipt,
                peer_info: published_peer_info
            },
        ))
        .await
        .wrap_err("failed to send signal")?);

        // Wait for the return receipt
        let inbound_nr = match eventual_value
            .timeout_at(stop_token)
            .in_current_span()
            .await
        {
            Err(_) => {
                return Ok(NetworkResult::service_unavailable("network is stopping"));
            }
            Ok(v) => {
                let receipt_event = v.take_value().unwrap();
                match receipt_event {
                    ReceiptEvent::ReturnedPrivate { private_route: _ }
                    | ReceiptEvent::ReturnedOutOfBand
                    | ReceiptEvent::ReturnedSafety => {
                        return Ok(NetworkResult::invalid_message(
                            "reverse connect receipt should be returned in-band",
                        ));
                    }
                    ReceiptEvent::ReturnedInBand { inbound_noderef } => inbound_noderef,
                    ReceiptEvent::Expired => {
                        return Ok(NetworkResult::timeout());
                    }
                    ReceiptEvent::Cancelled => {
                        return Ok(NetworkResult::no_connection_other(format!(
                            "reverse connect receipt cancelled from {}",
                            target_nr
                        )))
                    }
                }
            }
        };

        // We expect the inbound noderef to be the same as the target noderef
        // if they aren't the same, we should error on this and figure out what then hell is up
        if !target_nr.same_entry(&inbound_nr) {
            bail!("unexpected noderef mismatch on reverse connect");
        }

        // And now use the existing connection to send over
        if let Some(flow) = inbound_nr.last_flow() {
            let net = self.net();
            match pin_future!(net.send_data_to_existing_flow(flow, data)).await? {
                SendDataToExistingFlowResult::Sent(unique_flow) => {
                    Ok(NetworkResult::value(unique_flow))
                }
                SendDataToExistingFlowResult::NotSent(_) => Ok(NetworkResult::no_connection_other(
                    "unable to send over reverse connection",
                )),
            }
        } else {
            return Ok(NetworkResult::no_connection_other(format!(
                "reverse connection dropped from {}",
                target_nr
            )));
        }
    }

    /// Send a hole punch signal and do a negotiating ping and wait for the return receipt
    /// Then send the data across the new connection
    /// Only usable for PublicInternet routing domain
    #[instrument(level = "trace", target = "net", skip_all, err)]
    async fn do_hole_punch(
        &self,
        relay_nr: FilteredNodeRef,
        target_nr: FilteredNodeRef,
        data: Vec<u8>,
    ) -> EyreResult<NetworkResult<UniqueFlow>> {
        // Detect if network is stopping so we can break out of this
        let Some(stop_token) = self.startup_context.startup_lock.stop_token() else {
            return Ok(NetworkResult::service_unavailable("network is stopping"));
        };

        // Ensure target is filtered down to UDP (the only hole punch protocol supported today)
        // Relay can be any protocol because the signal rpc contains the dialinfo to connect over
        assert_eq!(
            target_nr.dial_info_filter().protocol_type_set,
            ProtocolType::UDP
        );

        // Build a return receipt for the signal
        let receipt_timeout = TimestampDuration::new_ms(
            self.config()
                .with(|c| c.network.hole_punch_receipt_time_ms as u64),
        );
        let (receipt, eventual_value) = self.generate_single_shot_receipt(receipt_timeout, [])?;

        // Get target routing domain
        let Some(routing_domain) = target_nr.best_routing_domain() else {
            return Ok(NetworkResult::no_connection_other(
                "No routing domain for target for hole punch",
            ));
        };

        // Get our published peer info
        let Some(published_peer_info) =
            self.routing_table().get_published_peer_info(routing_domain)
        else {
            return Ok(NetworkResult::no_connection_other(
                "Network class not yet valid for hole punch",
            ));
        };

        // Get the udp direct dialinfo for the hole punch
        let hole_punch_did = target_nr
            .first_dial_info_detail()
            .ok_or_else(|| eyre!("No hole punch capable dialinfo found for node"))?;

        // Do our half of the hole punch by sending an empty packet
        // Both sides will do this and then the receipt will get sent over the punched hole
        // Don't bother storing the returned flow as the 'last flow' because the other side of the hole
        // punch should come through and create a real 'last connection' for us if this succeeds
        let net = self.net();
        network_result_try!(
            pin_future!(net.send_data_to_dial_info(hole_punch_did.dial_info.clone(), Vec::new()))
                .await?
        );

        // Add small delay to encourage packets to be delivered in order
        sleep(HOLE_PUNCH_DELAY_MS).await;

        // Issue the signal
        let rpc = self.rpc_processor();
        network_result_try!(pin_future!(rpc.rpc_call_signal(
            Destination::relay(relay_nr, target_nr.unfiltered()),
            SignalInfo::HolePunch {
                receipt,
                peer_info: published_peer_info
            },
        ))
        .await
        .wrap_err("failed to send signal")?);

        // Another hole punch after the signal for UDP redundancy
        let net = self.net();
        network_result_try!(
            pin_future!(net.send_data_to_dial_info(hole_punch_did.dial_info, Vec::new())).await?
        );

        // Wait for the return receipt
        let inbound_nr = match eventual_value
            .timeout_at(stop_token)
            .in_current_span()
            .await
        {
            Err(_) => {
                return Ok(NetworkResult::service_unavailable("network is stopping"));
            }
            Ok(v) => {
                let receipt_event = v.take_value().unwrap();
                match receipt_event {
                    ReceiptEvent::ReturnedPrivate { private_route: _ }
                    | ReceiptEvent::ReturnedOutOfBand
                    | ReceiptEvent::ReturnedSafety => {
                        return Ok(NetworkResult::invalid_message(
                            "hole punch receipt should be returned in-band",
                        ));
                    }
                    ReceiptEvent::ReturnedInBand { inbound_noderef } => inbound_noderef,
                    ReceiptEvent::Expired => {
                        return Ok(NetworkResult::timeout());
                    }
                    ReceiptEvent::Cancelled => {
                        return Ok(NetworkResult::no_connection_other(format!(
                            "hole punch receipt cancelled from {}",
                            target_nr
                        )))
                    }
                }
            }
        };

        // We expect the inbound noderef to be the same as the target noderef
        // if they aren't the same, we should error on this and figure out what then hell is up
        if !target_nr.same_entry(&inbound_nr) {
            bail!(
                "unexpected noderef mismatch on hole punch {}, expected {}",
                inbound_nr,
                target_nr
            );
        }

        // And now use the existing connection to send over
        if let Some(flow) = inbound_nr.last_flow() {
            match self.net().send_data_to_existing_flow(flow, data).await? {
                SendDataToExistingFlowResult::Sent(unique_flow) => {
                    Ok(NetworkResult::value(unique_flow))
                }
                SendDataToExistingFlowResult::NotSent(_) => Ok(NetworkResult::no_connection_other(
                    "unable to send over hole punch",
                )),
            }
        } else {
            return Ok(NetworkResult::no_connection_other(format!(
                "hole punch dropped from {}",
                target_nr
            )));
        }
    }
}
