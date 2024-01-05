use crate::*;

////////////////////////////////////////////////////////////////////////////////////////////////
pub type ConfigCallbackReturn = VeilidAPIResult<Box<dyn core::any::Any + Send>>;
pub type ConfigCallback = Arc<dyn Fn(String) -> ConfigCallbackReturn + Send + Sync>;

/// Enable and configure HTTPS access to the Veilid node
///
/// ```yaml
/// https:
///     enabled: false
///     listen_address: ':5150'
///     path: 'app'
///     url: 'https://localhost:5150'
/// ```
///
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
pub struct VeilidConfigHTTPS {
    pub enabled: bool,
    pub listen_address: String,
    pub path: String,
    #[cfg_attr(target_arch = "wasm32", tsify(optional))]
    pub url: Option<String>, // Fixed URL is not optional for TLS-based protocols and is dynamically validated
}

/// Enable and configure HTTP access to the Veilid node
///
/// ```yaml
/// http:
///     enabled: false
///     listen_address: ':5150'
///     path: 'app"
///     url: 'https://localhost:5150'
/// ```
///
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
pub struct VeilidConfigHTTP {
    pub enabled: bool,
    pub listen_address: String,
    pub path: String,
    #[cfg_attr(target_arch = "wasm32", tsify(optional))]
    pub url: Option<String>,
}

/// Application configuration
///
/// Configure web access to the Progressive Web App (PWA)
///
/// To be implemented...
///
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
pub struct VeilidConfigApplication {
    pub https: VeilidConfigHTTPS,
    pub http: VeilidConfigHTTP,
}

/// Enable and configure UDP
///
/// ```yaml
/// udp:
///     enabled: true
///     socket_pool_size: 0
///     listen_address: ':5150'
///     public_address: ''
/// ```
///
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
pub struct VeilidConfigUDP {
    pub enabled: bool,
    pub socket_pool_size: u32,
    pub listen_address: String,
    #[cfg_attr(target_arch = "wasm32", tsify(optional))]
    pub public_address: Option<String>,
}

/// Enable and configure TCP
///
/// ```yaml
/// tcp:
///     connect: true
///     listen: true
///     max_connections: 32
///     listen_address: ':5150'
///     public_address: ''
///
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
pub struct VeilidConfigTCP {
    pub connect: bool,
    pub listen: bool,
    pub max_connections: u32,
    pub listen_address: String,
    #[cfg_attr(target_arch = "wasm32", tsify(optional))]
    pub public_address: Option<String>,
}

/// Enable and configure Web Sockets
///
/// ```yaml
/// ws:
///     connect: true
///     listen: true
///     max_connections: 16
///     listen_address: ':5150'
///     path: 'ws'
///     url: 'ws://localhost:5150/ws'
///
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]

pub struct VeilidConfigWS {
    pub connect: bool,
    pub listen: bool,
    pub max_connections: u32,
    pub listen_address: String,
    pub path: String,
    #[cfg_attr(target_arch = "wasm32", tsify(optional))]
    pub url: Option<String>,
}

/// Enable and configure Secure Web Sockets
///
/// ```yaml
/// wss:
///     connect: true
///     listen: false
///     max_connections: 16
///     listen_address: ':5150'
///     path: 'ws'
///     url: ''
///
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]

pub struct VeilidConfigWSS {
    pub connect: bool,
    pub listen: bool,
    pub max_connections: u32,
    pub listen_address: String,
    pub path: String,
    #[cfg_attr(target_arch = "wasm32", tsify(optional))]
    pub url: Option<String>, // Fixed URL is not optional for TLS-based protocols and is dynamically validated
}

/// Configure Network Protocols
///
/// Veilid can communicate over UDP, TCP, and Web Sockets.
///
/// All protocols are available by default, and the Veilid node will
/// sort out which protocol is used for each peer connection.
///
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]

pub struct VeilidConfigProtocol {
    pub udp: VeilidConfigUDP,
    pub tcp: VeilidConfigTCP,
    pub ws: VeilidConfigWS,
    pub wss: VeilidConfigWSS,
}

/// Configure TLS
///
/// ```yaml
/// tls:
///     certificate_path: /path/to/cert
///     private_key_path: /path/to/private/key
///     connection_initial_timeout_ms: 2000
///
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
pub struct VeilidConfigTLS {
    pub certificate_path: String,
    pub private_key_path: String,
    pub connection_initial_timeout_ms: u32,
}

