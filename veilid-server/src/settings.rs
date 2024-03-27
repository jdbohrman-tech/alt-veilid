use clap::ValueEnum;
use directories::*;

use crate::tools::*;
use serde_derive::*;
use std::ffi::OsStr;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use url::Url;
use veilid_core::tools::*;
use veilid_core::*;

use lazy_static::*;

lazy_static! {
    static ref SYSTEM: sysinfo::System = {
        sysinfo::System::new_with_specifics(
            sysinfo::RefreshKind::new().with_memory(sysinfo::MemoryRefreshKind::everything()),
        )
    };
    static ref DISKS: sysinfo::Disks = {
        let mut disks = sysinfo::Disks::new_with_refreshed_list();
        disks.sort_by(|a, b| {
            b.mount_point()
                .to_string_lossy()
                .len()
                .cmp(&a.mount_point().to_string_lossy().len())
        });
        disks
    };
}

pub fn load_default_config() -> EyreResult<config::Config> {
    let mut default_config = String::from(
        r#"---
daemon:
    enabled: false
client_api:
    ipc_enabled: true
    ipc_directory: '%IPC_DIRECTORY%'
    network_enabled: false
    listen_address: 'localhost:5959'
auto_attach: true
logging:
    system:
        enabled: false
        level: 'info'
        ignore_log_targets: []
    terminal:
        enabled: true
        level: 'info'
        ignore_log_targets: []
    file: 
        enabled: false
        path: ''
        append: true
        level: 'info'
        ignore_log_targets: []
    api:
        enabled: true
        level: 'info'
        ignore_log_targets: []
    otlp:
        enabled: false
        level: 'trace'
        grpc_endpoint: 'localhost:4317'
        ignore_log_targets: []
    console:
        enabled: false
testing:
    subnode_index: 0
core:
    capabilities:
        disable: []
    protected_store:
        allow_insecure_fallback: true
        always_use_insecure_storage: true
        directory: '%DIRECTORY%'
        delete: false
        device_encryption_key_password: '%DEVICE_ENCRYPTION_KEY_PASSWORD%'
        new_device_encryption_key_password: %NEW_DEVICE_ENCRYPTION_KEY_PASSWORD%
    table_store:
        directory: '%TABLE_STORE_DIRECTORY%'
        delete: false
    block_store:
        directory: '%BLOCK_STORE_DIRECTORY%'
        delete: false
    network:
        connection_initial_timeout_ms: 2000
        connection_inactivity_timeout_ms: 60000
        max_connections_per_ip4: 32
        max_connections_per_ip6_prefix: 32
        max_connections_per_ip6_prefix_size: 56
        max_connection_frequency_per_min: 128
        client_allowlist_timeout_ms: 300000 
        reverse_connection_receipt_time_ms: 5000 
        hole_punch_receipt_time_ms: 5000 
        network_key_password: null
        disable_capabilites: []
        routing_table:
            node_id: null
            node_id_secret: null
            bootstrap: ['bootstrap.veilid.net']
            limit_over_attached: 64
            limit_fully_attached: 32
            limit_attached_strong: 16
            limit_attached_good: 8
            limit_attached_weak: 4
        rpc: 
            concurrency: 0
            queue_size: 1024
            max_timestamp_behind_ms: 10000
            max_timestamp_ahead_ms: 10000
            timeout_ms: 5000
            max_route_hop_count: 4
            default_route_hop_count: 1
        dht:
            max_find_node_count: 20
            resolve_node_timeout_ms: 10000
            resolve_node_count: 1
            resolve_node_fanout: 4
            get_value_timeout_ms: 10000
            get_value_count: 3
            get_value_fanout: 4
            set_value_timeout_ms: 10000
            set_value_count: 5
            set_value_fanout: 4
            min_peer_count: 20
            min_peer_refresh_time_ms: 60000
            validate_dial_info_receipt_time_ms: 2000
            local_subkey_cache_size: 128
            local_max_subkey_cache_memory_mb: 256
            remote_subkey_cache_size: 1024
            remote_max_records: 65536
            remote_max_subkey_cache_memory_mb: %REMOTE_MAX_SUBKEY_CACHE_MEMORY_MB%
            remote_max_storage_space_mb: 0
            public_watch_limit: 32
            member_watch_limit: 8
            max_watch_expiration_ms: 600000
        upnp: true
        detect_address_changes: true
        restricted_nat_retries: 0
        tls:
            certificate_path: '%CERTIFICATE_PATH%'
            private_key_path: '%PRIVATE_KEY_PATH%'
            connection_initial_timeout_ms: 2000
        application:
            https:
                enabled: false
                listen_address: ':443'
                path: 'app'
                # url: 'https://localhost'
            http:
                enabled: false
                listen_address: ':80'
                path: 'app'
                # url: 'http://localhost'
        protocol:
            udp:
                enabled: true
                socket_pool_size: 0
                listen_address: ''
                # public_address: ''
            tcp:
                connect: true
                listen: true
                max_connections: 32
                listen_address: ''
                #'public_address: ''
            ws:
                connect: true
                listen: true
                max_connections: 32
                listen_address: ''
                path: 'ws'
                # url: 'ws://localhost:5150/ws'
            wss:
                connect: true
                listen: false
                max_connections: 32
                listen_address: ''
                path: 'ws'
                # url: ''
        "#,
    )
    .replace(
        "%IPC_DIRECTORY%",
        &Settings::get_default_ipc_directory().to_string_lossy(),
    )
    .replace(
        "%TABLE_STORE_DIRECTORY%",
        &VeilidConfigTableStore::default().directory,
    )
    .replace(
        "%BLOCK_STORE_DIRECTORY%",
        &VeilidConfigBlockStore::default().directory,
    )
    .replace(
        "%DIRECTORY%",
        &VeilidConfigProtectedStore::default().directory,
    )
    .replace(
        "%CERTIFICATE_PATH%",
        &VeilidConfigTLS::default().certificate_path,
    )
    .replace(
        "%PRIVATE_KEY_PATH%",
        &VeilidConfigTLS::default().private_key_path,
    )
    .replace(
        "%REMOTE_MAX_SUBKEY_CACHE_MEMORY_MB%",
        &Settings::get_default_remote_max_subkey_cache_memory_mb().to_string(),
    );

    let dek_password = if let Some(dek_password) = std::env::var_os("DEK_PASSWORD") {
        dek_password
            .to_str()
            .ok_or_else(|| eyre!("DEK_PASSWORD is not valid unicode"))?
            .to_owned()
    } else {
        "".to_owned()
    };
    default_config = default_config.replace("%DEVICE_ENCRYPTION_KEY_PASSWORD%", &dek_password);

    let new_dek_password = if let Some(new_dek_password) = std::env::var_os("NEW_DEK_PASSWORD") {
        format!(
            "'{}'",
            new_dek_password
                .to_str()
                .ok_or_else(|| eyre!("NEW_DEK_PASSWORD is not valid unicode"))?
        )
    } else {
        "null".to_owned()
    };
    default_config =
        default_config.replace("%NEW_DEVICE_ENCRYPTION_KEY_PASSWORD%", &new_dek_password);

    config::Config::builder()
        .add_source(config::File::from_str(
            &default_config,
            config::FileFormat::Yaml,
        ))
        .build()
        .wrap_err("failed to parse default config")
}

