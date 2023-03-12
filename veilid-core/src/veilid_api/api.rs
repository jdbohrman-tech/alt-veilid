use super::*;

/////////////////////////////////////////////////////////////////////////////////////////////////////

struct VeilidAPIInner {
    context: Option<VeilidCoreContext>,
}

impl fmt::Debug for VeilidAPIInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VeilidAPIInner")
    }
}

impl Drop for VeilidAPIInner {
    fn drop(&mut self) {
        if let Some(context) = self.context.take() {
            spawn_detached(api_shutdown(context));
        }
    }
}

#[derive(Clone, Debug)]
pub struct VeilidAPI {
    inner: Arc<Mutex<VeilidAPIInner>>,
}

impl VeilidAPI {
    #[instrument(skip_all)]
    pub(crate) fn new(context: VeilidCoreContext) -> Self {
        Self {
            inner: Arc::new(Mutex::new(VeilidAPIInner {
                context: Some(context),
            })),
        }
    }

    #[instrument(skip_all)]
    pub async fn shutdown(self) {
        let context = { self.inner.lock().context.take() };
        if let Some(context) = context {
            api_shutdown(context).await;
        }
    }

    pub fn is_shutdown(&self) -> bool {
        self.inner.lock().context.is_none()
    }

    ////////////////////////////////////////////////////////////////
    // Accessors
    pub fn config(&self) -> Result<VeilidConfig, VeilidAPIError> {
        let inner = self.inner.lock();
        if let Some(context) = &inner.context {
            return Ok(context.config.clone());
        }
        Err(VeilidAPIError::NotInitialized)
    }
    pub fn crypto(&self) -> Result<Crypto, VeilidAPIError> {
        let inner = self.inner.lock();
        if let Some(context) = &inner.context {
            return Ok(context.crypto.clone());
        }
        Err(VeilidAPIError::NotInitialized)
    }
    pub fn table_store(&self) -> Result<TableStore, VeilidAPIError> {
        let inner = self.inner.lock();
        if let Some(context) = &inner.context {
            return Ok(context.table_store.clone());
        }
        Err(VeilidAPIError::not_initialized())
    }
    pub fn block_store(&self) -> Result<BlockStore, VeilidAPIError> {
        let inner = self.inner.lock();
        if let Some(context) = &inner.context {
            return Ok(context.block_store.clone());
        }
        Err(VeilidAPIError::not_initialized())
    }
    pub fn protected_store(&self) -> Result<ProtectedStore, VeilidAPIError> {
        let inner = self.inner.lock();
        if let Some(context) = &inner.context {
            return Ok(context.protected_store.clone());
        }
        Err(VeilidAPIError::not_initialized())
    }
    pub fn attachment_manager(&self) -> Result<AttachmentManager, VeilidAPIError> {
        let inner = self.inner.lock();
        if let Some(context) = &inner.context {
            return Ok(context.attachment_manager.clone());
        }
        Err(VeilidAPIError::not_initialized())
    }
    pub fn network_manager(&self) -> Result<NetworkManager, VeilidAPIError> {
        let inner = self.inner.lock();
        if let Some(context) = &inner.context {
            return Ok(context.attachment_manager.network_manager());
        }
        Err(VeilidAPIError::not_initialized())
    }
    pub fn rpc_processor(&self) -> Result<RPCProcessor, VeilidAPIError> {
        let inner = self.inner.lock();
        if let Some(context) = &inner.context {
            return Ok(context.attachment_manager.network_manager().rpc_processor());
        }
        Err(VeilidAPIError::NotInitialized)
    }
    pub fn routing_table(&self) -> Result<RoutingTable, VeilidAPIError> {
        let inner = self.inner.lock();
        if let Some(context) = &inner.context {
            return Ok(context.attachment_manager.network_manager().routing_table());
        }
        Err(VeilidAPIError::NotInitialized)
    }

    ////////////////////////////////////////////////////////////////
    // Attach/Detach

    /// Get a full copy of the current state
    pub async fn get_state(&self) -> Result<VeilidState, VeilidAPIError> {
        let attachment_manager = self.attachment_manager()?;
        let network_manager = attachment_manager.network_manager();
        let config = self.config()?;

        let attachment = attachment_manager.get_veilid_state();
        let network = network_manager.get_veilid_state();
        let config = config.get_veilid_state();

        Ok(VeilidState {
            attachment,
            network,
            config,
        })
    }

    /// Connect to the network
    #[instrument(level = "debug", err, skip_all)]
    pub async fn attach(&self) -> Result<(), VeilidAPIError> {
        let attachment_manager = self.attachment_manager()?;
        if !attachment_manager.attach().await {
            apibail_generic!("Already attached");
        }
        Ok(())
    }

