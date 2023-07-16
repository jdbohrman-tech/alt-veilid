use super::*;

/// DHT Record Descriptor
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct DHTRecordDescriptor {
    /// DHT Key = Hash(ownerKeyKind) of: [ ownerKeyValue, schema ]
    #[schemars(with = "String")]
    key: TypedKey,
    /// The public key of the owner
    #[schemars(with = "String")]
    owner: PublicKey,
    /// If this key is being created: Some(the secret key of the owner)
    /// If this key is just being opened: None
    #[schemars(with = "Option<String>")]
    owner_secret: Option<SecretKey>,
    /// The schema in use associated with the key
    schema: DHTSchema,
}

impl DHTRecordDescriptor {
    pub fn new(
        key: TypedKey,
        owner: PublicKey,
        owner_secret: Option<SecretKey>,
        schema: DHTSchema,
    ) -> Self {
        Self {
            key,
            owner,
            owner_secret,
            schema,
        }
    }

    pub fn key(&self) -> &TypedKey {
        &self.key
    }
    pub fn owner(&self) -> &PublicKey {
        &self.owner
    }

    pub fn owner_secret(&self) -> Option<&SecretKey> {
        self.owner_secret.as_ref()
    }

    pub fn schema(&self) -> &DHTSchema {
        &self.schema
    }
}
