use crate::attachment_manager::{AttachmentManager, AttachmentManagerStartupContext};
use crate::crypto::Crypto;
use crate::logging::*;
use crate::network_manager::{NetworkManager, NetworkManagerStartupContext};
use crate::routing_table::RoutingTable;
use crate::rpc_processor::{RPCProcessor, RPCProcessorStartupContext};
use crate::storage_manager::StorageManager;
use crate::veilid_api::*;
use crate::veilid_config::*;
use crate::*;

impl_veilid_log_facility!("corectx");

pub type UpdateCallback = Arc<dyn Fn(VeilidUpdate) + Send + Sync>;

type InitKey = (String, String);

/////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug)]
pub(crate) struct VeilidCoreContext {
    registry: VeilidComponentRegistry,
}

impl_veilid_component_registry_accessor!(VeilidCoreContext);

impl VeilidCoreContext {
    #[instrument(level = "trace", target = "core_context", err, skip_all)]
    async fn new_with_config_callback(
        update_callback: UpdateCallback,
        config_callback: ConfigCallback,
    ) -> VeilidAPIResult<VeilidCoreContext> {
        // Set up config from callback
        let config = VeilidConfig::new_from_callback(config_callback, update_callback)?;

        Self::new_common(config).await
    }

    #[instrument(level = "trace", target = "core_context", err, skip_all)]
    async fn new_with_config(
        update_callback: UpdateCallback,
        config_inner: VeilidConfigInner,
    ) -> VeilidAPIResult<VeilidCoreContext> {
        // Set up config from json
        let config = VeilidConfig::new_from_config(config_inner, update_callback);
        Self::new_common(config).await
    }

    #[instrument(level = "trace", target = "core_context", err, skip_all)]
    async fn new_common(config: VeilidConfig) -> VeilidAPIResult<VeilidCoreContext> {
        cfg_if! {
            if #[cfg(target_os = "android")] {
                if !crate::intf::android::is_android_ready() {
                    apibail_internal!("Android globals are not set up");
                }
            }
        }

        let (program_name, namespace, update_callback) = {
            let cfginner = config.get();
            (
                cfginner.program_name.clone(),
                cfginner.namespace.clone(),
                config.update_callback(),
            )
        };

        let log_key = VeilidLayerFilter::make_veilid_log_key(&program_name, &namespace).to_string();
        ApiTracingLayer::add_callback(log_key, update_callback.clone()).await?;

        // Create component registry
        let registry = VeilidComponentRegistry::new(config);

        veilid_log!(registry info "Veilid API starting up");
        veilid_log!(registry info "Version: {}", veilid_version_string());
        veilid_log!(registry info "Features: {:?}", veilid_features());

        // Register all components
        registry.register(ProtectedStore::new);
        registry.register(Crypto::new);
        registry.register(TableStore::new);
        #[cfg(feature = "unstable-blockstore")]
        registry.register(BlockStore::new);
        registry.register(StorageManager::new);
        registry.register(RoutingTable::new);
        registry
            .register_with_context(NetworkManager::new, NetworkManagerStartupContext::default());
        registry.register_with_context(RPCProcessor::new, RPCProcessorStartupContext::default());
        registry.register_with_context(
            AttachmentManager::new,
            AttachmentManagerStartupContext::default(),
        );

        // Run initialization
        // This should make the majority of subsystems functional
        registry.init().await.map_err(VeilidAPIError::internal)?;

        // Run post-initialization
        // This should resolve any inter-subsystem dependencies
        // required for background processes that utilize multiple subsystems
        // Background processes also often require registry lookup of the
        // current subsystem, which is not available until after init succeeds
        if let Err(e) = registry.post_init().await {
            registry.terminate().await;
            return Err(VeilidAPIError::internal(e));
        }

        veilid_log!(registry info "Veilid API startup complete");

