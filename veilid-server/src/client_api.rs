use crate::settings::*;
use crate::tools::*;
use crate::veilid_logs::VeilidLogs;
use cfg_if::*;
use futures_util::{future::join_all, stream::FuturesUnordered, StreamExt};
use parking_lot::Mutex;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use stop_token::future::FutureExt as _;
use stop_token::*;
use tracing::*;
use veilid_core::tools::*;
use veilid_core::*;
use veilid_remote_api::JsonRequestProcessor;
use wg::AsyncWaitGroup;

const MAX_NON_JSON_LOGGING: usize = 50;

cfg_if! {

    if #[cfg(feature="rt-async-std")] {
        use futures_util::{AsyncBufReadExt, AsyncWriteExt};
    } else
    if #[cfg(feature="rt-tokio")] {
        use tokio::io::AsyncBufReadExt;
        use tokio::io::AsyncWriteExt;
    } else {
        compile_error!("needs executor implementation");
    }
}

// --- Client API Server-Side ---------------------------------

type ClientApiAllFuturesJoinHandle = MustJoinHandle<Vec<()>>;

struct RequestLine {
    // Request to process
    line: String,
    // Where to send the response
    responses_tx: flume::Sender<Arc<String>>,
}

struct ClientApiInner {
    veilid_api: veilid_core::VeilidAPI,
    veilid_logs: VeilidLogs,
    settings: Settings,
    stop: Option<StopSource>,
    join_handle: Option<ClientApiAllFuturesJoinHandle>,
    update_channels: HashMap<u64, flume::Sender<Arc<String>>>,
}

#[derive(Clone)]
pub struct ClientApi {
    inner: Arc<Mutex<ClientApiInner>>,
}

impl ClientApi {
    #[instrument(level = "trace", skip_all)]
    pub fn new(
        veilid_api: veilid_core::VeilidAPI,
        veilid_logs: VeilidLogs,
        settings: Settings,
    ) -> Self {
        Self {
            inner: Arc::new(Mutex::new(ClientApiInner {
                veilid_api,
                veilid_logs,
                settings,
                stop: Some(StopSource::new()),
                join_handle: None,
                update_channels: HashMap::new(),
            })),
        }
    }

    #[instrument(level = "trace", skip_all)]
    fn shutdown(&self) {
        trace!(target: "client_api", "ClientApi::shutdown");

        crate::server::shutdown();
    }

    fn change_log_level(
        &self,
        layer: String,
        log_level: VeilidConfigLogLevel,
    ) -> VeilidAPIResult<()> {
        trace!(target: "client_api", "ClientApi::change_log_level");

        let veilid_logs = self.inner.lock().veilid_logs.clone();
        veilid_logs.change_log_level(layer, log_level)
    }

    fn change_log_ignore(&self, layer: String, log_ignore: String) -> VeilidAPIResult<()> {
        trace!(target: "client_api", "ClientApi::change_log_ignore");

        let veilid_logs = self.inner.lock().veilid_logs.clone();
        veilid_logs.change_log_ignore(layer, log_ignore)
    }

    #[instrument(level = "trace", skip(self))]
    pub async fn stop(&self) {
        trace!(target: "client_api", "ClientApi::stop requested");
        let jh = {
            let mut inner = self.inner.lock();
            if inner.join_handle.is_none() {
                trace!(target: "client_api", "ClientApi stop ignored");
                return;
            }
            drop(inner.stop.take());
            inner.join_handle.take().unwrap()
        };
        trace!(target: "client_api", "ClientApi::stop: waiting for stop");
        jh.await;
        trace!(target: "client_api", "ClientApi::stop: stopped");
    }

    async fn handle_ipc_incoming(self, ipc_path: PathBuf) -> std::io::Result<()> {
        if ipc_path.exists() {
            if let Err(e) = std::fs::remove_file(&ipc_path) {
                error!("Binding failed because IPC path is in use: {}\nAnother copy of this application may be using the same IPC path.", e);
                return Err(e);
            }
        }
        let mut listener = IpcListener::bind(ipc_path.clone()).await?;
        debug!(target: "client_api", "IPC Client API listening on: {:?}", ipc_path);

        // Process the incoming accept stream
        let mut incoming_stream = listener.incoming()?;

        // Make wait group for all incoming connections
        let awg = AsyncWaitGroup::new();

        let stop_token = match self.inner.lock().stop.as_ref() {
            Some(stop) => stop.token(),
            None => {
                debug!(target: "client_api", "Already stopped");
                return Ok(());
            }
        };
        while let Ok(Some(stream_result)) =
            incoming_stream.next().timeout_at(stop_token.clone()).await
        {
            // Get the stream to process
            let stream = stream_result?;

            // Increment wait group
            awg.add(1);
            let t_awg = awg.clone();

            // Process the connection
            spawn(
                "client_api handle_ipc_connection",
                self.clone().handle_ipc_connection(stream, t_awg),
            )
            .detach();
        }

        // Wait for all connections to terminate
        awg.wait().await;

        Ok(())
    }

