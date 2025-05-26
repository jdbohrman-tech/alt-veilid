use super::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RecordTableKey {
    pub key: TypedRecordKey,
}
impl RecordTableKey {
    pub fn bytes(&self) -> [u8; HASH_DIGEST_LENGTH + 4] {
        let mut bytes = [0u8; HASH_DIGEST_LENGTH + 4];
        bytes[0..4].copy_from_slice(&self.key.kind.0);
        bytes[4..HASH_DIGEST_LENGTH + 4].copy_from_slice(&self.key.value.bytes);
        bytes
    }
}

impl TryFrom<&[u8]> for RecordTableKey {
    type Error = EyreReport;
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() != HASH_DIGEST_LENGTH + 4 {
            bail!("invalid bytes length");
        }
        let kind = CryptoKind::try_from(&bytes[0..4]).wrap_err("invalid kind")?;
        let value =
            RecordKey::try_from(&bytes[4..HASH_DIGEST_LENGTH + 4]).wrap_err("invalid value")?;
        let key = TypedRecordKey::new(kind, value);
        Ok(RecordTableKey { key })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SubkeyTableKey {
    pub key: TypedRecordKey,
    pub subkey: ValueSubkey,
}
impl SubkeyTableKey {
    pub fn bytes(&self) -> [u8; HASH_DIGEST_LENGTH + 4 + 4] {
        let mut bytes = [0u8; HASH_DIGEST_LENGTH + 4 + 4];
        bytes[0..4].copy_from_slice(&self.key.kind.0);
        bytes[4..HASH_DIGEST_LENGTH + 4].copy_from_slice(&self.key.value.bytes);
        bytes[HASH_DIGEST_LENGTH + 4..HASH_DIGEST_LENGTH + 4 + 4]
            .copy_from_slice(&self.subkey.to_le_bytes());
        bytes
    }
}
impl TryFrom<&[u8]> for SubkeyTableKey {
    type Error = EyreReport;
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() != HASH_DIGEST_LENGTH + 4 {
            bail!("invalid bytes length");
        }
        let kind = CryptoKind::try_from(&bytes[0..4]).wrap_err("invalid kind")?;
        let value =
            RecordKey::try_from(&bytes[4..HASH_DIGEST_LENGTH + 4]).wrap_err("invalid value")?;
        let subkey = ValueSubkey::from_le_bytes(
            bytes[HASH_DIGEST_LENGTH + 4..HASH_DIGEST_LENGTH + 4 + 4]
                .try_into()
                .wrap_err("invalid subkey")?,
        );

        let key = TypedRecordKey::new(kind, value);
        Ok(SubkeyTableKey { key, subkey })
    }
}
