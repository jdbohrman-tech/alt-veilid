#![cfg_attr(target_arch = "wasm32", expect(dead_code))]

use super::*;

#[derive(Debug)]
enum RoutingDomainChangeLocalNetwork {
    SetLocalNetworks {
        local_networks: Vec<(IpAddr, IpAddr)>,
    },
    Common(RoutingDomainChangeCommon),
}

pub struct RoutingDomainEditorLocalNetwork {
    routing_table: RoutingTable,
    changes: Vec<RoutingDomainChangeLocalNetwork>,
}

impl RoutingDomainEditorLocalNetwork {
    pub(in crate::routing_table) fn new(routing_table: RoutingTable) -> Self {
        Self {
            routing_table: routing_table.clone(),
            changes: Vec::new(),
        }
    }

    pub fn set_local_networks(&mut self, local_networks: Vec<(IpAddr, IpAddr)>) -> &mut Self {
        self.changes
            .push(RoutingDomainChangeLocalNetwork::SetLocalNetworks { local_networks });
        self
    }
}

impl RoutingDomainEditorCommonTrait for RoutingDomainEditorLocalNetwork {
    #[instrument(level = "debug", skip(self))]
    fn clear_dial_info_details(
        &mut self,
        address_type: Option<AddressType>,
        protocol_type: Option<ProtocolType>,
    ) -> &mut Self {
        self.changes.push(RoutingDomainChangeLocalNetwork::Common(
            RoutingDomainChangeCommon::ClearDialInfoDetails {
                address_type,
                protocol_type,
            },
        ));

        self
    }
    #[instrument(level = "debug", skip(self))]
    fn clear_relay_node(&mut self) -> &mut Self {
        self.changes.push(RoutingDomainChangeLocalNetwork::Common(
            RoutingDomainChangeCommon::ClearRelayNode,
        ));
        self
    }
    #[instrument(level = "debug", skip(self))]
    fn set_relay_node(&mut self, relay_node: NodeRef) -> &mut Self {
        self.changes.push(RoutingDomainChangeLocalNetwork::Common(
            RoutingDomainChangeCommon::SetRelayNode { relay_node },
        ));
        self
    }
    #[instrument(level = "debug", skip(self))]
    fn set_relay_node_keepalive(&mut self, ts: Option<Timestamp>) -> &mut Self {
        self.changes.push(RoutingDomainChangeLocalNetwork::Common(
            RoutingDomainChangeCommon::SetRelayNodeKeepalive { ts },
        ));
        self
    }
    #[instrument(level = "debug", skip(self))]
    fn add_dial_info(&mut self, dial_info: DialInfo, class: DialInfoClass) -> &mut Self {
        self.changes.push(RoutingDomainChangeLocalNetwork::Common(
            RoutingDomainChangeCommon::AddDialInfo {
                dial_info_detail: DialInfoDetail {
                    dial_info: dial_info.clone(),
                    class,
                },
            },
        ));
        self
    }
    // #[instrument(level = "debug", skip_all)]
    // fn retain_dial_info<F: Fn(&DialInfo, DialInfoClass) -> bool>(
    //     &mut self,
    //     closure: F,
    // ) -> EyreResult<&mut Self> {
    //     let dids = self.routing_table.dial_info_details(self.routing_domain);
    //     for did in dids {
    //         if !closure(&did.dial_info, did.class) {
    //             self.changes
    //                 .push(RoutingDomainChangePublicInternet::Common(RoutingDomainChange::RemoveDialInfoDetail {
    //                     dial_info_detail: did,
    //                 }));
    //         }
    //     }

    //     Ok(self)
    // }

    #[instrument(level = "debug", skip(self))]
    fn setup_network(
        &mut self,
        outbound_protocols: ProtocolTypeSet,
        inbound_protocols: ProtocolTypeSet,
        address_types: AddressTypeSet,
        capabilities: Vec<Capability>,
    ) -> &mut Self {
        self.changes.push(RoutingDomainChangeLocalNetwork::Common(
            RoutingDomainChangeCommon::SetupNetwork {
                outbound_protocols,
                inbound_protocols,
                address_types,
                capabilities,
            },
        ));
        self
    }

    #[instrument(level = "debug", skip(self))]
    fn set_network_class(&mut self, network_class: Option<NetworkClass>) -> &mut Self {
        self.changes.push(RoutingDomainChangeLocalNetwork::Common(
            RoutingDomainChangeCommon::SetNetworkClass { network_class },
        ));
        self
    }

