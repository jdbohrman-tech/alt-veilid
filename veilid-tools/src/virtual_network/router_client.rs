use super::*;
use core::sync::atomic::AtomicU64;
use futures_codec::{Bytes, BytesCodec, FramedRead, FramedWrite};
use futures_util::{
    stream::FuturesUnordered, AsyncReadExt, AsyncWriteExt, StreamExt, TryStreamExt,
};
use postcard::{from_bytes, to_stdvec};
use router_op_table::*;
use std::io;
use stop_token::future::FutureExt as _;

struct RouterClientInner {
    jh_handler: Option<MustJoinHandle<()>>,
    stop_source: Option<StopSource>,
}

impl fmt::Debug for RouterClientInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RouterClientInner")
            .field("jh_handler", &self.jh_handler)
            .field("stop_source", &self.stop_source)
            .finish()
    }
}

struct RouterClientUnlockedInner {
    sender: flume::Sender<ServerProcessorCommand>,
    next_message_id: AtomicU64,
    router_op_waiter: RouterOpWaiter<ServerProcessorReplyStatus, ()>,
}

impl fmt::Debug for RouterClientUnlockedInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RouterClientUnlockedInner")
            .field("sender", &self.sender)
            .field("next_message_id", &self.next_message_id)
            .field("router_op_waiter", &self.router_op_waiter)
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct RouterClient {
    unlocked_inner: Arc<RouterClientUnlockedInner>,
    inner: Arc<Mutex<RouterClientInner>>,
}

impl RouterClient {
    //////////////////////////////////////////////////////////////////////////
    // Public interface

