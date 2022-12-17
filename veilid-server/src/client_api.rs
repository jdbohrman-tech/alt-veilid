use crate::settings::*;
use crate::tools::*;
use crate::veilid_client_capnp::*;
use crate::veilid_logs::VeilidLogs;
use capnp::capability::Promise;
use capnp_rpc::{pry, rpc_twoparty_capnp, twoparty, RpcSystem};
use cfg_if::*;
use futures_util::{future::try_join_all, FutureExt as FuturesFutureExt, StreamExt};
use serde::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::net::SocketAddr;
use std::rc::Rc;
use stop_token::future::FutureExt;
use stop_token::*;
use tracing::*;
use veilid_core::*;

// Encoding for ApiResult
fn encode_api_result<T: Serialize + fmt::Debug>(
    result: &Result<T, VeilidAPIError>,
    builder: &mut api_result::Builder,
) {
    match result {
        Ok(v) => {
            builder.set_ok(&serialize_json(v));
        }
        Err(e) => {
            builder.set_err(&serialize_json(e));
        }
    }
}

// --- interface Registration ---------------------------------

struct RegistrationHandle {
    client: veilid_client::Client,
    requests_in_flight: i32,
}

struct RegistrationMap {
    registrations: HashMap<u64, RegistrationHandle>,
}

impl RegistrationMap {
    fn new() -> Self {
        Self {
            registrations: HashMap::new(),
        }
    }
}

struct RegistrationImpl {
    id: u64,
    registration_map: Rc<RefCell<RegistrationMap>>,
}

impl RegistrationImpl {
    fn new(id: u64, registrations: Rc<RefCell<RegistrationMap>>) -> Self {
        Self {
            id,
            registration_map: registrations,
        }
    }
}

impl Drop for RegistrationImpl {
    fn drop(&mut self) {
        debug!("Registration dropped");
        self.registration_map
            .borrow_mut()
            .registrations
            .remove(&self.id);
    }
}

impl registration::Server for RegistrationImpl {}

// --- interface VeilidServer ---------------------------------

struct VeilidServerImpl {
    veilid_api: veilid_core::VeilidAPI,
    veilid_logs: VeilidLogs,
    settings: Settings,
    next_id: u64,
    pub registration_map: Rc<RefCell<RegistrationMap>>,
}

impl VeilidServerImpl {
    #[instrument(level = "trace", skip_all)]
    pub fn new(
        veilid_api: veilid_core::VeilidAPI,
        veilid_logs: VeilidLogs,
        settings: Settings,
    ) -> Self {
        Self {
            next_id: 0,
            registration_map: Rc::new(RefCell::new(RegistrationMap::new())),
            veilid_api,
            veilid_logs,
            settings,
        }
    }
}

impl veilid_server::Server for VeilidServerImpl {
    #[instrument(level = "trace", skip_all)]
    fn register(
        &mut self,
        params: veilid_server::RegisterParams,
        mut results: veilid_server::RegisterResults,
    ) -> Promise<(), ::capnp::Error> {
        trace!("VeilidServerImpl::register");

        self.registration_map.borrow_mut().registrations.insert(
            self.next_id,
            RegistrationHandle {
                client: pry!(pry!(params.get()).get_veilid_client()),
                requests_in_flight: 0,
            },
        );

        let veilid_api = self.veilid_api.clone();
        let settings = self.settings.clone();
        let registration = capnp_rpc::new_client(RegistrationImpl::new(
            self.next_id,
            self.registration_map.clone(),
        ));
        self.next_id += 1;

        Promise::from_future(async move {
            let state = veilid_api
                .get_state()
                .await
                .map_err(|e| ::capnp::Error::failed(format!("{:?}", e)))?;
            let state = serialize_json(state);

            let mut res = results.get();
            res.set_registration(registration);
            res.set_state(&state);

            let settings = &*settings.read();
            let settings_json_string = serialize_json(settings);
            let mut settings_json = json::parse(&settings_json_string)
                .map_err(|e| ::capnp::Error::failed(format!("{:?}", e)))?;
            settings_json["core"]["network"].remove("node_id_secret");
            let safe_settings_json = settings_json.to_string();
            res.set_settings(&safe_settings_json);

            Ok(())
        })
    }

    #[instrument(level = "trace", skip_all)]
    fn debug(
        &mut self,
        params: veilid_server::DebugParams,
        mut results: veilid_server::DebugResults,
    ) -> Promise<(), ::capnp::Error> {
        trace!("VeilidServerImpl::debug");
        let veilid_api = self.veilid_api.clone();
        let command = pry!(pry!(params.get()).get_command()).to_owned();

        Promise::from_future(async move {
            let result = veilid_api.debug(command).await;
            encode_api_result(&result, &mut results.get().init_result());
            Ok(())
        })
    }