/// Configure the Distributed Hash Table (DHT)
///
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
pub struct VeilidConfigDHT {
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
}

/// Configure RPC
///
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
pub struct VeilidConfigRPC {
    pub concurrency: u32,
    pub queue_size: u32,
    #[cfg_attr(target_arch = "wasm32", tsify(optional))]
    pub max_timestamp_behind_ms: Option<u32>,
    #[cfg_attr(target_arch = "wasm32", tsify(optional))]
    pub max_timestamp_ahead_ms: Option<u32>,
    pub timeout_ms: u32,
    pub max_route_hop_count: u8,
    pub default_route_hop_count: u8,
}

impl Default for VeilidConfigRPC {
    fn default() -> Self {
        Self {
            concurrency: 0,
            queue_size: 1024,
            max_timestamp_behind_ms: None,
            max_timestamp_ahead_ms: None,
            timeout_ms: 5000,
            max_route_hop_count: 4,
            default_route_hop_count: 1,
        }
    }
}

/// Configure the network routing table
///
#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
pub struct VeilidConfigRoutingTable {
    #[schemars(with = "Vec<String>")]
    pub node_id: TypedKeyGroup,
    #[schemars(with = "Vec<String>")]
    pub node_id_secret: TypedSecretGroup,
    pub bootstrap: Vec<String>,
    pub limit_over_attached: u32,
    pub limit_fully_attached: u32,
    pub limit_attached_strong: u32,
    pub limit_attached_good: u32,
    pub limit_attached_weak: u32,
    // xxx pub enable_public_internet: bool,
    // xxx pub enable_local_network: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
pub struct VeilidConfigNetwork {
    pub connection_initial_timeout_ms: u32,
    pub connection_inactivity_timeout_ms: u32,
    pub max_connections_per_ip4: u32,
    pub max_connections_per_ip6_prefix: u32,
    pub max_connections_per_ip6_prefix_size: u32,
    pub max_connection_frequency_per_min: u32,
    pub client_allowlist_timeout_ms: u32,
    pub reverse_connection_receipt_time_ms: u32,
    pub hole_punch_receipt_time_ms: u32,
    #[cfg_attr(target_arch = "wasm32", tsify(optional))]
    pub network_key_password: Option<String>,
    pub routing_table: VeilidConfigRoutingTable,
    pub rpc: VeilidConfigRPC,
    pub dht: VeilidConfigDHT,
    pub upnp: bool,
    pub detect_address_changes: bool,
    pub restricted_nat_retries: u32,
    pub tls: VeilidConfigTLS,
    pub application: VeilidConfigApplication,
    pub protocol: VeilidConfigProtocol,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
pub struct VeilidConfigTableStore {
    pub directory: String,
    pub delete: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
pub struct VeilidConfigBlockStore {
    pub directory: String,
    pub delete: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
pub struct VeilidConfigProtectedStore {
    pub allow_insecure_fallback: bool,
    pub always_use_insecure_storage: bool,
    pub directory: String,
    pub delete: bool,
    pub device_encryption_key_password: String,
    #[cfg_attr(target_arch = "wasm32", tsify(optional))]
    pub new_device_encryption_key_password: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
pub struct VeilidConfigCapabilities {
    pub disable: Vec<FourCC>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
#[cfg_attr(target_arch = "wasm32", tsify(namespace, from_wasm_abi))]
pub enum VeilidConfigLogLevel {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl VeilidConfigLogLevel {
    pub fn to_veilid_log_level(&self) -> Option<VeilidLogLevel> {
        match self {
            Self::Off => None,
            Self::Error => Some(VeilidLogLevel::Error),
            Self::Warn => Some(VeilidLogLevel::Warn),
            Self::Info => Some(VeilidLogLevel::Info),
            Self::Debug => Some(VeilidLogLevel::Debug),
            Self::Trace => Some(VeilidLogLevel::Trace),
        }
    }
    pub fn to_tracing_level_filter(&self) -> level_filters::LevelFilter {
        match self {
            Self::Off => level_filters::LevelFilter::OFF,
            Self::Error => level_filters::LevelFilter::ERROR,
            Self::Warn => level_filters::LevelFilter::WARN,
            Self::Info => level_filters::LevelFilter::INFO,
            Self::Debug => level_filters::LevelFilter::DEBUG,
            Self::Trace => level_filters::LevelFilter::TRACE,
        }
    }
    pub fn from_veilid_log_level(level: Option<VeilidLogLevel>) -> Self {
        match level {
            None => Self::Off,
            Some(VeilidLogLevel::Error) => Self::Error,
            Some(VeilidLogLevel::Warn) => Self::Warn,
            Some(VeilidLogLevel::Info) => Self::Info,
            Some(VeilidLogLevel::Debug) => Self::Debug,
            Some(VeilidLogLevel::Trace) => Self::Trace,
        }
    }
    pub fn from_tracing_level_filter(level: level_filters::LevelFilter) -> Self {
        match level {
            level_filters::LevelFilter::OFF => Self::Off,
            level_filters::LevelFilter::ERROR => Self::Error,
            level_filters::LevelFilter::WARN => Self::Warn,
            level_filters::LevelFilter::INFO => Self::Info,
            level_filters::LevelFilter::DEBUG => Self::Debug,
            level_filters::LevelFilter::TRACE => Self::Trace,
        }
    }
}
impl Default for VeilidConfigLogLevel {
    fn default() -> Self {
        Self::Off
    }
}
impl FromStr for VeilidConfigLogLevel {
    type Err = VeilidAPIError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Off" => Self::Off,
            "Error" => Self::Error,
            "Warn" => Self::Warn,
            "Info" => Self::Info,
            "Debug" => Self::Debug,
            "Trace" => Self::Trace,
            _ => {
                apibail_invalid_argument!("Can't convert str", "s", s);
            }
        })
    }
}
impl fmt::Display for VeilidConfigLogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let text = match self {
            Self::Off => "Off",
            Self::Error => "Error",
            Self::Warn => "Warn",
            Self::Info => "Info",
            Self::Debug => "Debug",
            Self::Trace => "Trace",
        };
        write!(f, "{}", text)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify))]
pub struct VeilidConfigInner {
    pub program_name: String,
    pub namespace: String,
    pub capabilities: VeilidConfigCapabilities,
    pub protected_store: VeilidConfigProtectedStore,
    pub table_store: VeilidConfigTableStore,
    pub block_store: VeilidConfigBlockStore,
    pub network: VeilidConfigNetwork,
}

/// The Veilid Configuration
///
/// Veilid is configured
#[derive(Clone)]
pub struct VeilidConfig {
    update_cb: Option<UpdateCallback>,
    inner: Arc<RwLock<VeilidConfigInner>>,
}

impl fmt::Debug for VeilidConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inner = self.inner.read();
        f.debug_struct("VeilidConfig")
            .field("inner", &*inner)
            .finish()
    }
}

