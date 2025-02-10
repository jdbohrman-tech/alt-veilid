use super::*;

#[derive(Debug, Clone)]
pub(super) struct Allocation {
    pub config: config::Allocation,
    pub address_pool: AddressPool<()>,
}

#[derive(Debug, Clone)]
pub(super) struct GlobalStateManagerInner {
    unlocked_inner: Arc<GlobalStateManagerUnlockedInner>,
    srng: StableRng,
    default_network: Option<String>,
    default_model: Option<String>,
    default_pool: Option<String>,
    models: imbl::HashMap<String, config::Model>,
    allocations: imbl::HashMap<String, Arc<Allocation>>,
    allocated_machines: imbl::HashSet<MachineStateId>,
    profile_state_registry: StateRegistry<ProfileState>,
    machine_state_registry: StateRegistry<MachineState>,
    template_state_registry: StateRegistry<TemplateState>,
    network_state_registry: StateRegistry<NetworkState>,
    blueprint_state_registry: StateRegistry<BlueprintState>,
}

impl GlobalStateManagerInner {
    ///////////////////////////////////////////////////////////
    /// Public Interface

    pub fn new(unlocked_inner: Arc<GlobalStateManagerUnlockedInner>) -> Self {
        GlobalStateManagerInner {
            unlocked_inner,
            srng: StableRng::new(0),
            default_network: None,
            default_model: None,
            default_pool: None,
            models: imbl::HashMap::new(),
            allocations: imbl::HashMap::new(),
            allocated_machines: imbl::HashSet::new(),
            profile_state_registry: StateRegistry::new(),
            machine_state_registry: StateRegistry::new(),
            template_state_registry: StateRegistry::new(),
            network_state_registry: StateRegistry::new(),
            blueprint_state_registry: StateRegistry::new(),
        }
    }

    #[instrument(level = "debug", skip_all, err)]
    pub fn execute_config(&mut self, cfg: config::Config) -> GlobalStateManagerResult<()> {
        // Create random number generator
        if let Some(seed) = cfg.seed {
            self.srng = StableRng::new(seed);
        }

        // Set default network name
        if let Some(default_network) = cfg.default_network {
            self.default_network = Some(default_network);
        }

        // Set default model name
        if let Some(default_model) = cfg.default_model {
            self.default_model = Some(default_model);
        }

        // Set default pool name
        if let Some(default_pool) = cfg.default_pool {
            self.default_pool = Some(default_pool);
        }

        // Import all allocation definitions
        self.execute_config_allocations(&cfg.allocations)?;

        // Import all models
        for (name, model) in cfg.models {
            self.execute_config_model(&name, &model)?;
        }

        // Create all profile states
        for (name, profile) in cfg.profiles {
            self.execute_config_profile(&name, &profile)?;
        }

        // Create all network states
        // Don't process gateways yet because they will depend on networks existing
        for (name, network) in &cfg.networks {
            self.execute_config_network(name, network)?;
        }
        // Process all ipv4 and ipv6 configurations
        for (name, network) in &cfg.networks {
            if let Some(ipv4) = network.ipv4.as_ref() {
                self.execute_config_network_ipv4(name, ipv4)?;
            }
            if let Some(ipv6) = network.ipv6.as_ref() {
                self.execute_config_network_ipv6(name, ipv6)?;
            }
        }
        // Process all network gateways
        for (name, network) in &cfg.networks {
            if let Some(ipv4) = network.ipv4.as_ref() {
                if let Some(ipv4gw) = ipv4.gateway.as_ref() {
                    self.execute_config_network_ipv4_gateway(name, ipv4gw)?;
                }
            }
            if let Some(ipv6) = network.ipv6.as_ref() {
                if let Some(ipv6gw) = ipv6.gateway.as_ref() {
                    self.execute_config_network_ipv6_gateway(name, ipv6gw)?;
                }
            }
        }

        // Create all blueprint states
        // Don't process gateways yet because they will depend on blueprints existing
        for (name, blueprint) in &cfg.blueprints {
            self.execute_config_blueprint(name, blueprint)?;
        }
        // Process all ipv4 and ipv6 configurations
        for (name, blueprint) in &cfg.blueprints {
            if let Some(ipv4) = blueprint.ipv4.as_ref() {
                self.execute_config_blueprint_ipv4(name, ipv4)?;
            }
            if let Some(ipv6) = blueprint.ipv6.as_ref() {
                self.execute_config_blueprint_ipv6(name, ipv6)?;
            }
        }
        // Process all blueprint gateways
        for (name, blueprint) in &cfg.blueprints {
            if let Some(ipv4) = blueprint.ipv4.as_ref() {
                if let Some(ipv4gw) = ipv4.gateway.as_ref() {
                    self.execute_config_blueprint_ipv4_gateway(name, ipv4gw)?;
                }
            }
            if let Some(ipv6) = blueprint.ipv6.as_ref() {
                if let Some(ipv6gw) = ipv6.gateway.as_ref() {
                    self.execute_config_blueprint_ipv6_gateway(name, ipv6gw)?;
                }
            }
        }

        // Create all template states
        for (name, template) in &cfg.templates {
            self.execute_config_template(name, template)?;
        }

        // Create all machine states
        for (name, machine) in &cfg.machines {
            self.execute_config_machine(name, machine)?;
        }

        Ok(())
    }

