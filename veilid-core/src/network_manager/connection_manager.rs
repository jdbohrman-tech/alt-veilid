use super::*;
use connection_table::*;
use network_connection::*;
use stop_token::future::FutureExt;

///////////////////////////////////////////////////////////
// Connection manager

#[derive(Debug)]
enum ConnectionManagerEvent {
    Accepted(ProtocolNetworkConnection),
    Dead(NetworkConnection),
}

#[derive(Debug)]
struct ConnectionManagerInner {
    next_id: NetworkConnectionId,
    sender: flume::Sender<ConnectionManagerEvent>,
    async_processor_jh: Option<MustJoinHandle<()>>,
    stop_source: Option<StopSource>,
}

struct ConnectionManagerArc {
    network_manager: NetworkManager,
    connection_initial_timeout_ms: u32,
    connection_inactivity_timeout_ms: u32,
    connection_table: ConnectionTable,
    address_lock_table: AsyncTagLockTable<SocketAddr>,
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
        stop_source: StopSource,
        sender: flume::Sender<ConnectionManagerEvent>,
        async_processor_jh: MustJoinHandle<()>,
    ) -> ConnectionManagerInner {
        ConnectionManagerInner {
            next_id: 0.into(),
            stop_source: Some(stop_source),
            sender,
            async_processor_jh: Some(async_processor_jh),
        }
    }
    fn new_arc(network_manager: NetworkManager) -> ConnectionManagerArc {
        let config = network_manager.config();
        let (connection_initial_timeout_ms, connection_inactivity_timeout_ms) = {
            let c = config.get();
            (
                c.network.connection_initial_timeout_ms,
                c.network.connection_inactivity_timeout_ms,
            )
        };
        let address_filter = network_manager.address_filter();

        ConnectionManagerArc {
            network_manager,
            connection_initial_timeout_ms,
            connection_inactivity_timeout_ms,
            connection_table: ConnectionTable::new(config, address_filter),
            address_lock_table: AsyncTagLockTable::new(),
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

    pub fn connection_initial_timeout_ms(&self) -> u32 {
        self.arc.connection_initial_timeout_ms
    }

    pub fn connection_inactivity_timeout_ms(&self) -> u32 {
        self.arc.connection_inactivity_timeout_ms
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
        *inner = Some(Self::new_inner(stop_source, sender, async_processor));
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
        self.arc.connection_table.join().await;
        debug!("finished connection manager shutdown");
    }

    // Internal routine to register new connection atomically.
    // Registers connection in the connection table for later access
    // and spawns a message processing loop for the connection
    #[instrument(level = "trace", skip(self, inner), ret, err)]
    fn on_new_protocol_network_connection(
        &self,
        inner: &mut ConnectionManagerInner,
        prot_conn: ProtocolNetworkConnection,
    ) -> EyreResult<NetworkResult<ConnectionHandle>> {
        // Get next connection id to use
        let id = inner.next_id;
        inner.next_id += 1u64;
        log_net!(
            "on_new_protocol_network_connection: id={} prot_conn={:?}",
            id,
            prot_conn
        );

        // Wrap with NetworkConnection object to start the connection processing loop
        let stop_token = match &inner.stop_source {
            Some(ss) => ss.token(),
            None => bail!("not creating connection because we are stopping"),
        };

        let conn = NetworkConnection::from_protocol(self.clone(), stop_token, prot_conn, id);
        let handle = conn.get_handle();
        // Add to the connection table
        match self.arc.connection_table.add_connection(conn) {
            Ok(None) => {
                // Connection added
            }
            Ok(Some(conn)) => {
                // Connection added and a different one LRU'd out
                // Send it to be terminated
                let _ = inner.sender.send(ConnectionManagerEvent::Dead(conn));
            }
            Err(ConnectionTableAddError::AddressFilter(conn, e)) => {
                // Connection filtered
                let desc = conn.connection_descriptor();
                let _ = inner.sender.send(ConnectionManagerEvent::Dead(conn));
                return Ok(NetworkResult::no_connection_other(format!(
                    "connection filtered: {:?} ({})",
                    desc, e
                )));
            }
            Err(ConnectionTableAddError::AlreadyExists(conn)) => {
                // Connection already exists
                let desc = conn.connection_descriptor();
                let _ = inner.sender.send(ConnectionManagerEvent::Dead(conn));
                return Ok(NetworkResult::no_connection_other(format!(
                    "connection already exists: {:?}",
                    desc
                )));
            }
        };
        Ok(NetworkResult::Value(handle))
    }

    // Returns a network connection if one already is established
    //#[instrument(level = "trace", skip(self), ret)]
    pub fn get_connection(&self, descriptor: ConnectionDescriptor) -> Option<ConnectionHandle> {
        self.arc
            .connection_table
            .get_connection_by_descriptor(descriptor)
    }

    // Terminate any connections that would collide with a new connection
    // using different protocols to the same remote address and port. Used to ensure
    // that we can switch quickly between TCP and WS if necessary to the same node
    // Returns true if we killed off colliding connections
    async fn kill_off_colliding_connections(&self, dial_info: &DialInfo) -> bool {
        let protocol_type = dial_info.protocol_type();
        let socket_address = dial_info.socket_address();

        let killed = self.arc.connection_table.drain_filter(|prior_descriptor| {
            // If the protocol types aren't the same, then this is a candidate to be killed off
            // If they are the same, then we would just return the exact same connection from get_or_create_connection()
            if prior_descriptor.protocol_type() == protocol_type {
                return false;
            }
            // If the prior remote is not the same address, then we're not going to collide
            if *prior_descriptor.remote().socket_address() != socket_address {
                return false;
            }

            log_net!(debug
                ">< Terminating connection prior_descriptor={:?}",
                prior_descriptor
            );
            true
        });
        // Wait for the killed connections to end their recv loops
        let did_kill = !killed.is_empty();
        for mut k in killed {
            k.close();
            k.await;
        }
        did_kill
    }

    /// Called when we want to create a new connection or get the current one that already exists
    /// This will kill off any connections that are in conflict with the new connection to be made
    /// in order to make room for the new connection in the system's connection table
    /// This routine needs to be atomic, or connections may exist in the table that are not established
    #[instrument(level = "trace", skip(self), ret, err)]
    pub async fn get_or_create_connection(
        &self,
        local_addr: Option<SocketAddr>,
        dial_info: DialInfo,
    ) -> EyreResult<NetworkResult<ConnectionHandle>> {
        // Async lock on the remote address for atomicity per remote
        let peer_address = dial_info.to_peer_address();
        let remote_addr = peer_address.to_socket_addr();

        let _lock_guard = self.arc.address_lock_table.lock_tag(remote_addr).await;

        log_net!(
            "== get_or_create_connection local_addr={:?} dial_info={:?}",
            local_addr,
            dial_info
        );

        // Kill off any possibly conflicting connections
        let did_kill = self.kill_off_colliding_connections(&dial_info).await;
        let mut retry_count = if did_kill { 2 } else { 0 };

        // If any connection to this remote exists that has the same protocol, return it
        // Any connection will do, we don't have to match the local address
        if let Some(conn) = self
            .arc
            .connection_table
            .get_last_connection_by_remote(peer_address)
        {
            log_net!(
                "== Returning existing connection local_addr={:?} peer_address={:?}",
                local_addr,
                peer_address
            );

            return Ok(NetworkResult::Value(conn));
        }

        // Attempt new connection
        let prot_conn = network_result_try!(loop {
            let result_net_res = ProtocolNetworkConnection::connect(
                local_addr,
                &dial_info,
                self.arc.connection_initial_timeout_ms,
                self.network_manager().address_filter(),
            )
            .await;
            match result_net_res {
                Ok(net_res) => {
                    // If the connection 'already exists', then try one last time to return a connection from the table, in case
                    // an 'accept' happened at literally the same time as our connect
                    if net_res.is_already_exists() {
                        if let Some(conn) = self
                            .arc
                            .connection_table
                            .get_last_connection_by_remote(peer_address)
                        {
                            log_net!(
                                    "== Returning existing connection in race local_addr={:?} peer_address={:?}",
                                    local_addr,
                                    peer_address
                                );

                            return Ok(NetworkResult::Value(conn));
                        }
                    }
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
            sleep(500).await;
        });

        // Add to the connection table
        let mut inner = self.arc.inner.lock();
        let inner = match &mut *inner {
            Some(v) => v,
            None => {
                bail!("shutting down");
            }
        };

        self.on_new_protocol_network_connection(inner, prot_conn)
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
            }
        }
    }

    // Called by low-level network when any connection-oriented protocol connection appears
    // either from incoming connections.
    #[cfg_attr(target_arch = "wasm32", allow(dead_code))]
    pub(super) async fn on_accepted_protocol_network_connection(
        &self,
        protocol_connection: ProtocolNetworkConnection,
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
            .send_async(ConnectionManagerEvent::Accepted(protocol_connection))
            .await;
        Ok(())
    }

    // Callback from network connection receive loop when it exits
    // cleans up the entry in the connection table
    #[instrument(level = "trace", skip(self))]
    pub(super) async fn report_connection_finished(&self, connection_id: NetworkConnectionId) {
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

        // Remove the connection
        let conn = self
            .arc
            .connection_table
            .remove_connection_by_id(connection_id);

        // Inform the processor of the event
        if let Some(conn) = conn {
            let _ = sender.send_async(ConnectionManagerEvent::Dead(conn)).await;
        }
    }
}
