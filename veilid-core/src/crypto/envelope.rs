#![allow(dead_code)]
#![allow(clippy::absurd_extreme_comparisons)]
use super::*;
use crate::*;
use core::convert::TryInto;

/// Envelopes are versioned
///
/// These are the formats for the on-the-wire serialization performed by this module
///
/// #[repr(C, packed)]
/// struct EnvelopeHeader {
///     // Size is at least 4 bytes. Depending on the version specified, the size may vary and should be case to the appropriate struct
///     magic: [u8; 3],              // 0x00: 0x56 0x4C 0x44 ("VLD")
///     version: u8,                 // 0x03: 0 = EnvelopeV0
/// }
///
/// #[repr(C, packed)]
/// struct EnvelopeV0 {
///     // Size is 106 bytes without signature and 170 with signature
///     magic: [u8; 3],              // 0x00: 0x56 0x4C 0x44 ("VLD")
///     version: u8,                 // 0x03: 0 = EnvelopeV0
///     crypto_kind: [u8; 4],        // 0x04: CryptoSystemVersion FOURCC code (CryptoKind)
///     size: u16,                   // 0x08: Total size of the envelope including the encrypted operations message. Maximum size is 65,507 bytes, which is the data size limit for a single UDP message on IPv4.
///     timestamp: u64,              // 0x0A: Duration since UNIX_EPOCH in microseconds when this message is sent. Messages older than 10 seconds are dropped.
///     nonce: [u8; 24],             // 0x12: Random nonce for replay protection and for dh
///     sender_id: [u8; 32],         // 0x2A: Node ID of the message source, which is the public key of the sender (must be verified with find_node if this is a new node_id/address combination)
///     recipient_id: [u8; 32],      // 0x4A: Node ID of the intended recipient, which is the public key of the recipient (must be the receiving node, or a relay lease holder)
///                                  // 0x6A: message is appended (operations)
///     signature: [u8; 64],         // 0x?? (end-0x40): Signature of the entire envelope including header is appended to the packet
///                                  // entire header needs to be included in message digest, relays are not allowed to modify the envelope without invalidating the signature.
/// }

pub const MAX_ENVELOPE_SIZE: usize = 65507;
pub const MIN_ENVELOPE_SIZE: usize = 0x6A + 0x40; // Header + Signature
pub const ENVELOPE_MAGIC: &[u8; 3] = b"VLD";

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Envelope {
    version: EnvelopeVersion,
    crypto_kind: CryptoKind,
    timestamp: Timestamp,
    nonce: Nonce,
    sender_id: PublicKey,
    recipient_id: PublicKey,
}

impl Envelope {
    pub fn new(
        version: EnvelopeVersion,
        crypto_kind: CryptoKind,
        timestamp: Timestamp,
        nonce: Nonce,
        sender_id: PublicKey,
        recipient_id: PublicKey,
    ) -> Self {
        assert!(VALID_ENVELOPE_VERSIONS.contains(&version));
        assert!(VALID_CRYPTO_KINDS.contains(&crypto_kind));
        Self {
            version,
            crypto_kind,
            timestamp,
            nonce,
            sender_id,
            recipient_id,
        }
    }

