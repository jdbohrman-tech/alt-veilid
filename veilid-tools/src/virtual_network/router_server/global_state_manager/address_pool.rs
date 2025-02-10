use super::*;

#[derive(Debug, Clone)]
pub struct AddressPool<T: fmt::Debug + Clone> {
    scope_v4: imbl::Vector<Ipv4Net>,
    scope_v6: imbl::Vector<Ipv6Net>,

    allocated_v4: imbl::Vector<Ipv4Net>,
    allocated_v6: imbl::Vector<Ipv6Net>,

    owner_tags_v4: imbl::HashMap<Ipv4Net, Option<T>>,
    owner_tags_v6: imbl::HashMap<Ipv6Net, Option<T>>,
}

impl<T: fmt::Debug + Clone> AddressPool<T> {
    pub fn new() -> Self {
        Self {
            scope_v4: imbl::Vector::new(),
            scope_v6: imbl::Vector::new(),
            allocated_v4: imbl::Vector::new(),
            allocated_v6: imbl::Vector::new(),
            owner_tags_v4: imbl::HashMap::new(),
            owner_tags_v6: imbl::HashMap::new(),
        }
    }

    /////////////////////////////////////////////////////////////////////

    pub fn scopes_v4(&self) -> Vec<Ipv4Net> {
        self.scope_v4.iter().cloned().collect()
    }
    pub fn allocations_v4(&self) -> Vec<Ipv4Net> {
        self.allocated_v4.iter().cloned().collect()
    }
    pub fn scopes_v6(&self) -> Vec<Ipv6Net> {
        self.scope_v6.iter().cloned().collect()
    }
    pub fn allocations_v6(&self) -> Vec<Ipv6Net> {
        self.allocated_v6.iter().cloned().collect()
    }

    pub fn add_scope_v4(&mut self, allocation: Ipv4Net) {
        let mut scopes = self.scope_v4.iter().copied().collect::<Vec<_>>();
        scopes.push(allocation);
        scopes = Ipv4Net::aggregate(&scopes);
        self.scope_v4 = scopes.into();
    }

    pub fn add_scope_v6(&mut self, allocation: Ipv6Net) {
        let mut scopes = self.scope_v6.iter().copied().collect::<Vec<_>>();
        scopes.push(allocation);
        scopes = Ipv6Net::aggregate(&scopes);
        self.scope_v6 = scopes.into();
    }

    pub fn find_scope_v4(&self, allocation: Ipv4Net) -> Option<Ipv4Net> {
        for x in &self.scope_v4 {
            if x.contains(&allocation) {
                return Some(*x);
            }
        }
        None
    }

    pub fn find_scope_v6(&self, allocation: Ipv6Net) -> Option<Ipv6Net> {
        for x in &self.scope_v6 {
            if x.contains(&allocation) {
                return Some(*x);
            }
        }
        None
    }

    pub fn can_allocate_v6(&self, prefix: u8) -> GlobalStateManagerResult<bool> {
        if prefix > 128 {
            return Err(GlobalStateManagerError::InvalidPrefix(prefix));
        }

        let mut srng = StableRng::new(0);
        let opt_allocation = self.find_random_allocation_v6(&mut srng, prefix);
        Ok(opt_allocation.is_some())
    }

    pub fn can_allocate_v4(&self, prefix: u8) -> GlobalStateManagerResult<bool> {
        if prefix > 32 {
            return Err(GlobalStateManagerError::InvalidPrefix(prefix));
        }

        let mut srng = StableRng::new(0);
        let opt_allocation = self.find_random_allocation_v4(&mut srng, prefix);
        Ok(opt_allocation.is_some())
    }

