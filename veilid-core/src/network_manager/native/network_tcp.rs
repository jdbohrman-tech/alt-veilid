use super::*;
use async_tls::TlsAcceptor;
use sockets::*;
use stop_token::future::FutureExt;

/////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub(in crate::network_manager) struct ListenerState {
    pub protocol_accept_handlers: Vec<Box<dyn ProtocolAcceptHandler + 'static>>,
    pub tls_protocol_handlers: Vec<Box<dyn ProtocolAcceptHandler + 'static>>,
    pub tls_acceptor: Option<TlsAcceptor>,
}

impl ListenerState {
    pub fn new() -> Self {
        Self {
            protocol_accept_handlers: Vec::new(),
            tls_protocol_handlers: Vec::new(),
            tls_acceptor: None,
        }
    }
}

/////////////////////////////////////////////////////////////////

impl Network {
    fn get_or_create_tls_acceptor(&self) -> EyreResult<TlsAcceptor> {
        if let Some(ts) = self.inner.lock().tls_acceptor.as_ref() {
            return Ok(ts.clone());
        }

        let server_config = self
            .load_server_config()
            .wrap_err("Couldn't create TLS configuration")?;
        let acceptor = TlsAcceptor::from(server_config);
        self.inner.lock().tls_acceptor = Some(acceptor.clone());
        Ok(acceptor)
    }

    #[instrument(level = "trace", skip_all)]
    async fn try_tls_handlers(
        &self,
        tls_acceptor: &TlsAcceptor,
        stream: AsyncPeekStream,
        peer_addr: SocketAddr,
        local_addr: SocketAddr,
        protocol_handlers: &[Box<dyn ProtocolAcceptHandler>],
        tls_connection_initial_timeout_ms: u32,
    ) -> EyreResult<Option<ProtocolNetworkConnection>> {
        let tls_stream = tls_acceptor
            .accept(stream)
            .await
            .wrap_err("TLS stream failed handshake")?;
        let ps = AsyncPeekStream::new(tls_stream);
        let mut first_packet = [0u8; PEEK_DETECT_LEN];

        // Try the handlers but first get a chunk of data for them to process
        // Don't waste more than N seconds getting it though, in case someone
        // is trying to DoS us with a bunch of connections or something
        // read a chunk of the stream
        timeout(
            tls_connection_initial_timeout_ms,
            ps.peek_exact(&mut first_packet).in_current_span(),
        )
        .await
        .wrap_err("tls initial timeout")?
        .wrap_err("failed to peek tls stream")?;

        self.try_handlers(ps, peer_addr, local_addr, protocol_handlers)
            .await
    }

    #[instrument(level = "trace", skip_all)]
    async fn try_handlers(
        &self,
        stream: AsyncPeekStream,
        peer_addr: SocketAddr,
        local_addr: SocketAddr,
        protocol_accept_handlers: &[Box<dyn ProtocolAcceptHandler>],
    ) -> EyreResult<Option<ProtocolNetworkConnection>> {
        for ah in protocol_accept_handlers.iter() {
            if let Some(nc) = ah
                .on_accept(stream.clone(), peer_addr, local_addr)
                .await
                .wrap_err("io error")?
            {
                return Ok(Some(nc));
            }
        }

        Ok(None)
    }

