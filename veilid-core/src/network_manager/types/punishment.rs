use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PunishmentReason {
    // IP-level punishments
    FailedToDecryptEnvelopeBody,
    FailedToDecodeEnvelope,
    ShortPacket,
    InvalidFraming,
    // Node-level punishments
    FailedToDecodeOperation,
    WrongSenderPeerInfo,
    FailedToVerifySenderPeerInfo,
    FailedToRegisterSenderPeerInfo,
    // Route-level punishments
    // FailedToDecodeRoutedMessage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Punishment {
    pub reason: PunishmentReason,
    pub timestamp: Timestamp,
}
