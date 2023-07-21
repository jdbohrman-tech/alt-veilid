#![allow(dead_code)]
#![allow(clippy::absurd_extreme_comparisons)]
use super::*;
use crate::*;
use core::convert::TryInto;

/// Out-of-band receipts are versioned along with envelope versions
///
/// These are the formats for the on-the-wire serialization performed by this module
///
/// #[repr(C, packed)]
/// struct ReceiptHeader {
///     // Size is at least 4 bytes. Depending on the version specified, the size may vary and should be case to the appropriate struct
///     magic: [u8; 3],              // 0x00: 0x52 0x43 0x50 ("RCP")
///     version: u8,                 // 0x03: 0 = ReceiptV0
/// }
///
/// #[repr(C, packed)]
/// struct ReceiptV0 {
///     // Size is 66 bytes without extra data and signature, 130 with signature
///     magic: [u8; 3],              // 0x00: 0x52 0x43 0x50 ("RCP")
///     version: u8,                 // 0x03: 0 = ReceiptV0
///     crypto_kind: [u8; 4],        // 0x04: CryptoSystemVersion FOURCC code
///     size: u16,                   // 0x08: Total size of the receipt including the extra data and the signature. Maximum size is 1380 bytes.
///     nonce: [u8; 24],             // 0x0A: Randomly chosen bytes that represent a unique receipt. Could be used to encrypt the extra data, but it's not required.
///     sender_id: [u8; 32],         // 0x22: Node ID of the message source, which is the public key of the sender
///     extra_data: [u8; ??],        // 0x42: Extra data is appended (arbitrary extra data, not encrypted by receipt itself, maximum size is 1250 bytes)
///     signature: [u8; 64],         // 0x?? (end-0x40): Signature of the entire receipt including header and extra data is appended to the packet
/// }

pub const MAX_RECEIPT_SIZE: usize = 1380;
pub const MAX_EXTRA_DATA_SIZE: usize = MAX_RECEIPT_SIZE - MIN_RECEIPT_SIZE; // 1250
pub const MIN_RECEIPT_SIZE: usize = 130;
pub const RECEIPT_MAGIC: &[u8; 3] = b"RCP";

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Receipt {
    version: u8,
    crypto_kind: CryptoKind,
    nonce: Nonce,
    sender_id: PublicKey,
    extra_data: Vec<u8>,
}

impl Receipt {
    pub fn try_new<D: AsRef<[u8]>>(
        version: u8,
        crypto_kind: CryptoKind,
        nonce: Nonce,
        sender_id: PublicKey,
        extra_data: D,
    ) -> VeilidAPIResult<Self> {
        assert!(VALID_ENVELOPE_VERSIONS.contains(&version));
        assert!(VALID_CRYPTO_KINDS.contains(&crypto_kind));

        if extra_data.as_ref().len() > MAX_EXTRA_DATA_SIZE {
            apibail_parse_error!(
                "extra data too large for receipt",
                extra_data.as_ref().len()
            );
        }
        Ok(Self {
            version,
            crypto_kind,
            nonce,
            sender_id,
            extra_data: Vec::from(extra_data.as_ref()),
        })
    }

    pub fn from_signed_data(crypto: Crypto, data: &[u8]) -> VeilidAPIResult<Receipt> {
        // Ensure we are at least the length of the envelope
        if data.len() < MIN_RECEIPT_SIZE {
            apibail_parse_error!("receipt too small", data.len());
        }

        // Verify magic number
        let magic: [u8; 3] = data[0x00..0x03]
            .try_into()
            .map_err(VeilidAPIError::internal)?;
        if magic != *RECEIPT_MAGIC {
            apibail_generic!("bad magic number");
        }

        // Check version
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
        if (size as usize) > MAX_RECEIPT_SIZE {
            apibail_parse_error!("receipt size is too large", size);
        }
        if (size as usize) != data.len() {
            apibail_parse_error!(
                "size doesn't match receipt size",
                format!("size={} data.len()={}", size, data.len())
            );
        }

        // Get sender id
        let sender_id = PublicKey::new(
            data[0x22..0x42]
                .try_into()
                .map_err(VeilidAPIError::internal)?,
        );

        // Get signature
        let signature = Signature::new(
            data[(data.len() - 64)..]
                .try_into()
                .map_err(VeilidAPIError::internal)?,
        );

        // Validate signature
        vcrypto
            .verify(&sender_id, &data[0..(data.len() - 64)], &signature)
            .map_err(VeilidAPIError::generic)?;

        // Get nonce
        let nonce: Nonce = Nonce::new(
            data[0x0A..0x22]
                .try_into()
                .map_err(VeilidAPIError::internal)?,
        );

        // Get extra data and signature
        let extra_data: Vec<u8> = Vec::from(&data[0x42..(data.len() - 64)]);

        // Return receipt
        Ok(Self {
            version,
            crypto_kind,
            nonce,
            sender_id,
            extra_data,
        })
    }

    pub fn to_signed_data(&self, crypto: Crypto, secret: &SecretKey) -> VeilidAPIResult<Vec<u8>> {
        // Ensure extra data isn't too long
        let receipt_size: usize = self.extra_data.len() + MIN_RECEIPT_SIZE;
        if receipt_size > MAX_RECEIPT_SIZE {
            apibail_parse_error!("receipt too large", receipt_size);
        }
        // Get crypto version
        let vcrypto = crypto
            .get(self.crypto_kind)
            .expect("need to ensure only valid crypto kinds here");

        let mut data: Vec<u8> = vec![0u8; receipt_size];

        // Write magic
        data[0x00..0x03].copy_from_slice(RECEIPT_MAGIC);
        // Write version
        data[0x03] = self.version;
        // Write crypto kind
        data[0x04..0x08].copy_from_slice(&self.crypto_kind.0);
        // Write size
        data[0x08..0x0A].copy_from_slice(&(receipt_size as u16).to_le_bytes());
        // Write nonce
        data[0x0A..0x22].copy_from_slice(&self.nonce.bytes);
        // Write sender node id
        data[0x22..0x42].copy_from_slice(&self.sender_id.bytes);
        // Write extra data
        if !self.extra_data.is_empty() {
            data[0x42..(receipt_size - 64)].copy_from_slice(self.extra_data.as_slice());
        }
        // Sign the receipt
        let signature = vcrypto
            .sign(&self.sender_id, secret, &data[0..(receipt_size - 64)])
            .map_err(VeilidAPIError::generic)?;
        // Append the signature
        data[(receipt_size - 64)..].copy_from_slice(&signature.bytes);

        Ok(data)
    }

    pub fn get_version(&self) -> u8 {
        self.version
    }

    pub fn get_crypto_kind(&self) -> CryptoKind {
        self.crypto_kind
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

    pub fn get_extra_data(&self) -> &[u8] {
        &self.extra_data
    }
}