impl Default for VeilidConfig {
    fn default() -> Self {
        Self::new()
    }
}
impl VeilidConfig {
    fn new_inner() -> VeilidConfigInner {
        VeilidConfigInner::default()
    }

    pub fn new() -> Self {
        Self {
            update_cb: None,
            inner: Arc::new(RwLock::new(Self::new_inner())),
        }
    }

    pub fn setup_from_json(
        &mut self,
        config: String,
        update_cb: UpdateCallback,
    ) -> VeilidAPIResult<()> {
        self.update_cb = Some(update_cb);

        self.with_mut(|inner| {
            *inner = serde_json::from_str(&config).map_err(VeilidAPIError::generic)?;
            Ok(())
        })
    }

    pub fn setup_from_config(
        &mut self,
        config: VeilidConfigInner,
        update_cb: UpdateCallback,
    ) -> VeilidAPIResult<()> {
        self.update_cb = Some(update_cb);

        self.with_mut(|inner| {
            *inner = config;
            Ok(())
        })
    }

    pub fn setup(&mut self, cb: ConfigCallback, update_cb: UpdateCallback) -> VeilidAPIResult<()> {
        self.update_cb = Some(update_cb);
        self.with_mut(|inner| {
            // Simple config transformation
            macro_rules! get_config {
                ($key:expr) => {
                    let keyname = &stringify!($key)[6..];
                    let v = cb(keyname.to_owned())?;
                    $key = match v.downcast() {
                        Ok(v) => *v,
                        Err(e) => {
                            apibail_generic!(format!(
                                "incorrect type for key {}: {:?}",
                                keyname,
                                type_name_of_val(&*e)
                            ))
                        }
                    };
                };
            }

            get_config!(inner.program_name);
            get_config!(inner.namespace);
            get_config!(inner.capabilities.disable);
            get_config!(inner.table_store.directory);
            get_config!(inner.table_store.delete);
            get_config!(inner.block_store.directory);
            get_config!(inner.block_store.delete);
            get_config!(inner.protected_store.allow_insecure_fallback);
            get_config!(inner.protected_store.always_use_insecure_storage);
            get_config!(inner.protected_store.directory);
            get_config!(inner.protected_store.delete);
            get_config!(inner.protected_store.device_encryption_key_password);
            get_config!(inner.protected_store.new_device_encryption_key_password);
            get_config!(inner.network.connection_initial_timeout_ms);
            get_config!(inner.network.connection_inactivity_timeout_ms);
            get_config!(inner.network.max_connections_per_ip4);
            get_config!(inner.network.max_connections_per_ip6_prefix);
            get_config!(inner.network.max_connections_per_ip6_prefix_size);
            get_config!(inner.network.max_connection_frequency_per_min);
            get_config!(inner.network.client_allowlist_timeout_ms);
            get_config!(inner.network.reverse_connection_receipt_time_ms);
            get_config!(inner.network.hole_punch_receipt_time_ms);
            get_config!(inner.network.network_key_password);
            get_config!(inner.network.routing_table.node_id);
            get_config!(inner.network.routing_table.node_id_secret);
            get_config!(inner.network.routing_table.bootstrap);
            get_config!(inner.network.routing_table.limit_over_attached);
            get_config!(inner.network.routing_table.limit_fully_attached);
            get_config!(inner.network.routing_table.limit_attached_strong);
            get_config!(inner.network.routing_table.limit_attached_good);
            get_config!(inner.network.routing_table.limit_attached_weak);
            get_config!(inner.network.dht.max_find_node_count);
            get_config!(inner.network.dht.resolve_node_timeout_ms);
            get_config!(inner.network.dht.resolve_node_count);
            get_config!(inner.network.dht.resolve_node_fanout);
            get_config!(inner.network.dht.get_value_timeout_ms);
            get_config!(inner.network.dht.get_value_count);
            get_config!(inner.network.dht.get_value_fanout);
            get_config!(inner.network.dht.set_value_timeout_ms);
            get_config!(inner.network.dht.set_value_count);
            get_config!(inner.network.dht.set_value_fanout);
            get_config!(inner.network.dht.min_peer_count);
            get_config!(inner.network.dht.min_peer_refresh_time_ms);
            get_config!(inner.network.dht.validate_dial_info_receipt_time_ms);
            get_config!(inner.network.dht.local_subkey_cache_size);
            get_config!(inner.network.dht.local_max_subkey_cache_memory_mb);
            get_config!(inner.network.dht.remote_subkey_cache_size);
            get_config!(inner.network.dht.remote_max_records);
            get_config!(inner.network.dht.remote_max_subkey_cache_memory_mb);
            get_config!(inner.network.dht.remote_max_storage_space_mb);
            get_config!(inner.network.rpc.concurrency);
            get_config!(inner.network.rpc.queue_size);
            get_config!(inner.network.rpc.max_timestamp_behind_ms);
            get_config!(inner.network.rpc.max_timestamp_ahead_ms);
            get_config!(inner.network.rpc.timeout_ms);
            get_config!(inner.network.rpc.max_route_hop_count);
            get_config!(inner.network.rpc.default_route_hop_count);
            get_config!(inner.network.upnp);
            get_config!(inner.network.detect_address_changes);
            get_config!(inner.network.restricted_nat_retries);
            get_config!(inner.network.tls.certificate_path);
            get_config!(inner.network.tls.private_key_path);
            get_config!(inner.network.tls.connection_initial_timeout_ms);
            get_config!(inner.network.application.https.enabled);
            get_config!(inner.network.application.https.listen_address);
            get_config!(inner.network.application.https.path);
            get_config!(inner.network.application.https.url);
            get_config!(inner.network.application.http.enabled);
            get_config!(inner.network.application.http.listen_address);
            get_config!(inner.network.application.http.path);
            get_config!(inner.network.application.http.url);
            get_config!(inner.network.protocol.udp.enabled);
            get_config!(inner.network.protocol.udp.socket_pool_size);
            get_config!(inner.network.protocol.udp.listen_address);
            get_config!(inner.network.protocol.udp.public_address);
            get_config!(inner.network.protocol.tcp.connect);
            get_config!(inner.network.protocol.tcp.listen);
            get_config!(inner.network.protocol.tcp.max_connections);
            get_config!(inner.network.protocol.tcp.listen_address);
            get_config!(inner.network.protocol.tcp.public_address);
            get_config!(inner.network.protocol.ws.connect);
            get_config!(inner.network.protocol.ws.listen);
            get_config!(inner.network.protocol.ws.max_connections);
            get_config!(inner.network.protocol.ws.listen_address);
            get_config!(inner.network.protocol.ws.path);
            get_config!(inner.network.protocol.ws.url);
            get_config!(inner.network.protocol.wss.connect);
            get_config!(inner.network.protocol.wss.listen);
            get_config!(inner.network.protocol.wss.max_connections);
            get_config!(inner.network.protocol.wss.listen_address);
            get_config!(inner.network.protocol.wss.path);
            get_config!(inner.network.protocol.wss.url);
            Ok(())
        })
    }

