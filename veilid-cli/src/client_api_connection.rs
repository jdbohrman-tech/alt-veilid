use crate::command_processor::*;
use crate::tools::*;
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::time::SystemTime;
use stop_token::{future::FutureExt as _, StopSource};

cfg_if! {
    if #[cfg(feature="rt-async-std")] {
        use futures::{AsyncBufReadExt, AsyncWriteExt};
        use async_std::io::BufReader;
    } else if #[cfg(feature="rt-tokio")] {
        use tokio::io::AsyncBufReadExt;
        use tokio::io::AsyncWriteExt;
        use tokio::io::BufReader;
    }
}

struct ClientApiConnectionInner {
    comproc: CommandProcessor,
    request_sender: Option<flume::Sender<String>>,
    disconnector: Option<StopSource>,
    disconnect_requested: bool,
    reply_channels: HashMap<u32, flume::Sender<json::JsonValue>>,
    next_req_id: u32,
}

#[derive(Clone)]
pub struct ClientApiConnection {
    inner: Arc<Mutex<ClientApiConnectionInner>>,
}

impl ClientApiConnection {
    pub fn new(comproc: CommandProcessor) -> Self {
        Self {
            inner: Arc::new(Mutex::new(ClientApiConnectionInner {
                comproc,
                request_sender: None,
                disconnector: None,
                disconnect_requested: false,
                reply_channels: HashMap::new(),
                next_req_id: 0,
            })),
        }
    }

    pub fn cancel_all(&self) {
        let mut inner = self.inner.lock();
        inner.reply_channels.clear();
    }

    fn process_veilid_state(&self, state: &json::JsonValue) {
        let comproc = self.inner.lock().comproc.clone();
        comproc.update_attachment(&state["attachment"]);
        comproc.update_network_status(&state["network"]);
        comproc.update_config(&state["config"]);
    }

    async fn process_response(&self, response: json::JsonValue) {
        // find the operation id and send the response to the channel for it
        let Some(id) = response["id"].as_u32() else {
            error!("invalid id: {}", response);
            return;
        };
        let reply_channel = {
            let mut inner = self.inner.lock();
            inner.reply_channels.remove(&id)
        };
        let Some(reply_channel) = reply_channel else {
            warn!("received cancelled reply: {}", response);
            return;
        };
        if let Err(e) = reply_channel.send_async(response).await {
            error!("failed to process reply: {}", e);
        }
    }

    fn process_veilid_update(&self, update: json::JsonValue) {
        let comproc = self.inner.lock().comproc.clone();
        let Some(kind) = update["kind"].as_str() else {
            comproc.log_message(Level::Error, &format!("missing update kind: {}", update));
            return;
        };
        match kind {
            "Log" => {
                comproc.update_log(&update);
            }
            "AppMessage" => {
                comproc.update_app_message(&update);
            }
            "AppCall" => {
                comproc.update_app_call(&update);
            }
            "Attachment" => {
                comproc.update_attachment(&update);
            }
            "Network" => {
                comproc.update_network_status(&update);
            }
            "Config" => {
                comproc.update_config(&update);
            }
            "RouteChange" => {
                comproc.update_route(&update);
            }
            "Shutdown" => comproc.update_shutdown(),
            "ValueChange" => {
                comproc.update_value_change(&update);
            }
            _ => {
                comproc.log_message(Level::Error, &format!("unknown update kind: {}", update));
            }
        }
    }