    pub fn allocate(&mut self, profile: String) -> GlobalStateManagerResult<MachineId> {
        // Get current profile state
        let Some(profile_state_id) = self.profile_state_registry.get_state_id_by_name(&profile)
        else {
            return Err(GlobalStateManagerError::ProfileNotFound(profile));
        };

        // Get the next instance from the definition
        loop {
            // Move to the next profile instance
            let mut profile_state = self.profile_states().get_state(profile_state_id)?;
            let Some(instance_def) = profile_state.next_instance() else {
                return Err(GlobalStateManagerError::ProfileComplete(profile));
            };
            self.profile_states_mut().set_state(profile_state);

            let machine_state_id = match instance_def {
                config::Instance::Machine {
                    machine: machine_names,
                } => {
                    // Filter out machines that are already allocated
                    let opt_machine_states_ids = machine_names.try_filter_map(|name| {
                        let Some(machine_state_id) =
                            self.machine_states().get_state_id_by_name(name)
                        else {
                            return Err(GlobalStateManagerError::MachineNotFound(name.clone()));
                        };
                        if self.allocated_machines.contains(&machine_state_id) {
                            Ok(None)
                        } else {
                            Ok(Some(machine_state_id))
                        }
                    })?;
                    let Some(machine_state_ids) = opt_machine_states_ids else {
                        // All machines in this instance are allocated
                        continue;
                    };

                    // Choose a machine state to activate
                    let machine_state_id = self.srng.weighted_choice(machine_state_ids);

                    // Activate it
                    self.allocated_machines.insert(machine_state_id);

                    machine_state_id
                }
                config::Instance::Template {
                    template: template_names,
                } => {
                    // Filter out templates that are no longer active
                    let opt_template_states = template_names.try_filter_map(|name| {
                        let Some(template_state_id) =
                            self.template_states().get_state_id_by_name(name)
                        else {
                            return Err(GlobalStateManagerError::TemplateNotFound(name.clone()));
                        };
                        let template_state = self
                            .template_states()
                            .get_state(template_state_id)
                            .expect("must exist");
                        if !template_state.is_active(self) {
                            Ok(None)
                        } else {
                            Ok(Some(template_state))
                        }
                    })?;
                    let Some(template_states) = opt_template_states else {
                        // No templates in this instance are still active
                        continue;
                    };

                    // Chose a template
                    let mut template_state = self.srng.weighted_choice(template_states);

                    // Generate a machine from the template
                    let machine_state_id = template_state.generate(self)?;

                    // Save the updated template
                    self.template_states_mut().set_state(template_state);

                    machine_state_id
                }
            };

            break Ok(machine_state_id.external_id());
        }
    }

    pub fn release(&mut self, machine_id: MachineId) -> GlobalStateManagerResult<()> {
        let id = StateId::<MachineState>::new(machine_id);
        if self.allocated_machines.contains(&id) {
            // Was a fixed machine, so we leave the machine state so it can
            // be reallocated later
            self.allocated_machines.remove(&id);
        } else {
            // Was a templated machine, so remove the machine state
            let machine_state = self.machine_states().get_state(id)?;
            machine_state.release(self);
            self.machine_states_mut().release_id(id)?;
        }

        Ok(())
    }

