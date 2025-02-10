use super::*;

#[derive(Debug, Clone)]
enum BlueprintAvailability {
    Existing(NetworkState),
    Generate(BlueprintState),
}

/// Locations where a machine can be instantiated when a template is generated
#[derive(Debug, Clone)]
pub enum TemplateLocationsList {
    Networks {
        networks: WeightedList<NetworkStateId>,
    },
    Blueprints {
        blueprints: WeightedList<BlueprintStateId>,
    },
}

impl TemplateLocationsList {
    #[instrument(level = "debug", skip_all, err)]
    pub fn can_pick<F>(
        &self,
        gsm_inner: &mut GlobalStateManagerInner,
        mut network_filter: F,
    ) -> GlobalStateManagerResult<bool>
    where
        F: FnMut(&GlobalStateManagerInner, NetworkStateId) -> GlobalStateManagerResult<bool>,
    {
        match self {
            TemplateLocationsList::Networks { networks } => {
                // Filter the weighted list of networks to those that are still active and or not yet started
                if networks
                    .try_filter(|id| {
                        let network_state = gsm_inner.network_states().get_state(*id)?;
                        self.is_network_available(gsm_inner, network_state, &mut network_filter)
                    })?
                    .is_none()
                {
                    return Ok(false);
                };
            }
            TemplateLocationsList::Blueprints { blueprints } => {
                // Filter the weighted list of blueprints to those that are still active or not yet started and can allocate
                if blueprints
                    .try_filter(|id| {
                        let blueprint_state = gsm_inner.blueprint_states().get_state(*id)?;

                        self.is_blueprint_available(gsm_inner, blueprint_state, &mut network_filter)
                            .map(|x| x.is_some())
                    })?
                    .is_none()
                {
                    return Ok(false);
                };
            }
        };
        Ok(true)
    }

    #[instrument(level = "debug", skip_all, err)]
    pub fn pick<F>(
        &self,
        gsm_inner: &mut GlobalStateManagerInner,
        mut network_filter: F,
    ) -> GlobalStateManagerResult<Option<NetworkState>>
    where
        F: FnMut(&GlobalStateManagerInner, NetworkStateId) -> GlobalStateManagerResult<bool>,
    {
        // Get a network to generate the machine on
        let network_state = match self {
            TemplateLocationsList::Networks { networks } => {
                // Filter the weighted list of networks to those that are still active and or not yet started
                let Some(available_networks) = networks.try_filter_map(|id| {
                    let network_state = gsm_inner.network_states().get_state(*id)?;
                    if self.is_network_available(
                        gsm_inner,
                        network_state.clone(),
                        &mut network_filter,
                    )? {
                        Ok(Some(network_state))
                    } else {
                        Ok(None)
                    }
                })?
                else {
                    return Ok(None);
                };

                // Weighted choice of network now that we have a candidate list
                let network_state = gsm_inner.srng().weighted_choice(available_networks);

                // Return network state to use
                network_state
            }
            TemplateLocationsList::Blueprints { blueprints } => {
                // Filter the weighted list of blueprints to those that are still active or not yet started and can allocate
                let Some(available_blueprints) = blueprints.try_filter_map(|id| {
                    let blueprint_state = gsm_inner.blueprint_states().get_state(*id)?;

                    self.is_blueprint_available(gsm_inner, blueprint_state, &mut network_filter)
                })?
                else {
                    return Ok(None);
                };

                // Weighted choice of blueprint now that we have a candidate list
                match gsm_inner.srng().weighted_choice(available_blueprints) {
                    BlueprintAvailability::Existing(network_state) => network_state,
                    BlueprintAvailability::Generate(mut blueprint_state) => {
                        // Generate network state from blueprint state
                        let network_state_id = blueprint_state.generate(gsm_inner)?;

                        // Update blueprint state
                        gsm_inner.blueprint_states_mut().set_state(blueprint_state);

                        // Return network state
                        gsm_inner.network_states().get_state(network_state_id)?
                    }
                }
            }
        };

        Ok(Some(network_state))
    }

    #[instrument(level = "debug", skip_all, err)]
    fn is_network_available<F>(
        &self,
        gsm_inner: &GlobalStateManagerInner,
        network_state: NetworkState,
        mut network_filter: F,
    ) -> GlobalStateManagerResult<bool>
    where
        F: FnMut(&GlobalStateManagerInner, NetworkStateId) -> GlobalStateManagerResult<bool>,
    {
        // If the network is not active, it is not available
        if !network_state.is_active()? {
            return Ok(false);
        }

        // Check the network filter
        if !network_filter(gsm_inner, network_state.id())? {
            return Ok(false);
        }

        Ok(true)
    }

    #[instrument(level = "debug", skip_all, err)]
    fn is_blueprint_available<F>(
        &self,
        gsm_inner: &mut GlobalStateManagerInner,
        blueprint_state: BlueprintState,
        mut network_filter: F,
    ) -> GlobalStateManagerResult<Option<BlueprintAvailability>>
    where
        F: FnMut(&GlobalStateManagerInner, NetworkStateId) -> GlobalStateManagerResult<bool>,
    {
        // See if the networks generated from this blueprint so far have availability
        // in this template
        if let Some(available_network_state) = blueprint_state.for_each_network_id(|id| {
            // Check the network's availability
            let network_state = gsm_inner.network_states().get_state(id)?;
            if self.is_network_available(gsm_inner, network_state.clone(), &mut network_filter)? {
                // We found one
                return Ok(Some(network_state));
            }
            // Try next network
            Ok(None)
        })? {
            // We found a usable network
            return Ok(Some(BlueprintAvailability::Existing(
                available_network_state,
            )));
        }

        // If the blueprint is active, it is available because it can make a new network
        if blueprint_state.is_active(gsm_inner) {
            return Ok(Some(BlueprintAvailability::Generate(blueprint_state)));
        }

        Ok(None)
    }
}
