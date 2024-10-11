mod editor;

pub use editor::*;

use super::*;

/// Local Network routing domain internals
#[derive(Debug)]
pub struct LocalNetworkRoutingDomainDetail {
    /// The local networks this domain will communicate with
    local_networks: Vec<(IpAddr, IpAddr)>,
    /// Common implementation for all routing domains
    common: RoutingDomainDetailCommon,
    /// Published peer info for this routing domain
    published_peer_info: Mutex<Option<Arc<PeerInfo>>>,
}

impl Default for LocalNetworkRoutingDomainDetail {
    fn default() -> Self {
        Self {
            local_networks: Default::default(),
            common: RoutingDomainDetailCommon::new(RoutingDomain::LocalNetwork),
            published_peer_info: Default::default(),
        }
    }
}

impl LocalNetworkRoutingDomainDetail {
    #[expect(dead_code)]
    pub fn local_networks(&self) -> Vec<(IpAddr, IpAddr)> {
        self.local_networks.clone()
    }

    pub fn set_local_networks(&mut self, mut local_networks: Vec<(IpAddr, IpAddr)>) -> bool {
        local_networks.sort();
        if local_networks == self.local_networks {
            return false;
        }
        self.local_networks = local_networks;
        true
    }
}

impl RoutingDomainDetailCommonAccessors for LocalNetworkRoutingDomainDetail {
    fn common(&self) -> &RoutingDomainDetailCommon {
        &self.common
    }
    fn common_mut(&mut self) -> &mut RoutingDomainDetailCommon {
        &mut self.common
    }
}

impl RoutingDomainDetail for LocalNetworkRoutingDomainDetail {
    fn routing_domain(&self) -> RoutingDomain {
        RoutingDomain::LocalNetwork
    }

    fn network_class(&self) -> Option<NetworkClass> {
        self.common.network_class()
    }
    fn outbound_protocols(&self) -> ProtocolTypeSet {
        self.common.outbound_protocols()
    }
    fn inbound_protocols(&self) -> ProtocolTypeSet {
        self.common.inbound_protocols()
    }
    fn address_types(&self) -> AddressTypeSet {
        self.common.address_types()
    }
    fn capabilities(&self) -> Vec<Capability> {
        self.common.capabilities()
    }
    fn requires_relay(&self) -> Option<RelayKind> {
        self.common.requires_relay()
    }
    fn relay_node(&self) -> Option<FilteredNodeRef> {
        self.common.relay_node()
    }
    fn relay_node_last_keepalive(&self) -> Option<Timestamp> {
        self.common.relay_node_last_keepalive()
    }
    fn dial_info_details(&self) -> &Vec<DialInfoDetail> {
        self.common.dial_info_details()
    }
    fn inbound_dial_info_filter(&self) -> DialInfoFilter {
        self.common.inbound_dial_info_filter()
    }
    fn outbound_dial_info_filter(&self) -> DialInfoFilter {
        self.common.outbound_dial_info_filter()
    }

    fn get_peer_info(&self, rti: &RoutingTableInner) -> Arc<PeerInfo> {
        self.common.get_peer_info(rti)
    }

    fn get_published_peer_info(&self) -> Option<Arc<PeerInfo>> {
        (*self.published_peer_info.lock()).clone()
    }

    fn can_contain_address(&self, address: Address) -> bool {
        let ip = address.ip_addr();
        for localnet in &self.local_networks {
            if ipaddr_in_network(ip, localnet.0, localnet.1) {
                return true;
            }
        }
        false
    }

    fn refresh(&self) {
        self.common.clear_cache();
    }

    fn publish_peer_info(&self, rti: &RoutingTableInner) -> bool {
        let peer_info = {
            let pi = self.get_peer_info(rti);

            // If the network class is not yet determined, don't publish
            if pi.signed_node_info().node_info().network_class() == NetworkClass::Invalid {
                log_rtab!(debug "[LocalNetwork] Not publishing peer info with invalid network class");
                return false;
            }

            // If we need a relay and we don't have one, don't publish yet
            if let Some(_relay_kind) = self.requires_relay() {
                if pi.signed_node_info().relay_ids().is_empty() {
                    log_rtab!(debug "[LocalNetwork] Not publishing peer info that wants relay until we have a relay");
                    return false;
                }
            }

            // Don't publish if the peer info hasnt changed from our previous publication
            let mut ppi_lock = self.published_peer_info.lock();
            if let Some(old_peer_info) = &*ppi_lock {
                if pi.equivalent(old_peer_info) {
                    log_rtab!(debug "[LocalNetwork] Not publishing peer info because it is equivalent");
                    return false;
                }
            }

            log_rtab!(debug "[LocalNetwork] Published new peer info: {}", pi);
            *ppi_lock = Some(pi.clone());

            pi
        };

        rti.unlocked_inner
            .network_manager()
            .report_peer_info_change(peer_info);

        true
    }

    fn unpublish_peer_info(&self) {
        let mut ppi_lock = self.published_peer_info.lock();
        log_rtab!(debug "[LocalNetwork] Unpublished peer info");
        *ppi_lock = None;
    }

    fn ensure_dial_info_is_valid(&self, dial_info: &DialInfo) -> bool {
        let address = dial_info.socket_address().address();
        let can_contain_address = self.can_contain_address(address);

        if !can_contain_address {
            log_rtab!(debug "[LocalNetwork] can not add dial info to this routing domain: {:?}", dial_info);
            return false;
        }
        if !dial_info.is_valid() {
            log_rtab!(debug
                "shouldn't be registering invalid addresses: {:?}",
                dial_info
            );
            return false;
        }
        true
    }

    fn get_contact_method(
        &self,
        _rti: &RoutingTableInner,
        peer_a: Arc<PeerInfo>,
        peer_b: Arc<PeerInfo>,
        dial_info_filter: DialInfoFilter,
        sequencing: Sequencing,
        dif_sort: Option<Arc<DialInfoDetailSort>>,
    ) -> ContactMethod {
        // Get the nodeinfos for convenience
        let node_a = peer_a.signed_node_info().node_info();
        let node_b = peer_b.signed_node_info().node_info();

        // Get the node ids that would be used between these peers
        let cck = common_crypto_kinds(&peer_a.node_ids().kinds(), &peer_b.node_ids().kinds());
        let Some(_best_ck) = cck.first().copied() else {
            // No common crypto kinds between these nodes, can't contact
            return ContactMethod::Unreachable;
        };

        if let Some(target_did) = first_filtered_dial_info_detail_between_nodes(
            node_a,
            node_b,
            &dial_info_filter,
            sequencing,
            dif_sort,
        ) {
            return ContactMethod::Direct(target_did.dial_info);
        }

        ContactMethod::Unreachable
    }

    fn set_relay_node_last_keepalive(&mut self, ts: Option<Timestamp>) {
        self.common.set_relay_node_last_keepalive(ts);
    }
}