pub fn load_config(cfg: config::Config, config_file: &Path) -> EyreResult<config::Config> {
    if let Some(config_file_str) = config_file.to_str() {
        config::Config::builder()
            .add_source(cfg)
            .add_source(config::File::new(config_file_str, config::FileFormat::Yaml))
            .build()
            .wrap_err("failed to load config")
    } else {
        bail!("config file path is not valid UTF-8")
    }
}

#[derive(Copy, Clone, Debug, PartialEq, ValueEnum)]
pub enum LogLevel {
    Off,
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
            "off" => Ok(LogLevel::Off),
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
impl serde::Serialize for LogLevel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = match self {
            LogLevel::Off => "off",
            LogLevel::Error => "error",
            LogLevel::Warn => "warn",
            LogLevel::Info => "info",
            LogLevel::Debug => "debug",
            LogLevel::Trace => "trace",
        };
        s.serialize(serializer)
    }
}

pub fn convert_loglevel(log_level: LogLevel) -> veilid_core::VeilidConfigLogLevel {
    match log_level {
        LogLevel::Off => veilid_core::VeilidConfigLogLevel::Off,
        LogLevel::Error => veilid_core::VeilidConfigLogLevel::Error,
        LogLevel::Warn => veilid_core::VeilidConfigLogLevel::Warn,
        LogLevel::Info => veilid_core::VeilidConfigLogLevel::Info,
        LogLevel::Debug => veilid_core::VeilidConfigLogLevel::Debug,
        LogLevel::Trace => veilid_core::VeilidConfigLogLevel::Trace,
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParsedUrl {
    pub urlstring: String,
    pub url: Url,
}

impl ParsedUrl {
    pub fn offset_port(&mut self, offset: u16) -> EyreResult<()> {
        // Bump port on url
        self.url
            .set_port(Some(self.url.port().unwrap() + offset))
            .map_err(|_| eyre!("failed to set port on url"))?;
        self.urlstring = self.url.to_string();
        Ok(())
    }
}

impl FromStr for ParsedUrl {
    type Err = url::ParseError;
    fn from_str(s: &str) -> Result<ParsedUrl, url::ParseError> {
        let mut url = Url::parse(s)?;
        if url.scheme().to_lowercase() == "http" && url.port().is_none() {
            url.set_port(Some(80))
                .map_err(|_| url::ParseError::InvalidPort)?
        }
        if url.scheme().to_lowercase() == "https" && url.port().is_none() {
            url.set_port(Some(443))
                .map_err(|_| url::ParseError::InvalidPort)?;
        }
        let parsed_urlstring = url.to_string();
        Ok(Self {
            urlstring: parsed_urlstring,
            url,
        })
    }
}

impl<'de> serde::Deserialize<'de> for ParsedUrl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ParsedUrl::from_str(s.as_str()).map_err(serde::de::Error::custom)
    }
}

impl serde::Serialize for ParsedUrl {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.urlstring.serialize(serializer)
    }
}

#[derive(Debug, PartialEq)]
pub struct NamedSocketAddrs {
    pub name: String,
    pub addrs: Vec<SocketAddr>,
}

impl FromStr for NamedSocketAddrs {
    type Err = std::io::Error;
    fn from_str(s: &str) -> Result<NamedSocketAddrs, std::io::Error> {
        if s.is_empty() {
            return Ok(NamedSocketAddrs {
                name: String::new(),
                addrs: vec![],
            });
        }
        let addr_iter = listen_address_to_socket_addrs(s)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?;
        Ok(NamedSocketAddrs {
            name: s.to_owned(),
            addrs: addr_iter,
        })
    }
}

impl<'de> serde::Deserialize<'de> for NamedSocketAddrs {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NamedSocketAddrs::from_str(s.as_str()).map_err(serde::de::Error::custom)
    }
}

impl serde::Serialize for NamedSocketAddrs {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.name.serialize(serializer)
    }
}

