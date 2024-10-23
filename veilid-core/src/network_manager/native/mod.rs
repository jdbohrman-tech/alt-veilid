mod discovery_context;
mod igd_manager;
mod network_state;
mod network_tcp;
mod network_udp;
mod protocol;
mod start_protocols;
mod tasks;

pub(super) use protocol::*;

use super::*;
use crate::routing_table::*;
use connection_manager::*;
use discovery_context::*;
use network_state::*;
use network_tcp::*;
use protocol::tcp::RawTcpProtocolHandler;
use protocol::udp::RawUdpProtocolHandler;
use protocol::ws::WebsocketProtocolHandler;
use start_protocols::*;

use async_tls::TlsAcceptor;
use futures_util::StreamExt;
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys, rsa_private_keys};
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::path::{Path, PathBuf};

/////////////////////////////////////////////////////////////////

pub const UPDATE_NETWORK_CLASS_TASK_TICK_PERIOD_SECS: u32 = 1;
pub const NETWORK_INTERFACES_TASK_TICK_PERIOD_SECS: u32 = 1;
pub const UPNP_TASK_TICK_PERIOD_SECS: u32 = 1;

pub const PEEK_DETECT_LEN: usize = 64;

cfg_if! {
    if #[cfg(all(feature = "unstable-blockstore", feature="unstable-tunnels"))] {
        const PUBLIC_INTERNET_CAPABILITIES_LEN: usize = 9;
    } else if #[cfg(any(feature = "unstable-blockstore", feature="unstable-tunnels"))] {
        const PUBLIC_INTERNET_CAPABILITIES_LEN: usize = 8;
    } else  {
        const PUBLIC_INTERNET_CAPABILITIES_LEN: usize = 7;
    }
}
pub const PUBLIC_INTERNET_CAPABILITIES: [Capability; PUBLIC_INTERNET_CAPABILITIES_LEN] = [
    CAP_ROUTE,
    #[cfg(feature = "unstable-tunnels")]
    CAP_TUNNEL,
    CAP_SIGNAL,
    CAP_RELAY,
    CAP_VALIDATE_DIAL_INFO,
    CAP_DHT,
    CAP_DHT_WATCH,
    CAP_APPMESSAGE,
    #[cfg(feature = "unstable-blockstore")]
    CAP_BLOCKSTORE,
];

#[cfg(feature = "unstable-blockstore")]
const LOCAL_NETWORK_CAPABILITIES_LEN: usize = 5;
#[cfg(not(feature = "unstable-blockstore"))]
const LOCAL_NETWORK_CAPABILITIES_LEN: usize = 4;

pub const LOCAL_NETWORK_CAPABILITIES: [Capability; LOCAL_NETWORK_CAPABILITIES_LEN] = [
    CAP_RELAY,
    CAP_DHT,
    CAP_DHT_WATCH,
    CAP_APPMESSAGE,
    #[cfg(feature = "unstable-blockstore")]
    CAP_BLOCKSTORE,
];

pub const MAX_CAPABILITIES: usize = 64;

/////////////////////////////////////////////////////////////////

struct NetworkInner {
    /// set if the network needs to be restarted due to a low level configuration change
    /// such as dhcp release or change of address or interfaces being added or removed
    network_needs_restart: bool,

    /// join handles for all the low level network background tasks
    join_handles: Vec<MustJoinHandle<()>>,
    /// stop source for shutting down the low level network background tasks
    stop_source: Option<StopSource>,
    /// set if we need to calculate our public dial info again
    needs_public_dial_info_check: bool,
    /// set if we have yet to clear the network during public dial info checking
    network_already_cleared: bool,
    /// the punishment closure to enax
    public_dial_info_check_punishment: Option<Box<dyn FnOnce() + Send + 'static>>,
    /// Actual bound addresses per protocol
    bound_address_per_protocol: BTreeMap<ProtocolType, Vec<SocketAddr>>,
    /// mapping of protocol handlers to accept messages from a set of bound socket addresses
    udp_protocol_handlers: BTreeMap<SocketAddr, RawUdpProtocolHandler>,
    /// outbound udp protocol handler for udpv4
    default_udpv4_protocol_handler: Option<RawUdpProtocolHandler>,
    /// outbound udp protocol handler for udpv6
    default_udpv6_protocol_handler: Option<RawUdpProtocolHandler>,
    /// TLS handling socket controller
    tls_acceptor: Option<TlsAcceptor>,
    /// Multiplexer record for protocols on low level TCP sockets
    listener_states: BTreeMap<SocketAddr, Arc<RwLock<ListenerState>>>,
    /// Preferred local addresses for protocols/address combinations for outgoing connections
    preferred_local_addresses: BTreeMap<(ProtocolType, AddressType), SocketAddr>,
    /// set of statically configured protocols with public dialinfo
    static_public_dial_info: ProtocolTypeSet,
    /// Network state
    network_state: Option<NetworkState>,
}