    async fn handle_tcp_incoming(self, bind_addr: SocketAddr) -> std::io::Result<()> {
        let listener = bind_async_tcp_listener(bind_addr)?
            .ok_or(std::io::Error::from(std::io::ErrorKind::AddrInUse))?;
        debug!(target: "client_api", "TCPClient API listening on: {:?}", bind_addr);

        // Process the incoming accept stream
        let mut incoming_stream = async_tcp_listener_incoming(listener);

        // Make wait group for all incoming connections
        let awg = AsyncWaitGroup::new();

        let stop_token = match self.inner.lock().stop.as_ref() {
            Some(stop) => stop.token(),
            None => {
                debug!(target: "client_api", "Already stopped");
                return Ok(());
            }
        };
        while let Ok(Some(stream_result)) =
            incoming_stream.next().timeout_at(stop_token.clone()).await
        {
            // Get the stream to process
            let stream = stream_result?;
            stream.set_nodelay(true)?;

            // Increment wait group
            awg.add(1);
            let t_awg = awg.clone();

            // Process the connection
            spawn(
                "client_api handle_tcp_connection",
                self.clone().handle_tcp_connection(stream, t_awg),
            )
            .detach();
        }

        // Wait for all connections to terminate
        awg.wait().await;

        Ok(())
    }

    // Process control messages for the server
    fn process_control(self, args: Vec<String>) -> VeilidAPIResult<String> {
        if args.is_empty() {
            apibail_generic!("no control request specified");
        }
        if args[0] == "Shutdown" {
            if args.len() != 1 {
                apibail_generic!("wrong number of arguments");
            }
            self.shutdown();
            Ok("".to_owned())
        } else if args[0] == "ChangeLogLevel" {
            if args.len() != 3 {
                apibail_generic!("wrong number of arguments");
            }
            let log_level = VeilidConfigLogLevel::from_str(&args[2])?;
            self.change_log_level(args[1].clone(), log_level)?;
            Ok("".to_owned())
        } else if args[0] == "ChangeLogIgnore" {
            if args.len() != 3 {
                apibail_generic!("wrong number of arguments");
            }
            self.change_log_ignore(args[1].clone(), args[2].clone())?;
            Ok("".to_owned())
        } else if args[0] == "GetServerSettings" {
            if args.len() != 1 {
                apibail_generic!("wrong number of arguments");
            }
            let settings = self.inner.lock().settings.clone();
            let settings = &*settings.read();
            let settings_json_string = serialize_json(settings);
            let mut settings_json =
                json::parse(&settings_json_string).map_err(VeilidAPIError::internal)?;
            settings_json["core"]["network"].remove("node_id_secret");
            settings_json["core"]["protected_store"].remove("device_encryption_key_password");
            settings_json["core"]["protected_store"].remove("new_device_encryption_key_password");
            let safe_settings_json = settings_json.to_string();
            Ok(safe_settings_json)
        } else if args[0] == "EmitSchema" {
            if args.len() != 2 {
                apibail_generic!("wrong number of arguments");
            }

            let mut schemas = HashMap::<String, String>::new();
            veilid_remote_api::emit_schemas(&mut schemas);

            let Some(schema) = schemas.get(&args[1]) else {
                apibail_invalid_argument!("invalid schema", "schema", args[1].clone());
            };

            Ok(schema.clone())
        } else {
            apibail_generic!("unknown control message");
        }
    }