    ///////////////////////////////////////////////////////////
    /// Private Implementation

    pub(super) fn srng(&mut self) -> &mut StableRng {
        &mut self.srng
    }

    pub(super) fn or_default_network(
        &self,
        network: Option<String>,
    ) -> GlobalStateManagerResult<String> {
        match network {
            Some(x) => Ok(x),
            None => self
                .default_network
                .clone()
                .ok_or(GlobalStateManagerError::NoDefaultNetwork),
        }
    }
    pub(super) fn or_default_model(
        &self,
        model: Option<String>,
    ) -> GlobalStateManagerResult<String> {
        match model {
            Some(x) => Ok(x),
            None => self
                .default_model
                .clone()
                .ok_or(GlobalStateManagerError::NoDefaultModel),
        }
    }
    pub(super) fn or_default_pool(&self, pool: Option<String>) -> GlobalStateManagerResult<String> {
        match pool {
            Some(x) => Ok(x),
            None => self
                .default_pool
                .clone()
                .ok_or(GlobalStateManagerError::NoDefaultPool),
        }
    }

    pub(super) fn models(&self) -> &imbl::HashMap<String, config::Model> {
        &self.models
    }
    pub(super) fn allocations(&self) -> &imbl::HashMap<String, Arc<Allocation>> {
        &self.allocations
    }

    pub(super) fn profile_states(&self) -> &StateRegistry<ProfileState> {
        &self.profile_state_registry
    }
    pub(super) fn machine_states(&self) -> &StateRegistry<MachineState> {
        &self.machine_state_registry
    }
    pub(super) fn template_states(&self) -> &StateRegistry<TemplateState> {
        &self.template_state_registry
    }
    pub(super) fn network_states(&self) -> &StateRegistry<NetworkState> {
        &self.network_state_registry
    }
    pub(super) fn blueprint_states(&self) -> &StateRegistry<BlueprintState> {
        &self.blueprint_state_registry
    }

    pub(super) fn profile_states_mut(&mut self) -> &mut StateRegistry<ProfileState> {
        &mut self.profile_state_registry
    }
    pub(super) fn machine_states_mut(&mut self) -> &mut StateRegistry<MachineState> {
        &mut self.machine_state_registry
    }
    pub(super) fn template_states_mut(&mut self) -> &mut StateRegistry<TemplateState> {
        &mut self.template_state_registry
    }
    pub(super) fn network_states_mut(&mut self) -> &mut StateRegistry<NetworkState> {
        &mut self.network_state_registry
    }
    pub(super) fn blueprint_states_mut(&mut self) -> &mut StateRegistry<BlueprintState> {
        &mut self.blueprint_state_registry
    }

    #[instrument(level = "debug", skip_all, err)]
    fn execute_config_allocations(
        &mut self,
        config_allocations: &HashMap<String, config::Allocation>,
    ) -> GlobalStateManagerResult<()> {
        for (name, allocation_config) in config_allocations {
            if self.allocations.contains_key(name) {
                return Err(GlobalStateManagerError::DuplicateName(name.clone()));
            }
            let address_pool = self.resolve_address_pool(name.clone(), config_allocations)?;

            let allocation = Arc::new(Allocation {
                config: allocation_config.clone(),
                address_pool,
            });
            debug!("Added allocation: {}: {:?}", name, allocation);
            self.allocations.insert(name.clone(), allocation);
        }
        Ok(())
    }
    #[instrument(level = "debug", skip(self, model), err)]
    fn execute_config_model(
        &mut self,
        name: &str,
        model: &config::Model,
    ) -> GlobalStateManagerResult<()> {
        if self.models.contains_key(name) {
            return Err(GlobalStateManagerError::DuplicateName(name.to_owned()));
        }
        debug!("Added model: {}: {:?}", name, model);
        self.models.insert(name.to_owned(), model.to_owned());
        Ok(())
    }

