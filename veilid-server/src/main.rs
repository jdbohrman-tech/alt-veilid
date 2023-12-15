#![forbid(unsafe_code)]
#![deny(clippy::all)]
#![allow(clippy::comparison_chain, clippy::upper_case_acronyms)]
#![deny(unused_must_use)]
#![recursion_limit = "256"]

mod client_api;
mod server;
mod settings;
mod tools;
#[cfg(unix)]
mod unix;
mod veilid_logs;
#[cfg(windows)]
mod windows;

use crate::settings::*;

use clap::{Args, Parser};
use server::*;
use settings::LogLevel;
use std::collections::HashMap;
use std::ffi::OsString;
use std::path::Path;
use std::str::FromStr;
use tools::*;
use veilid_core::{TypedKeyGroup, TypedSecretGroup};
use veilid_logs::*;

#[derive(Args, Debug, Clone)]
#[group(multiple = false)]
pub struct Logging {
    /// Turn on debug logging on the terminal
    #[arg(long)]
    debug: bool,
    /// Turn on trace logging on the terminal
    #[arg(long)]
    trace: bool,
}

#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
pub struct CmdlineArgs {
    /// Run in daemon mode in the background
    #[arg(short, long)]
    daemon: bool,

    /// Run in the foreground
    #[arg(short, long)]
    foreground: bool,

    /// Specify a configuration file to use
    #[arg(short, long, value_name = "FILE", default_value = OsString::from(Settings::get_default_config_path()))]
    config_file: Option<OsString>,

    /// Specify configuration value to set (key in dot format, value in json format), eg: logging.api.enabled=true
    #[arg(short, long, value_name = "CONFIG")]
    set_config: Vec<String>,

    /// Specify password to use to protect the device encryption key
    #[arg(short, long, value_name = "PASSWORD")]
    password: Option<String>,

    /// Change password used to protect the device encryption key. Device storage will be migrated.
    #[arg(long, value_name = "PASSWORD")]
    new_password: Option<String>,

    /// Do not automatically attach the server to the Veilid network
    ///
    /// Default behaviour is to automatically attach the server to the Veilid network, this option disables this behaviour.
    #[arg(long, value_name = "BOOL")]
    no_attach: bool,

    #[command(flatten)]
    logging: Logging,

    /// Turn on OpenTelemetry tracing
    ///
    /// This option uses the GRPC OpenTelemetry protocol, not HTTP. The format for the endpoint is host:port, like 'localhost:4317'
    #[arg(long, value_name = "endpoint")]
    otlp: Option<String>,

    /// Run as an extra daemon on the same machine for testing purposes, specify a number greater than zero to offset the listening ports
    #[arg(long)]
    subnode_index: Option<u16>,

    /// Only generate a new keypair and print it
    ///
    /// Generate a new keypair for a specific crypto kind and print both the key and its secret to the terminal, then exit immediately.
    #[arg(long, value_name = "crypto_kind")]
    generate_key_pair: Option<String>,

    /// Set the node ids and secret keys
    ///
    /// Specify node ids in typed key set format ('\[VLD0:xxxx,VLD1:xxxx\]') on the command line, a prompt appears to enter the secret key set interactively.
    #[arg(long, value_name = "key_set")]
    set_node_id: Option<String>,

    /// Delete the entire contents of the protected store (DANGER, NO UNDO!)
    #[arg(long)]
    delete_protected_store: bool,

    /// Delete the entire contents of the table store (DANGER, NO UNDO!)
    #[arg(long)]
    delete_table_store: bool,

    /// Delete the entire contents of the block store (DANGER, NO UNDO!)
    #[arg(long)]
    delete_block_store: bool,

    /// Instead of running the server, print the configuration it would use to the console
    #[arg(long)]
    dump_config: bool,

    /// Prints the bootstrap TXT record for this node and then quits
    #[arg(long)]
    dump_txt_record: bool,

    /// Emits a JSON-Schema for a named type
    #[arg(long, value_name = "schema_name")]
    emit_schema: Option<String>,

    /// Specify a list of bootstrap hostnames to use
    #[arg(long, value_name = "BOOTSTRAP_LIST")]
    bootstrap: Option<String>,

    /// panic on ctrl-c instead of graceful shutdown
    #[arg(long)]
    panic: bool,

    /// password override to use for network isolation
    #[arg(long, value_name = "KEY")]
    network_key: Option<String>,

    /// Wait for debugger to attach
    #[cfg(debug_assertions)]
    #[arg(long)]
    wait_for_debug: bool,

    /// enable tokio console
    #[cfg(feature = "rt-tokio")]
    #[arg(long)]
    console: bool,
}

