use crate::settings::*;
use crate::*;
use clap::{Arg, ArgMatches, Command};
use std::ffi::OsStr;
use std::path::Path;
use std::str::FromStr;
use veilid_core::{SecretKey, TypedKey};

fn do_clap_matches(default_config_path: &OsStr) -> Result<clap::ArgMatches, clap::Error> {
    let matches = Command::new("veilid-server")
        .version("0.1")
        .about("Veilid Server")
        .color(clap::ColorChoice::Auto)
        .arg(
            Arg::new("daemon")
                .long("daemon")
                .short('d')
                .help("Run in daemon mode in the background"),
        )
        .arg(
            Arg::new("foreground")
                .long("foreground")
                .short('f')
                .conflicts_with("daemon")
                .help("Run in the foreground"),
        )
        .arg(
            Arg::new("config-file")
                .short('c')
                .long("config-file")
                .takes_value(true)
                .value_name("FILE")
                .default_value_os(default_config_path)
                .allow_invalid_utf8(true)
                .help("Specify a configuration file to use"),
        )
        .arg(
            Arg::new("set-config")
                .short('s')
                .long("set-config")
                .takes_value(true)
                .multiple_occurrences(true)
                .help("Specify configuration value to set (key in dot format, value in json format), eg: logging.api.enabled=true")
        )
        .arg(
            Arg::new("attach")
                .long("attach")
                .takes_value(true)
                .value_name("BOOL")
                .possible_values(&["false", "true"])
                .help("Automatically attach the server to the Veilid network"),
        )
        // Dev options
        .arg(
            Arg::new("debug")
                .long("debug")
                .help("Turn on debug logging on the terminal"),
        )
        .arg(
            Arg::new("trace")
                .long("trace")
                .conflicts_with("debug")
                .help("Turn on trace logging on the terminal"),
        )
        .arg(
            Arg::new("otlp")
                .long("otlp")
                .takes_value(true)
                .value_name("endpoint")
                .default_missing_value("localhost:4317")
                .help("Turn on OpenTelemetry tracing")
                .long_help("This option uses the GRPC OpenTelemetry protocol, not HTTP. The format for the endpoint is host:port, like 'localhost:4317'"),
        )
        .arg(
            Arg::new("subnode-index")
                .long("subnode-index")
                .takes_value(true)
                .help("Run as an extra daemon on the same machine for testing purposes, specify a number greater than zero to offset the listening ports"),
        )
        .arg(
            Arg::new("generate-dht-key")
                .long("generate-dht-key")
                .help("Only generate a new dht key and print it"),
        )
        .arg(
            Arg::new("set-node-id")
                .long("set-node-id")
                .takes_value(true)
                .value_name("ID")
                .help("Set the node id and secret key")
                .long_help("To specify both node id and secret key on the command line, use a ID:SECRET syntax with a colon, like:\n  zsVXz5aTU98vZxwTcDmvpcnO5g1B2jRO3wpdNiDrRgw:gJzQLmzuBvA-dFvEmLcYvLoO5bh7hzCWFzfpJHapZKg\nIf no colon is used, the node id is specified, and a prompt appears to enter the secret key interactively.")
        )
        .arg(
            Arg::new("delete-protected-store")
                .long("delete-protected-store")
                .help("Delete the entire contents of the protected store (DANGER, NO UNDO!)"),
        )
        .arg(
            Arg::new("delete-table-store")
                .long("delete-table-store")
                .help("Delete the entire contents of the table store (DANGER, NO UNDO!)"),
        )
        .arg(
            Arg::new("delete-block-store")
                .long("delete-block-store")
                .help("Delete the entire contents of the block store (DANGER, NO UNDO!)"),
        )
        .arg(
            Arg::new("dump-config")
                .long("dump-config")
                .help("Instead of running the server, print the configuration it would use to the console"),
        )
        .arg(
            Arg::new("dump-txt-record")
                .long("dump-txt-record")
                .help("Prints the bootstrap TXT record for this node and then quits")
        )
        .arg(
            Arg::new("bootstrap")
                .long("bootstrap")
                .takes_value(true)
                .value_name("BOOTSTRAP_LIST")
                .help("Specify a list of bootstrap hostnames to use")
        )
        .arg(
            Arg::new("bootstrap-nodes")
                .conflicts_with("bootstrap")
                .long("bootstrap-nodes")
                .takes_value(true)
                .value_name("BOOTSTRAP_NODE_LIST")
                .help("Specify a list of bootstrap node dialinfos to use"),
        )
        .arg(
            Arg::new("panic")
                .long("panic")
                .help("panic on ctrl-c instead of graceful shutdown"),
        )
        ;

    #[cfg(feature = "rt-tokio")]
    let matches = matches.arg(
        Arg::new("console")
            .long("console")
            .help("enable tokio console"),
    );

    #[cfg(debug_assertions)]
    let matches = matches.arg(
        Arg::new("wait-for-debug")
            .long("wait-for-debug")
            .help("Wait for debugger to attach"),
    );

    Ok(matches.get_matches())
}