    pub fn get_veilid_state(&self) -> Box<VeilidStateConfig> {
        let inner = self.inner.read();
        Box::new(VeilidStateConfig {
            config: inner.clone(),
        })
    }

    pub fn get(&self) -> RwLockReadGuard<VeilidConfigInner> {
        self.inner.read()
    }

    fn safe_config_inner(&self) -> VeilidConfigInner {
        let mut safe_cfg = self.inner.read().clone();

        // Remove secrets
        safe_cfg.network.routing_table.node_id_secret = TypedSecretGroup::new();
        safe_cfg.protected_store.device_encryption_key_password = "".to_owned();
        safe_cfg.protected_store.new_device_encryption_key_password = None;

        safe_cfg
    }

    pub fn safe_config(&self) -> VeilidConfig {
        let mut safe_cfg = self.inner.read().clone();

        // Remove secrets
        safe_cfg.network.routing_table.node_id_secret = TypedSecretGroup::new();
        safe_cfg.protected_store.device_encryption_key_password = "".to_owned();
        safe_cfg.protected_store.new_device_encryption_key_password = None;

        VeilidConfig {
            update_cb: self.update_cb.clone(),
            inner: Arc::new(RwLock::new(safe_cfg)),
        }
    }

    pub fn with_mut<F, R>(&self, f: F) -> VeilidAPIResult<R>
    where
        F: FnOnce(&mut VeilidConfigInner) -> VeilidAPIResult<R>,
    {
        let out = {
            let inner = &mut *self.inner.write();
            // Edit a copy
            let mut editedinner = inner.clone();
            // Make changes
            let out = f(&mut editedinner)?;
            // Validate
            Self::validate(&editedinner)?;
            // See if things have changed
            if *inner == editedinner {
                // No changes, return early
                return Ok(out);
            }
            // Commit changes
            *inner = editedinner.clone();
            out
        };

        // Send configuration update to clients
        if let Some(update_cb) = &self.update_cb {
            let safe_cfg = self.safe_config_inner();
            update_cb(VeilidUpdate::Config(Box::new(VeilidStateConfig {
                config: safe_cfg,
            })));
        }

        Ok(out)
    }