    #[instrument(level = "trace", skip_all)]
    async fn tcp_acceptor(
        self,
        tcp_stream: io::Result<TcpStream>,
        listener_state: Arc<RwLock<ListenerState>>,
        connection_manager: ConnectionManager,
        connection_initial_timeout_ms: u32,
        tls_connection_initial_timeout_ms: u32,
    ) {
        let tcp_stream = match tcp_stream {
            Ok(v) => v,
            Err(_) => {
                // If this happened our low-level listener socket probably died
                // so it's time to restart the network
                self.inner.lock().network_needs_restart = true;
                return;
            }
        };

        // Limit the number of connections from the same IP address
        // and the number of total connections
        // XXX limiting here instead for connection table? may be faster and avoids tls negotiation
        let peer_addr = match tcp_stream.peer_addr() {
            Ok(addr) => addr,
            Err(e) => {
                log_net!(debug "failed to get peer address: {}", e);
                return;
            }
        };
        // Check to see if it is punished
        let address_filter = self.network_manager().address_filter();
        if address_filter.is_ip_addr_punished(peer_addr.ip()) {
            return;
        }

        let local_addr = match tcp_stream.local_addr() {
            Ok(addr) => addr,
            Err(e) => {
                log_net!(debug "failed to get local address: {}", e);
                return;
            }
        };

        #[cfg(all(feature = "rt-async-std", unix))]
        {
            // async-std does not directly support linger on TcpStream yet
            use std::os::fd::{AsRawFd, FromRawFd, IntoRawFd};
            if let Err(e) = unsafe {
                let s = socket2::Socket::from_raw_fd(tcp_stream.as_raw_fd());
                let res = s.set_linger(Some(core::time::Duration::from_secs(0)));
                s.into_raw_fd();
                res
            } {
                log_net!(debug "Couldn't set TCP linger: {}", e);
                return;
            }
        }
        #[cfg(all(feature = "rt-async-std", windows))]
        {
            // async-std does not directly support linger on TcpStream yet
            use std::os::windows::io::{AsRawSocket, FromRawSocket, IntoRawSocket};
            if let Err(e) = unsafe {
                let s = socket2::Socket::from_raw_socket(tcp_stream.as_raw_socket());
                let res = s.set_linger(Some(core::time::Duration::from_secs(0)));
                s.into_raw_socket();
                res
            } {
                log_net!(debug "Couldn't set TCP linger: {}", e);
                return;
            }
        }
        #[cfg(not(feature = "rt-async-std"))]
        if let Err(e) = tcp_stream.set_linger(Some(core::time::Duration::from_secs(0))) {
            log_net!(debug "Couldn't set TCP linger: {}", e);
            return;
        }
        if let Err(e) = tcp_stream.set_nodelay(true) {
            log_net!(debug "Couldn't set TCP nodelay: {}", e);
            return;
        }

        let listener_state = listener_state.clone();
        let connection_manager = connection_manager.clone();

        log_net!("TCP connection from: {}", peer_addr);

        // Create a stream we can peek on
        #[cfg(feature = "rt-tokio")]
        let tcp_stream = tcp_stream.compat();
        let ps = AsyncPeekStream::new(tcp_stream);

        /////////////////////////////////////////////////////////////
        let mut first_packet = [0u8; PEEK_DETECT_LEN];

        // read a chunk of the stream
        if timeout(
            connection_initial_timeout_ms,
            ps.peek_exact(&mut first_packet).in_current_span(),
        )
        .await
        .is_err()
        {
            // If we fail to get a packet within the connection initial timeout
            // then we punt this connection
            log_net!("connection initial timeout from: {:?}", peer_addr);
            return;
        }

        // Run accept handlers on accepted stream

        // Check if this could be TLS
        let ls = listener_state.read().clone();

        let conn = if ls.tls_acceptor.is_some() && first_packet[0] == 0x16 {
            self.try_tls_handlers(
                ls.tls_acceptor.as_ref().unwrap(),
                ps,
                peer_addr,
                local_addr,
                &ls.tls_protocol_handlers,
                tls_connection_initial_timeout_ms,
            )
            .await
        } else {
            self.try_handlers(ps, peer_addr, local_addr, &ls.protocol_accept_handlers)
                .await
        };

        let conn = match conn {
            Ok(Some(c)) => {
                log_net!("protocol handler found for {:?}: {:?}", peer_addr, c);
                c
            }
            Ok(None) => {
                // No protocol handlers matched? drop it.
                log_net!(debug "no protocol handler for connection from {:?}", peer_addr);
                return;
            }
            Err(e) => {
                // Failed to negotiate connection? drop it.
                log_net!(debug "failed to negotiate connection from {:?}: {}", peer_addr, e);
                return;
            }
        };

        // Register the new connection in the connection manager
        if let Err(e) = connection_manager
            .on_accepted_protocol_network_connection(conn)
            .await
        {
            log_net!(error "failed to register new connection: {}", e);
        }
    }

