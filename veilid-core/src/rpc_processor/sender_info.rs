use super::*;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Ord, Eq, Hash, Default)]
pub(crate) struct SenderInfo {
    pub socket_address: SocketAddress,
}
