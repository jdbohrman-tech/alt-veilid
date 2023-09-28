/// Detect NetworkClass and DialInfo for the DialInfo for the PublicInternet RoutingDomain
use super::*;
use futures_util::stream::FuturesUnordered;
use stop_token::future::FutureExt as StopTokenFutureExt;

impl Network {
    #[instrument(level = "trace", skip(self), err)]
    pub async fn update_with_detected_dial_info(&self, ddi: DetectedDialInfo) -> EyreResult<()> {
        let existing_network_class = self
            .routing_table()
            .get_network_class(RoutingDomain::PublicInternet)
            .unwrap_or_default();

        match ddi {
            DetectedDialInfo::SymmetricNAT => {
                // If we get any symmetric nat dialinfo, this whole network class is outbound only,
                // and all dial info should be treated as invalid
                if !matches!(existing_network_class, NetworkClass::OutboundOnly) {
                    let mut editor = self
                        .routing_table()
                        .edit_routing_domain(RoutingDomain::PublicInternet);

                    editor.clear_dial_info_details(None, None);
                    editor.set_network_class(Some(NetworkClass::OutboundOnly));
                    editor.clear_relay_node();
                    editor.commit(true).await;
                }
            }
            DetectedDialInfo::Detected(did) => {
                // get existing dial info into table by protocol/address type
                let mut existing_dial_info =
                    BTreeMap::<(ProtocolType, AddressType), DialInfoDetail>::new();
                for did in self.routing_table().all_filtered_dial_info_details(
                    RoutingDomain::PublicInternet.into(),
                    &DialInfoFilter::all(),
                ) {
                    // Only need to keep one per pt/at pair, since they will all have the same dialinfoclass
                    existing_dial_info.insert(
                        (did.dial_info.protocol_type(), did.dial_info.address_type()),
                        did,
                    );
                }
                // We got a dial info, upgrade everything unless we are fixed to outbound only due to a symmetric nat
                if !matches!(existing_network_class, NetworkClass::OutboundOnly) {
                    // Get existing dial info for protocol/address type combination
                    let pt = did.dial_info.protocol_type();
                    let at = did.dial_info.address_type();

                    // See what operations to perform with this dialinfo
                    let mut clear = false;
                    let mut add = false;

                    if let Some(edi) = existing_dial_info.get(&(pt, at)) {
                        if did.class < edi.class {
                            // Better dial info class was found, clear existing dialinfo for this pt/at pair
                            clear = true;
                            add = true;
                        } else if did.class == edi.class {
                            // Same dial info class, just add dial info
                            add = true;
                        }
                        // Otherwise, don't upgrade, don't add, this is worse than what we have already
                    } else {
                        // No existing dial info of this type accept it, no need to upgrade, but add it
                        add = true;
                    }

                    if clear || add {
                        let mut editor = self
                            .routing_table()
                            .edit_routing_domain(RoutingDomain::PublicInternet);

                        if clear {
                            editor.clear_dial_info_details(
                                Some(did.dial_info.address_type()),
                                Some(did.dial_info.protocol_type()),
                            );
                        }

                        if add {
                            if let Err(e) =
                                editor.register_dial_info(did.dial_info.clone(), did.class)
                            {
                                log_net!(debug "Failed to register detected dialinfo {:?}: {}", did, e);
                            }
                        }

                        editor.set_network_class(Some(NetworkClass::InboundCapable));
                        editor.commit(true).await;
                    }
                }
            }
        }
        Ok(())
    }