impl NamedSocketAddrs {
    pub fn offset_port(&mut self, offset: u16) -> EyreResult<bool> {
        // Bump port on name
        if let Some(split) = self.name.rfind(':') {
            let hoststr = &self.name[0..split];
            let portstr = &self.name[split + 1..];
            let port: u16 = portstr.parse::<u16>().wrap_err("failed to parse port")? + offset;

            self.name = format!("{}:{}", hoststr, port);
        } else {
            return Ok(false);
        }

        // Bump port on addresses
        for addr in self.addrs.iter_mut() {
            addr.set_port(addr.port() + offset);
        }

        Ok(true)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Terminal {
    pub enabled: bool,
    pub level: LogLevel,
    pub ignore_log_targets: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Console {
    pub enabled: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct File {
    pub enabled: bool,
    pub path: String,
    pub append: bool,
    pub level: LogLevel,
    pub ignore_log_targets: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct System {
    pub enabled: bool,
    pub level: LogLevel,
    pub ignore_log_targets: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Api {
    pub enabled: bool,
    pub level: LogLevel,
    pub ignore_log_targets: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Otlp {
    pub enabled: bool,
    pub level: LogLevel,
    pub grpc_endpoint: NamedSocketAddrs,
    pub ignore_log_targets: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClientApi {
    pub ipc_enabled: bool,
    pub ipc_directory: PathBuf,
    pub network_enabled: bool,
    pub listen_address: NamedSocketAddrs,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Logging {
    pub system: System,
    pub terminal: Terminal,
    pub file: File,
    pub api: Api,
    pub otlp: Otlp,
    pub console: Console,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Https {
    pub enabled: bool,
    pub listen_address: NamedSocketAddrs,
    pub path: PathBuf,
    pub url: Option<ParsedUrl>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Http {
    pub enabled: bool,
    pub listen_address: NamedSocketAddrs,
    pub path: PathBuf,
    pub url: Option<ParsedUrl>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Application {
    pub https: Https,
    pub http: Http,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Udp {
    pub enabled: bool,
    pub socket_pool_size: u32,
    pub listen_address: NamedSocketAddrs,
    pub public_address: Option<NamedSocketAddrs>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Tcp {
    pub connect: bool,
    pub listen: bool,
    pub max_connections: u32,
    pub listen_address: NamedSocketAddrs,
    pub public_address: Option<NamedSocketAddrs>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Ws {
    pub connect: bool,
    pub listen: bool,
    pub max_connections: u32,
    pub listen_address: NamedSocketAddrs,
    pub path: PathBuf,
    pub url: Option<ParsedUrl>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Wss {
    pub connect: bool,
    pub listen: bool,
    pub max_connections: u32,
    pub listen_address: NamedSocketAddrs,
    pub path: PathBuf,
    pub url: Option<ParsedUrl>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Protocol {
    pub udp: Udp,
    pub tcp: Tcp,
    pub ws: Ws,
    pub wss: Wss,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Tls {
    pub certificate_path: String,
    pub private_key_path: String,
    pub connection_initial_timeout_ms: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Rpc {
    pub concurrency: u32,
    pub queue_size: u32,
    pub max_timestamp_behind_ms: Option<u32>,
    pub max_timestamp_ahead_ms: Option<u32>,
    pub timeout_ms: u32,
    pub max_route_hop_count: u8,
    pub default_route_hop_count: u8,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Dht {
    pub max_find_node_count: u32,
    pub resolve_node_timeout_ms: u32,
    pub resolve_node_count: u32,
    pub resolve_node_fanout: u32,
    pub get_value_timeout_ms: u32,
    pub get_value_count: u32,
    pub get_value_fanout: u32,
    pub set_value_timeout_ms: u32,
    pub set_value_count: u32,
    pub set_value_fanout: u32,
    pub min_peer_count: u32,
    pub min_peer_refresh_time_ms: u32,
    pub validate_dial_info_receipt_time_ms: u32,
    pub local_subkey_cache_size: u32,
    pub local_max_subkey_cache_memory_mb: u32,
    pub remote_subkey_cache_size: u32,
    pub remote_max_records: u32,
    pub remote_max_subkey_cache_memory_mb: u32,
    pub remote_max_storage_space_mb: u32,
    pub public_watch_limit: u32,
    pub member_watch_limit: u32,
    pub max_watch_expiration_ms: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RoutingTable {
    pub node_id: Option<veilid_core::TypedKeyGroup>,
    pub node_id_secret: Option<veilid_core::TypedSecretGroup>,
    pub bootstrap: Vec<String>,
    pub limit_over_attached: u32,
    pub limit_fully_attached: u32,
    pub limit_attached_strong: u32,
    pub limit_attached_good: u32,
    pub limit_attached_weak: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Network {
    pub connection_initial_timeout_ms: u32,
    pub connection_inactivity_timeout_ms: u32,
    pub max_connections_per_ip4: u32,
    pub max_connections_per_ip6_prefix: u32,
    pub max_connections_per_ip6_prefix_size: u32,
    pub max_connection_frequency_per_min: u32,
    pub client_allowlist_timeout_ms: u32,
    pub reverse_connection_receipt_time_ms: u32,
    pub hole_punch_receipt_time_ms: u32,
    pub network_key_password: Option<String>,
    pub routing_table: RoutingTable,
    pub rpc: Rpc,
    pub dht: Dht,
    pub upnp: bool,
    pub detect_address_changes: bool,
    pub restricted_nat_retries: u32,
    pub tls: Tls,
    pub application: Application,
    pub protocol: Protocol,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Testing {
    pub subnode_index: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TableStore {
    pub directory: String,
    pub delete: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BlockStore {
    pub directory: String,
    pub delete: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProtectedStore {
    pub allow_insecure_fallback: bool,
    pub always_use_insecure_storage: bool,
    pub directory: String,
    pub delete: bool,
    pub device_encryption_key_password: String,
    pub new_device_encryption_key_password: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Capabilities {
    pub disable: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Core {
    pub capabilities: Capabilities,
    pub protected_store: ProtectedStore,
    pub table_store: TableStore,
    pub block_store: BlockStore,
    pub network: Network,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Daemon {
    pub enabled: bool,
    pub pid_file: Option<String>,
    pub chroot: Option<String>,
    pub working_directory: Option<String>,
    pub user: Option<String>,
    pub group: Option<String>,
    pub stdout_file: Option<String>,
    pub stderr_file: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SettingsInner {
    pub daemon: Daemon,
    pub client_api: ClientApi,
    pub auto_attach: bool,
    pub logging: Logging,
    pub testing: Testing,
    pub core: Core,
}

#[derive(Clone, Debug)]
pub struct Settings {
    inner: Arc<RwLock<SettingsInner>>,
}

impl Settings {
    pub fn new(config_file: Option<&OsStr>) -> EyreResult<Self> {
        // Load the default config
        let mut cfg = load_default_config()?;

        // Merge in the config file if we have one
        if let Some(config_file) = config_file {
            let config_file_path = Path::new(config_file);
            // If the user specifies a config file on the command line then it must exist
            cfg = load_config(cfg, config_file_path)?;
        }

        // Generate config
        let mut inner: SettingsInner = cfg.try_deserialize()?;

        // Fill in missing defaults
        if inner.core.network.dht.remote_max_storage_space_mb == 0 {
            inner.core.network.dht.remote_max_storage_space_mb =
                Self::get_default_remote_max_storage_space_mb(&inner);
        }

        //
        Ok(Self {
            inner: Arc::new(RwLock::new(inner)),
        })
    }
    pub fn read(&self) -> RwLockReadGuard<SettingsInner> {
        self.inner.read()
    }
    pub fn write(&self) -> RwLockWriteGuard<SettingsInner> {
        self.inner.write()
    }

    pub fn apply_subnode_index(&self) -> EyreResult<()> {
        let mut settingsrw = self.write();
        let idx = settingsrw.testing.subnode_index;
        if idx == 0 {
            return Ok(());
        }

        // bump client api port
        settingsrw.client_api.listen_address.offset_port(idx)?;

        // bump protocol ports
        settingsrw
            .core
            .network
            .protocol
            .udp
            .listen_address
            .offset_port(idx)?;
        settingsrw
            .core
            .network
            .protocol
            .tcp
            .listen_address
            .offset_port(idx)?;
        settingsrw
            .core
            .network
            .protocol
            .ws
            .listen_address
            .offset_port(idx)?;
        if let Some(url) = &mut settingsrw.core.network.protocol.ws.url {
            url.offset_port(idx)?;
        }
        settingsrw
            .core
            .network
            .protocol
            .wss
            .listen_address
            .offset_port(idx)?;
        if let Some(url) = &mut settingsrw.core.network.protocol.wss.url {
            url.offset_port(idx)?;
        }
        // bump application ports
        settingsrw
            .core
            .network
            .application
            .http
            .listen_address
            .offset_port(idx)?;
        if let Some(url) = &mut settingsrw.core.network.application.http.url {
            url.offset_port(idx)?;
        }
        settingsrw
            .core
            .network
            .application
            .https
            .listen_address
            .offset_port(idx)?;
        if let Some(url) = &mut settingsrw.core.network.application.https.url {
            url.offset_port(idx)?;
        }
        Ok(())
    }

    /// Determine default config path
    ///
    /// In a unix-like environment, veilid-server will look for its config file
    /// in /etc/veilid-server. If a config is not found in this location, it will
    /// follow the XDG user directory spec, and look in `~/.config/veilid-server/`.
    ///
    /// For Windows, a user-local config may be created at
    /// `C:\Users\<user>\AppData\Roaming\Veilid\Veilid`, and for macOS, at
    /// `/Users/<user>/Library/Application Support/org.Veilid.Veilid`
    ///
    pub fn get_default_config_path() -> PathBuf {
        #[cfg(unix)]
        {
            let default_path = PathBuf::from("/etc/veilid-server/veilid-server.conf");
            if default_path.exists() {
                return default_path;
            }
        }

        ProjectDirs::from("org", "Veilid", "Veilid")
            .map(|dirs| dirs.config_dir().join("veilid-server.conf"))
            .unwrap_or_else(|| PathBuf::from("./veilid-server.conf"))
    }

    #[allow(dead_code)]
    fn get_or_create_private_directory<P: AsRef<Path>>(path: P, group_read: bool) -> bool {
        let path = path.as_ref();
        if !path.is_dir()
            && (std::fs::create_dir_all(path).is_err()
                || ensure_directory_private_owner(path, group_read).is_err())
        {
            return false;
        }
        true
    }

    #[allow(dead_code)]
    fn get_or_create_default_directory(subpath: &str) -> PathBuf {
        #[cfg(unix)]
        {
            let globalpath = PathBuf::from("/var/db/veilid-server").join(subpath);

            if Self::get_or_create_private_directory(&globalpath, true) {
                return globalpath;
            }
        }

        let mut ts_path = if let Some(my_proj_dirs) = ProjectDirs::from("org", "Veilid", "Veilid") {
            PathBuf::from(my_proj_dirs.data_local_dir())
        } else {
            PathBuf::from("./")
        };
        ts_path.push(subpath);

        if Self::get_or_create_private_directory(&ts_path, true) {
            return ts_path;
        }

        panic!("Failed to create private directory for '{}'", subpath);
    }

    pub fn get_default_ipc_directory() -> PathBuf {
        cfg_if! {
            if #[cfg(windows)] {
                PathBuf::from(r"\\.\PIPE\veilid-server")
            } else {
                Self::get_or_create_default_directory("ipc")
            }
        }
    }

    pub fn get_default_remote_max_subkey_cache_memory_mb() -> u32 {
        if sysinfo::IS_SUPPORTED_SYSTEM {
            ((SYSTEM.free_memory() / (1024u64 * 1024u64)) / 16) as u32
        } else {
            256
        }
    }

    pub fn get_default_remote_max_storage_space_mb(inner: &SettingsInner) -> u32 {
        let dht_storage_path = inner.core.table_store.directory.clone();
        // Sort longer mount point paths first since we want the mount point closest to our table store directory

        if sysinfo::IS_SUPPORTED_SYSTEM {
            for disk in DISKS.list() {
                if dht_storage_path.starts_with(&*disk.mount_point().to_string_lossy()) {
                    let available_mb = disk.available_space() / 1_000_000u64;
                    if available_mb > 40_000 {
                        // Default to 10GB if more than 40GB is available
                        return 10_000;
                    }
                    // Default to 1/4 of the available space, if less than 40GB is available
                    return available_mb as u32;
                }
            }
        }

        // If we can't figure out our storage path go with 1GB of space and pray
        1_000
    }

    pub fn set(&self, key: &str, value: &str) -> EyreResult<()> {
        let mut inner = self.inner.write();

        macro_rules! set_config_value {
            ($innerkey:expr, $value:expr) => {{
                let innerkeyname = &stringify!($innerkey)[6..];
                if innerkeyname == key {
                    match veilid_core::deserialize_json(value) {
                        Ok(v) => {
                            $innerkey = v;
                            return Ok(());
                        }
                        Err(e) => {
                            return Err(eyre!(
                                "invalid type for key {}, value: {}: {}",
                                key,
                                value,
                                e
                            ))
                        }
                    }
                }
            }};
        }

        set_config_value!(inner.daemon.enabled, value);
        set_config_value!(inner.client_api.ipc_enabled, value);
        set_config_value!(inner.client_api.ipc_directory, value);
        set_config_value!(inner.client_api.network_enabled, value);
        set_config_value!(inner.client_api.listen_address, value);
        set_config_value!(inner.auto_attach, value);
        set_config_value!(inner.logging.system.enabled, value);
        set_config_value!(inner.logging.system.level, value);
        set_config_value!(inner.logging.system.ignore_log_targets, value);
        set_config_value!(inner.logging.terminal.enabled, value);
        set_config_value!(inner.logging.terminal.level, value);
        set_config_value!(inner.logging.terminal.ignore_log_targets, value);
        set_config_value!(inner.logging.file.enabled, value);
        set_config_value!(inner.logging.file.path, value);
        set_config_value!(inner.logging.file.append, value);
        set_config_value!(inner.logging.file.level, value);
        set_config_value!(inner.logging.file.ignore_log_targets, value);
        set_config_value!(inner.logging.api.enabled, value);
        set_config_value!(inner.logging.api.level, value);
        set_config_value!(inner.logging.api.ignore_log_targets, value);
        set_config_value!(inner.logging.otlp.enabled, value);
        set_config_value!(inner.logging.otlp.level, value);
        set_config_value!(inner.logging.otlp.grpc_endpoint, value);
        set_config_value!(inner.logging.otlp.ignore_log_targets, value);
        set_config_value!(inner.logging.console.enabled, value);
        set_config_value!(inner.testing.subnode_index, value);
        set_config_value!(inner.core.capabilities.disable, value);
        set_config_value!(inner.core.protected_store.allow_insecure_fallback, value);
        set_config_value!(
            inner.core.protected_store.always_use_insecure_storage,
            value
        );
        set_config_value!(inner.core.protected_store.directory, value);
        set_config_value!(inner.core.protected_store.delete, value);
        set_config_value!(
            inner.core.protected_store.device_encryption_key_password,
            value
        );
        set_config_value!(
            inner
                .core
                .protected_store
                .new_device_encryption_key_password,
            value
        );
        set_config_value!(inner.core.table_store.directory, value);
        set_config_value!(inner.core.table_store.delete, value);
        set_config_value!(inner.core.block_store.directory, value);
        set_config_value!(inner.core.block_store.delete, value);
        set_config_value!(inner.core.network.connection_initial_timeout_ms, value);
        set_config_value!(inner.core.network.connection_inactivity_timeout_ms, value);
        set_config_value!(inner.core.network.max_connections_per_ip4, value);
        set_config_value!(inner.core.network.max_connections_per_ip6_prefix, value);
        set_config_value!(
            inner.core.network.max_connections_per_ip6_prefix_size,
            value
        );
        set_config_value!(inner.core.network.max_connection_frequency_per_min, value);
        set_config_value!(inner.core.network.client_allowlist_timeout_ms, value);
        set_config_value!(inner.core.network.reverse_connection_receipt_time_ms, value);
        set_config_value!(inner.core.network.hole_punch_receipt_time_ms, value);
        set_config_value!(inner.core.network.network_key_password, value);
        set_config_value!(inner.core.network.routing_table.node_id, value);
        set_config_value!(inner.core.network.routing_table.node_id_secret, value);
        set_config_value!(inner.core.network.routing_table.bootstrap, value);
        set_config_value!(inner.core.network.routing_table.limit_over_attached, value);
        set_config_value!(inner.core.network.routing_table.limit_fully_attached, value);
        set_config_value!(
            inner.core.network.routing_table.limit_attached_strong,
            value
        );
        set_config_value!(inner.core.network.routing_table.limit_attached_good, value);
        set_config_value!(inner.core.network.routing_table.limit_attached_weak, value);
        set_config_value!(inner.core.network.rpc.concurrency, value);
        set_config_value!(inner.core.network.rpc.queue_size, value);
        set_config_value!(inner.core.network.rpc.max_timestamp_behind_ms, value);
        set_config_value!(inner.core.network.rpc.max_timestamp_ahead_ms, value);
        set_config_value!(inner.core.network.rpc.timeout_ms, value);
        set_config_value!(inner.core.network.rpc.max_route_hop_count, value);
        set_config_value!(inner.core.network.rpc.default_route_hop_count, value);
        set_config_value!(inner.core.network.dht.max_find_node_count, value);
        set_config_value!(inner.core.network.dht.resolve_node_timeout_ms, value);
        set_config_value!(inner.core.network.dht.resolve_node_count, value);
        set_config_value!(inner.core.network.dht.resolve_node_fanout, value);
        set_config_value!(inner.core.network.dht.get_value_timeout_ms, value);
        set_config_value!(inner.core.network.dht.get_value_count, value);
        set_config_value!(inner.core.network.dht.get_value_fanout, value);
        set_config_value!(inner.core.network.dht.set_value_timeout_ms, value);
        set_config_value!(inner.core.network.dht.set_value_count, value);
        set_config_value!(inner.core.network.dht.set_value_fanout, value);
        set_config_value!(inner.core.network.dht.min_peer_count, value);
        set_config_value!(inner.core.network.dht.min_peer_refresh_time_ms, value);
        set_config_value!(
            inner.core.network.dht.validate_dial_info_receipt_time_ms,
            value
        );
        set_config_value!(inner.core.network.dht.local_subkey_cache_size, value);
        set_config_value!(
            inner.core.network.dht.local_max_subkey_cache_memory_mb,
            value
        );
        set_config_value!(inner.core.network.dht.remote_subkey_cache_size, value);
        set_config_value!(inner.core.network.dht.remote_max_records, value);
        set_config_value!(
            inner.core.network.dht.remote_max_subkey_cache_memory_mb,
            value
        );
        set_config_value!(inner.core.network.dht.remote_max_storage_space_mb, value);
        set_config_value!(inner.core.network.dht.public_watch_limit, value);
        set_config_value!(inner.core.network.dht.member_watch_limit, value);
        set_config_value!(inner.core.network.dht.max_watch_expiration_ms, value);
        set_config_value!(inner.core.network.upnp, value);
        set_config_value!(inner.core.network.detect_address_changes, value);
        set_config_value!(inner.core.network.restricted_nat_retries, value);
        set_config_value!(inner.core.network.tls.certificate_path, value);
        set_config_value!(inner.core.network.tls.private_key_path, value);
        set_config_value!(inner.core.network.tls.connection_initial_timeout_ms, value);
        set_config_value!(inner.core.network.application.https.enabled, value);
        set_config_value!(inner.core.network.application.https.listen_address, value);
        set_config_value!(inner.core.network.application.https.path, value);
        set_config_value!(inner.core.network.application.https.url, value);
        set_config_value!(inner.core.network.application.http.enabled, value);
        set_config_value!(inner.core.network.application.http.listen_address, value);
        set_config_value!(inner.core.network.application.http.path, value);
        set_config_value!(inner.core.network.application.http.url, value);
        set_config_value!(inner.core.network.protocol.udp.enabled, value);
        set_config_value!(inner.core.network.protocol.udp.socket_pool_size, value);
        set_config_value!(inner.core.network.protocol.udp.listen_address, value);
        set_config_value!(inner.core.network.protocol.udp.public_address, value);
        set_config_value!(inner.core.network.protocol.tcp.connect, value);
        set_config_value!(inner.core.network.protocol.tcp.listen, value);
        set_config_value!(inner.core.network.protocol.tcp.max_connections, value);
        set_config_value!(inner.core.network.protocol.tcp.listen_address, value);
        set_config_value!(inner.core.network.protocol.tcp.public_address, value);
        set_config_value!(inner.core.network.protocol.ws.connect, value);
        set_config_value!(inner.core.network.protocol.ws.listen, value);
        set_config_value!(inner.core.network.protocol.ws.max_connections, value);
        set_config_value!(inner.core.network.protocol.ws.listen_address, value);
        set_config_value!(inner.core.network.protocol.ws.path, value);
        set_config_value!(inner.core.network.protocol.ws.url, value);
        set_config_value!(inner.core.network.protocol.wss.connect, value);
        set_config_value!(inner.core.network.protocol.wss.listen, value);
        set_config_value!(inner.core.network.protocol.wss.max_connections, value);
        set_config_value!(inner.core.network.protocol.wss.listen_address, value);
        set_config_value!(inner.core.network.protocol.wss.path, value);
        set_config_value!(inner.core.network.protocol.wss.url, value);
        Err(eyre!("settings key not found"))
    }

    pub fn get_core_config_callback(&self) -> veilid_core::ConfigCallback {
        let inner = self.inner.clone();

        Arc::new(move |key: String| {
            let inner = inner.read();
            let out: ConfigCallbackReturn = match key.as_str() {
                "program_name" => Ok(Box::new("veilid-server".to_owned())),
                "namespace" => Ok(Box::new(if inner.testing.subnode_index == 0 {
                    "".to_owned()
                } else {
                    format!("subnode{}", inner.testing.subnode_index)
                })),
                "capabilities.disable" => {
                    let mut caps = Vec::<FourCC>::new();
                    for c in &inner.core.capabilities.disable {
                        let cap = FourCC::from_str(c.as_str()).map_err(VeilidAPIError::generic)?;
                        caps.push(cap);
                    }
                    Ok(Box::new(caps))
                }
                "protected_store.allow_insecure_fallback" => {
                    Ok(Box::new(inner.core.protected_store.allow_insecure_fallback))
                }
                "protected_store.always_use_insecure_storage" => Ok(Box::new(
                    inner.core.protected_store.always_use_insecure_storage,
                )),
                "protected_store.directory" => {
                    Ok(Box::new(inner.core.protected_store.directory.clone()))
                }
                "protected_store.delete" => Ok(Box::new(inner.core.protected_store.delete)),
                "protected_store.device_encryption_key_password" => Ok(Box::new(
                    inner
                        .core
                        .protected_store
                        .device_encryption_key_password
                        .clone(),
                )),
                "protected_store.new_device_encryption_key_password" => Ok(Box::new(
                    inner
                        .core
                        .protected_store
                        .new_device_encryption_key_password
                        .clone(),
                )),

                "table_store.directory" => Ok(Box::new(inner.core.table_store.directory.clone())),
                "table_store.delete" => Ok(Box::new(inner.core.table_store.delete)),

                "block_store.directory" => Ok(Box::new(inner.core.block_store.directory.clone())),
                "block_store.delete" => Ok(Box::new(inner.core.block_store.delete)),

                "network.connection_initial_timeout_ms" => {
                    Ok(Box::new(inner.core.network.connection_initial_timeout_ms))
                }
                "network.connection_inactivity_timeout_ms" => Ok(Box::new(
                    inner.core.network.connection_inactivity_timeout_ms,
                )),
                "network.max_connections_per_ip4" => {
                    Ok(Box::new(inner.core.network.max_connections_per_ip4))
                }
                "network.max_connections_per_ip6_prefix" => {
                    Ok(Box::new(inner.core.network.max_connections_per_ip6_prefix))
                }
                "network.max_connections_per_ip6_prefix_size" => Ok(Box::new(
                    inner.core.network.max_connections_per_ip6_prefix_size,
                )),
                "network.max_connection_frequency_per_min" => Ok(Box::new(
                    inner.core.network.max_connection_frequency_per_min,
                )),
                "network.client_allowlist_timeout_ms" => {
                    Ok(Box::new(inner.core.network.client_allowlist_timeout_ms))
                }
                "network.reverse_connection_receipt_time_ms" => Ok(Box::new(
                    inner.core.network.reverse_connection_receipt_time_ms,
                )),
                "network.hole_punch_receipt_time_ms" => {
                    Ok(Box::new(inner.core.network.hole_punch_receipt_time_ms))
                }
                "network.network_key_password" => {
                    Ok(Box::new(inner.core.network.network_key_password.clone()))
                }
                "network.routing_table.node_id" => Ok(Box::new(
                    inner
                        .core
                        .network
                        .routing_table
                        .node_id
                        .clone()
                        .unwrap_or_default(),
                )),
                "network.routing_table.node_id_secret" => Ok(Box::new(
                    inner
                        .core
                        .network
                        .routing_table
                        .node_id_secret
                        .clone()
                        .unwrap_or_default(),
                )),
                "network.routing_table.bootstrap" => {
                    Ok(Box::new(inner.core.network.routing_table.bootstrap.clone()))
                }
                "network.routing_table.limit_over_attached" => Ok(Box::new(
                    inner.core.network.routing_table.limit_over_attached,
                )),
                "network.routing_table.limit_fully_attached" => Ok(Box::new(
                    inner.core.network.routing_table.limit_fully_attached,
                )),
                "network.routing_table.limit_attached_strong" => Ok(Box::new(
                    inner.core.network.routing_table.limit_attached_strong,
                )),
                "network.routing_table.limit_attached_good" => Ok(Box::new(
                    inner.core.network.routing_table.limit_attached_good,
                )),
                "network.routing_table.limit_attached_weak" => Ok(Box::new(
                    inner.core.network.routing_table.limit_attached_weak,
                )),
                "network.rpc.concurrency" => Ok(Box::new(inner.core.network.rpc.concurrency)),
                "network.rpc.queue_size" => Ok(Box::new(inner.core.network.rpc.queue_size)),
                "network.rpc.max_timestamp_behind_ms" => {
                    Ok(Box::new(inner.core.network.rpc.max_timestamp_behind_ms))
                }
                "network.rpc.max_timestamp_ahead_ms" => {
                    Ok(Box::new(inner.core.network.rpc.max_timestamp_ahead_ms))
                }
                "network.rpc.timeout_ms" => Ok(Box::new(inner.core.network.rpc.timeout_ms)),
                "network.rpc.max_route_hop_count" => {
                    Ok(Box::new(inner.core.network.rpc.max_route_hop_count))
                }
                "network.rpc.default_route_hop_count" => {
                    Ok(Box::new(inner.core.network.rpc.default_route_hop_count))
                }
                "network.dht.max_find_node_count" => {
                    Ok(Box::new(inner.core.network.dht.max_find_node_count))
                }
                "network.dht.resolve_node_timeout_ms" => {
                    Ok(Box::new(inner.core.network.dht.resolve_node_timeout_ms))
                }
                "network.dht.resolve_node_count" => {
                    Ok(Box::new(inner.core.network.dht.resolve_node_count))
                }
                "network.dht.resolve_node_fanout" => {
                    Ok(Box::new(inner.core.network.dht.resolve_node_fanout))
                }
                "network.dht.get_value_timeout_ms" => {
                    Ok(Box::new(inner.core.network.dht.get_value_timeout_ms))
                }
                "network.dht.get_value_count" => {
                    Ok(Box::new(inner.core.network.dht.get_value_count))
                }
                "network.dht.get_value_fanout" => {
                    Ok(Box::new(inner.core.network.dht.get_value_fanout))
                }
                "network.dht.set_value_timeout_ms" => {
                    Ok(Box::new(inner.core.network.dht.set_value_timeout_ms))
                }
                "network.dht.set_value_count" => {
                    Ok(Box::new(inner.core.network.dht.set_value_count))
                }
                "network.dht.set_value_fanout" => {
                    Ok(Box::new(inner.core.network.dht.set_value_fanout))
                }
                "network.dht.min_peer_count" => Ok(Box::new(inner.core.network.dht.min_peer_count)),
                "network.dht.min_peer_refresh_time_ms" => {
                    Ok(Box::new(inner.core.network.dht.min_peer_refresh_time_ms))
                }
                "network.dht.validate_dial_info_receipt_time_ms" => Ok(Box::new(
                    inner.core.network.dht.validate_dial_info_receipt_time_ms,
                )),
                "network.dht.local_subkey_cache_size" => {
                    Ok(Box::new(inner.core.network.dht.local_subkey_cache_size))
                }
                "network.dht.local_max_subkey_cache_memory_mb" => Ok(Box::new(
                    inner.core.network.dht.local_max_subkey_cache_memory_mb,
                )),
                "network.dht.remote_subkey_cache_size" => {
                    Ok(Box::new(inner.core.network.dht.remote_subkey_cache_size))
                }
                "network.dht.remote_max_records" => {
                    Ok(Box::new(inner.core.network.dht.remote_max_records))
                }
                "network.dht.remote_max_subkey_cache_memory_mb" => Ok(Box::new(
                    inner.core.network.dht.remote_max_subkey_cache_memory_mb,
                )),
                "network.dht.remote_max_storage_space_mb" => {
                    Ok(Box::new(inner.core.network.dht.remote_max_storage_space_mb))
                }
                "network.dht.public_watch_limit" => {
                    Ok(Box::new(inner.core.network.dht.public_watch_limit))
                }
                "network.dht.member_watch_limit" => {
                    Ok(Box::new(inner.core.network.dht.member_watch_limit))
                }
                "network.dht.max_watch_expiration_ms" => {
                    Ok(Box::new(inner.core.network.dht.max_watch_expiration_ms))
                }
                "network.upnp" => Ok(Box::new(inner.core.network.upnp)),
                "network.detect_address_changes" => {
                    Ok(Box::new(inner.core.network.detect_address_changes))
                }
                "network.restricted_nat_retries" => {
                    Ok(Box::new(inner.core.network.restricted_nat_retries))
                }
                "network.tls.certificate_path" => {
                    Ok(Box::new(inner.core.network.tls.certificate_path.clone()))
                }
                "network.tls.private_key_path" => {
                    Ok(Box::new(inner.core.network.tls.private_key_path.clone()))
                }
                "network.tls.connection_initial_timeout_ms" => Ok(Box::new(
                    inner.core.network.tls.connection_initial_timeout_ms,
                )),
                "network.application.https.enabled" => {
                    Ok(Box::new(inner.core.network.application.https.enabled))
                }
                "network.application.https.listen_address" => Ok(Box::new(
                    inner
                        .core
                        .network
                        .application
                        .https
                        .listen_address
                        .name
                        .clone(),
                )),
                "network.application.https.path" => Ok(Box::new(
                    inner
                        .core
                        .network
                        .application
                        .https
                        .path
                        .to_string_lossy()
                        .to_string(),
                )),
                "network.application.https.url" => Ok(Box::new(
                    inner
                        .core
                        .network
                        .application
                        .https
                        .url
                        .as_ref()
                        .map(|a| a.urlstring.clone()),
                )),
                "network.application.http.enabled" => {
                    Ok(Box::new(inner.core.network.application.http.enabled))
                }
                "network.application.http.listen_address" => Ok(Box::new(
                    inner
                        .core
                        .network
                        .application
                        .http
                        .listen_address
                        .name
                        .clone(),
                )),
                "network.application.http.path" => Ok(Box::new(
                    inner
                        .core
                        .network
                        .application
                        .http
                        .path
                        .to_string_lossy()
                        .to_string(),
                )),
                "network.application.http.url" => Ok(Box::new(
                    inner
                        .core
                        .network
                        .application
                        .http
                        .url
                        .as_ref()
                        .map(|a| a.urlstring.clone()),
                )),
                "network.protocol.udp.enabled" => {
                    Ok(Box::new(inner.core.network.protocol.udp.enabled))
                }
                "network.protocol.udp.socket_pool_size" => {
                    Ok(Box::new(inner.core.network.protocol.udp.socket_pool_size))
                }
                "network.protocol.udp.listen_address" => Ok(Box::new(
                    inner.core.network.protocol.udp.listen_address.name.clone(),
                )),
                "network.protocol.udp.public_address" => Ok(Box::new(
                    inner
                        .core
                        .network
                        .protocol
                        .udp
                        .public_address
                        .as_ref()
                        .map(|a| a.name.clone()),
                )),
                "network.protocol.tcp.connect" => {
                    Ok(Box::new(inner.core.network.protocol.tcp.connect))
                }
                "network.protocol.tcp.listen" => {
                    Ok(Box::new(inner.core.network.protocol.tcp.listen))
                }
                "network.protocol.tcp.max_connections" => {
                    Ok(Box::new(inner.core.network.protocol.tcp.max_connections))
                }
                "network.protocol.tcp.listen_address" => Ok(Box::new(
                    inner.core.network.protocol.tcp.listen_address.name.clone(),
                )),
                "network.protocol.tcp.public_address" => Ok(Box::new(
                    inner
                        .core
                        .network
                        .protocol
                        .tcp
                        .public_address
                        .as_ref()
                        .map(|a| a.name.clone()),
                )),
                "network.protocol.ws.connect" => {
                    Ok(Box::new(inner.core.network.protocol.ws.connect))
                }
                "network.protocol.ws.listen" => Ok(Box::new(inner.core.network.protocol.ws.listen)),
                "network.protocol.ws.max_connections" => {
                    Ok(Box::new(inner.core.network.protocol.ws.max_connections))
                }
                "network.protocol.ws.listen_address" => Ok(Box::new(
                    inner.core.network.protocol.ws.listen_address.name.clone(),
                )),
                "network.protocol.ws.path" => Ok(Box::new(
                    inner
                        .core
                        .network
                        .protocol
                        .ws
                        .path
                        .to_string_lossy()
                        .to_string(),
                )),
                "network.protocol.ws.url" => Ok(Box::new(
                    inner
                        .core
                        .network
                        .protocol
                        .ws
                        .url
                        .as_ref()
                        .map(|a| a.urlstring.clone()),
                )),
                "network.protocol.wss.connect" => {
                    Ok(Box::new(inner.core.network.protocol.wss.connect))
                }
                "network.protocol.wss.listen" => {
                    Ok(Box::new(inner.core.network.protocol.wss.listen))
                }
                "network.protocol.wss.max_connections" => {
                    Ok(Box::new(inner.core.network.protocol.wss.max_connections))
                }
                "network.protocol.wss.listen_address" => Ok(Box::new(
                    inner.core.network.protocol.wss.listen_address.name.clone(),
                )),
                "network.protocol.wss.path" => Ok(Box::new(
                    inner
                        .core
                        .network
                        .protocol
                        .wss
                        .path
                        .to_string_lossy()
                        .to_string(),
                )),
                "network.protocol.wss.url" => Ok(Box::new(
                    inner
                        .core
                        .network
                        .protocol
                        .wss
                        .url
                        .as_ref()
                        .map(|a| a.urlstring.clone()),
                )),
                _ => Err(VeilidAPIError::generic(format!(
                    "config key '{}' doesn't exist",
                    key
                ))),
            };
            out
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_default_config() {
        let cfg = load_default_config().unwrap();
        let inner = cfg.try_deserialize::<SettingsInner>().unwrap();
        println!("default settings: {:?}", inner);
    }

    #[test]
    #[serial]
    fn test_default_config_settings() {
        let settings = Settings::new(None).unwrap();

        let s = settings.read();
        assert!(!s.daemon.enabled);
        assert_eq!(s.daemon.pid_file, None);
        assert_eq!(s.daemon.chroot, None);
        assert_eq!(s.daemon.working_directory, None);
        assert_eq!(s.daemon.user, None);
        assert_eq!(s.daemon.group, None);
        assert_eq!(s.daemon.stdout_file, None);
        assert_eq!(s.daemon.stderr_file, None);
        assert!(s.client_api.ipc_enabled);
        assert!(!s.client_api.network_enabled);
        assert_eq!(s.client_api.listen_address.name, "localhost:5959");
        assert_eq!(
            s.client_api.listen_address.addrs,
            listen_address_to_socket_addrs("localhost:5959").unwrap()
        );
        assert!(s.auto_attach);
        assert!(!s.logging.system.enabled);
        assert_eq!(s.logging.system.level, LogLevel::Info);
        assert!(s.logging.terminal.enabled);
        assert_eq!(s.logging.terminal.level, LogLevel::Info);
        assert!(!s.logging.file.enabled);
        assert_eq!(s.logging.file.path, "");
        assert!(s.logging.file.append);
        assert_eq!(s.logging.file.level, LogLevel::Info);
        assert!(s.logging.api.enabled);
        assert_eq!(s.logging.api.level, LogLevel::Info);
        assert!(!s.logging.otlp.enabled);
        assert_eq!(s.logging.otlp.level, LogLevel::Trace);
        assert_eq!(
            s.logging.otlp.grpc_endpoint,
            NamedSocketAddrs::from_str("localhost:4317").unwrap()
        );
        assert!(!s.logging.console.enabled);
        assert_eq!(s.testing.subnode_index, 0);

        assert_eq!(
            s.core.table_store.directory,
            VeilidConfigTableStore::default().directory,
        );
        assert!(!s.core.table_store.delete);

        assert_eq!(
            s.core.block_store.directory,
            VeilidConfigBlockStore::default().directory,
        );
        assert!(!s.core.block_store.delete);

        assert!(s.core.protected_store.allow_insecure_fallback);
        assert!(s.core.protected_store.always_use_insecure_storage);
        assert_eq!(
            s.core.protected_store.directory,
            VeilidConfigProtectedStore::default().directory
        );
        assert!(!s.core.protected_store.delete);
        assert_eq!(s.core.protected_store.device_encryption_key_password, "");
        assert_eq!(
            s.core.protected_store.new_device_encryption_key_password,
            None
        );

        assert_eq!(s.core.network.connection_initial_timeout_ms, 2_000u32);
        assert_eq!(s.core.network.connection_inactivity_timeout_ms, 60_000u32);
        assert_eq!(s.core.network.max_connections_per_ip4, 32u32);
        assert_eq!(s.core.network.max_connections_per_ip6_prefix, 32u32);
        assert_eq!(s.core.network.max_connections_per_ip6_prefix_size, 56u32);
        assert_eq!(s.core.network.max_connection_frequency_per_min, 128u32);
        assert_eq!(s.core.network.client_allowlist_timeout_ms, 300_000u32);
        assert_eq!(s.core.network.reverse_connection_receipt_time_ms, 5_000u32);
        assert_eq!(s.core.network.hole_punch_receipt_time_ms, 5_000u32);
        assert_eq!(s.core.network.network_key_password, None);
        assert_eq!(s.core.network.routing_table.node_id, None);
        assert_eq!(s.core.network.routing_table.node_id_secret, None);
        //
        assert_eq!(
            s.core.network.routing_table.bootstrap,
            vec!["bootstrap.veilid.net".to_owned()]
        );
        //
        assert_eq!(s.core.network.rpc.concurrency, 0);
        assert_eq!(s.core.network.rpc.queue_size, 1024);
        assert_eq!(s.core.network.rpc.max_timestamp_behind_ms, Some(10_000u32));
        assert_eq!(s.core.network.rpc.max_timestamp_ahead_ms, Some(10_000u32));
        assert_eq!(s.core.network.rpc.timeout_ms, 5_000u32);
        assert_eq!(s.core.network.rpc.max_route_hop_count, 4);
        assert_eq!(s.core.network.rpc.default_route_hop_count, 1);
        //
        assert_eq!(s.core.network.dht.max_find_node_count, 20u32);
        assert_eq!(s.core.network.dht.resolve_node_timeout_ms, 10_000u32);
        assert_eq!(s.core.network.dht.resolve_node_count, 1u32);
        assert_eq!(s.core.network.dht.resolve_node_fanout, 4u32);
        assert_eq!(s.core.network.dht.get_value_timeout_ms, 10_000u32);
        assert_eq!(s.core.network.dht.get_value_count, 3u32);
        assert_eq!(s.core.network.dht.get_value_fanout, 4u32);
        assert_eq!(s.core.network.dht.set_value_timeout_ms, 10_000u32);
        assert_eq!(s.core.network.dht.set_value_count, 5u32);
        assert_eq!(s.core.network.dht.set_value_fanout, 4u32);
        assert_eq!(s.core.network.dht.min_peer_count, 20u32);
        assert_eq!(s.core.network.dht.min_peer_refresh_time_ms, 60_000u32);
        assert_eq!(
            s.core.network.dht.validate_dial_info_receipt_time_ms,
            2_000u32
        );
        assert_eq!(s.core.network.dht.public_watch_limit, 32u32);
        assert_eq!(s.core.network.dht.member_watch_limit, 8u32);
        assert_eq!(s.core.network.dht.max_watch_expiration_ms, 600_000u32);
        //
        assert!(s.core.network.upnp);
        assert!(s.core.network.detect_address_changes);
        assert_eq!(s.core.network.restricted_nat_retries, 0u32);
        //
        assert_eq!(
            s.core.network.tls.certificate_path,
            VeilidConfigTLS::default().certificate_path
        );
        assert_eq!(
            s.core.network.tls.private_key_path,
            VeilidConfigTLS::default().private_key_path
        );
        assert_eq!(s.core.network.tls.connection_initial_timeout_ms, 2_000u32);
        //
        assert!(!s.core.network.application.https.enabled);
        assert_eq!(s.core.network.application.https.listen_address.name, ":443");
        assert_eq!(
            s.core.network.application.https.listen_address.addrs,
            listen_address_to_socket_addrs(":443").unwrap()
        );
        assert_eq!(
            s.core.network.application.https.path,
            std::path::PathBuf::from("app")
        );
        assert_eq!(s.core.network.application.https.url, None);
        assert!(!s.core.network.application.http.enabled);
        assert_eq!(s.core.network.application.http.listen_address.name, ":80");
        assert_eq!(
            s.core.network.application.http.listen_address.addrs,
            listen_address_to_socket_addrs(":80").unwrap()
        );
        assert_eq!(
            s.core.network.application.http.path,
            std::path::PathBuf::from("app")
        );
        assert_eq!(s.core.network.application.http.url, None);
        //
        assert!(s.core.network.protocol.udp.enabled);
        assert_eq!(s.core.network.protocol.udp.socket_pool_size, 0);
        assert_eq!(s.core.network.protocol.udp.listen_address.name, "");
        assert_eq!(s.core.network.protocol.udp.listen_address.addrs, vec![]);
        assert_eq!(s.core.network.protocol.udp.public_address, None);

        //
        assert!(s.core.network.protocol.tcp.connect);
        assert!(s.core.network.protocol.tcp.listen);
        assert_eq!(s.core.network.protocol.tcp.max_connections, 32);
        assert_eq!(s.core.network.protocol.tcp.listen_address.name, "");
        assert_eq!(s.core.network.protocol.tcp.listen_address.addrs, vec![]);
        assert_eq!(s.core.network.protocol.tcp.public_address, None);

        //
        assert!(s.core.network.protocol.ws.connect);
        assert!(s.core.network.protocol.ws.listen);
        assert_eq!(s.core.network.protocol.ws.max_connections, 32);
        assert_eq!(s.core.network.protocol.ws.listen_address.name, "");
        assert_eq!(s.core.network.protocol.ws.listen_address.addrs, vec![]);
        assert_eq!(
            s.core.network.protocol.ws.path,
            std::path::PathBuf::from("ws")
        );
        assert_eq!(s.core.network.protocol.ws.url, None);
        //
        assert!(s.core.network.protocol.wss.connect);
        assert!(!s.core.network.protocol.wss.listen);
        assert_eq!(s.core.network.protocol.wss.max_connections, 32);
        assert_eq!(s.core.network.protocol.wss.listen_address.name, "");
        assert_eq!(s.core.network.protocol.wss.listen_address.addrs, vec![]);
        assert_eq!(
            s.core.network.protocol.wss.path,
            std::path::PathBuf::from("ws")
        );
        assert_eq!(s.core.network.protocol.wss.url, None);
        //
    }
}
