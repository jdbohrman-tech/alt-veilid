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
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct ValueData {
    pub seq: ValueSeqNum,
    pub data: Vec<u8>,
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
