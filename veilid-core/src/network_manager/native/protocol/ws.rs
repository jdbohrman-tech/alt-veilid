use super::*;

use async_tls::TlsConnector;
use async_tungstenite::tungstenite::protocol::Message;
use async_tungstenite::{accept_async, client_async, WebSocketStream};
use futures_util::{AsyncRead, AsyncWrite, SinkExt};
use sockets::*;
cfg_if! {
    if #[cfg(feature="rt-async-std")] {
        pub type WebsocketNetworkConnectionWSS =
            WebsocketNetworkConnection<async_tls::client::TlsStream<TcpStream>>;
        pub type WebsocketNetworkConnectionWS = WebsocketNetworkConnection<TcpStream>;
    } else if #[cfg(feature="rt-tokio")] {
        pub type WebsocketNetworkConnectionWSS =
            WebsocketNetworkConnection<async_tls::client::TlsStream<Compat<TcpStream>>>;
        pub type WebsocketNetworkConnectionWS = WebsocketNetworkConnection<Compat<TcpStream>>;
    }
}

fn err_to_network_result<T>(err: async_tungstenite::tungstenite::Error) -> NetworkResult<T> {
    match err {
        async_tungstenite::tungstenite::Error::ConnectionClosed
        | async_tungstenite::tungstenite::Error::AlreadyClosed
        | async_tungstenite::tungstenite::Error::Io(_)
        | async_tungstenite::tungstenite::Error::Protocol(
            async_tungstenite::tungstenite::error::ProtocolError::ResetWithoutClosingHandshake,
        ) => NetworkResult::NoConnection(to_io_error_other(err)),
        _ => NetworkResult::InvalidMessage(err.to_string()),
    }
}

pub type WebSocketNetworkConnectionAccepted = WebsocketNetworkConnection<AsyncPeekStream>;

pub struct WebsocketNetworkConnection<T>
where
    T: AsyncRead + AsyncWrite + Send + Unpin + 'static,
{
    descriptor: ConnectionDescriptor,
    stream: CloneStream<WebSocketStream<T>>,
}

impl<T> fmt::Debug for WebsocketNetworkConnection<T>
where
    T: AsyncRead + AsyncWrite + Send + Unpin + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", std::any::type_name::<Self>())
    }
}

impl<T> WebsocketNetworkConnection<T>
where
    T: AsyncRead + AsyncWrite + Send + Unpin + 'static,
{
    pub fn new(descriptor: ConnectionDescriptor, stream: WebSocketStream<T>) -> Self {
        Self {
            descriptor,
            stream: CloneStream::new(stream),
        }
    }

    pub fn descriptor(&self) -> ConnectionDescriptor {
        self.descriptor.clone()
    }

    // #[instrument(level = "trace", err, skip(self))]
    // pub async fn close(&self) -> io::Result<()> {
    //     // Make an attempt to flush the stream
    //     self.stream.clone().close().await.map_err(to_io_error_other)?;
    //     // Then forcibly close the socket
    //     self.tcp_stream
    //         .shutdown(Shutdown::Both)
    //         .map_err(to_io_error_other)
    // }

    #[cfg_attr(feature="verbose-tracing", instrument(level = "trace", err, skip(self, message), fields(network_result, message.len = message.len())))]
    pub async fn send(&self, message: Vec<u8>) -> io::Result<NetworkResult<()>> {
        if message.len() > MAX_MESSAGE_SIZE {
            bail_io_error_other!("received too large WS message");
        }
        let out = match self.stream.clone().send(Message::binary(message)).await {
            Ok(v) => NetworkResult::value(v),
            Err(e) => err_to_network_result(e),
        };
        if !out.is_value() {
            #[cfg(feature = "verbose-tracing")]
            tracing::Span::current().record("network_result", &tracing::field::display(&out));
            return Ok(out);
        }
        let out = match self.stream.clone().flush().await {
            Ok(v) => NetworkResult::value(v),
            Err(e) => err_to_network_result(e),
        };

        #[cfg(feature = "verbose-tracing")]
        tracing::Span::current().record("network_result", &tracing::field::display(&out));
        Ok(out)
    }

    #[cfg_attr(feature="verbose-tracing", instrument(level = "trace", err, skip(self), fields(network_result, ret.len)))]
    pub async fn recv(&self) -> io::Result<NetworkResult<Vec<u8>>> {
        let out = match self.stream.clone().next().await {
            Some(Ok(Message::Binary(v))) => {
                if v.len() > MAX_MESSAGE_SIZE {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "too large ws message",
                    ));
                }
                NetworkResult::Value(v)
            }
            Some(Ok(Message::Close(_))) => NetworkResult::NoConnection(io::Error::new(
                io::ErrorKind::ConnectionReset,
                "closeframe",
            )),
            Some(Ok(x)) => NetworkResult::NoConnection(io::Error::new(
                io::ErrorKind::ConnectionReset,
                format!("Unexpected WS message type: {:?}", x),
            )),
            Some(Err(e)) => err_to_network_result(e),
            None => NetworkResult::NoConnection(io::Error::new(
                io::ErrorKind::ConnectionReset,
                "connection ended normally",
            )),
        };

        #[cfg(feature = "verbose-tracing")]
        tracing::Span::current().record("network_result", &tracing::field::display(&out));
        Ok(out)
    }
}

///////////////////////////////////////////////////////////
///
struct WebsocketProtocolHandlerArc {
    tls: bool,
    request_path: Vec<u8>,
    connection_initial_timeout_ms: u32,
}