    pub fn get_key_json(&self, key: &str, pretty: bool) -> VeilidAPIResult<String> {
        let c = self.get();

        // Generate json from whole config
        let jc = serde_json::to_string(&*c).map_err(VeilidAPIError::generic)?;
        let jvc = json::parse(&jc).map_err(VeilidAPIError::generic)?;

        // Find requested subkey
        if key.is_empty() {
            Ok(if pretty {
                jvc.pretty(2)
            } else {
                jvc.to_string()
            })
        } else {
            // Split key into path parts
            let keypath: Vec<&str> = key.split('.').collect();
            let mut out = &jvc;
            for k in keypath {
                if !out.has_key(k) {
                    apibail_parse_error!(format!("invalid subkey in key '{}'", key), k);
                }
                out = &out[k];
            }
            Ok(if pretty {
                out.pretty(2)
            } else {
                out.to_string()
            })
        }
    }
    pub fn set_key_json(&self, key: &str, value: &str) -> VeilidAPIResult<()> {
        self.with_mut(|c| {
            // Split key into path parts
            let keypath: Vec<&str> = key.split('.').collect();

            // Convert value into jsonvalue
            let newval = json::parse(value).map_err(VeilidAPIError::generic)?;

            // Generate json from whole config
            let jc = serde_json::to_string(&*c).map_err(VeilidAPIError::generic)?;
            let mut jvc = json::parse(&jc).map_err(VeilidAPIError::generic)?;

            // Find requested subkey
            let newconfigstring = if let Some((objkeyname, objkeypath)) = keypath.split_last() {
                // Replace subkey
                let mut out = &mut jvc;
                for k in objkeypath {
                    if !out.has_key(k) {
                        apibail_parse_error!(format!("invalid subkey in key '{}'", key), k);
                    }
                    out = &mut out[*k];
                }
                if !out.has_key(objkeyname) {
                    apibail_parse_error!(format!("invalid subkey in key '{}'", key), objkeyname);
                }
                out[*objkeyname] = newval;
                jvc.to_string()
            } else {
                newval.to_string()
            };

            // Generate new config
            *c = serde_json::from_str(&newconfigstring).map_err(VeilidAPIError::generic)?;
            Ok(())
        })
    }

