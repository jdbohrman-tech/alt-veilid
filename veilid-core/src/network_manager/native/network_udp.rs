use super::*;
use stop_token::future::FutureExt;

impl Network {
    #[instrument(level = "trace", skip_all)]
    pub(super) async fn create_udp_listener_tasks(&self) -> EyreResult<()> {
        // Spawn socket tasks
        let mut task_count = self
            .config()
            .with(|c| c.network.protocol.udp.socket_pool_size);
        if task_count == 0 {
            task_count = get_concurrency() / 2;
            if task_count == 0 {
                task_count = 1;
            }
        }
        veilid_log!(self trace "task_count: {}", task_count);
        for task_n in 0..task_count {
            veilid_log!(self trace "Spawning UDP listener task");

            ////////////////////////////////////////////////////////////
            // Run thread task to process stream of messages
            let this = self.clone();

            let jh = spawn(&format!("UDP listener {}", task_n), async move {
                veilid_log!(this trace "UDP listener task spawned");

                // Collect all our protocol handlers into a vector
                let protocol_handlers: Vec<RawUdpProtocolHandler> = this
                    .inner
                    .lock()
                    .udp_protocol_handlers
                    .values()
                    .cloned()
                    .collect();

                // Spawn a local async task for each socket
                let mut protocol_handlers_unordered = FuturesUnordered::new();
                let stop_token = {
                    let inner = this.inner.lock();
                    if inner.stop_source.is_none() {
                        veilid_log!(this debug "exiting UDP listener before it starts because we encountered an error");
                        return;
                    }
                    inner.stop_source.as_ref().unwrap().token()
                };

                for ph in protocol_handlers {
                    let network_manager = this.network_manager();
                    let stop_token = stop_token.clone();
                    let ph_future = async move {
                        let mut data = vec![0u8; 65536];

                        loop {
                            match ph
                                .recv_message(&mut data)
                                .timeout_at(stop_token.clone())
                                .in_current_span()
                                .await
                            {
                                Ok(Ok((size, flow))) => {
                                    // Network accounting
                                    network_manager.stats_packet_rcvd(
                                        flow.remote_address().ip_addr(),
                                        ByteCount::new(size as u64),
                                    );

                                    // Pass it up for processing
                                    if let Err(e) = network_manager
                                        .on_recv_envelope(&mut data[..size], flow)
                                        .await
                                    {
                                        veilid_log!(network_manager debug "failed to process received udp envelope: {}", e);
                                    }
                                }
                                Ok(Err(_)) => {
                                    return false;
                                }
                                Err(_) => {
                                    return true;
                                }
                            }
                        }
                    };

                    protocol_handlers_unordered.push(ph_future);
                }
                // Now we wait for join handles to exit,
                // if any error out it indicates an error needing
                // us to completely restart the network
                while let Some(v) = protocol_handlers_unordered.next().in_current_span().await {
                    // true = stopped, false = errored
                    if !v {
                        // If any protocol handler fails, our socket died and we need to restart the network
                        this.inner.lock().network_needs_restart = true;
                    }
                }

                veilid_log!(this trace "UDP listener task stopped");
            }.instrument(trace_span!(parent: None, "UDP Listener")));
            ////////////////////////////////////////////////////////////

            // Add to join handle
            self.add_to_join_handles(jh);
        }

        Ok(())
    }

    #[instrument(level = "trace", skip_all)]
    async fn create_udp_protocol_handler(&self, addr: SocketAddr) -> EyreResult<bool> {
        veilid_log!(self debug "create_udp_protocol_handler on {:?}", &addr);

        // Create a single-address-family UDP socket with default options bound to an address
        let Some(udp_socket) = bind_async_udp_socket(addr)? else {
            return Ok(false);
        };
        let socket_arc = Arc::new(udp_socket);

        // Create protocol handler
        let protocol_handler = RawUdpProtocolHandler::new(self.registry(), socket_arc);

        // Record protocol handler
        let mut inner = self.inner.lock();
        inner
            .udp_protocol_handlers
            .insert(addr, protocol_handler.clone());
        if addr.is_ipv4() && inner.default_udpv4_protocol_handler.is_none() {
            inner.default_udpv4_protocol_handler = Some(protocol_handler);
        } else if addr.is_ipv6() && inner.default_udpv6_protocol_handler.is_none() {
            inner.default_udpv6_protocol_handler = Some(protocol_handler);
        }

        Ok(true)
    }

    #[instrument(level = "trace", skip_all)]
    pub(super) async fn create_udp_protocol_handlers(
        &self,
        bind_set: NetworkBindSet,
    ) -> EyreResult<bool> {
        for ip_addr in bind_set.addrs {
            let mut port = bind_set.port;
            loop {
                let addr = SocketAddr::new(ip_addr, port);

                // see if we've already bound to this already
                // if not, spawn a listener
                if !self.inner.lock().udp_protocol_handlers.contains_key(&addr) {
                    let bound = self.clone().create_udp_protocol_handler(addr).await?;

                    // Return interface dial infos we listen on
                    if bound {
                        let mut inner = self.inner.lock();
                        let bapp = inner
                            .bound_address_per_protocol
                            .entry(ProtocolType::UDP)
                            .or_default();
                        bapp.push(addr);

                        veilid_log!(self
                            debug
                            "set_preferred_local_address: {:?} {:?} -> {:?}",
                            ProtocolType::UDP,
                            addr,
                            PeerAddress::new(SocketAddress::from_socket_addr(addr), ProtocolType::UDP)
                        );

                        Self::set_preferred_local_address(
                            &mut inner,
                            PeerAddress::new(
                                SocketAddress::from_socket_addr(addr),
                                ProtocolType::UDP,
                            ),
                        );

                        break;
                    }
                }
                if !bind_set.search {
                    veilid_log!(self debug "unable to bind to udp {}", addr);
                    return Ok(false);
                }

                if port == 65535u16 {
                    port = 1024;
                } else {
                    port += 1;
                }

                if port == bind_set.port {
                    bail!("unable to find a free port for udp {}", ip_addr);
                }
            }
        }
        Ok(true)
    }

    /////////////////////////////////////////////////////////////////

    pub(super) fn find_best_udp_protocol_handler(
        &self,
        peer_socket_addr: &SocketAddr,
        local_socket_addr: &Option<SocketAddr>,
    ) -> Option<RawUdpProtocolHandler> {
        let inner = self.inner.lock();
        // if our last communication with this peer came from a particular inbound udp protocol handler, use it
        if let Some(sa) = local_socket_addr {
            if let Some(ph) = inner.udp_protocol_handlers.get(sa) {
                return Some(ph.clone());
            }
        }

        // otherwise find the first outbound udp protocol handler that matches the ip protocol version of the peer addr
        match peer_socket_addr {
            SocketAddr::V4(_) => inner.udp_protocol_handlers.iter().find_map(|x| {
                if x.0.is_ipv4() {
                    Some(x.1.clone())
                } else {
                    None
                }
            }),
            SocketAddr::V6(_) => inner.udp_protocol_handlers.iter().find_map(|x| {
                if x.0.is_ipv6() {
                    Some(x.1.clone())
                } else {
                    None
                }
            }),
        }
    }
}
