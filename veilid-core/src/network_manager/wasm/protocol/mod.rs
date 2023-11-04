pub mod wrtc;
pub mod ws;

use super::*;
use std::io;

#[derive(Debug)]
pub(in crate::network_manager) enum ProtocolNetworkConnection {
    #[allow(dead_code)]
    //Dummy(DummyNetworkConnection),
    Ws(ws::WebsocketNetworkConnection),
    //WebRTC(wrtc::WebRTCNetworkConnection),
}

impl ProtocolNetworkConnection {
    pub async fn connect(
        _local_address: Option<SocketAddr>,
        dial_info: &DialInfo,
        timeout_ms: u32,
        address_filter: AddressFilter,
    ) -> io::Result<NetworkResult<ProtocolNetworkConnection>> {
        if address_filter.is_ip_addr_punished(dial_info.address().ip_addr()) {
            return Ok(NetworkResult::no_connection_other("punished"));
        }
        match dial_info.protocol_type() {
            ProtocolType::UDP => {
                panic!("UDP dial info is not supported on WASM targets");
            }
            ProtocolType::TCP => {
                panic!("TCP dial info is not supported on WASM targets");
            }
            ProtocolType::WS | ProtocolType::WSS => {
                ws::WebsocketProtocolHandler::connect(dial_info, timeout_ms).await
            }
        }
    }

    pub fn flow(&self) -> Flow {
        match self {
            //            Self::Dummy(d) => d.flow(),
            Self::Ws(w) => w.flow(),
        }
    }
    pub async fn close(&self) -> io::Result<NetworkResult<()>> {
        match self {
            //            Self::Dummy(d) => d.close(),
            Self::Ws(w) => w.close().await,
        }
    }
    pub async fn send(&self, message: Vec<u8>) -> io::Result<NetworkResult<()>> {
        match self {
            //            Self::Dummy(d) => d.send(message),
            Self::Ws(w) => w.send(message).await,
        }
    }

    pub async fn recv(&self) -> io::Result<NetworkResult<Vec<u8>>> {
        match self {
            //            Self::Dummy(d) => d.recv(),
            Self::Ws(w) => w.recv().await,
        }
    }
}
