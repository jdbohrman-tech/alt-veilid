use super::*;
use futures_util::{AsyncReadExt, AsyncWriteExt};
use sockets::*;

pub struct RawTcpNetworkConnection {
    descriptor: ConnectionDescriptor,
    stream: AsyncPeekStream,
}

impl fmt::Debug for RawTcpNetworkConnection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RawTCPNetworkConnection").finish()
    }
}

impl RawTcpNetworkConnection {
    pub fn new(descriptor: ConnectionDescriptor, stream: AsyncPeekStream) -> Self {
        Self { descriptor, stream }
    }

    pub fn descriptor(&self) -> ConnectionDescriptor {
        self.descriptor.clone()
    }

    // #[instrument(level = "trace", err, skip(self))]
    // pub async fn close(&mut self) -> io::Result<NetworkResult<()>> {
    //     // Make an attempt to flush the stream
    //     self.stream.clone().close().await?;
    //     // Then shut down the write side of the socket to effect a clean close
    //     cfg_if! {
    //         if #[cfg(feature="rt-async-std")] {
    //             self.tcp_stream
    //                 .shutdown(async_std::net::Shutdown::Write)
    //         } else if #[cfg(feature="rt-tokio")] {
    //             use tokio::io::AsyncWriteExt;
    //             self.tcp_stream.get_mut()
    //                 .shutdown()
    //                 .await
    //         } else {
    //              compile_error!("needs executor implementation")
    //          }
    //     }
    // }

    async fn send_internal(
        stream: &mut AsyncPeekStream,
        message: Vec<u8>,
    ) -> io::Result<NetworkResult<()>> {
        log_net!("sending TCP message of size {}", message.len());
        if message.len() > MAX_MESSAGE_SIZE {
            bail_io_error_other!("sending too large TCP message");
        }
        let len = message.len() as u16;
        let header = [b'V', b'L', len as u8, (len >> 8) as u8];

        network_result_try!(stream.write_all(&header).await.into_network_result()?);
        network_result_try!(stream.write_all(&message).await.into_network_result()?);
        stream.flush().await.into_network_result()
    }

    #[cfg_attr(feature="verbose-tracing", instrument(level="trace", err, skip(self, message), fields(network_result, message.len = message.len())))]
    pub async fn send(&self, message: Vec<u8>) -> io::Result<NetworkResult<()>> {
        let mut stream = self.stream.clone();
        let out = Self::send_internal(&mut stream, message).await?;
        #[cfg(feature = "verbose-tracing")]
        tracing::Span::current().record("network_result", &tracing::field::display(&out));
        Ok(out)
    }

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

    #[cfg_attr(
        feature = "verbose-tracing",
        instrument(level = "trace", err, skip(self), fields(network_result))
    )]
    pub async fn recv(&self) -> io::Result<NetworkResult<Vec<u8>>> {
        let mut stream = self.stream.clone();
        let out = Self::recv_internal(&mut stream).await?;
        #[cfg(feature = "verbose-tracing")]
        tracing::Span::current().record("network_result", &tracing::field::display(&out));
        Ok(out)
    }
}

///////////////////////////////////////////////////////////
///

#[derive(Clone)]
pub struct RawTcpProtocolHandler
where
    Self: ProtocolAcceptHandler,
{
    connection_initial_timeout_ms: u32,
}

impl RawTcpProtocolHandler {
    pub fn new(config: VeilidConfig) -> Self {
        let c = config.get();
        let connection_initial_timeout_ms = c.network.connection_initial_timeout_ms;
        Self {
            connection_initial_timeout_ms,
        }
    }

    #[instrument(level = "trace", err, skip(self, ps))]
    async fn on_accept_async(
        self,
        ps: AsyncPeekStream,
        socket_addr: SocketAddr,
        local_addr: SocketAddr,
    ) -> io::Result<Option<ProtocolNetworkConnection>> {
        log_net!("TCP: on_accept_async: enter");
        let mut peekbuf: [u8; PEEK_DETECT_LEN] = [0u8; PEEK_DETECT_LEN];
        if let Err(_) = timeout(
            self.connection_initial_timeout_ms,
            ps.peek_exact(&mut peekbuf),
        )
        .await
        {
            return Ok(None);
        }

        let peer_addr = PeerAddress::new(
            SocketAddress::from_socket_addr(socket_addr),
            ProtocolType::TCP,
        );
        let conn = ProtocolNetworkConnection::RawTcp(RawTcpNetworkConnection::new(
            ConnectionDescriptor::new(peer_addr, SocketAddress::from_socket_addr(local_addr)),
            ps,
        ));

        log_net!(debug "Connection accepted from: {} (TCP)", socket_addr);

        Ok(Some(conn))
    }

    #[instrument(level = "trace", ret, err)]
    pub async fn connect(
        local_address: Option<SocketAddr>,
        socket_addr: SocketAddr,
        timeout_ms: u32,
    ) -> io::Result<NetworkResult<ProtocolNetworkConnection>> {
        // Make a shared socket
        let socket = match local_address {
            Some(a) => new_bound_shared_tcp_socket(a)?,
            None => new_unbound_tcp_socket(socket2::Domain::for_address(socket_addr))?,
        };

        // Non-blocking connect to remote address
        let ts = network_result_try!(nonblocking_connect(socket, socket_addr, timeout_ms)
            .await
            .folded()?);

        // See what local address we ended up with and turn this into a stream
        let actual_local_address = ts.local_addr()?;
        #[cfg(feature = "rt-tokio")]
        let ts = ts.compat();
        let ps = AsyncPeekStream::new(ts);

        // Wrap the stream in a network connection and return it
        let conn = ProtocolNetworkConnection::RawTcp(RawTcpNetworkConnection::new(
            ConnectionDescriptor::new(
                PeerAddress::new(
                    SocketAddress::from_socket_addr(socket_addr),
                    ProtocolType::TCP,
                ),
                SocketAddress::from_socket_addr(actual_local_address),
            ),
            ps,
        ));

        Ok(NetworkResult::Value(conn))
    }
}

impl ProtocolAcceptHandler for RawTcpProtocolHandler {
    fn on_accept(
        &self,
        stream: AsyncPeekStream,
        peer_addr: SocketAddr,
        local_addr: SocketAddr,
    ) -> SendPinBoxFuture<io::Result<Option<ProtocolNetworkConnection>>> {
        Box::pin(self.clone().on_accept_async(stream, peer_addr, local_addr))
    }
}