    pub async fn run_json_api_processor<R, W>(
        self,
        mut reader: R,
        mut writer: W,
    ) -> Result<(), String>
    where
        R: AsyncBufReadExt + Unpin + Send,
        W: AsyncWriteExt + Unpin + Send,
    {
        // Requests to send
        let (requests_tx, requests_rx) = flume::unbounded();

        // Create disconnection mechanism
        let stop_token = {
            let stop_source = StopSource::new();
            let token = stop_source.token();
            let mut inner = self.inner.lock();
            inner.disconnector = Some(stop_source);
            inner.request_sender = Some(requests_tx);
            token
        };

        // Futures to process unordered
        let mut unord = FuturesUnordered::new();

        // Process lines
        let this = self.clone();
        let recv_messages_future = async move {
            let mut linebuf = String::new();
            while let Ok(size) = reader.read_line(&mut linebuf).await {
                // Exit on EOF
                if size == 0 {
                    // Disconnected
                    break;
                }

                let line = linebuf.trim().to_owned();
                linebuf.clear();

                // Unmarshal json
                let j = match json::parse(&line) {
                    Ok(v) => v,
                    Err(e) => {
                        error!("failed to parse server response: {}", e);
                        continue;
                    }
                };

                if j["type"] == "Update" {
                    this.process_veilid_update(j);
                } else if j["type"] == "Response" {
                    this.process_response(j).await;
                }
            }
            //
            let mut inner = this.inner.lock();
            inner.request_sender = None;
        };
        unord.push(pin_dyn_future!(recv_messages_future));

        // Requests send processor
        let send_requests_future = async move {
            while let Ok(req) = requests_rx.recv_async().await {
                if let Err(e) = writer.write_all(req.as_bytes()).await {
                    error!("failed to write request: {}", e)
                }
            }
        };
        unord.push(pin_dyn_future!(send_requests_future));

        // Request initial server state
        let capi = self.clone();
        spawn_detached_local("get initial server state", async move {
            let mut req = json::JsonValue::new_object();
            req["op"] = "GetState".into();
            let Some(resp) = capi.perform_request(req).await else {
                error!("failed to get state");
                return;
            };
            if resp.has_key("error") {
                error!("failed to get state: {}", resp["error"]);
                return;
            }
            capi.process_veilid_state(&resp["value"]);
        });

        // Send and receive until we're done or a stop is requested
        while let Ok(Some(())) = unord.next().timeout_at(stop_token.clone()).await {}

        // // Drop the server and disconnector too (if we still have it)
        let mut inner = self.inner.lock();
        let disconnect_requested = inner.disconnect_requested;
        inner.request_sender = None;
        inner.disconnector = None;
        inner.disconnect_requested = false;

        // Connection finished
        if disconnect_requested {
            Ok(())
        } else {
            Err("Connection lost".to_owned())
        }
    }

    async fn handle_tcp_connection(&self, connect_addr: SocketAddr) -> Result<(), String> {
        trace!("ClientApiConnection::handle_tcp_connection");

        // Connect the TCP socket
        let stream = connect_async_tcp_stream(None, connect_addr, 10_000)
            .await
            .map_err(map_to_string)?
            .into_timeout_error()
            .map_err(map_to_string)?;

        // State we connected
        let comproc = self.inner.lock().comproc.clone();
        comproc.set_connection_state(ConnectionState::ConnectedTCP(
            connect_addr,
            SystemTime::now(),
        ));

        // Split into reader and writer halves
        // with line buffering on the reader
        let (reader, writer) = split_async_tcp_stream(stream);
        let reader = BufReader::new(reader);

        self.clone().run_json_api_processor(reader, writer).await
    }

    async fn handle_ipc_connection(&self, ipc_path: PathBuf) -> Result<(), String> {
        trace!("ClientApiConnection::handle_ipc_connection");

        // Connect the IPC socket
        let stream = IpcStream::connect(&ipc_path).await.map_err(map_to_string)?;

        // State we connected
        let comproc = self.inner.lock().comproc.clone();
        comproc.set_connection_state(ConnectionState::ConnectedIPC(ipc_path, SystemTime::now()));

        // Split into reader and writer halves
        // with line buffering on the reader
        use futures::AsyncReadExt;
        let (reader, writer) = stream.split();
        cfg_if! {
            if #[cfg(feature = "rt-tokio")] {
                use tokio_util::compat::{FuturesAsyncReadCompatExt, FuturesAsyncWriteCompatExt};
                let reader = reader.compat();
                let writer = writer.compat_write();
            }
        }
        let reader = BufReader::new(reader);

