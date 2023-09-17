#![deny(clippy::all)]
#![deny(unused_must_use)]
#![recursion_limit = "256"]

use crate::tools::*;

use clap::{Parser, ValueEnum};
use flexi_logger::*;
use std::{net::ToSocketAddrs, path::PathBuf};

mod client_api_connection;
mod command_processor;
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
    /// Address to connect to
    #[arg(long)]
    address: Option<String>,
    /// Wait for debugger to attach
    #[arg(long)]
    wait_for_debug: bool,
    /// Specify a configuration file to use
    #[arg(short, long, value_name = "FILE")]
    config_file: Option<PathBuf>,
    /// log level
    #[arg(value_enum)]
    log_level: Option<LogLevel>,
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

    // Create UI object
    let (mut sivui, uisender) = ui::UI::new(settings.interface.node_log.scrollback, &settings);

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
            let flv = sivui.cursive_flexi_logger();
            if settings.logging.file.enabled {
                std::fs::create_dir_all(settings.logging.file.directory.clone())
                    .map_err(map_to_string)?;
                logger
                    .log_to_file_and_writer(
                        FileSpec::default()
                            .directory(settings.logging.file.directory.clone())
                            .suppress_timestamp(),
                        flv,
                    )
                    .start()
                    .expect("failed to initialize logger!");
            } else {
                logger
                    .log_to_writer(flv)
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
    let server_addrs = if let Some(address_arg) = args.address {
        address_arg
            .to_socket_addrs()
            .map_err(|e| format!("Invalid server address '{}'", e))?
            .collect()
    } else {
        settings.address.addrs.clone()
    };
    let server_addr = server_addrs.first().cloned();

    // Create command processor
    debug!("Creating Command Processor ");
    let comproc = command_processor::CommandProcessor::new(uisender, &settings);
    sivui.set_command_processor(comproc.clone());

    // Create client api client side
    info!("Starting API connection");
    let capi = client_api_connection::ClientApiConnection::new(comproc.clone());

    // Save client api in command processor
    comproc.set_client_api_connection(capi.clone());

    // Keep a connection to the server
    comproc.set_server_address(server_addr);
    let comproc2 = comproc.clone();
    let connection_future = comproc.connection_manager();

    // Start async
    block_on(async move {
        // Start UI
        let ui_future = async move {
            sivui.run_async().await;

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
