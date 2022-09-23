use super::*;

enum RoutingDomainChange {
    ClearDialInfoDetails,
    ClearRelayNode,
    SetRelayNode { relay_node: NodeRef },
    AddDialInfoDetail { dial_info_detail: DialInfoDetail },
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
    pub async fn commit(self) {
        let mut changed = false;
        {
            let mut inner = self.routing_table.inner.write();
            let inner = &mut *inner;
            let node_id = inner.node_id;

            RoutingTable::with_routing_domain_mut(inner, self.routing_domain, |detail| {
                for change in self.changes {
                    match change {
                        RoutingDomainChange::ClearDialInfoDetails => {
                            debug!("[{:?}] cleared dial info details", self.routing_domain);
                            detail.clear_dial_info_details();
                            changed = true;
                        }
                        RoutingDomainChange::ClearRelayNode => {
                            debug!("[{:?}] cleared relay node", self.routing_domain);
                            detail.set_relay_node(None);
                            changed = true;
                        }
                        RoutingDomainChange::SetRelayNode { relay_node } => {
                            debug!("[{:?}] set relay node: {}", self.routing_domain, relay_node);
                            detail.set_relay_node(Some(relay_node));
                            changed = true;
                        }
                        RoutingDomainChange::AddDialInfoDetail { dial_info_detail } => {
                            debug!(
                                "[{:?}] add dial info detail: {:?}",
                                self.routing_domain, dial_info_detail
                            );
                            detail.add_dial_info_detail(dial_info_detail.clone());

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
                    }
                }
            });
            if changed {
                RoutingTable::reset_all_seen_our_node_info(inner, self.routing_domain);
                RoutingTable::reset_all_updated_since_last_network_change(inner);
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
