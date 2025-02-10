use super::*;

#[derive(Debug)]
struct TemplateStateImmutable {
    /// The unique id of this template
    id: TemplateStateId,
    /// The name of this template state
    name: String,
}

#[derive(Debug, Clone)]
struct PerNetworkInfo {
    limit_machine_count: Option<usize>,
    machines: imbl::HashSet<MachineStateId>,
}

#[derive(Debug, Clone)]
struct TemplateStateFields {
    limit_machine_count: Option<usize>,
    limit_machines_per_network: Option<WeightedList<usize>>,
    locations_list: Option<TemplateLocationsList>,
    machines: imbl::HashSet<MachineStateId>,
    machines_per_network: imbl::HashMap<NetworkStateId, PerNetworkInfo>,
    disable_capabilities: imbl::Vector<Arc<String>>,
}

#[derive(Debug, Clone)]
pub struct TemplateState {
    immutable: Arc<TemplateStateImmutable>,
    fields: Arc<TemplateStateFields>,
}

pub type TemplateStateId = StateId<TemplateState>;

impl TemplateState {
    pub fn new(id: TemplateStateId, name: String) -> Self {
        Self {
            immutable: Arc::new(TemplateStateImmutable { id, name }),
            fields: Arc::new(TemplateStateFields {
                limit_machine_count: None,
                limit_machines_per_network: None,
                locations_list: None,
                machines: imbl::HashSet::new(),
                machines_per_network: imbl::HashMap::new(),
                disable_capabilities: imbl::Vector::new(),
            }),
        }
    }

    #[instrument(level = "debug", skip(self))]
    pub fn set_disable_capabilities(&mut self, disable_capabilities: Vec<String>) {
        let disable_capabilities =
            imbl::Vector::from_iter(disable_capabilities.into_iter().map(Arc::new));
        // Update fields
        self.fields = Arc::new(TemplateStateFields {
            disable_capabilities,
            ..(*self.fields).clone()
        });
    }

    #[instrument(level = "debug", skip(self))]
    pub fn set_networks_list(&mut self, networks: WeightedList<NetworkStateId>) {
        let locations_list = Some(TemplateLocationsList::Networks { networks });

        // Update fields
        self.fields = Arc::new(TemplateStateFields {
            locations_list,
            ..(*self.fields).clone()
        });
    }

    #[instrument(level = "debug", skip(self))]
    pub fn set_blueprints_list(&mut self, blueprints: WeightedList<BlueprintStateId>) {
        let locations_list = Some(TemplateLocationsList::Blueprints { blueprints });

        // Update fields
        self.fields = Arc::new(TemplateStateFields {
            locations_list,
            ..(*self.fields).clone()
        });
    }

    #[instrument(level = "debug", skip(self))]
    pub fn clear_locations_list(&mut self) {
        let locations_list = None;

        // Update fields
        self.fields = Arc::new(TemplateStateFields {
            locations_list,
            ..(*self.fields).clone()
        });
    }

    #[instrument(level = "debug", skip(self))]
    pub fn set_limit_machine_count(&mut self, limit_machine_count: Option<usize>) {
        // Update fields
        self.fields = Arc::new(TemplateStateFields {
            limit_machine_count,
            ..(*self.fields).clone()
        });
    }

    #[instrument(level = "debug", skip(self))]
    pub fn set_limit_machines_per_network(
        &mut self,
        limit_machines_per_network: Option<WeightedList<usize>>,
    ) {
        // Update fields
        self.fields = Arc::new(TemplateStateFields {
            limit_machines_per_network,
            ..(*self.fields).clone()
        });
    }

    pub fn is_active(&self, gsm_inner: &mut GlobalStateManagerInner) -> bool {
        // Save a backup of the entire state
        let backup = gsm_inner.clone();

        // Make a copy of this template state
        let mut current_state = self.clone();

        // See what would happen if we try to generate this template
        let ok = current_state.generate(gsm_inner).is_ok();

        // Restore the backup
        *gsm_inner = backup;

        // Return if this worked or not
        ok
    }