    pub fn from_signed_data(
        crypto: Crypto,
        data: &[u8],
        network_key: &Option<SharedSecret>,
    ) -> VeilidAPIResult<Envelope> {
        // Ensure we are at least the length of the envelope
        // Silent drop here, as we use zero length packets as part of the protocol for hole punching
        if data.len() < MIN_ENVELOPE_SIZE {
            apibail_generic!("envelope data too small");
        }

        // Verify magic number
        let magic: [u8; 3] = data[0x00..0x03]
            .try_into()
            .map_err(VeilidAPIError::internal)?;
        if magic != *ENVELOPE_MAGIC {
            apibail_generic!("bad magic number");
        }

        // Check envelope version
        let version = data[0x03];
        if !VALID_ENVELOPE_VERSIONS.contains(&version) {
            apibail_parse_error!("unsupported envelope version", version);
        }

        // Check crypto kind
        let crypto_kind = FourCC(
            data[0x04..0x08]
                .try_into()
                .map_err(VeilidAPIError::internal)?,
        );
        let Some(vcrypto) = crypto.get(crypto_kind) else {
            apibail_parse_error!("unsupported crypto kind", crypto_kind);
        };

        // Get size and ensure it matches the size of the envelope and is less than the maximum message size
        let size: u16 = u16::from_le_bytes(
            data[0x08..0x0A]
                .try_into()
                .map_err(VeilidAPIError::internal)?,
        );
        if (size as usize) > MAX_ENVELOPE_SIZE {
            apibail_parse_error!("envelope too large", size);
        }
        if (size as usize) != data.len() {
            apibail_parse_error!(
                "size doesn't match envelope size",
                format!(
                    "size doesn't match envelope size: size={} data.len()={}",
                    size,
                    data.len()
                )
            );
        }

        // Get the timestamp
        let timestamp: Timestamp = u64::from_le_bytes(
            data[0x0A..0x12]
                .try_into()
                .map_err(VeilidAPIError::internal)?,
        )
        .into();

        // Get nonce and sender node id
        let nonce_slice: [u8; NONCE_LENGTH] = data[0x12..0x2A]
            .try_into()
            .map_err(VeilidAPIError::internal)?;
        let sender_id_slice: [u8; PUBLIC_KEY_LENGTH] = data[0x2A..0x4A]
            .try_into()
            .map_err(VeilidAPIError::internal)?;
        let recipient_id_slice: [u8; PUBLIC_KEY_LENGTH] = data[0x4A..0x6A]
            .try_into()
            .map_err(VeilidAPIError::internal)?;
        let mut nonce: Nonce = Nonce::new(nonce_slice);
        let mut sender_id = PublicKey::new(sender_id_slice);
        let mut recipient_id = PublicKey::new(recipient_id_slice);

        // Apply network key (not the best, but it will keep networks from colliding without much overhead)
        if let Some(nk) = network_key.as_ref() {
            for n in 0..NONCE_LENGTH {
                nonce.bytes[n] ^= nk.bytes[n];
            }
            for n in 0..CRYPTO_KEY_LENGTH {
                sender_id.bytes[n] ^= nk.bytes[n];
            }
            for n in 0..CRYPTO_KEY_LENGTH {
                recipient_id.bytes[n] ^= nk.bytes[n];
            }
        }

        // Ensure sender_id and recipient_id are not the same
        if sender_id == recipient_id {
            apibail_parse_error!(
                "sender_id should not be same as recipient_id",
                recipient_id.encode()
            );
        }

        // Get signature
        let signature = Signature::new(
            data[(data.len() - 64)..]
                .try_into()
                .map_err(VeilidAPIError::internal)?,
        );

        // Validate signature
        vcrypto
            .verify(&sender_id, &data[0..(data.len() - 64)], &signature)
            .map_err(VeilidAPIError::internal)?;

        // Return envelope
        Ok(Self {
            version,
            crypto_kind,
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
        node_id_secret: &SecretKey,
        network_key: &Option<SharedSecret>,
    ) -> VeilidAPIResult<Vec<u8>> {
        // Get DH secret
        let vcrypto = crypto
            .get(self.crypto_kind)
            .expect("need to ensure only valid crypto kinds here");
        let mut dh_secret = vcrypto.cached_dh(&self.sender_id, node_id_secret)?;

        // Apply network key
        if let Some(nk) = network_key.as_ref() {
            for n in 0..CRYPTO_KEY_LENGTH {
                dh_secret.bytes[n] ^= nk.bytes[n];
            }
        }
        // Decrypt message without authentication
        let body = vcrypto.crypt_no_auth_aligned_8(
            &data[0x6A..data.len() - 64],
            &self.nonce.bytes,
            &dh_secret,
        );

        // Decompress body
        let body = decompress_size_prepended(&body, Some(MAX_ENVELOPE_SIZE))?;

        Ok(body)
    }

    pub fn to_encrypted_data(
        &self,
        crypto: Crypto,
        body: &[u8],
        node_id_secret: &SecretKey,
        network_key: &Option<SharedSecret>,
    ) -> VeilidAPIResult<Vec<u8>> {
        // Ensure body isn't too long
        let uncompressed_body_size: usize = body.len() + MIN_ENVELOPE_SIZE;
        if uncompressed_body_size > MAX_ENVELOPE_SIZE {
            apibail_parse_error!(
                "envelope size before compression is too large",
                uncompressed_body_size
            );
        }

        // Compress body
        let body = compress_prepend_size(&body);

        // Ensure body isn't too long
        let envelope_size: usize = body.len() + MIN_ENVELOPE_SIZE;
        if envelope_size > MAX_ENVELOPE_SIZE {
            apibail_parse_error!(
                "envelope size after compression is too large",
                envelope_size
            );
        }
        // Generate dh secret
        let vcrypto = crypto
            .get(self.crypto_kind)
            .expect("need to ensure only valid crypto kinds here");
        let mut dh_secret = vcrypto.cached_dh(&self.recipient_id, node_id_secret)?;

        // Write envelope body
        let mut data = vec![0u8; envelope_size];

        // Write magic
        data[0x00..0x03].copy_from_slice(ENVELOPE_MAGIC);
        // Write version
        data[0x03] = self.version;
        // Write crypto kind
        data[0x04..0x08].copy_from_slice(&self.crypto_kind.0);
        // Write size
        data[0x08..0x0A].copy_from_slice(&(envelope_size as u16).to_le_bytes());
        // Write timestamp
        data[0x0A..0x12].copy_from_slice(&self.timestamp.as_u64().to_le_bytes());
        // Write nonce
        data[0x12..0x2A].copy_from_slice(&self.nonce.bytes);
        // Write sender node id
        data[0x2A..0x4A].copy_from_slice(&self.sender_id.bytes);
        // Write recipient node id
        data[0x4A..0x6A].copy_from_slice(&self.recipient_id.bytes);

        // Apply network key (not the best, but it will keep networks from colliding without much overhead)
        if let Some(nk) = network_key.as_ref() {
            for n in 0..SECRET_KEY_LENGTH {
                dh_secret.bytes[n] ^= nk.bytes[n];
            }
            for n in 0..NONCE_LENGTH {
                data[0x12 + n] ^= nk.bytes[n];
            }
            for n in 0..CRYPTO_KEY_LENGTH {
                data[0x2A + n] ^= nk.bytes[n];
            }
            for n in 0..CRYPTO_KEY_LENGTH {
                data[0x4A + n] ^= nk.bytes[n];
            }
        }

        // Encrypt message
        let encrypted_body = vcrypto.crypt_no_auth_unaligned(&body, &self.nonce.bytes, &dh_secret);

        // Write body
        if !encrypted_body.is_empty() {
            data[0x6A..envelope_size - 64].copy_from_slice(encrypted_body.as_slice());
        }

        // Sign the envelope
        let signature = vcrypto.sign(
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

    pub fn get_crypto_kind(&self) -> CryptoKind {
        self.crypto_kind
    }

    pub fn get_timestamp(&self) -> Timestamp {
        self.timestamp
    }

    pub fn get_nonce(&self) -> Nonce {
        self.nonce
    }

    pub fn get_sender_id(&self) -> PublicKey {
        self.sender_id
    }

    pub fn get_sender_typed_id(&self) -> TypedKey {
        TypedKey::new(self.crypto_kind, self.sender_id)
    }

    pub fn get_recipient_id(&self) -> PublicKey {
        self.recipient_id
    }

    pub fn get_recipient_typed_id(&self) -> TypedKey {
        TypedKey::new(self.crypto_kind, self.recipient_id)
    }
}
