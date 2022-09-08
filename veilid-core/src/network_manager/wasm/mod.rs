mod protocol;

use super::*;
use crate::routing_table::*;
use connection_manager::*;
use protocol::ws::WebsocketProtocolHandler;
pub use protocol::*;
use std::io;

/////////////////////////////////////////////////////////////////

struct NetworkInner {
    network_manager: NetworkManager,
    network_started: bool,
    network_needs_restart: bool,
    protocol_config: Option<ProtocolConfig>,
}

#[derive(Clone)]
pub struct Network {
    config: VeilidConfig,
    inner: Arc<Mutex<NetworkInner>>,
}

impl Network {
    fn new_inner(network_manager: NetworkManager) -> NetworkInner {
        NetworkInner {
            network_manager,
            network_started: false,
            network_needs_restart: false,
            protocol_config: None, //join_handle: None,
        }
    }

    pub fn new(
        network_manager: NetworkManager,
        routing_table: RoutingTable,
        connection_manager: ConnectionManager,
    ) -> Self {
        Self {
            config: network_manager.config(),
            inner: Arc::new(Mutex::new(Self::new_inner(network_manager))),
        }
    }

    fn network_manager(&self) -> NetworkManager {
        self.inner.lock().network_manager.clone()
    }
    fn connection_manager(&self) -> ConnectionManager {
        self.inner.lock().network_manager.connection_manager()
    }

    /////////////////////////////////////////////////////////////////

    #[instrument(level="trace", err, skip(self, data), fields(data.len = data.len()))]
    pub async fn send_data_unbound_to_dial_info(
        &self,
        dial_info: DialInfo,
        data: Vec<u8>,
    ) -> EyreResult<NetworkResult<()>> {
        let data_len = data.len();
        let timeout_ms = {
            let c = self.config.get();
            c.network.connection_initial_timeout_ms
        };

        match dial_info.protocol_type() {
            ProtocolType::UDP => {
                bail!("no support for UDP protocol")
            }
            ProtocolType::TCP => {
                bail!("no support for TCP protocol")
            }
            ProtocolType::WS | ProtocolType::WSS => {
                let pnc =
                    network_result_try!(WebsocketProtocolHandler::connect(&dial_info, timeout_ms)
                        .await
                        .wrap_err("connect failure")?);
                network_result_try!(pnc.send(data).await.wrap_err("send failure")?);
            }
        };

        // Network accounting
        self.network_manager()
            .stats_packet_sent(dial_info.to_ip_addr(), data_len as u64);

        Ok(NetworkResult::Value(()))
    }

    // Send data to a dial info, unbound, using a new connection from a random port
    // Waits for a specified amount of time to receive a single response
    // This creates a short-lived connection in the case of connection-oriented protocols
    // for the purpose of sending this one message.
    // This bypasses the connection table as it is not a 'node to node' connection.
    #[instrument(level="trace", err, skip(self, data), fields(data.len = data.len()))]
    pub async fn send_recv_data_unbound_to_dial_info(
        &self,
        dial_info: DialInfo,
        data: Vec<u8>,
        timeout_ms: u32,
    ) -> EyreResult<NetworkResult<Vec<u8>>> {
        let data_len = data.len();
        let connect_timeout_ms = {
            let c = self.config.get();
            c.network.connection_initial_timeout_ms
        };

        match dial_info.protocol_type() {
            ProtocolType::UDP => {
                bail!("no support for UDP protocol")
            }
            ProtocolType::TCP => {
                bail!("no support for TCP protocol")
            }
            ProtocolType::WS | ProtocolType::WSS => {
                let pnc = network_result_try!(match dial_info.protocol_type() {
                    ProtocolType::UDP => unreachable!(),
                    ProtocolType::TCP => unreachable!(),
                    ProtocolType::WS | ProtocolType::WSS => {
                        WebsocketProtocolHandler::connect(&dial_info, connect_timeout_ms)
                            .await
                            .wrap_err("connect failure")?
                    }
                });

                network_result_try!(pnc.send(data).await.wrap_err("send failure")?);
                self.network_manager()
                    .stats_packet_sent(dial_info.to_ip_addr(), data_len as u64);

                let out = network_result_try!(network_result_try!(timeout(timeout_ms, pnc.recv())
                    .await
                    .into_network_result())
                .wrap_err("recv failure")?);

                self.network_manager()
                    .stats_packet_rcvd(dial_info.to_ip_addr(), out.len() as u64);

                Ok(NetworkResult::Value(out))
            }
        }
    }