    #[instrument(level = "trace", skip_all)]
    fn attach(
        &mut self,
        _params: veilid_server::AttachParams,
        mut results: veilid_server::AttachResults,
    ) -> Promise<(), ::capnp::Error> {
        trace!("VeilidServerImpl::attach");
        let veilid_api = self.veilid_api.clone();
        Promise::from_future(async move {
            let result = veilid_api.attach().await;
            encode_api_result(&result, &mut results.get().init_result());
            Ok(())
        })
    }

    #[instrument(level = "trace", skip_all)]
    fn detach(
        &mut self,
        _params: veilid_server::DetachParams,
        mut results: veilid_server::DetachResults,
    ) -> Promise<(), ::capnp::Error> {
        trace!("VeilidServerImpl::detach");
        let veilid_api = self.veilid_api.clone();
        Promise::from_future(async move {
            let result = veilid_api.detach().await;
            encode_api_result(&result, &mut results.get().init_result());
            Ok(())
        })
    }

    #[instrument(level = "trace", skip_all)]
    fn shutdown(
        &mut self,
        _params: veilid_server::ShutdownParams,
        mut _results: veilid_server::ShutdownResults,
    ) -> Promise<(), ::capnp::Error> {
        trace!("VeilidServerImpl::shutdown");

        cfg_if::cfg_if! {
            if #[cfg(windows)] {
                assert!(false, "write me!");
            }
            else {
                crate::server::shutdown();
            }
        }

        Promise::ok(())
    }

    #[instrument(level = "trace", skip_all)]
    fn get_state(
        &mut self,
        _params: veilid_server::GetStateParams,
        mut results: veilid_server::GetStateResults,
    ) -> Promise<(), ::capnp::Error> {
        trace!("VeilidServerImpl::get_state");
        let veilid_api = self.veilid_api.clone();
        Promise::from_future(async move {
            let result = veilid_api.get_state().await;
            encode_api_result(&result, &mut results.get().init_result());
            Ok(())
        })
    }

    #[instrument(level = "trace", skip_all)]
    fn change_log_level(
        &mut self,
        params: veilid_server::ChangeLogLevelParams,
        mut results: veilid_server::ChangeLogLevelResults,
    ) -> Promise<(), ::capnp::Error> {
        trace!("VeilidServerImpl::change_log_level");

        let layer = pry!(pry!(params.get()).get_layer()).to_owned();
        let log_level_json = pry!(pry!(params.get()).get_log_level()).to_owned();
        let log_level: veilid_core::VeilidConfigLogLevel =
            pry!(veilid_core::deserialize_json(&log_level_json)
                .map_err(|e| ::capnp::Error::failed(format!("{:?}", e))));

        let result = self.veilid_logs.change_log_level(layer, log_level);
        encode_api_result(&result, &mut results.get().init_result());
        Promise::ok(())
    }

    #[instrument(level = "trace", skip_all)]
    fn app_call_reply(
        &mut self,
        params: veilid_server::AppCallReplyParams,
        mut results: veilid_server::AppCallReplyResults,
    ) -> Promise<(), ::capnp::Error> {
        trace!("VeilidServerImpl::app_call_reply");

        let id = OperationId::new(pry!(params.get()).get_id());
        let message = pry!(pry!(params.get()).get_message()).to_owned();

        let veilid_api = self.veilid_api.clone();
        Promise::from_future(async move {
            let result = veilid_api.app_call_reply(id, message).await;
            encode_api_result(&result, &mut results.get().init_result());
            Ok(())
        })
    }
}

// --- Client API Server-Side ---------------------------------

