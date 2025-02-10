use super::*;

#[derive(Debug)]
struct BlueprintStateImmutable {
    /// The unique id of this blueprint
    id: BlueprintStateId,
    /// The name of this blueprint state
    name: String,
}

#[derive(Debug, Clone)]
pub struct BlueprintStateIpv4Params {
    pub locations: BlueprintLocationsList,
    pub prefix: WeightedList<u8>,
    pub gateway: Option<BlueprintStateGatewayParams>,
}

#[derive(Debug, Clone)]
pub struct BlueprintStateIpv6Params {
    pub locations: BlueprintLocationsList,
    pub prefix: WeightedList<u8>,
    pub gateway: Option<BlueprintStateGatewayParams>,
}

#[derive(Debug, Clone)]
pub struct BlueprintStateGatewayParams {
    pub translation: WeightedList<config::Translation>,
    pub upnp: Probability,
    pub locations: Option<TemplateLocationsList>,
}

#[derive(Debug, Clone)]
struct BlueprintStateIpv4 {
    params: BlueprintStateIpv4Params,
    gateway: Option<BlueprintStateIpv4Gateway>,
}

#[derive(Debug, Clone)]
struct BlueprintStateIpv6 {
    params: BlueprintStateIpv6Params,
    gateway: Option<BlueprintStateIpv6Gateway>,
}

#[derive(Debug, Clone)]
struct BlueprintStateIpv4Gateway {
    params: BlueprintStateGatewayParams,
}

#[derive(Debug, Clone)]
struct BlueprintStateIpv6Gateway {
    params: BlueprintStateGatewayParams,
}

#[derive(Debug, Clone)]
struct BlueprintStateFields {
    limit_network_count: Option<usize>,
    networks: imbl::Vector<NetworkStateId>,
    model: Option<WeightedList<Arc<String>>>,
    ipv4: Option<BlueprintStateIpv4>,
    ipv6: Option<BlueprintStateIpv6>,
}

#[derive(Debug, Clone)]
pub struct BlueprintState {
    immutable: Arc<BlueprintStateImmutable>,
    fields: Arc<BlueprintStateFields>,
}

pub type BlueprintStateId = StateId<BlueprintState>;

impl BlueprintState {
    pub fn new(id: BlueprintStateId, name: String) -> Self {
        Self {
            immutable: Arc::new(BlueprintStateImmutable { id, name }),
            fields: Arc::new(BlueprintStateFields {
                limit_network_count: None,
                networks: imbl::Vector::new(),
                model: None,
                ipv4: None,
                ipv6: None,
            }),
        }
    }

    #[instrument(level = "debug", skip(self))]
    pub fn set_limit_network_count(&mut self, limit_network_count: Option<usize>) {
        // Update fields
        self.fields = Arc::new(BlueprintStateFields {
            limit_network_count,
            ..(*self.fields).clone()
        });
    }

    #[instrument(level = "debug", skip(self))]
    pub fn set_model(&mut self, model: WeightedList<String>) {
        let model = Some(model.map(|x| Arc::new(x.clone())));
        // Update fields
        self.fields = Arc::new(BlueprintStateFields {
            model,
            ..(*self.fields).clone()
        });
    }

    #[instrument(level = "debug", skip(self, gsm_inner), err)]
    pub fn clear_ipv4(
        &mut self,
        gsm_inner: &mut GlobalStateManagerInner,
    ) -> GlobalStateManagerResult<()> {
        self.clear_ipv4_gateway(gsm_inner)?;

        if self.fields.ipv4.is_none() {
            return Ok(());
        };

        // Update fields
        self.fields = Arc::new(BlueprintStateFields {
            ipv4: None,
            ..(*self.fields).clone()
        });

        Ok(())
    }