    #[instrument(level = "debug", skip(self, profile), err)]
    fn execute_config_profile(
        &mut self,
        name: &str,
        profile: &config::Profile,
    ) -> GlobalStateManagerResult<()> {
        if self
            .profile_state_registry
            .get_state_id_by_name(name)
            .is_some()
        {
            return Err(GlobalStateManagerError::DuplicateName(name.to_owned()));
        }

        let id = self.profile_state_registry.allocate_id();
        let state = ProfileState::new(id, name.to_owned(), profile.clone());
        self.profile_state_registry
            .attach_state(state)
            .expect("must attach");

        debug!("Added profile: {}: {:?}", name, profile);

        Ok(())
    }

    #[instrument(level = "debug", skip(self, network), err)]
    fn execute_config_network(
        &mut self,
        name: &str,
        network: &config::Network,
    ) -> GlobalStateManagerResult<()> {
        if self
            .network_state_registry
            .get_state_id_by_name(name)
            .is_some()
        {
            return Err(GlobalStateManagerError::DuplicateName(name.to_owned()));
        }

        let id = self.network_state_registry.allocate_id();
        let state = {
            let mut network_state =
                NetworkState::new(id, Some(name.to_owned()), NetworkOrigin::Direct);

            // Set model
            let model_name = self.or_default_model(network.model.to_owned())?;
            let model = self
                .models
                .get(&model_name)
                .ok_or(GlobalStateManagerError::ModelNotFound(model_name))?;
            network_state.set_model(NetworkStateModelParams {
                latency: model.latency.clone(),
                distance: model.distance.clone(),
                loss: model.loss,
            });

            Ok(network_state)
        }
        .inspect_err(|_| {
            self.network_state_registry
                .release_id(id)
                .expect("must release");
        })?;
        self.network_state_registry
            .attach_state(state)
            .expect("must attach");

        debug!("Added network: {}: {:?}", name, network);
        Ok(())
    }

    #[instrument(level = "debug", skip(self, ipv4), err)]
    fn execute_config_network_ipv4(
        &mut self,
        name: &str,
        ipv4: &config::NetworkIpv4,
    ) -> GlobalStateManagerResult<()> {
        let network_state_id = self
            .network_state_registry
            .get_state_id_by_name(name)
            .expect("must exist");
        let mut network_state = self
            .network_state_registry
            .get_state(network_state_id)
            .expect("must exist");

        // Get IPV4 allocation
        let address_pool = &self
            .allocations
            .get(&ipv4.allocation)
            .cloned()
            .ok_or_else(|| GlobalStateManagerError::AllocationNotFound(ipv4.allocation.clone()))?
            .address_pool;
        let scope = address_pool.scopes_v4();
        let reserve = address_pool.allocations_v4();

        // Set IPV4 config
        network_state.set_ipv4(
            self,
            NetworkStateIpv4Params {
                scope,
                reserve,
                super_net: None,
            },
        )?;

        // Update state
        self.network_state_registry.set_state(network_state);

        Ok(())
    }

    #[instrument(level = "debug", skip(self, ipv6), err)]
    fn execute_config_network_ipv6(
        &mut self,
        name: &str,
        ipv6: &config::NetworkIpv6,
    ) -> GlobalStateManagerResult<()> {
        let network_state_id = self
            .network_state_registry
            .get_state_id_by_name(name)
            .expect("must exist");
        let mut network_state = self
            .network_state_registry
            .get_state(network_state_id)
            .expect("must exist");

        // Get IPV4 allocation
        let address_pool = &self
            .allocations
            .get(&ipv6.allocation)
            .cloned()
            .ok_or_else(|| GlobalStateManagerError::AllocationNotFound(ipv6.allocation.clone()))?
            .address_pool;
        let scope = address_pool.scopes_v6();
        let reserve = address_pool.allocations_v6();

        // Set IPV4 config
        network_state.set_ipv6(
            self,
            NetworkStateIpv6Params {
                scope,
                reserve,
                super_net: None,
            },
        )?;

        // Update state
        self.network_state_registry.set_state(network_state);

        Ok(())
    }

