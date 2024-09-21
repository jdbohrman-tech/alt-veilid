use super::*;

impl NetworkManager {
    // Determine if a local IP address has changed
    // this means we should restart the low level network and and recreate all of our dial info
    // Wait until we have received confirmation from N different peers
    pub fn report_local_network_socket_address(
        &self,
        _socket_address: SocketAddress,
        _flow: Flow,
        _reporting_peer: NodeRef,
    ) {
        // XXX: Nothing here yet.
    }
}