    /// Disconnect from the network
    #[instrument(level = "debug", err, skip_all)]
    pub async fn detach(&self) -> Result<(), VeilidAPIError> {
        let attachment_manager = self.attachment_manager()?;
        if !attachment_manager.detach().await {
            apibail_generic!("Already detached");
        }
        Ok(())
    }

    ////////////////////////////////////////////////////////////////
    // Routing Context

    #[instrument(level = "debug", skip(self))]
    pub fn routing_context(&self) -> RoutingContext {
        RoutingContext::new(self.clone())
    }

    ////////////////////////////////////////////////////////////////
    // Private route allocation

    /// Allocate a new private route set with default cryptography and network options
    /// Returns a route id and a publishable 'blob' with the route encrypted with each crypto kind
    /// Those nodes importing the blob will have their choice of which crypto kind to use
    #[instrument(level = "debug", skip(self))]
    pub async fn new_private_route(&self) -> Result<(RouteId, Vec<u8>), VeilidAPIError> {
        self.new_custom_private_route(
            &VALID_CRYPTO_KINDS,
            Stability::default(),
            Sequencing::default(),
        )
        .await
    }

    ///
    #[instrument(level = "debug", skip(self))]
    pub async fn new_custom_private_route(
        &self,
        crypto_kinds: &[CryptoKind],
        stability: Stability,
        sequencing: Sequencing,
    ) -> Result<(RouteId, Vec<u8>), VeilidAPIError> {
        let default_route_hop_count: usize = {
            let config = self.config()?;
            let c = config.get();
            c.network.rpc.default_route_hop_count.into()
        };

        let rss = self.routing_table()?.route_spec_store();
        let r = rss
            .allocate_route(
                &crypto_kinds,
                stability,
                sequencing,
                default_route_hop_count,
                Direction::Inbound.into(),
                &[],
            )
            .map_err(VeilidAPIError::internal)?;
        let Some(route_id) = r else {
            apibail_generic!("unable to allocate route");
        };
        if !rss
            .test_route(route_id.clone())
            .await
            .map_err(VeilidAPIError::no_connection)?
        {
            rss.release_route(route_id);
            apibail_generic!("allocated route failed to test");
        }
        let private_routes = rss
            .assemble_private_routes(&route_id, Some(true))
            .map_err(VeilidAPIError::generic)?;
        let blob = match RouteSpecStore::private_routes_to_blob(&private_routes) {
            Ok(v) => v,
            Err(e) => {
                rss.release_route(route_id);
                apibail_internal!(e);
            }
        };

        rss.mark_route_published(&route_id, true)
            .map_err(VeilidAPIError::internal)?;

        Ok((route_id, blob))
    }

    #[instrument(level = "debug", skip(self))]
    pub fn import_remote_private_route(&self, blob: Vec<u8>) -> Result<RouteId, VeilidAPIError> {
        let rss = self.routing_table()?.route_spec_store();
        rss.import_remote_private_route(blob)
            .map_err(|e| VeilidAPIError::invalid_argument(e, "blob", "private route blob"))
    }

    #[instrument(level = "debug", skip(self))]
    pub fn release_private_route(&self, route_id: RouteId) -> Result<(), VeilidAPIError> {
        let rss = self.routing_table()?.route_spec_store();
        if !rss.release_route(route_id) {
            apibail_invalid_argument!("release_private_route", "key", route_id);
        }
        Ok(())
    }

    ////////////////////////////////////////////////////////////////
    // App Calls

    #[instrument(level = "debug", skip(self))]
    pub async fn app_call_reply(
        &self,
        id: OperationId,
        message: Vec<u8>,
    ) -> Result<(), VeilidAPIError> {
        let rpc_processor = self.rpc_processor()?;
        rpc_processor
            .app_call_reply(id, message)
            .await
            .map_err(|e| e.into())
    }

    ////////////////////////////////////////////////////////////////
    // Tunnel Building

    #[instrument(level = "debug", err, skip(self))]
    pub async fn start_tunnel(
        &self,
        _endpoint_mode: TunnelMode,
        _depth: u8,
    ) -> Result<PartialTunnel, VeilidAPIError> {
        panic!("unimplemented");
    }

    #[instrument(level = "debug", err, skip(self))]
    pub async fn complete_tunnel(
        &self,
        _endpoint_mode: TunnelMode,
        _depth: u8,
        _partial_tunnel: PartialTunnel,
    ) -> Result<FullTunnel, VeilidAPIError> {
        panic!("unimplemented");
    }

    #[instrument(level = "debug", err, skip(self))]
    pub async fn cancel_tunnel(&self, _tunnel_id: TunnelId) -> Result<bool, VeilidAPIError> {
        panic!("unimplemented");
    }
}