    #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
    pub async fn router_connect_tcp<H: ToSocketAddrs>(host: H) -> io::Result<RouterClient> {
        let addrs = host.to_socket_addrs()?.collect::<Vec<_>>();

        // Connect to RouterServer
        let ts_reader;
        let ts_writer;
        cfg_if! {
            if #[cfg(feature="rt-tokio")] {
                let ts = ::tokio::net::TcpStream::connect(addrs.as_slice()).await?;
                let (reader, writer) = ts.into_split();
                use tokio_util::compat::{TokioAsyncReadCompatExt, TokioAsyncWriteCompatExt};
                ts_reader = reader.compat();
                ts_writer = writer.compat_write();
            } else if #[cfg(feature="rt-async-std")] {
                use futures_util::io::AsyncReadExt;
                let ts = ::async_std::net::TcpStream::connect(addrs.as_slice()).await?;
                (ts_reader, ts_writer) = ts.split();
            } else {
                compile_error!("must choose an executor");
            }
        }

        // Create channels
        let (client_sender, server_receiver) = flume::unbounded::<ServerProcessorCommand>();

        // Create stopper
        let stop_source = StopSource::new();

        // Create router operation waiter
        let router_op_waiter = RouterOpWaiter::new();

        // Spawn a client connection handler
        let jh_handler = spawn(
            "RouterClient server processor",
            Self::run_server_processor(
                ts_reader,
                ts_writer,
                server_receiver,
                router_op_waiter.clone(),
                stop_source.token(),
            ),
        );

        Ok(Self::new(
            client_sender,
            router_op_waiter,
            jh_handler,
            stop_source,
        ))
    }

    #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
    pub async fn router_connect_ws<R: AsRef<str>>(request: R) -> io::Result<RouterClient> {
        let request = request.as_ref();

        // Connect to RouterServer
        let wsio_reader;
        let wsio_writer;
        cfg_if! {
            if #[cfg(feature="rt-wasm-bindgen")] {
                use ws_stream_wasm::*;
                let (_wsmeta, wsio) = WsMeta::connect(request, None)
                    .await.map_err(ws_err_to_io_error)?;
                use futures_util::io::AsyncReadExt;
                (wsio_reader, wsio_writer) = wsio.into_io().split();
            } else {
                compile_error!("must choose an executor");
            }
        }

        // Create channels
        let (client_sender, server_receiver) = flume::unbounded::<Bytes>();

        // Create stopper
        let stop_source = StopSource::new();

        // Create router operation waiter
        let router_op_waiter = RouterOpWaiter::new();

        // Spawn a client connection handler
        let jh_handler = spawn(
            "RouterClient server processor",
            Self::run_server_processor(
                wsio_reader,
                wsio_writer,
                server_receiver,
                router_op_waiter.clone(),
                stop_source.token(),
            ),
        );

        Ok(Self::new(
            client_sender,
            router_op_waiter,
            jh_handler,
            stop_source,
        ))
    }

    pub(super) fn local_router_client(
        client_sender: flume::Sender<ServerProcessorCommand>,
        server_receiver: flume::Receiver<ServerProcessorEvent>,
    ) -> RouterClient {
        // Create stopper
        let stop_source = StopSource::new();

        // Create router operation waiter
        let router_op_waiter = RouterOpWaiter::new();

        // Spawn a client connection handler
        let jh_handler = spawn(
            "RouterClient local processor",
            Self::run_local_processor(
                server_receiver,
                router_op_waiter.clone(),
                stop_source.token(),
            ),
        );

        Self::new(client_sender, router_op_waiter, jh_handler, stop_source)
    }

    pub async fn disconnect(self) {
        drop(self.inner.lock().stop_source.take());
        let jh_handler = self.inner.lock().jh_handler.take();
        if let Some(jh_handler) = jh_handler {
            jh_handler.await;
        }
    }

    pub async fn allocate_machine(self, profile: String) -> VirtualNetworkResult<MachineId> {
        let request = ServerProcessorRequest::AllocateMachine { profile };
        let ServerProcessorReplyValue::AllocateMachine { machine_id } =
            self.perform_request(request).await?
        else {
            return Err(VirtualNetworkError::ResponseMismatch);
        };
        Ok(machine_id)
    }

    pub async fn release_machine(self, machine_id: MachineId) -> VirtualNetworkResult<()> {
        let request = ServerProcessorRequest::ReleaseMachine { machine_id };
        let ServerProcessorReplyValue::ReleaseMachine = self.perform_request(request).await? else {
            return Err(VirtualNetworkError::ResponseMismatch);
        };
        Ok(())
    }

    pub async fn get_interfaces(
        self,
        machine_id: MachineId,
    ) -> VirtualNetworkResult<BTreeMap<String, NetworkInterface>> {
        let request = ServerProcessorRequest::GetInterfaces { machine_id };
        let ServerProcessorReplyValue::GetInterfaces { interfaces } =
            self.perform_request(request).await?
        else {
            return Err(VirtualNetworkError::ResponseMismatch);
        };
        Ok(interfaces)
    }

    pub async fn tcp_connect(
        self,
        machine_id: MachineId,
        remote_address: SocketAddr,
        opt_local_address: Option<SocketAddr>,
        timeout_ms: u32,
        options: VirtualTcpOptions,
    ) -> VirtualNetworkResult<(SocketId, SocketAddr)> {
        let request = ServerProcessorRequest::TcpConnect {
            machine_id,
            local_address: opt_local_address,
            remote_address,
            timeout_ms,
            options,
        };
        let ServerProcessorReplyValue::TcpConnect {
            socket_id,
            local_address,
        } = self.perform_request(request).await?
        else {
            return Err(VirtualNetworkError::ResponseMismatch);
        };
        Ok((socket_id, local_address))
    }

    pub async fn tcp_bind(
        self,
        machine_id: MachineId,
        opt_local_address: Option<SocketAddr>,
        options: VirtualTcpOptions,
    ) -> VirtualNetworkResult<(SocketId, SocketAddr)> {
        let request = ServerProcessorRequest::TcpBind {
            machine_id,
            local_address: opt_local_address,
            options,
        };
        let ServerProcessorReplyValue::TcpBind {
            socket_id,
            local_address,
        } = self.perform_request(request).await?
        else {
            return Err(VirtualNetworkError::ResponseMismatch);
        };
        Ok((socket_id, local_address))
    }

    pub async fn tcp_accept(
        self,
        machine_id: MachineId,
        listen_socket_id: SocketId,
    ) -> VirtualNetworkResult<(SocketId, SocketAddr)> {
        let request = ServerProcessorRequest::TcpAccept {
            machine_id,
            listen_socket_id,
        };
        let ServerProcessorReplyValue::TcpAccept { socket_id, address } =
            self.perform_request(request).await?
        else {
            return Err(VirtualNetworkError::ResponseMismatch);
        };
        Ok((socket_id, address))
    }

    pub async fn tcp_shutdown(
        self,
        machine_id: MachineId,
        socket_id: SocketId,
    ) -> VirtualNetworkResult<()> {
        let request = ServerProcessorRequest::TcpShutdown {
            machine_id,
            socket_id,
        };
        let ServerProcessorReplyValue::TcpShutdown = self.perform_request(request).await? else {
            return Err(VirtualNetworkError::ResponseMismatch);
        };
        Ok(())
    }

    pub async fn udp_bind(
        self,
        machine_id: MachineId,
        opt_local_address: Option<SocketAddr>,
        options: VirtualUdpOptions,
    ) -> VirtualNetworkResult<(SocketId, SocketAddr)> {
        let request = ServerProcessorRequest::UdpBind {
            machine_id,
            local_address: opt_local_address,
            options,
        };
        let ServerProcessorReplyValue::UdpBind {
            socket_id,
            local_address,
        } = self.perform_request(request).await?
        else {
            return Err(VirtualNetworkError::ResponseMismatch);
        };
        Ok((socket_id, local_address))
    }

    pub async fn send(
        self,
        machine_id: MachineId,
        socket_id: SocketId,
        data: Vec<u8>,
    ) -> VirtualNetworkResult<usize> {
        let request = ServerProcessorRequest::Send {
            machine_id,
            socket_id,
            data,
        };
        let ServerProcessorReplyValue::Send { len } = self.perform_request(request).await? else {
            return Err(VirtualNetworkError::ResponseMismatch);
        };
        Ok(len as usize)
    }

    pub async fn send_to(
        self,
        machine_id: MachineId,
        socket_id: SocketId,
        remote_address: SocketAddr,
        data: Vec<u8>,
    ) -> VirtualNetworkResult<usize> {
        let request = ServerProcessorRequest::SendTo {
            machine_id,
            socket_id,
            data,
            remote_address,
        };
        let ServerProcessorReplyValue::SendTo { len } = self.perform_request(request).await? else {
            return Err(VirtualNetworkError::ResponseMismatch);
        };
        Ok(len as usize)
    }

    pub async fn recv(
        self,
        machine_id: MachineId,
        socket_id: SocketId,
        len: usize,
    ) -> VirtualNetworkResult<Vec<u8>> {
        let request = ServerProcessorRequest::Recv {
            machine_id,
            socket_id,
            len: len as u32,
        };
        let ServerProcessorReplyValue::Recv { data } = self.perform_request(request).await? else {
            return Err(VirtualNetworkError::ResponseMismatch);
        };
        Ok(data)
    }

    pub async fn recv_from(
        self,
        machine_id: MachineId,
        socket_id: SocketId,
        len: usize,
    ) -> VirtualNetworkResult<(Vec<u8>, SocketAddr)> {
        let request = ServerProcessorRequest::RecvFrom {
            machine_id,
            socket_id,
            len: len as u32,
        };
        let ServerProcessorReplyValue::RecvFrom {
            data,
            remote_address,
        } = self.perform_request(request).await?
        else {
            return Err(VirtualNetworkError::ResponseMismatch);
        };
        Ok((data, remote_address))
    }

    pub async fn get_routed_local_address(
        self,
        machine_id: MachineId,
        address_type: VirtualAddressType,
    ) -> VirtualNetworkResult<IpAddr> {
        let request = ServerProcessorRequest::GetRoutedLocalAddress {
            machine_id,
            address_type,
        };
        let ServerProcessorReplyValue::GetRoutedLocalAddress { address } =
            self.perform_request(request).await?
        else {
            return Err(VirtualNetworkError::ResponseMismatch);
        };
        Ok(address)
    }

    pub async fn find_gateway(
        self,
        machine_id: MachineId,
    ) -> VirtualNetworkResult<Option<GatewayId>> {
        let request = ServerProcessorRequest::FindGateway { machine_id };
        let ServerProcessorReplyValue::FindGateway { opt_gateway_id } =
            self.perform_request(request).await?
        else {
            return Err(VirtualNetworkError::ResponseMismatch);
        };
        Ok(opt_gateway_id)
    }

    pub async fn get_external_address(self, gateway_id: GatewayId) -> VirtualNetworkResult<IpAddr> {
        let request = ServerProcessorRequest::GetExternalAddress { gateway_id };
        let ServerProcessorReplyValue::GetExternalAddress { address } =
            self.perform_request(request).await?
        else {
            return Err(VirtualNetworkError::ResponseMismatch);
        };
        Ok(address)
    }

    pub async fn add_port(
        self,
        gateway_id: GatewayId,
        protocol: VirtualProtocolType,
        external_port: Option<u16>,
        local_address: SocketAddr,
        lease_duration_ms: u32,
        description: String,
    ) -> VirtualNetworkResult<u16> {
        let request = ServerProcessorRequest::AddPort {
            gateway_id,
            protocol,
            external_port,
            local_address,
            lease_duration_ms,
            description,
        };
        let ServerProcessorReplyValue::AddPort { external_port } =
            self.perform_request(request).await?
        else {
            return Err(VirtualNetworkError::ResponseMismatch);
        };
        Ok(external_port)
    }

    pub async fn remove_port(
        self,
        gateway_id: GatewayId,
        protocol: VirtualProtocolType,
        external_port: u16,
    ) -> VirtualNetworkResult<()> {
        let request = ServerProcessorRequest::RemovePort {
            gateway_id,
            protocol,
            external_port,
        };
        let ServerProcessorReplyValue::RemovePort = self.perform_request(request).await? else {
            return Err(VirtualNetworkError::ResponseMismatch);
        };
        Ok(())
    }

    pub async fn txt_query(self, name: String) -> VirtualNetworkResult<Vec<String>> {
        let request = ServerProcessorRequest::TXTQuery { name };
        let ServerProcessorReplyValue::TXTQuery { result } = self.perform_request(request).await?
        else {
            return Err(VirtualNetworkError::ResponseMismatch);
        };
        Ok(result)
    }

    //////////////////////////////////////////////////////////////////////////
    // Private implementation

    fn new(
        sender: flume::Sender<ServerProcessorCommand>,
        router_op_waiter: RouterOpWaiter<ServerProcessorReplyStatus, ()>,
        jh_handler: MustJoinHandle<()>,
        stop_source: StopSource,
    ) -> RouterClient {
        RouterClient {
            unlocked_inner: Arc::new(RouterClientUnlockedInner {
                sender,
                next_message_id: AtomicU64::new(0),
                router_op_waiter,
            }),
            inner: Arc::new(Mutex::new(RouterClientInner {
                jh_handler: Some(jh_handler),
                stop_source: Some(stop_source),
            })),
        }
    }

    fn report_closed_socket(&self, machine_id: MachineId, socket_id: SocketId) {
        let command = ServerProcessorCommand::CloseSocket {
            machine_id,
            socket_id,
        };

        if let Err(e) = self
            .unlocked_inner
            .sender
            .send(command)
            .map_err(|_| VirtualNetworkError::IoError(io::ErrorKind::BrokenPipe))
        {
            error!("{}", e);
        }
    }

    pub(super) fn drop_tcp_stream(&self, machine_id: MachineId, socket_id: SocketId) {
        self.report_closed_socket(machine_id, socket_id);
    }

    pub(super) fn drop_tcp_listener(&self, machine_id: MachineId, socket_id: SocketId) {
        self.report_closed_socket(machine_id, socket_id);
    }

    pub(super) fn drop_udp_socket(&self, machine_id: MachineId, socket_id: SocketId) {
        self.report_closed_socket(machine_id, socket_id);
    }

    async fn perform_request(
        &self,
        request: ServerProcessorRequest,
    ) -> VirtualNetworkResult<ServerProcessorReplyValue> {
        let message_id = MessageId(
            self.unlocked_inner
                .next_message_id
                .fetch_add(1, Ordering::AcqRel),
        );
        let command = ServerProcessorCommand::Message(ServerProcessorMessage {
            message_id,
            request,
        });

        self.unlocked_inner
            .sender
            .send_async(command)
            .await
            .map_err(|_| VirtualNetworkError::IoError(io::ErrorKind::BrokenPipe))?;
        let handle = self
            .unlocked_inner
            .router_op_waiter
            .add_op_waiter(message_id.0, ());

        let status = self
            .unlocked_inner
            .router_op_waiter
            .wait_for_op(handle)
            .await
            .map_err(|_| VirtualNetworkError::WaitError)?;

        match status {
            ServerProcessorReplyStatus::Value(server_processor_response) => {
                Ok(server_processor_response)
            }
            ServerProcessorReplyStatus::InvalidMachineId => {
                Err(VirtualNetworkError::InvalidMachineId)
            }
            ServerProcessorReplyStatus::InvalidSocketId => {
                Err(VirtualNetworkError::InvalidSocketId)
            }
            ServerProcessorReplyStatus::MissingProfile => Err(VirtualNetworkError::MissingProfile),
            ServerProcessorReplyStatus::ProfileComplete => {
                Err(VirtualNetworkError::ProfileComplete)
            }
            ServerProcessorReplyStatus::IoError(k) => Err(VirtualNetworkError::IoError(k)),
        }
    }

    async fn run_server_processor<R, W>(
        reader: R,
        writer: W,
        receiver: flume::Receiver<ServerProcessorCommand>,
        router_op_waiter: RouterOpWaiter<ServerProcessorReplyStatus, ()>,
        stop_token: StopToken,
    ) where
        R: AsyncReadExt + Unpin + Send,
        W: AsyncWriteExt + Unpin + Send,
    {
        let mut unord = FuturesUnordered::new();

        let framed_reader = FramedRead::new(reader, BytesCodec);
        let framed_writer = FramedWrite::new(writer, BytesCodec);

        let framed_writer_fut = Box::pin(async move {
            if let Err(e) = receiver
                .into_stream()
                .map(|command| {
                    to_stdvec(&command)
                        .map_err(io::Error::other)
                        .map(Bytes::from)
                })
                .forward(framed_writer)
                .await
            {
                error!("{}", e);
            }
        });
        let framed_reader_fut = Box::pin(async move {
            let fut = framed_reader.try_for_each(|x| async {
                let x = x;
                let evt = from_bytes::<ServerProcessorEvent>(&x)
                    .map_err(VirtualNetworkError::SerializationError)?;

                Self::process_event(evt, router_op_waiter.clone()).await
            });
            if let Err(e) = fut.await {
                error!("{}", e);
            }
        });

        unord.push(framed_writer_fut);
        unord.push(framed_reader_fut);
        while let Ok(Some(_)) = unord.next().timeout_at(stop_token.clone()).await {}
    }

    async fn run_local_processor(
        receiver: flume::Receiver<ServerProcessorEvent>,
        router_op_waiter: RouterOpWaiter<ServerProcessorReplyStatus, ()>,
        stop_token: StopToken,
    ) {
        let mut unord = FuturesUnordered::new();
        let receiver = receiver
            .into_stream()
            .map(io::Result::<ServerProcessorEvent>::Ok);

        let receiver_fut = Box::pin(async move {
            let fut =
                receiver.try_for_each(|evt| Self::process_event(evt, router_op_waiter.clone()));
            if let Err(e) = fut.await {
                error!("{}", e);
            }
        });
        unord.push(receiver_fut);
        while let Ok(Some(_)) = unord.next().timeout_at(stop_token.clone()).await {}
    }

    async fn process_event(
        evt: ServerProcessorEvent,
        router_op_waiter: RouterOpWaiter<ServerProcessorReplyStatus, ()>,
    ) -> io::Result<()> {
        match evt {
            ServerProcessorEvent::Reply(reply) => {
                router_op_waiter
                    .complete_op_waiter(reply.message_id.0, reply.status)
                    .map_err(io::Error::other)?;
            } // ServerProcessorEvent::DeadSocket {
              //     machine_id,
              //     socket_id,
              // } => {
              //     //
              // }
        }

        Ok(())
    }
}
