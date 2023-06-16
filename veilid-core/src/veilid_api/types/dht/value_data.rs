use super::*;

#[derive(
    Clone,
    Debug,
    Default,
    PartialOrd,
    PartialEq,
    Eq,
    Ord,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
    JsonSchema,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct ValueData {
    /// An increasing sequence number to time-order the DHT record changes
    pub seq: ValueSeqNum,

    /// The contents of a DHT Record
    #[serde(with = "json_as_base64")]
    #[schemars(with = "String")]
    pub data: Vec<u8>,

    /// The public identity key of the writer of the data
    #[schemars(with = "String")]
    pub writer: PublicKey,
}
impl ValueData {
    pub const MAX_LEN: usize = 32768;

    pub fn new(data: Vec<u8>, writer: PublicKey) -> Self {
        assert!(data.len() <= Self::MAX_LEN);
        Self {
            seq: 0,
            data,
            writer,
        }
    }
    pub fn new_with_seq(seq: ValueSeqNum, data: Vec<u8>, writer: PublicKey) -> Self {
        assert!(data.len() <= Self::MAX_LEN);
        Self { seq, data, writer }
    }

    pub fn seq(&self) -> ValueSeqNum {
        self.seq
    }

    pub fn writer(&self) -> &PublicKey {
        &self.writer
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn total_size(&self) -> usize {
        mem::size_of::<Self>() + self.data.len()
    }
}
