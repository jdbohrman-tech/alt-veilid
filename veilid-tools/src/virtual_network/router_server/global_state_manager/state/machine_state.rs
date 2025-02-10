use super::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum MachineOrigin {
    Config,
    Direct,
    Template(TemplateStateId),
}

#[derive(Debug, Clone)]
struct MachineStateFields {
    /// The current network interfaces definition
    interfaces: imbl::HashMap<Arc<String>, MachineStateInterface>,
    /// Capabilities to disable on this machine
    disable_capabilities: imbl::Vector<String>,
    /// If this machine is a bootstrap
    bootstrap: bool,
}

#[derive(Debug, Clone)]
pub struct MachineStateInterface {
    /// The network this interface belongs to
    pub network_id: Option<NetworkStateId>,
    /// The veilid NetworkInterface state
    pub network_interface: Arc<NetworkInterface>,
}

#[derive(Debug)]
struct MachineStateImmutable {
    /// The id of this machine
    id: MachineStateId,
    /// The name of this machine if it is named
    opt_name: Option<String>,
    /// Where this machine came for housekeeping purposes
    origin: MachineOrigin,
}

#[derive(Debug, Clone)]
pub struct MachineState {
    immutable: Arc<MachineStateImmutable>,
    fields: Arc<MachineStateFields>,
}

pub type MachineStateId = StateId<MachineState>;

impl MachineState {
    pub fn new(id: MachineStateId, opt_name: Option<String>, origin: MachineOrigin) -> Self {
        // Create a localhost interface for this machine
        Self {
            immutable: Arc::new(MachineStateImmutable {
                id,
                opt_name,
                origin,
            }),
            fields: Arc::new(MachineStateFields {
                interfaces: imbl::HashMap::new(),
                disable_capabilities: imbl::Vector::new(),
                bootstrap: false,
            }),
        }
    }

    #[instrument(level = "debug", skip(self, gsm_inner))]
    pub fn release(mut self, gsm_inner: &mut GlobalStateManagerInner) {
        self.release_all_interfaces(gsm_inner)
            .expect("must succeed");

        if let MachineOrigin::Template(generating_template) = self.immutable.origin {
            let mut template_state = gsm_inner
                .template_states()
                .get_state(generating_template)
                .expect("must exist");
            template_state.on_machine_released(self.id());
            gsm_inner.template_states_mut().set_state(template_state);
        }
    }

    #[instrument(level = "debug", skip(self))]
    pub fn set_disable_capabilities(&mut self, disable_capabilities: Vec<String>) {
        self.fields = Arc::new(MachineStateFields {
            disable_capabilities: disable_capabilities.into(),
            ..(*self.fields).clone()
        });
    }

    #[instrument(level = "debug", skip(self))]
    pub fn set_bootstrap(&mut self, bootstrap: bool) {
        self.fields = Arc::new(MachineStateFields {
            bootstrap,
            ..(*self.fields).clone()
        });
    }

    fn next_free_interface_key(&self) -> Arc<String> {
        let mut inum = 0usize;
        loop {
            let name = format!("vin{}", inum);
            if !self.fields.interfaces.contains_key(&name) {
                return Arc::new(name);
            }
            inum += 1;
        }
    }

    #[instrument(level = "debug", skip(self), err)]
    pub fn allocate_interface(
        &mut self,
        interface_name: Option<String>,
        opt_interface_flags: Option<InterfaceFlags>,
    ) -> GlobalStateManagerResult<Arc<String>> {
        let interface_key = interface_name
            .map(Arc::new)
            .unwrap_or_else(|| self.next_free_interface_key());
        if self.fields.interfaces.contains_key(&interface_key) {
            return Err(GlobalStateManagerError::DuplicateName(
                (*interface_key).clone(),
            ));
        }
        let flags = opt_interface_flags.unwrap_or(InterfaceFlags {
            is_loopback: false,
            is_running: true,
            is_point_to_point: false,
            has_default_route: true,
        });
        let interfaces = self.fields.interfaces.update(
            interface_key.clone(),
            MachineStateInterface {
                network_id: None,
                network_interface: Arc::new(NetworkInterface {
                    name: (*interface_key).clone(),
                    flags,
                    addrs: Vec::new(),
                }),
            },
        );

        self.fields = Arc::new(MachineStateFields {
            interfaces,
            ..(*self.fields).clone()
        });

        Ok(interface_key)
    }

