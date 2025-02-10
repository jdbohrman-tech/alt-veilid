use super::*;
use serde::*;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct VirtualUdpOptions {
    only_v6: bool,
    reuse_address_port: bool,
}

#[derive(Debug)]
pub struct VirtualUdpSocket {
    machine: Machine,
    socket_id: SocketId,
    local_address: SocketAddr,
}

impl VirtualUdpSocket {
    /////////////////////////////////////////////////////////////
    // Public Interface

    pub async fn bind(
        opt_local_address: Option<SocketAddr>,
        options: VirtualUdpOptions,
    ) -> VirtualNetworkResult<Self> {
        let machine = default_machine().unwrap();
        Self::bind_with_machine(machine, opt_local_address, options).await
    }

    pub async fn bind_with_machine(
        machine: Machine,
        opt_local_address: Option<SocketAddr>,
        options: VirtualUdpOptions,
    ) -> VirtualNetworkResult<Self> {
        machine
            .router_client
            .clone()
            .udp_bind(machine.id, opt_local_address, options)
            .await
            .map(|(socket_id, local_address)| Self::new(machine, socket_id, local_address))
    }

    pub async fn send_to(&self, buf: &[u8], target: SocketAddr) -> VirtualNetworkResult<usize> {
        self.machine
            .router_client
            .clone()
            .send_to(self.machine.id, self.socket_id, target, buf.to_vec())
            .await
    }

    pub async fn recv_from(&self, buf: &mut [u8]) -> VirtualNetworkResult<(usize, SocketAddr)> {
        let (v, addr) = self
            .machine
            .router_client
            .clone()
            .recv_from(self.machine.id, self.socket_id, buf.len())
            .await?;

        let len = usize::min(buf.len(), v.len());
        buf[0..len].copy_from_slice(&v[0..len]);

        Ok((len, addr))
    }

    pub fn local_addr(&self) -> VirtualNetworkResult<SocketAddr> {
        Ok(self.local_address)
    }

    /////////////////////////////////////////////////////////////
    // Private Implementation

    fn new(machine: Machine, socket_id: SocketId, local_address: SocketAddr) -> Self {
        Self {
            machine,
            socket_id,
            local_address,
        }
    }
}

impl Drop for VirtualUdpSocket {
    fn drop(&mut self) {
        self.machine
            .router_client
            .drop_udp_socket(self.machine.id, self.socket_id);
    }
}