    #[instrument(level = "debug", skip(self, ipv4gw), err)]
    fn execute_config_network_ipv4_gateway(
        &mut self,
        name: &str,
        ipv4gw: &config::NetworkGateway,
    ) -> GlobalStateManagerResult<()> {
        let network_state_id = self
            .network_state_registry
            .get_state_id_by_name(name)
            .expect("must exist");
        let mut network_state = self
            .network_state_registry
            .get_state(network_state_id)
            .expect("must exist");

        let translation = ipv4gw.translation;
        let upnp = ipv4gw.upnp;
        let external_network_name = self.or_default_network(ipv4gw.network.clone())?;
        let external_network = self
            .network_state_registry
            .get_state_id_by_name(&external_network_name)
            .ok_or(GlobalStateManagerError::NetworkNotFound(
                external_network_name,
            ))?;

        let gateway_params = NetworkStateIpv4GatewayParams {
            translation,
            upnp,
            external_network,
            internal_address: None,
            external_address: None,
        };

        network_state.set_ipv4_gateway(self, gateway_params)?;

        // Update state
        self.network_state_registry.set_state(network_state);

        Ok(())
    }

    #[instrument(level = "debug", skip(self, ipv6gw), err)]
    fn execute_config_network_ipv6_gateway(
        &mut self,
        name: &str,
        ipv6gw: &config::NetworkGateway,
    ) -> GlobalStateManagerResult<()> {
        let network_state_id = self
            .network_state_registry
            .get_state_id_by_name(name)
            .expect("must exist");
        let mut network_state = self
            .network_state_registry
            .get_state(network_state_id)
            .expect("must exist");

        let translation = ipv6gw.translation;
        let upnp = ipv6gw.upnp;
        let external_network_name = self.or_default_network(ipv6gw.network.clone())?;
        let external_network = self
            .network_state_registry
            .get_state_id_by_name(&external_network_name)
            .ok_or(GlobalStateManagerError::NetworkNotFound(
                external_network_name,
            ))?;

        let gateway_params = NetworkStateIpv4GatewayParams {
            translation,
            upnp,
            external_network,
            internal_address: None,
            external_address: None,
        };

        network_state.set_ipv4_gateway(self, gateway_params)?;

        // Update state
        self.network_state_registry.set_state(network_state);

        Ok(())
    }

    #[instrument(level = "debug", skip(self, blueprint), err)]
    fn execute_config_blueprint(
        &mut self,
        name: &str,
        blueprint: &config::Blueprint,
    ) -> GlobalStateManagerResult<()> {
        if self
            .blueprint_state_registry
            .get_state_id_by_name(name)
            .is_some()
        {
            return Err(GlobalStateManagerError::DuplicateName(name.to_owned()));
        }

        let id = self.blueprint_state_registry.allocate_id();
        let state = {
            let mut blueprint_state = BlueprintState::new(id, name.to_owned());

            // Set model
            let model = match blueprint.model.to_owned() {
                Some(x) => x,
                None => WeightedList::Single(
                    self.default_model
                        .clone()
                        .ok_or(GlobalStateManagerError::NoDefaultModel)?,
                ),
            };
            blueprint_state.set_model(model);
            blueprint_state.set_limit_network_count(
                blueprint
                    .limits
                    .to_owned()
                    .network_count
                    .map(|wl| self.srng().weighted_choice(wl)),
            );

            Ok(blueprint_state)
        }
        .inspect_err(|_| {
            self.blueprint_state_registry
                .release_id(id)
                .expect("must release");
        })?;
        self.blueprint_state_registry
            .attach_state(state)
            .expect("must attach");

        debug!("Added blueprint: {}: {:?}", name, blueprint);

        Ok(())
    }