    pub fn interfaces(&self) -> Vec<Arc<String>> {
        let mut intfs: Vec<_> = self.fields.interfaces.keys().cloned().collect();
        intfs.sort();
        intfs
    }

    #[instrument(level = "debug", skip(self, gsm_inner), err)]
    pub fn allocate_address_ipv4(
        &mut self,
        gsm_inner: &mut GlobalStateManagerInner,
        interface_name: &str,
        opt_address: Option<Ipv4Addr>,
        opt_address_flags: Option<AddressFlags>,
    ) -> GlobalStateManagerResult<Ifv4Addr> {
        let interface_key = Arc::new(interface_name.to_string());
        let Some(mut machine_state_interface) = self.fields.interfaces.get(&interface_key).cloned()
        else {
            return Err(GlobalStateManagerError::InvalidName(
                (*interface_key).clone(),
            ));
        };

        // Get the network state
        let Some(network_id) = machine_state_interface.network_id else {
            return Err(GlobalStateManagerError::NetworkNotFound(
                (*interface_key).clone(),
            ));
        };
        let mut network_state = gsm_inner.network_states().get_state(network_id)?;

        // Allocate interface address
        let is_dynamic = opt_address.is_none();
        let ifv4_addr = network_state.allocate_address_v4(
            gsm_inner,
            OwnerTag::Machine(self.id()),
            opt_address,
        )?;

        // Update the network state
        gsm_inner.network_states_mut().set_state(network_state);

        // Get address flags
        let flags = opt_address_flags.unwrap_or(AddressFlags {
            is_dynamic,
            is_temporary: false,
            is_preferred: true,
        });

        // Update interface addresses
        let mut new_intf = (*machine_state_interface.network_interface).clone();
        new_intf.addrs.push(InterfaceAddress {
            if_addr: IfAddr::V4(ifv4_addr.clone()),
            flags,
        });

        // Update interface
        machine_state_interface.network_interface = Arc::new(new_intf);

        // Update interfaces map
        let interfaces = self
            .fields
            .interfaces
            .update(interface_key, machine_state_interface);

        // Update fields
        self.fields = Arc::new(MachineStateFields {
            interfaces,
            ..(*self.fields).clone()
        });

        Ok(ifv4_addr)
    }

    #[instrument(level = "debug", skip(self, gsm_inner), err)]
    pub fn allocate_address_ipv6(
        &mut self,
        gsm_inner: &mut GlobalStateManagerInner,
        interface_name: &str,
        opt_address: Option<Ipv6Addr>,
        opt_address_flags: Option<AddressFlags>,
    ) -> GlobalStateManagerResult<Ifv6Addr> {
        let interface_key = Arc::new(interface_name.to_string());
        let Some(mut machine_state_interface) = self.fields.interfaces.get(&interface_key).cloned()
        else {
            return Err(GlobalStateManagerError::InvalidName(
                (*interface_key).clone(),
            ));
        };

        // Get the network state
        let Some(network_id) = machine_state_interface.network_id else {
            return Err(GlobalStateManagerError::NetworkNotFound(
                (*interface_key).clone(),
            ));
        };
        let mut network_state = gsm_inner.network_states().get_state(network_id)?;

        // Allocate interface address
        let is_dynamic = opt_address.is_none();
        let ifv6_addr = network_state.allocate_address_v6(
            gsm_inner,
            OwnerTag::Machine(self.id()),
            opt_address,
        )?;
        // Update the network state
        gsm_inner.network_states_mut().set_state(network_state);

        // Get address flags
        let flags = opt_address_flags.unwrap_or(AddressFlags {
            is_dynamic,
            is_temporary: false,
            is_preferred: true,
        });

        // Update interface addresses
        let mut new_intf = (*machine_state_interface.network_interface).clone();
        new_intf.addrs.push(InterfaceAddress {
            if_addr: IfAddr::V6(ifv6_addr.clone()),
            flags,
        });

        // Update interface
        machine_state_interface.network_interface = Arc::new(new_intf);

        // Update interfaces map
        let interfaces = self
            .fields
            .interfaces
            .update(interface_key, machine_state_interface);

        // Update fields
        self.fields = Arc::new(MachineStateFields {
            interfaces,
            ..(*self.fields).clone()
        });

        Ok(ifv6_addr)
    }