    #[instrument(level = "debug", skip(self), err)]
    pub fn reserve_allocation_v4(
        &mut self,
        allocation: Ipv4Net,
        opt_tag: Option<T>,
    ) -> GlobalStateManagerResult<Ipv4Net> {
        // Ensure the allocation is in our scope
        let Some(scope) = self.find_scope_v4(allocation) else {
            return Err(GlobalStateManagerError::NoAllocation);
        };

        // Only reserve if it's not overlapping an allocation
        if !self.get_overlaps_v4(allocation).is_empty() {
            return Err(GlobalStateManagerError::NoAllocation);
        }

        // Add to our allocated pool
        self.allocated_v4.insert_ord(allocation);
        self.owner_tags_v4.insert(allocation, opt_tag);

        Ok(scope)
    }

    #[instrument(level = "debug", skip(self), err)]
    pub fn reserve_allocation_v6(
        &mut self,
        allocation: Ipv6Net,
        opt_tag: Option<T>,
    ) -> GlobalStateManagerResult<Ipv6Net> {
        // Ensure the allocation is in our scope
        let Some(scope) = self.find_scope_v6(allocation) else {
            return Err(GlobalStateManagerError::NoAllocation);
        };

        // Only reserve if it's not overlapping an allocation
        if !self.get_overlaps_v6(allocation).is_empty() {
            return Err(GlobalStateManagerError::NoAllocation);
        }

        // Add to our allocated pool
        self.allocated_v6.insert_ord(allocation);
        self.owner_tags_v6.insert(allocation, opt_tag);

        Ok(scope)
    }

    pub fn get_overlaps_v4(&self, allocation: Ipv4Net) -> Vec<Ipv4Net> {
        let mut overlaps = Vec::<Ipv4Net>::new();
        for x in &self.allocated_v4 {
            if x.contains(&allocation) || allocation.contains(x) {
                overlaps.push(*x);
                overlaps = Ipv4Net::aggregate(&overlaps);
            }
        }
        overlaps
    }

    pub fn get_overlaps_v6(&self, allocation: Ipv6Net) -> Vec<Ipv6Net> {
        let mut overlaps = Vec::<Ipv6Net>::new();
        for x in &self.allocated_v6 {
            if x.contains(&allocation) || allocation.contains(x) {
                overlaps.push(*x);
                overlaps = Ipv6Net::aggregate(&overlaps);
            }
        }
        overlaps
    }

    #[instrument(level = "debug", skip(self, srng), err)]
    pub fn allocate_random_v4(
        &mut self,
        srng: &mut StableRng,
        prefix: u8,
        tag: T,
    ) -> GlobalStateManagerResult<Option<Ipv4Net>> {
        if prefix > 32 {
            return Err(GlobalStateManagerError::InvalidPrefix(prefix));
        }

        let opt_allocation = self.find_random_allocation_v4(srng, prefix);

        // If we found a free subnet, add it to our allocations
        if let Some(allocation) = opt_allocation {
            // Add to our allocated pool
            self.allocated_v4.insert_ord(allocation);
            self.owner_tags_v4.insert(allocation, Some(tag));
            return Ok(Some(allocation));
        }

        // No allocation
        Ok(None)
    }

    #[instrument(level = "debug", skip(self, srng), err)]
    pub fn allocate_random_v6(
        &mut self,
        srng: &mut StableRng,
        prefix: u8,
        tag: T,
    ) -> GlobalStateManagerResult<Option<Ipv6Net>> {
        if prefix > 128 {
            return Err(GlobalStateManagerError::InvalidPrefix(prefix));
        }

        let opt_allocation = self.find_random_allocation_v6(srng, prefix);

        // If we found a free subnet, add it to our allocations
        if let Some(allocation) = opt_allocation {
            // Add to our allocated pool
            self.allocated_v6.insert_ord(allocation);
            self.owner_tags_v6.insert(allocation, Some(tag));
            return Ok(Some(allocation));
        }

        // No allocation
        Ok(None)
    }

    #[instrument(level = "debug", skip(self), err)]
    pub fn release_allocation_v4(
        &mut self,
        allocation: Ipv4Net,
    ) -> GlobalStateManagerResult<Option<T>> {
        let Some(pos) = self.allocated_v4.iter().position(|x| *x == allocation) else {
            return Err(GlobalStateManagerError::NoAllocation);
        };

        let Some(opt_tag) = self.owner_tags_v4.remove(&allocation) else {
            return Err(GlobalStateManagerError::NoAllocation);
        };

        self.allocated_v4.remove(pos);

        Ok(opt_tag)
    }