    /// Network filter that keeps this template generation within per-network limits
    fn network_filter(&self, network_state_id: NetworkStateId) -> GlobalStateManagerResult<bool> {
        // Get the per network info
        let Some(pni) = self.fields.machines_per_network.get(&network_state_id) else {
            // If we haven't allocated anything in the network yet it is
            // by definition available
            return Ok(true);
        };

        // If this template has allocated the maximum number of machines per-network
        // for this network, then it is not available
        if let Some(limit_machine_count) = pni.limit_machine_count {
            if pni.machines.len() >= limit_machine_count {
                return Ok(false);
            }
        }
        Ok(true)
    }

    #[instrument(level = "debug", skip(self, gsm_inner), err)]
    pub fn generate(
        &mut self,
        gsm_inner: &mut GlobalStateManagerInner,
    ) -> GlobalStateManagerResult<MachineStateId> {
        // See if we have reached our machine limit
        if let Some(limit_machine_count) = self.fields.limit_machine_count {
            if self.fields.machines.len() < limit_machine_count {
                return Err(GlobalStateManagerError::TemplateComplete(self.debug_name()));
            }
        }

        // If existing networks are all full, we'd have to allocate one, see if we'd be able to do that
        let Some(locations_list) = self.fields.locations_list.as_ref() else {
            return Err(GlobalStateManagerError::TemplateComplete(self.debug_name()));
        };

        // Get a network to generate the machine on
        let Some(network_state) = locations_list.pick(gsm_inner, |_, x| self.network_filter(x))?
        else {
            return Err(GlobalStateManagerError::TemplateComplete(self.debug_name()));
        };

        // Allocate a machine id
        let machine_state_id = gsm_inner.machine_states_mut().allocate_id();

        // Create an anonymous machine state
        let mut machine_state =
            MachineState::new(machine_state_id, None, MachineOrigin::Template(self.id()));

        // Scope to release state on error
        if let Err(e) = (|| {
            // Build out the machine state from the template
            machine_state.set_disable_capabilities(
                self.fields
                    .disable_capabilities
                    .iter()
                    .map(|x| (**x).clone())
                    .collect(),
            );
            machine_state.set_bootstrap(false);

            // Make the default route interface
            let vin0 = machine_state.allocate_interface(None, None)?;
            machine_state.attach_network(gsm_inner, &vin0, network_state.id())?;
            if network_state.is_ipv4() {
                machine_state.allocate_address_ipv4(gsm_inner, &vin0, None, None)?;
            }
            if network_state.is_ipv6() {
                machine_state.allocate_address_ipv6(gsm_inner, &vin0, None, None)?;
            }
            Ok(())
        })() {
            // Release the machine state and id if things failed to allocate
            machine_state.release(gsm_inner);
            gsm_inner
                .machine_states_mut()
                .release_id(machine_state_id)
                .expect("must succeed");
            return Err(e);
        }

        // Attach the state to the id
        gsm_inner
            .machine_states_mut()
            .attach_state(machine_state)
            .expect("must succeed");

        // Record the newly instantiated machine
        let machines = self.fields.machines.update(machine_state_id);
        let mut machines_per_network = self.fields.machines_per_network.clone();
        let per_network_info = machines_per_network
            .entry(network_state.id())
            .or_insert_with(|| {
                let limit_machine_count = self
                    .fields
                    .limit_machines_per_network
                    .as_ref()
                    .map(|wl| *gsm_inner.srng().weighted_choice_ref(wl));
                PerNetworkInfo {
                    limit_machine_count,
                    machines: imbl::HashSet::new(),
                }
            });
        per_network_info.machines.insert(machine_state_id);

        // Update fields
        self.fields = Arc::new(TemplateStateFields {
            machines,
            machines_per_network,
            ..(*self.fields).clone()
        });

        Ok(machine_state_id)
    }

    #[instrument(level = "debug", skip(self))]
    pub fn on_machine_released(&mut self, machine_state_id: MachineStateId) {
        let machines = self.fields.machines.without(&machine_state_id);
        let mut machines_per_network = self.fields.machines_per_network.clone();
        for (_network_id, pni) in machines_per_network.iter_mut() {
            pni.machines.remove(&machine_state_id);
        }

        // Update fields
        self.fields = Arc::new(TemplateStateFields {
            machines,
            machines_per_network,
            ..(*self.fields).clone()
        });
    }
}

impl State for TemplateState {
    fn id(&self) -> StateId<Self> {
        self.immutable.id
    }

    fn name(&self) -> Option<String> {
        Some(self.immutable.name.clone())
    }
}
