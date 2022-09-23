#![deny(clippy::all)]
#![deny(unused_must_use)]

use veilid_core::xx::*;

use clap::{Arg, ColorChoice, Command};
use flexi_logger::*;
use std::ffi::OsStr;
use std::net::ToSocketAddrs;
use std::path::Path;
use tools::*;

mod client_api_connection;
mod command_processor;
mod peers_table_view;
mod settings;
mod tools;
mod ui;

#[allow(clippy::all)]
pub mod veilid_client_capnp {
    include!(concat!(env!("OUT_DIR"), "/proto/veilid_client_capnp.rs"));
}

fn parse_command_line(default_config_path: &OsStr) -> Result<clap::ArgMatches, String> {
    let matches = Command::new("veilid-cli")
        .version("0.1")
        .color(ColorChoice::Auto)
        .about("Veilid Console Client")
        .arg(
            Arg::new("address")
                .required(false)
                .help("Address to connect to"),
        )
        .arg(
            Arg::new("debug")
                .long("debug")
                .help("Turn on debug logging"),
        )
        .arg(
            Arg::new("wait-for-debug")
                .long("wait-for-debug")
                .help("Wait for debugger to attach"),
        )
        .arg(
            Arg::new("trace")
                .long("trace")
                .conflicts_with("debug")
                .help("Turn on trace logging"),
        )
        .arg(
            Arg::new("config-file")
                .short('c')
                .takes_value(true)
                .value_name("FILE")
                .default_value_os(default_config_path)
                .allow_invalid_utf8(true)
                .help("Specify a configuration file to use"),
        )
        .get_matches();

    Ok(matches)
}

fn main() -> Result<(), String> {
    // Get command line options
    let default_config_path = settings::Settings::get_default_config_path();
    let matches = parse_command_line(default_config_path.as_os_str())?;
    if matches.occurrences_of("wait-for-debug") != 0 {
        use bugsalot::debugger;
        debugger::wait_until_attached(None).expect("state() not implemented on this platform");
    }

    // Attempt to load configuration
    let settings_path = if let Some(config_file) = matches.value_of_os("config-file") {
        if Path::new(config_file).exists() {
            Some(config_file)
        } else {
            None
        }
    } else {
        None
    };

    let mut settings = settings::Settings::new(settings_path)
        .map_err(|e| format!("configuration is invalid: {}", e))?;

    // Set config from command line
    if matches.occurrences_of("debug") != 0 {
        settings.logging.level = settings::LogLevel::Debug;
        settings.logging.terminal.enabled = true;
    }
    if matches.occurrences_of("trace") != 0 {
        settings.logging.level = settings::LogLevel::Trace;
        settings.logging.terminal.enabled = true;
    }

    // Create UI object
    let mut sivui = ui::UI::new(settings.interface.node_log.scrollback, &settings);

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
    let server_addrs = if let Some(address_arg) = matches.value_of("address") {
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
    let mut comproc = command_processor::CommandProcessor::new(sivui.clone(), &settings);
    sivui.set_command_processor(comproc.clone());

    // Create client api client side
    info!("Starting API connection");
    let mut capi = client_api_connection::ClientApiConnection::new(comproc.clone());

    // Save client api in command processor
    comproc.set_client_api_connection(capi.clone());

    // Keep a connection to the server
    comproc.set_server_address(server_addr);
    let mut comproc2 = comproc.clone();
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
            }
        }
    });

    Ok(())
}
