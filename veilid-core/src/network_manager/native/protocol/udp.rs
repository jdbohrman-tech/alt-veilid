use super::*;
use sockets::*;

#[derive(Clone)]
pub(in crate::network_manager) struct RawUdpProtocolHandler {
    socket: Arc<UdpSocket>,
    assembly_buffer: AssemblyBuffer,
    address_filter: Option<AddressFilter>,
}

impl RawUdpProtocolHandler {
    pub fn new(socket: Arc<UdpSocket>, address_filter: Option<AddressFilter>) -> Self {
        Self {
            socket,
            assembly_buffer: AssemblyBuffer::new(),
            address_filter,
        }
    }

    #[cfg_attr(feature="verbose-tracing", instrument(level = "trace", err, skip(self, data), fields(data.len = data.len(), ret.len, ret.descriptor)))]
    pub async fn recv_message(&self, data: &mut [u8]) -> io::Result<(usize, ConnectionDescriptor)> {
        let (message_len, descriptor) = loop {
            // Get a packet
            let (size, remote_addr) = network_result_value_or_log!(self.socket.recv_from(data).await.into_network_result()? => continue);

            // Check to see if it is punished
            if let Some(af) = self.address_filter.as_ref() {
                if af.is_ip_addr_punished(remote_addr.ip()) {
                    continue;
                }
            }

            // Insert into assembly buffer
            let message = match self
                .assembly_buffer
                .insert_frame(&data[0..size], remote_addr)
            {
                NetworkResult::Value(Some(v)) => v,
                NetworkResult::Value(None) => {
                    continue;
                }
                #[cfg(feature = "network-result-extra")]
                nres => {
                    log_network_result!(
                        "UDP::recv_message insert_frame failed: {:?} <= size={} remote_addr={}",
                        nres,
                        size,
                        remote_addr
                    );
                    continue;
                }
                #[cfg(not(feature = "network-result-extra"))]
                _ => {
                    continue;
                }
            };

            // Check length of reassembled message (same for all protocols)
            if message.len() > MAX_MESSAGE_SIZE {
                log_net!(debug "{}({}) at {}@{}:{}", "Invalid message", "received too large UDP message", file!(), line!(), column!());
                continue;
            }

            // Copy assemble message out if we got one
            data[0..message.len()].copy_from_slice(&message);

            // Return a connection descriptor and the amount of data in the message
            let peer_addr = PeerAddress::new(
                SocketAddress::from_socket_addr(remote_addr),
                ProtocolType::UDP,
            );
            let local_socket_addr = self.socket.local_addr()?;
            let descriptor = ConnectionDescriptor::new(
                peer_addr,
                SocketAddress::from_socket_addr(local_socket_addr),
            );

            break (message.len(), descriptor);
        };

        #[cfg(feature = "verbose-tracing")]
        tracing::Span::current().record("ret.len", message_len);
        #[cfg(feature = "verbose-tracing")]
        tracing::Span::current().record("ret.descriptor", format!("{:?}", descriptor).as_str());
        Ok((message_len, descriptor))
    }

    #[cfg_attr(feature="verbose-tracing", instrument(level = "trace", err, skip(self, data), fields(data.len = data.len(), ret.descriptor)))]
    pub async fn send_message(
        &self,
        data: Vec<u8>,
        remote_addr: SocketAddr,
    ) -> io::Result<NetworkResult<ConnectionDescriptor>> {
        if data.len() > MAX_MESSAGE_SIZE {
            bail_io_error_other!("sending too large UDP message");
        }

        // Check to see if it is punished
        if let Some(af) = self.address_filter.as_ref() {
            if af.is_ip_addr_punished(remote_addr.ip()) {
                return Ok(NetworkResult::no_connection_other("punished"));
            }
        }

        // Fragment and send
        let sender = |framed_chunk: Vec<u8>, remote_addr: SocketAddr| async move {
            let len = network_result_try!(self
                .socket
                .send_to(&framed_chunk, remote_addr)
                .await
                .into_network_result()?);
            if len != framed_chunk.len() {
                bail_io_error_other!("UDP partial send")
            }
            Ok(NetworkResult::value(()))
        };

        network_result_try!(
            self.assembly_buffer
                .split_message(data, remote_addr, sender)
                .await?
        );

        // Return a connection descriptor for the sent message
        let peer_addr = PeerAddress::new(
            SocketAddress::from_socket_addr(remote_addr),
            ProtocolType::UDP,
        );
        let local_socket_addr = self.socket.local_addr()?;

        let descriptor = ConnectionDescriptor::new(
            peer_addr,
            SocketAddress::from_socket_addr(local_socket_addr),
        );

        #[cfg(feature = "verbose-tracing")]
        tracing::Span::current().record("ret.descriptor", format!("{:?}", descriptor).as_str());
        Ok(NetworkResult::value(descriptor))
    }

    #[instrument(level = "trace", err)]
    pub async fn new_unspecified_bound_handler(
        socket_addr: &SocketAddr,
    ) -> io::Result<RawUdpProtocolHandler> {
        // get local wildcard address for bind
        let local_socket_addr = compatible_unspecified_socket_addr(socket_addr);
        let socket = UdpSocket::bind(local_socket_addr).await?;
        Ok(RawUdpProtocolHandler::new(Arc::new(socket), None))
    }
}