#[derive(Clone)]
pub struct WebsocketProtocolHandler
where
    Self: ProtocolAcceptHandler,
{
    arc: Arc<WebsocketProtocolHandlerArc>,
}
impl WebsocketProtocolHandler {
    pub fn new(config: VeilidConfig, tls: bool) -> Self {
        let c = config.get();
        let path = if tls {
            format!("GET /{}", c.network.protocol.ws.path.trim_end_matches('/'))
        } else {
            format!("GET /{}", c.network.protocol.wss.path.trim_end_matches('/'))
        };
        let connection_initial_timeout_ms = if tls {
            c.network.tls.connection_initial_timeout_ms
        } else {
            c.network.connection_initial_timeout_ms
        };

        Self {
            arc: Arc::new(WebsocketProtocolHandlerArc {
                tls,
                request_path: path.as_bytes().to_vec(),
                connection_initial_timeout_ms,
            }),
        }
    }

    #[instrument(level = "trace", err, skip(self, ps))]
    pub async fn on_accept_async(
        self,
        ps: AsyncPeekStream,
        socket_addr: SocketAddr,
        local_addr: SocketAddr,
    ) -> io::Result<Option<ProtocolNetworkConnection>> {
        log_net!("WS: on_accept_async: enter");
        let request_path_len = self.arc.request_path.len() + 2;

        let mut peekbuf: Vec<u8> = vec![0u8; request_path_len];
        if let Err(_) = timeout(
            self.arc.connection_initial_timeout_ms,
            ps.peek_exact(&mut peekbuf),
        )
        .await
        {
            return Ok(None);
        }

        // Check for websocket path
        let matches_path = &peekbuf[0..request_path_len - 2] == self.arc.request_path.as_slice()
            && (peekbuf[request_path_len - 2] == b' '
                || (peekbuf[request_path_len - 2] == b'/'
                    && peekbuf[request_path_len - 1] == b' '));

        if !matches_path {
            return Ok(None);
        }

        let ws_stream = accept_async(ps)
            .await
            .map_err(|e| io_error_other!(format!("failed websockets handshake: {}", e)))?;

        // Wrap the websocket in a NetworkConnection and register it
        let protocol_type = if self.arc.tls {
            ProtocolType::WSS
        } else {
            ProtocolType::WS
        };

        let peer_addr =
            PeerAddress::new(SocketAddress::from_socket_addr(socket_addr), protocol_type);

        let conn = ProtocolNetworkConnection::WsAccepted(WebsocketNetworkConnection::new(
            ConnectionDescriptor::new(peer_addr, SocketAddress::from_socket_addr(local_addr)),
            ws_stream,
        ));

        log_net!(debug "Connection accepted from: {} ({})", socket_addr, if self.arc.tls { "WSS" } else { "WS" });

        Ok(Some(conn))
    }

    #[instrument(level = "trace", ret, err)]
    pub async fn connect(
        local_address: Option<SocketAddr>,
        dial_info: &DialInfo,
        timeout_ms: u32,
    ) -> io::Result<NetworkResult<ProtocolNetworkConnection>> {
        // Split dial info up
        let (tls, scheme) = match dial_info {
            DialInfo::WS(_) => (false, "ws"),
            DialInfo::WSS(_) => (true, "wss"),
            _ => panic!("invalid dialinfo for WS/WSS protocol"),
        };
        let request = dial_info.request().unwrap();
        let split_url = SplitUrl::from_str(&request).map_err(to_io_error_other)?;
        if split_url.scheme != scheme {
            bail_io_error_other!("invalid websocket url scheme");
        }
        let domain = split_url.host.clone();

        // Resolve remote address
        let remote_socket_addr = dial_info.to_socket_addr();

        // Make a shared socket
        let socket = match local_address {
            Some(a) => new_bound_shared_tcp_socket(a)?,
            None => new_unbound_tcp_socket(socket2::Domain::for_address(remote_socket_addr))?,
        };

        // Non-blocking connect to remote address
        let tcp_stream =
            network_result_try!(nonblocking_connect(socket, remote_socket_addr, timeout_ms)
                .await
                .folded()?);

        // See what local address we ended up with
        let actual_local_addr = tcp_stream.local_addr()?;

        #[cfg(feature = "rt-tokio")]
        let tcp_stream = tcp_stream.compat();

        // Make our connection descriptor
        let descriptor = ConnectionDescriptor::new(
            dial_info.to_peer_address(),
            SocketAddress::from_socket_addr(actual_local_addr),
        );

        // Negotiate TLS if this is WSS
        if tls {
            let connector = TlsConnector::default();
            let tls_stream = connector.connect(domain.to_string(), tcp_stream).await?;
            let (ws_stream, _response) = client_async(request, tls_stream)
                .await
                .map_err(to_io_error_other)?;

            Ok(NetworkResult::Value(ProtocolNetworkConnection::Wss(
                WebsocketNetworkConnection::new(descriptor, ws_stream),
            )))
        } else {
            let (ws_stream, _response) = client_async(request, tcp_stream)
                .await
                .map_err(to_io_error_other)?;
            Ok(NetworkResult::Value(ProtocolNetworkConnection::Ws(
                WebsocketNetworkConnection::new(descriptor, ws_stream),
            )))
        }
    }
}

impl ProtocolAcceptHandler for WebsocketProtocolHandler {
    fn on_accept(
        &self,
        stream: AsyncPeekStream,
        peer_addr: SocketAddr,
        local_addr: SocketAddr,
    ) -> SendPinBoxFuture<io::Result<Option<ProtocolNetworkConnection>>> {
        Box::pin(self.clone().on_accept_async(stream, peer_addr, local_addr))
    }
}