type ClientApiAllFuturesJoinHandle =
    JoinHandle<Result<Vec<()>, Box<(dyn std::error::Error + 'static)>>>;

struct ClientApiInner {
    veilid_api: veilid_core::VeilidAPI,
    veilid_logs: VeilidLogs,
    settings: Settings,
    registration_map: Rc<RefCell<RegistrationMap>>,
    stop: Option<StopSource>,
    join_handle: Option<ClientApiAllFuturesJoinHandle>,
}

pub struct ClientApi {
    inner: RefCell<ClientApiInner>,
}

impl ClientApi {
    #[instrument(level = "trace", skip_all)]
    pub fn new(
        veilid_api: veilid_core::VeilidAPI,
        veilid_logs: VeilidLogs,
        settings: Settings,
    ) -> Rc<Self> {
        Rc::new(Self {
            inner: RefCell::new(ClientApiInner {
                veilid_api,
                veilid_logs,
                settings,
                registration_map: Rc::new(RefCell::new(RegistrationMap::new())),
                stop: Some(StopSource::new()),
                join_handle: None,
            }),
        })
    }

    #[instrument(level = "trace", skip(self))]
    pub async fn stop(self: Rc<Self>) {
        trace!("ClientApi::stop requested");
        let jh = {
            let mut inner = self.inner.borrow_mut();
            if inner.join_handle.is_none() {
                trace!("ClientApi stop ignored");
                return;
            }
            drop(inner.stop.take());
            inner.join_handle.take().unwrap()
        };
        trace!("ClientApi::stop: waiting for stop");
        if let Err(err) = jh.await {
            error!("{}", err);
        }
        trace!("ClientApi::stop: stopped");
    }

    #[instrument(level = "trace", skip(self, client), err)]
    async fn handle_incoming(
        self: Rc<Self>,
        bind_addr: SocketAddr,
        client: veilid_server::Client,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(bind_addr).await?;
        debug!("Client API listening on: {:?}", bind_addr);

        // Process the incoming accept stream
        cfg_if! {
            if #[cfg(feature="rt-async-std")] {
                let mut incoming_stream = listener.incoming();
            } else if #[cfg(feature="rt-tokio")] {
                let mut incoming_stream = tokio_stream::wrappers::TcpListenerStream::new(listener);
            }
        }

        let stop_token = self.inner.borrow().stop.as_ref().unwrap().token();
        let incoming_loop = async move {
            while let Ok(Some(stream_result)) =
                incoming_stream.next().timeout_at(stop_token.clone()).await
            {
                let stream = stream_result?;
                stream.set_nodelay(true)?;
                cfg_if! {
                    if #[cfg(feature="rt-async-std")] {
                        use futures_util::AsyncReadExt;
                        let (reader, writer) = stream.split();
                    } else if #[cfg(feature="rt-tokio")] {
                        use tokio_util::compat::*;
                        let (reader, writer) = stream.into_split();
                        let reader = reader.compat();
                        let writer = writer.compat_write();
                    }
                }
                let network = twoparty::VatNetwork::new(
                    reader,
                    writer,
                    rpc_twoparty_capnp::Side::Server,
                    Default::default(),
                );

                let rpc_system = RpcSystem::new(Box::new(network), Some(client.clone().client));

                spawn_local(rpc_system.map(drop));
            }
            Ok::<(), Box<dyn std::error::Error>>(())
        };

        incoming_loop.await
    }

    #[instrument(level = "trace", skip_all)]
    fn send_request_to_all_clients<F, T>(self: Rc<Self>, request: F)
    where
        F: Fn(u64, &mut RegistrationHandle) -> Option<::capnp::capability::RemotePromise<T>>,
        T: capnp::traits::Pipelined + capnp::traits::Owned + 'static + Unpin,
    {
        // Send status update to each registered client
        let registration_map = self.inner.borrow().registration_map.clone();
        let registration_map1 = registration_map.clone();
        let regs = &mut registration_map.borrow_mut().registrations;
        for (&id, mut registration) in regs.iter_mut() {
            if registration.requests_in_flight >= 256 {
                println!(
                    "too many requests in flight: {}",
                    registration.requests_in_flight
                );
            }
            registration.requests_in_flight += 1;

            if let Some(request_promise) = request(id, registration) {
                let registration_map2 = registration_map1.clone();
                spawn_local(request_promise.promise.map(move |r| match r {
                    Ok(_) => {
                        if let Some(ref mut s) =
                            registration_map2.borrow_mut().registrations.get_mut(&id)
                        {
                            s.requests_in_flight -= 1;
                        }
                    }
                    Err(e) => {
                        println!("Got error: {:?}. Dropping registation.", e);
                        registration_map2.borrow_mut().registrations.remove(&id);
                    }
                }));
            }
        }
    }

    #[instrument(level = "trace", skip(self))]
    pub fn handle_update(self: Rc<Self>, veilid_update: veilid_core::VeilidUpdate) {
        // serialize update
        let veilid_update = serialize_json(veilid_update);

        // Pass other updates to clients
        self.send_request_to_all_clients(|_id, registration| {
            match veilid_update
                .len()
                .try_into()
                .map_err(|e| ::capnp::Error::failed(format!("{:?}", e)))
            {
                Ok(len) => {
                    let mut request = registration.client.update_request();
                    let mut rpc_veilid_update = request.get().init_veilid_update(len);
                    rpc_veilid_update.push_str(&veilid_update);
                    Some(request.send())
                }
                Err(_) => None,
            }
        });
    }

    #[instrument(level = "trace", skip(self))]
    pub fn run(self: Rc<Self>, bind_addrs: Vec<SocketAddr>) {
        // Create client api VeilidServer
        let veilid_server_impl = VeilidServerImpl::new(
            self.inner.borrow().veilid_api.clone(),
            self.inner.borrow().veilid_logs.clone(),
            self.inner.borrow().settings.clone(),
        );
        self.inner.borrow_mut().registration_map = veilid_server_impl.registration_map.clone();

        // Make a client object for the server to send to each rpc client
        let client: veilid_server::Client = capnp_rpc::new_client(veilid_server_impl);

        let bind_futures = bind_addrs
            .iter()
            .map(|addr| self.clone().handle_incoming(*addr, client.clone()));
        let bind_futures_join = try_join_all(bind_futures);
        self.inner.borrow_mut().join_handle = Some(spawn_local(bind_futures_join));
    }
}