    #[instrument(level = "debug", skip(self, _gsm_inner), err)]
    pub fn clear_ipv4_gateway(
        &mut self,
        _gsm_inner: &mut GlobalStateManagerInner,
    ) -> GlobalStateManagerResult<()> {
        let Some(mut ipv4) = self.fields.ipv4.clone() else {
            return Ok(());
        };
        let Some(_gateway) = ipv4.gateway else {
            return Ok(());
        };

        // Clear gateway
        ipv4.gateway = None;

        // Update fields
        self.fields = Arc::new(BlueprintStateFields {
            ipv4: Some(ipv4),
            ..(*self.fields).clone()
        });

        Ok(())
    }

    #[instrument(level = "debug", skip(self, gsm_inner), err)]
    pub fn set_ipv4(
        &mut self,
        gsm_inner: &mut GlobalStateManagerInner,
        params: BlueprintStateIpv4Params,
    ) -> GlobalStateManagerResult<()> {
        self.clear_ipv4(gsm_inner)?;

        let ipv4 = if let Some(ipv4) = self.fields.ipv4.clone() {
            BlueprintStateIpv4 { params, ..ipv4 }
        } else {
            BlueprintStateIpv4 {
                params,
                gateway: None,
            }
        };

        // Update fields
        self.fields = Arc::new(BlueprintStateFields {
            ipv4: Some(ipv4),
            ..(*self.fields).clone()
        });

        Ok(())
    }

    #[instrument(level = "debug", skip(self, gsm_inner), err)]
    pub fn set_ipv4_gateway(
        &mut self,
        gsm_inner: &mut GlobalStateManagerInner,
        gateway_params: Option<BlueprintStateGatewayParams>,
    ) -> GlobalStateManagerResult<()> {
        self.clear_ipv4_gateway(gsm_inner)?;

        let Some(mut ipv4) = self.fields.ipv4.clone() else {
            return Err(GlobalStateManagerError::InvalidGateway);
        };

        if ipv4.gateway.is_some() {
            if let Some(gateway_params) = gateway_params {
                ipv4.gateway.as_mut().expect("must exist").params = gateway_params;
            } else {
                ipv4.gateway = None;
            }
        } else if let Some(gateway_params) = gateway_params {
            ipv4.gateway = Some(BlueprintStateIpv4Gateway {
                params: gateway_params,
            })
        }

        // Update fields
        self.fields = Arc::new(BlueprintStateFields {
            ipv4: Some(ipv4),
            ..(*self.fields).clone()
        });

        Ok(())
    }

    #[instrument(level = "debug", skip(self, gsm_inner), err)]
    pub fn clear_ipv6(
        &mut self,
        gsm_inner: &mut GlobalStateManagerInner,
    ) -> GlobalStateManagerResult<()> {
        self.clear_ipv6_gateway(gsm_inner)?;

        if self.fields.ipv6.is_none() {
            return Ok(());
        };

        // Update fields
        self.fields = Arc::new(BlueprintStateFields {
            ipv6: None,
            ..(*self.fields).clone()
        });

        Ok(())
    }

    #[instrument(level = "debug", skip(self, _gsm_inner), err)]
    pub fn clear_ipv6_gateway(
        &mut self,
        _gsm_inner: &mut GlobalStateManagerInner,
    ) -> GlobalStateManagerResult<()> {
        let Some(mut ipv6) = self.fields.ipv6.clone() else {
            return Ok(());
        };
        let Some(_gateway) = ipv6.gateway else {
            return Ok(());
        };

        // Clear gateway
        ipv6.gateway = None;

        // Update fields
        self.fields = Arc::new(BlueprintStateFields {
            ipv6: Some(ipv6),
            ..(*self.fields).clone()
        });

        Ok(())
    }

    #[instrument(level = "debug", skip(self, gsm_inner), err)]
    pub fn set_ipv6(
        &mut self,
        gsm_inner: &mut GlobalStateManagerInner,
        params: BlueprintStateIpv6Params,
    ) -> GlobalStateManagerResult<()> {
        self.clear_ipv6(gsm_inner)?;

        let ipv6 = if let Some(ipv6) = self.fields.ipv6.clone() {
            BlueprintStateIpv6 { params, ..ipv6 }
        } else {
            BlueprintStateIpv6 {
                params,
                gateway: None,
            }
        };

        // Update fields
        self.fields = Arc::new(BlueprintStateFields {
            ipv6: Some(ipv6),
            ..(*self.fields).clone()
        });

        Ok(())
    }

