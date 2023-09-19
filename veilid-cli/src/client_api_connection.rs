use crate::command_processor::*;
use crate::tools::*;
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use std::net::SocketAddr;
use std::time::SystemTime;
use stop_token::{future::FutureExt as _, StopSource};

cfg_if! {
    if #[cfg(feature="rt-async-std")] {
        use async_std::io::prelude::BufReadExt;
        use async_std::io::WriteExt;
        use async_std::io::BufReader;
    } else if #[cfg(feature="rt-tokio")] {
        use tokio::io::AsyncBufReadExt;
        use tokio::io::AsyncWriteExt;
        use tokio::io::BufReader;
    }
}

struct ClientApiConnectionInner {
    comproc: CommandProcessor,
    connect_addr: Option<SocketAddr>,
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
                connect_addr: None,
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

    async fn process_veilid_state<'a>(&self, state: &json::JsonValue) {
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

    async fn process_veilid_update(&self, update: json::JsonValue) {
        let comproc = self.inner.lock().comproc.clone();
        let Some(kind) = update["kind"].as_str() else {
            comproc.log_message(Level::Error, format!("missing update kind: {}", update));
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
                comproc.log_message(Level::Error, format!("unknown update kind: {}", update));
            }
        }
    }

    async fn handle_connection(&self, connect_addr: SocketAddr) -> Result<(), String> {
        trace!("ClientApiConnection::handle_connection");

        // Connect the TCP socket
        let stream = TcpStream::connect(connect_addr)
            .await
            .map_err(map_to_string)?;

        // If it succeed, disable nagle algorithm
        stream.set_nodelay(true).map_err(map_to_string)?;

        // State we connected
        let comproc = self.inner.lock().comproc.clone();
        comproc.set_connection_state(ConnectionState::Connected(connect_addr, SystemTime::now()));

        // Split the stream
        cfg_if! {
            if #[cfg(feature="rt-async-std")] {
                use futures::AsyncReadExt;
                let (reader, mut writer) = stream.split();
                let mut reader = BufReader::new(reader);
            } else if #[cfg(feature="rt-tokio")] {
                let (reader, mut writer) = stream.into_split();
                let mut reader = BufReader::new(reader);
            }
        }

        // Requests to send
        let (requests_tx, requests_rx) = flume::unbounded();

        // Create disconnection mechanism
        let stop_token = {
            let stop_source = StopSource::new();
            let token = stop_source.token();
            let mut inner = self.inner.lock();
            inner.connect_addr = Some(connect_addr);
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
                    this.process_veilid_update(j).await;
                } else if j["type"] == "Response" {
                    this.process_response(j).await;
                }
            }
            //
            let mut inner = this.inner.lock();
            inner.request_sender = None;
        };
        unord.push(system_boxed(recv_messages_future));

        // Requests send processor
        let send_requests_future = async move {
            while let Ok(req) = requests_rx.recv_async().await {
                if let Err(e) = writer.write_all(req.as_bytes()).await {
                    error!("failed to write request: {}", e)
                }
            }
        };
        unord.push(system_boxed(send_requests_future));

        // Request initial server state
        let capi = self.clone();
        spawn_detached_local(async move {
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
            capi.process_veilid_state(&resp["value"]).await;
        });

        // Send and receive until we're done or a stop is requested
        while let Ok(Some(())) = unord.next().timeout_at(stop_token.clone()).await {}

        // // Drop the server and disconnector too (if we still have it)
        let mut inner = self.inner.lock();
        let disconnect_requested = inner.disconnect_requested;
        inner.request_sender = None;
        inner.disconnector = None;
        inner.disconnect_requested = false;
        inner.connect_addr = None;

        // Connection finished
        if disconnect_requested {
            Ok(())
        } else {
            Err("Connection lost".to_owned())
        }
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

    // Start Client API connection
    pub async fn connect(&self, connect_addr: SocketAddr) -> Result<(), String> {
        trace!("ClientApiConnection::connect");
        // Save the address to connect to
        self.handle_connection(connect_addr).await
    }

    // End Client API connection
    pub async fn disconnect(&self) {
        trace!("ClientApiConnection::disconnect");
        let mut inner = self.inner.lock();
        if inner.disconnector.is_some() {
            inner.disconnector = None;
            inner.disconnect_requested = true;
        }
    }
}