    fn validate(inner: &VeilidConfigInner) -> VeilidAPIResult<()> {
        if inner.program_name.is_empty() {
            apibail_generic!("Program name must not be empty in 'program_name'");
        }

        // if inner.network.protocol.udp.enabled {
        //     // Validate UDP settings
        // }
        if inner.network.protocol.tcp.listen {
            // Validate TCP settings
            if inner.network.protocol.tcp.max_connections == 0 {
                apibail_generic!("TCP max connections must be > 0 in config key 'network.protocol.tcp.max_connections'");
            }
        }
        if inner.network.protocol.ws.listen {
            // Validate WS settings
            if inner.network.protocol.ws.max_connections == 0 {
                apibail_generic!("WS max connections must be > 0 in config key 'network.protocol.ws.max_connections'");
            }
            if inner.network.application.https.enabled
                && inner.network.application.https.path == inner.network.protocol.ws.path
            {
                apibail_generic!("WS path conflicts with HTTPS application path in config key 'network.protocol.ws.path'");
            }
            if inner.network.application.http.enabled
                && inner.network.application.http.path == inner.network.protocol.ws.path
            {
                apibail_generic!("WS path conflicts with HTTP application path in config key 'network.protocol.ws.path'");
            }
        }
        if inner.network.protocol.wss.listen {
            // Validate WSS settings
            if inner.network.protocol.wss.max_connections == 0 {
                apibail_generic!("WSS max connections must be > 0 in config key 'network.protocol.wss.max_connections'");
            }
            if inner
                .network
                .protocol
                .wss
                .url
                .as_ref()
                .map(|u| u.is_empty())
                .unwrap_or_default()
            {
                apibail_generic!(
                    "WSS URL must be specified in config key 'network.protocol.wss.url'"
                );
            }
            if inner.network.application.https.enabled
                && inner.network.application.https.path == inner.network.protocol.wss.path
            {
                apibail_generic!("WSS path conflicts with HTTPS application path in config key 'network.protocol.ws.path'");
            }
            if inner.network.application.http.enabled
                && inner.network.application.http.path == inner.network.protocol.wss.path
            {
                apibail_generic!("WSS path conflicts with HTTP application path in config key 'network.protocol.ws.path'");
            }
        }
        if inner.network.application.https.enabled {
            // Validate HTTPS settings
            if inner
                .network
                .application
                .https
                .url
                .as_ref()
                .map(|u| u.is_empty())
                .unwrap_or_default()
            {
                apibail_generic!(
                    "HTTPS URL must be specified in config key 'network.application.https.url'"
                );
            }
        }
        if inner.network.rpc.max_route_hop_count == 0 {
            apibail_generic!(
                "max route hop count must be >= 1 in 'network.rpc.max_route_hop_count'"
            );
        }
        if inner.network.rpc.max_route_hop_count > 5 {
            apibail_generic!(
                "max route hop count must be <= 5 in 'network.rpc.max_route_hop_count'"
            );
        }
        if inner.network.rpc.default_route_hop_count == 0 {
            apibail_generic!(
                "default route hop count must be >= 1 in 'network.rpc.default_route_hop_count'"
            );
        }
        if inner.network.rpc.default_route_hop_count > inner.network.rpc.max_route_hop_count {
            apibail_generic!(
                "default route hop count must be <= max route hop count in 'network.rpc.default_route_hop_count <= network.rpc.max_route_hop_count'"
            );
        }
        if inner.network.rpc.queue_size < 256 {
            apibail_generic!("rpc queue size must be >= 256 in 'network.rpc.queue_size'");
        }
        if inner.network.rpc.timeout_ms < 1000 {
            apibail_generic!("rpc timeout must be >= 1000 in 'network.rpc.timeout_ms'");
        }

        Ok(())
    }

