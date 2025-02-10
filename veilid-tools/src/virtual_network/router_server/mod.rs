pub mod config;
mod global_state_manager;
mod server_processor;
mod stable_rng;
mod weighted_list;

use super::*;

use global_state_manager::*;
use server_processor::*;
use stable_rng::*;
use weighted_list::*;

use async_tungstenite::accept_async;
use futures_codec::{Bytes, BytesCodec, FramedRead, FramedWrite};
use futures_util::{stream::FuturesUnordered, AsyncReadExt, StreamExt, TryStreamExt};
use ipnet::*;
use postcard::{from_bytes, to_stdvec};
use std::io;
use stop_token::future::FutureExt as _;
use ws_stream_tungstenite::*;

#[derive(ThisError, Debug, Clone, PartialEq, Eq)]
pub enum RouterServerError {
    #[error("Serialization Error: {0}")]
    SerializationError(postcard::Error),
    #[error("IO Error: {0}")]
    IoError(io::ErrorKind),
    #[error("State Error: {0}")]
    StateError(global_state_manager::GlobalStateManagerError),
}

pub type RouterServerResult<T> = Result<T, RouterServerError>;

pub const DEFAULT_VIRTUAL_ROUTER_PORT_TCP: u16 = 5149u16;
pub const DEFAULT_VIRTUAL_ROUTER_PORT_WS: u16 = 5148u16;

enum RunLoopEvent {
    AddClient(SendPinBoxFuture<RunLoopEvent>),
    Done,
}

#[derive(Debug)]
struct RouterServerUnlockedInner {
    new_client_sender: flume::Sender<SendPinBoxFuture<RunLoopEvent>>,
    new_client_receiver: flume::Receiver<SendPinBoxFuture<RunLoopEvent>>,
    server_processor: ServerProcessor,
    global_state_manager: GlobalStateManager,
}

#[derive(Debug)]
struct RouterServerInner {}

/// Router server for virtual networking
///
/// Connect to this with a `RouterClient`. Simulates machines, allocates sockets
/// and gateways, manages a virtual simulated Internet and routes packets
/// virtually between `Machines` associated with `RouterClient`s.
#[derive(Debug, Clone)]
pub struct RouterServer {
    unlocked_inner: Arc<RouterServerUnlockedInner>,
    _inner: Arc<Mutex<RouterServerInner>>,
}

impl Default for RouterServer {
    fn default() -> Self {
        Self::new()
    }
}

impl RouterServer {
    ////////////////////////////////////////////////////////////////////
    // Public Interface

    /// Create a router server for virtual networking
    pub fn new() -> Self {
        // Make a channel to receive new clients
        let (new_client_sender, new_client_receiver) = flume::unbounded();

        // Make a machine registry to manage state
        let global_state_manager = GlobalStateManager::new();

        // Make a server processor to handle messages
        let server_processor = ServerProcessor::new(global_state_manager.clone());

        Self {
            unlocked_inner: Arc::new(RouterServerUnlockedInner {
                new_client_sender,
                new_client_receiver,
                server_processor,
                global_state_manager,
            }),
            _inner: Arc::new(Mutex::new(RouterServerInner {})),
        }
    }

    /// Execute a config file on the global state manager
    pub fn execute_config(&self, cfg: config::Config) -> RouterServerResult<()> {
        self.unlocked_inner
            .global_state_manager
            .execute_config(cfg)
            .map_err(RouterServerError::StateError)
    }

    /// Accept RouterClient connections on a TCP socket
    pub async fn listen_tcp(&self, addr: Option<SocketAddr>) -> RouterServerResult<StopSource> {
        let listener = TcpListener::bind(addr.unwrap_or(SocketAddr::V6(SocketAddrV6::new(
            Ipv6Addr::UNSPECIFIED,
            DEFAULT_VIRTUAL_ROUTER_PORT_TCP,
            0,
            0,
        ))))
        .await
        .map_err(|e| RouterServerError::IoError(e.kind()))?;

        let stop_source = StopSource::new();
        let stop_token = stop_source.token();

        let this = self.clone();
        let listener_fut = system_boxed(async move {
            loop {
                // Wait for a new connection
                match listener.accept().timeout_at(stop_token.clone()).await {
                    Ok(Ok((conn, _addr))) => {
                        let conn = conn.compat();
                        // Register a connection processing inbound receiver
                        let this2 = this.clone();
                        let inbound_receiver_fut = system_boxed(async move {
                            let (reader, writer) = conn.split();

                            this2.process_connection(reader, writer).await
                        });
                        if let Err(e) = this
                            .unlocked_inner
                            .new_client_sender
                            .send(inbound_receiver_fut)
                        {
                            // Error registering connection processor
                            error!("{}", e);
                            break;
                        }
                    }
                    Ok(Err(e)) => {
                        // Error processing an accept
                        error!("{}", e);
                        break;
                    }
                    Err(_) => {
                        // Stop requested
                        break;
                    }
                }
            }

            RunLoopEvent::Done
        });

        self.unlocked_inner
            .new_client_sender
            .send(listener_fut)
            .expect("should be able to send client");

        Ok(stop_source)
    }

