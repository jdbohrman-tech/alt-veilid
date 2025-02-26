use super::*;

struct ServerProcessorCommandRecord {
    cmd: ServerProcessorCommand,
    outbound_sender: flume::Sender<ServerProcessorEvent>,
}

#[derive(Debug)]
struct ServerProcessorInner {
    //
}

#[derive(Debug)]
struct ServerProcessorUnlockedInner {
    machine_registry: GlobalStateManager,
    receiver: flume::Receiver<ServerProcessorCommandRecord>,
    sender: flume::Sender<ServerProcessorCommandRecord>,
}

#[derive(Debug, Clone)]
pub struct ServerProcessor {
    unlocked_inner: Arc<ServerProcessorUnlockedInner>,
    _inner: Arc<Mutex<ServerProcessorInner>>,
}

impl ServerProcessor {
    ////////////////////////////////////////////////////////////////////////
    // Public Interface

    pub fn new(machine_registry: GlobalStateManager) -> Self {
        let (sender, receiver) = flume::unbounded();

        Self {
            unlocked_inner: Arc::new(ServerProcessorUnlockedInner {
                sender,
                receiver,
                machine_registry,
            }),
            _inner: Arc::new(Mutex::new(ServerProcessorInner {})),
        }
    }

    pub fn enqueue_command(
        &self,
        cmd: ServerProcessorCommand,
        outbound_sender: flume::Sender<ServerProcessorEvent>,
    ) {
        if let Err(e) = self
            .unlocked_inner
            .sender
            .send(ServerProcessorCommandRecord {
                cmd,
                outbound_sender,
            })
        {
            eprintln!("Dropped command: {}", e);
        }
    }

    pub fn run_loop_process_commands(&self) -> PinBoxFuture<RunLoopEvent> {
        let receiver_stream = self.unlocked_inner.receiver.clone().into_stream();
        let this = self.clone();
        Box::pin(async move {
            receiver_stream
                .for_each_concurrent(None, |x| {
                    let this = this.clone();
                    async move {
                        if let Err(e) = this.process_command(x.cmd, x.outbound_sender).await {
                            eprintln!("Failed to process command: {}", e);
                        }
                    }
                })
                .await;

            RunLoopEvent::Done
        })
    }

    ////////////////////////////////////////////////////////////////////////
    // Private Implementation

    async fn process_command(
        self,
        cmd: ServerProcessorCommand,
        outbound_sender: flume::Sender<ServerProcessorEvent>,
    ) -> RouterServerResult<()> {
        match cmd {
            ServerProcessorCommand::Message(server_processor_message) => {
                self.process_message(
                    server_processor_message.message_id,
                    server_processor_message.request,
                    outbound_sender,
                )
                .await
            }
            ServerProcessorCommand::CloseSocket {
                machine_id,
                socket_id,
            } => {
                self.process_close_socket(machine_id, socket_id, outbound_sender)
                    .await
            }
        }
    }
    async fn process_close_socket(
        self,
        machine_id: MachineId,
        socket_id: SocketId,
        outbound_sender: flume::Sender<ServerProcessorEvent>,
    ) -> RouterServerResult<()> {
        //
        Ok(())
    }

    async fn process_message(
        self,
        message_id: MessageId,
        request: ServerProcessorRequest,
        outbound_sender: flume::Sender<ServerProcessorEvent>,
    ) -> RouterServerResult<()> {
        match request {
            ServerProcessorRequest::AllocateMachine { profile } => todo!(),
            ServerProcessorRequest::ReleaseMachine { machine_id } => todo!(),
            ServerProcessorRequest::GetInterfaces { machine_id } => todo!(),
            ServerProcessorRequest::TcpConnect {
                machine_id,
                local_address,
                remote_address,
                timeout_ms,
                options,
            } => todo!(),
            ServerProcessorRequest::TcpBind {
                machine_id,
                local_address,
                options,
            } => todo!(),
            ServerProcessorRequest::TcpAccept {
                machine_id,
                listen_socket_id,
            } => todo!(),
            ServerProcessorRequest::TcpShutdown {
                machine_id,
                socket_id,
            } => todo!(),
            ServerProcessorRequest::UdpBind {
                machine_id,
                local_address,
                options,
            } => todo!(),
            ServerProcessorRequest::Send {
                machine_id,
                socket_id,
                data,
            } => todo!(),
            ServerProcessorRequest::SendTo {
                machine_id,
                socket_id,
                remote_address,
                data,
            } => todo!(),
            ServerProcessorRequest::Recv {
                machine_id,
                socket_id,
                len,
            } => todo!(),
            ServerProcessorRequest::RecvFrom {
                machine_id,
                socket_id,
                len,
            } => todo!(),
            ServerProcessorRequest::GetRoutedLocalAddress {
                machine_id,
                address_type,
            } => todo!(),
            ServerProcessorRequest::FindGateway { machine_id } => todo!(),
            ServerProcessorRequest::GetExternalAddress { gateway_id } => todo!(),
            ServerProcessorRequest::AddPort {
                gateway_id,
                protocol,
                external_port,
                local_address,
                lease_duration_ms,
                description,
            } => todo!(),
            ServerProcessorRequest::RemovePort {
                gateway_id,
                protocol,
                external_port,
            } => todo!(),
            ServerProcessorRequest::TXTQuery { name } => todo!(),
        }
    }
}
