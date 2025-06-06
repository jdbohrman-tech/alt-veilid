use super::*;

#[derive(Clone, Default, Debug, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize)]
pub(crate) struct DialInfoTCP {
    pub socket_address: SocketAddress,
}
