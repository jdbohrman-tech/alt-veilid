#![deny(clippy::all)]
#![allow(clippy::comparison_chain, clippy::upper_case_acronyms)]
#![deny(unused_must_use)]
#![recursion_limit = "256"]

use crate::{settings::NamedSocketAddrs, tools::*, ui::*};

use clap::{Parser, ValueEnum};
use flexi_logger::*;
use std::path::PathBuf;
mod cached_text_view;
mod client_api_connection;
mod command_processor;
mod cursive_ui;
mod interactive_ui;
mod peers_table_view;
mod settings;
mod tools;
mod ui;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum LogLevel {
    /// Turn on debug logging
    Debug,
    /// Turn on trace logging
    Trace,
}

#[derive(Parser, Debug)]
#[command(author, version, about = "Veilid Console Client")]
struct CmdlineArgs {
    /// IPC socket to connect to
    #[arg(long, short = 'p')]
    ipc_path: Option<PathBuf>,
    /// Subnode index to use when connecting
    #[arg(long, default_value = "0")]
    subnode_index: usize,
    /// Address to connect to
    #[arg(long, short = 'a')]
    address: Option<String>,
    /// Wait for debugger to attach
    #[arg(long)]
    wait_for_debug: bool,
    /// Specify a configuration file to use
    #[arg(short = 'c', long, value_name = "FILE")]
    config_file: Option<PathBuf>,
    /// log level
    #[arg(value_enum)]
    log_level: Option<LogLevel>,
    /// interactive
    #[arg(long, short = 'i', group = "execution_mode")]
    interactive: bool,
    /// evaluate
    #[arg(long, short = 'e', group = "execution_mode")]
    evaluate: Option<String>,
    /// show log
    #[arg(long, short = 'l', group = "execution_mode")]
    show_log: bool,
    /// read commands from file
    #[arg(
        long,
        short = 'f',
        group = "execution_mode",
        value_name = "COMMAND_FILE"
    )]
    command_file: Option<PathBuf>,
}