    #[instrument(level = "debug", skip(self, gsm_inner), err)]
    pub fn set_ipv6_gateway(
        &mut self,
        gsm_inner: &mut GlobalStateManagerInner,
        gateway_params: Option<BlueprintStateGatewayParams>,
    ) -> GlobalStateManagerResult<()> {
        self.clear_ipv6_gateway(gsm_inner)?;

        let Some(mut ipv6) = self.fields.ipv6.clone() else {
            return Err(GlobalStateManagerError::InvalidGateway);
        };

        if ipv6.gateway.is_some() {
            if let Some(gateway_params) = gateway_params {
                ipv6.gateway.as_mut().expect("must exist").params = gateway_params;
            } else {
                ipv6.gateway = None;
            }
        } else if let Some(gateway_params) = gateway_params {
            ipv6.gateway = Some(BlueprintStateIpv6Gateway {
                params: gateway_params,
            })
        }

        // Update fields
        self.fields = Arc::new(BlueprintStateFields {
            ipv6: Some(ipv6),
            ..(*self.fields).clone()
        });

        Ok(())
    }

    pub fn is_active(&self, gsm_inner: &mut GlobalStateManagerInner) -> bool {
        // Save a backup of the entire state
        let backup = gsm_inner.clone();

        // Make a copy of this blueprint state
        let mut current_state = self.clone();

        // See what would happen if we try to generate this blueprint
        let ok = current_state.generate(gsm_inner).is_ok();

        // Restore the backup
        *gsm_inner = backup;

        // Return if this worked or not
        ok
    }

    #[instrument(level = "debug", skip(self, gsm_inner), err)]
    fn generate_model_inner(
        &mut self,
        gsm_inner: &mut GlobalStateManagerInner,
        network_state: &mut NetworkState,
    ) -> GlobalStateManagerResult<()> {
        let Some(model_list) = self.fields.model.as_ref() else {
            return Err(GlobalStateManagerError::NoDefaultModel);
        };
        let model_name = (**gsm_inner.srng().weighted_choice_ref(model_list)).clone();

        let Some(model) = gsm_inner.models().get(&model_name) else {
            return Err(GlobalStateManagerError::ModelNotFound(model_name));
        };

        let params = NetworkStateModelParams {
            latency: model.latency.clone(),
            distance: model.distance.clone(),
            loss: model.loss,
        };
        network_state.set_model(params);
        Ok(())
    }

    /// Network filter that ensures we can allocate an ipv4 gateway address on a network
    #[instrument(level = "debug", skip(self, gsm_inner), err)]
    fn gateway_network_filter_v4(
        &self,
        gsm_inner: &GlobalStateManagerInner,
        network_state_id: NetworkStateId,
    ) -> GlobalStateManagerResult<bool> {
        // Get the network state
        let network_state = gsm_inner.network_states().get_state(network_state_id)?;

        // See if we can allocate on this network
        let can_allocate = network_state.can_allocate_address_v4(None);

        Ok(can_allocate)
    }

    /// Network filter that ensures we can allocate an ipv4 gateway address on a network
    #[instrument(level = "debug", skip(self, gsm_inner), err)]
    fn gateway_network_filter_v6(
        &self,
        gsm_inner: &GlobalStateManagerInner,
        network_state_id: NetworkStateId,
    ) -> GlobalStateManagerResult<bool> {
        // Get the network state
        let network_state = gsm_inner.network_states().get_state(network_state_id)?;

        // See if we can allocate on this network
        let can_allocate = network_state.can_allocate_address_v6(None);

        Ok(can_allocate)
    }

