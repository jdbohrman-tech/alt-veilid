use super::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeRefFilter {
    pub routing_domain_set: RoutingDomainSet,
    pub dial_info_filter: DialInfoFilter,
}

impl Default for NodeRefFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeRefFilter {
    pub fn new() -> Self {
        Self {
            routing_domain_set: RoutingDomainSet::all(),
            dial_info_filter: DialInfoFilter::all(),
        }
    }
    pub fn with_routing_domain(mut self, routing_domain: RoutingDomain) -> Self {
        self.routing_domain_set = routing_domain.into();
        self
    }
    pub fn with_routing_domain_set(mut self, routing_domain_set: RoutingDomainSet) -> Self {
        self.routing_domain_set = routing_domain_set;
        self
    }
    pub fn with_dial_info_filter(mut self, dial_info_filter: DialInfoFilter) -> Self {
        self.dial_info_filter = dial_info_filter;
        self
    }
    pub fn with_protocol_type(mut self, protocol_type: ProtocolType) -> Self {
        self.dial_info_filter = self.dial_info_filter.with_protocol_type(protocol_type);
        self
    }
    #[allow(dead_code)]
    pub fn with_protocol_type_set(mut self, protocol_set: ProtocolTypeSet) -> Self {
        self.dial_info_filter = self.dial_info_filter.with_protocol_type_set(protocol_set);
        self
    }
    pub fn with_address_type(mut self, address_type: AddressType) -> Self {
        self.dial_info_filter = self.dial_info_filter.with_address_type(address_type);
        self
    }
    #[allow(dead_code)]
    pub fn with_address_type_set(mut self, address_set: AddressTypeSet) -> Self {
        self.dial_info_filter = self.dial_info_filter.with_address_type_set(address_set);
        self
    }
    pub fn filtered(mut self, other_filter: &NodeRefFilter) -> Self {
        self.routing_domain_set &= other_filter.routing_domain_set;
        self.dial_info_filter = self
            .dial_info_filter
            .filtered(&other_filter.dial_info_filter);
        self
    }
    #[allow(dead_code)]
    pub fn is_dead(&self) -> bool {
        self.dial_info_filter.is_dead() || self.routing_domain_set.is_empty()
    }
    pub fn with_sequencing(mut self, sequencing: Sequencing) -> (bool, Self) {
        let (ordered, dif) = self.dial_info_filter.with_sequencing(sequencing);
        self.dial_info_filter = dif;
        (ordered, self)
    }
}

impl From<RoutingDomain> for NodeRefFilter {
    fn from(other: RoutingDomain) -> Self {
        Self {
            routing_domain_set: other.into(),
            dial_info_filter: DialInfoFilter::all(),
        }
    }
}

impl From<RoutingDomainSet> for NodeRefFilter {
    fn from(other: RoutingDomainSet) -> Self {
        Self {
            routing_domain_set: other,
            dial_info_filter: DialInfoFilter::all(),
        }
    }
}

impl From<DialInfoFilter> for NodeRefFilter {
    fn from(other: DialInfoFilter) -> Self {
        Self {
            routing_domain_set: RoutingDomainSet::all(),
            dial_info_filter: other,
        }
    }
}

impl From<ProtocolType> for NodeRefFilter {
    fn from(other: ProtocolType) -> Self {
        Self {
            routing_domain_set: RoutingDomainSet::all(),
            dial_info_filter: DialInfoFilter::from(other),
        }
    }
}

impl From<AddressType> for NodeRefFilter {
    fn from(other: AddressType) -> Self {
        Self {
            routing_domain_set: RoutingDomainSet::all(),
            dial_info_filter: DialInfoFilter::from(other),
        }
    }
}
