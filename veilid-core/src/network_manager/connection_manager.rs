use super::*;
use crate::xx::*;
use connection_table::*;
use network_connection::*;
use stop_token::future::FutureExt;

///////////////////////////////////////////////////////////
// Connection manager

#[derive(Debug)]
enum ConnectionManagerEvent {
    Accepted(ProtocolNetworkConnection),
    Dead(NetworkConnection),
    Finished(ConnectionDescriptor),
}

#[derive(Debug)]
struct ConnectionManagerInner {
    connection_table: ConnectionTable,
    sender: flume::Sender<ConnectionManagerEvent>,
    async_processor_jh: Option<MustJoinHandle<()>>,
    stop_source: Option<StopSource>,
}

struct ConnectionManagerArc {
    network_manager: NetworkManager,
    inner: Mutex<Option<ConnectionManagerInner>>,
}
impl core::fmt::Debug for ConnectionManagerArc {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ConnectionManagerArc")
            .field("inner", &self.inner)
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct ConnectionManager {
    arc: Arc<ConnectionManagerArc>,
}

impl ConnectionManager {
    fn new_inner(
        config: VeilidConfig,
        stop_source: StopSource,
        sender: flume::Sender<ConnectionManagerEvent>,
        async_processor_jh: MustJoinHandle<()>,
    ) -> ConnectionManagerInner {
        ConnectionManagerInner {
            stop_source: Some(stop_source),
            sender: sender,
            async_processor_jh: Some(async_processor_jh),
            connection_table: ConnectionTable::new(config),
        }
    }
    fn new_arc(network_manager: NetworkManager) -> ConnectionManagerArc {
        ConnectionManagerArc {
            network_manager,
            inner: Mutex::new(None),
        }
    }
    pub fn new(network_manager: NetworkManager) -> Self {
        Self {
            arc: Arc::new(Self::new_arc(network_manager)),
        }
    }

    pub fn network_manager(&self) -> NetworkManager {
        self.arc.network_manager.clone()
    }

    pub async fn startup(&self) {
        trace!("startup connection manager");
        let mut inner = self.arc.inner.lock();
        if inner.is_some() {
            panic!("shouldn't start connection manager twice without shutting it down first");
        }

        // Create channel for async_processor to receive notifications of networking events
        let (sender, receiver) = flume::unbounded();

        // Create the stop source we'll use to stop the processor and the connection table
        let stop_source = StopSource::new();

        // Spawn the async processor
        let async_processor = spawn(self.clone().async_processor(stop_source.token(), receiver));

        // Store in the inner object
        *inner = Some(Self::new_inner(
            self.network_manager().config(),
            stop_source,
            sender,
            async_processor,
        ));
    }

    pub async fn shutdown(&self) {
        debug!("starting connection manager shutdown");
        // Remove the inner from the lock
        let mut inner = {
            let mut inner_lock = self.arc.inner.lock();
            let inner = match inner_lock.take() {
                Some(v) => v,
                None => {
                    panic!("not started");
                }
            };
            inner
        };

        // Stop all the connections and the async processor
        debug!("stopping async processor task");
        drop(inner.stop_source.take());
        let async_processor_jh = inner.async_processor_jh.take().unwrap();
        // wait for the async processor to stop
        debug!("waiting for async processor to stop");
        async_processor_jh.await;
        // Wait for the connections to complete
        debug!("waiting for connection handlers to complete");
        inner.connection_table.join().await;
        debug!("finished connection manager shutdown");
    }

    // Returns a network connection if one already is established
    pub fn get_connection(&self, descriptor: ConnectionDescriptor) -> Option<ConnectionHandle> {
        let mut inner = self.arc.inner.lock();
        let inner = match &mut *inner {
            Some(v) => v,
            None => {
                panic!("not started");
            }
        };
        inner.connection_table.get_connection(descriptor)
    }