    #[instrument(level = "debug", skip(self, gsm_inner), err)]
    fn generate_ipv4_inner(
        &mut self,
        gsm_inner: &mut GlobalStateManagerInner,
        network_state: &mut NetworkState,
    ) -> GlobalStateManagerResult<()> {
        network_state.clear_ipv4(gsm_inner)?;
        let Some(ipv4) = self.fields.ipv4.as_ref() else {
            return Ok(());
        };

        // Get addresses for network
        let Some(NetworkLocation {
            scope,
            reserve,
            super_net,
        }) = ipv4
            .params
            .locations
            .pick_v4(gsm_inner, &ipv4.params.prefix)?
        else {
            return Err(GlobalStateManagerError::BlueprintComplete(
                self.debug_name(),
            ));
        };

        let params = NetworkStateIpv4Params {
            scope,
            reserve,
            super_net,
        };

        let gateway_params = match ipv4.gateway.as_ref() {
            Some(v4gw) => {
                let translation = *gsm_inner
                    .srng()
                    .weighted_choice_ref(&v4gw.params.translation);
                let upnp = gsm_inner.srng().probability_test(v4gw.params.upnp);

                let (external_network, external_address) = match v4gw.params.locations.as_ref() {
                    Some(locations_list) => {
                        // A external network location was specified, pick one
                        // Get a network to generate the machine on
                        let Some(mut gateway_network_state) = locations_list
                            .pick(gsm_inner, |gsm_inner, id| {
                                self.gateway_network_filter_v4(gsm_inner, id)
                            })?
                        else {
                            return Err(GlobalStateManagerError::BlueprintComplete(
                                self.debug_name(),
                            ));
                        };

                        let gateway_network_state_id = gateway_network_state.id();

                        // Allocate an external address on this network
                        let external_interface_address = gateway_network_state
                            .allocate_address_v4(
                                gsm_inner,
                                OwnerTag::Gateway(network_state.id()),
                                None,
                            )?;

                        // Update the network state
                        gsm_inner
                            .network_states_mut()
                            .set_state(gateway_network_state);

                        (
                            gateway_network_state_id,
                            Some(external_interface_address.ip),
                        )
                    }
                    None => {
                        // No external network specified for gateway machine
                        // So use the same network as ourselves
                        (network_state.id(), None)
                    }
                };

                Some(NetworkStateIpv4GatewayParams {
                    translation,
                    upnp,
                    external_network,
                    internal_address: None,
                    external_address,
                })
            }
            None => None,
        };

        network_state.set_ipv4(gsm_inner, params)?;
        if let Some(gateway_params) = gateway_params {
            network_state.set_ipv4_gateway(gsm_inner, gateway_params)?;
        }
        Ok(())
    }

    #[instrument(level = "debug", skip(self, gsm_inner), err)]
    fn generate_ipv6_inner(
        &mut self,
        gsm_inner: &mut GlobalStateManagerInner,
        network_state: &mut NetworkState,
    ) -> GlobalStateManagerResult<()> {
        network_state.clear_ipv6(gsm_inner)?;
        let Some(ipv6) = self.fields.ipv6.as_ref() else {
            return Ok(());
        };

        // Get addresses for network
        let Some(NetworkLocation {
            scope,
            reserve,
            super_net,
        }) = ipv6
            .params
            .locations
            .pick_v6(gsm_inner, &ipv6.params.prefix)?
        else {
            return Err(GlobalStateManagerError::BlueprintComplete(
                self.debug_name(),
            ));
        };

        let params = NetworkStateIpv6Params {
            scope,
            reserve,
            super_net,
        };

        let gateway_params = match ipv6.gateway.as_ref() {
            Some(v6gw) => {
                let translation = *gsm_inner
                    .srng()
                    .weighted_choice_ref(&v6gw.params.translation);
                let upnp = gsm_inner.srng().probability_test(v6gw.params.upnp);

                let (external_network, external_address) = match v6gw.params.locations.as_ref() {
                    Some(locations_list) => {
                        // A external network location was specified, pick one
                        // Get a network to generate the machine on
                        let Some(mut gateway_network_state) = locations_list
                            .pick(gsm_inner, |gsm_inner, id| {
                                self.gateway_network_filter_v6(gsm_inner, id)
                            })?
                        else {
                            return Err(GlobalStateManagerError::BlueprintComplete(
                                self.debug_name(),
                            ));
                        };

                        let gateway_network_state_id = gateway_network_state.id();

                        // Allocate an external address on this network
                        let external_interface_address = gateway_network_state
                            .allocate_address_v6(
                                gsm_inner,
                                OwnerTag::Gateway(network_state.id()),
                                None,
                            )?;

                        // Update the network state
                        gsm_inner
                            .network_states_mut()
                            .set_state(gateway_network_state);

                        (
                            gateway_network_state_id,
                            Some(external_interface_address.ip),
                        )
                    }
                    None => {
                        // No external network specified for gateway machine
                        // So use the same network as ourselves
                        (network_state.id(), None)
                    }
                };

                Some(NetworkStateIpv6GatewayParams {
                    translation,
                    upnp,
                    external_network,
                    internal_address: None,
                    external_address,
                })
            }
            None => None,
        };

        network_state.set_ipv6(gsm_inner, params)?;
        if let Some(gateway_params) = gateway_params {
            network_state.set_ipv6_gateway(gsm_inner, gateway_params)?;
        }
        Ok(())
    }

