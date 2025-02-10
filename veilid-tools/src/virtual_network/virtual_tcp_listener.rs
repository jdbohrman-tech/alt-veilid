use super::*;

#[derive(Debug)]
pub struct VirtualTcpListener {
    pub(super) machine: Machine,
    pub(super) socket_id: SocketId,
    pub(super) local_address: SocketAddr,
}

impl VirtualTcpListener {
    /////////////////////////////////////////////////////////////
    // Public Interface

    pub async fn bind(
        opt_local_address: Option<SocketAddr>,
        options: VirtualTcpOptions,
    ) -> VirtualNetworkResult<Self> {
        let machine = default_machine().unwrap();
        Self::bind_with_machine(machine, opt_local_address, options).await
    }

    pub async fn bind_with_machine(
        machine: Machine,
        opt_local_address: Option<SocketAddr>,
        options: VirtualTcpOptions,
    ) -> VirtualNetworkResult<Self> {
        machine
            .router_client
            .clone()
            .tcp_bind(machine.id, opt_local_address, options)
            .await
            .map(|(socket_id, local_address)| Self::new(machine, socket_id, local_address))
    }

    pub async fn accept(&self) -> VirtualNetworkResult<(VirtualTcpStream, SocketAddr)> {
        self.machine
            .router_client
            .clone()
            .tcp_accept(self.machine.id, self.socket_id)
            .await
            .map(|v| {
                (
                    VirtualTcpStream::new(self.machine.clone(), v.0, self.local_address, v.1),
                    v.1,
                )
            })
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

impl Drop for VirtualTcpListener {
    fn drop(&mut self) {
        self.machine
            .router_client
            .drop_tcp_listener(self.machine.id, self.socket_id);
    }
}