#[instrument(err)]
fn main() -> EyreResult<()> {
    #[cfg(windows)]
    let _ = ansi_term::enable_ansi_support();
    color_eyre::install()?;

    // Get command line options
    let args = CmdlineArgs::parse();

    let svc_args = args.clone();

    // Check for one-off commands
    #[cfg(debug_assertions)]
    if args.wait_for_debug {
        use bugsalot::debugger;
        debugger::wait_until_attached(None).expect("state() not implemented on this platform");
    }

    // Attempt to load configuration
    let settings_path: Option<OsString> = args
        .config_file
        .filter(|config_file| Path::new(&config_file).exists());

    let settings = Settings::new(settings_path.as_deref()).wrap_err("configuration is invalid")?;

    // write lock the settings
    let mut settingsrw = settings.write();

    // Set config from command line
    if args.daemon {
        settingsrw.daemon.enabled = true;
        settingsrw.logging.terminal.enabled = false;
    }
    if args.foreground {
        settingsrw.daemon.enabled = false;
    }
    if let Some(subnode_index) = args.subnode_index {
        settingsrw.testing.subnode_index = subnode_index;
    };

    if args.logging.debug {
        settingsrw.logging.terminal.enabled = true;
        settingsrw.logging.terminal.level = LogLevel::Debug;
    }
    if args.logging.trace {
        settingsrw.logging.terminal.enabled = true;
        settingsrw.logging.terminal.level = LogLevel::Trace;
    }
    if args.otlp.is_some() {
        println!("Enabling OTLP tracing");
        settingsrw.logging.otlp.enabled = true;
        settingsrw.logging.otlp.grpc_endpoint = NamedSocketAddrs::from_str(
            args.otlp
                .expect("should not be null because of default missing value")
                .as_str(),
        )
        .wrap_err("failed to parse OTLP address")?;
        settingsrw.logging.otlp.level = LogLevel::Trace;
    }
    if args.no_attach {
        settingsrw.auto_attach = false;
    }
    if args.delete_protected_store {
        settingsrw.core.protected_store.delete = true;
    }
    if args.delete_block_store {
        settingsrw.core.block_store.delete = true;
    }
    if args.delete_table_store {
        settingsrw.core.table_store.delete = true;
    }
    if let Some(password) = args.password {
        settingsrw
            .core
            .protected_store
            .device_encryption_key_password = password;
    }
    if let Some(new_password) = args.new_password {
        settingsrw
            .core
            .protected_store
            .new_device_encryption_key_password = Some(new_password);
    }
    if let Some(network_key) = args.network_key {
        settingsrw.core.network.network_key_password = Some(network_key);
    }
    if args.dump_txt_record {
        // Turn off terminal logging so we can be interactive
        settingsrw.logging.terminal.enabled = false;
    }
    let mut node_id_set = false;
    if let Some(key_set) = args.set_node_id {
        node_id_set = true;
        // Turn off terminal logging so we can be interactive
        settingsrw.logging.terminal.enabled = false;

        // Split or get secret
        let tks = TypedKeyGroup::from_str(&key_set)
            .wrap_err("failed to decode node id set from command line")?;

        let buffer = rpassword::prompt_password("Enter secret key set (will not echo): ")
            .wrap_err("invalid secret key")?;
        let buffer = buffer.trim().to_string();
        let tss = TypedSecretGroup::from_str(&buffer).wrap_err("failed to decode secret set")?;

        settingsrw.core.network.routing_table.node_id = Some(tks);
        settingsrw.core.network.routing_table.node_id_secret = Some(tss);
    }

    if let Some(bootstrap) = args.bootstrap {
        println!("Overriding bootstrap list with: ");
        let mut bootstrap_list: Vec<String> = Vec::new();
        for x in bootstrap.split(',') {
            let x = x.trim().to_string();
            if !x.is_empty() {
                println!("    {}", x);
                bootstrap_list.push(x);
            }
        }
        settingsrw.core.network.routing_table.bootstrap = bootstrap_list;
    };

    #[cfg(feature = "rt-tokio")]
    if args.console {
        settingsrw.logging.console.enabled = true;
    }

    drop(settingsrw);

    // Set specific config settings
    for set_config in args.set_config {
        if let Some((k, v)) = set_config.split_once('=') {
            let k = k.trim();
            let v = v.trim();
            settings.set(k, v)?;
        }
    }

    // Apply subnode index if we're testing
    settings
        .apply_subnode_index()
        .wrap_err("failed to apply subnode index")?;

    // --- Dump Config ---
    if args.dump_config {
        return serde_yaml::to_writer(std::io::stdout(), &*settings.read())
            .wrap_err("failed to write yaml");
    }

    // --- Generate DHT Key ---
    if let Some(ckstr) = args.generate_key_pair {
        if ckstr.is_empty() {
            let mut tks = veilid_core::TypedKeyGroup::new();
            let mut tss = veilid_core::TypedSecretGroup::new();
            for ck in veilid_core::VALID_CRYPTO_KINDS {
                let tkp =
                    veilid_core::Crypto::generate_keypair(ck).wrap_err("invalid crypto kind")?;
                tks.add(veilid_core::TypedKey::new(tkp.kind, tkp.value.key));
                tss.add(veilid_core::TypedSecret::new(tkp.kind, tkp.value.secret));
            }
            println!("Public Keys:\n{}\nSecret Keys:\n{}\n", tks, tss);
        } else {
            let ck: veilid_core::CryptoKind =
                veilid_core::FourCC::from_str(&ckstr).wrap_err("couldn't parse crypto kind")?;
            let tkp = veilid_core::Crypto::generate_keypair(ck).wrap_err("invalid crypto kind")?;
            println!("{}", tkp);
        }
        return Ok(());
    }

    // -- Emit JSON-Schema --
    if let Some(esstr) = args.emit_schema {
        let mut schemas = HashMap::<String, String>::new();
        veilid_core::json_api::emit_schemas(&mut schemas);

        if let Some(schema) = schemas.get(&esstr) {
            println!("{}", schema);
        } else {
            println!("Valid schemas:");
            for s in schemas.keys() {
                println!("  {}", s);
            }
        }

        return Ok(());
    }

    // See if we're just running a quick command
    let (server_mode, success, failure) = if node_id_set {
        (
            ServerMode::ShutdownImmediate,
            "Node Id and Secret set successfully",
            "Failed to set Node Id and Secret",
        )
    } else if args.dump_txt_record {
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
                return windows::run_service(settings, svc_args);
            } else if #[cfg(unix)] {
                return unix::run_daemon(settings, svc_args);
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

    let panic_on_shutdown = args.panic;
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