    #[instrument(level = "debug", skip(self, ipv4), err)]
    fn execute_config_blueprint_ipv4(
        &mut self,
        name: &str,
        ipv4: &config::BlueprintIpv4,
    ) -> GlobalStateManagerResult<()> {
        let blueprint_state_id = self
            .blueprint_state_registry
            .get_state_id_by_name(name)
            .expect("must exist");
        let mut blueprint_state = self
            .blueprint_state_registry
            .get_state(blueprint_state_id)
            .expect("must exist");

        let locations = match ipv4.location.clone() {
            config::BlueprintLocation::Allocation { allocation } => {
                BlueprintLocationsList::Allocations {
                    allocations: allocation,
                }
            }
            config::BlueprintLocation::Network { network } => {
                if let Some(network) = network {
                    let networks = network.try_map(|n| {
                        self.network_state_registry
                            .get_state_id_by_name(n)
                            .ok_or_else(|| GlobalStateManagerError::NetworkNotFound(n.clone()))
                    })?;
                    BlueprintLocationsList::Networks { networks }
                } else {
                    let default_network = self.or_default_network(None)?;
                    let default_network_state_id = self
                        .network_state_registry
                        .get_state_id_by_name(&default_network)
                        .ok_or(GlobalStateManagerError::NetworkNotFound(default_network))?;

                    BlueprintLocationsList::Networks {
                        networks: WeightedList::Single(default_network_state_id),
                    }
                }
            }
        };

        let prefix = ipv4.prefix.clone();

        // Set IPV4 config
        blueprint_state.set_ipv4(
            self,
            BlueprintStateIpv4Params {
                locations,
                prefix,
                gateway: None,
            },
        )?;

        // Update state
        self.blueprint_state_registry.set_state(blueprint_state);

        Ok(())
    }

    #[instrument(level = "debug", skip(self, ipv4gw), err)]
    fn execute_config_blueprint_ipv4_gateway(
        &mut self,
        name: &str,
        ipv4gw: &config::BlueprintGateway,
    ) -> GlobalStateManagerResult<()> {
        let blueprint_state_id = self
            .blueprint_state_registry
            .get_state_id_by_name(name)
            .expect("must exist");
        let mut blueprint_state = self
            .blueprint_state_registry
            .get_state(blueprint_state_id)
            .expect("must exist");

        let translation = ipv4gw.translation.clone();
        let upnp = ipv4gw.upnp;
        let locations = match ipv4gw.location.clone() {
            Some(config::TemplateLocation::Network { network }) => {
                let networks = network.try_map(|n| {
                    self.network_state_registry
                        .get_state_id_by_name(n)
                        .ok_or_else(|| GlobalStateManagerError::NetworkNotFound(n.clone()))
                })?;
                Some(TemplateLocationsList::Networks { networks })
            }
            Some(config::TemplateLocation::Blueprint { blueprint }) => {
                let blueprints = blueprint.try_map(|n| {
                    self.blueprint_state_registry
                        .get_state_id_by_name(n)
                        .ok_or_else(|| GlobalStateManagerError::BlueprintNotFound(n.clone()))
                })?;
                Some(TemplateLocationsList::Blueprints { blueprints })
            }
            None => None,
        };

        let gateway_params = BlueprintStateGatewayParams {
            translation,
            upnp,
            locations,
        };

        blueprint_state.set_ipv4_gateway(self, Some(gateway_params))?;

        // Update state
        self.blueprint_state_registry.set_state(blueprint_state);

        Ok(())
    }

    #[instrument(level = "debug", skip(self, ipv6), err)]
    fn execute_config_blueprint_ipv6(
        &mut self,
        name: &str,
        ipv6: &config::BlueprintIpv6,
    ) -> GlobalStateManagerResult<()> {
        let blueprint_state_id = self
            .blueprint_state_registry
            .get_state_id_by_name(name)
            .expect("must exist");
        let mut blueprint_state = self
            .blueprint_state_registry
            .get_state(blueprint_state_id)
            .expect("must exist");

        let locations = match ipv6.location.clone() {
            config::BlueprintLocation::Allocation { allocation } => {
                BlueprintLocationsList::Allocations {
                    allocations: allocation,
                }
            }
            config::BlueprintLocation::Network { network } => {
                if let Some(network) = network {
                    let networks = network.try_map(|n| {
                        self.network_state_registry
                            .get_state_id_by_name(n)
                            .ok_or_else(|| GlobalStateManagerError::NetworkNotFound(n.clone()))
                    })?;
                    BlueprintLocationsList::Networks { networks }
                } else {
                    let default_network = self.or_default_network(None)?;
                    let default_network_state_id = self
                        .network_state_registry
                        .get_state_id_by_name(&default_network)
                        .ok_or(GlobalStateManagerError::NetworkNotFound(default_network))?;

                    BlueprintLocationsList::Networks {
                        networks: WeightedList::Single(default_network_state_id),
                    }
                }
            }
        };

        let prefix = ipv6.prefix.clone();

        // Set IPV6 config
        blueprint_state.set_ipv6(
            self,
            BlueprintStateIpv6Params {
                locations,
                prefix,
                gateway: None,
            },
        )?;

        // Update state
        self.blueprint_state_registry.set_state(blueprint_state);

        Ok(())
    }

