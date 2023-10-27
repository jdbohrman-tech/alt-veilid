mod discovery_context;
mod igd_manager;
mod network_class_discovery;
mod network_tcp;
mod network_udp;
mod protocol;
mod start_protocols;

use super::*;
use crate::routing_table::*;
use connection_manager::*;
use discovery_context::*;
use network_tcp::*;
use protocol::tcp::RawTcpProtocolHandler;
use protocol::udp::RawUdpProtocolHandler;
use protocol::ws::WebsocketProtocolHandler;
pub(in crate::network_manager) use protocol::*;

use async_tls::TlsAcceptor;
use futures_util::StreamExt;
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys, rsa_private_keys};
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::path::{Path, PathBuf};

/////////////////////////////////////////////////////////////////

pub const PEEK_DETECT_LEN: usize = 64;

cfg_if! {
    if #[cfg(all(feature = "unstable-blockstore", feature="unstable-tunnels"))] {
        const PUBLIC_INTERNET_CAPABILITIES_LEN: usize = 8;
    } else if #[cfg(any(feature = "unstable-blockstore", feature="unstable-tunnels"))] {
        const PUBLIC_INTERNET_CAPABILITIES_LEN: usize = 7;
    } else  {
        const PUBLIC_INTERNET_CAPABILITIES_LEN: usize = 6;
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
    CAP_APPMESSAGE,
    #[cfg(feature = "unstable-blockstore")]
    CAP_BLOCKSTORE,
];

#[cfg(feature = "unstable-blockstore")]
const LOCAL_NETWORK_CAPABILITIES_LEN: usize = 4;
#[cfg(not(feature = "unstable-blockstore"))]
const LOCAL_NETWORK_CAPABILITIES_LEN: usize = 3;

pub const LOCAL_NETWORK_CAPABILITIES: [Capability; LOCAL_NETWORK_CAPABILITIES_LEN] = [
    CAP_RELAY,
    CAP_DHT,
    CAP_APPMESSAGE,
    #[cfg(feature = "unstable-blockstore")]
    CAP_BLOCKSTORE,
];

pub const MAX_CAPABILITIES: usize = 64;

/////////////////////////////////////////////////////////////////

struct NetworkInner {
    /// true if the low-level network is running
    network_started: bool,
    /// set if the network needs to be restarted due to a low level configuration change
    /// such as dhcp release or change of address or interfaces being added or removed
    network_needs_restart: bool,
    /// the calculated protocol configuration for inbound/outbound protocols
    protocol_config: ProtocolConfig,
    /// set of statically configured protocols with public dialinfo
    static_public_dialinfo: ProtocolTypeSet,
    /// join handles for all the low level network background tasks
    join_handles: Vec<MustJoinHandle<()>>,
    /// stop source for shutting down the low level network background tasks
    stop_source: Option<StopSource>,
    /// port we are binding raw udp listen to
    udp_port: u16,
    /// port we are binding raw tcp listen to
    tcp_port: u16,
    /// port we are binding websocket listen to
    ws_port: u16,
    /// port we are binding secure websocket listen to
    wss_port: u16,
    /// does our network have ipv4 on any network?
    enable_ipv4: bool,
    /// does our network have ipv6 on the global internet?
    enable_ipv6_global: bool,
    /// does our network have ipv6 on the local network?
    enable_ipv6_local: bool,
    /// set if we need to calculate our public dial info again
    needs_public_dial_info_check: bool,
    /// set if we have yet to clear the network during public dial info checking
    network_already_cleared: bool,
    /// the punishment closure to enax
    public_dial_info_check_punishment: Option<Box<dyn FnOnce() + Send + 'static>>,
    /// udp socket record for bound-first sockets, which are used to guarantee a port is available before
    /// creating a 'reuseport' socket there. we don't want to pick ports that other programs are using
    bound_first_udp: BTreeMap<u16, (Option<socket2::Socket>, Option<socket2::Socket>)>,
    /// mapping of protocol handlers to accept messages from a set of bound socket addresses
    inbound_udp_protocol_handlers: BTreeMap<SocketAddr, RawUdpProtocolHandler>,
    /// outbound udp protocol handler for udpv4
    outbound_udpv4_protocol_handler: Option<RawUdpProtocolHandler>,
    /// outbound udp protocol handler for udpv6
    outbound_udpv6_protocol_handler: Option<RawUdpProtocolHandler>,
    /// tcp socket record for bound-first sockets, which are used to guarantee a port is available before
    /// creating a 'reuseport' socket there. we don't want to pick ports that other programs are using
    bound_first_tcp: BTreeMap<u16, (Option<socket2::Socket>, Option<socket2::Socket>)>,
    /// TLS handling socket controller
    tls_acceptor: Option<TlsAcceptor>,
    /// Multiplexer record for protocols on low level TCP sockets
    listener_states: BTreeMap<SocketAddr, Arc<RwLock<ListenerState>>>,
}

