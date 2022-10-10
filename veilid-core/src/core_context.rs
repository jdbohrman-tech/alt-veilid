use crate::api_tracing_layer::*;
use crate::attachment_manager::*;
use crate::dht::Crypto;
use crate::veilid_api::*;
use crate::veilid_config::*;
use crate::xx::*;

pub type UpdateCallback = Arc<dyn Fn(VeilidUpdate) + Send + Sync>;

struct ServicesContext {
    pub config: VeilidConfig,
    pub update_callback: UpdateCallback,

    pub protected_store: Option<ProtectedStore>,
    pub table_store: Option<TableStore>,
    pub block_store: Option<BlockStore>,
    pub crypto: Option<Crypto>,
    pub attachment_manager: Option<AttachmentManager>,
}

impl ServicesContext {
    pub fn new_empty(config: VeilidConfig, update_callback: UpdateCallback) -> Self {
        Self {
            config,
            update_callback,
            protected_store: None,
            table_store: None,
            block_store: None,
            crypto: None,
            attachment_manager: None,
        }
    }

    pub fn new_full(
        config: VeilidConfig,
        update_callback: UpdateCallback,
        protected_store: ProtectedStore,
        table_store: TableStore,
        block_store: BlockStore,
        crypto: Crypto,
        attachment_manager: AttachmentManager,
    ) -> Self {
        Self {
            config,
            update_callback,
            protected_store: Some(protected_store),
            table_store: Some(table_store),
            block_store: Some(block_store),
            crypto: Some(crypto),
            attachment_manager: Some(attachment_manager),
        }
    }

    #[instrument(err, skip_all)]
    pub async fn startup(&mut self) -> EyreResult<()> {
        info!("Veilid API starting up");

        info!("init api tracing");
        ApiTracingLayer::init(self.update_callback.clone()).await;

        // Set up protected store
        trace!("init protected store");
        let protected_store = ProtectedStore::new(self.config.clone());
        if let Err(e) = protected_store.init().await {
            self.shutdown().await;
            return Err(e);
        }
        self.protected_store = Some(protected_store.clone());

        // Init node id from config now that protected store is set up
        if let Err(e) = self.config.init_node_id(protected_store.clone()).await {
            self.shutdown().await;
            return Err(e).wrap_err("init node id failed");
        }

        // Set up tablestore
        trace!("init table store");
        let table_store = TableStore::new(self.config.clone());
        if let Err(e) = table_store.init().await {
            self.shutdown().await;
            return Err(e);
        }
        self.table_store = Some(table_store.clone());

        // Set up crypto
        trace!("init crypto");
        let crypto = Crypto::new(self.config.clone(), table_store.clone());
        if let Err(e) = crypto.init().await {
            self.shutdown().await;
            return Err(e);
        }
        self.crypto = Some(crypto.clone());

        // Set up block store
        trace!("init block store");
        let block_store = BlockStore::new(self.config.clone());
        if let Err(e) = block_store.init().await {
            self.shutdown().await;
            return Err(e);
        }
        self.block_store = Some(block_store.clone());

        // Set up attachment manager
        trace!("init attachment manager");
        let update_callback = self.update_callback.clone();
        let attachment_manager = AttachmentManager::new(self.config.clone(), protected_store, table_store, block_store, crypto);
        if let Err(e) = attachment_manager.init(update_callback).await {
            self.shutdown().await;
            return Err(e);
        }
        self.attachment_manager = Some(attachment_manager);

        info!("Veilid API startup complete");
        Ok(())
    }

    #[instrument(skip_all)]
    pub async fn shutdown(&mut self) {
        info!("Veilid API shutting down");

        if let Some(attachment_manager) = &mut self.attachment_manager {
            trace!("terminate attachment manager");
            attachment_manager.terminate().await;
        }
        if let Some(block_store) = &mut self.block_store {
            trace!("terminate block store");
            block_store.terminate().await;
        }
        if let Some(crypto) = &mut self.crypto {
            trace!("terminate crypto");
            crypto.terminate().await;
        }
        if let Some(table_store) = &mut self.table_store {
            trace!("terminate table store");
            table_store.terminate().await;
        }
        if let Some(protected_store) = &mut self.protected_store {
            trace!("terminate protected store");
            protected_store.terminate().await;
        }

        info!("Veilid API shutdown complete");

        // api logger terminate is idempotent
        ApiTracingLayer::terminate().await;

        // send final shutdown update
        (self.update_callback)(VeilidUpdate::Shutdown);
    }
}

