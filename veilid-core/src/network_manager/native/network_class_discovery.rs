/// Detect NetworkClass and DialInfo for the DialInfo for the PublicInternet RoutingDomain
use super::*;
use futures_util::stream::FuturesUnordered;
use futures_util::FutureExt;
use stop_token::future::FutureExt as StopTokenFutureExt;

impl Network {
    #[instrument(level = "trace", skip(self, context), err)]
    pub async fn update_protocol_dialinfo(
        &self,
        context: &DiscoveryContext,
        protocol_type: ProtocolType,
        address_type: AddressType,
    ) -> EyreResult<()> {
        let mut retry_count = {
            let c = self.config.get();
            c.network.restricted_nat_retries
        };

        // Start doing protocol
        context.protocol_begin(protocol_type, address_type);

        // UPNP Automatic Mapping
        ///////////
        let this = self.clone();
        let do_mapped_fut: SendPinBoxFuture<Option<DetectedDialInfo>> = Box::pin(async move {
            // Attempt a port mapping via all available and enabled mechanisms
            // Try this before the direct mapping in the event that we are restarting
            // and may not have recorded a mapping created the last time
            if let Some(external_mapped_dial_info) = this.try_port_mapping().await {
                // Got a port mapping, let's use it
                return Some(DetectedDialInfo {
                    dial_info: external_mapped_dial_info.clone(),
                    dial_info_class: DialInfoClass::Mapped,
                    network_class: NetworkClass::InboundCapable,
                });
            }
            None
        });
        ord.push_back(do_mapped_fut);

        // Loop for restricted NAT retries
        loop {
            log_net!(debug
                "=== update_protocol_dialinfo {:?} {:?} tries_left={} ===",
                address_type,
                protocol_type,
                retry_count
            );
            // Get our external address from some fast node, call it node 1
            if !context.protocol_get_external_address_1().await {
                // If we couldn't get an external address, then we should just try the whole network class detection again later
                return Ok(());
            }

            // If our local interface list contains external_1 then there is no NAT in place

            let res = {
                let inner = context.inner.lock();
                inner
                    .intf_addrs
                    .as_ref()
                    .unwrap()
                    .contains(inner.external_1_address.as_ref().unwrap())
            };
            if res {
                // No NAT
                context.protocol_process_no_nat().await?;

                // No more retries
                break;
            }

            // There is -some NAT-
            if context.protocol_process_nat().await? {
                // We either got dial info or a network class without one
                break;
            }

            // If we tried everything, break anyway after N attempts
            if retry_count == 0 {
                break;
            }
            retry_count -= 1;
        }

        Ok(())
    }

    #[instrument(level = "trace", skip(self), err)]
    pub async fn update_with_discovery_context(&self, ctx: DiscoveryContext) -> EyreResult<()> {
        //
    }

