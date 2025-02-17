use super::*;

pub trait RoutingDomainEditorCommonTrait {
    fn clear_dial_info_details(
        &mut self,
        address_type: Option<AddressType>,
        protocol_type: Option<ProtocolType>,
    ) -> &mut Self;
    fn set_relay_node(&mut self, relay_node: Option<NodeRef>) -> &mut Self;
    #[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), expect(dead_code))]
    fn add_dial_info(&mut self, dial_info: DialInfo, class: DialInfoClass) -> &mut Self;
    fn setup_network(
        &mut self,
        outbound_protocols: ProtocolTypeSet,
        inbound_protocols: ProtocolTypeSet,
        address_types: AddressTypeSet,
        capabilities: Vec<Capability>,
        confirmed: bool,
    ) -> &mut Self;
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

            RoutingDomainChangeCommon::SetRelayNode { relay_node } => {
                self.common_mut().set_relay_node(relay_node)
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
                confirmed,
            } => {
                self.common_mut().setup_network(
                    outbound_protocols,
                    inbound_protocols,
                    address_types,
                    capabilities.clone(),
                    confirmed,
                );
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
    SetRelayNode {
        relay_node: Option<NodeRef>,
    },
    AddDialInfo {
        dial_info_detail: DialInfoDetail,
    },
    // #[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), expect(dead_code))]
    // RemoveDialInfoDetail {
    //     dial_info_detail: DialInfoDetail,
    // },
    SetupNetwork {
        outbound_protocols: ProtocolTypeSet,
        inbound_protocols: ProtocolTypeSet,
        address_types: AddressTypeSet,
        capabilities: Vec<Capability>,
        confirmed: bool,
    },
}
