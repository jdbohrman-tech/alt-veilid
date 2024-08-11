use super::*;
use sockets::*;
use stop_token::future::FutureExt;

impl Network {
    #[instrument(level = "trace", skip_all)]
    pub(super) async fn create_udp_listener_tasks(&self) -> EyreResult<()> {
        // Spawn socket tasks
        let mut task_count = {
            let c = self.config.get();
            c.network.protocol.udp.socket_pool_size
        };
        if task_count == 0 {
            task_count = get_concurrency() / 2;
            if task_count == 0 {
                task_count = 1;
            }
        }
        log_net!("task_count: {}", task_count);
        for task_n in 0..task_count {
            log_net!("Spawning UDP listener task");

            ////////////////////////////////////////////////////////////
            // Run thread task to process stream of messages
            let this = self.clone();

            let jh = spawn(&format!("UDP listener {}", task_n), async move {
                log_net!("UDP listener task spawned");

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
                let network_manager = this.network_manager();
                let stop_token = {
                    let inner = this.inner.lock();
                    if inner.stop_source.is_none() {
                        log_net!(debug "exiting UDP listener before it starts because we encountered an error");
                        return;
                    }
                    inner.stop_source.as_ref().unwrap().token()
                };

                for ph in protocol_handlers {
                    let network_manager = network_manager.clone();
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
                                        log_net!(debug "failed to process received udp envelope: {}", e);
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

                log_net!("UDP listener task stopped");
            }.instrument(trace_span!(parent: None, "UDP Listener")));
            ////////////////////////////////////////////////////////////

            // Add to join handle
            self.add_to_join_handles(jh);
        }

        Ok(())
    }

    #[instrument(level = "trace", skip_all)]
    async fn create_udp_protocol_handler(&self, addr: SocketAddr) -> EyreResult<bool> {
        log_net!(debug "create_udp_protocol_handler on {:?}", &addr);

        // Create a reusable socket
        let Some(socket) = new_bound_default_udp_socket(addr)? else {
            return Ok(false);
        };

        // Make an async UdpSocket from the socket2 socket
        let std_udp_socket: std::net::UdpSocket = socket.into();
        cfg_if! {
            if #[cfg(feature="rt-async-std")] {
                let udp_socket = UdpSocket::from(std_udp_socket);
            } else if #[cfg(feature="rt-tokio")] {
                std_udp_socket.set_nonblocking(true).expect("failed to set nonblocking");
                let udp_socket = UdpSocket::from_std(std_udp_socket).wrap_err("failed to make inbound tokio udpsocket")?;
            } else {
                compile_error!("needs executor implementation");
            }
        }
        let socket_arc = Arc::new(udp_socket);

        // Create protocol handler
        let protocol_handler =
            RawUdpProtocolHandler::new(socket_arc, Some(self.network_manager().address_filter()));

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
    ) -> EyreResult<Option<Vec<DialInfo>>> {
        let mut out = Vec::<DialInfo>::new();

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
                        let idi_addrs = self.translate_unspecified_address(&addr);
                        for idi_addr in idi_addrs {
                            out.push(DialInfo::udp_from_socketaddr(idi_addr));
                        }
                        break;
                    }
                }

                if !bind_set.search {
                    log_net!(debug "unable to bind to udp {}", addr);
                    return Ok(None);
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
        Ok(Some(out))
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