    #[instrument(level = "debug", skip(self, gsm_inner), err)]
    pub fn generate(
        &mut self,
        gsm_inner: &mut GlobalStateManagerInner,
    ) -> GlobalStateManagerResult<NetworkStateId> {
        // See if there's room for another network
        if let Some(limit_network_count) = self.fields.limit_network_count {
            if self.fields.networks.len() >= limit_network_count {
                return Err(GlobalStateManagerError::BlueprintComplete(
                    self.debug_name(),
                ));
            }
        }

        // Allocate a network id
        let network_state_id = gsm_inner.network_states_mut().allocate_id();

        // Create an anonymous network state
        let mut network_state =
            NetworkState::new(network_state_id, None, NetworkOrigin::Blueprint(self.id()));

        if let Err(e) = (|| {
            self.generate_model_inner(gsm_inner, &mut network_state)?;
            self.generate_ipv4_inner(gsm_inner, &mut network_state)?;
            self.generate_ipv6_inner(gsm_inner, &mut network_state)?;
            Ok(())
        })() {
            // Release the network state and id if things failed to allocate
            network_state.release(gsm_inner);
            gsm_inner
                .network_states_mut()
                .release_id(network_state_id)
                .expect("must succeed");
            return Err(e);
        }

        // Attach the state to the id
        gsm_inner.network_states_mut().attach_state(network_state)?;

        // Record the newly instantiated network
        let mut networks = self.fields.networks.clone();
        networks.push_back(network_state_id);

        // Update fields
        self.fields = Arc::new(BlueprintStateFields {
            networks,
            ..(*self.fields).clone()
        });

        Ok(network_state_id)
    }

    #[instrument(level = "debug", skip(self, callback), err)]
    pub fn for_each_network_id<F, R>(&self, mut callback: F) -> GlobalStateManagerResult<Option<R>>
    where
        F: FnMut(NetworkStateId) -> GlobalStateManagerResult<Option<R>>,
    {
        for network_id in &self.fields.networks {
            if let Some(res) = callback(*network_id)? {
                return Ok(Some(res));
            }
        }
        Ok(None)
    }

    #[instrument(level = "debug", skip(self))]
    pub fn on_network_released(&mut self, network_id: NetworkStateId) {
        // Remove network from list
        let pos = self
            .fields
            .networks
            .iter()
            .position(|id| *id == network_id)
            .expect("must exist");
        let mut networks = self.fields.networks.clone();
        networks.remove(pos);

        // Update fields
        self.fields = Arc::new(BlueprintStateFields {
            networks,
            ..(*self.fields).clone()
        });
    }
}

impl State for BlueprintState {
    fn id(&self) -> StateId<Self> {
        self.immutable.id
    }

    fn name(&self) -> Option<String> {
        Some(self.immutable.name.clone())
    }
}
