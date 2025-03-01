use super::*;

impl_veilid_log_facility!("net");

#[derive(Clone)]
pub struct RawUdpProtocolHandler {
    registry: VeilidComponentRegistry,
    socket: Arc<UdpSocket>,
    assembly_buffer: AssemblyBuffer,
    is_ipv6: bool,
    default_ttl: u32,
    current_ttl: Arc<AsyncMutex<u32>>,
}

impl_veilid_component_registry_accessor!(RawUdpProtocolHandler);

impl RawUdpProtocolHandler {
    pub fn new(registry: VeilidComponentRegistry, socket: Arc<UdpSocket>, is_ipv6: bool) -> Self {
        // Get original TTL
        let default_ttl = if is_ipv6 {
            socket2_operation(socket.as_ref(), |s| s.unicast_hops_v6())
                .expect("getting IPV6_UNICAST_HOPS should not fail")
        } else {
            socket2_operation(socket.as_ref(), |s| s.ttl()).expect("getting IP_TTL should not fail")
        };

        Self {
            registry,
            socket,
            assembly_buffer: AssemblyBuffer::new(),
            is_ipv6,
            default_ttl,
            current_ttl: Arc::new(AsyncMutex::new(default_ttl)),
        }
    }

    #[instrument(level = "trace", target = "protocol", err, skip(self, data), fields(data.len = data.len(), ret.len, ret.flow))]
    pub async fn recv_message(&self, data: &mut [u8]) -> io::Result<(usize, Flow)> {
        let (message_len, flow) = loop {
            // Get a packet
            let (size, remote_addr) = network_result_value_or_log!(self self.socket.recv_from(data).await.into_network_result()? => continue);

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
                    veilid_log!(self debug target:"network_result",
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
                veilid_log!(self debug "{}({}) at {}@{}:{}", "Invalid message", "received too large UDP message", file!(), line!(), column!());
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

        // Ensure the TTL for sent packets is the default,
        // then fragment and send the packets
        {
            let current_ttl = self.current_ttl.lock().await;
            if *current_ttl != self.default_ttl {
                veilid_log!(self error "Incorrect TTL on sent UDP packet ({} != {}): len={}, remote_addr={:?}", *current_ttl, self.default_ttl, data.len(), remote_addr);
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

                veilid_log!(self trace "udp::send_message:chunk(len={}) {:?}", len, remote_addr);
                Ok(NetworkResult::value(()))
            };

            network_result_try!(
                self.assembly_buffer
                    .split_message(data, remote_addr, sender)
                    .await?
            );
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

        veilid_log!(self trace "udp::send_message: {:?}", flow);

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

        // Ensure the TTL for sent packets is the default,
        // then fragment and send the packets
        let res = {
            let mut current_ttl = self.current_ttl.lock().await;
            if *current_ttl != self.default_ttl {
                veilid_log!(self error "Incorrect TTL before sending holepunch UDP packet ({} != {}): remote_addr={:?}", *current_ttl, self.default_ttl, remote_addr);
            }

            // Get synchronous socket
            socket2_operation(self.socket.as_ref(), |s| {
                // Set TTL
                let ttl_res = if self.is_ipv6 {
                    s.set_unicast_hops_v6(ttl)
                } else {
                    s.set_ttl(ttl)
                };
                ttl_res.inspect_err(|e| {
                    veilid_log!(self error "Failed to set TTL on holepunch UDP socket: {} remote_addr={:?}", e, remote_addr);
                })?;
                *current_ttl = ttl;

                // Send zero length packet
                let res = s.send_to(&[], &remote_addr.into());

                // Restore TTL immediately
                let ttl_res = if self.is_ipv6 {
                    s.set_unicast_hops_v6(self.default_ttl)
                } else {
                    s.set_ttl(self.default_ttl)
                };
                ttl_res.inspect_err(|e| {
                    veilid_log!(self error "Failed to reset TTL on holepunch UDP socket: {} remote_addr={:?}", e, remote_addr);
                })?;
                *current_ttl = self.default_ttl;

                res
            })
        };

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

        veilid_log!(self trace "udp::send_hole_punch: {:?}", flow);

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
        Ok(RawUdpProtocolHandler::new(
            registry,
            Arc::new(socket),
            local_socket_addr.is_ipv6(),
        ))
    }
}