    // Internal routine to register new connection atomically.
    // Registers connection in the connection table for later access
    // and spawns a message processing loop for the connection
    fn on_new_protocol_network_connection(
        &self,
        inner: &mut ConnectionManagerInner,
        prot_conn: ProtocolNetworkConnection,
    ) -> EyreResult<NetworkResult<ConnectionHandle>> {
        log_net!("on_new_protocol_network_connection: {:?}", prot_conn);

        // Wrap with NetworkConnection object to start the connection processing loop
        let stop_token = match &inner.stop_source {
            Some(ss) => ss.token(),
            None => bail!("not creating connection because we are stopping"),
        };

        let conn = NetworkConnection::from_protocol(self.clone(), stop_token, prot_conn);
        let handle = conn.get_handle();
        // Add to the connection table
        match inner.connection_table.add_connection(conn) {
            Ok(None) => {
                // Connection added
            }
            Ok(Some(conn)) => {
                // Connection added and a different one LRU'd out
                let _ = inner.sender.send(ConnectionManagerEvent::Dead(conn));
            }
            Err(ConnectionTableAddError::AddressFilter(conn, e)) => {
                // Connection filtered
                let desc = conn.connection_descriptor();
                let _ = inner.sender.send(ConnectionManagerEvent::Dead(conn));
                return Err(eyre!("connection filtered: {:?} ({})", desc, e));
            }
            Err(ConnectionTableAddError::AlreadyExists(conn)) => {
                // Connection already exists
                let desc = conn.connection_descriptor();
                let _ = inner.sender.send(ConnectionManagerEvent::Dead(conn));
                return Err(eyre!("connection already exists: {:?}", desc));
            }
        };
        Ok(NetworkResult::Value(handle))
    }

