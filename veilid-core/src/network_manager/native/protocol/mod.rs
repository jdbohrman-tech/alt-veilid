pub mod sockets;
pub mod tcp;
pub mod udp;
pub mod wrtc;
pub mod ws;

use super::*;
use std::io;

#[derive(Debug)]
pub enum ProtocolNetworkConnection {
    Dummy(DummyNetworkConnection),
    RawTcp(tcp::RawTcpNetworkConnection),
    WsAccepted(ws::WebSocketNetworkConnectionAccepted),
    Ws(ws::WebsocketNetworkConnectionWS),
    Wss(ws::WebsocketNetworkConnectionWSS),
    //WebRTC(wrtc::WebRTCNetworkConnection),
}

impl ProtocolNetworkConnection {
    pub async fn connect(
        local_address: Option<SocketAddr>,
        dial_info: &DialInfo,
        timeout_ms: u32,
        address_filter: AddressFilter,
    ) -> io::Result<NetworkResult<ProtocolNetworkConnection>> {
        if address_filter.is_ip_addr_punished(dial_info.address().to_ip_addr()) {
            return Ok(NetworkResult::no_connection_other("punished"));
        }
        match dial_info.protocol_type() {
            ProtocolType::UDP => {
                panic!("Should not connect to UDP dialinfo");
            }
            ProtocolType::TCP => {
                tcp::RawTcpProtocolHandler::connect(
                    local_address,
                    dial_info.to_socket_addr(),
                    timeout_ms,
                )
                .await
            }
            ProtocolType::WS | ProtocolType::WSS => {
                ws::WebsocketProtocolHandler::connect(local_address, dial_info, timeout_ms).await
            }
        }
    }

    pub fn descriptor(&self) -> ConnectionDescriptor {
        match self {
            Self::Dummy(d) => d.descriptor(),
            Self::RawTcp(t) => t.descriptor(),
            Self::WsAccepted(w) => w.descriptor(),
            Self::Ws(w) => w.descriptor(),
            Self::Wss(w) => w.descriptor(),
        }
    }

    // pub async fn close(&self) -> io::Result<NetworkResult<()>> {
    //     match self {
    //         Self::Dummy(d) => d.close(),
    //         Self::RawTcp(t) => t.close().await,
    //         Self::WsAccepted(w) => w.close().await,
    //         Self::Ws(w) => w.close().await,
    //         Self::Wss(w) => w.close().await,
    //     }
    // }

    pub async fn send(&self, message: Vec<u8>) -> io::Result<NetworkResult<()>> {
        match self {
            Self::Dummy(d) => d.send(message),
            Self::RawTcp(t) => t.send(message).await,
            Self::WsAccepted(w) => w.send(message).await,
            Self::Ws(w) => w.send(message).await,
            Self::Wss(w) => w.send(message).await,
        }
    }
    pub async fn recv(&self) -> io::Result<NetworkResult<Vec<u8>>> {
        match self {
            Self::Dummy(d) => d.recv(),
            Self::RawTcp(t) => t.recv().await,
            Self::WsAccepted(w) => w.recv().await,
            Self::Ws(w) => w.recv().await,
            Self::Wss(w) => w.recv().await,
        }
    }
}
