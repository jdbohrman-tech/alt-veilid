use super::*;

/////////////////////////////////////////////////////////////////////////////////////////////////////

pub(super) struct VeilidAPIInner {
    context: Option<VeilidCoreContext>,
    pub(super) debug_cache: DebugCache,
}

impl fmt::Debug for VeilidAPIInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VeilidAPIInner")
    }
}

impl Drop for VeilidAPIInner {
    fn drop(&mut self) {
        if let Some(context) = self.context.take() {
            spawn_detached("api shutdown", api_shutdown(context));
        }
    }
}

/// The primary developer entrypoint into `veilid-core` functionality.
///
/// From [VeilidAPI] one can access various components:
///
/// * [VeilidConfig] - The Veilid configuration specified at startup time.
/// * [Crypto] - The available set of cryptosystems provided by Veilid.
/// * [TableStore] - The Veilid table-based encrypted persistent key-value store.
/// * [ProtectedStore] - The Veilid abstract of the device's low-level 'protected secret storage'.
/// * [VeilidState] - The current state of the Veilid node this API accesses.
/// * [RoutingContext] - Communication methods between Veilid nodes and private routes.
/// * Attach and detach from the network.
/// * Create and import private routes.
/// * Reply to `AppCall` RPCs.
#[derive(Clone, Debug)]
pub struct VeilidAPI {
    inner: Arc<Mutex<VeilidAPIInner>>,
}

impl VeilidAPI {
    #[instrument(target = "veilid_api", level = "debug", skip_all)]
    pub(crate) fn new(context: VeilidCoreContext) -> Self {
        event!(target: "veilid_api", Level::DEBUG,
            "VeilidAPI::new()");
        Self {
            inner: Arc::new(Mutex::new(VeilidAPIInner {
                context: Some(context),
                debug_cache: DebugCache {
                    imported_routes: Vec::new(),
                    opened_record_contexts: once_cell::sync::Lazy::new(
                        hashlink::LinkedHashMap::new,
                    ),
                },
            })),
        }
    }

    /// Shut down Veilid and terminate the API.
    #[instrument(target = "veilid_api", level = "debug", skip_all)]
    pub async fn shutdown(self) {
        event!(target: "veilid_api", Level::DEBUG,
            "VeilidAPI::shutdown()");
        let context = { self.inner.lock().context.take() };
        if let Some(context) = context {
            api_shutdown(context).await;
        }
    }

    /// Check to see if Veilid is already shut down.
    pub fn is_shutdown(&self) -> bool {
        self.inner.lock().context.is_none()
    }

    ////////////////////////////////////////////////////////////////
    // Public Accessors

    /// Access the configuration that Veilid was initialized with.
    pub fn config(&self) -> VeilidAPIResult<VeilidConfig> {
        let inner = self.inner.lock();
        let Some(context) = &inner.context else {
            return Err(VeilidAPIError::NotInitialized);
        };
        Ok(context.registry().config())
    }