    #[instrument(level = "trace", skip(self), err)]
    pub async fn do_public_dial_info_check(
        &self,
        stop_token: StopToken,
        _l: u64,
        _t: u64,
    ) -> EyreResult<()> {
        let routing_table = self.routing_table();

        // Figure out if we can optimize TCP/WS checking since they are often on the same port
        let (protocol_config, existing_network_class, tcp_same_port) = {
            let inner = self.inner.lock();
            let protocol_config = inner.protocol_config;
            let existing_network_class =
                routing_table.get_network_class(RoutingDomain::PublicInternet);
            let tcp_same_port = if protocol_config.inbound.contains(ProtocolType::TCP)
                && protocol_config.inbound.contains(ProtocolType::WS)
            {
                inner.tcp_port == inner.ws_port
            } else {
                false
            };
            (protocol_config, existing_network_class, tcp_same_port)
        };

        // Process all protocol and address combinations
        let mut unord = FuturesUnordered::new();
        // Do UDPv4+v6 at the same time as everything else
        if protocol_config.inbound.contains(ProtocolType::UDP) {
            // UDPv4
            if protocol_config.family_global.contains(AddressType::IPV4) {
                unord.push(
                    async {
                        let udpv4_context =
                            DiscoveryContext::new(self.routing_table(), self.clone());
                        if let Err(e) = self
                            .update_protocol_dialinfo(
                                &udpv4_context,
                                ProtocolType::UDP,
                                AddressType::IPV4,
                            )
                            .await
                        {
                            log_net!(debug "Failed UDPv4 dialinfo discovery: {}", e);
                            return None;
                        }
                        Some(udpv4_context)
                    }
                    .instrument(trace_span!("do_public_dial_info_check UDPv4"))
                    .boxed(),
                );
            }

            // UDPv6
            if protocol_config.family_global.contains(AddressType::IPV6) {
                unord.push(
                    async {
                        let udpv6_context =
                            DiscoveryContext::new(self.routing_table(), self.clone());
                        if let Err(e) = self
                            .update_protocol_dialinfo(
                                &udpv6_context,
                                ProtocolType::UDP,
                                AddressType::IPV6,
                            )
                            .await
                        {
                            log_net!(debug "Failed UDPv6 dialinfo discovery: {}", e);
                            return None;
                        }
                        Some(udpv6_context)
                    }
                    .instrument(trace_span!("do_public_dial_info_check UDPv6"))
                    .boxed(),
                );
            }
        }

        // Do TCPv4. Possibly do WSv4 if it is on a different port
        if protocol_config.family_global.contains(AddressType::IPV4) {
            if protocol_config.inbound.contains(ProtocolType::TCP) {
                unord.push(
                    async {
                        // TCPv4
                        let tcpv4_context =
                            DiscoveryContext::new(self.routing_table(), self.clone());
                        if let Err(e) = self
                            .update_protocol_dialinfo(
                                &tcpv4_context,
                                ProtocolType::TCP,
                                AddressType::IPV4,
                            )
                            .await
                        {
                            log_net!(debug "Failed TCPv4 dialinfo discovery: {}", e);
                            return None;
                        }
                        Some(tcpv4_context)
                    }
                    .instrument(trace_span!("do_public_dial_info_check TCPv4"))
                    .boxed(),
                );
            }

            if protocol_config.inbound.contains(ProtocolType::WS) && !tcp_same_port {
                unord.push(
                    async {
                        // WSv4
                        let wsv4_context =
                            DiscoveryContext::new(self.routing_table(), self.clone());
                        if let Err(e) = self
                            .update_protocol_dialinfo(
                                &wsv4_context,
                                ProtocolType::WS,
                                AddressType::IPV4,
                            )
                            .await
                        {
                            log_net!(debug "Failed WSv4 dialinfo discovery: {}", e);
                            return None;
                        }
                        Some(wsv4_context)
                    }
                    .instrument(trace_span!("do_public_dial_info_check WSv4"))
                    .boxed(),
                );
            }
        }

        // Do TCPv6. Possibly do WSv6 if it is on a different port
        if protocol_config.family_global.contains(AddressType::IPV6) {
            if protocol_config.inbound.contains(ProtocolType::TCP) {
                unord.push(
                    async {
                        // TCPv6
                        let tcpv6_context =
                            DiscoveryContext::new(self.routing_table(), self.clone());
                        if let Err(e) = self
                            .update_protocol_dialinfo(
                                &tcpv6_context,
                                ProtocolType::TCP,
                                AddressType::IPV6,
                            )
                            .await
                        {
                            log_net!(debug "Failed TCPv6 dialinfo discovery: {}", e);
                            return None;
                        }
                        Some(tcpv6_context)
                    }
                    .instrument(trace_span!("do_public_dial_info_check TCPv6"))
                    .boxed(),
                );
            }

            // WSv6
            if protocol_config.inbound.contains(ProtocolType::WS) && !tcp_same_port {
                unord.push(
                    async {
                        let wsv6_context =
                            DiscoveryContext::new(self.routing_table(), self.clone());
                        if let Err(e) = self
                            .update_protocol_dialinfo(
                                &wsv6_context,
                                ProtocolType::WS,
                                AddressType::IPV6,
                            )
                            .await
                        {
                            log_net!(debug "Failed WSv6 dialinfo discovery: {}", e);
                            return None;
                        }
                        Some(wsv6_context)
                    }
                    .instrument(trace_span!("do_public_dial_info_check WSv6"))
                    .boxed(),
                );
            }
        }

        // Wait for all discovery futures to complete and collect contexts
        let mut contexts = Vec::<DiscoveryContext>::new();
        let mut new_network_class = Option::<NetworkClass>::None;

        loop {
            match unord.next().timeout_at(stop_token.clone()).await {
                Ok(Some(ctx)) => {
                    if let Some(ctx) = ctx {
                        if let Some(nc) = ctx.inner.lock().detected_network_class {
                            if let Some(last_nc) = new_network_class {
                                if nc < last_nc {
                                    new_network_class = Some(nc);
                                }
                            } else {
                                new_network_class = Some(nc);
                            }
                        }

                        contexts.push(ctx);
                    }
                }
                Ok(None) => {
                    // Normal completion
                    break;
                }
                Err(_) => {
                    // Stop token, exit early without error propagation
                    return Ok(());
                }
            }
        }

        // If a network class could be determined
        // see about updating our public dial info
        let mut changed = false;
        let mut editor = routing_table.edit_routing_domain(RoutingDomain::PublicInternet);
        if new_network_class.is_some() {
            // Get existing public dial info
            let existing_public_dial_info: HashSet<DialInfoDetail> = routing_table
                .all_filtered_dial_info_details(
                    RoutingDomain::PublicInternet.into(),
                    &DialInfoFilter::all(),
                )
                .into_iter()
                .collect();

            // Get new public dial info and ensure it is valid
            let mut new_public_dial_info: HashSet<DialInfoDetail> = HashSet::new();
            for ctx in contexts {
                let inner = ctx.inner.lock();
                if let Some(pdi) = &inner.detected_public_dial_info {
                    if routing_table
                        .ensure_dial_info_is_valid(RoutingDomain::PublicInternet, &pdi.dial_info)
                    {
                        new_public_dial_info.insert(DialInfoDetail {
                            class: pdi.class,
                            dial_info: pdi.dial_info.clone(),
                        });
                    }

                    // duplicate for same port
                    if tcp_same_port && pdi.dial_info.protocol_type() == ProtocolType::TCP {
                        let ws_dial_info =
                            ctx.make_dial_info(pdi.dial_info.socket_address(), ProtocolType::WS);
                        if routing_table
                            .ensure_dial_info_is_valid(RoutingDomain::PublicInternet, &ws_dial_info)
                        {
                            new_public_dial_info.insert(DialInfoDetail {
                                class: pdi.class,
                                dial_info: ws_dial_info,
                            });
                        }
                    }
                }
            }

            // Is the public dial info different?
            if existing_public_dial_info != new_public_dial_info {
                // If so, clear existing public dial info and re-register the new public dial info
                editor.clear_dial_info_details();
                for did in new_public_dial_info {
                    if let Err(e) = editor.register_dial_info(did.dial_info, did.class) {
                        log_net!(error "Failed to register detected public dial info: {}", e);
                    }
                }
                changed = true;
            }

            // Is the network class different?
            if existing_network_class != new_network_class {
                editor.set_network_class(new_network_class);
                changed = true;
                log_net!(debug "PublicInternet network class changed to {:?}", new_network_class);
            }
        } else if existing_network_class.is_some() {
            // Network class could not be determined
            editor.clear_dial_info_details();
            editor.set_network_class(None);
            editor.clear_relay_node();
            changed = true;
            log_net!(debug "PublicInternet network class cleared");
        }

        // Punish nodes that told us our public address had changed when it didn't
        if !changed {
            if let Some(punish) = self.inner.lock().public_dial_info_check_punishment.take() {
                punish();
            }
        } else {
            // Commit updates
            editor.commit();
        }

        Ok(())
    }
    #[instrument(level = "trace", skip(self), err)]
    pub async fn update_network_class_task_routine(
        self,
        stop_token: StopToken,
        l: u64,
        t: u64,
    ) -> EyreResult<()> {
        // Do the public dial info check
        let out = self.do_public_dial_info_check(stop_token, l, t).await;

        // Done with public dial info check
        self.inner.lock().needs_public_dial_info_check = false;

        out
    }
}