fn main() -> Result<(), String> {
    // Get command line options
    let default_config_path = settings::Settings::get_default_config_path();
    let args = CmdlineArgs::parse();

    if args.wait_for_debug {
        use bugsalot::debugger;
        debugger::wait_until_attached(None).expect("state() not implemented on this platform");
    }

    // Attempt to load configuration
    let settings_path = args.config_file.unwrap_or(default_config_path);
    let settings_path = if settings_path.exists() {
        Some(settings_path.into_os_string())
    } else {
        None
    };

    let mut settings = settings::Settings::new(settings_path.as_deref())
        .map_err(|e| format!("configuration is invalid: {}", e))?;

    // Set config from command line
    if let Some(LogLevel::Debug) = args.log_level {
        settings.logging.level = settings::LogLevel::Debug;
        settings.logging.terminal.enabled = true;
    }
    if let Some(LogLevel::Trace) = args.log_level {
        settings.logging.level = settings::LogLevel::Trace;
        settings.logging.terminal.enabled = true;
    }

    // If we are running in interactive mode disable some things
    let mut enable_cursive = true;
    if args.interactive || args.show_log || args.command_file.is_some() || args.evaluate.is_some() {
        settings.logging.terminal.enabled = false;
        enable_cursive = false;
    }

    // Create UI object
    let (mut ui, uisender) = if enable_cursive {
        let (ui, uisender) = cursive_ui::CursiveUI::new(&settings);
        (
            Box::new(ui) as Box<dyn UI>,
            Box::new(uisender) as Box<dyn UISender>,
        )
    } else if args.interactive {
        let (ui, uisender) = interactive_ui::InteractiveUI::new(&settings);
        (
            Box::new(ui) as Box<dyn UI>,
            Box::new(uisender) as Box<dyn UISender>,
        )
    } else {
        panic!("unknown ui mode");
    };

    // Set up loggers
    {
        let mut specbuilder = LogSpecBuilder::new();
        specbuilder.default(settings::convert_loglevel(settings.logging.level));
        specbuilder.module("cursive", LevelFilter::Off);
        specbuilder.module("cursive_core", LevelFilter::Off);
        specbuilder.module("cursive_buffered_backend", LevelFilter::Off);
        specbuilder.module("tokio_util", LevelFilter::Off);
        specbuilder.module("mio", LevelFilter::Off);
        specbuilder.module("async_std", LevelFilter::Off);
        specbuilder.module("async_io", LevelFilter::Off);
        specbuilder.module("polling", LevelFilter::Off);

        let logger = Logger::with(specbuilder.build());

        if settings.logging.terminal.enabled {
            if settings.logging.file.enabled {
                std::fs::create_dir_all(settings.logging.file.directory.clone())
                    .map_err(map_to_string)?;
                logger
                    .log_to_file_and_writer(
                        FileSpec::default()
                            .directory(settings.logging.file.directory.clone())
                            .suppress_timestamp(),
                        uisender.as_logwriter().unwrap(),
                    )
                    .start()
                    .expect("failed to initialize logger!");
            } else {
                logger
                    .log_to_writer(uisender.as_logwriter().unwrap())
                    .start()
                    .expect("failed to initialize logger!");
            }
        } else if settings.logging.file.enabled {
            std::fs::create_dir_all(settings.logging.file.directory.clone())
                .map_err(map_to_string)?;
            logger
                .log_to_file(
                    FileSpec::default()
                        .directory(settings.logging.file.directory.clone())
                        .suppress_timestamp(),
                )
                .start()
                .expect("failed to initialize logger!");
        }
    }

    // Get client address
    let enable_ipc = (settings.enable_ipc && args.address.is_none()) || args.ipc_path.is_some();
    let mut enable_network =
        (settings.enable_network && args.ipc_path.is_none()) || args.address.is_some();

    // Determine IPC path to try
    let mut client_api_ipc_path = None;
    if enable_ipc {
        cfg_if::cfg_if! {
            if #[cfg(windows)] {
                if let Some(ipc_path) = args.ipc_path.or(settings.ipc_path.clone()) {
                    if is_ipc_socket_path(&ipc_path) {
                        // try direct path
                        enable_network = false;
                        client_api_ipc_path = Some(ipc_path);
                    } else {
                        // try subnode index inside path
                        let ipc_path = ipc_path.join(args.subnode_index.to_string());
                        if is_ipc_socket_path(&ipc_path) {
                            // subnode indexed path exists
                            enable_network = false;
                            client_api_ipc_path = Some(ipc_path);
                        }
                    }
                }
            } else {
                if let Some(ipc_path) = args.ipc_path.or(settings.ipc_path.clone()) {
                    if is_ipc_socket_path(&ipc_path) {
                        // try direct path
                        enable_network = false;
                        client_api_ipc_path = Some(ipc_path);
                    } else if ipc_path.exists() && ipc_path.is_dir() {
                        // try subnode index inside path
                        let ipc_path = ipc_path.join(args.subnode_index.to_string());
                        if is_ipc_socket_path(&ipc_path) {
                            // subnode indexed path exists
                            enable_network = false;
                            client_api_ipc_path = Some(ipc_path);
                        }
                    }
                }
            }
        }
    }
    let mut client_api_network_addresses = None;
    if enable_network {
        let args_address = if let Some(args_address) = args.address {
            match NamedSocketAddrs::try_from(args_address) {
                Ok(v) => Some(v),
                Err(e) => {
                    return Err(format!("Invalid server address: {}", e));
                }
            }
        } else {
            None
        };
        if let Some(address_arg) = args_address.or(settings.address.clone()) {
            client_api_network_addresses = Some(address_arg.addrs);
        } else if let Some(address) = settings.address.clone() {
            client_api_network_addresses = Some(address.addrs.clone());
        }
    }

    // Create command processor
    debug!("Creating Command Processor ");
    let comproc = command_processor::CommandProcessor::new(uisender, &settings);

    ui.set_command_processor(comproc.clone());

    // Create client api client side
    info!("Starting API connection");
    let capi = client_api_connection::ClientApiConnection::new(comproc.clone());

    // Save client api in command processor
    comproc.set_client_api_connection(capi.clone());

    // Keep a connection to the server
    if let Some(client_api_ipc_path) = client_api_ipc_path {
        comproc.set_ipc_path(Some(client_api_ipc_path));
    } else if let Some(client_api_network_address) = client_api_network_addresses {
        let network_addr = client_api_network_address.first().cloned();
        comproc.set_network_address(network_addr);
    } else {
        return Err("veilid-server could not be reached".to_owned());
    }

    let comproc2 = comproc.clone();
    let connection_future = comproc.connection_manager();

    // Start async
    block_on(async move {
        // Start UI
        let ui_future = async move {
            ui.run_async().await;

            // When UI quits, close connection and command processor cleanly
            comproc2.quit();
            capi.disconnect().await;
        };

        cfg_if! {
            if #[cfg(feature="rt-async-std")] {
                use async_std::prelude::*;
                // Wait for ui and connection to complete
                let _  = ui_future.join(connection_future).await;
            } else if #[cfg(feature="rt-tokio")] {
                // Wait for ui and connection to complete
                let _ = tokio::join!(ui_future, connection_future);
            } else {
                compile_error!("needs executor implementation")
            }
        }
    });

    Ok(())
}