struct NetworkUnlockedInner {
    // Startup lock
    startup_lock: StartupLock,

    // Accessors
    routing_table: RoutingTable,
    network_manager: NetworkManager,
    connection_manager: ConnectionManager,
    // Network
    interfaces: NetworkInterfaces,
    // Background processes
    update_network_class_task: TickTask<EyreReport>,
    network_interfaces_task: TickTask<EyreReport>,
    upnp_task: TickTask<EyreReport>,
    network_task_lock: AsyncMutex<()>,

    // Managers
    igd_manager: igd_manager::IGDManager,
}

#[derive(Clone)]
pub(super) struct Network {
    config: VeilidConfig,
    inner: Arc<Mutex<NetworkInner>>,
    unlocked_inner: Arc<NetworkUnlockedInner>,
}

impl Network {
    fn new_inner() -> NetworkInner {
        NetworkInner {
            network_needs_restart: false,
            needs_public_dial_info_check: false,
            network_already_cleared: false,
            public_dial_info_check_punishment: None,
            join_handles: Vec::new(),
            stop_source: None,
            bound_address_per_protocol: BTreeMap::new(),
            udp_protocol_handlers: BTreeMap::new(),
            default_udpv4_protocol_handler: None,
            default_udpv6_protocol_handler: None,
            tls_acceptor: None,
            listener_states: BTreeMap::new(),
            preferred_local_addresses: BTreeMap::new(),
            static_public_dial_info: ProtocolTypeSet::new(),
            network_state: None,
        }
    }

    fn new_unlocked_inner(
        network_manager: NetworkManager,
        routing_table: RoutingTable,
        connection_manager: ConnectionManager,
    ) -> NetworkUnlockedInner {
        let config = network_manager.config();
        NetworkUnlockedInner {
            startup_lock: StartupLock::new(),
            network_manager,
            routing_table,
            connection_manager,
            interfaces: NetworkInterfaces::new(),
            update_network_class_task: TickTask::new(
                "update_network_class_task",
                UPDATE_NETWORK_CLASS_TASK_TICK_PERIOD_SECS,
            ),
            network_interfaces_task: TickTask::new(
                "network_interfaces_task",
                NETWORK_INTERFACES_TASK_TICK_PERIOD_SECS,
            ),
            upnp_task: TickTask::new("upnp_task", UPNP_TASK_TICK_PERIOD_SECS),
            network_task_lock: AsyncMutex::new(()),
            igd_manager: igd_manager::IGDManager::new(config.clone()),
        }
    }

    pub fn new(
        network_manager: NetworkManager,
        routing_table: RoutingTable,
        connection_manager: ConnectionManager,
    ) -> Self {
        let this = Self {
            config: network_manager.config(),
            inner: Arc::new(Mutex::new(Self::new_inner())),
            unlocked_inner: Arc::new(Self::new_unlocked_inner(
                network_manager,
                routing_table,
                connection_manager,
            )),
        };

        this.setup_tasks();

        this
    }

    fn network_manager(&self) -> NetworkManager {
        self.unlocked_inner.network_manager.clone()
    }

    fn routing_table(&self) -> RoutingTable {
        self.unlocked_inner.routing_table.clone()
    }

    fn connection_manager(&self) -> ConnectionManager {
        self.unlocked_inner.connection_manager.clone()
    }

    fn load_certs(path: &Path) -> io::Result<Vec<Certificate>> {
        let cvec = certs(&mut BufReader::new(File::open(path)?))
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid TLS certificate"))?;
        Ok(cvec.into_iter().map(Certificate).collect())
    }