    /// Get the cryptosystem component.
    pub fn crypto(&self) -> VeilidAPIResult<VeilidComponentGuard<'_, Crypto>> {
        let inner = self.inner.lock();
        let Some(context) = &inner.context else {
            return Err(VeilidAPIError::NotInitialized);
        };
        context
            .registry()
            .lookup::<Crypto>()
            .ok_or(VeilidAPIError::NotInitialized)
    }

    /// Get the TableStore component.
    pub fn table_store(&self) -> VeilidAPIResult<VeilidComponentGuard<'_, TableStore>> {
        let inner = self.inner.lock();
        let Some(context) = &inner.context else {
            return Err(VeilidAPIError::NotInitialized);
        };
        context
            .registry()
            .lookup::<TableStore>()
            .ok_or(VeilidAPIError::NotInitialized)
    }

    /// Get the ProtectedStore component.
    pub fn protected_store(&self) -> VeilidAPIResult<VeilidComponentGuard<'_, ProtectedStore>> {
        let inner = self.inner.lock();
        let Some(context) = &inner.context else {
            return Err(VeilidAPIError::NotInitialized);
        };
        context
            .registry()
            .lookup::<ProtectedStore>()
            .ok_or(VeilidAPIError::NotInitialized)
    }

    /// Get the BlockStore component.
    #[cfg(feature = "unstable-blockstore")]
    pub fn block_store(&self) -> VeilidAPIResult<VeilidComponentGuard<'_, BlockStore>> {
        let inner = self.inner.lock();
        let Some(context) = &inner.context else {
            return Err(VeilidAPIError::NotInitialized);
        };
        context
            .registry()
            .lookup::<BlockStore>()
            .ok_or(VeilidAPIError::NotInitialized)
    }

    ////////////////////////////////////////////////////////////////
    // Internal Accessors

    pub(crate) fn core_context(&self) -> VeilidAPIResult<VeilidCoreContext> {
        let inner = self.inner.lock();
        let Some(context) = &inner.context else {
            return Err(VeilidAPIError::NotInitialized);
        };
        Ok(context.clone())
    }

    pub(crate) fn with_debug_cache<R, F: FnOnce(&mut DebugCache) -> R>(&self, callback: F) -> R {
        let mut inner = self.inner.lock();
        callback(&mut inner.debug_cache)
    }

    ////////////////////////////////////////////////////////////////
    // Attach/Detach

    /// Get a full copy of the current state of Veilid.
    pub async fn get_state(&self) -> VeilidAPIResult<VeilidState> {
        let attachment_manager = self.core_context()?.attachment_manager();
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

    /// Connect to the network.
    #[instrument(target = "veilid_api", level = "debug", skip_all, ret, err)]
    pub async fn attach(&self) -> VeilidAPIResult<()> {
        event!(target: "veilid_api", Level::DEBUG,
            "VeilidAPI::attach()");

        let attachment_manager = self.core_context()?.attachment_manager();
        if !attachment_manager.attach().await {
            apibail_generic!("Already attached");
        }
        Ok(())
    }

    /// Disconnect from the network.
    #[instrument(target = "veilid_api", level = "debug", skip_all, ret, err)]
    pub async fn detach(&self) -> VeilidAPIResult<()> {
        event!(target: "veilid_api", Level::DEBUG,
            "VeilidAPI::detach()");

        let attachment_manager = self.core_context()?.attachment_manager();
        if !attachment_manager.detach().await {
            apibail_generic!("Already detached");
        }
        Ok(())
    }

    ////////////////////////////////////////////////////////////////
    // Routing Context

    /// Get a new `RoutingContext` object to use to send messages over the Veilid network with default safety, sequencing, and stability parameters.
    #[instrument(target = "veilid_api", level = "debug", skip_all, err, ret)]
    pub fn routing_context(&self) -> VeilidAPIResult<RoutingContext> {
        event!(target: "veilid_api", Level::DEBUG,
            "VeilidAPI::routing_context()");

        RoutingContext::try_new(self.clone())
    }

    /// Parse a string into a target object that can be used in a [RoutingContext].
    ///
    /// Strings are in base64url format and can either be a remote route id or a node id.
    /// Strings may have a [CryptoKind] [FourCC] prefix separated by a colon, such as:
    /// `VLD0:XmnGyJrjMJBRC5ayJZRPXWTBspdX36-pbLb98H3UMeE` but if the prefix is left off
    /// `XmnGyJrjMJBRC5ayJZRPXWTBspdX36-pbLb98H3UMeE` will be parsed with the 'best' cryptosystem
    /// available (at the time of this writing this is `VLD0`).
    #[instrument(target = "veilid_api", level = "debug", skip(self), fields(s=s.to_string()), ret, err)]
    pub fn parse_as_target<S: ToString>(&self, s: S) -> VeilidAPIResult<Target> {
        let s = s.to_string();

        event!(target: "veilid_api", Level::DEBUG,
            "VeilidAPI::parse_as_target(s: {:?})", s);

        // Is this a route id?
        if let Ok(rrid) = RouteId::from_str(&s) {
            let routing_table = self.core_context()?.routing_table();
            let rss = routing_table.route_spec_store();

            // Is this a valid remote route id? (can't target allocated routes)
            if rss.is_route_id_remote(&rrid) {
                return Ok(Target::PrivateRoute(rrid));
            }
        }

        // Is this a node id?
        if let Ok(nid) = TypedKey::from_str(&s) {
            return Ok(Target::NodeId(nid));
        }

        Err(VeilidAPIError::parse_error("Unable to parse as target", s))
    }

    ////////////////////////////////////////////////////////////////
    // Private route allocation

    /// Allocate a new private route set with default cryptography and network options.
    /// Default settings are for [Stability::Reliable] and [Sequencing::PreferOrdered].
    /// Returns a route id and a publishable 'blob' with the route encrypted with each crypto kind.
    /// Those nodes importing the blob will have their choice of which crypto kind to use.
    ///
    /// Returns a route id and 'blob' that can be published over some means (DHT or otherwise) to be
    /// imported by another Veilid node.
    //#[instrument(target = "veilid_api", level = "debug", skip(self), ret, err)]
    pub async fn new_private_route(&self) -> VeilidAPIResult<(RouteId, Vec<u8>)> {
        self.new_custom_private_route(
            &VALID_CRYPTO_KINDS,
            Stability::Reliable,
            Sequencing::PreferOrdered,
        )
        .await
    }

    /// Allocate a new private route and specify a specific cryptosystem, stability and sequencing preference.
    /// Faster connections may be possible with [Stability::LowLatency], and [Sequencing::NoPreference] at the
    /// expense of some loss of messages.
    /// Returns a route id and a publishable 'blob' with the route encrypted with each crypto kind.
    /// Those nodes importing the blob will have their choice of which crypto kind to use.
    ///
    /// Returns a route id and 'blob' that can be published over some means (DHT or otherwise) to be
    /// imported by another Veilid node.
    #[instrument(target = "veilid_api", level = "debug", skip(self), ret)]
    pub async fn new_custom_private_route(
        &self,
        crypto_kinds: &[CryptoKind],
        stability: Stability,
        sequencing: Sequencing,
    ) -> VeilidAPIResult<(RouteId, Vec<u8>)> {
        event!(target: "veilid_api", Level::DEBUG,
            "VeilidAPI::new_custom_private_route(crypto_kinds: {:?}, stability: {:?}, sequencing: {:?})",
            crypto_kinds,
            stability,
            sequencing);

        for kind in crypto_kinds {
            Crypto::validate_crypto_kind(*kind)?;
        }

        let default_route_hop_count: usize = {
            let config = self.config()?;
            let c = config.get();
            c.network.rpc.default_route_hop_count.into()
        };

        let safety_spec = SafetySpec {
            preferred_route: None,
            hop_count: default_route_hop_count,
            stability,
            sequencing,
        };

        let routing_table = self.core_context()?.routing_table();
        let rss = routing_table.route_spec_store();
        let route_id =
            rss.allocate_route(crypto_kinds, &safety_spec, DirectionSet::all(), &[], false)?;
        match rss.test_route(route_id).await? {
            Some(true) => {
                // route tested okay
            }
            Some(false) => {
                rss.release_route(route_id);
                apibail_generic!("allocated route failed to test");
            }
            None => {
                rss.release_route(route_id);
                apibail_generic!("allocated route could not be tested");
            }
        }
        let private_routes = rss.assemble_private_routes(&route_id, Some(true))?;
        let blob = match RouteSpecStore::private_routes_to_blob(&private_routes) {
            Ok(v) => v,
            Err(e) => {
                rss.release_route(route_id);
                return Err(e);
            }
        };

        rss.mark_route_published(&route_id, true)?;

        Ok((route_id, blob))
    }

    /// Import a private route blob as a remote private route.
    ///
    /// Returns a route id that can be used to send private messages to the node creating this route.
    #[instrument(target = "veilid_api", level = "debug", skip(self), ret, err)]
    pub fn import_remote_private_route(&self, blob: Vec<u8>) -> VeilidAPIResult<RouteId> {
        event!(target: "veilid_api", Level::DEBUG,
            "VeilidAPI::import_remote_private_route(blob: {:?})", blob);
        let routing_table = self.core_context()?.routing_table();
        let rss = routing_table.route_spec_store();
        rss.import_remote_private_route_blob(blob)
    }

    /// Release either a locally allocated or remotely imported private route.
    ///
    /// This will deactivate the route and free its resources and it can no longer be sent to
    /// or received from.
    #[instrument(target = "veilid_api", level = "debug", skip(self), ret, err)]
    pub fn release_private_route(&self, route_id: RouteId) -> VeilidAPIResult<()> {
        event!(target: "veilid_api", Level::DEBUG,
            "VeilidAPI::release_private_route(route_id: {:?})", route_id);
        let routing_table = self.core_context()?.routing_table();
        let rss = routing_table.route_spec_store();
        if !rss.release_route(route_id) {
            apibail_invalid_argument!("release_private_route", "key", route_id);
        }
        Ok(())
    }

    ////////////////////////////////////////////////////////////////
    // App Calls

    /// Respond to an AppCall received over a [VeilidUpdate::AppCall].
    ///
    /// * `call_id` - specifies which call to reply to, and it comes from a [VeilidUpdate::AppCall], specifically the [VeilidAppCall::id()] value.
    /// * `message` - is an answer blob to be returned by the remote node's [RoutingContext::app_call()] function, and may be up to 32768 bytes.
    #[instrument(target = "veilid_api", level = "debug", skip(self), ret, err)]
    pub async fn app_call_reply(
        &self,
        call_id: OperationId,
        message: Vec<u8>,
    ) -> VeilidAPIResult<()> {
        event!(target: "veilid_api", Level::DEBUG,
            "VeilidAPI::app_call_reply(call_id: {:?}, message: {:?})", call_id, message);

        let rpc_processor = self.core_context()?.rpc_processor();
        rpc_processor
            .app_call_reply(call_id, message)
            .map_err(|e| e.into())
    }

    ////////////////////////////////////////////////////////////////
    // Tunnel Building

    #[cfg(feature = "unstable-tunnels")]
    #[instrument(target = "veilid_api", level = "debug", skip(self), ret, err)]
    pub async fn start_tunnel(
        &self,
        _endpoint_mode: TunnelMode,
        _depth: u8,
    ) -> VeilidAPIResult<PartialTunnel> {
        panic!("unimplemented");
    }

    #[cfg(feature = "unstable-tunnels")]
    #[instrument(target = "veilid_api", level = "debug", skip(self), ret, err)]
    pub async fn complete_tunnel(
        &self,
        _endpoint_mode: TunnelMode,
        _depth: u8,
        _partial_tunnel: PartialTunnel,
    ) -> VeilidAPIResult<FullTunnel> {
        panic!("unimplemented");
    }

    #[cfg(feature = "unstable-tunnels")]
    #[instrument(target = "veilid_api", level = "debug", skip(self), ret, err)]
    pub async fn cancel_tunnel(&self, _tunnel_id: TunnelId) -> VeilidAPIResult<bool> {
        panic!("unimplemented");
    }
}