pub fn process_command_line() -> EyreResult<(Settings, ArgMatches)> {
    // Get command line options
    let default_config_path = Settings::get_default_config_path();
    let matches = do_clap_matches(default_config_path.as_os_str())
        .wrap_err("failed to parse command line: {}")?;

    // Check for one-off commands
    #[cfg(debug_assertions)]
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

    let settings = Settings::new(settings_path).wrap_err("configuration is invalid")?;

    // write lock the settings
    let mut settingsrw = settings.write();

    // Set config from command line
    if matches.occurrences_of("daemon") != 0 {
        settingsrw.daemon.enabled = true;
        settingsrw.logging.terminal.enabled = false;
    }
    if matches.occurrences_of("foreground") != 0 {
        settingsrw.daemon.enabled = false;
    }
    if matches.occurrences_of("subnode-index") != 0 {
        let subnode_index = match matches.value_of("subnode-index") {
            Some(x) => x.parse().wrap_err("couldn't parse subnode index")?,
            None => {
                bail!("value not specified for subnode-index");
            }
        };
        if subnode_index == 0 {
            bail!("value of subnode_index should be between 1 and 65535");
        }
        settingsrw.testing.subnode_index = subnode_index;
    }

    if matches.occurrences_of("debug") != 0 {
        settingsrw.logging.terminal.enabled = true;
        settingsrw.logging.terminal.level = LogLevel::Debug;
    }
    if matches.occurrences_of("trace") != 0 {
        settingsrw.logging.terminal.enabled = true;
        settingsrw.logging.terminal.level = LogLevel::Trace;
    }
    if matches.occurrences_of("otlp") != 0 {
        settingsrw.logging.otlp.enabled = true;
        settingsrw.logging.otlp.grpc_endpoint = NamedSocketAddrs::from_str(
            &matches
                .value_of("otlp")
                .expect("should not be null because of default missing value")
                .to_string(),
        )
        .wrap_err("failed to parse OTLP address")?;
        settingsrw.logging.otlp.level = LogLevel::Trace;
    }
    if matches.is_present("attach") {
        settingsrw.auto_attach = !matches!(matches.value_of("attach"), Some("true"));
    }
    if matches.occurrences_of("delete-protected-store") != 0 {
        settingsrw.core.protected_store.delete = true;
    }
    if matches.occurrences_of("delete-block-store") != 0 {
        settingsrw.core.block_store.delete = true;
    }
    if matches.occurrences_of("delete-table-store") != 0 {
        settingsrw.core.table_store.delete = true;
    }
    if matches.occurrences_of("dump-txt-record") != 0 {
        // Turn off terminal logging so we can be interactive
        settingsrw.logging.terminal.enabled = false;
    }
    if let Some(v) = matches.value_of("set-node-id") {
        // Turn off terminal logging so we can be interactive
        settingsrw.logging.terminal.enabled = false;

        // Split or get secret
        let (k, s) = if let Some((k, s)) = v.split_once(':') {
            let k =
                TypedKey::try_decode(k).wrap_err("failed to decode node id from command line")?;
            let s = SecretKey::try_decode(s)?;
            (k, s)
        } else {
            let k = TypedKey::try_decode(v)?;
            let buffer = rpassword::prompt_password("Enter secret key (will not echo): ")
                .wrap_err("invalid secret key")?;
            let buffer = buffer.trim().to_string();
            let s = SecretKey::try_decode(&buffer)?;
            (k, s)
        };
        settingsrw.core.network.node_id = Some(k);
        settingsrw.core.network.node_id_secret = Some(s);
    }

    if matches.occurrences_of("bootstrap") != 0 {
        let bootstrap_list = match matches.value_of("bootstrap") {
            Some(x) => {
                println!("Overriding bootstrap list with: ");
                let mut out: Vec<String> = Vec::new();
                for x in x.split(',') {
                    let x = x.trim().to_string();
                    println!("    {}", x);
                    out.push(x);
                }
                out
            }
            None => {
                bail!("value not specified for bootstrap");
            }
        };
        settingsrw.core.network.bootstrap = bootstrap_list;
    }

    if matches.occurrences_of("bootstrap-nodes") != 0 {
        let bootstrap_list = match matches.value_of("bootstrap-nodes") {
            Some(x) => {
                println!("Overriding bootstrap node list with: ");
                let mut out: Vec<ParsedNodeDialInfo> = Vec::new();
                for x in x.split(',') {
                    let x = x.trim();
                    println!("    {}", x);
                    out.push(
                        ParsedNodeDialInfo::from_str(x)
                            .wrap_err("unable to parse dial info in bootstrap node list")?,
                    );
                }
                out
            }
            None => {
                bail!("value not specified for bootstrap node list");
            }
        };
        settingsrw.core.network.bootstrap_nodes = bootstrap_list;
    }

    #[cfg(feature = "rt-tokio")]
    if matches.occurrences_of("console") != 0 {
        settingsrw.logging.console.enabled = true;
    }

    drop(settingsrw);

    // Set specific config settings
    if let Some(set_configs) = matches.values_of("set-config") {
        for set_config in set_configs {
            if let Some((k, v)) = set_config.split_once('=') {
                let k = k.trim();
                let v = v.trim();
                settings.set(k, v)?;
            }
        }
    }

    // Apply subnode index if we're testing
    settings
        .apply_subnode_index()
        .wrap_err("failed to apply subnode index")?;

    Ok((settings, matches))
}