    #[instrument(level = "trace", skip(self), err)]
    pub async fn do_public_dial_info_check(
        &self,
        stop_token: StopToken,
        _l: u64,
        _t: u64,
    ) -> EyreResult<()> {
        // Figure out if we can optimize TCP/WS checking since they are often on the same port
        let (protocol_config, tcp_same_port) = {
            let inner = self.inner.lock();
            let protocol_config = inner.protocol_config;
            let tcp_same_port = if protocol_config.inbound.contains(ProtocolType::TCP)
                && protocol_config.inbound.contains(ProtocolType::WS)
            {
                inner.tcp_port == inner.ws_port
            } else {
                false
            };
            (protocol_config, tcp_same_port)
        };

        // Save off existing public dial info for change detection later
        let existing_public_dial_info: HashSet<DialInfoDetail> = self
            .routing_table()
            .all_filtered_dial_info_details(
                RoutingDomain::PublicInternet.into(),
                &DialInfoFilter::all(),
            )
            .into_iter()
            .collect();

        // Clear public dialinfo and network class in prep for discovery
        let mut editor = self
            .routing_table()
            .edit_routing_domain(RoutingDomain::PublicInternet);
        editor.clear_dial_info_details(None, None);
        editor.set_network_class(None);
        editor.clear_relay_node();
        editor.commit(true).await;

        // Process all protocol and address combinations
        let mut unord = FuturesUnordered::new();
        // Do UDPv4+v6 at the same time as everything else
        if protocol_config.inbound.contains(ProtocolType::UDP) {
            // UDPv4
            if protocol_config.family_global.contains(AddressType::IPV4) {
                let udpv4_context = DiscoveryContext::new(
                    self.routing_table(),
                    self.clone(),
                    ProtocolType::UDP,
                    AddressType::IPV4,
                );
                udpv4_context
                    .discover(&mut unord)
                    .instrument(trace_span!("udpv4_context.discover"))
                    .await;
            }

            // UDPv6
            if protocol_config.family_global.contains(AddressType::IPV6) {
                let udpv6_context = DiscoveryContext::new(
                    self.routing_table(),
                    self.clone(),
                    ProtocolType::UDP,
                    AddressType::IPV6,
                );
                udpv6_context
                    .discover(&mut unord)
                    .instrument(trace_span!("udpv6_context.discover"))
                    .await;
            }
        }

        // Do TCPv4. Possibly do WSv4 if it is on a different port
        if protocol_config.family_global.contains(AddressType::IPV4) {
            if protocol_config.inbound.contains(ProtocolType::TCP) {
                let tcpv4_context = DiscoveryContext::new(
                    self.routing_table(),
                    self.clone(),
                    ProtocolType::TCP,
                    AddressType::IPV4,
                );
                tcpv4_context
                    .discover(&mut unord)
                    .instrument(trace_span!("tcpv4_context.discover"))
                    .await;
            }

            if protocol_config.inbound.contains(ProtocolType::WS) && !tcp_same_port {
                let wsv4_context = DiscoveryContext::new(
                    self.routing_table(),
                    self.clone(),
                    ProtocolType::WS,
                    AddressType::IPV4,
                );
                wsv4_context
                    .discover(&mut unord)
                    .instrument(trace_span!("wsv4_context.discover"))
                    .await;
            }
        }

        // Do TCPv6. Possibly do WSv6 if it is on a different port
        if protocol_config.family_global.contains(AddressType::IPV6) {
            if protocol_config.inbound.contains(ProtocolType::TCP) {
                let tcpv6_context = DiscoveryContext::new(
                    self.routing_table(),
                    self.clone(),
                    ProtocolType::TCP,
                    AddressType::IPV6,
                );
                tcpv6_context
                    .discover(&mut unord)
                    .instrument(trace_span!("tcpv6_context.discover"))
                    .await;
            }

            // WSv6
            if protocol_config.inbound.contains(ProtocolType::WS) && !tcp_same_port {
                let wsv6_context = DiscoveryContext::new(
                    self.routing_table(),
                    self.clone(),
                    ProtocolType::WS,
                    AddressType::IPV6,
                );
                wsv6_context
                    .discover(&mut unord)
                    .instrument(trace_span!("wsv6_context.discover"))
                    .await;
            }
        }

        // Wait for all discovery futures to complete and apply discoverycontexts
        loop {
            match unord.next().timeout_at(stop_token.clone()).await {
                Ok(Some(Some(ddi))) => {
                    // Found some new dial info for this protocol/address combination
                    self.update_with_detected_dial_info(ddi.clone()).await?;

                    // Add WS dialinfo as well if it is on the same port as TCP
                    if let DetectedDialInfo::Detected(did) = &ddi {
                        if did.dial_info.protocol_type() == ProtocolType::TCP && tcp_same_port {
                            // Make WS dialinfo as well with same socket address as TCP
                            let ws_ddi = DetectedDialInfo::Detected(DialInfoDetail {
                                dial_info: self.make_dial_info(
                                    did.dial_info.socket_address(),
                                    ProtocolType::WS,
                                ),
                                class: did.class,
                            });
                            // Add additional WS dialinfo
                            self.update_with_detected_dial_info(ws_ddi).await?;
                        }
                    }
                }
                Ok(Some(None)) => {
                    // Found no new dial info for this protocol/address combination
                }
                Ok(None) => {
                    // All done, normally
                    break;
                }
                Err(_) => {
                    // Stop token, exit early without error propagation
                    return Ok(());
                }
            }
        }

        // All done, see if things changed
        let new_public_dial_info: HashSet<DialInfoDetail> = self
            .routing_table()
            .all_filtered_dial_info_details(
                RoutingDomain::PublicInternet.into(),
                &DialInfoFilter::all(),
            )
            .into_iter()
            .collect();

        // Punish nodes that told us our public address had changed when it didn't
        if new_public_dial_info == existing_public_dial_info {
            if let Some(punish) = self.inner.lock().public_dial_info_check_punishment.take() {
                punish();
            }
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
        {
            let mut inner = self.inner.lock();
            inner.needs_public_dial_info_check = false;
            inner.public_dial_info_check_punishment = None;
        }

        out
    }

    /// Make a dialinfo from an address and protocol type
    pub fn make_dial_info(&self, addr: SocketAddress, protocol_type: ProtocolType) -> DialInfo {
        match protocol_type {
            ProtocolType::UDP => DialInfo::udp(addr),
            ProtocolType::TCP => DialInfo::tcp(addr),
            ProtocolType::WS => {
                let c = self.config.get();
                DialInfo::try_ws(
                    addr,
                    format!("ws://{}/{}", addr, c.network.protocol.ws.path),
                )
                .unwrap()
            }
            ProtocolType::WSS => panic!("none of the discovery functions are used for wss"),
        }
    }
}
