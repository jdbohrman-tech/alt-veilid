#![allow(dead_code)]
#![allow(clippy::absurd_extreme_comparisons)]
use super::*;
use crate::xx::*;
use crate::*;
use core::convert::TryInto;
use crate::routing_table::VersionRange;

// #[repr(C, packed)]
// struct EnvelopeHeader {
//     // Size is at least 8 bytes. Depending on the version specified, the size may vary and should be case to the appropriate struct
//     magic: [u8; 4],              // 0x00: 0x56 0x4C 0x49 0x44 ("VLID")
//     version: u8,                 // 0x04: 0 = EnvelopeV0
//     min_version: u8,             // 0x05: 0 = EnvelopeV0
//     max_version: u8,             // 0x06: 0 = EnvelopeV0
//     reserved: u8,                // 0x07: Reserved for future use
// }

// #[repr(C, packed)]
// struct EnvelopeV0 {
//     // Size is 106 bytes.
//     magic: [u8; 4],              // 0x00: 0x56 0x4C 0x49 0x44 ("VLID")
//     version: u8,                 // 0x04: 0 = EnvelopeV0
//     min_version: u8,             // 0x05: 0 = EnvelopeV0
//     max_version: u8,             // 0x06: 0 = EnvelopeV0
//     reserved: u8,                // 0x07: Reserved for future use
//     size: u16,                   // 0x08: Total size of the envelope including the encrypted operations message. Maximum size is 65,507 bytes, which is the data size limit for a single UDP message on IPv4.
//     timestamp: u64,              // 0x0A: Duration since UNIX_EPOCH in microseconds when this message is sent. Messages older than 10 seconds are dropped.
//     nonce: [u8; 24],             // 0x12: Random nonce for replay protection and for x25519
//     sender_id: [u8; 32],         // 0x2A: Node ID of the message source, which is the Ed25519 public key of the sender (must be verified with find_node if this is a new node_id/address combination)
//     recipient_id: [u8; 32],      // 0x4A: Node ID of the intended recipient, which is the Ed25519 public key of the recipient (must be the receiving node, or a relay lease holder)
//                                  // 0x6A: message is appended (operations)
//                                  // encrypted by XChaCha20Poly1305(nonce,x25519(recipient_id, sender_secret_key))
//     signature: [u8; 64],         // 0x?? (end-0x40): Ed25519 signature of the entire envelope including header is appended to the packet
//                                  // entire header needs to be included in message digest, relays are not allowed to modify the envelope without invalidating the signature.
// }

pub const MAX_ENVELOPE_SIZE: usize = 65507;
pub const MIN_ENVELOPE_SIZE: usize = 0x6A + 0x40; // Header + Signature
pub const ENVELOPE_MAGIC: &[u8; 4] = b"VLID";
pub type EnvelopeNonce = [u8; 24];

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Envelope {
    version: u8,
    min_version: u8,
    max_version: u8,
    timestamp: u64,
    nonce: EnvelopeNonce,
    sender_id: DHTKey,
    recipient_id: DHTKey,
}

impl Envelope {
    pub fn new(
        version: u8,
        timestamp: u64,
        nonce: EnvelopeNonce,
        sender_id: DHTKey,
        recipient_id: DHTKey,
    ) -> Self {
        assert!(sender_id.valid);
        assert!(recipient_id.valid);

        assert!(version >= MIN_CRYPTO_VERSION);
        assert!(version <= MAX_CRYPTO_VERSION);
        Self {
            version,
            min_version: MIN_CRYPTO_VERSION,
            max_version: MAX_CRYPTO_VERSION,
            timestamp,
            nonce,
            sender_id,
            recipient_id,
        }
    }

