use super::*;

/// Parameter for Signal operation
#[derive(Clone, Debug, Serialize, Deserialize)]
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

impl SignalInfo {
    pub fn validate(&self, crypto: Crypto) -> Result<(), RPCError> {
        match self {
            SignalInfo::HolePunch { receipt, peer_info } => {
                if receipt.len() < MIN_RECEIPT_SIZE {
                    return Err(RPCError::protocol("SignalInfo HolePunch receipt too short"));
                }
                if receipt.len() > MAX_RECEIPT_SIZE {
                    return Err(RPCError::protocol("SignalInfo HolePunch receipt too long"));
                }
                peer_info.validate(crypto).map_err(RPCError::protocol)
            }
            SignalInfo::ReverseConnect { receipt, peer_info } => {
                if receipt.len() < MIN_RECEIPT_SIZE {
                    return Err(RPCError::protocol(
                        "SignalInfo ReverseConnect receipt too short",
                    ));
                }
                if receipt.len() > MAX_RECEIPT_SIZE {
                    return Err(RPCError::protocol(
                        "SignalInfo ReverseConnect receipt too long",
                    ));
                }
                peer_info.validate(crypto).map_err(RPCError::protocol)
            }
        }
    }
}