    #[instrument(level = "debug", skip(self, ipv6gw), err)]
    fn execute_config_blueprint_ipv6_gateway(
        &mut self,
        name: &str,
        ipv6gw: &config::BlueprintGateway,
    ) -> GlobalStateManagerResult<()> {
        let blueprint_state_id = self
            .blueprint_state_registry
            .get_state_id_by_name(name)
            .expect("must exist");
        let mut blueprint_state = self
            .blueprint_state_registry
            .get_state(blueprint_state_id)
            .expect("must exist");

        let translation = ipv6gw.translation.clone();
        let upnp = ipv6gw.upnp;
        let locations = match ipv6gw.location.clone() {
            Some(config::TemplateLocation::Network { network }) => {
                let networks = network.try_map(|n| {
                    self.network_state_registry
                        .get_state_id_by_name(n)
                        .ok_or_else(|| GlobalStateManagerError::NetworkNotFound(n.clone()))
                })?;
                Some(TemplateLocationsList::Networks { networks })
            }
            Some(config::TemplateLocation::Blueprint { blueprint }) => {
                let blueprints = blueprint.try_map(|n| {
                    self.blueprint_state_registry
                        .get_state_id_by_name(n)
                        .ok_or_else(|| GlobalStateManagerError::BlueprintNotFound(n.clone()))
                })?;
                Some(TemplateLocationsList::Blueprints { blueprints })
            }
            None => None,
        };

        let gateway_params = BlueprintStateGatewayParams {
            translation,
            upnp,
            locations,
        };

        blueprint_state.set_ipv6_gateway(self, Some(gateway_params))?;

        // Update state
        self.blueprint_state_registry.set_state(blueprint_state);

        Ok(())
    }

    #[instrument(level = "debug", skip(self, template), err)]
    fn execute_config_template(
        &mut self,
        name: &str,
        template: &config::Template,
    ) -> GlobalStateManagerResult<()> {
        if self
            .template_state_registry
            .get_state_id_by_name(name)
            .is_some()
        {
            return Err(GlobalStateManagerError::DuplicateName(name.to_owned()));
        }

        let id = self.template_state_registry.allocate_id();
        let state = {
            let mut template_state = TemplateState::new(id, name.to_owned());

            template_state.set_disable_capabilities(template.disable_capabilities.to_owned());
            if let Some(wl) = template.limits.to_owned().machine_count {
                template_state.set_limit_machine_count(Some(self.srng().weighted_choice(wl)));
            }
            template_state
                .set_limit_machines_per_network(template.limits.machines_per_network.clone());

            match template.location.clone() {
                config::TemplateLocation::Network { network } => {
                    let networks = network.try_map(|x| {
                        self.network_state_registry
                            .get_state_id_by_name(x)
                            .ok_or_else(|| GlobalStateManagerError::NetworkNotFound(x.clone()))
                    })?;

                    template_state.set_networks_list(networks);
                }
                config::TemplateLocation::Blueprint { blueprint } => {
                    let blueprints = blueprint.try_map(|x| {
                        self.blueprint_state_registry
                            .get_state_id_by_name(x)
                            .ok_or_else(|| GlobalStateManagerError::BlueprintNotFound(x.clone()))
                    })?;

                    template_state.set_blueprints_list(blueprints);
                }
            }

            Ok(template_state)
        }
        .inspect_err(|_| {
            self.template_state_registry
                .release_id(id)
                .expect("must release");
        })?;
        self.template_state_registry
            .attach_state(state)
            .expect("must attach");

        debug!("Added template: {}: {:?}", name, template);
        Ok(())
    }

