use super::*;
use futures_util::{FutureExt, StreamExt};
use std::{io, sync::Arc};
use stop_token::prelude::*;

cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        // No accept support for WASM
    } else {

        ///////////////////////////////////////////////////////////
        // Accept

        pub trait ProtocolAcceptHandler: ProtocolAcceptHandlerClone + Send + Sync {
            fn on_accept(
                &self,
                stream: AsyncPeekStream,
                peer_addr: SocketAddr,
                local_addr: SocketAddr,
            ) -> SendPinBoxFuture<io::Result<Option<ProtocolNetworkConnection>>>;
        }

        pub trait ProtocolAcceptHandlerClone {
            fn clone_box(&self) -> Box<dyn ProtocolAcceptHandler>;
        }

        impl<T> ProtocolAcceptHandlerClone for T
        where
            T: 'static + ProtocolAcceptHandler + Clone,
        {
            fn clone_box(&self) -> Box<dyn ProtocolAcceptHandler> {
                Box::new(self.clone())
            }
        }
        impl Clone for Box<dyn ProtocolAcceptHandler> {
            fn clone(&self) -> Box<dyn ProtocolAcceptHandler> {
                self.clone_box()
            }
        }

        pub type NewProtocolAcceptHandler =
            dyn Fn(VeilidConfig, bool) -> Box<dyn ProtocolAcceptHandler> + Send;
    }
}
///////////////////////////////////////////////////////////
// Dummy protocol network connection for testing

#[derive(Debug)]
pub struct DummyNetworkConnection {
    descriptor: ConnectionDescriptor,
}

impl DummyNetworkConnection {
    pub fn descriptor(&self) -> ConnectionDescriptor {
        self.descriptor.clone()
    }
    // pub fn close(&self) -> io::Result<()> {
    //     Ok(())
    // }
    pub fn send(&self, _message: Vec<u8>) -> io::Result<NetworkResult<()>> {
        Ok(NetworkResult::Value(()))
    }
    pub fn recv(&self) -> io::Result<NetworkResult<Vec<u8>>> {
        Ok(NetworkResult::Value(Vec::new()))
    }
}

///////////////////////////////////////////////////////////
// Top-level protocol independent network connection object

#[derive(Clone, Copy, Debug)]
enum RecvLoopAction {
    Send,
    Recv,
    Finish,
    Timeout,
}

#[derive(Debug, Clone)]
pub struct NetworkConnectionStats {
    last_message_sent_time: Option<Timestamp>,
    last_message_recv_time: Option<Timestamp>,
}


pub type NetworkConnectionId = AlignedU64;

#[derive(Debug)]
pub struct NetworkConnection {
    connection_id: NetworkConnectionId,
    descriptor: ConnectionDescriptor,
    processor: Option<MustJoinHandle<()>>,
    established_time: Timestamp,
    stats: Arc<Mutex<NetworkConnectionStats>>,
    sender: flume::Sender<(Option<Id>, Vec<u8>)>,
    stop_source: Option<StopSource>,
}

impl NetworkConnection {
    pub(super) fn dummy(id: NetworkConnectionId, descriptor: ConnectionDescriptor) -> Self {
        // Create handle for sending (dummy is immediately disconnected)
        let (sender, _receiver) = flume::bounded(get_concurrency() as usize);

        Self {
            connection_id: id,
            descriptor,
            processor: None,
            established_time: get_aligned_timestamp(),
            stats: Arc::new(Mutex::new(NetworkConnectionStats {
                last_message_sent_time: None,
                last_message_recv_time: None,
            })),
            sender,
            stop_source: None,
        }
    }

    pub(super) fn from_protocol(
        connection_manager: ConnectionManager,
        manager_stop_token: StopToken,
        protocol_connection: ProtocolNetworkConnection,
        connection_id: NetworkConnectionId,
    ) -> Self {
        // Get descriptor
        let descriptor = protocol_connection.descriptor();

        // Create handle for sending
        let (sender, receiver) = flume::bounded(get_concurrency() as usize);

        // Create stats
        let stats = Arc::new(Mutex::new(NetworkConnectionStats {
            last_message_sent_time: None,
            last_message_recv_time: None,
        }));

        let stop_source = StopSource::new();
        let local_stop_token = stop_source.token();

        // Spawn connection processor and pass in protocol connection
        let processor = spawn(Self::process_connection(
            connection_manager,
            local_stop_token,
            manager_stop_token,
            connection_id,
            descriptor.clone(),
            receiver,
            protocol_connection,
            stats.clone(),
        ));

        // Return the connection
        Self {
            connection_id,
            descriptor,
            processor: Some(processor),
            established_time: get_aligned_timestamp(),
            stats,
            sender,
            stop_source: Some(stop_source),
        }
    }