    // Called when we want to create a new connection or get the current one that already exists
    // This will kill off any connections that are in conflict with the new connection to be made
    // in order to make room for the new connection in the system's connection table
    pub async fn get_or_create_connection(
        &self,
        local_addr: Option<SocketAddr>,
        dial_info: DialInfo,
    ) -> EyreResult<NetworkResult<ConnectionHandle>> {
        let killed = {
            let mut inner = self.arc.inner.lock();
            let inner = match &mut *inner {
                Some(v) => v,
                None => {
                    panic!("not started");
                }
            };

            log_net!(
                "== get_or_create_connection local_addr={:?} dial_info={:?}",
                local_addr.green(),
                dial_info.green()
            );

            let peer_address = dial_info.to_peer_address();

            // Make a connection to the address
            // reject connections to addresses with an unknown or unsupported peer scope
            let descriptor = match local_addr {
                Some(la) => {
                    ConnectionDescriptor::new(peer_address, SocketAddress::from_socket_addr(la))
                }
                None => ConnectionDescriptor::new_no_local(peer_address),
            }?;

            // If any connection to this remote exists that has the same protocol, return it
            // Any connection will do, we don't have to match the local address

            if let Some(conn) = inner
                .connection_table
                .get_last_connection_by_remote(descriptor.remote())
            {
                log_net!(
                    "== Returning existing connection local_addr={:?} peer_address={:?}",
                    local_addr.green(),
                    peer_address.green()
                );

                return Ok(NetworkResult::Value(conn));
            }

            // Drop any other protocols connections to this remote that have the same local addr
            // otherwise this connection won't succeed due to binding
            let mut killed = Vec::<NetworkConnection>::new();
            if let Some(local_addr) = local_addr {
                if local_addr.port() != 0 {
                    for pt in [ProtocolType::TCP, ProtocolType::WS, ProtocolType::WSS] {
                        let pa = PeerAddress::new(descriptor.remote_address().clone(), pt);
                        for prior_descriptor in inner
                            .connection_table
                            .get_connection_descriptors_by_remote(pa)
                        {
                            let mut kill = false;
                            // See if the local address would collide
                            if let Some(prior_local) = prior_descriptor.local() {
                                if (local_addr.ip().is_unspecified()
                                    || prior_local.to_ip_addr().is_unspecified()
                                    || (local_addr.ip() == prior_local.to_ip_addr()))
                                    && prior_local.port() == local_addr.port()
                                {
                                    kill = true;
                                }
                            }
                            if kill {
                                log_net!(debug
                                    ">< Terminating connection prior_descriptor={:?}",
                                    prior_descriptor
                                );
                                let mut conn = inner
                                    .connection_table
                                    .remove_connection(prior_descriptor)
                                    .expect("connection not in table");

                                conn.close();

                                killed.push(conn);
                            }
                        }
                    }
                }
            }
            killed
        };

        // Wait for the killed connections to end their recv loops
        let mut retry_count = if !killed.is_empty() { 2 } else { 0 };
        for k in killed {
            k.await;
        }

        // Get connection timeout
        let timeout_ms = {
            let config = self.network_manager().config();
            let c = config.get();
            c.network.connection_initial_timeout_ms
        };

        // Attempt new connection
        let conn = network_result_try!(loop {
            let result_net_res =
                ProtocolNetworkConnection::connect(local_addr, &dial_info, timeout_ms).await;
            match result_net_res {
                Ok(net_res) => {
                    if net_res.is_value() || retry_count == 0 {
                        break net_res;
                    }
                }
                Err(e) => {
                    if retry_count == 0 {
                        return Err(e).wrap_err("failed to connect");
                    }
                }
            };
            log_net!(debug "get_or_create_connection retries left: {}", retry_count);
            retry_count -= 1;
            intf::sleep(500).await;
        });

        // Add to the connection table
        let mut inner = self.arc.inner.lock();
        let inner = match &mut *inner {
            Some(v) => v,
            None => {
                bail!("shutting down");
            }
        };
        self.on_new_protocol_network_connection(inner, conn)
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////////////
    /// Callbacks

    #[instrument(level = "trace", skip_all)]
    async fn async_processor(
        self,
        stop_token: StopToken,
        receiver: flume::Receiver<ConnectionManagerEvent>,
    ) {
        // Process async commands
        while let Ok(Ok(event)) = receiver.recv_async().timeout_at(stop_token.clone()).await {
            match event {
                ConnectionManagerEvent::Accepted(conn) => {
                    let mut inner = self.arc.inner.lock();
                    match &mut *inner {
                        Some(inner) => {
                            // Register the connection
                            // We don't care if this fails, since nobody here asked for the inbound connection.
                            // If it does, we just drop the connection
                            let _ = self.on_new_protocol_network_connection(inner, conn);
                        }
                        None => {
                            // If this somehow happens, we're shutting down
                        }
                    };
                }
                ConnectionManagerEvent::Dead(mut conn) => {
                    conn.close();
                    conn.await;
                }
                ConnectionManagerEvent::Finished(desc) => {
                    let conn = {
                        let mut inner_lock = self.arc.inner.lock();
                        match &mut *inner_lock {
                            Some(inner) => {
                                // Remove the connection and wait for the connection loop to terminate
                                if let Ok(conn) = inner.connection_table.remove_connection(desc) {
                                    // Must close and wait to ensure things join
                                    Some(conn)
                                } else {
                                    None
                                }
                            }
                            None => None,
                        }
                    };

                    if let Some(mut conn) = conn {
                        conn.close();
                        conn.await;
                    }
                }
            }
        }
    }

    // Called by low-level network when any connection-oriented protocol connection appears
    // either from incoming connections.
    #[cfg_attr(target_os = "wasm32", allow(dead_code))]
    pub(super) async fn on_accepted_protocol_network_connection(
        &self,
        conn: ProtocolNetworkConnection,
    ) -> EyreResult<()> {
        // Get channel sender
        let sender = {
            let mut inner = self.arc.inner.lock();
            let inner = match &mut *inner {
                Some(v) => v,
                None => {
                    // If we are shutting down, just drop this and return
                    return Ok(());
                }
            };
            inner.sender.clone()
        };

        // Inform the processor of the event
        let _ = sender
            .send_async(ConnectionManagerEvent::Accepted(conn))
            .await;
        Ok(())
    }

    // Callback from network connection receive loop when it exits
    // cleans up the entry in the connection table
    pub(super) async fn report_connection_finished(&self, descriptor: ConnectionDescriptor) {
        // Get channel sender
        let sender = {
            let mut inner = self.arc.inner.lock();
            let inner = match &mut *inner {
                Some(v) => v,
                None => {
                    // If we are shutting down, just drop this and return
                    return;
                }
            };
            inner.sender.clone()
        };

        // Inform the processor of the event
        let _ = sender
            .send_async(ConnectionManagerEvent::Finished(descriptor))
            .await;
    }
}