    #[instrument(level = "debug", skip(self))]
    fn commit(&mut self, pause_tasks: bool) -> SendPinBoxFutureLifetime<'_, bool> {
        Box::pin(async move {
            // No locking if we have nothing to do
            if self.changes.is_empty() {
                return false;
            }
            // Briefly pause routing table ticker while changes are made
            let _tick_guard = if pause_tasks {
                Some(self.routing_table.pause_tasks().await)
            } else {
                None
            };

            // Apply changes
            let mut peer_info_changed = false;

            let mut rti_lock = self.routing_table.inner.write();
            let rti = &mut rti_lock;
            rti.with_local_network_routing_domain_mut(|detail| {
                let old_dial_info_details = detail.dial_info_details().clone();
                let old_relay_node = detail.relay_node();
                let old_outbound_protocols = detail.outbound_protocols();
                let old_inbound_protocols = detail.inbound_protocols();
                let old_address_types = detail.address_types();
                let old_capabilities = detail.capabilities();
                let old_network_class = detail.network_class();

                for change in self.changes.drain(..) {
                    match change {
                        RoutingDomainChangeLocalNetwork::Common(common_change) => {
                            detail.apply_common_change(common_change);
                        }
                        RoutingDomainChangeLocalNetwork::SetLocalNetworks { local_networks } => {
                            detail.set_local_networks(local_networks);
                        }
                    }
                }

                let new_dial_info_details = detail.dial_info_details().clone();
                let new_relay_node = detail.relay_node();
                let new_outbound_protocols = detail.outbound_protocols();
                let new_inbound_protocols = detail.inbound_protocols();
                let new_address_types = detail.address_types();
                let new_capabilities = detail.capabilities();
                let new_network_class = detail.network_class();

                // Compare and see if peerinfo needs republication
                let removed_dial_info = old_dial_info_details
                    .iter()
                    .filter(|di| !new_dial_info_details.contains(di))
                    .collect::<Vec<_>>();
                if !removed_dial_info.is_empty() {
                    info!("[LocalNetwork] removed dial info: {:#?}", removed_dial_info);
                    peer_info_changed = true;
                }
                let added_dial_info = new_dial_info_details
                    .iter()
                    .filter(|di| !old_dial_info_details.contains(di))
                    .collect::<Vec<_>>();
                if !added_dial_info.is_empty() {
                    info!("[LocalNetwork] added dial info: {:#?}", added_dial_info);
                    peer_info_changed = true;
                }
                if let Some(nrn) = new_relay_node {
                    if let Some(orn) = old_relay_node {
                        if !nrn.same_entry(&orn) {
                            info!("[LocalNetwork] change relay: {} -> {}", orn, nrn);
                            peer_info_changed = true;
                        }
                    } else {
                        info!("[LocalNetwork] set relay: {}", nrn);
                        peer_info_changed = true;
                    }
                }
                if old_outbound_protocols != new_outbound_protocols {
                    info!(
                        "[LocalNetwork] changed network: outbound {:?}->{:?}\n",
                        old_outbound_protocols, new_outbound_protocols
                    );
                    peer_info_changed = true;
                }
                if old_inbound_protocols != new_inbound_protocols {
                    info!(
                        "[LocalNetwork] changed network: inbound {:?}->{:?}\n",
                        old_inbound_protocols, new_inbound_protocols,
                    );
                    peer_info_changed = true;
                }
                if old_address_types != new_address_types {
                    info!(
                        "[LocalNetwork] changed network: address types {:?}->{:?}\n",
                        old_address_types, new_address_types,
                    );
                    peer_info_changed = true;
                }
                if old_capabilities != new_capabilities {
                    info!(
                        "[PublicInternet] changed network: capabilities {:?}->{:?}\n",
                        old_capabilities, new_capabilities
                    );
                    peer_info_changed = true;
                }
                if old_network_class != new_network_class {
                    info!(
                        "[LocalNetwork] changed network class: {:?}->{:?}\n",
                        old_network_class, new_network_class
                    );
                    peer_info_changed = true;
                }
            });

            if peer_info_changed {
                // Allow signed node info updates at same timestamp for otherwise dead nodes if our network has changed
                rti.reset_all_updated_since_last_network_change();
            }

            peer_info_changed
        })
    }

    #[instrument(level = "debug", skip(self))]
    fn publish(&mut self) {
        self.routing_table
            .inner
            .write()
            .publish_peer_info(RoutingDomain::LocalNetwork);
    }

    #[instrument(level = "debug", skip(self))]
    fn shutdown(&mut self) -> SendPinBoxFutureLifetime<'_, ()> {
        Box::pin(async move {
            self.clear_dial_info_details(None, None)
                .set_network_class(None)
                .clear_relay_node()
                .commit(true)
                .await;
            self.routing_table
                .inner
                .write()
                .unpublish_peer_info(RoutingDomain::LocalNetwork);
        })
    }
}
