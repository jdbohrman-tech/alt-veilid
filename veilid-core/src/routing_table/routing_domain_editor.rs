use super::*;

enum RoutingDomainChange {
    ClearDialInfoDetails,
    ClearRelayNode,
    SetRelayNode {
        relay_node: NodeRef,
    },
    AddDialInfoDetail {
        dial_info_detail: DialInfoDetail,
    },
    SetupNode {
        node_id: DHTKey,
        node_id_secret: DHTKeySecret,
    },
    SetupNetwork {
        outbound_protocols: ProtocolTypeSet,
        inbound_protocols: ProtocolTypeSet,
        address_types: AddressTypeSet,
    },
    SetNetworkClass {
        network_class: Option<NetworkClass>,
    },
}

pub struct RoutingDomainEditor {
    routing_table: RoutingTable,
    routing_domain: RoutingDomain,
    changes: Vec<RoutingDomainChange>,
    send_node_info_updates: bool,
}

impl RoutingDomainEditor {
    pub(super) fn new(routing_table: RoutingTable, routing_domain: RoutingDomain) -> Self {
        Self {
            routing_table,
            routing_domain,
            changes: Vec::new(),
            send_node_info_updates: true,
        }
    }
    #[instrument(level = "debug", skip(self))]
    pub fn disable_node_info_updates(&mut self) {
        self.send_node_info_updates = false;
    }

    #[instrument(level = "debug", skip(self))]
    pub fn clear_dial_info_details(&mut self) {
        self.changes.push(RoutingDomainChange::ClearDialInfoDetails);
    }
    #[instrument(level = "debug", skip(self))]
    pub fn clear_relay_node(&mut self) {
        self.changes.push(RoutingDomainChange::ClearRelayNode);
    }
    #[instrument(level = "debug", skip(self))]
    pub fn set_relay_node(&mut self, relay_node: NodeRef) {
        self.changes
            .push(RoutingDomainChange::SetRelayNode { relay_node })
    }
    #[instrument(level = "debug", skip(self), err)]
    pub fn register_dial_info(
        &mut self,
        dial_info: DialInfo,
        class: DialInfoClass,
    ) -> EyreResult<()> {
        if !self
            .routing_table
            .ensure_dial_info_is_valid(self.routing_domain, &dial_info)
        {
            return Err(eyre!(
                "dial info '{}' is not valid in routing domain '{:?}'",
                dial_info,
                self.routing_domain
            ));
        }

        self.changes.push(RoutingDomainChange::AddDialInfoDetail {
            dial_info_detail: DialInfoDetail {
                dial_info: dial_info.clone(),
                class,
            },
        });

        Ok(())
    }
    #[instrument(level = "debug", skip(self))]
    pub fn setup_node(&mut self, node_id: DHTKey, node_id_secret: DHTKeySecret) {
        self.changes.push(RoutingDomainChange::SetupNode {
            node_id,
            node_id_secret,
        })
    }
    #[instrument(level = "debug", skip(self))]
    pub fn setup_network(
        &mut self,
        outbound_protocols: ProtocolTypeSet,
        inbound_protocols: ProtocolTypeSet,
        address_types: AddressTypeSet,
    ) {
        self.changes.push(RoutingDomainChange::SetupNetwork {
            outbound_protocols,
            inbound_protocols,
            address_types,
        })
    }

    #[instrument(level = "debug", skip(self))]
    pub fn set_network_class(&mut self, network_class: Option<NetworkClass>) {
        self.changes
            .push(RoutingDomainChange::SetNetworkClass { network_class })
    }

    #[instrument(level = "debug", skip(self))]
    pub async fn commit(self) {
        // No locking if we have nothing to do
        if self.changes.is_empty() {
            return;
        }

        let mut changed = false;
        {
            let node_id = self.routing_table.node_id();

            let mut inner = self.routing_table.inner.write();
            inner.with_routing_domain_mut(self.routing_domain, |detail| {
                for change in self.changes {
                    match change {
                        RoutingDomainChange::ClearDialInfoDetails => {
                            debug!("[{:?}] cleared dial info details", self.routing_domain);
                            detail.common_mut().clear_dial_info_details();
                            changed = true;
                        }
                        RoutingDomainChange::ClearRelayNode => {
                            debug!("[{:?}] cleared relay node", self.routing_domain);
                            detail.common_mut().set_relay_node(None);
                            changed = true;
                        }
                        RoutingDomainChange::SetRelayNode { relay_node } => {
                            debug!("[{:?}] set relay node: {}", self.routing_domain, relay_node);
                            detail.common_mut().set_relay_node(Some(relay_node));
                            changed = true;
                        }
                        RoutingDomainChange::AddDialInfoDetail { dial_info_detail } => {
                            debug!(
                                "[{:?}] add dial info detail: {:?}",
                                self.routing_domain, dial_info_detail
                            );
                            detail
                                .common_mut()
                                .add_dial_info_detail(dial_info_detail.clone());

                            info!(
                                "{:?} Dial Info: {}",
                                self.routing_domain,
                                NodeDialInfo {
                                    node_id: NodeId::new(node_id),
                                    dial_info: dial_info_detail.dial_info
                                }
                                .to_string(),
                            );
                            changed = true;
                        }
                        RoutingDomainChange::SetupNode {
                            node_id,
                            node_id_secret,
                        } => {
                            debug!(
                                "[{:?}] setup node: {}",
                                self.routing_domain,
                                node_id.encode()
                            );
                            detail.common_mut().setup_node(node_id, node_id_secret);
                            changed = true;
                        }
                        RoutingDomainChange::SetupNetwork {
                            outbound_protocols,
                            inbound_protocols,
                            address_types,
                        } => {
                            let old_outbound_protocols = detail.common().outbound_protocols();
                            let old_inbound_protocols = detail.common().inbound_protocols();
                            let old_address_types = detail.common().address_types();

                            let this_changed = old_outbound_protocols != outbound_protocols
                                || old_inbound_protocols != inbound_protocols
                                || old_address_types != address_types;

                            debug!(
                                "[{:?}] setup network: {:?} {:?} {:?}",
                                self.routing_domain,
                                outbound_protocols,
                                inbound_protocols,
                                address_types
                            );

                            detail.common_mut().setup_network(
                                outbound_protocols,
                                inbound_protocols,
                                address_types,
                            );
                            if this_changed {
                                changed = true;
                            }
                        }
                        RoutingDomainChange::SetNetworkClass { network_class } => {
                            let old_network_class = detail.common().network_class();

                            let this_changed = old_network_class != network_class;

                            debug!(
                                "[{:?}] set network class: {:?}",
                                self.routing_domain, network_class,
                            );

                            detail.common_mut().set_network_class(network_class);
                            if this_changed {
                                changed = true;
                            }
                        }
                    }
                }
                if changed {
                    detail.common_mut().clear_cache()
                }
            });
            if changed {
                inner.reset_all_seen_our_node_info(self.routing_domain);
                inner.reset_all_updated_since_last_network_change();
            }
        }
        if changed && self.send_node_info_updates {
            let network_manager = self.routing_table.unlocked_inner.network_manager.clone();
            network_manager
                .send_node_info_updates(self.routing_domain, true)
                .await;
        }
    }
}