        Ok(Self { registry })
    }

    #[instrument(level = "trace", target = "core_context", skip_all)]
    async fn shutdown(self) {
        veilid_log!(self info "Veilid API shutting down");

        let (program_name, namespace, update_callback) = {
            let config = self.registry.config();
            let cfginner = config.get();
            (
                cfginner.program_name.clone(),
                cfginner.namespace.clone(),
                config.update_callback(),
            )
        };

        // Run pre-termination
        // This should shut down background processes that may require the existence of
        // other subsystems that may not exist during final termination
        self.registry.pre_terminate().await;

        // Run termination
        // This should finish any shutdown operations for the subsystems
        self.registry.terminate().await;

        veilid_log!(self info "Veilid API shutdown complete");

        let log_key = VeilidLayerFilter::make_veilid_log_key(&program_name, &namespace).to_string();
        if let Err(e) = ApiTracingLayer::remove_callback(log_key).await {
            error!("Error removing callback from ApiTracingLayer: {}", e);
        }

        // send final shutdown update
        update_callback(VeilidUpdate::Shutdown);
    }
}

/////////////////////////////////////////////////////////////////////////////

pub(crate) trait RegisteredComponents {
    fn protected_store<'a>(&self) -> VeilidComponentGuard<'a, ProtectedStore>;
    fn crypto<'a>(&self) -> VeilidComponentGuard<'a, Crypto>;
    fn table_store<'a>(&self) -> VeilidComponentGuard<'a, TableStore>;
    fn storage_manager<'a>(&self) -> VeilidComponentGuard<'a, StorageManager>;
    fn routing_table<'a>(&self) -> VeilidComponentGuard<'a, RoutingTable>;
    fn network_manager<'a>(&self) -> VeilidComponentGuard<'a, NetworkManager>;
    fn rpc_processor<'a>(&self) -> VeilidComponentGuard<'a, RPCProcessor>;
    fn attachment_manager<'a>(&self) -> VeilidComponentGuard<'a, AttachmentManager>;
}

impl<T: VeilidComponentRegistryAccessor> RegisteredComponents for T {
    fn protected_store<'a>(&self) -> VeilidComponentGuard<'a, ProtectedStore> {
        self.registry().lookup::<ProtectedStore>().unwrap()
    }
    fn crypto<'a>(&self) -> VeilidComponentGuard<'a, Crypto> {
        self.registry().lookup::<Crypto>().unwrap()
    }
    fn table_store<'a>(&self) -> VeilidComponentGuard<'a, TableStore> {
        self.registry().lookup::<TableStore>().unwrap()
    }
    fn storage_manager<'a>(&self) -> VeilidComponentGuard<'a, StorageManager> {
        self.registry().lookup::<StorageManager>().unwrap()
    }
    fn routing_table<'a>(&self) -> VeilidComponentGuard<'a, RoutingTable> {
        self.registry().lookup::<RoutingTable>().unwrap()
    }
    fn network_manager<'a>(&self) -> VeilidComponentGuard<'a, NetworkManager> {
        self.registry().lookup::<NetworkManager>().unwrap()
    }
    fn rpc_processor<'a>(&self) -> VeilidComponentGuard<'a, RPCProcessor> {
        self.registry().lookup::<RPCProcessor>().unwrap()
    }
    fn attachment_manager<'a>(&self) -> VeilidComponentGuard<'a, AttachmentManager> {
        self.registry().lookup::<AttachmentManager>().unwrap()
    }
}

/////////////////////////////////////////////////////////////////////////////

lazy_static::lazy_static! {
    static ref INITIALIZED: Mutex<HashSet<InitKey>> = Mutex::new(HashSet::new());
    static ref STARTUP_TABLE: AsyncTagLockTable<InitKey> = AsyncTagLockTable::new();
}

