use super::*;

#[derive(Clone)]
pub struct RawUdpProtocolHandler {
    registry: VeilidComponentRegistry,
    socket: Arc<UdpSocket>,
    assembly_buffer: AssemblyBuffer,
}

impl_veilid_component_registry_accessor!(RawUdpProtocolHandler);

impl RawUdpProtocolHandler {
    pub fn new(registry: VeilidComponentRegistry, socket: Arc<UdpSocket>) -> Self {
        Self {
            registry,
            socket,
            assembly_buffer: AssemblyBuffer::new(),
        }
    }

    #[instrument(level = "trace", target = "protocol", err, skip(self, data), fields(data.len = data.len(), ret.len, ret.flow))]
    pub async fn recv_message(&self, data: &mut [u8]) -> io::Result<(usize, Flow)> {
        let (message_len, flow) = loop {
            // Get a packet
            let (size, remote_addr) = network_result_value_or_log!(self.socket.recv_from(data).await.into_network_result()? => continue);

            // Check to see if it is punished
            if self
                .network_manager()
                .address_filter()
                .is_ip_addr_punished(remote_addr.ip())
            {
                continue;
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
                nres => {
                    log_network_result!(debug
                        "UDP::recv_message insert_frame failed: {:?} <= size={} remote_addr={}",
                        nres,
                        size,
                        remote_addr
                    );
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

            // Return a flow and the amount of data in the message
            let peer_addr = PeerAddress::new(
                SocketAddress::from_socket_addr(remote_addr),
                ProtocolType::UDP,
            );
            let local_socket_addr = self.socket.local_addr()?;
            let flow = Flow::new(
                peer_addr,
                SocketAddress::from_socket_addr(local_socket_addr),
            );

            break (message.len(), flow);
        };

        #[cfg(feature = "verbose-tracing")]
        tracing::Span::current().record("ret.len", message_len);
        #[cfg(feature = "verbose-tracing")]
        tracing::Span::current().record("ret.flow", format!("{:?}", flow).as_str());
        Ok((message_len, flow))
    }

    #[instrument(level = "trace", target = "protocol", err, skip(self, data), fields(data.len = data.len(), ret.flow))]
    pub async fn send_message(
        &self,
        data: Vec<u8>,
        remote_addr: SocketAddr,
    ) -> io::Result<NetworkResult<Flow>> {
        if data.len() > MAX_MESSAGE_SIZE {
            bail_io_error_other!("sending too large UDP message");
        }

        // Check to see if it is punished
        if self
            .network_manager()
            .address_filter()
            .is_ip_addr_punished(remote_addr.ip())
        {
            return Ok(NetworkResult::no_connection_other("punished"));
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

        // Return a flow for the sent message
        let peer_addr = PeerAddress::new(
            SocketAddress::from_socket_addr(remote_addr),
            ProtocolType::UDP,
        );
        let local_socket_addr = self.socket.local_addr()?;

        let flow = Flow::new(
            peer_addr,
            SocketAddress::from_socket_addr(local_socket_addr),
        );

        log_net!("udp::send_message: {:?}", flow);

        #[cfg(feature = "verbose-tracing")]
        tracing::Span::current().record("ret.flow", format!("{:?}", flow).as_str());
        Ok(NetworkResult::value(flow))
    }

    #[instrument(level = "trace", target = "protocol", err, skip(self), fields(ret.flow))]
    pub async fn send_hole_punch(
        &self,
        remote_addr: SocketAddr,
        ttl: u32,
    ) -> io::Result<NetworkResult<Flow>> {
        // Check to see if it is punished
        if self
            .network_manager()
            .address_filter()
            .is_ip_addr_punished(remote_addr.ip())
        {
            return Ok(NetworkResult::no_connection_other("punished"));
        }

        // Get synchronous socket
        let res = socket2_operation(self.socket.as_ref(), |s| {
            // Get original TTL
            let original_ttl = s.ttl()?;

            // Set TTL
            s.set_ttl(ttl)?;

            // Send zero length packet
            let res = s.send_to(&[], &remote_addr.into());

            // Restore TTL immediately
            s.set_ttl(original_ttl)?;

            res
        });

        // Check for errors
        let len = network_result_try!(res.into_network_result()?);
        if len != 0 {
            bail_io_error_other!("wrong size send");
        }

        // Return a flow for the sent message
        let peer_addr = PeerAddress::new(
            SocketAddress::from_socket_addr(remote_addr),
            ProtocolType::UDP,
        );
        let local_socket_addr = self.socket.local_addr()?;

        let flow = Flow::new(
            peer_addr,
            SocketAddress::from_socket_addr(local_socket_addr),
        );

        log_net!("udp::send_hole_punch: {:?}", flow);

        #[cfg(feature = "verbose-tracing")]
        tracing::Span::current().record("ret.flow", format!("{:?}", flow).as_str());
        Ok(NetworkResult::value(flow))
    }

    #[instrument(level = "trace", target = "protocol", err)]
    pub async fn new_unspecified_bound_handler(
        registry: VeilidComponentRegistry,
        socket_addr: &SocketAddr,
    ) -> io::Result<RawUdpProtocolHandler> {
        // get local wildcard address for bind
        let local_socket_addr = compatible_unspecified_socket_addr(socket_addr);
        let socket = bind_async_udp_socket(local_socket_addr)?
            .ok_or(io::Error::from(io::ErrorKind::AddrInUse))?;
        Ok(RawUdpProtocolHandler::new(registry, Arc::new(socket)))
    }
}