    #[instrument(level = "debug", skip(self, gsm_inner), err)]
    pub fn attach_network(
        &mut self,
        gsm_inner: &mut GlobalStateManagerInner,
        interface_name: &str,
        network_id: NetworkStateId,
    ) -> GlobalStateManagerResult<()> {
        let interface_key = Arc::new(interface_name.to_string());
        let Some(mut machine_state_interface) = self.fields.interfaces.get(&interface_key).cloned()
        else {
            return Err(GlobalStateManagerError::InvalidName(
                (*interface_key).clone(),
            ));
        };

        if machine_state_interface.network_id.is_some() {
            Self::detach_network_inner(gsm_inner, &mut machine_state_interface)?;
        }

        machine_state_interface.network_id = Some(network_id);

        // Update interfaces map
        let interfaces = self
            .fields
            .interfaces
            .update(interface_key, machine_state_interface);

        // Update fields
        self.fields = Arc::new(MachineStateFields {
            interfaces,
            ..(*self.fields).clone()
        });

        Ok(())
    }

    #[instrument(level = "debug", skip(self, gsm_inner), err)]
    pub fn detach_network(
        &mut self,
        gsm_inner: &mut GlobalStateManagerInner,
        interface_name: &str,
    ) -> GlobalStateManagerResult<()> {
        let interface_key = Arc::new(interface_name.to_string());
        let Some(mut machine_state_interface) = self.fields.interfaces.get(&interface_key).cloned()
        else {
            return Err(GlobalStateManagerError::InvalidName(
                (*interface_key).clone(),
            ));
        };

        Self::detach_network_inner(gsm_inner, &mut machine_state_interface)?;

        // Update interfaces map
        let interfaces = self
            .fields
            .interfaces
            .update(interface_key, machine_state_interface);

        // Update fields
        self.fields = Arc::new(MachineStateFields {
            interfaces,
            ..(*self.fields).clone()
        });

        Ok(())
    }

    pub fn attached_network_interfaces(
        &self,
        network_id: NetworkStateId,
    ) -> GlobalStateManagerResult<Vec<Arc<String>>> {
        let mut out = Vec::new();
        for intf in &self.fields.interfaces {
            if intf.1.network_id == Some(network_id) {
                out.push(intf.0.clone());
            }
        }
        Ok(out)
    }

    #[instrument(level = "debug", skip(self, gsm_inner), err)]
    pub fn release_address(
        &mut self,
        gsm_inner: &mut GlobalStateManagerInner,
        interface_name: &str,
        address: IpAddr,
    ) -> GlobalStateManagerResult<()> {
        let interface_key = Arc::new(interface_name.to_owned());
        let Some(mut machine_state_interface) = self.fields.interfaces.get(&interface_key).cloned()
        else {
            return Err(GlobalStateManagerError::InvalidName(
                (*interface_key).clone(),
            ));
        };

        let Some(network_id) = machine_state_interface.network_id else {
            return Err(GlobalStateManagerError::NetworkNotFound(
                (*interface_key).clone(),
            ));
        };

        // Get the network state
        let mut network_state = gsm_inner.network_states().get_state(network_id)?;

        // Release the address from the network
        match address {
            IpAddr::V4(ipv4_addr) => network_state.release_address_v4(ipv4_addr)?,
            IpAddr::V6(ipv6_addr) => network_state.release_address_v6(ipv6_addr)?,
        };

        // Update the network state
        gsm_inner.network_states_mut().set_state(network_state);

        // Remove the address from the interface
        let addrs: Vec<_> = machine_state_interface
            .network_interface
            .addrs
            .iter()
            .filter(|x| x.if_addr().ip() != address)
            .cloned()
            .collect();

        // Update network interface
        machine_state_interface.network_interface = Arc::new(NetworkInterface {
            addrs,
            ..(*machine_state_interface.network_interface).clone()
        });

        // Update interfaces map
        let interfaces = self
            .fields
            .interfaces
            .update(interface_key, machine_state_interface);

        // Update fields
        self.fields = Arc::new(MachineStateFields {
            interfaces,
            ..(*self.fields).clone()
        });

        Ok(())
    }