struct NetworkUnlockedInner {
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

    // Managers
    igd_manager: igd_manager::IGDManager,
}

#[derive(Clone)]
pub(in crate::network_manager) struct Network {
    config: VeilidConfig,
    inner: Arc<Mutex<NetworkInner>>,
    unlocked_inner: Arc<NetworkUnlockedInner>,
}

impl Network {
    fn new_inner() -> NetworkInner {
        NetworkInner {
            network_started: false,
            network_needs_restart: false,
            needs_public_dial_info_check: false,
            network_already_cleared: false,
            public_dial_info_check_punishment: None,
            protocol_config: Default::default(),
            static_public_dialinfo: ProtocolTypeSet::empty(),
            join_handles: Vec::new(),
            stop_source: None,
            udp_port: 0u16,
            tcp_port: 0u16,
            ws_port: 0u16,
            wss_port: 0u16,
            enable_ipv4: false,
            enable_ipv6_global: false,
            enable_ipv6_local: false,
            bound_first_udp: BTreeMap::new(),
            inbound_udp_protocol_handlers: BTreeMap::new(),
            outbound_udpv4_protocol_handler: None,
            outbound_udpv6_protocol_handler: None,
            bound_first_tcp: BTreeMap::new(),
            tls_acceptor: None,
            listener_states: BTreeMap::new(),
        }
    }

    fn new_unlocked_inner(
        network_manager: NetworkManager,
        routing_table: RoutingTable,
        connection_manager: ConnectionManager,
    ) -> NetworkUnlockedInner {
        let config = network_manager.config();
        NetworkUnlockedInner {
            network_manager,
            routing_table,
            connection_manager,
            interfaces: NetworkInterfaces::new(),
            update_network_class_task: TickTask::new(1),
            network_interfaces_task: TickTask::new(5),
            upnp_task: TickTask::new(1),
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

        // Set update network class tick task
        {
            let this2 = this.clone();
            this.unlocked_inner
                .update_network_class_task
                .set_routine(move |s, l, t| {
                    Box::pin(this2.clone().update_network_class_task_routine(s, l, t))
                });
        }
        // Set network interfaces tick task
        {
            let this2 = this.clone();
            this.unlocked_inner
                .network_interfaces_task
                .set_routine(move |s, l, t| {
                    Box::pin(this2.clone().network_interfaces_task_routine(s, l, t))
                });
        }
        // Set upnp tick task
        {
            let this2 = this.clone();
            this.unlocked_inner
                .upnp_task
                .set_routine(move |s, l, t| Box::pin(this2.clone().upnp_task_routine(s, l, t)));
        }

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
        trace!(
            "loading certificate from {}",
            c.network.tls.certificate_path
        );
        let certs = Self::load_certs(&PathBuf::from(&c.network.tls.certificate_path))?;
        trace!("loaded {} certificates", certs.len());
        if certs.is_empty() {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Certificates at {} could not be loaded.\nEnsure it is in PEM format, beginning with '-----BEGIN CERTIFICATE-----'",c.network.tls.certificate_path)));
        }
        //
        trace!(
            "loading private key from {}",
            c.network.tls.private_key_path
        );
        let mut keys = Self::load_keys(&PathBuf::from(&c.network.tls.private_key_path))?;
        trace!("loaded {} keys", keys.len());
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

    fn translate_unspecified_address(&self, from: &SocketAddr) -> Vec<SocketAddr> {
        if !from.ip().is_unspecified() {
            vec![*from]
        } else {
            let addrs = self.get_stable_interface_addresses();
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

    pub fn get_local_port(&self, protocol_type: ProtocolType) -> Option<u16> {
        let inner = self.inner.lock();
        let local_port = match protocol_type {
            ProtocolType::UDP => inner.udp_port,
            ProtocolType::TCP => inner.tcp_port,
            ProtocolType::WS => inner.ws_port,
            ProtocolType::WSS => inner.wss_port,
        };
        Some(local_port)
    }

    pub fn get_preferred_local_address(&self, dial_info: &DialInfo) -> Option<SocketAddr> {
        let inner = self.inner.lock();

        let local_port = match dial_info.protocol_type() {
            ProtocolType::UDP => inner.udp_port,
            ProtocolType::TCP => inner.tcp_port,
            ProtocolType::WS => inner.ws_port,
            ProtocolType::WSS => inner.wss_port,
        };

        Some(match dial_info.address_type() {
            AddressType::IPV4 => SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), local_port),
            AddressType::IPV6 => SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), local_port),
        })
    }

