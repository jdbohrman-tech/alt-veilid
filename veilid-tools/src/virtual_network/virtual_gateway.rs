use super::*;

#[derive(Debug)]
pub struct VirtualGateway {
    machine: Machine,
    gateway_id: GatewayId,
}

impl VirtualGateway {
    /////////////////////////////////////////////////////////////
    // Public Interface

    pub async fn find() -> VirtualNetworkResult<Option<Self>> {
        let machine = default_machine().unwrap();
        Self::find_with_machine(machine).await
    }

    pub async fn find_with_machine(machine: Machine) -> VirtualNetworkResult<Option<Self>> {
        machine
            .router_client
            .clone()
            .find_gateway(machine.id)
            .await
            .map(|opt_gateway_id| opt_gateway_id.map(|gateway_id| Self::new(machine, gateway_id)))
    }

    pub async fn get_routed_local_address(
        &self,
        address_type: VirtualAddressType,
    ) -> VirtualNetworkResult<IpAddr> {
        self.machine
            .router_client
            .clone()
            .get_routed_local_address(self.machine.id, address_type)
            .await
    }

    pub async fn get_external_address(&self) -> VirtualNetworkResult<IpAddr> {
        self.machine
            .router_client
            .clone()
            .get_external_address(self.gateway_id)
            .await
    }

    pub async fn add_port(
        &self,
        protocol: VirtualProtocolType,
        external_port: Option<u16>,
        local_address: SocketAddr,
        lease_duration_ms: u32,
        description: String,
    ) -> VirtualNetworkResult<u16> {
        self.machine
            .router_client
            .clone()
            .add_port(
                self.gateway_id,
                protocol,
                external_port,
                local_address,
                lease_duration_ms,
                description,
            )
            .await
    }

    pub async fn remove_port(
        &self,
        protocol: VirtualProtocolType,
        external_port: u16,
    ) -> VirtualNetworkResult<()> {
        self.machine
            .router_client
            .clone()
            .remove_port(self.gateway_id, protocol, external_port)
            .await
    }

    /////////////////////////////////////////////////////////////
    // Private Implementation

    fn new(machine: Machine, gateway_id: GatewayId) -> Self {
        Self {
            machine,
            gateway_id,
        }
    }
}