    fn load_keys(path: &Path) -> io::Result<Vec<PrivateKey>> {
        {
            if let Ok(v) = rsa_private_keys(&mut BufReader::new(File::open(path)?)) {
                if !v.is_empty() {
                    return Ok(v.into_iter().map(PrivateKey).collect());
                }
            }
        }
        {
            if let Ok(v) = pkcs8_private_keys(&mut BufReader::new(File::open(path)?)) {
                if !v.is_empty() {
                    return Ok(v.into_iter().map(PrivateKey).collect());
                }
            }
        }

        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "invalid TLS private key",
        ))
    }

    fn load_server_config(&self) -> io::Result<ServerConfig> {
        let c = self.config.get();
        //
        log_net!(
            "loading certificate from {}",
            c.network.tls.certificate_path
        );
        let certs = Self::load_certs(&PathBuf::from(&c.network.tls.certificate_path))?;
        log_net!("loaded {} certificates", certs.len());
        if certs.is_empty() {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Certificates at {} could not be loaded.\nEnsure it is in PEM format, beginning with '-----BEGIN CERTIFICATE-----'",c.network.tls.certificate_path)));
        }
        //
        log_net!(
            "loading private key from {}",
            c.network.tls.private_key_path
        );
        let mut keys = Self::load_keys(&PathBuf::from(&c.network.tls.private_key_path))?;
        log_net!("loaded {} keys", keys.len());
        if keys.is_empty() {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Private key at {} could not be loaded.\nEnsure it is unencrypted and in RSA or PKCS8 format, beginning with '-----BEGIN RSA PRIVATE KEY-----' or '-----BEGIN PRIVATE KEY-----'",c.network.tls.private_key_path)));
        }

        let config = ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(certs, keys.remove(0))
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?;

        Ok(config)
    }

    fn add_to_join_handles(&self, jh: MustJoinHandle<()>) {
        let mut inner = self.inner.lock();
        inner.join_handles.push(jh);
    }

    fn translate_unspecified_address(&self, from: SocketAddr) -> Vec<SocketAddr> {
        if !from.ip().is_unspecified() {
            vec![from]
        } else {
            let addrs = self.last_network_state().stable_interface_addresses;
            addrs
                .iter()
                .filter_map(|a| {
                    // We create sockets that are only ipv6 or ipv6 (not dual, so only translate matching unspecified address)
                    if (a.is_ipv4() && from.is_ipv4()) || (a.is_ipv6() && from.is_ipv6()) {
                        Some(SocketAddr::new(*a, from.port()))
                    } else {
                        None
                    }
                })
                .collect()
        }
    }

    pub fn get_preferred_local_address(&self, dial_info: &DialInfo) -> Option<SocketAddr> {
        let inner = self.inner.lock();
        let key = (dial_info.protocol_type(), dial_info.address_type());
        inner.preferred_local_addresses.get(&key).copied()
    }

    pub fn get_preferred_local_address_by_key(
        &self,
        pt: ProtocolType,
        at: AddressType,
    ) -> Option<SocketAddr> {
        let inner = self.inner.lock();
        let key = (pt, at);
        inner.preferred_local_addresses.get(&key).copied()
    }

    ////////////////////////////////////////////////////////////

    // Record DialInfo failures
    async fn record_dial_info_failure<T, F: Future<Output = EyreResult<NetworkResult<T>>>>(
        &self,
        dial_info: DialInfo,
        fut: F,
    ) -> EyreResult<NetworkResult<T>> {
        let network_result = fut.await?;
        if matches!(network_result, NetworkResult::NoConnection(_)) {
            self.network_manager()
                .address_filter()
                .set_dial_info_failed(dial_info);
        }
        Ok(network_result)
    }

    // Send data to a dial info, unbound, using a new connection from a random port
    // This creates a short-lived connection in the case of connection-oriented protocols
    // for the purpose of sending this one message.
    // This bypasses the connection table as it is not a 'node to node' connection.
    #[instrument(level="trace", target="net", err, skip(self, data), fields(data.len = data.len()))]
    pub async fn send_data_unbound_to_dial_info(
        &self,
        dial_info: DialInfo,
        data: Vec<u8>,
    ) -> EyreResult<NetworkResult<()>> {
        let _guard = self.unlocked_inner.startup_lock.enter()?;

        self.record_dial_info_failure(
            dial_info.clone(),
            async move {
                let data_len = data.len();
                let connect_timeout_ms = {
                    let c = self.config.get();
                    c.network.connection_initial_timeout_ms
                };

                if self
                    .network_manager()
                    .address_filter()
                    .is_ip_addr_punished(dial_info.address().ip_addr())
                {
                    return Ok(NetworkResult::no_connection_other("punished"));
                }

                match dial_info.protocol_type() {
                    ProtocolType::UDP => {
                        let peer_socket_addr = dial_info.to_socket_addr();
                        let h =
                            RawUdpProtocolHandler::new_unspecified_bound_handler(&peer_socket_addr)
                                .await
                                .wrap_err("create socket failure")?;
                        let _ = network_result_try!(h
                            .send_message(data, peer_socket_addr)
                            .await
                            .map(NetworkResult::Value)
                            .wrap_err("send message failure")?);
                    }
                    ProtocolType::TCP => {
                        let peer_socket_addr = dial_info.to_socket_addr();
                        let pnc = network_result_try!(RawTcpProtocolHandler::connect(
                            None,
                            peer_socket_addr,
                            connect_timeout_ms
                        )
                        .await
                        .wrap_err("connect failure")?);
                        network_result_try!(pnc.send(data).await.wrap_err("send failure")?);
                    }
                    ProtocolType::WS | ProtocolType::WSS => {
                        let pnc = network_result_try!(WebsocketProtocolHandler::connect(
                            None,
                            &dial_info,
                            connect_timeout_ms
                        )
                        .await
                        .wrap_err("connect failure")?);
                        network_result_try!(pnc.send(data).await.wrap_err("send failure")?);
                    }
                }
                // Network accounting
                self.network_manager()
                    .stats_packet_sent(dial_info.ip_addr(), ByteCount::new(data_len as u64));

                Ok(NetworkResult::Value(()))
            }
            .in_current_span(),
        )
        .await
    }

    // Send data to a dial info, unbound, using a new connection from a random port
    // Waits for a specified amount of time to receive a single response
    // This creates a short-lived connection in the case of connection-oriented protocols
    // for the purpose of sending this one message.
    // This bypasses the connection table as it is not a 'node to node' connection.
    #[instrument(level="trace", target="net", err, skip(self, data), fields(data.len = data.len()))]
    pub async fn send_recv_data_unbound_to_dial_info(
        &self,
        dial_info: DialInfo,
        data: Vec<u8>,
        timeout_ms: u32,
    ) -> EyreResult<NetworkResult<Vec<u8>>> {
        let _guard = self.unlocked_inner.startup_lock.enter()?;

        self.record_dial_info_failure(
            dial_info.clone(),
            async move {
                let data_len = data.len();
                let connect_timeout_ms = {
                    let c = self.config.get();
                    c.network.connection_initial_timeout_ms
                };

                if self
                    .network_manager()
                    .address_filter()
                    .is_ip_addr_punished(dial_info.address().ip_addr())
                {
                    return Ok(NetworkResult::no_connection_other("punished"));
                }

                match dial_info.protocol_type() {
                    ProtocolType::UDP => {
                        let peer_socket_addr = dial_info.to_socket_addr();
                        let h =
                            RawUdpProtocolHandler::new_unspecified_bound_handler(&peer_socket_addr)
                                .await
                                .wrap_err("create socket failure")?;
                        network_result_try!(h
                            .send_message(data, peer_socket_addr)
                            .await
                            .wrap_err("send message failure")?);
                        self.network_manager().stats_packet_sent(
                            dial_info.ip_addr(),
                            ByteCount::new(data_len as u64),
                        );

                        // receive single response
                        let mut out = vec![0u8; MAX_MESSAGE_SIZE];
                        let (recv_len, recv_addr) = network_result_try!(timeout(
                            timeout_ms,
                            h.recv_message(&mut out).in_current_span()
                        )
                        .await
                        .into_network_result())
                        .wrap_err("recv_message failure")?;

                        let recv_socket_addr = recv_addr.remote_address().socket_addr();
                        self.network_manager().stats_packet_rcvd(
                            recv_socket_addr.ip(),
                            ByteCount::new(recv_len as u64),
                        );

                        // if the from address is not the same as the one we sent to, then drop this
                        if recv_socket_addr != peer_socket_addr {
                            bail!("wrong address");
                        }
                        out.resize(recv_len, 0u8);
                        Ok(NetworkResult::Value(out))
                    }
                    ProtocolType::TCP | ProtocolType::WS | ProtocolType::WSS => {
                        let pnc = network_result_try!(match dial_info.protocol_type() {
                            ProtocolType::UDP => unreachable!(),
                            ProtocolType::TCP => {
                                let peer_socket_addr = dial_info.to_socket_addr();
                                RawTcpProtocolHandler::connect(
                                    None,
                                    peer_socket_addr,
                                    connect_timeout_ms,
                                )
                                .await
                                .wrap_err("connect failure")?
                            }
                            ProtocolType::WS | ProtocolType::WSS => {
                                WebsocketProtocolHandler::connect(
                                    None,
                                    &dial_info,
                                    connect_timeout_ms,
                                )
                                .await
                                .wrap_err("connect failure")?
                            }
                        });

                        network_result_try!(pnc.send(data).await.wrap_err("send failure")?);
                        self.network_manager().stats_packet_sent(
                            dial_info.ip_addr(),
                            ByteCount::new(data_len as u64),
                        );

                        let out = network_result_try!(network_result_try!(timeout(
                            timeout_ms,
                            pnc.recv().in_current_span()
                        )
                        .await
                        .into_network_result())
                        .wrap_err("recv failure")?);

                        self.network_manager().stats_packet_rcvd(
                            dial_info.ip_addr(),
                            ByteCount::new(out.len() as u64),
                        );

                        Ok(NetworkResult::Value(out))
                    }
                }
            }
            .in_current_span(),
        )
        .await
    }

    #[instrument(level="trace", target="net", err, skip(self, data), fields(data.len = data.len()))]
    pub async fn send_data_to_existing_flow(
        &self,
        flow: Flow,
        data: Vec<u8>,
    ) -> EyreResult<SendDataToExistingFlowResult> {
        let _guard = self.unlocked_inner.startup_lock.enter()?;

        let data_len = data.len();

        // Handle connectionless protocol
        if flow.protocol_type() == ProtocolType::UDP {
            // send over the best udp socket we have bound since UDP is not connection oriented
            let peer_socket_addr = flow.remote().socket_addr();
            if let Some(ph) = self.find_best_udp_protocol_handler(
                &peer_socket_addr,
                &flow.local().map(|sa| sa.socket_addr()),
            ) {
                network_result_value_or_log!(ph.clone()
                    .send_message(data.clone(), peer_socket_addr)
                    .await
                    .wrap_err("sending data to existing connection")? => [ format!(": data.len={}, flow={:?}", data.len(), flow) ]
                    { return Ok(SendDataToExistingFlowResult::NotSent(data)); } );

                // Network accounting
                self.network_manager()
                    .stats_packet_sent(peer_socket_addr.ip(), ByteCount::new(data_len as u64));

                // Data was consumed
                let unique_flow = UniqueFlow {
                    flow,
                    connection_id: None,
                };
                return Ok(SendDataToExistingFlowResult::Sent(unique_flow));
            }
        }

        // Handle connection-oriented protocols

        // Try to send to the exact existing connection if one exists
        if let Some(conn) = self.connection_manager().get_connection(flow) {
            // connection exists, send over it
            match conn.send_async(data).await {
                ConnectionHandleSendResult::Sent => {
                    // Network accounting
                    self.network_manager().stats_packet_sent(
                        flow.remote().socket_addr().ip(),
                        ByteCount::new(data_len as u64),
                    );

                    // Data was consumed
                    return Ok(SendDataToExistingFlowResult::Sent(conn.unique_flow()));
                }
                ConnectionHandleSendResult::NotSent(data) => {
                    // Couldn't send
                    // Pass the data back out so we don't own it any more
                    return Ok(SendDataToExistingFlowResult::NotSent(data));
                }
            }
        }
        // Connection didn't exist
        // Pass the data back out so we don't own it any more
        Ok(SendDataToExistingFlowResult::NotSent(data))
    }

    // Send data directly to a dial info, possibly without knowing which node it is going to
    // Returns a flow for the connection used to send the data
    #[instrument(level="trace", target="net", err, skip(self, data), fields(data.len = data.len()))]
    pub async fn send_data_to_dial_info(
        &self,
        dial_info: DialInfo,
        data: Vec<u8>,
    ) -> EyreResult<NetworkResult<UniqueFlow>> {
        let _guard = self.unlocked_inner.startup_lock.enter()?;

        self.record_dial_info_failure(
            dial_info.clone(),
            async move {
                let data_len = data.len();
                let unique_flow;
                if dial_info.protocol_type() == ProtocolType::UDP {
                    // Handle connectionless protocol
                    let peer_socket_addr = dial_info.to_socket_addr();
                    let ph = match self.find_best_udp_protocol_handler(&peer_socket_addr, &None) {
                        Some(ph) => ph,
                        None => {
                            return Ok(NetworkResult::no_connection_other(
                                "no appropriate UDP protocol handler for dial_info",
                            ));
                        }
                    };
                    let flow = network_result_try!(ph
                        .send_message(data, peer_socket_addr)
                        .await
                        .wrap_err("failed to send data to dial info")?);
                    unique_flow = UniqueFlow {
                        flow,
                        connection_id: None,
                    };
                } else {
                    // Handle connection-oriented protocols
                    let conn = network_result_try!(
                        self.connection_manager()
                            .get_or_create_connection(dial_info.clone())
                            .await?
                    );

                    if let ConnectionHandleSendResult::NotSent(_) = conn.send_async(data).await {
                        return Ok(NetworkResult::NoConnection(io::Error::new(
                            io::ErrorKind::ConnectionReset,
                            "failed to send",
                        )));
                    }
                    unique_flow = conn.unique_flow();
                }

                // Network accounting
                self.network_manager()
                    .stats_packet_sent(dial_info.ip_addr(), ByteCount::new(data_len as u64));

                Ok(NetworkResult::value(unique_flow))
            }
            .in_current_span(),
        )
        .await
    }

    /////////////////////////////////////////////////////////////////

    pub async fn startup_internal(&self) -> EyreResult<StartupDisposition> {
        // Get the initial network state snapshot
        // Caution: this -must- happen first because we use unwrap() in last_network_state()
        let network_state = self.make_network_state().await?;

        {
            let mut inner = self.inner.lock();

            // Create the shutdown stopper
            inner.stop_source = Some(StopSource::new());

            // Store the first network state snapshot
            inner.network_state = Some(network_state.clone());
        }

        // Start editing routing table
        let mut editor_public_internet = self
            .unlocked_inner
            .routing_table
            .edit_public_internet_routing_domain();
        let mut editor_local_network = self
            .unlocked_inner
            .routing_table
            .edit_local_network_routing_domain();

        // Setup network
        editor_local_network.set_local_networks(network_state.local_networks);
        editor_local_network.setup_network(
            network_state.protocol_config.outbound,
            network_state.protocol_config.inbound,
            network_state.protocol_config.family_local,
            network_state.protocol_config.local_network_capabilities,
        );

        editor_public_internet.setup_network(
            network_state.protocol_config.outbound,
            network_state.protocol_config.inbound,
            network_state.protocol_config.family_global,
            network_state.protocol_config.public_internet_capabilities,
        );

        // Start listeners
        if network_state
            .protocol_config
            .inbound
            .contains(ProtocolType::UDP)
        {
            let res = self.bind_udp_protocol_handlers().await;
            if !matches!(res, Ok(StartupDisposition::Success)) {
                return res;
            }
        }
        if network_state
            .protocol_config
            .inbound
            .contains(ProtocolType::WS)
        {
            let res = self.start_ws_listeners().await;
            if !matches!(res, Ok(StartupDisposition::Success)) {
                return res;
            }
        }
        if network_state
            .protocol_config
            .inbound
            .contains(ProtocolType::WSS)
        {
            let res = self.start_wss_listeners().await;
            if !matches!(res, Ok(StartupDisposition::Success)) {
                return res;
            }
        }
        if network_state
            .protocol_config
            .inbound
            .contains(ProtocolType::TCP)
        {
            let res = self.start_tcp_listeners().await;
            if !matches!(res, Ok(StartupDisposition::Success)) {
                return res;
            }
        }

        // Register all dialinfo
        self.register_all_dial_info(&mut editor_public_internet, &mut editor_local_network)
            .await?;

        // Commit routing domain edits
        if editor_public_internet.commit(true).await {
            editor_public_internet.publish();
        }
        if editor_local_network.commit(true).await {
            editor_local_network.publish();
        }

        Ok(StartupDisposition::Success)
    }

    #[instrument(level = "debug", err, skip_all)]
    pub(super) async fn register_all_dial_info(
        &self,
        editor_public_internet: &mut RoutingDomainEditorPublicInternet,
        editor_local_network: &mut RoutingDomainEditorLocalNetwork,
    ) -> EyreResult<()> {
        let Some(protocol_config) = ({
            let inner = self.inner.lock();
            inner
                .network_state
                .as_ref()
                .map(|ns| ns.protocol_config.clone())
        }) else {
            bail!("can't register dial info without network state");
        };

        if protocol_config.inbound.contains(ProtocolType::UDP) {
            self.register_udp_dial_info(editor_public_internet, editor_local_network)
                .await?;
        }
        if protocol_config.inbound.contains(ProtocolType::WS) {
            self.register_ws_dial_info(editor_public_internet, editor_local_network)
                .await?;
        }
        if protocol_config.inbound.contains(ProtocolType::WSS) {
            self.register_wss_dial_info(editor_public_internet, editor_local_network)
                .await?;
        }
        if protocol_config.inbound.contains(ProtocolType::TCP) {
            self.register_tcp_dial_info(editor_public_internet, editor_local_network)
                .await?;
        }

        Ok(())
    }

    #[instrument(level = "debug", err, skip_all)]
    pub async fn startup(&self) -> EyreResult<StartupDisposition> {
        let guard = self.unlocked_inner.startup_lock.startup()?;

        match self.startup_internal().await {
            Ok(StartupDisposition::Success) => {
                info!("network started");
                guard.success();
                Ok(StartupDisposition::Success)
            }
            Ok(StartupDisposition::BindRetry) => {
                debug!("network bind retry");
                self.shutdown_internal().await;
                Ok(StartupDisposition::BindRetry)
            }
            Err(e) => {
                debug!("network failed to start");
                self.shutdown_internal().await;
                Err(e)
            }
        }
    }

    pub fn needs_restart(&self) -> bool {
        self.inner.lock().network_needs_restart
    }

    pub fn is_started(&self) -> bool {
        self.unlocked_inner.startup_lock.is_started()
    }

    #[instrument(level = "debug", skip_all)]
    pub fn restart_network(&self) {
        self.inner.lock().network_needs_restart = true;
    }

    #[instrument(level = "debug", skip_all)]
    async fn shutdown_internal(&self) {
        let routing_table = self.routing_table();

        // Stop all tasks
        log_net!(debug "stopping update network class task");
        if let Err(e) = self.unlocked_inner.update_network_class_task.stop().await {
            error!("update_network_class_task not cancelled: {}", e);
        }

        let mut unord = FuturesUnordered::new();
        {
            let mut inner = self.inner.lock();
            // take the join handles out
            for h in inner.join_handles.drain(..) {
                log_net!("joining: {:?}", h);
                unord.push(h);
            }
            // Drop the stop
            drop(inner.stop_source.take());
        }
        log_net!(debug "stopping {} low level network tasks", unord.len());
        // Wait for everything to stop
        while unord.next().await.is_some() {}

        log_net!(debug "clearing dial info");

        routing_table
            .edit_public_internet_routing_domain()
            .shutdown()
            .await;

        routing_table
            .edit_local_network_routing_domain()
            .shutdown()
            .await;

        // Reset state including network class
        *self.inner.lock() = Self::new_inner();
    }

    #[instrument(level = "debug", skip_all)]
    pub async fn shutdown(&self) {
        log_net!(debug "starting low level network shutdown");
        let Ok(guard) = self.unlocked_inner.startup_lock.shutdown().await else {
            log_net!(debug "low level network is already shut down");
            return;
        };

        self.shutdown_internal().await;

        guard.success();
        log_net!(debug "finished low level network shutdown");
    }

    //////////////////////////////////////////
    pub fn set_needs_public_dial_info_check(
        &self,
        punishment: Option<Box<dyn FnOnce() + Send + 'static>>,
    ) {
        let Ok(_guard) = self.unlocked_inner.startup_lock.enter() else {
            log_net!(debug "ignoring due to not started up");
            return;
        };
        let mut inner = self.inner.lock();
        inner.needs_public_dial_info_check = true;
        inner.public_dial_info_check_punishment = punishment;
    }

    pub fn needs_public_dial_info_check(&self) -> bool {
        let Ok(_guard) = self.unlocked_inner.startup_lock.enter() else {
            log_net!(debug "ignoring due to not started up");
            return false;
        };
        let inner = self.inner.lock();
        inner.needs_public_dial_info_check
    }

    //////////////////////////////////////////
}