    pub fn is_stable_interface_address(&self, addr: IpAddr) -> bool {
        let stable_addrs = self.get_stable_interface_addresses();
        stable_addrs.contains(&addr)
    }

    pub fn get_stable_interface_addresses(&self) -> Vec<IpAddr> {
        let addrs = self.unlocked_inner.interfaces.stable_addresses();
        let addrs: Vec<IpAddr> = addrs
            .into_iter()
            .filter(|addr| {
                let address = Address::from_ip_addr(*addr);
                address.is_local() || address.is_global()
            })
            .collect();
        addrs
    }

    // See if our interface addresses have changed, if so redo public dial info if necessary
    async fn check_interface_addresses(&self) -> EyreResult<bool> {
        if !self
            .unlocked_inner
            .interfaces
            .refresh()
            .await
            .wrap_err("failed to check network interfaces")?
        {
            return Ok(false);
        }

        self.inner.lock().needs_public_dial_info_check = true;

        Ok(true)
    }

    ////////////////////////////////////////////////////////////

    // Record DialInfo failures
    pub async fn record_dial_info_failure<T, F: Future<Output = EyreResult<NetworkResult<T>>>>(
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
    #[cfg_attr(feature="verbose-tracing", instrument(level="trace", err, skip(self, data), fields(data.len = data.len())))]
    pub async fn send_data_unbound_to_dial_info(
        &self,
        dial_info: DialInfo,
        data: Vec<u8>,
    ) -> EyreResult<NetworkResult<()>> {
        self.record_dial_info_failure(dial_info.clone(), async move {
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
                    let h = RawUdpProtocolHandler::new_unspecified_bound_handler(&peer_socket_addr)
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
        })
        .await
    }

    // Send data to a dial info, unbound, using a new connection from a random port
    // Waits for a specified amount of time to receive a single response
    // This creates a short-lived connection in the case of connection-oriented protocols
    // for the purpose of sending this one message.
    // This bypasses the connection table as it is not a 'node to node' connection.
    #[cfg_attr(feature="verbose-tracing", instrument(level="trace", err, skip(self, data), fields(data.len = data.len())))]
    pub async fn send_recv_data_unbound_to_dial_info(
        &self,
        dial_info: DialInfo,
        data: Vec<u8>,
        timeout_ms: u32,
    ) -> EyreResult<NetworkResult<Vec<u8>>> {
        self.record_dial_info_failure(dial_info.clone(), async move {
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
                    let h = RawUdpProtocolHandler::new_unspecified_bound_handler(&peer_socket_addr)
                        .await
                        .wrap_err("create socket failure")?;
                    network_result_try!(h
                        .send_message(data, peer_socket_addr)
                        .await
                        .wrap_err("send message failure")?);
                    self.network_manager()
                        .stats_packet_sent(dial_info.ip_addr(), ByteCount::new(data_len as u64));

                    // receive single response
                    let mut out = vec![0u8; MAX_MESSAGE_SIZE];
                    let (recv_len, recv_addr) = network_result_try!(timeout(
                        timeout_ms,
                        h.recv_message(&mut out).instrument(Span::current())
                    )
                    .await
                    .into_network_result())
                    .wrap_err("recv_message failure")?;

                    let recv_socket_addr = recv_addr.remote_address().socket_addr();
                    self.network_manager()
                        .stats_packet_rcvd(recv_socket_addr.ip(), ByteCount::new(recv_len as u64));

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
                            WebsocketProtocolHandler::connect(None, &dial_info, connect_timeout_ms)
                                .await
                                .wrap_err("connect failure")?
                        }
                    });

                    network_result_try!(pnc.send(data).await.wrap_err("send failure")?);
                    self.network_manager()
                        .stats_packet_sent(dial_info.ip_addr(), ByteCount::new(data_len as u64));

                    let out =
                        network_result_try!(network_result_try!(timeout(timeout_ms, pnc.recv())
                            .await
                            .into_network_result())
                        .wrap_err("recv failure")?);

                    self.network_manager()
                        .stats_packet_rcvd(dial_info.ip_addr(), ByteCount::new(out.len() as u64));

                    Ok(NetworkResult::Value(out))
                }
            }
        })
        .await
    }

    #[cfg_attr(feature="verbose-tracing", instrument(level="trace", err, skip(self, data), fields(data.len = data.len())))]
    pub async fn send_data_to_existing_connection(
        &self,
        descriptor: ConnectionDescriptor,
        data: Vec<u8>,
    ) -> EyreResult<Option<Vec<u8>>> {
        let data_len = data.len();

        // Handle connectionless protocol
        if descriptor.protocol_type() == ProtocolType::UDP {
            // send over the best udp socket we have bound since UDP is not connection oriented
            let peer_socket_addr = descriptor.remote().socket_addr();
            if let Some(ph) = self.find_best_udp_protocol_handler(
                &peer_socket_addr,
                &descriptor.local().map(|sa| sa.socket_addr()),
            ) {
                network_result_value_or_log!(ph.clone()
                    .send_message(data.clone(), peer_socket_addr)
                    .await
                    .wrap_err("sending data to existing connection")? => [ format!(": data.len={}, descriptor={:?}", data.len(), descriptor) ] 
                    { return Ok(Some(data)); } );

                // Network accounting
                self.network_manager()
                    .stats_packet_sent(peer_socket_addr.ip(), ByteCount::new(data_len as u64));

                // Data was consumed
                return Ok(None);
            }
        }

        // Handle connection-oriented protocols

        // Try to send to the exact existing connection if one exists
        if let Some(conn) = self.connection_manager().get_connection(descriptor) {
            // connection exists, send over it
            match conn.send_async(data).await {
                ConnectionHandleSendResult::Sent => {
                    // Network accounting
                    self.network_manager().stats_packet_sent(
                        descriptor.remote().socket_addr().ip(),
                        ByteCount::new(data_len as u64),
                    );

                    // Data was consumed
                    return Ok(None);
                }
                ConnectionHandleSendResult::NotSent(data) => {
                    // Couldn't send
                    // Pass the data back out so we don't own it any more
                    return Ok(Some(data));
                }
            }
        }
        // Connection didn't exist
        // Pass the data back out so we don't own it any more
        Ok(Some(data))
    }

    // Send data directly to a dial info, possibly without knowing which node it is going to
    // Returns a descriptor for the connection used to send the data
    #[cfg_attr(feature="verbose-tracing", instrument(level="trace", err, skip(self, data), fields(data.len = data.len())))]
    pub async fn send_data_to_dial_info(
        &self,
        dial_info: DialInfo,
        data: Vec<u8>,
    ) -> EyreResult<NetworkResult<ConnectionDescriptor>> {
        self.record_dial_info_failure(dial_info.clone(), async move {
            let data_len = data.len();
            let connection_descriptor;
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
                connection_descriptor = network_result_try!(ph
                    .send_message(data, peer_socket_addr)
                    .await
                    .wrap_err("failed to send data to dial info")?);
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
                connection_descriptor = conn.connection_descriptor();
            }

            // Network accounting
            self.network_manager()
                .stats_packet_sent(dial_info.ip_addr(), ByteCount::new(data_len as u64));

            Ok(NetworkResult::value(connection_descriptor))
        })
        .await
    }

    /////////////////////////////////////////////////////////////////

    #[instrument(level = "debug", err, skip_all)]
    pub async fn startup(&self) -> EyreResult<()> {
        // initialize interfaces
        self.unlocked_inner.interfaces.refresh().await?;

        // build the set of networks we should consider for the 'LocalNetwork' routing domain
        let mut local_networks: HashSet<(IpAddr, IpAddr)> = HashSet::new();
        self.unlocked_inner
            .interfaces
            .with_interfaces(|interfaces| {
                debug!("interfaces: {:#?}", interfaces);

                for intf in interfaces.values() {
                    // Skip networks that we should never encounter
                    if intf.is_loopback() || !intf.is_running() {
                        continue;
                    }
                    // Add network to local networks table
                    for addr in &intf.addrs {
                        let netmask = addr.if_addr().netmask();
                        let network_ip = ipaddr_apply_netmask(addr.if_addr().ip(), netmask);
                        local_networks.insert((network_ip, netmask));
                    }
                }
            });
        let local_networks: Vec<(IpAddr, IpAddr)> = local_networks.into_iter().collect();
        self.unlocked_inner
            .routing_table
            .configure_local_network_routing_domain(local_networks);

        // determine if we have ipv4/ipv6 addresses
        {
            let mut inner = self.inner.lock();
            inner.enable_ipv4 = false;
            for addr in self.get_stable_interface_addresses() {
                if addr.is_ipv4() {
                    log_net!(debug "enable address {:?} as ipv4", addr);
                    inner.enable_ipv4 = true;
                } else if addr.is_ipv6() {
                    let address = Address::from_ip_addr(addr);
                    if address.is_global() {
                        log_net!(debug "enable address {:?} as ipv6 global", address);
                        inner.enable_ipv6_global = true;
                    } else if address.is_local() {
                        log_net!(debug "enable address {:?} as ipv6 local", address);
                        inner.enable_ipv6_local = true;
                    }
                }
            }
        }

        // Build our protocol config to share it with other nodes
        let protocol_config = {
            let mut inner = self.inner.lock();

            // Create stop source
            inner.stop_source = Some(StopSource::new());

            // get protocol config
            let protocol_config = {
                let c = self.config.get();
                let mut inbound = ProtocolTypeSet::new();

                if c.network.protocol.udp.enabled {
                    inbound.insert(ProtocolType::UDP);
                }
                if c.network.protocol.tcp.listen {
                    inbound.insert(ProtocolType::TCP);
                }
                if c.network.protocol.ws.listen {
                    inbound.insert(ProtocolType::WS);
                }
                if c.network.protocol.wss.listen {
                    inbound.insert(ProtocolType::WSS);
                }

                let mut outbound = ProtocolTypeSet::new();
                if c.network.protocol.udp.enabled {
                    outbound.insert(ProtocolType::UDP);
                }
                if c.network.protocol.tcp.connect {
                    outbound.insert(ProtocolType::TCP);
                }
                if c.network.protocol.ws.connect {
                    outbound.insert(ProtocolType::WS);
                }
                if c.network.protocol.wss.connect {
                    outbound.insert(ProtocolType::WSS);
                }

                let mut family_global = AddressTypeSet::new();
                let mut family_local = AddressTypeSet::new();
                if inner.enable_ipv4 {
                    family_global.insert(AddressType::IPV4);
                    family_local.insert(AddressType::IPV4);
                }
                if inner.enable_ipv6_global {
                    family_global.insert(AddressType::IPV6);
                }
                if inner.enable_ipv6_local {
                    family_local.insert(AddressType::IPV6);
                }

                // set up the routing table's network config
                // if we have static public dialinfo, upgrade our network class
                let public_internet_capabilities = {
                    PUBLIC_INTERNET_CAPABILITIES
                        .iter()
                        .copied()
                        .filter(|cap| !c.capabilities.disable.contains(cap))
                        .collect::<Vec<Capability>>()
                };
                let local_network_capabilities = {
                    LOCAL_NETWORK_CAPABILITIES
                        .iter()
                        .copied()
                        .filter(|cap| !c.capabilities.disable.contains(cap))
                        .collect::<Vec<Capability>>()
                };

                ProtocolConfig {
                    outbound,
                    inbound,
                    family_global,
                    family_local,
                    public_internet_capabilities,
                    local_network_capabilities,
                }
            };
            inner.protocol_config = protocol_config.clone();

            protocol_config
        };

        // Start editing routing table
        let mut editor_public_internet = self
            .unlocked_inner
            .routing_table
            .edit_routing_domain(RoutingDomain::PublicInternet);
        let mut editor_local_network = self
            .unlocked_inner
            .routing_table
            .edit_routing_domain(RoutingDomain::LocalNetwork);

        // start listeners
        if protocol_config.inbound.contains(ProtocolType::UDP) {
            self.start_udp_listeners(&mut editor_public_internet, &mut editor_local_network)
                .await?;
        }
        if protocol_config.inbound.contains(ProtocolType::WS) {
            self.start_ws_listeners(&mut editor_public_internet, &mut editor_local_network)
                .await?;
        }
        if protocol_config.inbound.contains(ProtocolType::WSS) {
            self.start_wss_listeners(&mut editor_public_internet, &mut editor_local_network)
                .await?;
        }
        if protocol_config.inbound.contains(ProtocolType::TCP) {
            self.start_tcp_listeners(&mut editor_public_internet, &mut editor_local_network)
                .await?;
        }

        // release caches of available listener ports
        // this releases the 'first bound' ports we use to guarantee
        // that we have ports available to us
        self.free_bound_first_ports();

        editor_public_internet.setup_network(
            protocol_config.outbound,
            protocol_config.inbound,
            protocol_config.family_global,
            protocol_config.public_internet_capabilities,
        );
        editor_local_network.setup_network(
            protocol_config.outbound,
            protocol_config.inbound,
            protocol_config.family_local,
            protocol_config.local_network_capabilities,
        );
        let detect_address_changes = {
            let c = self.config.get();
            c.network.detect_address_changes
        };
        if !detect_address_changes {
            let inner = self.inner.lock();
            if !inner.static_public_dialinfo.is_empty() {
                editor_public_internet.set_network_class(Some(NetworkClass::InboundCapable));
            }
        }

        // commit routing table edits
        editor_public_internet.commit(true).await;
        editor_local_network.commit(true).await;

        info!("network started");
        self.inner.lock().network_started = true;

        Ok(())
    }

    pub fn needs_restart(&self) -> bool {
        self.inner.lock().network_needs_restart
    }

    pub fn is_started(&self) -> bool {
        self.inner.lock().network_started
    }

    #[instrument(level = "debug", skip_all)]
    pub fn restart_network(&self) {
        self.inner.lock().network_needs_restart = true;
    }

    #[instrument(level = "debug", skip_all)]
    pub async fn shutdown(&self) {
        debug!("starting low level network shutdown");

        let routing_table = self.routing_table();

        // Stop all tasks
        debug!("stopping update network class task");
        if let Err(e) = self.unlocked_inner.update_network_class_task.stop().await {
            error!("update_network_class_task not cancelled: {}", e);
        }

        let mut unord = FuturesUnordered::new();
        {
            let mut inner = self.inner.lock();
            // take the join handles out
            for h in inner.join_handles.drain(..) {
                trace!("joining: {:?}", h);
                unord.push(h);
            }
            // Drop the stop
            drop(inner.stop_source.take());
        }
        debug!("stopping {} low level network tasks", unord.len());
        // Wait for everything to stop
        while unord.next().await.is_some() {}

        debug!("clearing dial info");

        routing_table
            .edit_routing_domain(RoutingDomain::PublicInternet)
            .clear_dial_info_details(None, None)
            .set_network_class(None)
            .clear_relay_node()
            .commit(true)
            .await;

        routing_table
            .edit_routing_domain(RoutingDomain::LocalNetwork)
            .clear_dial_info_details(None, None)
            .set_network_class(None)
            .clear_relay_node()
            .commit(true)
            .await;

        // Reset state including network class
        *self.inner.lock() = Self::new_inner();

        debug!("finished low level network shutdown");
    }

    //////////////////////////////////////////
    pub fn set_needs_public_dial_info_check(
        &self,
        punishment: Option<Box<dyn FnOnce() + Send + 'static>>,
    ) {
        let mut inner = self.inner.lock();
        inner.needs_public_dial_info_check = true;
        inner.public_dial_info_check_punishment = punishment;
    }

    pub fn needs_public_dial_info_check(&self) -> bool {
        let inner = self.inner.lock();
        inner.needs_public_dial_info_check
    }

    //////////////////////////////////////////

    #[instrument(level = "trace", skip(self), err)]
    pub async fn network_interfaces_task_routine(
        self,
        stop_token: StopToken,
        _l: u64,
        _t: u64,
    ) -> EyreResult<()> {
        self.check_interface_addresses().await?;

        Ok(())
    }

    #[instrument(level = "trace", skip(self), err)]
    pub async fn upnp_task_routine(
        self,
        stop_token: StopToken,
        _l: u64,
        _t: u64,
    ) -> EyreResult<()> {
        if !self.unlocked_inner.igd_manager.tick().await? {
            info!("upnp failed, restarting local network");
            let mut inner = self.inner.lock();
            inner.network_needs_restart = true;
        }

        Ok(())
    }

    pub async fn tick(&self) -> EyreResult<()> {
        let (detect_address_changes, upnp) = {
            let config = self.network_manager().config();
            let c = config.get();
            (c.network.detect_address_changes, c.network.upnp)
        };

        // If we need to figure out our network class, tick the task for it
        if detect_address_changes {
            let public_internet_network_class = self
                .routing_table()
                .get_network_class(RoutingDomain::PublicInternet)
                .unwrap_or(NetworkClass::Invalid);
            let needs_public_dial_info_check = self.needs_public_dial_info_check();
            if public_internet_network_class == NetworkClass::Invalid
                || needs_public_dial_info_check
            {
                let routing_table = self.routing_table();
                let rth = routing_table.get_routing_table_health();

                // We want at least two live entries per crypto kind before we start doing this (bootstrap)
                let mut has_at_least_two = true;
                for ck in VALID_CRYPTO_KINDS {
                    if rth
                        .live_entry_counts
                        .get(&(RoutingDomain::PublicInternet, ck))
                        .copied()
                        .unwrap_or_default()
                        < 2
                    {
                        has_at_least_two = false;
                        break;
                    }
                }

                if has_at_least_two {
                    self.unlocked_inner.update_network_class_task.tick().await?;
                }
            }

            // Check our network interfaces to see if they have changed
            if !self.needs_restart() {
                self.unlocked_inner.network_interfaces_task.tick().await?;
            }
        }

        // If we need to tick upnp, do it
        if upnp && !self.needs_restart() {
            self.unlocked_inner.upnp_task.tick().await?;
        }

        Ok(())
    }
}
