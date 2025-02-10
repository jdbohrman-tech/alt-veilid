#![cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]

use cfg_if::*;
use clap::{Args, Parser};
use parking_lot::*;
use std::path::PathBuf;
use stop_token::StopSource;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, Layer, Registry};
use veilid_tools::*;
use virtual_network::*;

const VERSION: &str = env!("CARGO_PKG_VERSION");

cfg_if! {
    if #[cfg(feature="rt-async-std")] {
        pub fn block_on<F: Future<Output = T>, T>(f: F) -> T {
            async_std::task::block_on(f)
        }
    } else if #[cfg(feature="rt-tokio")] {
        pub fn block_on<F: Future<Output = T>, T>(f: F) -> T {
            let rt = tokio::runtime::Runtime::new().unwrap();
            let local = tokio::task::LocalSet::new();
            local.block_on(&rt, f)
        }
    } else {
        compile_error!("needs executor implementation");
    }
}

const DEFAULT_IGNORE_LOG_TARGETS: &[&str] = &["tokio", "runtime"];

#[derive(Args, Debug, Clone)]
#[group(multiple = false)]
pub struct Logging {
    /// Turn on debug logging on the terminal
    #[arg(long, group = "logging")]
    debug: bool,
    /// Turn on trace logging on the terminal
    #[arg(long, group = "logging")]
    trace: bool,
    /// Ignore log targets
    #[arg(long)]
    ignore_log_targets: Vec<String>,
    /// Enable log targets
    #[arg(long)]
    enable_log_targets: Vec<String>,
}

#[derive(Parser, Debug)]
#[command(author, version, about = "Veilid VirtualRouter")]
struct CmdlineArgs {
    /// TCP address to listen on
    #[arg(short('t'), long)]
    tcp_addr: Option<SocketAddr>,
    /// Turn off TCP listener
    #[arg(long)]
    no_tcp: bool,
    /// WS address to listen on
    #[arg(short('w'), long)]
    ws_addr: Option<SocketAddr>,
    /// Turn off WS listener
    #[arg(long)]
    no_ws: bool,
    /// Specify an initial list of configuration files to use
    #[arg(short = 'c', long, value_name = "FILE")]
    config_file: Vec<PathBuf>,
    /// Specify to load configuration without a predefined config first
    #[arg(long)]
    no_predefined_config: bool,
    /// Instead of running the virtual router, print the configuration it would use to the console
    #[arg(long)]
    dump_config: bool,
    /// Wait for debugger to attach
    #[cfg(debug_assertions)]
    #[arg(long)]
    wait_for_debug: bool,

    #[command(flatten)]
    logging: Logging,
}

fn setup_tracing(logging: &Logging) -> Result<(), String> {
    // Set up subscriber and layers
    let subscriber = Registry::default();
    let mut layers = Vec::new();

    // Get log level
    let level = if logging.trace {
        tracing::Level::TRACE
    } else if logging.debug {
        tracing::Level::DEBUG
    } else {
        tracing::Level::INFO
    };

    // Get ignore log targets
    let mut ignore_log_targets: Vec<String> = DEFAULT_IGNORE_LOG_TARGETS
        .iter()
        .map(|x| x.to_string())
        .collect();
    for x in &logging.ignore_log_targets {
        if !ignore_log_targets.contains(x) {
            ignore_log_targets.push(x.clone());
        }
    }
    ignore_log_targets.retain(|x| !logging.enable_log_targets.contains(x));

    let timer =
        time::format_description::parse("[hour]:[minute]:[second]").expect("invalid time format");

    // Use chrono instead of time crate to get local offset
    let offset_in_sec = chrono::Local::now().offset().local_minus_utc();
    let time_offset =
        time::UtcOffset::from_whole_seconds(offset_in_sec).expect("invalid utc offset");
    let timer = fmt::time::OffsetTime::new(time_offset, timer);

    let mut filter = tracing_subscriber::EnvFilter::from_default_env().add_directive(level.into());
    for x in ignore_log_targets {
        filter = filter.add_directive(format!("{x}=off").parse().unwrap());
    }

    let layer = fmt::Layer::new()
        .pretty()
        .with_timer(timer)
        .with_ansi(true)
        .with_writer(std::io::stdout)
        .with_filter(filter);

    layers.push(layer.boxed());

    let subscriber = subscriber.with(layers);
    subscriber
        .try_init()
        .map_err(|e| format!("failed to initialize tracing: {e}"))?;

    Ok(())
}

fn main() {
    if let Err(e) = real_main() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
    std::process::exit(0);
}
fn real_main() -> Result<(), String> {
    let stop_source = StopSource::new();
    let stop_token = stop_source.token();
    let stop_mutex = Mutex::new(Some(stop_source));

    ctrlc::set_handler(move || {
        println!("Exiting...");
        *(stop_mutex.lock()) = None;
    })
    .expect("Error setting Ctrl-C handler");

    block_on(async {
        println!("Veilid VirtualRouter v{}", VERSION);

        let args = CmdlineArgs::parse();

        #[cfg(debug_assertions)]
        if args.wait_for_debug {
            use bugsalot::debugger;
            debugger::wait_until_attached(None).expect("state() not implemented on this platform");
        }

        setup_tracing(&args.logging)?;

        let initial_config = config::Config::new(&args.config_file, args.no_predefined_config)
            .map_err(|e| format!("Error loading config: {}", e))?;

        if args.dump_config {
            let cfg_yaml = serde_yaml::to_string(&initial_config)
                .map_err(|e| format!("Error serializing config: {}", e))?;
            println!("{}", cfg_yaml);
            return Ok(());
        }

        let router_server = virtual_network::RouterServer::new();

        router_server
            .execute_config(initial_config)
            .map_err(|e| format!("Error executing config: {}", e))?;

        let _ss_tcp = if !args.no_tcp {
            Some(
                router_server
                    .listen_tcp(args.tcp_addr)
                    .await
                    .map_err(|e| e.to_string())?,
            )
        } else {
            None
        };

        let _ss_ws = if !args.no_ws {
            Some(
                router_server
                    .listen_ws(args.ws_addr)
                    .await
                    .map_err(|e| e.to_string())?,
            )
        } else {
            None
        };

        println!("Running...");
        router_server
            .run(stop_token)
            .await
            .map_err(|e| e.to_string())?;
        println!("Done");
        Ok(())
    })
}