    #[instrument(level="trace", err, skip(self, data), fields(data.len = data.len()))]
    pub async fn send_data_to_existing_connection(
        &self,
        descriptor: ConnectionDescriptor,
        data: Vec<u8>,
    ) -> EyreResult<Option<Vec<u8>>> {
        let data_len = data.len();
        match descriptor.protocol_type() {
            ProtocolType::UDP => {
                bail!("no support for UDP protocol")
            }
            ProtocolType::TCP => {
                bail!("no support for TCP protocol")
            }
            _ => {}
        }

        // Handle connection-oriented protocols

        // Try to send to the exact existing connection if one exists
        if let Some(conn) = self.connection_manager().get_connection(descriptor) {
            // connection exists, send over it
            match conn.send_async(data).await {
                ConnectionHandleSendResult::Sent => {
                    // Network accounting
                    self.network_manager().stats_packet_sent(
                        descriptor.remote().to_socket_addr().ip(),
                        data_len as u64,
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

    #[instrument(level="trace", err, skip(self, data), fields(data.len = data.len()))]
    pub async fn send_data_to_dial_info(
        &self,
        dial_info: DialInfo,
        data: Vec<u8>,
    ) -> EyreResult<NetworkResult<ConnectionDescriptor>> {
        let data_len = data.len();
        if dial_info.protocol_type() == ProtocolType::UDP {
            bail!("no support for UDP protocol");
        }
        if dial_info.protocol_type() == ProtocolType::TCP {
            bail!("no support for TCP protocol");
        }

        // Handle connection-oriented protocols
        let conn = network_result_try!(
            self.connection_manager()
                .get_or_create_connection(None, dial_info.clone())
                .await?
        );

        if let ConnectionHandleSendResult::NotSent(_) = conn.send_async(data).await {
            return Ok(NetworkResult::NoConnection(io::Error::new(
                io::ErrorKind::ConnectionReset,
                "failed to send",
            )));
        }
        let connection_descriptor = conn.connection_descriptor();

        // Network accounting
        self.network_manager()
            .stats_packet_sent(dial_info.to_ip_addr(), data_len as u64);

        Ok(NetworkResult::value(connection_descriptor))
    }

    /////////////////////////////////////////////////////////////////

    pub async fn startup(&self) -> EyreResult<()> {
        // get protocol config
        self.inner.lock().protocol_config = Some({
            let c = self.config.get();
            let inbound = ProtocolTypeSet::new();
            let mut outbound = ProtocolTypeSet::new();

            if c.network.protocol.ws.connect && c.capabilities.protocol_connect_ws {
                outbound.insert(ProtocolType::WS);
            }
            if c.network.protocol.wss.connect && c.capabilities.protocol_connect_wss {
                outbound.insert(ProtocolType::WSS);
            }

            // XXX: See issue #92
            let family_global = AddressTypeSet::all();
            let family_local = AddressTypeSet::all();

            ProtocolConfig {
                inbound,
                outbound,
                family_global,
                family_local,
            }
        });

        self.inner.lock().network_started = true;
        Ok(())
    }

    pub fn needs_restart(&self) -> bool {
        self.inner.lock().network_needs_restart
    }

    pub fn is_started(&self) -> bool {
        self.inner.lock().network_started
    }

    pub fn restart_network(&self) {
        self.inner.lock().network_needs_restart = true;
    }

    pub async fn shutdown(&self) {
        trace!("stopping network");

        // Reset state
        let network_manager = self.inner.lock().network_manager.clone();
        let routing_table = network_manager.routing_table();

        // Drop all dial info
        routing_table.clear_dial_info_details(RoutingDomain::PublicInternet);
        routing_table.clear_dial_info_details(RoutingDomain::LocalNetwork);

        // Cancels all async background tasks by dropping join handles
        *self.inner.lock() = Self::new_inner(network_manager);

        trace!("network stopped");
    }

    pub fn is_usable_interface_address(&self, addr: IpAddr) -> bool {
        false
    }

    pub fn get_usable_interface_addresses(&self) -> Vec<IpAddr> {
        Vec::new()
    }

    //////////////////////////////////////////
    
    pub fn set_needs_public_dial_info_check(&self, _punishment: Option<Box<dyn FnOnce() + Send + 'static>>) {
        //
    }

    pub fn doing_public_dial_info_check(&self) -> bool {
        false
    }

    pub fn get_network_class(&self, _routing_domain: RoutingDomain) -> Option<NetworkClass> {
        // xxx eventually detect tor browser?
        return if self.inner.lock().network_started {
            Some(NetworkClass::WebApp)
        } else {
            None
        };
    }

    pub fn get_protocol_config(&self) -> Option<ProtocolConfig> {
        self.inner.lock().protocol_config.clone()
    }

    //////////////////////////////////////////
    pub async fn tick(&self) -> EyreResult<()> {
        Ok(())
    }
}
