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
pub struct SignedValueDescriptor {
    owner: PublicKey,
    schema_data: Vec<u8>,
    signature: Signature,
}
impl SignedValueDescriptor {
    pub fn new(
        owner: PublicKey,
        schema_data: Vec<u8>,
        signature: Signature,
        vcrypto: CryptoSystemVersion,
    ) -> Result<Self, VeilidAPIError> {
        // validate signature
        vcrypto.verify(&owner, &schema_data, &signature)?;

        Ok(Self {
            owner,
            schema_data,
            signature,
        })
    }

    pub fn owner(&self) -> &PublicKey {
        &self.owner
    }

    pub fn schema_data(&self) -> &[u8] {
        &self.schema_data
    }

    pub fn schema(&self) -> Result<DHTSchema, VeilidAPIError> {
        DHTSchema::try_from(self.schema_data.as_slice())
    }

    pub fn signature(&self) -> &Signature {
        &self.signature
    }

    pub fn make_signature(
        owner: PublicKey,
        schema_data: Vec<u8>,
        vcrypto: CryptoSystemVersion,
        owner_secret: SecretKey,
    ) -> Result<Self, VeilidAPIError> {
        // create signature
        let signature = vcrypto.sign(&owner, &owner_secret, &schema_data)?;
        Ok(Self {
            owner,
            schema_data,
            signature,
        })
    }

    pub fn total_size(&self) -> usize {
        mem::size_of::<Self>() + self.schema_data.len()
    }
}
