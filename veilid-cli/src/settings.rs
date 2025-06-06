use directories::*;

use crate::tools::*;
use serde_derive::*;
use std::ffi::OsStr;
use std::net::{SocketAddr, ToSocketAddrs};
use std::path::{Path, PathBuf};

pub fn load_default_config() -> Result<config::Config, config::ConfigError> {
    let default_config = r#"---
enable_ipc: true
ipc_path: '%IPC_DIRECTORY%'
enable_network: false
address: "localhost:5959"
autoconnect: true
autoreconnect: true
logging: 
    level: "info"
    terminal: 
        enabled: false
    file:
        enabled: true
        directory: '%LOGGING_FILE_DIRECTORY%'
        append: true
interface:
    node_log:
        scrollback: 10000
    command_line:
        history_size: 2048
    theme:
        shadow: false
        borders: "simple"
        colors:
            background         : "black"
            shadow             : "black"
            view               : "black"
            primary            : "light cyan"
            secondary          : "cyan"
            tertiary           : "green"
            title_primary      : "light magenta"
            title_secondary    : "magenta"
            highlight          : "light white"
            highlight_inactive : "white"
            highlight_text     : "black"
        log_colors:
            trace              : "light blue"
            debug              : "light green"
            info               : "white"
            warn               : "light yellow"
            error              : "light red"
    "#
    .replace(
        "%IPC_DIRECTORY%",
        &Settings::get_default_ipc_directory().to_string_lossy(),
    )
    .replace(
        "%LOGGING_FILE_DIRECTORY%",
        &Settings::get_default_log_directory().to_string_lossy(),
    );

    config::Config::builder()
        .add_source(config::File::from_str(
            &default_config,
            config::FileFormat::Yaml,
        ))
        .build()
}

pub fn load_config(
    cfg: config::Config,
    config_file: &Path,
) -> Result<config::Config, config::ConfigError> {
    if let Some(config_file_str) = config_file.to_str() {
        config::Config::builder()
            .add_source(cfg)
            .add_source(config::File::new(config_file_str, config::FileFormat::Yaml))
            .build()
    } else {
        Err(config::ConfigError::Message(
            "config file path is not valid UTF-8".to_owned(),
        ))
    }
}

#[derive(Copy, Clone, Debug)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}
impl<'de> serde::Deserialize<'de> for LogLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_ascii_lowercase().as_str() {
            "error" => Ok(LogLevel::Error),
            "warn" => Ok(LogLevel::Warn),
            "info" => Ok(LogLevel::Info),
            "debug" => Ok(LogLevel::Debug),
            "trace" => Ok(LogLevel::Trace),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid log level: {}",
                s
            ))),
        }
    }
}
pub fn convert_loglevel(log_level: LogLevel) -> log::LevelFilter {
    match log_level {
        LogLevel::Error => log::LevelFilter::Error,
        LogLevel::Warn => log::LevelFilter::Warn,
        LogLevel::Info => log::LevelFilter::Info,
        LogLevel::Debug => log::LevelFilter::Debug,
        LogLevel::Trace => log::LevelFilter::Trace,
    }
}

#[derive(Clone, Debug)]
pub struct NamedSocketAddrs {
    pub _name: String,
    pub addrs: Vec<SocketAddr>,
}

impl TryFrom<String> for NamedSocketAddrs {
    type Error = std::io::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let addrs = value.to_socket_addrs()?.collect();
        let name = value;
        Ok(NamedSocketAddrs { _name: name, addrs })
    }
}