    #[instrument(level = "debug", skip(self), err)]
    pub fn release_allocation_v6(
        &mut self,
        allocation: Ipv6Net,
    ) -> GlobalStateManagerResult<Option<T>> {
        let Some(pos) = self.allocated_v6.iter().position(|x| *x == allocation) else {
            return Err(GlobalStateManagerError::NoAllocation);
        };

        let Some(opt_tag) = self.owner_tags_v6.remove(&allocation) else {
            return Err(GlobalStateManagerError::NoAllocation);
        };

        self.allocated_v4.remove(pos);

        Ok(opt_tag)
    }

    pub fn is_ipv4(&self) -> bool {
        !self.scope_v4.is_empty()
    }

    pub fn is_ipv4_allocated(&self) -> bool {
        self.is_ipv4() && !self.allocated_v4.is_empty()
    }

    pub fn is_ipv6(&self) -> bool {
        !self.scope_v6.is_empty()
    }

    pub fn is_ipv6_allocated(&self) -> bool {
        self.is_ipv6() && !self.allocated_v6.is_empty()
    }

    pub fn is_in_use<F: FnMut(IpNet, &T) -> bool>(&self, mut check: F) -> bool {
        for (netv4, opt_tag) in self.owner_tags_v4.iter() {
            if let Some(tag) = opt_tag.as_ref() {
                if check(IpNet::V4(*netv4), tag) {
                    return true;
                }
            }
        }
        for (netv6, opt_tag) in self.owner_tags_v6.iter() {
            if let Some(tag) = opt_tag.as_ref() {
                if check(IpNet::V6(*netv6), tag) {
                    return true;
                }
            }
        }

        false
    }

    #[instrument(level = "debug", skip_all, err)]
    pub fn clear_ipv4<F: FnMut(Ipv4Net, &T) -> bool>(
        &mut self,
        mut check: F,
    ) -> GlobalStateManagerResult<()> {
        if !self.is_ipv4() {
            return Ok(());
        }
        if self.is_in_use(|n, t| match n {
            IpNet::V4(ipv4_net) => check(ipv4_net, t),
            IpNet::V6(_ipv6_net) => false,
        }) {
            return Err(GlobalStateManagerError::ResourceInUse(
                "AddressPool-v4".to_owned(),
            ));
        }
        assert!(self.owner_tags_v4.is_empty(), "tags should be empty");
        self.scope_v4.clear();
        self.allocated_v4.clear();
        self.owner_tags_v4.clear();
        Ok(())
    }

    #[instrument(level = "debug", skip_all, err)]
    pub fn clear_ipv6<F: FnMut(Ipv6Net, &T) -> bool>(
        &mut self,
        mut check: F,
    ) -> GlobalStateManagerResult<()> {
        if !self.is_ipv6() {
            return Ok(());
        }
        if self.is_in_use(|n, t| match n {
            IpNet::V4(_ipv4_net) => false,
            IpNet::V6(ipv6_net) => check(ipv6_net, t),
        }) {
            return Err(GlobalStateManagerError::ResourceInUse(
                "AddressPool-v6".to_owned(),
            ));
        }
        assert!(self.owner_tags_v6.is_empty(), "tags should be empty");
        self.scope_v6.clear();
        self.allocated_v6.clear();
        self.owner_tags_v6.clear();

        Ok(())
    }

    /////////////////////////////////////////////////////////////////////

    fn range_in_prefix_32(scope_prefix: u8, iterable_prefix_bits: u8) -> u32 {
        // If we're allocating addresses, exclude scope's network and broadcast address
        if scope_prefix + iterable_prefix_bits == 32 {
            // Subtract two from total
            if scope_prefix == 0 {
                // Overflow case
                0xFFFF_FFFEu32
            } else {
                // Non-overflow case
                (1u32 << iterable_prefix_bits) - 2
            }
        } else {
            // network only iteration
            1u32 << iterable_prefix_bits
        }
    }