        self.clone().run_json_api_processor(reader, writer).await
    }

    async fn perform_request(&self, mut req: json::JsonValue) -> Option<json::JsonValue> {
        let (sender, reply_rx) = {
            let mut inner = self.inner.lock();

            // Get the request sender
            let Some(sender) = inner.request_sender.clone() else {
                error!("dropping request, not connected");
                return None;
            };

            // Get next id
            let id = inner.next_req_id;
            inner.next_req_id += 1;

            // Add the id
            req["id"] = id.into();

            // Make a reply receiver
            let (reply_tx, reply_rx) = flume::bounded(1);
            inner.reply_channels.insert(id, reply_tx);
            (sender, reply_rx)
        };

        // Send the request
        let req_ndjson = req.dump() + "\n";
        if let Err(e) = sender.send_async(req_ndjson).await {
            error!("failed to send request: {}", e);
            return None;
        }

        // Wait for the reply
        let Ok(r) = reply_rx.recv_async().await else {
            // Cancelled
            return None;
        };

        Some(r)
    }

    pub async fn server_attach(&self) -> Result<(), String> {
        trace!("ClientApiConnection::server_attach");

        let mut req = json::JsonValue::new_object();
        req["op"] = "Attach".into();
        let Some(resp) = self.perform_request(req).await else {
            return Err("Cancelled".to_owned());
        };
        if resp.has_key("error") {
            return Err(resp["error"].to_string());
        }
        Ok(())
    }

    pub async fn server_detach(&self) -> Result<(), String> {
        trace!("ClientApiConnection::server_detach");
        let mut req = json::JsonValue::new_object();
        req["op"] = "Detach".into();
        let Some(resp) = self.perform_request(req).await else {
            return Err("Cancelled".to_owned());
        };
        if resp.has_key("error") {
            return Err(resp["error"].to_string());
        }
        Ok(())
    }

    pub async fn server_shutdown(&self) -> Result<(), String> {
        trace!("ClientApiConnection::server_shutdown");
        let mut req = json::JsonValue::new_object();
        req["op"] = "Control".into();
        req["args"] = json::JsonValue::new_array();
        req["args"].push("Shutdown").unwrap();
        let Some(resp) = self.perform_request(req).await else {
            return Err("Cancelled".to_owned());
        };
        if resp.has_key("error") {
            return Err(resp["error"].to_string());
        }
        Ok(())
    }

    pub async fn server_debug(&self, what: String) -> Result<String, String> {
        trace!("ClientApiConnection::server_debug");
        let mut req = json::JsonValue::new_object();
        req["op"] = "Debug".into();
        req["command"] = what.into();
        let Some(resp) = self.perform_request(req).await else {
            return Err("Cancelled".to_owned());
        };
        if resp.has_key("error") {
            return Err(resp["error"].to_string());
        }
        Ok(resp["value"].to_string())
    }

    pub async fn server_change_log_level(
        &self,
        layer: String,
        log_level: String,
    ) -> Result<(), String> {
        trace!("ClientApiConnection::change_log_level");
        let mut req = json::JsonValue::new_object();
        req["op"] = "Control".into();
        req["args"] = json::JsonValue::new_array();
        req["args"].push("ChangeLogLevel").unwrap();
        req["args"].push(layer).unwrap();
        req["args"].push(log_level).unwrap();
        let Some(resp) = self.perform_request(req).await else {
            return Err("Cancelled".to_owned());
        };
        if resp.has_key("error") {
            return Err(resp["error"].to_string());
        }
        Ok(())
    }

    pub async fn server_change_log_ignore(
        &self,
        layer: String,
        log_ignore: String,
    ) -> Result<(), String> {
        trace!("ClientApiConnection::change_log_ignore");
        let mut req = json::JsonValue::new_object();
        req["op"] = "Control".into();
        req["args"] = json::JsonValue::new_array();
        req["args"].push("ChangeLogIgnore").unwrap();
        req["args"].push(layer).unwrap();
        req["args"].push(log_ignore).unwrap();
        let Some(resp) = self.perform_request(req).await else {
            return Err("Cancelled".to_owned());
        };
        if resp.has_key("error") {
            return Err(resp["error"].to_string());
        }
        Ok(())
    }

    // Start Client API connection
    pub async fn ipc_connect(&self, ipc_path: PathBuf) -> Result<(), String> {
        trace!("ClientApiConnection::ipc_connect");
        // Save the pathto connect to
        self.handle_ipc_connection(ipc_path).await
    }
    pub async fn tcp_connect(&self, connect_addr: SocketAddr) -> Result<(), String> {
        trace!("ClientApiConnection::tcp_connect");
        // Save the address to connect to
        self.handle_tcp_connection(connect_addr).await
    }

    // End Client API connection
    pub fn disconnect(&self) {
        trace!("ClientApiConnection::disconnect");
        let mut inner = self.inner.lock();
        if inner.disconnector.is_some() {
            inner.disconnector = None;
            inner.disconnect_requested = true;
        }
    }
}
