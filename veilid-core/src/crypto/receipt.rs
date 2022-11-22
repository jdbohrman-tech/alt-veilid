#![allow(dead_code)]
#![allow(clippy::absurd_extreme_comparisons)]
use super::*;
use crate::xx::*;
use crate::*;
use core::convert::TryInto;
use data_encoding::BASE64URL_NOPAD;

// #[repr(C, packed)]
// struct ReceiptHeader {
//     // Size is at least 8 bytes. Depending on the version specified, the size may vary and should be case to the appropriate struct
//     magic: [u8; 4],              // 0x00: 0x52 0x43 0x50 0x54 ("RCPT")
//     version: u8,                 // 0x04: 0 = ReceiptV0
//     reserved: u8,                // 0x05: Reserved for future use
// }

// #[repr(C, packed)]
// struct ReceiptV0 {
//     // Size is 106 bytes.
//     magic: [u8; 4],              // 0x00: 0x52 0x43 0x50 0x54 ("RCPT")
//     version: u8,                 // 0x04: 0 = ReceiptV0
//     reserved: u8,                // 0x05: Reserved for future use
//     size: u16,                   // 0x06: Total size of the receipt including the extra data and the signature. Maximum size is 1152 bytes.
//     nonce: [u8; 24],             // 0x08: Randomly chosen bytes that represent a unique receipt. Could be used to encrypt the extra data, but it's not required.
//     sender_id: [u8; 32],         // 0x20: Node ID of the message source, which is the Ed25519 public key of the sender
//     extra_data: [u8; ??],        // 0x40: Extra data is appended (arbitrary extra data, not encrypted by receipt itself, maximum size is 1024 bytes)
//     signature: [u8; 64],         // 0x?? (end-0x40): Ed25519 signature of the entire receipt including header and extra data is appended to the packet
// }

pub const MAX_RECEIPT_SIZE: usize = 1152;
pub const MAX_EXTRA_DATA_SIZE: usize = 1024;
pub const MIN_RECEIPT_SIZE: usize = 128;
pub const RECEIPT_MAGIC: &[u8; 4] = b"RCPT";
pub type ReceiptNonce = [u8; 24];

pub trait Encodable {
    fn encode(&self) -> String;
}

impl Encodable for ReceiptNonce {
    fn encode(&self) -> String {
        BASE64URL_NOPAD.encode(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Receipt {
    version: u8,
    nonce: ReceiptNonce,
    sender_id: DHTKey,
    extra_data: Vec<u8>,
}

impl Receipt {
    pub fn try_new<D: AsRef<[u8]>>(
        version: u8,
        nonce: ReceiptNonce,
        sender_id: DHTKey,
        extra_data: D,
    ) -> Result<Self, VeilidAPIError> {
        if extra_data.as_ref().len() > MAX_EXTRA_DATA_SIZE {
            return Err(VeilidAPIError::parse_error(
                "extra data too large for receipt",
                extra_data.as_ref().len(),
            ));
        }
        Ok(Self {
            version,
            nonce,
            sender_id,
            extra_data: Vec::from(extra_data.as_ref()),
        })
    }

    pub fn from_signed_data(data: &[u8]) -> Result<Receipt, VeilidAPIError> {
        // Ensure we are at least the length of the envelope
        if data.len() < MIN_RECEIPT_SIZE {
            return Err(VeilidAPIError::parse_error("receipt too small", data.len()));
        }

        // Verify magic number
        let magic: [u8; 4] = data[0x00..0x04]
            .try_into()
            .map_err(VeilidAPIError::internal)?;
        if magic != *RECEIPT_MAGIC {
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

        // Get size and ensure it matches the size of the envelope and is less than the maximum message size
        let size: u16 = u16::from_le_bytes(
            data[0x06..0x08]
                .try_into()
                .map_err(VeilidAPIError::internal)?,
        );
        if (size as usize) > MAX_RECEIPT_SIZE {
            return Err(VeilidAPIError::parse_error(
                "receipt size is too large",
                size,
            ));
        }
        if (size as usize) != data.len() {
            return Err(VeilidAPIError::parse_error(
                "size doesn't match receipt size",
                format!("size={} data.len()={}", size, data.len()),
            ));
        }

        // Get sender id
        let sender_id = DHTKey::new(
            data[0x20..0x40]
                .try_into()
                .map_err(VeilidAPIError::internal)?,
        );

        // Get signature
        let signature = DHTSignature::new(
            data[(data.len() - 64)..]
                .try_into()
                .map_err(VeilidAPIError::internal)?,
        );

        // Validate signature
        verify(&sender_id, &data[0..(data.len() - 64)], &signature)
            .map_err(VeilidAPIError::generic)?;

        // Get nonce
        let nonce: ReceiptNonce = data[0x08..0x20]
            .try_into()
            .map_err(VeilidAPIError::internal)?;

        // Get extra data and signature
        let extra_data: Vec<u8> = Vec::from(&data[0x40..(data.len() - 64)]);

        // Return receipt
        Ok(Self {
            version,
            nonce,
            sender_id,
            extra_data,
        })
    }

    pub fn to_signed_data(&self, secret: &DHTKeySecret) -> Result<Vec<u8>, VeilidAPIError> {
        // Ensure extra data isn't too long
        let receipt_size: usize = self.extra_data.len() + MIN_RECEIPT_SIZE;
        if receipt_size > MAX_RECEIPT_SIZE {
            return Err(VeilidAPIError::parse_error(
                "receipt too large",
                receipt_size,
            ));
        }
        let mut data: Vec<u8> = vec![0u8; receipt_size];

        // Write magic
        data[0x00..0x04].copy_from_slice(RECEIPT_MAGIC);
        // Write version
        data[0x04] = self.version;
        // Write size
        data[0x06..0x08].copy_from_slice(&(receipt_size as u16).to_le_bytes());
        // Write nonce
        data[0x08..0x20].copy_from_slice(&self.nonce);
        // Write sender node id
        data[0x20..0x40].copy_from_slice(&self.sender_id.bytes);
        // Write extra data
        if !self.extra_data.is_empty() {
            data[0x40..(receipt_size - 64)].copy_from_slice(self.extra_data.as_slice());
        }
        // Sign the receipt
        let signature = sign(&self.sender_id, secret, &data[0..(receipt_size - 64)])
            .map_err(VeilidAPIError::generic)?;
        // Append the signature
        data[(receipt_size - 64)..].copy_from_slice(&signature.bytes);

        Ok(data)
    }

    pub fn get_version(&self) -> u8 {
        self.version
    }

    pub fn get_nonce(&self) -> ReceiptNonce {
        self.nonce
    }

    pub fn get_sender_id(&self) -> DHTKey {
        self.sender_id
    }
    pub fn get_extra_data(&self) -> &[u8] {
        &self.extra_data
    }
}