    #[instrument(level = "trace", skip_all)]
    async fn spawn_socket_listener(&self, addr: SocketAddr) -> EyreResult<bool> {
        // Get config
        let (connection_initial_timeout_ms, tls_connection_initial_timeout_ms) = {
            let c = self.config.get();
            (
                c.network.connection_initial_timeout_ms,
                c.network.tls.connection_initial_timeout_ms,
            )
        };

        // Create a socket and bind it
        let Some(socket) = new_bound_default_tcp_socket(addr)
            .wrap_err("failed to create default socket listener")?
        else {
            return Ok(false);
        };

        // Drop the socket
        drop(socket);

        // Create a shared socket and bind it once we have determined the port is free
        let Some(socket) = new_bound_shared_tcp_socket(addr)
            .wrap_err("failed to create shared socket listener")?
        else {
            return Ok(false);
        };

        // Listen on the socket
        if socket.listen(128).is_err() {
            return Ok(false);
        }

        // Make an async tcplistener from the socket2 socket
        let std_listener: std::net::TcpListener = socket.into();
        cfg_if! {
            if #[cfg(feature="rt-async-std")] {
                let listener = TcpListener::from(std_listener);
            } else if #[cfg(feature="rt-tokio")] {
                std_listener.set_nonblocking(true).expect("failed to set nonblocking");
                let listener = TcpListener::from_std(std_listener).wrap_err("failed to create tokio tcp listener")?;
            } else {
                compile_error!("needs executor implementation")
            }
        }

        log_net!(debug "spawn_socket_listener: binding successful to {}", addr);

        // Create protocol handler records
        let listener_state = Arc::new(RwLock::new(ListenerState::new()));
        self.inner
            .lock()
            .listener_states
            .insert(addr, listener_state.clone());

        // Spawn the socket task
        let this = self.clone();
        let stop_token = self.inner.lock().stop_source.as_ref().unwrap().token();
        let connection_manager = self.connection_manager();

        ////////////////////////////////////////////////////////////
        let jh = spawn(&format!("TCP listener {}", addr), async move {
            // moves listener object in and get incoming iterator
            // when this task exists, the listener will close the socket

            cfg_if! {
                if #[cfg(feature="rt-async-std")] {
                    let incoming_stream = listener.incoming();
                } else if #[cfg(feature="rt-tokio")] {
                    let incoming_stream = tokio_stream::wrappers::TcpListenerStream::new(listener);
                } else {
                    compile_error!("needs executor implementation")
                }
            }

            let _ = incoming_stream
                .for_each_concurrent(None, |tcp_stream| {
                    let this = this.clone();
                    let listener_state = listener_state.clone();
                    let connection_manager = connection_manager.clone();
                    Self::tcp_acceptor(
                        this,
                        tcp_stream,
                        listener_state,
                        connection_manager,
                        connection_initial_timeout_ms,
                        tls_connection_initial_timeout_ms,
                    )
                })
                .timeout_at(stop_token)
                .await;

            log_net!(debug "exited incoming loop for {}", addr);
            // Remove our listener state from this address if we're stopping
            this.inner.lock().listener_states.remove(&addr);
            log_net!(debug "listener state removed for {}", addr);
        });
        ////////////////////////////////////////////////////////////

        // Add to join handles
        self.add_to_join_handles(jh);

        Ok(true)
    }

    /////////////////////////////////////////////////////////////////

    // TCP listener that multiplexes ports so multiple protocols can exist on a single port
    #[instrument(level = "trace", skip_all)]
    pub(super) async fn start_tcp_listener(
        &self,
        bind_set: NetworkBindSet,
        is_tls: bool,
        new_protocol_accept_handler: Box<NewProtocolAcceptHandler>,
    ) -> EyreResult<Option<Vec<SocketAddress>>> {
        let mut out = Vec::<SocketAddress>::new();

        for ip_addr in bind_set.addrs {
            let mut port = bind_set.port;
            loop {
                let addr = SocketAddr::new(ip_addr, port);

                // see if we've already bound to this already
                // if not, spawn a listener
                let mut got_listener = false;
                if !self.inner.lock().listener_states.contains_key(&addr) {
                    if self.clone().spawn_socket_listener(addr).await? {
                        got_listener = true;
                    }
                } else {
                    got_listener = true;
                }

                if got_listener {
                    let ls = if let Some(ls) = self.inner.lock().listener_states.get_mut(&addr) {
                        ls.clone()
                    } else {
                        panic!("this shouldn't happen");
                    };

                    if is_tls {
                        if ls.read().tls_acceptor.is_none() {
                            ls.write().tls_acceptor =
                                Some(self.clone().get_or_create_tls_acceptor()?);
                        }
                        ls.write()
                            .tls_protocol_handlers
                            .push(new_protocol_accept_handler(
                                self.network_manager().config(),
                                true,
                            ));
                    } else {
                        ls.write()
                            .protocol_accept_handlers
                            .push(new_protocol_accept_handler(
                                self.network_manager().config(),
                                false,
                            ));
                    }

                    // Return interface dial infos we listen on
                    let idi_addrs = self.translate_unspecified_address(&addr);
                    for idi_addr in idi_addrs {
                        out.push(SocketAddress::from_socket_addr(idi_addr));
                    }
                    break;
                }

                if !bind_set.search {
                    log_net!(debug "unable to bind to tcp {}", addr);
                    return Ok(None);
                }

                if port == 65535u16 {
                    port = 1024;
                } else {
                    port += 1;
                }

                if port == bind_set.port {
                    bail!("unable to find a free port for tcp {}", ip_addr);
                }
            }
        }

        Ok(Some(out))
    }
}
