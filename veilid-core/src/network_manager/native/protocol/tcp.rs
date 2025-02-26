use super::*;
use futures_util::{AsyncReadExt, AsyncWriteExt};

pub struct RawTcpNetworkConnection {
    registry: VeilidComponentRegistry,
    flow: Flow,
    stream: Mutex<Option<AsyncPeekStream>>,
}

impl fmt::Debug for RawTcpNetworkConnection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RawTcpNetworkConnection")
            //.field("registry", &self.registry)
            .field("flow", &self.flow)
            //.field("stream", &self.stream)
            .finish()
    }
}

impl_veilid_component_registry_accessor!(RawTcpNetworkConnection);

impl RawTcpNetworkConnection {
    pub fn new(registry: VeilidComponentRegistry, flow: Flow, stream: AsyncPeekStream) -> Self {
        Self {
            registry,
            flow,
            stream: Mutex::new(Some(stream)),
        }
    }

    pub fn flow(&self) -> Flow {
        self.flow
    }

    #[instrument(level = "trace", target = "protocol", err, skip_all)]
    pub async fn close(&self) -> io::Result<NetworkResult<()>> {
        // Drop the stream, without calling close, which calls shutdown, which causes TIME_WAIT regardless of SO_LINGER settings
        drop(self.stream.lock().take());
        // let _ = stream.close().await;
        Ok(NetworkResult::value(()))
    }

    #[instrument(level = "trace", target = "protocol", err, skip_all)]
    async fn send_internal(
        stream: &mut AsyncPeekStream,
        message: Vec<u8>,
    ) -> io::Result<NetworkResult<()>> {
        if message.len() > MAX_MESSAGE_SIZE {
            bail_io_error_other!("sending too large TCP message");
        }

        let len = message.len() as u16;
        let header = [b'V', b'L', len as u8, (len >> 8) as u8];

        let mut data = Vec::with_capacity(message.len() + 4);
        data.extend_from_slice(&header);
        data.extend_from_slice(&message);

        network_result_try!(stream.write_all(&data).await.into_network_result()?);

        stream.flush().await.into_network_result()
    }

    #[instrument(level="trace", target="protocol", err, skip(self, message), fields(network_result, message.len = message.len()))]
    pub async fn send(&self, message: Vec<u8>) -> io::Result<NetworkResult<()>> {
        let Some(mut stream) = self.stream.lock().clone() else {
            bail_io_error_other!("already closed");
        };
        let out = Self::send_internal(&mut stream, message).await?;
        #[cfg(feature = "verbose-tracing")]
        tracing::Span::current().record("network_result", &tracing::field::display(&out));
        Ok(out)
    }

    #[instrument(level = "trace", target = "protocol", err, skip_all)]
    async fn recv_internal(stream: &mut AsyncPeekStream) -> io::Result<NetworkResult<Vec<u8>>> {
        let mut header = [0u8; 4];

        network_result_try!(stream.read_exact(&mut header).await.into_network_result()?);
        if header[0] != b'V' || header[1] != b'L' {
            return Ok(NetworkResult::invalid_message(
                "received invalid TCP frame header",
            ));
        }
        let len = ((header[3] as usize) << 8) | (header[2] as usize);
        if len > MAX_MESSAGE_SIZE {
            return Ok(NetworkResult::invalid_message(
                "received too large TCP frame",
            ));
        }

        let mut out: Vec<u8> = vec![0u8; len];
        let nrout = stream.read_exact(&mut out).await.into_network_result()?;
        network_result_try!(nrout);

        Ok(NetworkResult::Value(out))
    }

    #[instrument(level = "trace", target = "protocol", err, skip_all)]
    pub async fn recv(&self) -> io::Result<NetworkResult<Vec<u8>>> {
        let Some(mut stream) = self.stream.lock().clone() else {
            bail_io_error_other!("already closed");
        };
        let out = Self::recv_internal(&mut stream).await?;
        #[cfg(feature = "verbose-tracing")]
        tracing::Span::current().record("network_result", &tracing::field::display(&out));
        Ok(out)
    }
}

///////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct RawTcpProtocolHandler
where
    Self: ProtocolAcceptHandler,
{
    registry: VeilidComponentRegistry,
    connection_initial_timeout_ms: u32,
}

impl_veilid_component_registry_accessor!(RawTcpProtocolHandler);

impl RawTcpProtocolHandler {
    pub fn new(registry: VeilidComponentRegistry) -> Self {
        let connection_initial_timeout_ms = registry
            .config()
            .with(|c| c.network.connection_initial_timeout_ms);
        Self {
            registry,
            connection_initial_timeout_ms,
        }
    }

    #[instrument(level = "trace", target = "protocol", err, skip_all)]
    async fn on_accept_async(
        self,
        ps: AsyncPeekStream,
        socket_addr: SocketAddr,
        local_addr: SocketAddr,
    ) -> io::Result<Option<ProtocolNetworkConnection>> {
        veilid_log!(self trace "TCP: on_accept_async: enter");
        let mut peekbuf: [u8; PEEK_DETECT_LEN] = [0u8; PEEK_DETECT_LEN];
        if (timeout(
            self.connection_initial_timeout_ms,
            ps.peek_exact(&mut peekbuf).in_current_span(),
        )
        .await)
            .is_err()
        {
            return Ok(None);
        }

        let peer_addr = PeerAddress::new(
            SocketAddress::from_socket_addr(socket_addr),
            ProtocolType::TCP,
        );
        let conn = ProtocolNetworkConnection::RawTcp(RawTcpNetworkConnection::new(
            self.registry(),
            Flow::new(peer_addr, SocketAddress::from_socket_addr(local_addr)),
            ps,
        ));

        veilid_log!(self trace "Connection accepted from: {} (TCP)", socket_addr);

        Ok(Some(conn))
    }

    #[instrument(level = "trace", target = "protocol", err)]
    pub async fn connect(
        registry: VeilidComponentRegistry,
        local_address: Option<SocketAddr>,
        remote_address: SocketAddr,
        timeout_ms: u32,
    ) -> io::Result<NetworkResult<ProtocolNetworkConnection>> {
        // Non-blocking connect to remote address
        let tcp_stream = network_result_try!(connect_async_tcp_stream(
            local_address,
            remote_address,
            timeout_ms
        )
        .await
        .folded()?);

        // See what local address we ended up with and turn this into a stream
        let actual_local_address = tcp_stream.local_addr()?;
        #[cfg(feature = "rt-tokio")]
        let tcp_stream = tcp_stream.compat();
        let ps = AsyncPeekStream::new(tcp_stream);

        // Wrap the stream in a network connection and return it
        let flow = Flow::new(
            PeerAddress::new(
                SocketAddress::from_socket_addr(remote_address),
                ProtocolType::TCP,
            ),
            SocketAddress::from_socket_addr(actual_local_address),
        );
        veilid_log!(registry trace "rawtcp::connect: {:?}", flow);

        let conn =
            ProtocolNetworkConnection::RawTcp(RawTcpNetworkConnection::new(registry, flow, ps));

        Ok(NetworkResult::Value(conn))
    }
}

impl ProtocolAcceptHandler for RawTcpProtocolHandler {
    fn on_accept(
        &self,
        stream: AsyncPeekStream,
        peer_addr: SocketAddr,
        local_addr: SocketAddr,
    ) -> PinBoxFutureStatic<io::Result<Option<ProtocolNetworkConnection>>> {
        Box::pin(self.clone().on_accept_async(stream, peer_addr, local_addr))
    }
}
