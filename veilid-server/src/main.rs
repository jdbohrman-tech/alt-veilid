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

use server::*;
use std::collections::HashMap;
use std::str::FromStr;
use tools::*;
use veilid_logs::*;

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
            if ckstr == "" {
                let mut tks = veilid_core::TypedKeyGroup::new();
                let mut tss = veilid_core::TypedSecretGroup::new();
                for ck in veilid_core::VALID_CRYPTO_KINDS {
                    let tkp = veilid_core::Crypto::generate_keypair(ck)
                        .wrap_err("invalid crypto kind")?;
                    tks.add(veilid_core::TypedKey::new(tkp.kind, tkp.value.key));
                    tss.add(veilid_core::TypedSecret::new(tkp.kind, tkp.value.secret));
                }
                println!(
                    "Public Keys:\n{}\nSecret Keys:\n{}\n",
                    tks.to_string(),
                    tss.to_string()
                );
            } else {
                let ck: veilid_core::CryptoKind =
                    veilid_core::FourCC::from_str(ckstr).wrap_err("couldn't parse crypto kind")?;
                let tkp =
                    veilid_core::Crypto::generate_keypair(ck).wrap_err("invalid crypto kind")?;
                println!("{}", tkp.to_string());
            }
            return Ok(());
        } else {
            bail!("missing crypto kind");
        }
    }
    // -- Emit JSON-Schema --
    if matches.occurrences_of("emit-schema") != 0 {
        if let Some(esstr) = matches.value_of("emit-schema") {
            let mut schemas = HashMap::<String, String>::new();
            veilid_core::json_api::emit_schemas(&mut schemas);

            if let Some(schema) = schemas.get(esstr) {
                println!("{}", schema);
            } else {
                println!("Valid schemas:");
                for s in schemas.keys() {
                    println!("  {}", s);
                }
            }

            return Ok(());
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
    let orig_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        // invoke the default handler and exit the process
        orig_hook(panic_info);

        let backtrace = backtrace::Backtrace::new();
        eprintln!("Backtrace:\n{:?}", backtrace);

        eprintln!("exiting!");
        std::process::exit(1);
    }));

    let panic_on_shutdown = matches.occurrences_of("panic") != 0;
    ctrlc::set_handler(move || {
        if panic_on_shutdown {
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
