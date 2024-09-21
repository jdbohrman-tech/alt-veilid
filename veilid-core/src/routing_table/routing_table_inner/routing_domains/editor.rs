use super::*;

pub trait RoutingDomainEditorCommonTrait {
    fn clear_dial_info_details(
        &mut self,
        address_type: Option<AddressType>,
        protocol_type: Option<ProtocolType>,
    ) -> &mut Self;
    fn clear_relay_node(&mut self) -> &mut Self;
    fn set_relay_node(&mut self, relay_node: NodeRef) -> &mut Self;
    fn set_relay_node_keepalive(&mut self, ts: Option<Timestamp>) -> &mut Self;
    #[cfg_attr(target_arch = "wasm32", expect(dead_code))]
    fn add_dial_info(&mut self, dial_info: DialInfo, class: DialInfoClass) -> &mut Self;
    fn setup_network(
        &mut self,
        outbound_protocols: ProtocolTypeSet,
        inbound_protocols: ProtocolTypeSet,
        address_types: AddressTypeSet,
        capabilities: Vec<Capability>,
    ) -> &mut Self;
    fn set_network_class(&mut self, network_class: Option<NetworkClass>) -> &mut Self;
    fn commit(&mut self, pause_tasks: bool) -> SendPinBoxFutureLifetime<'_, bool>;
    fn shutdown(&mut self) -> SendPinBoxFutureLifetime<'_, ()>;
    fn publish(&mut self);
}

pub(super) trait RoutingDomainDetailApplyCommonChange {
    /// Make a change from the routing domain editor
    fn apply_common_change(&mut self, change: RoutingDomainChangeCommon);
}

impl<T: RoutingDomainDetailCommonAccessors> RoutingDomainDetailApplyCommonChange for T {
    /// Make a change from the routing domain editor
    fn apply_common_change(&mut self, change: RoutingDomainChangeCommon) {
        match change {
            RoutingDomainChangeCommon::ClearDialInfoDetails {
                address_type,
                protocol_type,
            } => {
                self.common_mut()
                    .clear_dial_info_details(address_type, protocol_type);
            }

            RoutingDomainChangeCommon::ClearRelayNode => {
                self.common_mut().set_relay_node(None);
            }

            RoutingDomainChangeCommon::SetRelayNode { relay_node } => {
                self.common_mut().set_relay_node(Some(relay_node.clone()))
            }

            RoutingDomainChangeCommon::SetRelayNodeKeepalive { ts } => {
                self.common_mut().set_relay_node_last_keepalive(ts);
            }
            RoutingDomainChangeCommon::AddDialInfo { dial_info_detail } => {
                if !self.ensure_dial_info_is_valid(&dial_info_detail.dial_info) {
                    return;
                }

                self.common_mut()
                    .add_dial_info_detail(dial_info_detail.clone());
            }
            // RoutingDomainChange::RemoveDialInfoDetail { dial_info_detail } => {
            //     self.common
            //         .remove_dial_info_detail(dial_info_detail.clone());
            // }
            RoutingDomainChangeCommon::SetupNetwork {
                outbound_protocols,
                inbound_protocols,
                address_types,
                capabilities,
            } => {
                self.common_mut().setup_network(
                    outbound_protocols,
                    inbound_protocols,
                    address_types,
                    capabilities.clone(),
                );
            }
            RoutingDomainChangeCommon::SetNetworkClass { network_class } => {
                self.common_mut().set_network_class(network_class);
            }
        }
    }
}

#[derive(Debug)]
pub(super) enum RoutingDomainChangeCommon {
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
    AddDialInfo {
        dial_info_detail: DialInfoDetail,
    },
    // #[cfg_attr(target_arch = "wasm32", expect(dead_code))]
    // RemoveDialInfoDetail {
    //     dial_info_detail: DialInfoDetail,
    // },
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
