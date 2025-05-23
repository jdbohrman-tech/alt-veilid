use super::*;

/// DHT Record Descriptor
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(
    all(target_arch = "wasm32", target_os = "unknown"),
    derive(Tsify),
    tsify(from_wasm_abi, into_wasm_abi)
)]
#[must_use]
pub struct DHTRecordDescriptor {
    /// DHT Key = Hash(ownerKeyKind) of: [ ownerKeyValue, schema ]
    #[schemars(with = "String")]
    key: TypedRecordKey,
    /// The public key of the owner
    #[schemars(with = "String")]
    owner: PublicKey,
    /// If this key is being created: Some(the secret key of the owner)
    /// If this key is just being opened: None
    #[schemars(with = "Option<String>")]
    #[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), tsify(optional))]
    owner_secret: Option<SecretKey>,
    /// The schema in use associated with the key
    schema: DHTSchema,
}

impl DHTRecordDescriptor {
    pub(crate) fn new(
        key: TypedRecordKey,
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

    pub fn key(&self) -> &TypedRecordKey {
        &self.key
    }
    pub fn owner(&self) -> &PublicKey {
        &self.owner
    }

    #[must_use]
    pub fn owner_secret(&self) -> Option<&SecretKey> {
        self.owner_secret.as_ref()
    }

    pub fn schema(&self) -> &DHTSchema {
        &self.schema
    }
}