    async fn process_request_line(
        self,
        jrp: JsonRequestProcessor,
        request_line: RequestLine,
    ) -> VeilidAPIResult<Option<RequestLine>> {
        let line = request_line.line.trim_start();

        // Avoid logging failed deserialization of large adversarial payloads from
        // http://127.0.0.1:5959 by using an initial colon to force a parse error.
        let sanitized_line = if line.len() > MAX_NON_JSON_LOGGING && !line.starts_with('{') {
            ":skipped long input that's not a JSON object".to_string()
        } else {
            line.to_string()
        };

        let responses_tx = request_line.responses_tx;

        // Unmarshal NDJSON - newline => json
        // (trim all whitespace around input lines just to make things more permissive for API users)
        let request: veilid_remote_api::Request = deserialize_json(&sanitized_line)?;

        #[cfg(feature = "debug-json-api")]
        debug!("JSONAPI: Request: {:?}", request);

        // See if this is a control message or a veilid-core message
        let response = if let veilid_remote_api::RequestOp::Control { args } = request.op {
            // Process control messages
            veilid_remote_api::Response {
                id: request.id,
                op: veilid_remote_api::ResponseOp::Control {
                    result: veilid_remote_api::to_json_api_result(self.process_control(args)),
                },
            }
        } else {
            // Process with ndjson api
            jrp.clone().process_request(request).await
        };

        #[cfg(feature = "debug-json-api")]
        debug!("JSONAPI: Response: {:?}", response);

        // Marshal json + newline => NDJSON
        let response_string =
            Arc::new(serialize_json(veilid_remote_api::RecvMessage::Response(response)) + "\n");
        if let Err(e) = responses_tx.send_async(response_string).await {
            eprintln!("response not sent: {}", e)
        }
        VeilidAPIResult::Ok(None)
    }

    async fn next_request_line(
        requests_rx: flume::Receiver<Option<RequestLine>>,
    ) -> VeilidAPIResult<Option<RequestLine>> {
        Ok(requests_rx.recv_async().await.ok().flatten())
    }

    async fn receive_requests<R: AsyncBufReadExt + Unpin>(
        self,
        mut reader: R,
        requests_tx: flume::Sender<Option<RequestLine>>,
        responses_tx: flume::Sender<Arc<String>>,
    ) -> VeilidAPIResult<Option<RequestLine>> {
        let mut linebuf = String::new();
        while let Ok(size) = reader.read_line(&mut linebuf).await {
            // Eof?
            if size == 0 {
                break;
            }

            // Put the processing in the async queue
            let line = linebuf.trim().to_owned();
            linebuf.clear();

            // Ignore newlines
            if line.is_empty() {
                continue;
            }

            // Enqueue the line for processing in parallel
            let request_line = RequestLine {
                line,
                responses_tx: responses_tx.clone(),
            };
            if let Err(e) = requests_tx.send_async(Some(request_line)).await {
                eprintln!("failed to enqueue request: {}", e);
                break;
            }
        }

        VeilidAPIResult::Ok(None)
    }

    async fn send_responses<W: AsyncWriteExt + Unpin>(
        self,
        responses_rx: flume::Receiver<Arc<String>>,
        mut writer: W,
    ) -> VeilidAPIResult<Option<RequestLine>> {
        while let Ok(resp) = responses_rx.recv_async().await {
            if (writer.write_all(resp.as_bytes()).await).is_err() {
                break;
            }
        }
        VeilidAPIResult::Ok(None)
    }

    pub async fn run_json_request_processor<R, W>(self, reader: R, writer: W, stop_token: StopToken)
    where
        R: AsyncBufReadExt + Unpin + Send,
        W: AsyncWriteExt + Unpin + Send,
    {
        // Make request processor for this connection
        let api = self.inner.lock().veilid_api.clone();
        let jrp = veilid_remote_api::JsonRequestProcessor::new(api);

        // Futures to process unordered
        let mut unord = FuturesUnordered::new();

        // Requests and responses are done serially to the socket
        // but the requests are processed in parallel by the FuturesUnordered
        let (requests_tx, requests_rx) = flume::unbounded();
        let (responses_tx, responses_rx) = flume::unbounded();

        // Start sending updates
        let id = get_timestamp();
        self.inner
            .lock()
            .update_channels
            .insert(id, responses_tx.clone());

        // Request receive processor future
        // Receives from socket and enqueues RequestLines
        // Completes when the connection is closed or there is a failure
        unord.push(pin_dyn_future!(self.clone().receive_requests(
            reader,
            requests_tx,
            responses_tx,
        )));

        // Response send processor
        // Sends finished response strings out the socket
        // Completes when the responses channel is closed
        unord.push(pin_dyn_future!(self
            .clone()
            .send_responses(responses_rx, writer)));

        // Add future to process first request
        unord.push(pin_dyn_future!(Self::next_request_line(
            requests_rx.clone()
        )));

        // Send and receive until we're done or a stop is requested
        while let Ok(Some(r)) = unord.next().timeout_at(stop_token.clone()).await {
            // See if we got some work to do
            let request_line = match r {
                Ok(Some(request_line)) => {
                    // Add future to process next request
                    unord.push(pin_dyn_future!(Self::next_request_line(
                        requests_rx.clone()
                    )));

                    // Socket receive future returned something to process
                    request_line
                }
                Ok(None) => {
                    // Non-request future finished
                    continue;
                }
                Err(e) => {
                    // Connection processing failure, abort
                    eprintln!("Connection processing failure: {}", e);
                    break;
                }
            };

            // Enqueue unordered future to process request line in parallel
            unord.push(pin_dyn_future!(self
                .clone()
                .process_request_line(jrp.clone(), request_line)));
        }

        // Stop sending updates
        self.inner.lock().update_channels.remove(&id);
    }

