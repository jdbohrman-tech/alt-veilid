use crate::command_processor::*;
use crate::tools::*;
use core::str::FromStr;
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
    server_settings: Option<String>,
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
                server_settings: None,
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

    // async fn process_veilid_state<'a>(
    //     &'a mut self,
    //     veilid_state: VeilidState,
    // ) -> Result<(), String> {
    //     let mut inner = self.inner.borrow_mut();
    //     inner.comproc.update_attachment(veilid_state.attachment);
    //     inner.comproc.update_network_status(veilid_state.network);
    //     inner.comproc.update_config(veilid_state.config);
    //     Ok(())
    // }

    async fn process_response(&self, response: json::JsonValue) {
        // find the operation id and send the response to the channel for it
        let Some(id_str) = response["id"].as_str() else {
            error!("missing id: {}", response);
            return;
        };
        let Ok(id) = u32::from_str(id_str) else {
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
            return;
        }
    }

    async fn process_update(&self, update: json::JsonValue) {
        let comproc = self.inner.lock().comproc.clone();
        let Some(kind) = update["kind"].as_str() else {
            comproc.log_message(format!("missing update kind: {}", update));
            return;
        };
        match kind {
            "Log" => {
                comproc.update_log(update);
            }
            "AppMessage" => {
                comproc.update_app_message(update);
            }
            "AppCall" => {
                comproc.update_app_call(update);
            }
            "Attachment" => {
                comproc.update_attachment(update);
            }
            "Network" => {
                comproc.update_network_status(update);
            }
            "Config" => {
                comproc.update_config(update);
            }
            "RouteChange" => {
                comproc.update_route(update);
            }
            "Shutdown" => comproc.update_shutdown(),
            "ValueChange" => {
                comproc.update_value_change(update);
            }
            _ => {
                comproc.log_message(format!("unknown update kind: {}", update));
            }
        }
    }

    // async fn spawn_rpc_system(
    //     &self,
    //     connect_addr: SocketAddr,
    //     mut rpc_system: RpcSystem<rpc_twoparty_capnp::Side>,
    // ) -> Result<(), String> {
    //     let mut request;
    //     {
    //         let mut inner = self.inner.borrow_mut();

    //         // Get the bootstrap server connection object
    //         inner.server = Some(Rc::new(RefCell::new(
    //             rpc_system.bootstrap(rpc_twoparty_capnp::Side::Server),
    //         )));

    //         // Store our disconnector future for later (must happen after bootstrap, contrary to documentation)
    //         inner.disconnector = Some(rpc_system.get_disconnector());

    //         // Get a client object to pass to the server for status update callbacks
    //         let client = capnp_rpc::new_client(VeilidClientImpl::new(inner.comproc.clone()));

    //         // Register our client and get a registration object back
    //         request = inner
    //             .server
    //             .as_ref()
    //             .unwrap()
    //             .borrow_mut()
    //             .register_request();
    //         request.get().set_veilid_client(client);

    //         inner
    //             .comproc
    //             .set_connection_state(ConnectionState::Connected(
    //                 connect_addr,
    //                 std::time::SystemTime::now(),
    //             ));
    //     }

    //     let rpc_jh = spawn_local(rpc_system);

    //     let reg_res: Result<registration::Client, String> = (async {
    //         // Send the request and get the state object and the registration object
    //         let response = request
    //             .send()
    //             .promise
    //             .await
    //             .map_err(|e| format!("failed to send register request: {}", e))?;
    //         let response = response
    //             .get()
    //             .map_err(|e| format!("failed to get register response: {}", e))?;

    //         // Get the registration object, which drops our connection when it is dropped
    //         let registration = response
    //             .get_registration()
    //             .map_err(|e| format!("failed to get registration object: {}", e))?;

    //         // Get the initial veilid state
    //         let veilid_state = response
    //             .get_state()
    //             .map_err(|e| format!("failed to get initial veilid state: {}", e))?;

    //         // Set up our state for the first time
    //         let veilid_state: VeilidState = deserialize_json(veilid_state)
    //             .map_err(|e| format!("failed to get deserialize veilid state: {}", e))?;
    //         self.process_veilid_state(veilid_state).await?;

    //         // Save server settings
    //         let server_settings = response
    //             .get_settings()
    //             .map_err(|e| format!("failed to get initial veilid server settings: {}", e))?
    //             .to_owned();
    //         self.inner.borrow_mut().server_settings = Some(server_settings.clone());

    //         // Don't drop the registration, doing so will remove the client
    //         // object mapping from the server which we need for the update backchannel
    //         Ok(registration)
    //     })
    //     .await;

    //     let _registration = match reg_res {
    //         Ok(v) => v,
    //         Err(e) => {
    //             rpc_jh.abort().await;
    //             return Err(e);
    //         }
    //     };

    //     // Wait until rpc system completion or disconnect was requested
    //     let res = rpc_jh.await;
    //     res.map_err(|e| format!("client RPC system error: {}", e))
    // }

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
            let mut line = String::new();
            while let Ok(size) = reader.read_line(&mut line).await {
                // Exit on EOF
                if size == 0 {
                    // Disconnected
                    break;
                }

                // Unmarshal json
                let j = match json::parse(line.trim()) {
                    Ok(v) => v,
                    Err(e) => {
                        error!("failed to parse server response: {}", e);
                        continue;
                    }
                };

                if j["type"] == "Update" {
                    this.process_update(j).await;
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

        // Send and receive until we're done or a stop is requested
        while let Ok(Some(())) = unord.next().timeout_at(stop_token.clone()).await {}

        // // Drop the server and disconnector too (if we still have it)
        let mut inner = self.inner.lock();
        let disconnect_requested = inner.disconnect_requested;
        inner.server_settings = None;
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

    pub async fn server_appcall_reply(&self, id: u64, msg: Vec<u8>) -> Result<(), String> {
        trace!("ClientApiConnection::appcall_reply");
        let mut req = json::JsonValue::new_object();
        req["op"] = "AppCallReply".into();
        req["call_id"] = id.to_string().into();
        req["message"] = data_encoding::BASE64URL_NOPAD.encode(&msg).into();
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
