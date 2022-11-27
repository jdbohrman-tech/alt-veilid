use crate::command_processor::*;
use crate::tools::*;
use crate::veilid_client_capnp::*;
use capnp::capability::Promise;
use capnp_rpc::{pry, rpc_twoparty_capnp, twoparty, Disconnector, RpcSystem};
use futures::future::FutureExt;
use serde::de::DeserializeOwned;
use std::cell::RefCell;
use std::net::SocketAddr;
use std::rc::Rc;
use veilid_core::xx::*;
use veilid_core::*;

macro_rules! capnp_failed {
    ($ex:expr) => {{
        let msg = format!("Capnp Error: {}", $ex);
        error!("{}", msg);
        Promise::err(capnp::Error::failed(msg))
    }};
}

macro_rules! pry_result {
    ($ex:expr) => {
        match $ex {
            Ok(v) => v,
            Err(e) => {
                return capnp_failed!(e);
            }
        }
    };
}

fn map_to_internal_error<T: ToString>(e: T) -> VeilidAPIError {
    VeilidAPIError::Internal {
        message: e.to_string(),
    }
}

fn decode_api_result<T: DeserializeOwned + fmt::Debug>(
    reader: &api_result::Reader,
) -> Result<T, VeilidAPIError> {
    match reader.which().map_err(map_to_internal_error)? {
        api_result::Which::Ok(v) => {
            let ok_val = v.map_err(map_to_internal_error)?;
            let res: T = veilid_core::deserialize_json(ok_val).map_err(map_to_internal_error)?;
            Ok(res)
        }
        api_result::Which::Err(e) => {
            let err_val = e.map_err(map_to_internal_error)?;
            let res: VeilidAPIError =
                veilid_core::deserialize_json(err_val).map_err(map_to_internal_error)?;
            Err(res)
        }
    }
}

struct VeilidClientImpl {
    comproc: CommandProcessor,
}

impl VeilidClientImpl {
    pub fn new(comproc: CommandProcessor) -> Self {
        Self { comproc }
    }
}

impl veilid_client::Server for VeilidClientImpl {
    fn update(
        &mut self,
        params: veilid_client::UpdateParams,
        _results: veilid_client::UpdateResults,
    ) -> Promise<(), ::capnp::Error> {
        let veilid_update = pry!(pry!(params.get()).get_veilid_update());
        let veilid_update: VeilidUpdate = pry_result!(deserialize_json(veilid_update));

        match veilid_update {
            VeilidUpdate::Log(log) => {
                self.comproc.update_log(log);
            }
            VeilidUpdate::AppMessage(msg) => {
                self.comproc.update_app_message(msg);
            }
            VeilidUpdate::AppCall(call) => {
                self.comproc.update_app_call(call);
            }
            VeilidUpdate::Attachment(attachment) => {
                self.comproc.update_attachment(attachment);
            }
            VeilidUpdate::Network(network) => {
                self.comproc.update_network_status(network);
            }
            VeilidUpdate::Config(config) => {
                self.comproc.update_config(config);
            }
            VeilidUpdate::Route(route) => {
                self.comproc.update_route(route);
            }
            VeilidUpdate::Shutdown => self.comproc.update_shutdown(),
        }

        Promise::ok(())
    }
}

struct ClientApiConnectionInner {
    comproc: CommandProcessor,
    connect_addr: Option<SocketAddr>,
    disconnector: Option<Disconnector<rpc_twoparty_capnp::Side>>,
    server: Option<Rc<RefCell<veilid_server::Client>>>,
    server_settings: Option<String>,
    disconnect_requested: bool,
    cancel_eventual: Eventual,
}

type Handle<T> = Rc<RefCell<T>>;

#[derive(Clone)]
pub struct ClientApiConnection {
    inner: Handle<ClientApiConnectionInner>,
}

impl ClientApiConnection {
    pub fn new(comproc: CommandProcessor) -> Self {
        Self {
            inner: Rc::new(RefCell::new(ClientApiConnectionInner {
                comproc,
                connect_addr: None,
                disconnector: None,
                server: None,
                server_settings: None,
                disconnect_requested: false,
                cancel_eventual: Eventual::new(),
            })),
        }
    }

    pub fn cancel(&self) {
        let eventual = {
            let inner = self.inner.borrow();
            inner.cancel_eventual.clone()
        };
        eventual.resolve(); // don't need to await this
    }