    fn range_in_prefix_128(scope_prefix: u8, iterable_prefix_bits: u8) -> u128 {
        // If we're allocating addresses, exclude scope's network and broadcast address
        if scope_prefix + iterable_prefix_bits == 128 {
            // Subtract two from total
            if scope_prefix == 0 {
                // Overflow case
                0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFEu128
            } else {
                // Non-overflow case
                (1u128 << iterable_prefix_bits) - 2
            }
        } else {
            // network only iteration
            1u128 << iterable_prefix_bits
        }
    }

    fn find_random_allocation_v4(&self, srng: &mut StableRng, prefix: u8) -> Option<Ipv4Net> {
        // Scope ranges to iterate
        let mut scope_ranges = Vec::<(Ipv4Net, u8, u32)>::new();
        let mut total_subnets = 0u32;

        // Build range set from scopes, minus the prefix to allocate
        for scope in self.scope_v4.iter().copied() {
            // If the prefix we are looking to allocate doesn't fit in this scope
            // then we exclude it
            if scope.prefix_len() > prefix {
                continue;
            }

            // Get the number of prefix bits we can iterate
            let iterable_prefix_bits = prefix - scope.prefix_len();
            let iterable_range = Self::range_in_prefix_32(scope.prefix_len(), iterable_prefix_bits);

            // Scope ranges to try
            scope_ranges.push((scope, iterable_prefix_bits, iterable_range));
            total_subnets += iterable_range;
        }
        if total_subnets == 0 {
            // No range
            return None;
        }

        // Choose a random subnet to start with
        let chosen_subnet_index = srng.next_u32(0, total_subnets - 1);

        // Find the starting scope and starting subnet index within
        // the scope of the chosen subnet index
        let mut scope_index = 0usize;
        let mut scope_start_subnet_index = 0u32;
        loop {
            assert!(
                scope_index < scope_ranges.len(),
                "should always have chosen a starting point inside a scope"
            );

            let scope_end_subnet_index = scope_start_subnet_index + scope_ranges[scope_index].2;
            if chosen_subnet_index < scope_end_subnet_index {
                break;
            }

            // chosen starting point is in the next scope
            scope_index += 1;
            scope_start_subnet_index = scope_end_subnet_index;
        }
        let initial_subnet_index = chosen_subnet_index;
        let initial_scope_index = scope_index;

        // Iterate forward until we find a free range
        let mut current_subnet_index = initial_subnet_index;
        let mut current_scope_index = initial_scope_index;
        let mut current_scope_start_subnet_index = scope_start_subnet_index;
        let mut current_scope_end_subnet_index =
            scope_start_subnet_index + scope_ranges[scope_index].2;

        loop {
            // Get the net at this current subnet index
            let netbits = u32::from(scope_ranges[current_scope_index].0.network());
            let subnetbits = if prefix == 32 {
                // Allocating addresses
                ((current_subnet_index - current_scope_start_subnet_index) + 1) << (32 - prefix)
            } else {
                // Allocating subnets
                (current_subnet_index - current_scope_start_subnet_index) << (32 - prefix)
            };
            let net = Ipv4Net::new(Ipv4Addr::from(netbits | subnetbits), prefix)
                .expect("prefix must be valid");
            // See if this net is available
            if self.get_overlaps_v4(net).is_empty() {
                break Some(net);
            }
            // If not, go to the next subnet
            current_subnet_index += 1;

            // If we got back to the beginning we failed to allocate
            if current_scope_index == initial_scope_index
                && current_subnet_index == initial_subnet_index
            {
                break None;
            }

            // If we've reached the end of this scope then go to the next scope
            if current_subnet_index == current_scope_end_subnet_index {
                current_scope_index += 1;
                // Wrap around
                if current_scope_index == scope_ranges.len() {
                    current_subnet_index = 0;
                    current_scope_index = 0;
                    current_scope_start_subnet_index = 0;
                } else {
                    current_scope_start_subnet_index = current_scope_end_subnet_index;
                }
                current_scope_end_subnet_index =
                    current_scope_start_subnet_index + scope_ranges[current_scope_index].2;
            }
        }
    }

