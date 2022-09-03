use super::*;

/// General trait for all routing domains
pub trait RoutingDomainDetail {
    fn can_contain_address(&self, address: Address) -> bool;
    fn relay_node(&self) -> Option<NodeRef>;
    fn set_relay_node(&mut self, opt_relay_node: Option<NodeRef>);
    fn dial_info_details(&self) -> &Vec<DialInfoDetail>;
    fn clear_dial_info_details(&mut self);
    fn add_dial_info_detail(&mut self, did: DialInfoDetail);
}

/// Public Internet routing domain internals
#[derive(Debug, Default)]
pub struct PublicInternetRoutingDomainDetail {
    /// An optional node we relay through for this domain
    relay_node: Option<NodeRef>,
    /// The dial infos on this domain we can be reached by
    dial_info_details: Vec<DialInfoDetail>,
}

impl RoutingDomainDetail for PublicInternetRoutingDomainDetail {
    fn can_contain_address(&self, address: Address) -> bool {
        address.is_global()
    }
    fn relay_node(&self) -> Option<NodeRef> {
        self.relay_node.clone()
    }
    fn set_relay_node(&mut self, opt_relay_node: Option<NodeRef>) {
        self.relay_node = opt_relay_node
            .map(|nr| nr.filtered_clone(NodeRefFilter::new().with_routing_domain(PublicInternet)))
    }
    fn dial_info_details(&self) -> &Vec<DialInfoDetail> {
        &self.dial_info_details
    }
    fn clear_dial_info_details(&mut self) {
        self.dial_info_details.clear();
    }
    fn add_dial_info_detail(&mut self, did: DialInfoDetail) {
        self.dial_info_details.push(did);
        self.dial_info_details.sort();
    }
}

/// Local Network routing domain internals
#[derive(Debug, Default)]
pub struct LocalInternetRoutingDomainDetail {
    /// An optional node we relay through for this domain
    relay_node: Option<NodeRef>,
    /// The dial infos on this domain we can be reached by
    dial_info_details: Vec<DialInfoDetail>,
    /// The local networks this domain will communicate with
    local_networks: Vec<(IpAddr, IpAddr)>,
}

impl LocalInternetRoutingDomainDetail {
    pub fn set_local_networks(&mut self, local_networks: Vec<(IpAddr, IpAddr)>) -> bool {
        local_networks.sort();
        if local_networks == self.local_networks {
            return false;
        }
        self.local_networks = local_networks;
        true
    }
}

impl RoutingDomainDetail for LocalInternetRoutingDomainDetail {
    fn can_contain_address(&self, address: Address) -> bool {
        let ip = address.to_ip_addr();
        for localnet in self.local_networks {
            if ipaddr_in_network(ip, localnet.0, localnet.1) {
                return true;
            }
        }
        false
    }
    fn relay_node(&self) -> Option<NodeRef> {
        self.relay_node.clone()
    }
    fn set_relay_node(&mut self, opt_relay_node: Option<NodeRef>) {
        self.relay_node = opt_relay_node
            .map(|nr| nr.filtered_clone(NodeRefFilter::new().with_routing_domain(LocalNetwork)));
    }
    fn dial_info_details(&self) -> &Vec<DialInfoDetail> {
        &self.dial_info_details
    }
    fn clear_dial_info_details(&mut self) {
        self.dial_info_details.clear();
    }
    fn add_dial_info_detail(&mut self, did: DialInfoDetail) {
        self.dial_info_details.push(did);
        self.dial_info_details.sort();
    }
}
