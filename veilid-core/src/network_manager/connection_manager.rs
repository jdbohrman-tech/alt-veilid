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
            match inner_lock.take() {
                Some(v) => v,
                None => {
                    panic!("not started");
                }
            }
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

    // Internal routine to see if we should keep this connection
    // from being LRU removed. Used on our initiated relay connections.
    fn should_protect_connection(&self, conn: &NetworkConnection) -> bool {
        let netman = self.network_manager();
        let routing_table = netman.routing_table();
        let remote_address = conn.connection_descriptor().remote_address().address();
        let Some(routing_domain) = routing_table.routing_domain_for_address(remote_address) else {
            return false;
        };
        let Some(rn) = routing_table.relay_node(routing_domain) else {
            return false;
        };
        let relay_nr = rn.filtered_clone(
            NodeRefFilter::new()
                .with_routing_domain(routing_domain)
                .with_address_type(conn.connection_descriptor().address_type())
                .with_protocol_type(conn.connection_descriptor().protocol_type()),
        );
        let dids = relay_nr.all_filtered_dial_info_details();
        for did in dids {
            if did.dial_info.address() == remote_address {
                return true;
            }
        }
        false
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

        let mut conn = NetworkConnection::from_protocol(self.clone(), stop_token, prot_conn, id);
        let handle = conn.get_handle();

        // See if this should be a protected connection
        let protect = self.should_protect_connection(&conn);
        if protect {
            log_net!(debug "== PROTECTING connection: {} -> {}", id, conn.debug_print(get_aligned_timestamp()));
            conn.protect();
        }

        // Add to the connection table
        match self.arc.connection_table.add_connection(conn) {
            Ok(None) => {
                // Connection added
            }
            Ok(Some(conn)) => {
                // Connection added and a different one LRU'd out
                // Send it to be terminated
                // log_net!(debug "== LRU kill connection due to limit: {:?}", conn);
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

    /// Called when we want to create a new connection or get the current one that already exists
    /// This will kill off any connections that are in conflict with the new connection to be made
    /// in order to make room for the new connection in the system's connection table
    /// This routine needs to be atomic, or connections may exist in the table that are not established
    #[instrument(level = "trace", skip(self), ret, err)]
    pub async fn get_or_create_connection(
        &self,
        dial_info: DialInfo,
    ) -> EyreResult<NetworkResult<ConnectionHandle>> {
        let peer_address = dial_info.peer_address();
        let remote_addr = peer_address.socket_addr();
        let mut preferred_local_address = self
            .network_manager()
            .net()
            .get_preferred_local_address(&dial_info);
        let best_port = preferred_local_address.map(|pla| pla.port());

        // Async lock on the remote address for atomicity per remote
        let _lock_guard = self.arc.address_lock_table.lock_tag(remote_addr).await;

        log_net!("== get_or_create_connection dial_info={:?}", dial_info);

        // If any connection to this remote exists that has the same protocol, return it
        // Any connection will do, we don't have to match the local address but if we can
        // match the preferred port do it
        if let Some(best_existing_conn) = self
            .arc
            .connection_table
            .get_best_connection_by_remote(best_port, peer_address)
        {
            log_net!(
                "== Returning best existing connection {:?}",
                best_existing_conn
            );

            return Ok(NetworkResult::Value(best_existing_conn));
        }

        // If there is a low-level connection collision here, then we release the 'preferred local address'
        // so we can make a second connection with an ephemeral port
        if self
            .arc
            .connection_table
            .check_for_colliding_connection(&dial_info)
        {
            preferred_local_address = None;
        }

        // Attempt new connection
        let mut retry_count = 0; // Someday, if we need this

        let prot_conn = network_result_try!(loop {
            let result_net_res = ProtocolNetworkConnection::connect(
                preferred_local_address,
                &dial_info,
                self.arc.connection_initial_timeout_ms,
                self.network_manager().address_filter(),
            )
            .await;
            match result_net_res {
                Ok(net_res) => {
                    if net_res.is_value() || retry_count == 0 {
                        // Successful new connection, return it
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

    pub async fn debug_print(&self) -> String {
        //let inner = self.arc.inner.lock();
        format!(
            "Connection Table:\n\n{}",
            self.arc.connection_table.debug_print_table()
        )
    }
}