    pub fn from_signed_data(data: &[u8]) -> Result<Envelope, VeilidAPIError> {
        // Ensure we are at least the length of the envelope
        // Silent drop here, as we use zero length packets as part of the protocol for hole punching
        if data.len() < MIN_ENVELOPE_SIZE {
            return Err(VeilidAPIError::generic("envelope data too small"));
        }

        // Verify magic number
        let magic: [u8; 4] = data[0x00..0x04]
            .try_into()
            .map_err(VeilidAPIError::internal)?;
        if magic != *ENVELOPE_MAGIC {
            return Err(VeilidAPIError::generic("bad magic number"));
        }

        // Check version
        let version = data[0x04];
        if version > MAX_CRYPTO_VERSION || version < MIN_CRYPTO_VERSION {
            return Err(VeilidAPIError::parse_error(
                "unsupported cryptography version",
                version,
            ));
        }

        // Get min version
        let min_version = data[0x05];
        if min_version > version {
            return Err(VeilidAPIError::parse_error("version too low", version));
        }

        // Get max version
        let max_version = data[0x06];
        if version > max_version {
            return Err(VeilidAPIError::parse_error("version too high", version));
        }
        if min_version > max_version {
            return Err(VeilidAPIError::generic("version information invalid"));
        }

        // Get size and ensure it matches the size of the envelope and is less than the maximum message size
        let size: u16 = u16::from_le_bytes(
            data[0x08..0x0A]
                .try_into()
                .map_err(VeilidAPIError::internal)?,
        );
        if (size as usize) > MAX_ENVELOPE_SIZE {
            return Err(VeilidAPIError::parse_error("envelope too large", size));
        }
        if (size as usize) != data.len() {
            return Err(VeilidAPIError::parse_error(
                "size doesn't match envelope size",
                format!(
                    "size doesn't match envelope size: size={} data.len()={}",
                    size,
                    data.len()
                ),
            ));
        }

        // Get the timestamp
        let timestamp: u64 = u64::from_le_bytes(
            data[0x0A..0x12]
                .try_into()
                .map_err(VeilidAPIError::internal)?,
        );

        // Get nonce and sender node id
        let nonce: EnvelopeNonce = data[0x12..0x2A]
            .try_into()
            .map_err(VeilidAPIError::internal)?;
        let sender_id_slice: [u8; 32] = data[0x2A..0x4A]
            .try_into()
            .map_err(VeilidAPIError::internal)?;
        let recipient_id_slice: [u8; 32] = data[0x4A..0x6A]
            .try_into()
            .map_err(VeilidAPIError::internal)?;
        let sender_id = DHTKey::new(sender_id_slice);
        let recipient_id = DHTKey::new(recipient_id_slice);

        // Ensure sender_id and recipient_id are not the same
        if sender_id == recipient_id {
            return Err(VeilidAPIError::parse_error(
                "sender_id should not be same as recipient_id",
                recipient_id.encode(),
            ));
        }

        // Get signature
        let signature = DHTSignature::new(
            data[(data.len() - 64)..]
                .try_into()
                .map_err(VeilidAPIError::internal)?,
        );

        // Validate signature
        verify(&sender_id, &data[0..(data.len() - 64)], &signature)
            .map_err(VeilidAPIError::internal)?;

        // Return envelope
        Ok(Self {
            version,
            min_version,
            max_version,
            timestamp,
            nonce,
            sender_id,
            recipient_id,
        })
    }

    pub fn decrypt_body(
        &self,
        crypto: Crypto,
        data: &[u8],
        node_id_secret: &DHTKeySecret,
    ) -> Result<Vec<u8>, VeilidAPIError> {
        // Get DH secret
        let dh_secret = crypto.cached_dh(&self.sender_id, node_id_secret)?;

        // Decrypt message without authentication
        let body = Crypto::crypt_no_auth(&data[0x6A..data.len() - 64], &self.nonce, &dh_secret);

        Ok(body)
    }

    pub fn to_encrypted_data(
        &self,
        crypto: Crypto,
        body: &[u8],
        node_id_secret: &DHTKeySecret,
    ) -> Result<Vec<u8>, VeilidAPIError> {
        // Ensure sender node id is valid
        if !self.sender_id.valid {
            return Err(VeilidAPIError::generic("sender id is invalid"));
        }
        // Ensure recipient node id is valid
        if !self.recipient_id.valid {
            return Err(VeilidAPIError::generic("recipient id is invalid"));
        }

        // Ensure body isn't too long
        let envelope_size: usize = body.len() + MIN_ENVELOPE_SIZE;
        if envelope_size > MAX_ENVELOPE_SIZE {
            return Err(VeilidAPIError::parse_error(
                "envelope size is too large",
                envelope_size,
            ));
        }
        let mut data = vec![0u8; envelope_size];

        // Write magic
        data[0x00..0x04].copy_from_slice(ENVELOPE_MAGIC);
        // Write version
        data[0x04] = self.version;
        // Write min version
        data[0x05] = self.min_version;
        // Write max version
        data[0x06] = self.max_version;
        // Write size
        data[0x08..0x0A].copy_from_slice(&(envelope_size as u16).to_le_bytes());
        // Write timestamp
        data[0x0A..0x12].copy_from_slice(&self.timestamp.to_le_bytes());
        // Write nonce
        data[0x12..0x2A].copy_from_slice(&self.nonce);
        // Write sender node id
        data[0x2A..0x4A].copy_from_slice(&self.sender_id.bytes);
        // Write recipient node id
        data[0x4A..0x6A].copy_from_slice(&self.recipient_id.bytes);

        // Generate dh secret
        let dh_secret = crypto.cached_dh(&self.recipient_id, node_id_secret)?;

        // Encrypt and authenticate message
        let encrypted_body = Crypto::crypt_no_auth(body, &self.nonce, &dh_secret);

        // Write body
        if !encrypted_body.is_empty() {
            data[0x6A..envelope_size - 64].copy_from_slice(encrypted_body.as_slice());
        }

        // Sign the envelope
        let signature = sign(
            &self.sender_id,
            node_id_secret,
            &data[0..(envelope_size - 64)],
        )?;

        // Append the signature
        data[(envelope_size - 64)..].copy_from_slice(&signature.bytes);

        Ok(data)
    }

    pub fn get_version(&self) -> u8 {
        self.version
    }

    pub fn get_min_max_version(&self) -> VersionRange {
        VersionRange {
            min: self.min_version,
            max: self.max_version,
        }
    }

    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn get_nonce(&self) -> EnvelopeNonce {
        self.nonce
    }

    pub fn get_sender_id(&self) -> DHTKey {
        self.sender_id
    }
    pub fn get_recipient_id(&self) -> DHTKey {
        self.recipient_id
    }
}