/////////////////////////////////////////////////////////////////////////////
///
pub struct VeilidCoreContext {
    pub config: VeilidConfig,
    pub update_callback: UpdateCallback,
    // Services
    pub protected_store: ProtectedStore,
    pub table_store: TableStore,
    pub block_store: BlockStore,
    pub crypto: Crypto,
    pub attachment_manager: AttachmentManager,
}

impl VeilidCoreContext {
    #[instrument(err, skip_all)]
    async fn new_with_config_callback(
        update_callback: UpdateCallback,
        config_callback: ConfigCallback,
    ) -> Result<VeilidCoreContext, VeilidAPIError> {
        // Set up config from callback
        trace!("setup config with callback");
        let mut config = VeilidConfig::new();
        config.setup(config_callback)?;

        Self::new_common(update_callback, config).await
    }

    #[instrument(err, skip_all)]
    async fn new_with_config_json(
        update_callback: UpdateCallback,
        config_json: String,
    ) -> Result<VeilidCoreContext, VeilidAPIError> {
        // Set up config from callback
        trace!("setup config with json");
        let mut config = VeilidConfig::new();
        config.setup_from_json(config_json)?;
        Self::new_common(update_callback, config).await
    }

    #[instrument(err, skip_all)]
    async fn new_common(
        update_callback: UpdateCallback,
        config: VeilidConfig,
    ) -> Result<VeilidCoreContext, VeilidAPIError> {
        cfg_if! {
            if #[cfg(target_os = "android")] {
                if crate::intf::utils::android::ANDROID_GLOBALS.lock().is_none() {
                    error!("Android globals are not set up");
                    return Err(VeilidAPIError::Internal { message: "Android globals are not set up".to_owned() });
                }
            }
        }

        let mut sc = ServicesContext::new_empty(config.clone(), update_callback);
        sc.startup().await.map_err(VeilidAPIError::generic)?;

        Ok(VeilidCoreContext {
            update_callback: sc.update_callback,
            config: sc.config,
            protected_store: sc.protected_store.unwrap(),
            table_store: sc.table_store.unwrap(),
            block_store: sc.block_store.unwrap(),
            crypto: sc.crypto.unwrap(),
            attachment_manager: sc.attachment_manager.unwrap(),
        })
    }

    #[instrument(skip_all)]
    async fn shutdown(self) {
        let mut sc = ServicesContext::new_full(
            self.config.clone(),
            self.update_callback.clone(),
            self.protected_store,
            self.table_store,
            self.block_store,
            self.crypto,
            self.attachment_manager,
        );
        sc.shutdown().await;
    }
}

/////////////////////////////////////////////////////////////////////////////

lazy_static::lazy_static! {
    static ref INITIALIZED: AsyncMutex<bool> = AsyncMutex::new(false);
}

#[instrument(err, skip_all)]
pub async fn api_startup(
    update_callback: UpdateCallback,
    config_callback: ConfigCallback,
) -> Result<VeilidAPI, VeilidAPIError> {
    // See if we have an API started up already
    let mut initialized_lock = INITIALIZED.lock().await;
    if *initialized_lock {
        return Err(VeilidAPIError::AlreadyInitialized);
    }

    // Create core context
    let context =
        VeilidCoreContext::new_with_config_callback(update_callback, config_callback).await?;

    // Return an API object around our context
    let veilid_api = VeilidAPI::new(context);

    *initialized_lock = true;

    Ok(veilid_api)
}

#[instrument(err, skip(update_callback))]
pub async fn api_startup_json(
    update_callback: UpdateCallback,
    config_json: String,
) -> Result<VeilidAPI, VeilidAPIError> {
    // See if we have an API started up already
    let mut initialized_lock = INITIALIZED.lock().await;
    if *initialized_lock {
        return Err(VeilidAPIError::AlreadyInitialized);
    }

    // Create core context
    let context = VeilidCoreContext::new_with_config_json(update_callback, config_json).await?;

    // Return an API object around our context
    let veilid_api = VeilidAPI::new(context);

    *initialized_lock = true;

    Ok(veilid_api)
}

#[instrument(skip_all)]
pub(crate) async fn api_shutdown(context: VeilidCoreContext) {
    let mut initialized_lock = INITIALIZED.lock().await;
    context.shutdown().await;
    *initialized_lock = false;
}
