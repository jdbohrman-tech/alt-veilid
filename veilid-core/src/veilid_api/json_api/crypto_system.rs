use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CryptoSystemRequest {
    cs_id: String,
    #[serde(flatten)]
    cs_op: CryptoSystemRequestOp,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CryptoSystemResponse {
    cs_id: String,
    #[serde(flatten)]
    cs_op: CryptoSystemResponseOp,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "cs_op")]
pub enum CryptoSystemRequestOp {
    Release,
    CachedDh,
    ComputeDh,
    RandomBytes,
    DefaultSaltLength,
    HashPassword,
    VerifyPassword,
    DeriveSharedSecret,
    RandomNonce,
    RandomSharedSecret,
    GenerateKeyPair,
    GenerateHash,
    ValidateKeyPair,
    ValidateHash,
    Distance,
    Sign,
    Verify,
    AeadOverhead,
    DecryptAead,
    EncryptAead,
    CryptNoAuth,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "cs_op")]
pub enum CryptoSystemResponseOp {
    Release,
    CachedDh,
    ComputeDh,
    RandomBytes,
    DefaultSaltLength,
    HashPassword,
    VerifyPassword,
    DeriveSharedSecret,
    RandomNonce,
    RandomSharedSecret,
    GenerateKeyPair,
    GenerateHash,
    ValidateKeyPair,
    ValidateHash,
    Distance,
    Sign,
    Verify,
    AeadOverhead,
    DecryptAead,
    EncryptAead,
    CryptNoAuth,
}