    /// Accept RouterClient connections on a WebSocket
    pub async fn listen_ws(&self, addr: Option<SocketAddr>) -> RouterServerResult<StopSource> {
        let listener = TcpListener::bind(addr.unwrap_or(SocketAddr::V6(SocketAddrV6::new(
            Ipv6Addr::UNSPECIFIED,
            DEFAULT_VIRTUAL_ROUTER_PORT_WS,
            0,
            0,
        ))))
        .await
        .map_err(|e| RouterServerError::IoError(e.kind()))?;

        let stop_source = StopSource::new();
        let stop_token = stop_source.token();

        let this = self.clone();
        let listener_fut = system_boxed(async move {
            loop {
                // Wait for a new connection
                match listener.accept().timeout_at(stop_token.clone()).await {
                    Ok(Ok((conn, _addr))) => {
                        let conn = conn.compat();
                        if let Ok(s) = accept_async(conn).await {
                            let ws = WsStream::new(s);
                            // Register a connection processing inbound receiver
                            let this2 = this.clone();
                            let inbound_receiver_fut = system_boxed(async move {
                                let (reader, writer) = ws.split();
                                this2.process_connection(reader, writer).await
                            });
                            if let Err(e) = this
                                .unlocked_inner
                                .new_client_sender
                                .send(inbound_receiver_fut)
                            {
                                // Error registering connection processor
                                error!("{}", e);
                                break;
                            }
                        }
                    }
                    Ok(Err(e)) => {
                        // Error processing an accept
                        error!("{}", e);
                        break;
                    }
                    Err(_) => {
                        // Stop requested
                        break;
                    }
                }
            }

            RunLoopEvent::Done
        });

        self.unlocked_inner
            .new_client_sender
            .send(listener_fut)
            .expect("should be able to send client");

        Ok(stop_source)
    }

    /// Return a local RouterClient
    pub fn router_client(&self) -> RouterClient {
        // Create the inbound/outbound channels
        let (local_inbound_sender, local_inbound_receiver) = flume::unbounded();
        let (local_outbound_sender, local_outbound_receiver) = flume::unbounded();

        let this = self.clone();
        let inbound_receiver_fut = system_boxed(async move {
            local_inbound_receiver
                .into_stream()
                .for_each(|cmd| async {
                    this.unlocked_inner
                        .server_processor
                        .enqueue_command(cmd, local_outbound_sender.clone());
                })
                .await;
            RunLoopEvent::Done
        });

        // Send the new client to the run loop
        self.unlocked_inner
            .new_client_sender
            .send(inbound_receiver_fut)
            .expect("should be able to send client");

        // Create a RouterClient directly connected to this RouterServer
        RouterClient::local_router_client(local_inbound_sender, local_outbound_receiver)
    }

    /// Run the router server until a stop is requested
    pub async fn run(&self, stop_token: StopToken) -> RouterServerResult<()> {
        let mut unord = FuturesUnordered::<SendPinBoxFuture<RunLoopEvent>>::new();

        let mut need_new_client_fut = true;

        // Add server processor to run loop
        unord.push(
            self.unlocked_inner
                .server_processor
                .run_loop_process_commands(),
        );

        loop {
            if need_new_client_fut {
                let new_client_receiver = self.unlocked_inner.new_client_receiver.clone();
                unord.push(Box::pin(async move {
                    if let Ok(res) = new_client_receiver.into_recv_async().await {
                        return RunLoopEvent::AddClient(res);
                    }
                    RunLoopEvent::Done
                }));
            }

            match unord.next().timeout_at(stop_token.clone()).await {
                Ok(Some(RunLoopEvent::AddClient(client_fut))) => {
                    // Add new client
                    unord.push(client_fut);

                    // Wait for next new client
                    need_new_client_fut = true;
                }
                Ok(Some(RunLoopEvent::Done)) => {
                    // Do nothing
                }
                Ok(None) => {
                    // Finished normally
                    break;
                }
                Err(_) => {
                    // Stop requested
                    break;
                }
            }
        }

        Ok(())
    }

    ////////////////////////////////////////////////////////////////////
    // Private Implementation

    async fn process_connection<R, W>(self, reader: R, writer: W) -> RunLoopEvent
    where
        R: AsyncRead + Send + Unpin,
        W: AsyncWrite + Send + Unpin,
    {
        let framed_reader = FramedRead::new(reader, BytesCodec);
        let framed_writer = FramedWrite::new(writer, BytesCodec);

        let (outbound_sender, outbound_receiver) = flume::unbounded();
        let outbound_fut = system_boxed(
            outbound_receiver
                .into_stream()
                .map(|command| {
                    to_stdvec(&command)
                        .map_err(io::Error::other)
                        .map(Bytes::from)
                })
                .forward(framed_writer),
        );

        let inbound_fut = system_boxed(framed_reader.try_for_each(|x| async {
            let x = x;
            let cmd = from_bytes::<ServerProcessorCommand>(&x).map_err(io::Error::other)?;

            self.unlocked_inner
                .server_processor
                .enqueue_command(cmd, outbound_sender.clone());

            Ok(())
        }));

        let mut unord = FuturesUnordered::new();
        unord.push(outbound_fut);
        unord.push(inbound_fut);

        if let Some(Err(e)) = unord.next().await {
            error!("{}", e);
        }

        RunLoopEvent::Done
    }
}