    #[cfg(not(test))]
    async fn init_node_id(
        &self,
        vcrypto: CryptoSystemVersion,
        table_store: TableStore,
    ) -> VeilidAPIResult<(TypedKey, TypedSecret)> {
        let ck = vcrypto.kind();
        let mut node_id = self.inner.read().network.routing_table.node_id.get(ck);
        let mut node_id_secret = self
            .inner
            .read()
            .network
            .routing_table
            .node_id_secret
            .get(ck);

        // See if node id was previously stored in the table store
        let config_table = table_store.open("__veilid_config", 1).await?;

        let table_key_node_id = format!("node_id_{}", ck);
        let table_key_node_id_secret = format!("node_id_secret_{}", ck);

        if node_id.is_none() {
            debug!("pulling {} from storage", table_key_node_id);
            if let Ok(Some(stored_node_id)) = config_table
                .load_json::<TypedKey>(0, table_key_node_id.as_bytes())
                .await
            {
                debug!("{} found in storage", table_key_node_id);
                node_id = Some(stored_node_id);
            } else {
                debug!("{} not found in storage", table_key_node_id);
            }
        }

        // See if node id secret was previously stored in the protected store
        if node_id_secret.is_none() {
            debug!("pulling {} from storage", table_key_node_id_secret);
            if let Ok(Some(stored_node_id_secret)) = config_table
                .load_json::<TypedSecret>(0, table_key_node_id_secret.as_bytes())
                .await
            {
                debug!("{} found in storage", table_key_node_id_secret);
                node_id_secret = Some(stored_node_id_secret);
            } else {
                debug!("{} not found in storage", table_key_node_id_secret);
            }
        }

        // If we have a node id from storage, check it
        let (node_id, node_id_secret) =
            if let (Some(node_id), Some(node_id_secret)) = (node_id, node_id_secret) {
                // Validate node id
                if !vcrypto.validate_keypair(&node_id.value, &node_id_secret.value) {
                    apibail_generic!(format!(
                        "node_id_secret_{} and node_id_key_{} don't match",
                        ck, ck
                    ));
                }
                (node_id, node_id_secret)
            } else {
                // If we still don't have a valid node id, generate one
                debug!("generating new node_id_{}", ck);
                let kp = vcrypto.generate_keypair();
                (TypedKey::new(ck, kp.key), TypedSecret::new(ck, kp.secret))
            };
        info!("Node Id: {}", node_id);

        // Save the node id / secret in storage
        config_table
            .store_json(0, table_key_node_id.as_bytes(), &node_id)
            .await?;
        config_table
            .store_json(0, table_key_node_id_secret.as_bytes(), &node_id_secret)
            .await?;

        Ok((node_id, node_id_secret))
    }

    /// Get the node id from config if one is specified
    /// Must be done -after- protected store startup
    #[cfg_attr(test, allow(unused_variables))]
    pub async fn init_node_ids(
        &self,
        crypto: Crypto,
        table_store: TableStore,
    ) -> VeilidAPIResult<()> {
        let mut out_node_id = TypedKeyGroup::new();
        let mut out_node_id_secret = TypedSecretGroup::new();

        for ck in VALID_CRYPTO_KINDS {
            let vcrypto = crypto
                .get(ck)
                .expect("Valid crypto kind is not actually valid.");

            #[cfg(test)]
            let (node_id, node_id_secret) = {
                let kp = vcrypto.generate_keypair();
                (TypedKey::new(ck, kp.key), TypedSecret::new(ck, kp.secret))
            };
            #[cfg(not(test))]
            let (node_id, node_id_secret) = self.init_node_id(vcrypto, table_store.clone()).await?;

            // Save for config
            out_node_id.add(node_id);
            out_node_id_secret.add(node_id_secret);
        }

        // Commit back to config
        self.with_mut(|c| {
            c.network.routing_table.node_id = out_node_id;
            c.network.routing_table.node_id_secret = out_node_id_secret;
            Ok(())
        })?;

        trace!("init_node_ids complete");

        Ok(())
    }
}
