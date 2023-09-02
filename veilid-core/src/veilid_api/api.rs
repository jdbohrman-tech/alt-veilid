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

/// The primary developer entrypoint into `veilid-core` functionality
///
/// From [VeilidAPI] one can access:
///
/// * [VeilidConfig] - The Veilid configuration specified by at startup time
/// * [Crypto] - The available set of cryptosystems provided by Veilid
/// * [TableStore] - The Veilid table-based encrypted persistent key-value store
/// * [ProtectedStore] - The Veilid abstract of the device's low-level 'protected secret storage'
/// * [VeilidState] - The current state of the Veilid node this API accesses
/// * [RoutingContext] - Communication methods between Veilid nodes and private routes
/// * Attach and detach from the network
/// * Create and import private routes
/// * Reply to `AppCall` RPCs
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

    /// Shut down Veilid and terminate the API
    #[instrument(skip_all)]
    pub async fn shutdown(self) {
        let context = { self.inner.lock().context.take() };
        if let Some(context) = context {
            api_shutdown(context).await;
        }
    }

    /// Check to see if Veilid is already shut down
    pub fn is_shutdown(&self) -> bool {
        self.inner.lock().context.is_none()
    }

    ////////////////////////////////////////////////////////////////
    // Public Accessors

    /// Access the configuration that Veilid was initialized with
    pub fn config(&self) -> VeilidAPIResult<VeilidConfig> {
        let inner = self.inner.lock();
        if let Some(context) = &inner.context {
            return Ok(context.config.clone());
        }
        Err(VeilidAPIError::NotInitialized)
    }

    /// Get the cryptosystem manager
    pub fn crypto(&self) -> VeilidAPIResult<Crypto> {
        let inner = self.inner.lock();
        if let Some(context) = &inner.context {
            return Ok(context.crypto.clone());
        }
        Err(VeilidAPIError::NotInitialized)
    }

    /// Get the TableStore manager
    pub fn table_store(&self) -> VeilidAPIResult<TableStore> {
        let inner = self.inner.lock();
        if let Some(context) = &inner.context {
            return Ok(context.table_store.clone());
        }
        Err(VeilidAPIError::not_initialized())
    }

    /// Get the ProtectedStore manager
    pub fn protected_store(&self) -> VeilidAPIResult<ProtectedStore> {
        let inner = self.inner.lock();
        if let Some(context) = &inner.context {
            return Ok(context.protected_store.clone());
        }
        Err(VeilidAPIError::not_initialized())
    }

    ////////////////////////////////////////////////////////////////
    // Internal Accessors
    pub(crate) fn attachment_manager(&self) -> VeilidAPIResult<AttachmentManager> {
        let inner = self.inner.lock();
        if let Some(context) = &inner.context {
            return Ok(context.attachment_manager.clone());
        }
        Err(VeilidAPIError::not_initialized())
    }
    pub(crate) fn network_manager(&self) -> VeilidAPIResult<NetworkManager> {
        let inner = self.inner.lock();
        if let Some(context) = &inner.context {
            return Ok(context.attachment_manager.network_manager());
        }
        Err(VeilidAPIError::not_initialized())
    }
    pub(crate) fn rpc_processor(&self) -> VeilidAPIResult<RPCProcessor> {
        let inner = self.inner.lock();
        if let Some(context) = &inner.context {
            return Ok(context.attachment_manager.network_manager().rpc_processor());
        }
        Err(VeilidAPIError::NotInitialized)
    }
    pub(crate) fn routing_table(&self) -> VeilidAPIResult<RoutingTable> {
        let inner = self.inner.lock();
        if let Some(context) = &inner.context {
            return Ok(context.attachment_manager.network_manager().routing_table());
        }
        Err(VeilidAPIError::NotInitialized)
    }
    pub(crate) fn storage_manager(&self) -> VeilidAPIResult<StorageManager> {
        let inner = self.inner.lock();
        if let Some(context) = &inner.context {
            return Ok(context.storage_manager.clone());
        }
        Err(VeilidAPIError::NotInitialized)
    }
    #[cfg(feature = "unstable-blockstore")]
    pub(crate) fn block_store(&self) -> VeilidAPIResult<BlockStore> {
        let inner = self.inner.lock();
        if let Some(context) = &inner.context {
            return Ok(context.block_store.clone());
        }
        Err(VeilidAPIError::not_initialized())
    }

    ////////////////////////////////////////////////////////////////
    // Attach/Detach

    /// Get a full copy of the current state of Veilid
    pub async fn get_state(&self) -> VeilidAPIResult<VeilidState> {
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
    pub async fn attach(&self) -> VeilidAPIResult<()> {
        let attachment_manager = self.attachment_manager()?;
        if !attachment_manager.attach().await {
            apibail_generic!("Already attached");
        }
        Ok(())
    }

    /// Disconnect from the network
    pub async fn detach(&self) -> VeilidAPIResult<()> {
        let attachment_manager = self.attachment_manager()?;
        if !attachment_manager.detach().await {
            apibail_generic!("Already detached");
        }
        Ok(())
    }

    ////////////////////////////////////////////////////////////////
    // Routing Context

    /// Get a new `RoutingContext` object to use to send messages over the Veilid network.
    pub fn routing_context(&self) -> RoutingContext {
        RoutingContext::new(self.clone())
    }

    /// Parse a string into a target object that can be used in a [RoutingContext]
    ///
    /// Strings are in base64url format and can either be a remote route id or a node id.
    /// Strings may have a [CryptoKind] [FourCC] prefix separated by a colon, such as:
    /// `VLD0:XmnGyJrjMJBRC5ayJZRPXWTBspdX36-pbLb98H3UMeE` but if the prefix is left off
    /// `XmnGyJrjMJBRC5ayJZRPXWTBspdX36-pbLb98H3UMeE` will be parsed with the 'best' cryptosystem
    /// available (at the time of this writing this is `VLD0`)
    pub async fn parse_as_target<S: AsRef<str>>(&self, s: S) -> VeilidAPIResult<Target> {
        // Is this a route id?
        if let Ok(rrid) = RouteId::from_str(s.as_ref()) {
            let routing_table = self.routing_table()?;
            let rss = routing_table.route_spec_store();

            // Is this a valid remote route id? (can't target allocated routes)
            if rss.is_route_id_remote(&rrid) {
                return Ok(Target::PrivateRoute(rrid));
            }
        }

        // Is this a node id?
        if let Ok(nid) = TypedKey::from_str(s.as_ref()) {
            return Ok(Target::NodeId(nid));
        }

        Err(VeilidAPIError::invalid_target())
    }

    ////////////////////////////////////////////////////////////////
    // Private route allocation

    /// Allocate a new private route set with default cryptography and network options
    /// Returns a route id and a publishable 'blob' with the route encrypted with each crypto kind
    /// Those nodes importing the blob will have their choice of which crypto kind to use
    ///
    /// Returns a route id and 'blob' that can be published over some means (DHT or otherwise) to be
    /// imported by another Veilid node.
    pub async fn new_private_route(&self) -> VeilidAPIResult<(RouteId, Vec<u8>)> {
        self.new_custom_private_route(
            &VALID_CRYPTO_KINDS,
            Stability::default(),
            Sequencing::default(),
        )
        .await
    }

    /// Allocate a new private route and specify a specific cryptosystem, stability and sequencing preference
    /// Returns a route id and a publishable 'blob' with the route encrypted with each crypto kind
    /// Those nodes importing the blob will have their choice of which crypto kind to use
    ///
    /// Returns a route id and 'blob' that can be published over some means (DHT or otherwise) to be
    /// imported by another Veilid node.
    pub async fn new_custom_private_route(
        &self,
        crypto_kinds: &[CryptoKind],
        stability: Stability,
        sequencing: Sequencing,
    ) -> VeilidAPIResult<(RouteId, Vec<u8>)> {
        for kind in crypto_kinds {
            Crypto::validate_crypto_kind(*kind)?;
        }

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

    /// Import a private route blob as a remote private route.
    ///
    /// Returns a route id that can be used to send private messages to the node creating this route.
    pub fn import_remote_private_route(&self, blob: Vec<u8>) -> VeilidAPIResult<RouteId> {
        let rss = self.routing_table()?.route_spec_store();
        rss.import_remote_private_route(blob)
            .map_err(|e| VeilidAPIError::invalid_argument(e, "blob", "private route blob"))
    }

    /// Release either a locally allocated or remotely imported private route
    ///
    /// This will deactivate the route and free its resources and it can no longer be sent to
    /// or received from.
    pub fn release_private_route(&self, route_id: RouteId) -> VeilidAPIResult<()> {
        let rss = self.routing_table()?.route_spec_store();
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
    /// * `message` - is an answer blob to be returned by the remote node's [RoutingContext::app_call()] function, and may be up to 32768 bytes
    pub async fn app_call_reply(
        &self,
        call_id: OperationId,
        message: Vec<u8>,
    ) -> VeilidAPIResult<()> {
        let rpc_processor = self.rpc_processor()?;
        rpc_processor
            .app_call_reply(call_id, message)
            .await
            .map_err(|e| e.into())
    }

    ////////////////////////////////////////////////////////////////
    // Tunnel Building

    #[cfg(feature = "unstable-tunnels")]
    pub async fn start_tunnel(
        &self,
        _endpoint_mode: TunnelMode,
        _depth: u8,
    ) -> VeilidAPIResult<PartialTunnel> {
        panic!("unimplemented");
    }

    #[cfg(feature = "unstable-tunnels")]
    pub async fn complete_tunnel(
        &self,
        _endpoint_mode: TunnelMode,
        _depth: u8,
        _partial_tunnel: PartialTunnel,
    ) -> VeilidAPIResult<FullTunnel> {
        panic!("unimplemented");
    }

    #[cfg(feature = "unstable-tunnels")]
    pub async fn cancel_tunnel(&self, _tunnel_id: TunnelId) -> VeilidAPIResult<bool> {
        panic!("unimplemented");
    }
}
