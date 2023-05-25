use super::*;

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
    pub fn new(owner: PublicKey, schema_data: Vec<u8>, signature: Signature) -> Self {
        Self {
            owner,
            schema_data,
            signature,
        }
    }

    pub fn validate(&self, vcrypto: CryptoSystemVersion) -> VeilidAPIResult<()> {
        // validate signature
        vcrypto.verify(&self.owner, &self.schema_data, &self.signature)
    }

    pub fn owner(&self) -> &PublicKey {
        &self.owner
    }

    pub fn schema_data(&self) -> &[u8] {
        &self.schema_data
    }

    pub fn schema(&self) -> VeilidAPIResult<DHTSchema> {
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
    ) -> VeilidAPIResult<Self> {
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

    pub fn cmp_no_sig(&self, other: &Self) -> cmp::Ordering {
        let o = self.owner.cmp(&other.owner);
        if o != cmp::Ordering::Equal {
            return o;
        }
        self.schema_data.cmp(&other.schema_data)
    }
}
