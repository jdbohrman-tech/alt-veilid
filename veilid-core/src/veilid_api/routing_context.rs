use super::*;
///////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
pub enum Target {
    NodeId(NodeId),
    PrivateRoute(PrivateRoute),
}

pub struct RoutingContextInner {}

pub struct RoutingContextUnlockedInner {
    /// Enforce use of private routing
    privacy: usize,
    /// Choose reliable protocols over unreliable/faster protocols when available
    reliable: bool,
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
                privacy: 0,
                reliable: false,
            }),
        }
    }

    pub fn with_default_privacy(self) -> Result<Self, VeilidAPIError> {
        let config = self.api.config()?;
        let c = config.get();
        Ok(Self {
            api: self.api.clone(),
            inner: Arc::new(Mutex::new(RoutingContextInner {})),
            unlocked_inner: Arc::new(RoutingContextUnlockedInner {
                privacy: c.network.rpc.default_route_hop_count as usize,
                reliable: self.unlocked_inner.reliable,
            }),
        })
    }
    pub fn with_privacy(self, hops: usize) -> Result<Self, VeilidAPIError> {
        let config = self.api.config()?;
        let c = config.get();

        let privacy = if hops > 0 && hops <= c.network.rpc.max_route_hop_count as usize {
            hops
        } else {
            return Err(VeilidAPIError::invalid_argument(
                "hops value is too large",
                "hops",
                hops,
            ));
        };
        Ok(Self {
            api: self.api.clone(),
            inner: Arc::new(Mutex::new(RoutingContextInner {})),
            unlocked_inner: Arc::new(RoutingContextUnlockedInner {
                privacy,
                reliable: self.unlocked_inner.reliable,
            }),
        })
    }

    pub fn with_reliability(self) -> Self {
        Self {
            api: self.api.clone(),
            inner: Arc::new(Mutex::new(RoutingContextInner {})),
            unlocked_inner: Arc::new(RoutingContextUnlockedInner {
                privacy: self.unlocked_inner.privacy,
                reliable: true,
            }),
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
                let mut nr = match rpc_processor.resolve_node(node_id.key).await {
                    Ok(Some(nr)) => nr,
                    Ok(None) => return Err(VeilidAPIError::NodeNotFound { node_id }),
                    Err(e) => return Err(e.into()),
                };
                // Apply reliability sort
                if self.unlocked_inner.reliable {
                    nr.set_reliable();
                }
                Ok(rpc_processor::Destination::Direct {
                    target: nr,
                    safety_spec: Some(routing_table::SafetySpec {
                        preferred_route: None,
                        hop_count: self.unlocked_inner.privacy,
                        reliable: self.unlocked_inner.reliable,
                    }),
                })
            }
            Target::PrivateRoute(pr) => Ok(rpc_processor::Destination::PrivateRoute {
                private_route: pr,
                safety_spec: Some(routing_table::SafetySpec {
                    preferred_route: None,
                    hop_count: self.unlocked_inner.privacy,
                    reliable: self.unlocked_inner.reliable,
                }),
                reliable: self.unlocked_inner.reliable,
            }),
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
            Ok(NetworkResult::Timeout) => return Err(VeilidAPIError::Timeout),
            Ok(NetworkResult::NoConnection(e)) | Ok(NetworkResult::AlreadyExists(e)) => {
                return Err(VeilidAPIError::NoConnection {
                    message: e.to_string(),
                })
            }

            Ok(NetworkResult::InvalidMessage(message)) => {
                return Err(VeilidAPIError::Generic { message })
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
            Ok(NetworkResult::Timeout) => return Err(VeilidAPIError::Timeout),
            Ok(NetworkResult::NoConnection(e)) | Ok(NetworkResult::AlreadyExists(e)) => {
                return Err(VeilidAPIError::NoConnection {
                    message: e.to_string(),
                })
            }
            Ok(NetworkResult::InvalidMessage(message)) => {
                return Err(VeilidAPIError::Generic { message })
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

    pub async fn find_block(&self, _block_id: BlockId) -> Result<Vec<u8>, VeilidAPIError> {
        panic!("unimplemented");
    }

    pub async fn supply_block(&self, _block_id: BlockId) -> Result<bool, VeilidAPIError> {
        panic!("unimplemented");
    }
}
