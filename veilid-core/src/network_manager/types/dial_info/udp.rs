use super::*;

#[derive(Clone, Default, Debug, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize)]
pub(crate) struct DialInfoUDP {
    pub socket_address: SocketAddress,
}