impl<'de> serde::Deserialize<'de> for NamedSocketAddrs {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let addr_iter = s.to_socket_addrs().map_err(serde::de::Error::custom)?;
        Ok(NamedSocketAddrs {
            _name: s,
            addrs: addr_iter.collect(),
        })
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Terminal {
    pub enabled: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct File {
    pub enabled: bool,
    pub directory: String,
    pub append: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Logging {
    pub terminal: Terminal,
    pub file: File,
    pub level: LogLevel,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Colors {
    pub background: String,
    pub shadow: String,
    pub view: String,
    pub primary: String,
    pub secondary: String,
    pub tertiary: String,
    pub title_primary: String,
    pub title_secondary: String,
    pub highlight: String,
    pub highlight_inactive: String,
    pub highlight_text: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct LogColors {
    pub trace: String,
    pub debug: String,
    pub info: String,
    pub warn: String,
    pub error: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Theme {
    pub shadow: bool,
    pub borders: String,
    pub colors: Colors,
    pub log_colors: LogColors,
}

#[derive(Clone, Debug, Deserialize)]
pub struct NodeLog {
    pub scrollback: usize,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CommandLine {
    pub history_size: usize,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Interface {
    pub theme: Theme,
    pub node_log: NodeLog,
    pub command_line: CommandLine,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Settings {
    pub enable_ipc: bool,
    pub ipc_path: Option<PathBuf>,
    pub enable_network: bool,
    pub address: Option<NamedSocketAddrs>,
    pub autoconnect: bool,
    pub autoreconnect: bool,
    pub logging: Logging,
    pub interface: Interface,
}

impl Settings {
    //////////////////////////////////////////////////////////////////////////////////

    pub fn new(config_file: Option<&OsStr>) -> Result<Self, config::ConfigError> {
        // Load the default config
        let mut cfg = load_default_config()?;

        // Merge in the config file if we have one
        if let Some(config_file) = config_file {
            let config_file_path = Path::new(config_file);
            // If the user specifies a config file on the command line then it must exist
            cfg = load_config(cfg, config_file_path)?;
        }

        // Generate config
        cfg.try_deserialize()
    }

    pub fn resolve_ipc_path(
        &self,
        ipc_path: Option<PathBuf>,
        subnode_index: u16,
    ) -> Option<PathBuf> {
        let mut client_api_ipc_path = None;
        // Determine IPC path to try
        cfg_if::cfg_if! {
            if #[cfg(windows)] {
                if let Some(ipc_path) = ipc_path.or(self.ipc_path.clone()) {
                    if is_ipc_socket_path(&ipc_path) {
                        // try direct path
                        client_api_ipc_path = Some(ipc_path);
                    } else {
                        // try subnode index inside path
                        let ipc_path = ipc_path.join(subnode_index.to_string());
                        if is_ipc_socket_path(&ipc_path) {
                            // subnode indexed path exists
                            client_api_ipc_path = Some(ipc_path);
                        }
                    }
                }
            } else {
                if let Some(ipc_path) = ipc_path.or(self.ipc_path.clone()) {
                    if is_ipc_socket_path(&ipc_path) {
                        // try direct path
                        client_api_ipc_path = Some(ipc_path);
                    } else if ipc_path.exists() && ipc_path.is_dir() {
                        // try subnode index inside path
                        let ipc_path = ipc_path.join(subnode_index.to_string());
                        if is_ipc_socket_path(&ipc_path) {
                            // subnode indexed path exists
                            client_api_ipc_path = Some(ipc_path);
                        }
                    }
                }
            }
        }
        client_api_ipc_path
    }

    pub fn resolve_network_address(
        &self,
        address: Option<String>,
    ) -> Result<Option<Vec<SocketAddr>>, String> {
        let mut client_api_network_addresses = None;

        let args_address = if let Some(args_address) = address {
            match NamedSocketAddrs::try_from(args_address) {
                Ok(v) => Some(v),
                Err(e) => {
                    return Err(format!("Invalid server address: {}", e));
                }
            }
        } else {
            None
        };
        if let Some(address_arg) = args_address.or(self.address.clone()) {
            client_api_network_addresses = Some(address_arg.addrs);
        } else if let Some(address) = self.address.clone() {
            client_api_network_addresses = Some(address.addrs.clone());
        }
        Ok(client_api_network_addresses)
    }

    ////////////////////////////////////////////////////////////////////////////
    #[cfg_attr(windows, expect(dead_code))]
    fn get_server_default_directory(subpath: &str) -> PathBuf {
        #[cfg(unix)]
        {
            let globalpath = PathBuf::from("/var/db/veilid-server").join(subpath);
            if globalpath.is_dir() {
                return globalpath;
            }
        }

        let mut ts_path = if let Some(my_proj_dirs) = ProjectDirs::from("org", "Veilid", "Veilid") {
            PathBuf::from(my_proj_dirs.data_local_dir())
        } else {
            PathBuf::from("./")
        };
        ts_path.push(subpath);

        ts_path
    }

    pub fn get_default_ipc_directory() -> PathBuf {
        cfg_if::cfg_if! {
            if #[cfg(windows)] {
                PathBuf::from(r"\\.\PIPE\veilid-server")
            } else {
                Self::get_server_default_directory("ipc")
            }
        }
    }

    pub fn get_default_config_path() -> PathBuf {
        // Get default configuration file location
        let mut default_config_path =
            if let Some(my_proj_dirs) = ProjectDirs::from("org", "Veilid", "Veilid") {
                PathBuf::from(my_proj_dirs.config_dir())
            } else {
                PathBuf::from("./")
            };
        default_config_path.push("veilid-client.conf");

        default_config_path
    }

    pub fn get_default_log_directory() -> PathBuf {
        // Get default configuration file location
        let mut default_log_directory =
            if let Some(my_proj_dirs) = ProjectDirs::from("org", "Veilid", "Veilid") {
                PathBuf::from(my_proj_dirs.config_dir())
            } else {
                PathBuf::from("./")
            };
        default_log_directory.push("logs/");

        default_log_directory
    }
}

#[test]
fn test_default_config() {
    let cfg = load_default_config().unwrap();
    let settings = cfg.try_deserialize::<Settings>().unwrap();

    println!("default settings: {:?}", settings);
}