    pub fn connection_id(&self) -> NetworkConnectionId {
        self.connection_id
    }

    pub fn connection_descriptor(&self) -> ConnectionDescriptor {
        self.descriptor.clone()
    }

    pub fn get_handle(&self) -> ConnectionHandle {
        ConnectionHandle::new(self.connection_id, self.descriptor.clone(), self.sender.clone())
    }

    pub fn close(&mut self) {
        if let Some(stop_source) = self.stop_source.take() {
            // drop the stopper
            drop(stop_source);
        }
    }

    #[cfg_attr(feature="verbose-tracing", instrument(level="trace", skip(message, stats), fields(message.len = message.len()), ret))]
    async fn send_internal(
        protocol_connection: &ProtocolNetworkConnection,
        stats: Arc<Mutex<NetworkConnectionStats>>,
        message: Vec<u8>,
    ) -> io::Result<NetworkResult<()>> {
        let ts = get_aligned_timestamp();
        let out = network_result_try!(protocol_connection.send(message).await?);

        let mut stats = stats.lock();
        stats.last_message_sent_time.max_assign(Some(ts));

        Ok(NetworkResult::Value(out))
    }

    #[cfg_attr(feature="verbose-tracing", instrument(level="trace", skip(stats), fields(ret.len)))]
    async fn recv_internal(
        protocol_connection: &ProtocolNetworkConnection,
        stats: Arc<Mutex<NetworkConnectionStats>>,
    ) -> io::Result<NetworkResult<Vec<u8>>> {
        let ts = get_aligned_timestamp();
        let out = network_result_try!(protocol_connection.recv().await?);

        let mut stats = stats.lock();
        stats.last_message_recv_time.max_assign(Some(ts));

        #[cfg(feature = "verbose-tracing")]
        tracing::Span::current().record("ret.len", out.len());

        Ok(NetworkResult::Value(out))
    }

    #[allow(dead_code)]
    pub fn stats(&self) -> NetworkConnectionStats {
        let stats = self.stats.lock();
        stats.clone()
    }

    #[allow(dead_code)]
    pub fn established_time(&self) -> Timestamp {
        self.established_time
    }