    async fn process_veilid_state<'a>(
        &'a mut self,
        veilid_state: VeilidState,
    ) -> Result<(), String> {
        let mut inner = self.inner.borrow_mut();
        inner.comproc.update_attachment(veilid_state.attachment);
        inner.comproc.update_network_status(veilid_state.network);
        inner.comproc.update_config(veilid_state.config);
        Ok(())
    }

    async fn spawn_rpc_system(
        &mut self,
        connect_addr: SocketAddr,
        mut rpc_system: RpcSystem<rpc_twoparty_capnp::Side>,
    ) -> Result<(), String> {
        let mut request;
        {
            let mut inner = self.inner.borrow_mut();

            // Get the bootstrap server connection object
            inner.server = Some(Rc::new(RefCell::new(
                rpc_system.bootstrap(rpc_twoparty_capnp::Side::Server),
            )));

            // Store our disconnector future for later (must happen after bootstrap, contrary to documentation)
            inner.disconnector = Some(rpc_system.get_disconnector());

            // Get a client object to pass to the server for status update callbacks
            let client = capnp_rpc::new_client(VeilidClientImpl::new(inner.comproc.clone()));

            // Register our client and get a registration object back
            request = inner
                .server
                .as_ref()
                .unwrap()
                .borrow_mut()
                .register_request();
            request.get().set_veilid_client(client);

            inner
                .comproc
                .set_connection_state(ConnectionState::Connected(
                    connect_addr,
                    std::time::SystemTime::now(),
                ));
        }

        let rpc_jh = spawn_local(rpc_system);

        // Send the request and get the state object and the registration object
        let response = request
            .send()
            .promise
            .await
            .map_err(|e| format!("failed to send register request: {}", e))?;
        let response = response
            .get()
            .map_err(|e| format!("failed to get register response: {}", e))?;

        // Get the registration object, which drops our connection when it is dropped
        let _registration = response
            .get_registration()
            .map_err(|e| format!("failed to get registration object: {}", e))?;

        // Get the initial veilid state
        let veilid_state = response
            .get_state()
            .map_err(|e| format!("failed to get initial veilid state: {}", e))?;

        // Set up our state for the first time
        let veilid_state: VeilidState = deserialize_json(veilid_state)
            .map_err(|e| format!("failed to get deserialize veilid state: {}", e))?;
        self.process_veilid_state(veilid_state).await?;

        // Save server settings
        let server_settings = response
            .get_settings()
            .map_err(|e| format!("failed to get initial veilid server settings: {}", e))?
            .to_owned();
        self.inner.borrow_mut().server_settings = Some(server_settings.clone());

        // Don't drop the registration, doing so will remove the client
        // object mapping from the server which we need for the update backchannel

        // Wait until rpc system completion or disconnect was requested
        let res = rpc_jh.await;
        // #[cfg(feature = "rt-tokio")]
        // let res = res.map_err(|e| format!("join error: {}", e))?;
        res.map_err(|e| format!("client RPC system error: {}", e))
    }

    async fn handle_connection(&mut self, connect_addr: SocketAddr) -> Result<(), String> {
        trace!("ClientApiConnection::handle_connection");

        self.inner.borrow_mut().connect_addr = Some(connect_addr);
        // Connect the TCP socket
        let stream = TcpStream::connect(connect_addr)
            .await
            .map_err(map_to_string)?;
        // If it succeed, disable nagle algorithm
        stream.set_nodelay(true).map_err(map_to_string)?;

        // Create the VAT network
        cfg_if! {
            if #[cfg(feature="rt-async-std")] {
                use futures::AsyncReadExt;
                let (reader, writer) = stream.split();
            } else if #[cfg(feature="rt-tokio")] {
                pub use tokio_util::compat::*;
                let (reader, writer) = stream.into_split();
                let reader = reader.compat();
                let writer = writer.compat_write();
            }
        }

        let rpc_network = Box::new(twoparty::VatNetwork::new(
            reader,
            writer,
            rpc_twoparty_capnp::Side::Client,
            Default::default(),
        ));

        // Create the rpc system
        let rpc_system = RpcSystem::new(rpc_network, None);

        // Process the rpc system until we decide we're done
        match self.spawn_rpc_system(connect_addr, rpc_system).await {
            Ok(()) => {}
            Err(e) => {
                error!("Failed to spawn client RPC system: {}", e);
            }
        }

        // Drop the server and disconnector too (if we still have it)
        let mut inner = self.inner.borrow_mut();
        let disconnect_requested = inner.disconnect_requested;
        inner.server_settings = None;
        inner.server = None;
        inner.disconnector = None;
        inner.disconnect_requested = false;
        inner.connect_addr = None;

        if !disconnect_requested {
            // Connection lost
            Err("Connection lost".to_owned())
        } else {
            // Connection finished
            Ok(())
        }
    }

    pub fn cancellable<T>(&mut self, p: Promise<T, capnp::Error>) -> Promise<T, capnp::Error>
    where
        T: 'static,
    {
        let (mut cancel_instance, cancel_eventual) = {
            let inner = self.inner.borrow();
            (
                inner.cancel_eventual.instance_empty().fuse(),
                inner.cancel_eventual.clone(),
            )
        };
        let mut p = p.fuse();

        Promise::from_future(async move {
            let out = select! {
                a = p => {
                    a
                },
                _ = cancel_instance => {
                    Err(capnp::Error::failed("cancelled".into()))
                }
            };
            drop(cancel_instance);
            cancel_eventual.reset();
            out
        })
    }

    pub async fn server_attach(&mut self) -> Result<(), String> {
        trace!("ClientApiConnection::server_attach");
        let server = {
            let inner = self.inner.borrow();
            inner
                .server
                .as_ref()
                .ok_or_else(|| "Not connected, ignoring attach request".to_owned())?
                .clone()
        };
        let request = server.borrow().attach_request();
        let response = self
            .cancellable(request.send().promise)
            .await
            .map_err(map_to_string)?;
        let reader = response
            .get()
            .map_err(map_to_string)?
            .get_result()
            .map_err(map_to_string)?;
        let res: Result<(), VeilidAPIError> = decode_api_result(&reader);
        res.map_err(map_to_string)
    }

    pub async fn server_detach(&mut self) -> Result<(), String> {
        trace!("ClientApiConnection::server_detach");
        let server = {
            let inner = self.inner.borrow();
            inner
                .server
                .as_ref()
                .ok_or_else(|| "Not connected, ignoring detach request".to_owned())?
                .clone()
        };
        let request = server.borrow().detach_request();
        let response = self
            .cancellable(request.send().promise)
            .await
            .map_err(map_to_string)?;
        let reader = response
            .get()
            .map_err(map_to_string)?
            .get_result()
            .map_err(map_to_string)?;
        let res: Result<(), VeilidAPIError> = decode_api_result(&reader);
        res.map_err(map_to_string)
    }

    pub async fn server_shutdown(&mut self) -> Result<(), String> {
        trace!("ClientApiConnection::server_shutdown");
        let server = {
            let inner = self.inner.borrow();
            inner
                .server
                .as_ref()
                .ok_or_else(|| "Not connected, ignoring attach request".to_owned())?
                .clone()
        };
        let request = server.borrow().shutdown_request();
        let response = self
            .cancellable(request.send().promise)
            .await
            .map_err(map_to_string)?;
        response.get().map(drop).map_err(map_to_string)
    }

    pub async fn server_debug(&mut self, what: String) -> Result<String, String> {
        trace!("ClientApiConnection::server_debug");
        let server = {
            let inner = self.inner.borrow();
            inner
                .server
                .as_ref()
                .ok_or_else(|| "Not connected, ignoring debug request".to_owned())?
                .clone()
        };
        let mut request = server.borrow().debug_request();
        request.get().set_command(&what);
        let response = self
            .cancellable(request.send().promise)
            .await
            .map_err(map_to_string)?;
        let reader = response
            .get()
            .map_err(map_to_string)?
            .get_result()
            .map_err(map_to_string)?;
        let res: Result<String, VeilidAPIError> = decode_api_result(&reader);
        res.map_err(map_to_string)
    }

    pub async fn server_change_log_level(
        &mut self,
        layer: String,
        log_level: VeilidConfigLogLevel,
    ) -> Result<(), String> {
        trace!("ClientApiConnection::change_log_level");
        let server = {
            let inner = self.inner.borrow();
            inner
                .server
                .as_ref()
                .ok_or_else(|| "Not connected, ignoring change_log_level request".to_owned())?
                .clone()
        };
        let mut request = server.borrow().change_log_level_request();
        request.get().set_layer(&layer);
        let log_level_json = veilid_core::serialize_json(&log_level);
        request.get().set_log_level(&log_level_json);
        let response = self
            .cancellable(request.send().promise)
            .await
            .map_err(map_to_string)?;
        let reader = response
            .get()
            .map_err(map_to_string)?
            .get_result()
            .map_err(map_to_string)?;
        let res: Result<(), VeilidAPIError> = decode_api_result(&reader);
        res.map_err(map_to_string)
    }

    pub async fn server_appcall_reply(&mut self, id: u64, msg: Vec<u8>) -> Result<(), String> {
        trace!("ClientApiConnection::appcall_reply");
        let server = {
            let inner = self.inner.borrow();
            inner
                .server
                .as_ref()
                .ok_or_else(|| "Not connected, ignoring change_log_level request".to_owned())?
                .clone()
        };
        let mut request = server.borrow().app_call_reply_request();
        request.get().set_id(id);
        request.get().set_message(&msg);
        let response = self
            .cancellable(request.send().promise)
            .await
            .map_err(map_to_string)?;
        let reader = response
            .get()
            .map_err(map_to_string)?
            .get_result()
            .map_err(map_to_string)?;
        let res: Result<(), VeilidAPIError> = decode_api_result(&reader);
        res.map_err(map_to_string)
    }

    // Start Client API connection
    pub async fn connect(&mut self, connect_addr: SocketAddr) -> Result<(), String> {
        trace!("ClientApiConnection::connect");
        // Save the address to connect to
        self.handle_connection(connect_addr).await
    }

    // End Client API connection
    pub async fn disconnect(&mut self) {
        trace!("ClientApiConnection::disconnect");
        let disconnector = self.inner.borrow_mut().disconnector.take();
        match disconnector {
            Some(d) => {
                self.inner.borrow_mut().disconnect_requested = true;
                d.await.unwrap();
            }
            None => {
                debug!("disconnector doesn't exist");
            }
        }
    }
}
