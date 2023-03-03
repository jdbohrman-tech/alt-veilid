#![forbid(unsafe_code)]
#![deny(clippy::all)]
#![deny(unused_must_use)]
#![recursion_limit = "256"]

mod client_api;
mod cmdline;
mod server;
mod settings;
mod tools;
#[cfg(unix)]
mod unix;
mod veilid_logs;
#[cfg(windows)]
mod windows;

use cfg_if::*;
#[allow(unused_imports)]
use color_eyre::eyre::{bail, ensure, eyre, Result as EyreResult, WrapErr};
use server::*;
use std::str::FromStr;
use tools::*;
use tracing::*;
use veilid_logs::*;

#[allow(clippy::all)]
pub mod veilid_client_capnp {
    include!(concat!(env!("OUT_DIR"), "/proto/veilid_client_capnp.rs"));
}

#[instrument(err)]
fn main() -> EyreResult<()> {
    #[cfg(windows)]
    let _ = ansi_term::enable_ansi_support();
    color_eyre::install()?;

    let (settings, matches) = cmdline::process_command_line()?;

    // --- Dump Config ---
    if matches.occurrences_of("dump-config") != 0 {
        return serde_yaml::to_writer(std::io::stdout(), &*settings.read())
            .wrap_err("failed to write yaml");
    }

    // --- Generate DHT Key ---
    if matches.occurrences_of("generate-key-pair") != 0 {
        if let Some(ckstr) = matches.get_one::<String>("generate-key-pair") {
            let ck: veilid_core::CryptoKind =
                veilid_core::FourCC::from_str(ckstr).wrap_err("couldn't parse crypto kind")?;
            let tkp = veilid_core::Crypto::generate_keypair(ck).wrap_err("invalid crypto kind")?;
            println!("{}", tkp.to_string());
            return Ok(());
        } else {
            bail!("missing crypto kind");
        }
    }

    // See if we're just running a quick command
    let (server_mode, success, failure) = if matches.occurrences_of("set-node-id") != 0 {
        (
            ServerMode::ShutdownImmediate,
            "Node Id and Secret set successfully",
            "Failed to set Node Id and Secret",
        )
    } else if matches.occurrences_of("dump-txt-record") != 0 {
        (ServerMode::DumpTXTRecord, "", "Failed to dump txt record")
    } else {
        (ServerMode::Normal, "", "")
    };

    // Handle non-normal server modes
    if !matches!(server_mode, ServerMode::Normal) {
        // run the server to set the node id and quit
        return block_on(async {
            // Init combined console/file logger
            let veilid_logs = VeilidLogs::setup(settings.clone())?;

            run_veilid_server(settings, server_mode, veilid_logs).await
        })
        .map(|v| {
            println!("{}", success);
            v
        })
        .map_err(|e| {
            println!("{}", failure);
            e
        });
    }

    // --- Daemon Mode ----
    if settings.read().daemon.enabled {
        cfg_if! {
            if #[cfg(windows)] {
                return windows::run_service(settings, matches);
            } else if #[cfg(unix)] {
                return unix::run_daemon(settings, matches);
            }
        }
    }

    // --- Normal Startup ---
    let panic_on_shutdown = matches.occurrences_of("panic") != 0;
    ctrlc::set_handler(move || {
        if panic_on_shutdown {
            let orig_hook = std::panic::take_hook();
            std::panic::set_hook(Box::new(move |panic_info| {
                // invoke the default handler and exit the process
                orig_hook(panic_info);

                let backtrace = backtrace::Backtrace::new();
                eprintln!("Backtrace:\n{:?}", backtrace);

                std::process::exit(1);
            }));
            panic!("panic requested");
        } else {
            shutdown();
        }
    })
    .expect("Error setting Ctrl-C handler");

    // Run the server loop
    block_on(async {
        // Init combined console/file logger
        let veilid_logs = VeilidLogs::setup(settings.clone())?;

        run_veilid_server(settings, server_mode, veilid_logs).await
    })
}
