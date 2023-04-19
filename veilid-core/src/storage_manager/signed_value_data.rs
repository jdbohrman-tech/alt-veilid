use super::*;
use rkyv::{Archive as RkyvArchive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use serde::*;

/////////////////////////////////////////////////////////////////////////////////////////////////////
///

#[derive(
    Clone,
    Debug,
    PartialOrd,
    PartialEq,
    Eq,
    Ord,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct SignedValueData {
    value_data: ValueData,
    signature: Signature,
}
impl SignedValueData {
    pub fn new(
        value_data: ValueData,
        owner: PublicKey,
        subkey: ValueSubkey,
        signature: Signature,
        vcrypto: CryptoSystemVersion,
    ) -> Result<Self, VeilidAPIError> {
        let node_info_bytes = Self::make_signature_bytes(&value_data, &owner, subkey)?;

        // validate signature
        vcrypto.verify(&value_data.writer(), &node_info_bytes, &signature)?;
        Ok(Self {
            value_data,
            signature,
        })
    }

    pub fn make_signature(
        value_data: ValueData,
        owner: PublicKey,
        subkey: ValueSubkey,
        vcrypto: CryptoSystemVersion,
        writer_secret: SecretKey,
    ) -> Result<Self, VeilidAPIError> {
        let node_info_bytes = Self::make_signature_bytes(&value_data, &owner, subkey)?;

        // create signature
        let signature = vcrypto.sign(&value_data.writer(), &writer_secret, &node_info_bytes)?;
        Ok(Self {
            value_data,
            signature,
        })
    }

    pub fn value_data(&self) -> &ValueData {
        &self.value_data
    }

    pub fn signature(&self) -> &Signature {
        &self.signature
    }

    pub fn total_size(&self) -> usize {
        (mem::size_of::<Self>() - mem::size_of::<ValueData>()) + self.value_data.total_size()
    }

    fn make_signature_bytes(
        value_data: &ValueData,
        owner: &PublicKey,
        subkey: ValueSubkey,
    ) -> Result<Vec<u8>, VeilidAPIError> {
        let mut node_info_bytes =
            Vec::with_capacity(PUBLIC_KEY_LENGTH + 4 + 4 + value_data.data().len());

        // Add owner to signature
        node_info_bytes.extend_from_slice(&owner.bytes);
        // Add subkey to signature
        node_info_bytes.extend_from_slice(&subkey.to_le_bytes());
        // Add sequence number to signature
        node_info_bytes.extend_from_slice(&value_data.seq().to_le_bytes());
        // Add data to signature
        node_info_bytes.extend_from_slice(value_data.data());

        Ok(node_info_bytes)
    }
}
