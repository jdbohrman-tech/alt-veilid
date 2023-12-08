use super::*;

#[derive(Debug)]
enum RoutingDomainChange {
    ClearDialInfoDetails {
        address_type: Option<AddressType>,
        protocol_type: Option<ProtocolType>,
    },
    ClearRelayNode,
    SetRelayNode {
        relay_node: NodeRef,
    },
    SetRelayNodeKeepalive {
        ts: Option<Timestamp>,
    },
    #[cfg_attr(target_arch = "wasm32", allow(dead_code))]
    AddDialInfoDetail {
        dial_info_detail: DialInfoDetail,
    },
    SetupNetwork {
        outbound_protocols: ProtocolTypeSet,
        inbound_protocols: ProtocolTypeSet,
        address_types: AddressTypeSet,
        capabilities: Vec<Capability>,
    },
    SetNetworkClass {
        network_class: Option<NetworkClass>,
    },
}

pub(crate) struct RoutingDomainEditor {
    routing_table: RoutingTable,
    routing_domain: RoutingDomain,
    changes: Vec<RoutingDomainChange>,
}

impl RoutingDomainEditor {
    pub(super) fn new(routing_table: RoutingTable, routing_domain: RoutingDomain) -> Self {
        Self {
            routing_table,
            routing_domain,
            changes: Vec::new(),
        }
    }

    #[instrument(level = "debug", skip(self))]
    pub fn clear_dial_info_details(
        &mut self,
        address_type: Option<AddressType>,
        protocol_type: Option<ProtocolType>,
    ) -> &mut Self {
        self.changes
            .push(RoutingDomainChange::ClearDialInfoDetails {
                address_type,
                protocol_type,
            });
        self
    }
    #[instrument(level = "debug", skip(self))]
    pub fn clear_relay_node(&mut self) -> &mut Self {
        self.changes.push(RoutingDomainChange::ClearRelayNode);
        self
    }
    #[instrument(level = "debug", skip(self))]
    pub fn set_relay_node(&mut self, relay_node: NodeRef) -> &mut Self {
        self.changes
            .push(RoutingDomainChange::SetRelayNode { relay_node });
        self
    }
    #[instrument(level = "debug", skip(self))]
    pub fn set_relay_node_keepalive(&mut self, ts: Option<Timestamp>) -> &mut Self {
        self.changes
            .push(RoutingDomainChange::SetRelayNodeKeepalive { ts });
        self
    }
    #[instrument(level = "debug", skip(self))]
    pub fn register_dial_info(
        &mut self,
        dial_info: DialInfo,
        class: DialInfoClass,
    ) -> EyreResult<&mut Self> {
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

        Ok(self)
    }
    #[instrument(level = "debug", skip(self))]
    pub fn setup_network(
        &mut self,
        outbound_protocols: ProtocolTypeSet,
        inbound_protocols: ProtocolTypeSet,
        address_types: AddressTypeSet,
        capabilities: Vec<Capability>,
    ) -> &mut Self {
        self.changes.push(RoutingDomainChange::SetupNetwork {
            outbound_protocols,
            inbound_protocols,
            address_types,
            capabilities,
        });
        self
    }

    #[instrument(level = "debug", skip(self))]
    pub fn set_network_class(&mut self, network_class: Option<NetworkClass>) -> &mut Self {
        self.changes
            .push(RoutingDomainChange::SetNetworkClass { network_class });
        self
    }