    #[instrument(level = "debug", skip(self, gsm_inner), err)]
    pub fn release_all_addresses(
        &mut self,
        gsm_inner: &mut GlobalStateManagerInner,
        interface_name: &str,
    ) -> GlobalStateManagerResult<()> {
        let interface_key = Arc::new(interface_name.to_string());
        let Some(mut machine_state_interface) = self.fields.interfaces.get(&interface_key).cloned()
        else {
            return Err(GlobalStateManagerError::InvalidName(
                (*interface_key).clone(),
            ));
        };

        Self::release_all_addresses_inner(gsm_inner, &mut machine_state_interface)?;

        // Update interfaces map
        let interfaces = self
            .fields
            .interfaces
            .update(interface_key, machine_state_interface);

        // Update fields
        self.fields = Arc::new(MachineStateFields {
            interfaces,
            ..(*self.fields).clone()
        });

        Ok(())
    }

    #[instrument(level = "debug", skip(self, gsm_inner), err)]
    pub fn release_all_interfaces(
        &mut self,
        gsm_inner: &mut GlobalStateManagerInner,
    ) -> GlobalStateManagerResult<()> {
        let interface_names: Vec<String> = self
            .fields
            .interfaces
            .keys()
            .map(|x| (**x).clone())
            .collect();
        for interface_name in interface_names {
            let interface_key = Arc::new(interface_name);
            let Some(mut machine_state_interface) =
                self.fields.interfaces.get(&interface_key).cloned()
            else {
                return Err(GlobalStateManagerError::InvalidName(
                    (*interface_key).clone(),
                ));
            };

            Self::detach_network_inner(gsm_inner, &mut machine_state_interface)?;
        }

        // Update fields
        self.fields = Arc::new(MachineStateFields {
            interfaces: imbl::HashMap::new(),
            ..(*self.fields).clone()
        });

        Ok(())
    }

    ////////////////////////////////////////////////////////////////////////

    fn detach_network_inner(
        gsm_inner: &mut GlobalStateManagerInner,
        machine_state_interface: &mut MachineStateInterface,
    ) -> GlobalStateManagerResult<()> {
        Self::release_all_addresses_inner(gsm_inner, machine_state_interface)?;
        machine_state_interface.network_id = None;
        Ok(())
    }

    fn release_all_addresses_inner(
        gsm_inner: &mut GlobalStateManagerInner,
        machine_state_interface: &mut MachineStateInterface,
    ) -> GlobalStateManagerResult<()> {
        let Some(network_id) = machine_state_interface.network_id else {
            return Ok(());
        };

        // Get the network state
        let mut network_state = gsm_inner.network_states().get_state(network_id)?;

        // Release the addresses from the network
        for addr in &machine_state_interface.network_interface.addrs {
            match addr.if_addr.ip() {
                IpAddr::V4(ipv4_addr) => network_state.release_address_v4(ipv4_addr)?,
                IpAddr::V6(ipv6_addr) => network_state.release_address_v6(ipv6_addr)?,
            };
        }

        // Update the network state
        gsm_inner.network_states_mut().set_state(network_state);

        // Remove the addresses from the interface
        let mut new_intf = (*machine_state_interface.network_interface).clone();
        new_intf.addrs.clear();

        // Update interface
        machine_state_interface.network_interface = Arc::new(new_intf);

        Ok(())
    }

    pub fn release_interface(
        &mut self,
        gsm_inner: &mut GlobalStateManagerInner,
        interface_name: &str,
    ) -> GlobalStateManagerResult<()> {
        let interface_key = Arc::new(interface_name.to_string());
        let Some(mut machine_state_interface) = self.fields.interfaces.get(&interface_key).cloned()
        else {
            return Err(GlobalStateManagerError::InvalidName(
                (*interface_key).clone(),
            ));
        };

        Self::detach_network_inner(gsm_inner, &mut machine_state_interface)?;

        // Update interfaces map
        let interfaces = self.fields.interfaces.without(&interface_key);

        // Update fields
        self.fields = Arc::new(MachineStateFields {
            interfaces,
            ..(*self.fields).clone()
        });

        Ok(())
    }
}

impl State for MachineState {
    fn id(&self) -> StateId<Self> {
        self.immutable.id
    }

    fn name(&self) -> Option<String> {
        self.immutable.opt_name.clone()
    }
}
