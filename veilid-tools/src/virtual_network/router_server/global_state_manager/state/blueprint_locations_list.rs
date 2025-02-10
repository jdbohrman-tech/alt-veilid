use super::*;

/// Locations where a network can be instantiated when a blueprint is generated
#[derive(Debug, Clone)]
pub enum BlueprintLocationsList {
    /// Network will be a new allocation
    Allocations { allocations: WeightedList<String> },
    /// Network will be allocated as a subnet of an existing network
    Networks {
        networks: WeightedList<NetworkStateId>,
    },
}

#[derive(Debug, Clone)]
pub struct NetworkLocation<T> {
    pub scope: Vec<T>,
    pub reserve: Vec<T>,
    pub super_net: Option<NetworkStateId>,
}

impl BlueprintLocationsList {
    #[instrument(level = "debug", skip_all, err)]
    pub fn pick_v4(
        &self,
        gsm_inner: &mut GlobalStateManagerInner,
        prefix: &WeightedList<u8>,
    ) -> GlobalStateManagerResult<Option<NetworkLocation<Ipv4Net>>> {
        // Get maximum prefix
        let max_prefix = prefix
            .iter()
            .max()
            .copied()
            .expect("must have at least one element");

        // Get addresses for network
        match self {
            BlueprintLocationsList::Allocations { allocations } => {
                // Get allocations which have subnets that would fit
                // our maximum requested prefix
                let Some(address_pools) = allocations.try_filter_map(|allocation_name| {
                    let allocation = gsm_inner
                        .allocations()
                        .get(allocation_name)
                        .expect("must exist");
                    if allocation.address_pool.can_allocate_v4(max_prefix)? {
                        Ok(Some(allocation.address_pool.clone()))
                    } else {
                        Ok(None)
                    }
                })?
                else {
                    return Ok(None);
                };

                // Pick an address pool
                let mut address_pool = gsm_inner.srng().weighted_choice(address_pools);

                // Pick a prefix length that would fit in the subnet
                let opt_subnet = prefix
                    .try_filter(|p| address_pool.can_allocate_v4(*p))?
                    .as_ref()
                    .map(|wl| {
                        let subnet_prefix = *gsm_inner.srng().weighted_choice_ref(wl);

                        address_pool.allocate_random_v4(gsm_inner.srng(), subnet_prefix, ())
                    })
                    .transpose()?
                    .flatten();
                let Some(subnet) = opt_subnet else {
                    return Ok(None);
                };
                Ok(Some(NetworkLocation {
                    scope: vec![subnet],
                    reserve: Vec::new(),
                    super_net: None,
                }))
            }
            BlueprintLocationsList::Networks { networks } => {
                // Get networks which have subnets that would fit
                // our maximum requested prefix
                let Some(available_networks) = networks.try_filter(|network_id| {
                    let super_network_state = gsm_inner
                        .network_states()
                        .get_state(*network_id)
                        .expect("must exist");

                    Ok(super_network_state.can_allocate_subnet_v4(None, max_prefix))
                })?
                else {
                    return Ok(None);
                };

                // Pick a network
                let super_network_id = *gsm_inner.srng().weighted_choice_ref(&available_networks);
                let mut super_network_state = gsm_inner
                    .network_states()
                    .get_state(super_network_id)
                    .expect("must exist");

                // Pick a prefix that fits in this network and allocate from it
                let opt_subnet = prefix
                    .filter(|p| super_network_state.can_allocate_subnet_v4(None, *p))
                    .as_ref()
                    .map(|wl| {
                        let subnet_prefix = *gsm_inner.srng().weighted_choice_ref(wl);

                        // Allocate subnet from this network
                        super_network_state.allocate_subnet_v4(
                            gsm_inner,
                            OwnerTag::Network(super_network_state.id()),
                            None,
                            subnet_prefix,
                        )
                    })
                    .transpose()?;
                let Some(subnet) = opt_subnet else {
                    return Ok(None);
                };

                // Update network state
                gsm_inner
                    .network_states_mut()
                    .set_state(super_network_state);

                Ok(Some(NetworkLocation {
                    scope: vec![subnet],
                    reserve: Vec::new(),
                    super_net: Some(super_network_id),
                }))
            }
        }
    }

    #[instrument(level = "debug", skip_all, err)]
    pub fn pick_v6(
        &self,
        gsm_inner: &mut GlobalStateManagerInner,
        prefix: &WeightedList<u8>,
    ) -> GlobalStateManagerResult<Option<NetworkLocation<Ipv6Net>>> {
        // Get maximum prefix
        let max_prefix = prefix
            .iter()
            .max()
            .copied()
            .expect("must have at least one element");

        // Get addresses for network
        match self {
            BlueprintLocationsList::Allocations { allocations } => {
                // Get allocations which have subnets that would fit
                // our maximum requested prefix
                let Some(address_pools) = allocations.try_filter_map(|allocation_name| {
                    let allocation = gsm_inner
                        .allocations()
                        .get(allocation_name)
                        .expect("must exist");
                    if allocation.address_pool.can_allocate_v6(max_prefix)? {
                        Ok(Some(allocation.address_pool.clone()))
                    } else {
                        Ok(None)
                    }
                })?
                else {
                    return Ok(None);
                };

                // Pick an address pool
                let mut address_pool = gsm_inner.srng().weighted_choice(address_pools);

                // Pick a prefix length that would fit in the subnet
                let opt_subnet = prefix
                    .try_filter(|p| address_pool.can_allocate_v6(*p))?
                    .as_ref()
                    .map(|wl| {
                        let subnet_prefix = *gsm_inner.srng().weighted_choice_ref(wl);

                        address_pool.allocate_random_v6(gsm_inner.srng(), subnet_prefix, ())
                    })
                    .transpose()?
                    .flatten();
                let Some(subnet) = opt_subnet else {
                    return Ok(None);
                };
                Ok(Some(NetworkLocation {
                    scope: vec![subnet],
                    reserve: Vec::new(),
                    super_net: None,
                }))
            }
            BlueprintLocationsList::Networks { networks } => {
                // Get networks which have subnets that would fit
                // our maximum requested prefix
                let Some(available_networks) = networks.try_filter(|network_id| {
                    let super_network_state = gsm_inner
                        .network_states()
                        .get_state(*network_id)
                        .expect("must exist");

                    Ok(super_network_state.can_allocate_subnet_v6(None, max_prefix))
                })?
                else {
                    return Ok(None);
                };

                // Pick a network
                let super_network_id = *gsm_inner.srng().weighted_choice_ref(&available_networks);
                let mut super_network_state = gsm_inner
                    .network_states()
                    .get_state(super_network_id)
                    .expect("must exist");

                // Pick a prefix that fits in this network and allocate from it
                let opt_subnet = prefix
                    .filter(|p| super_network_state.can_allocate_subnet_v6(None, *p))
                    .as_ref()
                    .map(|wl| {
                        let subnet_prefix = *gsm_inner.srng().weighted_choice_ref(wl);

                        // Allocate subnet from this network
                        super_network_state.allocate_subnet_v6(
                            gsm_inner,
                            OwnerTag::Network(super_network_state.id()),
                            None,
                            subnet_prefix,
                        )
                    })
                    .transpose()?;
                let Some(subnet) = opt_subnet else {
                    return Ok(None);
                };

                // Update network state
                gsm_inner
                    .network_states_mut()
                    .set_state(super_network_state);

                Ok(Some(NetworkLocation {
                    scope: vec![subnet],
                    reserve: Vec::new(),
                    super_net: Some(super_network_id),
                }))
            }
        }
    }
}
