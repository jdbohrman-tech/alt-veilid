use super::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum OwnerTag {
    Machine(MachineStateId),
    Network(NetworkStateId),
    Gateway(NetworkStateId),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum NetworkOrigin {
    Config,
    Direct,
    Blueprint(BlueprintStateId),
}

#[derive(Debug)]
struct NetworkStateImmutable {
    /// The unique id of this network
    id: NetworkStateId,
    /// The name of this network state if it was made directly
    opt_name: Option<String>,
    /// Where this network came for housekeeping purposes
    origin: NetworkOrigin,
}

#[derive(Debug, Clone)]
struct NetworkStateFields {
    /// Model for this network
    model: NetworkStateModel,
    /// The addresses allocated by this network
    address_pool: AddressPool<OwnerTag>,
    /// IPv4 state if it is enabled
    ipv4: Option<NetworkStateIpv4>,
    /// IPv6 state if it is enabled
    ipv6: Option<NetworkStateIpv6>,
}

#[derive(Debug, Clone)]
struct NetworkStateModel {
    params: NetworkStateModelParams,
}

#[derive(Debug, Clone)]
pub struct NetworkStateModelParams {
    /// Network latency distribution
    pub latency: config::Distribution,
    /// Distance simulation metric
    pub distance: Option<config::Distance>,
    /// Packet loss probability
    pub loss: Probability,
}

#[derive(Debug, Clone)]
pub struct NetworkStateIpv4Params {
    pub scope: Vec<Ipv4Net>,
    pub reserve: Vec<Ipv4Net>,
    pub super_net: Option<NetworkStateId>,
}

#[derive(Debug, Clone)]
struct NetworkStateIpv4 {
    params: NetworkStateIpv4Params,
    gateway: Option<NetworkStateIpv4Gateway>,
}

#[derive(Debug, Clone)]
pub struct NetworkStateIpv6Params {
    pub scope: Vec<Ipv6Net>,
    pub reserve: Vec<Ipv6Net>,
    pub super_net: Option<NetworkStateId>,
}
#[derive(Debug, Clone)]
struct NetworkStateIpv6 {
    params: NetworkStateIpv6Params,
    gateway: Option<NetworkStateIpv6Gateway>,
}

#[derive(Debug, Clone)]
pub struct NetworkStateIpv4GatewayParams {
    pub translation: config::Translation,
    pub upnp: bool,
    pub external_network: NetworkStateId,
    pub internal_address: Option<Ipv4Addr>,
    pub external_address: Option<Ipv4Addr>,
}

#[derive(Debug, Clone)]
pub struct NetworkStateIpv6GatewayParams {
    pub translation: config::Translation,
    pub upnp: bool,
    pub external_network: NetworkStateId,
    pub internal_address: Option<Ipv6Addr>,
    pub external_address: Option<Ipv6Addr>,
}

#[derive(Debug, Clone)]
struct NetworkStateIpv4Gateway {
    params: NetworkStateIpv4GatewayParams,
    internal_interface_address: Ifv4Addr,
    external_interface_address: Ifv4Addr,
}

#[derive(Debug, Clone)]
struct NetworkStateIpv6Gateway {
    params: NetworkStateIpv6GatewayParams,
    internal_interface_address: Ifv6Addr,
    external_interface_address: Ifv6Addr,
}

#[derive(Debug, Clone)]
pub struct NetworkState {
    immutable: Arc<NetworkStateImmutable>,
    fields: Arc<NetworkStateFields>,
}

pub type NetworkStateId = StateId<NetworkState>;

impl NetworkState {
    pub fn new(id: NetworkStateId, opt_name: Option<String>, origin: NetworkOrigin) -> Self {
        Self {
            immutable: Arc::new(NetworkStateImmutable {
                id,
                opt_name,
                origin,
            }),
            fields: Arc::new(NetworkStateFields {
                address_pool: AddressPool::new(),
                model: NetworkStateModel {
                    params: NetworkStateModelParams {
                        latency: config::Distribution::default(),
                        distance: None,
                        loss: 0.0,
                    },
                },
                ipv4: None,
                ipv6: None,
            }),
        }
    }

    #[instrument(level = "debug", skip(self, gsm_inner))]
    pub fn release(self, gsm_inner: &mut GlobalStateManagerInner) {
        if let NetworkOrigin::Blueprint(generating_blueprint) = self.immutable.origin {
            let mut blueprint_state = gsm_inner
                .blueprint_states()
                .get_state(generating_blueprint)
                .expect("must exist");
            blueprint_state.on_network_released(self.id());
            gsm_inner.blueprint_states_mut().set_state(blueprint_state)
        }
    }

    #[instrument(level = "debug", skip(self))]
    pub fn set_model(&mut self, params: NetworkStateModelParams) {
        self.fields = Arc::new(NetworkStateFields {
            model: NetworkStateModel { params },
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

        let mut address_pool = self.fields.address_pool.clone();
        address_pool
            .clear_ipv4(|_n, t| match t {
                OwnerTag::Machine(_) => true,
                OwnerTag::Network(nsid) => *nsid != self.id(),
                OwnerTag::Gateway(nsid) => *nsid != self.id(),
            })
            .map_err(|_| {
                GlobalStateManagerError::ResourceInUse(format!("{}-v4", self.debug_name()))
            })?;

        // Update fields
        self.fields = Arc::new(NetworkStateFields {
            ipv4: None,
            address_pool,
            ..(*self.fields).clone()
        });

        Ok(())
    }

    #[instrument(level = "debug", skip(self, gsm_inner), err)]
    pub fn clear_ipv4_gateway(
        &mut self,
        gsm_inner: &mut GlobalStateManagerInner,
    ) -> GlobalStateManagerResult<()> {
        let Some(mut ipv4) = self.fields.ipv4.clone() else {
            return Ok(());
        };
        let Some(gateway) = ipv4.gateway else {
            return Ok(());
        };
        if gateway.params.external_network != self.id() {
            // Get the external network state
            let mut external_network_state = gsm_inner
                .network_states()
                .get_state(gateway.params.external_network)
                .expect("must succeed");

            // Release external address
            external_network_state
                .release_address_v4(gateway.external_interface_address.ip)
                .expect("must succeed");

            // Update external network
            gsm_inner
                .network_states_mut()
                .set_state(external_network_state);
        }

        // Release internal address
        self.release_address_v4(gateway.internal_interface_address.ip)
            .expect("must succeed");

        // Clear gateway
        ipv4.gateway = None;

        // Update fields
        self.fields = Arc::new(NetworkStateFields {
            ipv4: Some(ipv4),
            ..(*self.fields).clone()
        });

        Ok(())
    }

    #[instrument(level = "debug", skip(self, gsm_inner), err)]
    pub fn set_ipv4(
        &mut self,
        gsm_inner: &mut GlobalStateManagerInner,
        params: NetworkStateIpv4Params,
    ) -> GlobalStateManagerResult<()> {
        self.clear_ipv4(gsm_inner)?;

        let mut address_pool = self.fields.address_pool.clone();
        for scope in &params.scope {
            address_pool.add_scope_v4(*scope);
        }
        for reserve in &params.reserve {
            address_pool.reserve_allocation_v4(*reserve, None)?;
        }

        let ipv4 = NetworkStateIpv4 {
            params,
            gateway: None,
        };

        // Update fields
        self.fields = Arc::new(NetworkStateFields {
            ipv4: Some(ipv4),
            address_pool,
            ..(*self.fields).clone()
        });

        Ok(())
    }

    #[instrument(level = "debug", skip(self, gsm_inner), err)]
    pub fn set_ipv4_gateway(
        &mut self,
        gsm_inner: &mut GlobalStateManagerInner,
        gateway_params: NetworkStateIpv4GatewayParams,
    ) -> GlobalStateManagerResult<()> {
        self.clear_ipv4_gateway(gsm_inner)?;

        let Some(mut ipv4) = self.fields.ipv4.clone() else {
            return Err(GlobalStateManagerError::InvalidGateway);
        };

        let mut address_pool = self.fields.address_pool.clone();

        // Allocate or reserve an internal network address for the gateway
        let internal_interface_address =
            if let Some(internal_address) = gateway_params.internal_address {
                let scope = address_pool.reserve_allocation_v4(
                    Ipv4Net::new(internal_address, 32).expect("must succeed"),
                    Some(OwnerTag::Gateway(self.id())),
                )?;

                // Make interface address
                Ifv4Addr {
                    ip: internal_address,
                    netmask: scope.netmask(),
                    broadcast: Some(scope.broadcast()),
                }
            } else {
                let Some(internal_address) = address_pool.allocate_random_v4(
                    gsm_inner.srng(),
                    32,
                    OwnerTag::Gateway(self.id()),
                )?
                else {
                    return Err(GlobalStateManagerError::NoAllocation);
                };

                // Get the scope this allocation fits in
                let scope = address_pool
                    .find_scope_v4(internal_address)
                    .expect("must succeed");

                // Make interface address
                let internal_address = internal_address.addr();
                Ifv4Addr {
                    ip: internal_address,
                    netmask: scope.netmask(),
                    broadcast: Some(scope.broadcast()),
                }
            };

        // Get the external network state
        let mut external_network_state = gsm_inner
            .network_states()
            .get_state(gateway_params.external_network)
            .expect("must succeed");

        // Allocate or reserve an external network address for the gateway
        let external_interface_address =
            if matches!(gateway_params.translation, config::Translation::None) {
                // If the translation mode is 'none', then the external and internal
                // addresses must be the same
                external_network_state.allocate_address_v4(
                    gsm_inner,
                    OwnerTag::Gateway(self.id()),
                    Some(internal_interface_address.ip),
                )?
            } else {
                // Network translation means the internal and external addresses
                // will be different
                external_network_state.allocate_address_v4(
                    gsm_inner,
                    OwnerTag::Gateway(self.id()),
                    None,
                )?
            };

        // Update external network
        gsm_inner
            .network_states_mut()
            .set_state(external_network_state);

        // Set the gateway state
        ipv4.gateway = Some(NetworkStateIpv4Gateway {
            params: gateway_params,
            internal_interface_address,
            external_interface_address,
        });

        // Update fields
        self.fields = Arc::new(NetworkStateFields {
            ipv4: Some(ipv4),
            address_pool,
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

        let mut address_pool = self.fields.address_pool.clone();
        address_pool
            .clear_ipv6(|_n, t| match t {
                OwnerTag::Machine(_) => true,
                OwnerTag::Network(nsid) => *nsid != self.id(),
                OwnerTag::Gateway(nsid) => *nsid != self.id(),
            })
            .map_err(|_| {
                GlobalStateManagerError::ResourceInUse(format!("{}-v6", self.debug_name()))
            })?;

        // Update fields
        self.fields = Arc::new(NetworkStateFields {
            ipv6: None,
            address_pool,
            ..(*self.fields).clone()
        });

        Ok(())
    }

    #[instrument(level = "debug", skip(self, gsm_inner), err)]
    pub fn clear_ipv6_gateway(
        &mut self,
        gsm_inner: &mut GlobalStateManagerInner,
    ) -> GlobalStateManagerResult<()> {
        let Some(mut ipv6) = self.fields.ipv6.clone() else {
            return Ok(());
        };
        let Some(gateway) = ipv6.gateway else {
            return Ok(());
        };
        if gateway.params.external_network != self.id() {
            // Get the external network state
            let mut external_network_state = gsm_inner
                .network_states()
                .get_state(gateway.params.external_network)
                .expect("must succeed");

            // Release external address
            external_network_state
                .release_address_v6(gateway.external_interface_address.ip)
                .expect("must succeed");

            // Update external network
            gsm_inner
                .network_states_mut()
                .set_state(external_network_state);
        }

        // Release internal address
        self.release_address_v6(gateway.internal_interface_address.ip)
            .expect("must succeed");

        // Clear gateway
        ipv6.gateway = None;

        // Update fields
        self.fields = Arc::new(NetworkStateFields {
            ipv6: Some(ipv6),
            ..(*self.fields).clone()
        });

        Ok(())
    }

    #[instrument(level = "debug", skip(self, gsm_inner), err)]
    pub fn set_ipv6(
        &mut self,
        gsm_inner: &mut GlobalStateManagerInner,
        params: NetworkStateIpv6Params,
    ) -> GlobalStateManagerResult<()> {
        self.clear_ipv6(gsm_inner)?;

        let mut address_pool = self.fields.address_pool.clone();
        for scope in &params.scope {
            address_pool.add_scope_v6(*scope);
        }
        for reserve in &params.reserve {
            address_pool.reserve_allocation_v6(*reserve, None)?;
        }
        let ipv6 = NetworkStateIpv6 {
            params,
            gateway: None,
        };

        // Update fields
        self.fields = Arc::new(NetworkStateFields {
            ipv6: Some(ipv6),
            address_pool,
            ..(*self.fields).clone()
        });

        Ok(())
    }

    #[instrument(level = "debug", skip(self, gsm_inner), err)]
    pub fn set_ipv6_gateway(
        &mut self,
        gsm_inner: &mut GlobalStateManagerInner,
        gateway_params: NetworkStateIpv6GatewayParams,
    ) -> GlobalStateManagerResult<()> {
        self.clear_ipv6_gateway(gsm_inner)?;

        let Some(mut ipv6) = self.fields.ipv6.clone() else {
            return Err(GlobalStateManagerError::InvalidGateway);
        };

        let mut address_pool = self.fields.address_pool.clone();

        // Allocate or reserve an internal network address for the gateway
        let internal_interface_address =
            if let Some(internal_address) = gateway_params.internal_address {
                let scope = address_pool.reserve_allocation_v6(
                    Ipv6Net::new(internal_address, 128).expect("must succeed"),
                    Some(OwnerTag::Gateway(self.id())),
                )?;
                // Make interface address
                Ifv6Addr {
                    ip: internal_address,
                    netmask: scope.netmask(),
                    broadcast: Some(scope.broadcast()),
                }
            } else {
                let Some(internal_address) = address_pool.allocate_random_v6(
                    gsm_inner.srng(),
                    128,
                    OwnerTag::Gateway(self.id()),
                )?
                else {
                    return Err(GlobalStateManagerError::NoAllocation);
                };
                // Get the scope this allocation fits in
                let scope = address_pool
                    .find_scope_v6(internal_address)
                    .expect("must succeed");

                // Make interface address
                let internal_address = internal_address.addr();
                Ifv6Addr {
                    ip: internal_address,
                    netmask: scope.netmask(),
                    broadcast: Some(scope.broadcast()),
                }
            };

        // Get the external network state
        let mut external_network_state = gsm_inner
            .network_states()
            .get_state(gateway_params.external_network)
            .expect("must succeed");

        // Allocate or reserve an external network address for the gateway
        let external_interface_address =
            if matches!(gateway_params.translation, config::Translation::None) {
                // If the translation mode is 'none', then the external and internal
                // addresses must be the same
                external_network_state.allocate_address_v6(
                    gsm_inner,
                    OwnerTag::Gateway(self.id()),
                    Some(internal_interface_address.ip),
                )?
            } else {
                // Network translation means the internal and external addresses
                // will be different
                external_network_state.allocate_address_v6(
                    gsm_inner,
                    OwnerTag::Gateway(self.id()),
                    None,
                )?
            };

        // Update external network
        gsm_inner
            .network_states_mut()
            .set_state(external_network_state);

        // Set the gateway state
        ipv6.gateway = Some(NetworkStateIpv6Gateway {
            params: gateway_params,
            internal_interface_address,
            external_interface_address,
        });

        // Update fields
        self.fields = Arc::new(NetworkStateFields {
            ipv6: Some(ipv6),
            address_pool,
            ..(*self.fields).clone()
        });

        Ok(())
    }

    pub fn is_ipv4(&self) -> bool {
        self.fields.ipv4.is_some()
    }

    pub fn is_ipv6(&self) -> bool {
        self.fields.ipv6.is_some()
    }

    pub fn is_active(&self) -> GlobalStateManagerResult<bool> {
        let mut can_allocate = false;

        if self.fields.ipv4.is_some() {
            //
            if !self.fields.address_pool.can_allocate_v4(32)? {
                can_allocate = false;
            }
        }
        if self.fields.ipv6.is_some() {
            //
            if !self.fields.address_pool.can_allocate_v6(128)? {
                can_allocate = false;
            }
        }
        Ok(can_allocate)
    }

    #[instrument(level = "debug", skip(self, gsm_inner), err)]
    pub fn allocate_address_v4(
        &mut self,
        gsm_inner: &mut GlobalStateManagerInner,
        owner_tag: OwnerTag,
        opt_address: Option<Ipv4Addr>,
    ) -> GlobalStateManagerResult<Ifv4Addr> {
        let net = self.allocate_subnet_v4(gsm_inner, owner_tag, opt_address, 32)?;
        let scope = self
            .fields
            .address_pool
            .find_scope_v4(net)
            .expect("must succeed");
        let ip = net.addr();
        let netmask = scope.netmask();
        let broadcast = scope.broadcast();

        let ifaddr = Ifv4Addr {
            ip,
            netmask,
            broadcast: Some(broadcast),
        };

        Ok(ifaddr)
    }

    pub fn can_allocate_address_v4(&self, opt_address: Option<Ipv4Addr>) -> bool {
        self.can_allocate_subnet_v4(opt_address, 32)
    }

    #[instrument(level = "debug", skip(self, gsm_inner), err)]
    pub fn allocate_subnet_v4(
        &mut self,
        gsm_inner: &mut GlobalStateManagerInner,
        owner_tag: OwnerTag,
        opt_address: Option<Ipv4Addr>,
        prefix: u8,
    ) -> GlobalStateManagerResult<Ipv4Net> {
        if self.fields.ipv4.is_none() {
            return Err(GlobalStateManagerError::NoAllocation);
        }

        // See if we are requesting a specific address
        let mut address_pool = self.fields.address_pool.clone();

        let net = if let Some(address) = opt_address {
            // Get the net form for this address
            let net = Ipv4Net::new(address, prefix).expect("must succeed");
            address_pool.reserve_allocation_v4(net, Some(owner_tag))?;
            net
        } else {
            // Get a random address if available
            let Some(allocation) =
                address_pool.allocate_random_v4(gsm_inner.srng(), prefix, owner_tag)?
            else {
                return Err(GlobalStateManagerError::NoAllocation);
            };
            allocation
        };

        // Update fields
        self.fields = Arc::new(NetworkStateFields {
            address_pool,
            ..(*self.fields).clone()
        });

        Ok(net)
    }

    pub fn can_allocate_subnet_v4(&self, opt_address: Option<Ipv4Addr>, prefix: u8) -> bool {
        if self.fields.ipv4.is_none() {
            return false;
        };

        // See if we are requesting a specific address
        if let Some(address) = opt_address {
            // Get the net form for this address
            let net = Ipv4Net::new(address, prefix).expect("must succeed");
            self.fields.address_pool.get_overlaps_v4(net).is_empty()
        } else {
            // Get a random address if available
            self.fields
                .address_pool
                .can_allocate_v4(prefix)
                .unwrap_or(false)
        }
    }

    #[instrument(level = "debug", skip(self), err)]
    pub fn release_address_v4(
        &mut self,
        addr: Ipv4Addr,
    ) -> GlobalStateManagerResult<Option<OwnerTag>> {
        self.release_subnet_v4(Ipv4Net::new(addr, 32).expect("must succeed"))
    }

    #[instrument(level = "debug", skip(self), err)]
    pub fn release_subnet_v4(
        &mut self,
        net: Ipv4Net,
    ) -> GlobalStateManagerResult<Option<OwnerTag>> {
        let mut address_pool = self.fields.address_pool.clone();
        let opt_tag = address_pool.release_allocation_v4(net)?;

        // Update fields
        self.fields = Arc::new(NetworkStateFields {
            address_pool,
            ..(*self.fields).clone()
        });
        Ok(opt_tag)
    }

    #[instrument(level = "debug", skip(self, gsm_inner), err)]
    pub fn allocate_address_v6(
        &mut self,
        gsm_inner: &mut GlobalStateManagerInner,
        owner_tag: OwnerTag,
        opt_address: Option<Ipv6Addr>,
    ) -> GlobalStateManagerResult<Ifv6Addr> {
        let net = self.allocate_subnet_v6(gsm_inner, owner_tag, opt_address, 128)?;
        let scope = self
            .fields
            .address_pool
            .find_scope_v6(net)
            .expect("must succeed");

        let ip = net.addr();
        let netmask = scope.netmask();
        let broadcast = scope.broadcast();

        let ifaddr = Ifv6Addr {
            ip,
            netmask,
            broadcast: Some(broadcast),
        };

        Ok(ifaddr)
    }

    pub fn can_allocate_address_v6(&self, opt_address: Option<Ipv6Addr>) -> bool {
        self.can_allocate_subnet_v6(opt_address, 128)
    }

    #[instrument(level = "debug", skip(self, gsm_inner), err)]
    pub fn allocate_subnet_v6(
        &mut self,
        gsm_inner: &mut GlobalStateManagerInner,
        owner_tag: OwnerTag,
        opt_address: Option<Ipv6Addr>,
        prefix: u8,
    ) -> GlobalStateManagerResult<Ipv6Net> {
        if self.fields.ipv6.is_none() {
            return Err(GlobalStateManagerError::NoAllocation);
        }

        // See if we are requesting a specific address
        let mut address_pool = self.fields.address_pool.clone();

        let net = if let Some(address) = opt_address {
            // Get the net form for this address
            let net = Ipv6Net::new(address, prefix).expect("must succeed");
            address_pool.reserve_allocation_v6(net, Some(owner_tag))?;
            net
        } else {
            // Get a random address if available
            let Some(allocation) =
                address_pool.allocate_random_v6(gsm_inner.srng(), prefix, owner_tag)?
            else {
                return Err(GlobalStateManagerError::NoAllocation);
            };
            allocation
        };

        // Update fields
        self.fields = Arc::new(NetworkStateFields {
            address_pool,
            ..(*self.fields).clone()
        });

        Ok(net)
    }

    pub fn can_allocate_subnet_v6(&self, opt_address: Option<Ipv6Addr>, prefix: u8) -> bool {
        if self.fields.ipv6.is_none() {
            return false;
        };

        // See if we are requesting a specific address
        if let Some(address) = opt_address {
            // Get the net form for this address
            let net = Ipv6Net::new(address, prefix).expect("must succeed");
            self.fields.address_pool.get_overlaps_v6(net).is_empty()
        } else {
            // Get a random address if available
            self.fields
                .address_pool
                .can_allocate_v6(prefix)
                .unwrap_or(false)
        }
    }

    #[instrument(level = "debug", skip(self), err)]
    pub fn release_address_v6(
        &mut self,
        addr: Ipv6Addr,
    ) -> GlobalStateManagerResult<Option<OwnerTag>> {
        self.release_subnet_v6(Ipv6Net::new(addr, 128).expect("must succeed"))
    }

    #[instrument(level = "debug", skip(self), err)]
    pub fn release_subnet_v6(
        &mut self,
        net: Ipv6Net,
    ) -> GlobalStateManagerResult<Option<OwnerTag>> {
        let mut address_pool = self.fields.address_pool.clone();
        let opt_tag = address_pool.release_allocation_v6(net)?;

        // Update fields
        self.fields = Arc::new(NetworkStateFields {
            address_pool,
            ..(*self.fields).clone()
        });
        Ok(opt_tag)
    }
}

impl State for NetworkState {
    fn id(&self) -> StateId<Self> {
        self.immutable.id
    }

    fn name(&self) -> Option<String> {
        self.immutable.opt_name.clone()
    }
}