    #[instrument(level = "debug", skip(self, machine), err)]
    fn execute_config_machine(
        &mut self,
        name: &str,
        machine: &config::Machine,
    ) -> GlobalStateManagerResult<()> {
        if self
            .machine_state_registry
            .get_state_id_by_name(name)
            .is_some()
        {
            return Err(GlobalStateManagerError::DuplicateName(name.to_owned()));
        }

        let id = self.machine_state_registry.allocate_id();
        let state = {
            let mut machine_state =
                MachineState::new(id, Some(name.to_owned()), MachineOrigin::Config);

            machine_state.set_disable_capabilities(machine.disable_capabilities.to_owned());
            machine_state.set_bootstrap(machine.bootstrap);

            // Create primary interface
            let interface_name = machine_state.allocate_interface(None, None)?;

            match machine.location.to_owned() {
                config::MachineLocation::Network {
                    network,
                    address4,
                    address6,
                } => {
                    // Look up network
                    let network_state_id = self
                        .network_state_registry
                        .get_state_id_by_name(&network)
                        .ok_or(GlobalStateManagerError::NetworkNotFound(network))?;

                    machine_state.attach_network(self, &interface_name, network_state_id)?;
                    if let Some(address4) = address4 {
                        machine_state.allocate_address_ipv4(
                            self,
                            &interface_name,
                            Some(address4),
                            None,
                        )?;
                    }
                    if let Some(address6) = address6 {
                        machine_state.allocate_address_ipv6(
                            self,
                            &interface_name,
                            Some(address6),
                            None,
                        )?;
                    }
                }
            }

            Ok(machine_state)
        }
        .inspect_err(|_| {
            self.machine_state_registry
                .release_id(id)
                .expect("must release");
        })?;
        self.machine_state_registry
            .attach_state(state)
            .expect("must attach");
        debug!("Added machine: {}: {:?}", name, machine);
        Ok(())
    }

    #[instrument(level = "debug", skip(self, config_allocations), err)]
    fn resolve_address_pool(
        &self,
        allocation_name: String,
        config_allocations: &HashMap<String, config::Allocation>,
    ) -> GlobalStateManagerResult<AddressPool<()>> {
        // Get the allocation config
        let allocation = config_allocations
            .get(&allocation_name)
            .ok_or_else(|| GlobalStateManagerError::AllocationNotFound(allocation_name.clone()))?;

        // Create an address pool
        let mut address_pool = AddressPool::<()>::new();

        // Apply the scope present in the allocation
        if let Some(scope4) = allocation.scope4.as_ref() {
            for s in &scope4.scope4 {
                address_pool.add_scope_v4(*s);
            }
        }
        if let Some(scope6) = allocation.scope6.as_ref() {
            for s in &scope6.scope6 {
                address_pool.add_scope_v6(*s);
            }
        }

        // Reserve out any allocations that used this as their pool
        let mut scope4_allocs: Vec<Ipv4Net> = Vec::new();
        let mut scope6_allocs: Vec<Ipv6Net> = Vec::new();

        for (k, v) in config_allocations {
            // Exclude our own allocation
            if *k == allocation_name {
                continue;
            }
            if let Some(scope4) = v.scope4.as_ref() {
                let pool = self.or_default_pool(scope4.pool4.clone())?;
                if pool == allocation_name {
                    for s in &scope4.scope4 {
                        scope4_allocs.push(*s);
                        scope4_allocs = Ipv4Net::aggregate(&scope4_allocs);
                    }
                }
            }
            if let Some(scope6) = v.scope6.as_ref() {
                let pool = self.or_default_pool(scope6.pool6.clone())?;
                if pool == allocation_name {
                    for s in &scope6.scope6 {
                        scope6_allocs.push(*s);
                        scope6_allocs = Ipv6Net::aggregate(&scope6_allocs);
                    }
                }
            }
        }

        for s in scope4_allocs {
            address_pool.reserve_allocation_v4(s, None)?;
        }
        for s in scope6_allocs {
            address_pool.reserve_allocation_v6(s, None)?;
        }

        Ok(address_pool)
    }
}
