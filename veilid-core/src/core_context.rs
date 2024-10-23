use crate::attachment_manager::*;
use crate::crypto::Crypto;
use crate::logging::*;
use crate::storage_manager::*;
use crate::veilid_api::*;
use crate::veilid_config::*;
use crate::*;

pub type UpdateCallback = Arc<dyn Fn(VeilidUpdate) + Send + Sync>;

/// Internal services startup mechanism.
/// Ensures that everything is started up, and shut down in the right order
/// and provides an atomic state for if the system is properly operational.
struct StartupShutdownContext {
    pub config: VeilidConfig,
    pub update_callback: UpdateCallback,

    pub event_bus: Option<EventBus>,
    pub protected_store: Option<ProtectedStore>,
    pub table_store: Option<TableStore>,
    #[cfg(feature = "unstable-blockstore")]
    pub block_store: Option<BlockStore>,
    pub crypto: Option<Crypto>,
    pub attachment_manager: Option<AttachmentManager>,
    pub storage_manager: Option<StorageManager>,
}

impl StartupShutdownContext {
    pub fn new_empty(config: VeilidConfig, update_callback: UpdateCallback) -> Self {
        Self {
            config,
            update_callback,
            event_bus: None,
            protected_store: None,
            table_store: None,
            #[cfg(feature = "unstable-blockstore")]
            block_store: None,
            crypto: None,
            attachment_manager: None,
            storage_manager: None,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new_full(
        config: VeilidConfig,
        update_callback: UpdateCallback,
        event_bus: EventBus,
        protected_store: ProtectedStore,
        table_store: TableStore,
        #[cfg(feature = "unstable-blockstore")] block_store: BlockStore,
        crypto: Crypto,
        attachment_manager: AttachmentManager,
        storage_manager: StorageManager,
    ) -> Self {
        Self {
            config,
            update_callback,
            event_bus: Some(event_bus),
            protected_store: Some(protected_store),
            table_store: Some(table_store),
            #[cfg(feature = "unstable-blockstore")]
            block_store: Some(block_store),
            crypto: Some(crypto),
            attachment_manager: Some(attachment_manager),
            storage_manager: Some(storage_manager),
        }
    }

    #[instrument(level = "trace", target = "core_context", err, skip_all)]
    pub async fn startup(&mut self) -> EyreResult<()> {
        info!("Veilid API starting up");

        info!("init api tracing");
        let (program_name, namespace) = {
            let config = self.config.get();
            (config.program_name.clone(), config.namespace.clone())
        };

        ApiTracingLayer::add_callback(program_name, namespace, self.update_callback.clone())
            .await?;

        // Add the event bus
        let event_bus = EventBus::new();
        if let Err(e) = event_bus.startup().await {
            error!("failed to start up event bus: {}", e);
            self.shutdown().await;
            return Err(e.into());
        }
        self.event_bus = Some(event_bus.clone());

        // Set up protected store
        let protected_store = ProtectedStore::new(event_bus.clone(), self.config.clone());
        if let Err(e) = protected_store.init().await {
            error!("failed to init protected store: {}", e);
            self.shutdown().await;
            return Err(e);
        }
        self.protected_store = Some(protected_store.clone());

        // Set up tablestore and crypto system
        let table_store = TableStore::new(
            event_bus.clone(),
            self.config.clone(),
            protected_store.clone(),
        );
        let crypto = Crypto::new(event_bus.clone(), self.config.clone(), table_store.clone());
        table_store.set_crypto(crypto.clone());

        // Initialize table store first, so crypto code can load caches
        // Tablestore can use crypto during init, just not any cached operations or things
        // that require flushing back to the tablestore
        if let Err(e) = table_store.init().await {
            error!("failed to init table store: {}", e);
            self.shutdown().await;
            return Err(e);
        }
        self.table_store = Some(table_store.clone());

        // Set up crypto
        if let Err(e) = crypto.init().await {
            error!("failed to init crypto: {}", e);
            self.shutdown().await;
            return Err(e);
        }
        self.crypto = Some(crypto.clone());

        // Set up block store
        #[cfg(feature = "unstable-blockstore")]
        {
            let block_store = BlockStore::new(event_bus.clone(), self.config.clone());
            if let Err(e) = block_store.init().await {
                error!("failed to init block store: {}", e);
                self.shutdown().await;
                return Err(e);
            }
            self.block_store = Some(block_store.clone());
        }

        // Set up storage manager
        let update_callback = self.update_callback.clone();

        let storage_manager = StorageManager::new(
            event_bus.clone(),
            self.config.clone(),
            self.crypto.clone().unwrap(),
            self.table_store.clone().unwrap(),
            #[cfg(feature = "unstable-blockstore")]
            self.block_store.clone().unwrap(),
        );
        if let Err(e) = storage_manager.init(update_callback).await {
            error!("failed to init storage manager: {}", e);
            self.shutdown().await;
            return Err(e);
        }
        self.storage_manager = Some(storage_manager.clone());

        // Set up attachment manager
        let update_callback = self.update_callback.clone();
        let attachment_manager = AttachmentManager::new(
            event_bus.clone(),
            self.config.clone(),
            storage_manager,
            table_store,
            #[cfg(feature = "unstable-blockstore")]
            block_store,
            crypto,
        );
        if let Err(e) = attachment_manager.init(update_callback).await {
            error!("failed to init attachment manager: {}", e);
            self.shutdown().await;
            return Err(e);
        }
        self.attachment_manager = Some(attachment_manager);

        info!("Veilid API startup complete");
        Ok(())
    }

    #[instrument(level = "trace", target = "core_context", skip_all)]
    pub async fn shutdown(&mut self) {
        info!("Veilid API shutting down");

        if let Some(attachment_manager) = &mut self.attachment_manager {
            attachment_manager.terminate().await;
        }
        if let Some(storage_manager) = &mut self.storage_manager {
            storage_manager.terminate().await;
        }
        #[cfg(feature = "unstable-blockstore")]
        if let Some(block_store) = &mut self.block_store {
            block_store.terminate().await;
        }
        if let Some(crypto) = &mut self.crypto {
            crypto.terminate().await;
        }
        if let Some(table_store) = &mut self.table_store {
            table_store.terminate().await;
        }
        if let Some(protected_store) = &mut self.protected_store {
            protected_store.terminate().await;
        }
        if let Some(event_bus) = &mut self.event_bus {
            event_bus.shutdown().await;
        }

        info!("Veilid API shutdown complete");

        // api logger terminate is idempotent
        let (program_name, namespace) = {
            let config = self.config.get();
            (config.program_name.clone(), config.namespace.clone())
        };

        if let Err(e) = ApiTracingLayer::remove_callback(program_name, namespace).await {
            error!("Error removing callback from ApiTracingLayer: {}", e);
        }

        // send final shutdown update
        (self.update_callback)(VeilidUpdate::Shutdown);
    }
}

/////////////////////////////////////////////////////////////////////////////
pub struct VeilidCoreContext {
    pub config: VeilidConfig,
    pub update_callback: UpdateCallback,
    // Event bus
    pub event_bus: EventBus,
    // Services
    pub storage_manager: StorageManager,
    pub protected_store: ProtectedStore,
    pub table_store: TableStore,
    #[cfg(feature = "unstable-blockstore")]
    pub block_store: BlockStore,
    pub crypto: Crypto,
    pub attachment_manager: AttachmentManager,
}

impl VeilidCoreContext {
    #[instrument(level = "trace", target = "core_context", err, skip_all)]
    async fn new_with_config_callback(
        update_callback: UpdateCallback,
        config_callback: ConfigCallback,
    ) -> VeilidAPIResult<VeilidCoreContext> {
        // Set up config from callback
        let mut config = VeilidConfig::new();
        config.setup(config_callback, update_callback.clone())?;

        Self::new_common(update_callback, config).await
    }

    #[instrument(level = "trace", target = "core_context", err, skip_all)]
    async fn new_with_config(
        update_callback: UpdateCallback,
        config_inner: VeilidConfigInner,
    ) -> VeilidAPIResult<VeilidCoreContext> {
        // Set up config from json
        let mut config = VeilidConfig::new();
        config.setup_from_config(config_inner, update_callback.clone())?;
        Self::new_common(update_callback, config).await
    }

    #[instrument(level = "trace", target = "core_context", err, skip_all)]
    async fn new_common(
        update_callback: UpdateCallback,
        config: VeilidConfig,
    ) -> VeilidAPIResult<VeilidCoreContext> {
        cfg_if! {
            if #[cfg(target_os = "android")] {
                if !crate::intf::android::is_android_ready() {
                    apibail_internal!("Android globals are not set up");
                }
            }
        }

        let mut sc = StartupShutdownContext::new_empty(config.clone(), update_callback);
        sc.startup().await.map_err(VeilidAPIError::generic)?;

        Ok(VeilidCoreContext {
            config: sc.config,
            update_callback: sc.update_callback,
            event_bus: sc.event_bus.unwrap(),
            storage_manager: sc.storage_manager.unwrap(),
            protected_store: sc.protected_store.unwrap(),
            table_store: sc.table_store.unwrap(),
            #[cfg(feature = "unstable-blockstore")]
            block_store: sc.block_store.unwrap(),
            crypto: sc.crypto.unwrap(),
            attachment_manager: sc.attachment_manager.unwrap(),
        })
    }

    #[instrument(level = "trace", target = "core_context", skip_all)]
    async fn shutdown(self) {
        let mut sc = StartupShutdownContext::new_full(
            self.config.clone(),
            self.update_callback.clone(),
            self.event_bus,
            self.protected_store,
            self.table_store,
            #[cfg(feature = "unstable-blockstore")]
            self.block_store,
            self.crypto,
            self.attachment_manager,
            self.storage_manager,
        );
        sc.shutdown().await;
    }
}

/////////////////////////////////////////////////////////////////////////////

lazy_static::lazy_static! {
    static ref INITIALIZED: AsyncMutex<HashSet<(String,String)>> = AsyncMutex::new(HashSet::new());
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

    // See if we have an API started up already
    let mut initialized_lock = INITIALIZED.lock().await;
    if initialized_lock.contains(&init_key) {
        apibail_already_initialized!();
    }

    // Create core context
    let context =
        VeilidCoreContext::new_with_config_callback(update_callback, config_callback).await?;

    // Return an API object around our context
    let veilid_api = VeilidAPI::new(context);

    initialized_lock.insert(init_key);

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

    // See if we have an API started up already
    let mut initialized_lock = INITIALIZED.lock().await;
    if initialized_lock.contains(&init_key) {
        apibail_already_initialized!();
    }

    // Create core context
    let context = VeilidCoreContext::new_with_config(update_callback, config).await?;

    // Return an API object around our context
    let veilid_api = VeilidAPI::new(context);

    initialized_lock.insert(init_key);

    Ok(veilid_api)
}

#[instrument(level = "trace", target = "core_context", skip_all)]
pub async fn api_shutdown(context: VeilidCoreContext) {
    let mut initialized_lock = INITIALIZED.lock().await;

    let init_key = {
        let config = context.config.get();
        (config.program_name.clone(), config.namespace.clone())
    };

    context.shutdown().await;
    initialized_lock.remove(&init_key);
}
