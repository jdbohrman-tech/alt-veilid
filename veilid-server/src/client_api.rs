use crate::settings::*;
use crate::tools::*;
use crate::veilid_client_capnp::*;
use crate::veilid_logs::VeilidLogs;
use cfg_if::*;
use futures_util::{future::try_join_all, FutureExt as FuturesFutureExt, StreamExt};
use serde::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::net::SocketAddr;
use stop_token::future::FutureExt;
use stop_token::*;
use tracing::*;
use veilid_core::*;

// struct VeilidServerImpl {
//     veilid_api: veilid_core::VeilidAPI,
//     veilid_logs: VeilidLogs,
//     settings: Settings,
//     next_id: u64,
// }

// impl VeilidServerImpl {
//     #[instrument(level = "trace", skip_all)]
//     pub fn new(
//         veilid_api: veilid_core::VeilidAPI,
//         veilid_logs: VeilidLogs,
//         settings: Settings,
//     ) -> Self {
//         Self {
//             next_id: 0,
//             veilid_api,
//             veilid_logs,
//             settings,
//         }
//     }

//     #[instrument(level = "trace", skip_all)]
//     fn shutdown(
//         &mut self,
//         _params: veilid_server::ShutdownParams,
//         mut _results: veilid_server::ShutdownResults,
//     ) -> Promise<(), ::capnp::Error> {
//         trace!("VeilidServerImpl::shutdown");

//         cfg_if::cfg_if! {
//             if #[cfg(windows)] {
//                 assert!(false, "write me!");
//             }
//             else {
//                 crate::server::shutdown();
//             }
//         }

//         Promise::ok(())
//     }

//     #[instrument(level = "trace", skip_all)]
//     fn change_log_level(
//         &mut self,
//         params: veilid_server::ChangeLogLevelParams,
//         mut results: veilid_server::ChangeLogLevelResults,
//     ) -> Promise<(), ::capnp::Error> {
//         trace!("VeilidServerImpl::change_log_level");

//         let layer = pry!(pry!(params.get()).get_layer()).to_owned();
//         let log_level_json = pry!(pry!(params.get()).get_log_level()).to_owned();
//         let log_level: veilid_core::VeilidConfigLogLevel =
//             pry!(veilid_core::deserialize_json(&log_level_json)
//                 .map_err(|e| ::capnp::Error::failed(format!("{:?}", e))));

//         let result = self.veilid_logs.change_log_level(layer, log_level);
//         encode_api_result(&result, &mut results.get().init_result());
//         Promise::ok(())
//     }
// }

// --- Client API Server-Side ---------------------------------

type ClientApiAllFuturesJoinHandle =
    JoinHandle<Result<Vec<()>, Box<(dyn std::error::Error + 'static)>>>;

struct ClientApiInner {
    veilid_api: veilid_core::VeilidAPI,
    veilid_logs: VeilidLogs,
    settings: Settings,
    stop: Option<StopSource>,
    join_handle: Option<ClientApiAllFuturesJoinHandle>,
}

pub struct ClientApi {
    inner: Arc<Mutex<ClientApiInner>>,
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

    #[instrument(level = "trace", skip(self), err)]
    async fn handle_incoming(
        self,
        bind_addr: SocketAddr,
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

        let stop_token = self.inner.lock().stop.as_ref().unwrap().token();
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

                xxx spawn json_api handler
                spawn_local(rpc_system.map(drop));
            }
            Ok::<(), Box<dyn std::error::Error>>(())
        };

        incoming_loop.await
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
    pub fn run(&self, bind_addrs: Vec<SocketAddr>) {
        let bind_futures = bind_addrs
            .iter()
            .map(|addr| self.clone().handle_incoming(*addr));
        let bind_futures_join = try_join_all(bind_futures);
        self.inner.borrow_mut().join_handle = Some(spawn_local(bind_futures_join));
    }
}