    // Connection receiver loop
    fn process_connection(
        connection_manager: ConnectionManager,
        local_stop_token: StopToken,
        manager_stop_token: StopToken,
        connection_id: NetworkConnectionId,
        descriptor: ConnectionDescriptor,
        receiver: flume::Receiver<(Option<Id>, Vec<u8>)>,
        protocol_connection: ProtocolNetworkConnection,
        stats: Arc<Mutex<NetworkConnectionStats>>,
    ) -> SendPinBoxFuture<()> {
        Box::pin(async move {
            log_net!(
                "== Starting process_connection loop for id={}, {:?}", connection_id,
                descriptor
            );

            let network_manager = connection_manager.network_manager();
            let address_filter = network_manager.address_filter();
            let mut unord = FuturesUnordered::new();
            let mut need_receiver = true;
            let mut need_sender = true;

            // Push mutable timer so we can reset it
            // Normally we would use an io::timeout here, but WASM won't support that, so we use a mutable sleep future
            let new_timer = || {
                sleep(connection_manager.connection_inactivity_timeout_ms()).then(|_| async {
                    // timeout
                    log_net!("== Connection timeout on {:?}", descriptor);
                    RecvLoopAction::Timeout
                })
            };
            let timer = MutableFuture::new(new_timer());

            unord.push(system_boxed(timer.clone().instrument(Span::current())));

            loop {
                // Add another message sender future if necessary
                if need_sender {
                    need_sender = false;
                    let sender_fut = receiver.recv_async().then(|res| async {
                        match res {
                            Ok((_span_id, message)) => {

                                let recv_span = span!(Level::TRACE, "process_connection recv");
                                // xxx: causes crash (Missing otel data span extensions)
                                // recv_span.follows_from(span_id);
                    
                                // send the packet
                                if let Err(e) = Self::send_internal(
                                    &protocol_connection,
                                    stats.clone(),
                                    message,
                                ).instrument(recv_span)
                                .await
                                {
                                    // Sending the packet along can fail, if so, this connection is dead
                                    log_net!(debug e);
                                    RecvLoopAction::Finish
                                } else {
                                    RecvLoopAction::Send
                                }
                            }
                            Err(e) => {
                                // All senders gone, shouldn't happen since we store one alongside the join handle
                                log_net!(warn e);
                                RecvLoopAction::Finish
                            }
                        }
                    });
                    unord.push(system_boxed(sender_fut.instrument(Span::current())));
                }

                // Add another message receiver future if necessary
                if need_receiver {
                    need_receiver = false;
                    let receiver_fut = Self::recv_internal(&protocol_connection, stats.clone())
                        .then(|res| async {
                            match res {
                                Ok(v) => {
                                    let peer_address = protocol_connection.descriptor().remote();

                                    // Check to see if it is punished
                                    if address_filter.is_ip_addr_punished(peer_address.to_socket_addr().ip()) {
                                        return RecvLoopAction::Finish;
                                    }

                                    // Check for connection close
                                    if v.is_no_connection() {
                                        log_net!(debug "Connection closed from: {} ({})", peer_address.to_socket_addr(), peer_address.protocol_type());
                                        return RecvLoopAction::Finish;
                                    }

                                    // Punish invalid messages
                                    if v.is_invalid_message() {
                                        address_filter.punish_ip_addr(peer_address.to_socket_addr().ip());
                                        return RecvLoopAction::Finish;
                                    }

                                    // Log other network results
                                    let mut message = network_result_value_or_log!(v => [ format!(": protocol_connection={:?}", protocol_connection) ] {
                                        return RecvLoopAction::Finish;
                                    });

                                    // Pass received messages up to the network manager for processing
                                    if let Err(e) = network_manager
                                        .on_recv_envelope(message.as_mut_slice(), descriptor)
                                        .await
                                    {
                                        log_net!(debug "failed to process received envelope: {}", e);
                                        RecvLoopAction::Finish
                                    } else {
                                        RecvLoopAction::Recv
                                    }
                                }
                                Err(e) => {
                                    // Connection unable to receive, closed
                                    log_net!(error "connection unable to receive: {}", e);
                                    RecvLoopAction::Finish
                                }
                            }
                        });

                    unord.push(system_boxed(receiver_fut.instrument(Span::current())));
                }

                // Process futures
                match unord
                    .next()
                    .timeout_at(local_stop_token.clone())
                    .timeout_at(manager_stop_token.clone())
                    .await
                    .and_then(std::convert::identity)   // flatten stoptoken timeouts
                {
                    Ok(Some(RecvLoopAction::Send)) => {
                        // Don't reset inactivity timer if we're only sending
                        need_sender = true;
                    }
                    Ok(Some(RecvLoopAction::Recv)) => {
                        // Reset inactivity timer since we got something from this connection
                        timer.set(new_timer());

                        need_receiver = true;
                    }
                    Ok(Some(RecvLoopAction::Finish) | Some(RecvLoopAction::Timeout)) => {
                        break;
                    }
                    Ok(None) => {
                        // Should not happen
                        unreachable!();
                    }
                    Err(_) => {
                        // Either one of the stop tokens
                        break;
                    }
                }
            }

            log_net!(
                "== Connection loop finished descriptor={:?}",
                descriptor
            );

            // Let the connection manager know the receive loop exited
            connection_manager
                .report_connection_finished(connection_id)
                .await;
        }.instrument(trace_span!("process_connection")))
    }
}

// Resolves ready when the connection loop has terminated
impl Future for NetworkConnection {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        let mut pending = 0usize;

        // Process all sub-futures, nulling them out when they return ready
        if let Some(mut processor) = self.processor.as_mut() {
            if Pin::new(&mut processor).poll(cx).is_ready() {
                self.processor = None;
            } else {
                pending += 1
            }
        }

        // Any sub-futures pending?
        if pending > 0 {
            task::Poll::Pending
        } else {
            task::Poll::Ready(())
        }
    }
}
