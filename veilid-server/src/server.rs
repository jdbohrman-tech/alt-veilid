use crate::client_api;
use crate::settings::*;
use crate::tools::*;
use crate::veilid_logs::*;
use flume::{unbounded, Receiver, Sender};
use futures_util::select;
use futures_util::FutureExt;
use lazy_static::*;
use parking_lot::Mutex;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tracing::*;
use veilid_core::tools::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ServerMode {
    Normal,
    ShutdownImmediate,
    DumpTXTRecord,
}

lazy_static! {
    static ref SHUTDOWN_SWITCH: Mutex<Option<SingleShotEventual<()>>> =
        Mutex::new(Some(SingleShotEventual::new(Some(()))));
}

#[instrument]
pub fn shutdown() {
    let shutdown_switch = SHUTDOWN_SWITCH.lock().take();
    if let Some(shutdown_switch) = shutdown_switch {
        shutdown_switch.resolve(());
    }
}

//#[instrument(err, skip_all)]
pub async fn run_veilid_server(
    settings: Settings,
    server_mode: ServerMode,
    veilid_logs: VeilidLogs,
) -> EyreResult<()> {
    let (
        settings_auto_attach,
        settings_client_api_ipc_enabled,
        settings_client_api_network_enabled,
        settings_client_api_ipc_directory,
        settings_client_api_listen_address_addrs,
        subnode_index,
    ) = {
        let settingsr = settings.read();

        (
            settingsr.auto_attach,
            settingsr.client_api.ipc_enabled,
            settingsr.client_api.network_enabled,
            settingsr.client_api.ipc_directory.clone(),
            settingsr.client_api.listen_address.addrs.clone(),
            settingsr.testing.subnode_index,
        )
    };

    // Create client api state change pipe
    let (sender, receiver): (
        Sender<veilid_core::VeilidUpdate>,
        Receiver<veilid_core::VeilidUpdate>,
    ) = unbounded();

    // Create VeilidCore setup
    let update_callback = Arc::new(move |change: veilid_core::VeilidUpdate| {
        if let Err(e) = sender.send(change) {
            // Don't log here, as that loops the update callback in some cases and will deadlock
            let change = e.into_inner();
            eprintln!("error sending veilid update callback: {:?}", change);
        }
    });
    let config_callback = settings.get_core_config_callback();

    // Start Veilid Core and get API
    let veilid_api = veilid_core::api_startup(update_callback, config_callback)
        .await
        .wrap_err("VeilidCore startup failed")?;

    // Start client api if one is requested
    let capi_enabled = settings_client_api_ipc_enabled || settings_client_api_network_enabled;
    let mut capi = if capi_enabled && matches!(server_mode, ServerMode::Normal) {
        let some_capi =
            client_api::ClientApi::new(veilid_api.clone(), veilid_logs.clone(), settings.clone());
        some_capi.clone().run(
            if settings_client_api_ipc_enabled {
                Some(settings_client_api_ipc_directory.join(subnode_index.to_string()))
            } else {
                None
            },
            if settings_client_api_network_enabled {
                settings_client_api_listen_address_addrs
            } else {
                vec![]
            },
        );
        Some(some_capi)
    } else {
        None
    };

    // Drop rwlock on settings
    let auto_attach = settings_auto_attach || !matches!(server_mode, ServerMode::Normal);

    // Process all updates
    let capi2 = capi.clone();
    let update_receiver_shutdown = SingleShotEventual::new(Some(()));
    let mut update_receiver_shutdown_instance = update_receiver_shutdown.instance().fuse();
    let update_receiver_jh = spawn_local(async move {
        loop {
            select! {
                res = receiver.recv_async() => {
                    if let Ok(change) = res {
                        if let Some(capi) = &capi2 {
                            // Handle state changes on main thread for capnproto rpc
                            capi.clone().handle_update(change);
                        }
                    } else {
                        break;
                    }
                }
                _ = update_receiver_shutdown_instance => {
                    break;
                }
            };
        }
    });

    // Auto-attach if desired
    let mut out = Ok(());
    if auto_attach {
        info!("Auto-attach to the Veilid network");
        if let Err(e) = veilid_api.attach().await {
            out = Err(eyre!(
                "Auto-attaching to the Veilid network failed: {:?}",
                e
            ));
            shutdown();
        }
    }

    // Process dump-txt-record
    if matches!(server_mode, ServerMode::DumpTXTRecord) {
        let start_time = Instant::now();
        while Instant::now().duration_since(start_time) < Duration::from_secs(10) {
            match veilid_api.get_state().await {
                Ok(vs) => {
                    if vs.network.started {
                        break;
                    }
                }
                Err(e) => {
                    out = Err(eyre!("Getting state failed: {:?}", e));
                    break;
                }
            }
            sleep(100).await;
        }
        match veilid_api.debug("txtrecord".to_string()).await {
            Ok(v) => {
                print!("{}", v);
            }
            Err(e) => {
                out = Err(eyre!("Getting TXT record failed: {:?}", e));
            }
        };
        shutdown();
    }

    // Process shutdown-immediate
    if matches!(server_mode, ServerMode::ShutdownImmediate) {
        shutdown();
    }

    // Idle while waiting to exit
    let shutdown_switch = {
        let shutdown_switch_locked = SHUTDOWN_SWITCH.lock();
        (*shutdown_switch_locked).as_ref().map(|ss| ss.instance())
    };
    if let Some(shutdown_switch) = shutdown_switch {
        shutdown_switch.await;
    }

    // Stop the client api if we have one
    if let Some(c) = capi.as_mut().cloned() {
        c.stop().await;
    }

    // Shut down Veilid API to release state change sender
    veilid_api.shutdown().await;

    // Shut down update receiver now that there are no more updates
    update_receiver_shutdown.resolve(()).await;

    // Wait for update receiver to exit
    let _ = update_receiver_jh.await;

    // Finally, drop logs
    // this is explicit to ensure we don't accidentally drop them too soon via a move
    drop(veilid_logs);

    out
}