/// Initialize a Veilid node.
///
/// Must be called only once per 'program_name + namespace' combination at the start of an application.
/// The 'config_callback' must return a unique 'program_name + namespace' combination per simulataneous call to api_startup.
/// You can use the same program_name multiple times to create separate storage locations.
/// Multiple namespaces for the same program_name will use the same databases and on-disk locations, but will partition keys internally
/// to keep the namespaces distict.
///
/// * `update_callback` - called when internal state of the Veilid node changes, for example, when app-level messages are received, when private routes die and need to be reallocated, or when routing table states change.
/// * `config_callback` - called at startup to supply a configuration object directly to Veilid.
///
/// Returns a [VeilidAPI] object that can be used to operate the node.
#[instrument(level = "trace", target = "core_context", err, skip_all)]
pub async fn api_startup(
    update_callback: UpdateCallback,
    config_callback: ConfigCallback,
) -> VeilidAPIResult<VeilidAPI> {
    // Get the program_name and namespace we're starting up in
    let program_name = *config_callback("program_name".to_owned())?
        .downcast::<String>()
        .map_err(|_| {
            VeilidAPIError::invalid_argument("api_startup", "config_callback", "program_name")
        })?;
    let namespace = *config_callback("namespace".to_owned())?
        .downcast::<String>()
        .map_err(|_| {
            VeilidAPIError::invalid_argument("api_startup", "config_callback", "namespace")
        })?;
    let init_key = (program_name, namespace);

    // Only allow one startup/shutdown per program_name+namespace combination simultaneously
    let _tag_guard = STARTUP_TABLE.lock_tag(init_key.clone()).await;

    // See if we have an API started up already
    if INITIALIZED.lock().contains(&init_key) {
        apibail_already_initialized!();
    }

    // Create core context
    let context =
        VeilidCoreContext::new_with_config_callback(update_callback, config_callback).await?;

    // Return an API object around our context
    let veilid_api = VeilidAPI::new(context);

    // Add to the initialized set
    INITIALIZED.lock().insert(init_key);

    Ok(veilid_api)
}

/// Initialize a Veilid node, with the configuration in JSON format.
///
/// Must be called only once per 'program_name + namespace' combination at the start of an application.
/// The 'config_json' must specify a unique 'program_name + namespace' combination per simulataneous call to api_startup.
/// You can use the same program_name multiple times to create separate storage locations.
/// Multiple namespaces for the same program_name will use the same databases and on-disk locations, but will partition keys internally
/// to keep the namespaces distict.
///
/// * `update_callback` - called when internal state of the Veilid node changes, for example, when app-level messages are received, when private routes die and need to be reallocated, or when routing table states change.
/// * `config_json` - called at startup to supply a JSON configuration object.
///
/// Returns a [VeilidAPI] object that can be used to operate the node.
#[instrument(level = "trace", target = "core_context", err, skip_all)]
pub async fn api_startup_json(
    update_callback: UpdateCallback,
    config_json: String,
) -> VeilidAPIResult<VeilidAPI> {
    // Parse the JSON config
    let config: VeilidConfigInner =
        serde_json::from_str(&config_json).map_err(VeilidAPIError::generic)?;

    api_startup_config(update_callback, config).await
}

/// Initialize a Veilid node, with the configuration object.
///
/// Must be called only once at the start of an application.
///
/// * `update_callback` - called when internal state of the Veilid node changes, for example, when app-level messages are received, when private routes die and need to be reallocated, or when routing table states change.
/// * `config` - called at startup to supply a configuration object.
///
/// Returns a [VeilidAPI] object that can be used to operate the node.
#[instrument(level = "trace", target = "core_context", err, skip_all)]
pub async fn api_startup_config(
    update_callback: UpdateCallback,
    config: VeilidConfigInner,
) -> VeilidAPIResult<VeilidAPI> {
    // Get the program_name and namespace we're starting up in
    let program_name = config.program_name.clone();
    let namespace = config.namespace.clone();
    let init_key = (program_name, namespace);

    // Only allow one startup/shutdown per program_name+namespace combination simultaneously
    let _tag_guard = STARTUP_TABLE.lock_tag(init_key.clone()).await;
    // See if we have an API started up already
    if INITIALIZED.lock().contains(&init_key) {
        apibail_already_initialized!();
    }

    // Create core context
    let context = VeilidCoreContext::new_with_config(update_callback, config).await?;

    // Return an API object around our context
    let veilid_api = VeilidAPI::new(context);

    // Add to the initialized set
    INITIALIZED.lock().insert(init_key);

    Ok(veilid_api)
}

#[instrument(level = "trace", target = "core_context", skip_all)]
pub(crate) async fn api_shutdown(context: VeilidCoreContext) {
    let init_key = {
        let registry = context.registry();
        let config = registry.config();
        let cfginner = config.get();
        (cfginner.program_name.clone(), cfginner.namespace.clone())
    };

    // Only allow one startup/shutdown per program_name+namespace combination simultaneously
    let _tag_guard = STARTUP_TABLE.lock_tag(init_key.clone()).await;

    // See if we have an API started up already
    if !INITIALIZED.lock().contains(&init_key) {
        return;
    }

    // Shutdown the context
    context.shutdown().await;

    // Remove from the initialized set
    INITIALIZED.lock().remove(&init_key);
}