    pub async fn handle_tcp_connection(self, stream: TcpStream, awg: AsyncWaitGroup) {
        // Get address of peer
        let peer_addr = match stream.peer_addr() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("can't get peer address: {}", e);
                return;
            }
        };
        // Get local address
        let local_addr = match stream.local_addr() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("can't get local address: {}", e);
                return;
            }
        };
        // Get connection tuple
        debug!(
            "Accepted TCP Client API Connection: {:?} -> {:?}",
            peer_addr, local_addr
        );

        // Make stop token to quit when stop() is requested externally
        let stop_token = self.inner.lock().stop.as_ref().unwrap().token();

        // Split into reader and writer halves
        // with line buffering on the reader
        cfg_if! {
            if #[cfg(feature="rt-async-std")] {
                use futures_util::AsyncReadExt;
                let (reader, writer) = stream.split();
                let reader = BufReader::new(reader);
            } else {
                let (reader, writer) = stream.into_split();
                let reader = BufReader::new(reader);
            }
        }

        self.run_json_request_processor(reader, writer, stop_token)
            .await;

        debug!(
            "Closed TCP Client API Connection: {:?} -> {:?}",
            peer_addr, local_addr
        );

        awg.done();
    }

    pub async fn handle_ipc_connection(self, stream: IpcStream, awg: AsyncWaitGroup) {
        // Get connection tuple
        debug!("Accepted IPC Client API Connection");

        // Make stop token to quit when stop() is requested externally
        let stop_token = self.inner.lock().stop.as_ref().unwrap().token();

        // Split into reader and writer halves
        // with line buffering on the reader
        use futures_util::AsyncReadExt;
        let (reader, writer) = stream.split();
        cfg_if! {
            if #[cfg(feature = "rt-tokio")] {
                use tokio_util::compat::{FuturesAsyncReadCompatExt, FuturesAsyncWriteCompatExt};
                let reader = reader.compat();
                let writer = writer.compat_write();
            }
        }
        let reader = BufReader::new(reader);

        self.run_json_request_processor(reader, writer, stop_token)
            .await;

        debug!("Closed IPC Client API Connection",);

        awg.done();
    }

    pub fn handle_update(&self, veilid_update: veilid_core::VeilidUpdate) {
        let inner = self.inner.lock();
        if inner.update_channels.is_empty() {
            return;
        }

        // serialize update to NDJSON
        let veilid_update =
            Arc::new(serialize_json(veilid_remote_api::RecvMessage::Update(veilid_update)) + "\n");

        // Pass updates to clients
        for ch in inner.update_channels.values() {
            if ch.send(veilid_update.clone()).is_err() {
                // eprintln!("failed to send update: {}", e);
            }
        }
    }

    #[instrument(level = "trace", skip(self))]
    pub fn run(&self, ipc_path: Option<PathBuf>, tcp_bind_addrs: Vec<SocketAddr>) {
        let mut bind_futures: Vec<PinBoxFutureStatic<()>> = Vec::new();

        // Local IPC
        if let Some(ipc_path) = ipc_path {
            let this = self.clone();
            bind_futures.push(Box::pin(async move {
                if let Err(e) = this.handle_ipc_incoming(ipc_path.clone()).await {
                    warn!("Not binding IPC client API to {:?}: {}", ipc_path, e);
                }
            }));
        }

        // Network sockets
        for addr in tcp_bind_addrs.iter().copied() {
            let this = self.clone();
            bind_futures.push(Box::pin(async move {
                if let Err(e) = this.handle_tcp_incoming(addr).await {
                    warn!("Not binding TCP client API to {}: {}", addr, e);
                }
            }));
        }

        let bind_futures_join = join_all(bind_futures);
        self.inner.lock().join_handle = Some(spawn("client_api bind_futures", bind_futures_join));
    }
}