    #[instrument(level = "debug", skip(self))]
    pub async fn commit(&mut self, pause_tasks: bool) {
        // No locking if we have nothing to do
        if self.changes.is_empty() {
            return;
        }

        // Briefly pause routing table ticker while changes are made
        let _tick_guard = if pause_tasks {
            Some(self.routing_table.pause_tasks().await)
        } else {
            None
        };

        // Apply changes
        log_rtab!("[{:?}] COMMIT: {:?}", self.routing_domain, self.changes);
        let mut peer_info_changed = false;
        {
            let mut inner = self.routing_table.inner.write();
            inner.with_routing_domain_mut(self.routing_domain, |detail| {
                for change in self.changes.drain(..) {
                    match change {
                        RoutingDomainChange::ClearDialInfoDetails {
                            address_type,
                            protocol_type,
                        } => {
                            if address_type.is_some() || protocol_type.is_some() {
                                info!(
                                    "[{:?}] cleared dial info: {}:{}",
                                    self.routing_domain,
                                    address_type
                                        .map(|at| format!("{:?}", at))
                                        .unwrap_or("---".to_string()),
                                    protocol_type
                                        .map(|at| format!("{:?}", at))
                                        .unwrap_or("---".to_string()),
                                );
                            } else {
                                info!("[{:?}] cleared all dial info", self.routing_domain);
                            }
                            detail
                                .common_mut()
                                .clear_dial_info_details(address_type, protocol_type);
                            peer_info_changed = true;
                        }
                        RoutingDomainChange::ClearRelayNode => {
                            info!("[{:?}] cleared relay node", self.routing_domain);
                            detail.common_mut().set_relay_node(None);
                            peer_info_changed = true;
                        }
                        RoutingDomainChange::SetRelayNode { relay_node } => {
                            info!("[{:?}] set relay node: {}", self.routing_domain, relay_node);
                            detail.common_mut().set_relay_node(Some(relay_node.clone()));
                            peer_info_changed = true;
                        }
                        RoutingDomainChange::SetRelayNodeKeepalive { ts } => {
                            trace!("[{:?}] relay node keepalive: {:?}", self.routing_domain, ts);
                            detail.common_mut().set_relay_node_last_keepalive(ts);
                        }
                        RoutingDomainChange::AddDialInfoDetail { dial_info_detail } => {
                            info!(
                                "[{:?}] dial info: {:?}:{}",
                                self.routing_domain,
                                dial_info_detail.class,
                                dial_info_detail.dial_info
                            );
                            detail
                                .common_mut()
                                .add_dial_info_detail(dial_info_detail.clone());

                            peer_info_changed = true;
                        }
                        RoutingDomainChange::SetupNetwork {
                            outbound_protocols,
                            inbound_protocols,
                            address_types,
                            capabilities,
                        } => {
                            let old_outbound_protocols = detail.common().outbound_protocols();
                            let old_inbound_protocols = detail.common().inbound_protocols();
                            let old_address_types = detail.common().address_types();
                            let old_capabilities = detail.common().capabilities();

                            let this_changed = old_outbound_protocols != outbound_protocols
                                || old_inbound_protocols != inbound_protocols
                                || old_address_types != address_types
                                || old_capabilities != *capabilities;

                            if this_changed {
                                info!(
                                    "[{:?}] setup network: outbound {:?} inbound {:?} address types {:?} capabilities {:?}",
                                    self.routing_domain,
                                    outbound_protocols,
                                    inbound_protocols,
                                    address_types,
                                    capabilities
                                );

                                detail.common_mut().setup_network(
                                    outbound_protocols,
                                    inbound_protocols,
                                    address_types,
                                    capabilities.clone(),
                                );
                                peer_info_changed = true;
                            }
                        }
                        RoutingDomainChange::SetNetworkClass { network_class } => {
                            let old_network_class = detail.common().network_class();

                            let this_changed = old_network_class != network_class;
                            if this_changed {
                                if let Some(network_class) = network_class {
                                    info!(
                                        "[{:?}] set network class: {:?}",
                                        self.routing_domain, network_class,
                                    );
                                } else {
                                    info!("[{:?}] cleared network class", self.routing_domain,);
                                }
                                detail.common_mut().set_network_class(network_class);
                                peer_info_changed = true;
                            }
                        }
                    }
                }
            });
            if peer_info_changed {
                // Allow signed node info updates at same timestamp for otherwise dead nodes if our network has changed
                inner.reset_all_updated_since_last_network_change();
            }
        }
        // Clear the routespecstore cache if our PublicInternet dial info has changed
        if peer_info_changed && self.routing_domain == RoutingDomain::PublicInternet {
            let rss = self.routing_table.route_spec_store();
            rss.reset();
        }
    }
}
