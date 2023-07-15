use super::*;

#[derive(Clone, Default, Debug, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize)]
pub struct DialInfoWS {
    pub socket_address: SocketAddress,
    pub request: String,
}
