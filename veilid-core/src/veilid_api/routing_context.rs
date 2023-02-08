use super::*;

///////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub enum Target {
    NodeId(PublicKey),
    PrivateRoute(PublicKey),
}

pub struct RoutingContextInner {}

pub struct RoutingContextUnlockedInner {
    /// Safety routing requirements
    safety_selection: SafetySelection,
}

impl Drop for RoutingContextInner {
    fn drop(&mut self) {
        // self.api
        //     .borrow_mut()
        //     .routing_contexts
        //     //.remove(&self.id);
    }
}

#[derive(Clone)]
pub struct RoutingContext {
    /// Veilid API handle
    api: VeilidAPI,
    inner: Arc<Mutex<RoutingContextInner>>,
    unlocked_inner: Arc<RoutingContextUnlockedInner>,
}

impl RoutingContext {
    ////////////////////////////////////////////////////////////////

    pub(super) fn new(api: VeilidAPI) -> Self {
        Self {
            api,
            inner: Arc::new(Mutex::new(RoutingContextInner {})),
            unlocked_inner: Arc::new(RoutingContextUnlockedInner {
                safety_selection: SafetySelection::Unsafe(Sequencing::default()),
            }),
        }
    }

    pub fn with_privacy(self) -> Result<Self, VeilidAPIError> {
        self.with_custom_privacy(Stability::default())
    }

    pub fn with_custom_privacy(self, stability: Stability) -> Result<Self, VeilidAPIError> {
        let config = self.api.config()?;
        let c = config.get();

        Ok(Self {
            api: self.api.clone(),
            inner: Arc::new(Mutex::new(RoutingContextInner {})),
            unlocked_inner: Arc::new(RoutingContextUnlockedInner {
                safety_selection: SafetySelection::Safe(SafetySpec {
                    preferred_route: None,
                    hop_count: c.network.rpc.default_route_hop_count as usize,
                    stability,
                    sequencing: self.sequencing(),
                }),
            }),
        })
    }
    
    pub fn with_sequencing(self, sequencing: Sequencing) -> Self {
        Self {
            api: self.api.clone(),
            inner: Arc::new(Mutex::new(RoutingContextInner {})),
            unlocked_inner: Arc::new(RoutingContextUnlockedInner {
                safety_selection: match self.unlocked_inner.safety_selection {
                    SafetySelection::Unsafe(_) => SafetySelection::Unsafe(sequencing),
                    SafetySelection::Safe(safety_spec) => SafetySelection::Safe(SafetySpec {
                        preferred_route: safety_spec.preferred_route,
                        hop_count: safety_spec.hop_count,
                        stability: safety_spec.stability,
                        sequencing,
                    }),
                },
            }),
        }
    }

    fn sequencing(&self) -> Sequencing {
        match self.unlocked_inner.safety_selection {
            SafetySelection::Unsafe(sequencing) => sequencing,
            SafetySelection::Safe(safety_spec) => safety_spec.sequencing,
        }
    }

    pub fn api(&self) -> VeilidAPI {
        self.api.clone()
    }

    async fn get_destination(
        &self,
        target: Target,
    ) -> Result<rpc_processor::Destination, VeilidAPIError> {
        let rpc_processor = self.api.rpc_processor()?;

        match target {
            Target::NodeId(node_id) => {
                // Resolve node
                let mut nr = match rpc_processor.resolve_node(node_id).await {
                    Ok(Some(nr)) => nr,
                    Ok(None) => apibail_key_not_found!(node_id),
                    Err(e) => return Err(e.into()),
                };
                // Apply sequencing to match safety selection
                nr.set_sequencing(self.sequencing());

                Ok(rpc_processor::Destination::Direct {
                    target: nr,
                    safety_selection: self.unlocked_inner.safety_selection,
                })
            }
            Target::PrivateRoute(pr) => {
                // Get remote private route
                let rss = self.api.routing_table()?.route_spec_store();
                let Some(private_route) = rss
                    .get_remote_private_route(&pr) 
                    else {
                        apibail_key_not_found!(pr);
                    };

                Ok(rpc_processor::Destination::PrivateRoute {
                    private_route,
                    safety_selection: self.unlocked_inner.safety_selection,
                })
            }
        }
    }

    ////////////////////////////////////////////////////////////////
    // App-level Messaging

    #[instrument(level = "debug", err, skip(self))]
    pub async fn app_call(
        &self,
        target: Target,
        request: Vec<u8>,
    ) -> Result<Vec<u8>, VeilidAPIError> {
        let rpc_processor = self.api.rpc_processor()?;

        // Get destination
        let dest = self.get_destination(target).await?;

        // Send app message
        let answer = match rpc_processor.rpc_call_app_call(dest, request).await {
            Ok(NetworkResult::Value(v)) => v,
            Ok(NetworkResult::Timeout) => apibail_timeout!(),
            Ok(NetworkResult::ServiceUnavailable) => apibail_try_again!(),
            Ok(NetworkResult::NoConnection(e)) | Ok(NetworkResult::AlreadyExists(e)) => {
                apibail_no_connection!(e);
            }

            Ok(NetworkResult::InvalidMessage(message)) => {
                apibail_generic!(message);
            }
            Err(e) => return Err(e.into()),
        };

        Ok(answer.answer)
    }

    #[instrument(level = "debug", err, skip(self))]
    pub async fn app_message(
        &self,
        target: Target,
        message: Vec<u8>,
    ) -> Result<(), VeilidAPIError> {
        let rpc_processor = self.api.rpc_processor()?;

        // Get destination
        let dest = self.get_destination(target).await?;

        // Send app message
        match rpc_processor.rpc_call_app_message(dest, message).await {
            Ok(NetworkResult::Value(())) => {}
            Ok(NetworkResult::Timeout) => apibail_timeout!(),
            Ok(NetworkResult::ServiceUnavailable) => apibail_try_again!(),
            Ok(NetworkResult::NoConnection(e)) | Ok(NetworkResult::AlreadyExists(e)) => {
                apibail_no_connection!(e);
            }
            Ok(NetworkResult::InvalidMessage(message)) => {
                apibail_generic!(message);
            }
            Err(e) => return Err(e.into()),
        };

        Ok(())
    }

    ///////////////////////////////////
    /// DHT Values

    pub async fn get_value(&self, _value_key: ValueKey) -> Result<Vec<u8>, VeilidAPIError> {
        panic!("unimplemented");
    }

    pub async fn set_value(
        &self,
        _value_key: ValueKey,
        _value: Vec<u8>,
    ) -> Result<bool, VeilidAPIError> {
        panic!("unimplemented");
    }

    pub async fn watch_value(
        &self,
        _value_key: ValueKey,
        _callback: ValueChangeCallback,
    ) -> Result<bool, VeilidAPIError> {
        panic!("unimplemented");
    }

    pub async fn cancel_watch_value(&self, _value_key: ValueKey) -> Result<bool, VeilidAPIError> {
        panic!("unimplemented");
    }

    ///////////////////////////////////
    /// Block Store

    pub async fn find_block(&self, _block_id: TypedKey) -> Result<Vec<u8>, VeilidAPIError> {
        panic!("unimplemented");
    }

    pub async fn supply_block(&self, _block_id: TypedKey) -> Result<bool, VeilidAPIError> {
        panic!("unimplemented");
    }
}