    fn find_random_allocation_v6(&self, srng: &mut StableRng, prefix: u8) -> Option<Ipv6Net> {
        // Scope ranges to iterate
        let mut scope_ranges = Vec::<(Ipv6Net, u8, u128)>::new();
        let mut total_subnets = 0u128;

        // Build range set from scopes, minus the prefix to allocate
        for scope in self.scope_v6.iter().copied() {
            // If the prefix we are looking to allocate doesn't fit in this scope
            // then we exclude it
            if scope.prefix_len() > prefix {
                continue;
            }

            // Get the number of prefix bits we can iterate
            let iterable_prefix_bits = prefix - scope.prefix_len();
            let iterable_range =
                Self::range_in_prefix_128(scope.prefix_len(), iterable_prefix_bits);

            // Scope ranges to try
            scope_ranges.push((scope, iterable_prefix_bits, iterable_range));
            total_subnets += iterable_range;
        }
        if total_subnets == 0 {
            // No range
            return None;
        }

        // Choose a random subnet to start with
        let chosen_subnet_index = srng.next_u128(0, total_subnets - 1);

        // Find the starting scope and starting subnet index within
        // the scope of the chosen subnet index
        let mut scope_index = 0usize;
        let mut scope_start_subnet_index = 0u128;
        loop {
            assert!(
                scope_index < scope_ranges.len(),
                "should always have chosen a starting point inside a scope"
            );

            let scope_end_subnet_index = scope_start_subnet_index + scope_ranges[scope_index].2;
            if chosen_subnet_index < scope_end_subnet_index {
                break;
            }

            // chosen starting point is in the next scope
            scope_index += 1;
            scope_start_subnet_index = scope_end_subnet_index;
        }
        let initial_subnet_index = chosen_subnet_index;
        let initial_scope_index = scope_index;

        // Iterate forward until we find a free range
        let mut current_subnet_index = initial_subnet_index;
        let mut current_scope_index = initial_scope_index;
        let mut current_scope_start_subnet_index = scope_start_subnet_index;
        let mut current_scope_end_subnet_index =
            scope_start_subnet_index + scope_ranges[scope_index].2;

        loop {
            // Get the net at this current subnet index
            let netbits = u128::from(scope_ranges[current_scope_index].0.network());
            let subnetbits = if prefix == 128 {
                // Allocating addresses
                ((current_subnet_index - current_scope_start_subnet_index) + 1) << (128 - prefix)
            } else {
                // Allocating subnets
                (current_subnet_index - current_scope_start_subnet_index) << (128 - prefix)
            };
            let net = Ipv6Net::new(Ipv6Addr::from(netbits | subnetbits), prefix)
                .expect("prefix must be valid");
            // See if this net is available
            if self.get_overlaps_v6(net).is_empty() {
                break Some(net);
            }
            // If not, go to the next subnet
            current_subnet_index += 1;

            // If we got back to the beginning we failed to allocate
            if current_scope_index == initial_scope_index
                && current_subnet_index == initial_subnet_index
            {
                break None;
            }

            // If we've reached the end of this scope then go to the next scope
            if current_subnet_index == current_scope_end_subnet_index {
                current_scope_index += 1;
                // Wrap around
                if current_scope_index == scope_ranges.len() {
                    current_subnet_index = 0;
                    current_scope_index = 0;
                    current_scope_start_subnet_index = 0;
                } else {
                    current_scope_start_subnet_index = current_scope_end_subnet_index;
                }
                current_scope_end_subnet_index =
                    current_scope_start_subnet_index + scope_ranges[current_scope_index].2;
            }
        }
    }
}
