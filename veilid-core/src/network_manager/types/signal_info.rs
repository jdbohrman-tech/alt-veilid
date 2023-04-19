use super::*;

/// Parameter for Signal operation
#[derive(Clone, Debug, Serialize, Deserialize, RkyvArchive, RkyvSerialize, RkyvDeserialize)]
#[archive_attr(repr(u8), derive(CheckBytes))]
pub enum SignalInfo {
    /// UDP Hole Punch Request
    HolePunch {
        /// /// Receipt to be returned after the hole punch
        receipt: Vec<u8>,
        /// Sender's peer info
        peer_info: PeerInfo,
    },
    /// Reverse Connection Request
    ReverseConnect {
        /// Receipt to be returned by the reverse connection
        receipt: Vec<u8>,
        /// Sender's peer info
        peer_info: PeerInfo,
    },
    // XXX: WebRTC
}
