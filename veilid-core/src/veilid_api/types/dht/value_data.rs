use super::*;
use veilid_api::VeilidAPIResult;

#[derive(Clone, Default, PartialOrd, PartialEq, Eq, Ord, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(
    all(target_arch = "wasm32", target_os = "unknown"),
    derive(Tsify),
    tsify(into_wasm_abi)
)]
#[must_use]
pub struct ValueData {
    /// An increasing sequence number to time-order the DHT record changes
    seq: ValueSeqNum,

    /// The contents of a DHT Record
    #[cfg_attr(
        not(all(target_arch = "wasm32", target_os = "unknown")),
        serde(with = "as_human_base64")
    )]
    #[schemars(with = "String")]
    #[cfg_attr(
        all(target_arch = "wasm32", target_os = "unknown"),
        serde(with = "serde_bytes"),
        tsify(type = "Uint8Array")
    )]
    data: Vec<u8>,

    /// The public identity key of the writer of the data
    #[schemars(with = "String")]
    writer: PublicKey,
}

impl ValueData {
    pub const MAX_LEN: usize = 32768;

    pub fn new(data: Vec<u8>, writer: PublicKey) -> VeilidAPIResult<Self> {
        if data.len() > Self::MAX_LEN {
            apibail_generic!("invalid size");
        }
        Ok(Self {
            seq: 0,
            data,
            writer,
        })
    }
    pub fn new_with_seq(
        seq: ValueSeqNum,
        data: Vec<u8>,
        writer: PublicKey,
    ) -> VeilidAPIResult<Self> {
        if data.len() > Self::MAX_LEN {
            apibail_generic!("invalid size");
        }
        Ok(Self { seq, data, writer })
    }

    #[must_use]
    pub fn seq(&self) -> ValueSeqNum {
        self.seq
    }

    pub fn writer(&self) -> &PublicKey {
        &self.writer
    }

    #[must_use]
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    #[must_use]
    pub fn data_size(&self) -> usize {
        self.data.len()
    }

    #[must_use]
    pub fn total_size(&self) -> usize {
        mem::size_of::<Self>() + self.data.len()
    }
}

impl fmt::Debug for ValueData {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("ValueData")
            .field("seq", &self.seq)
            .field("data", &print_data(&self.data, Some(64)))
            .field("writer", &self.writer)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_data_ok() {
        assert!(ValueData::new(vec![0; ValueData::MAX_LEN], PublicKey { bytes: [0; 32] }).is_ok());
        assert!(ValueData::new_with_seq(
            0,
            vec![0; ValueData::MAX_LEN],
            PublicKey { bytes: [0; 32] }
        )
        .is_ok());
    }

    #[test]
    fn value_data_too_long() {
        assert!(ValueData::new(
            vec![0; ValueData::MAX_LEN + 1],
            PublicKey { bytes: [0; 32] }
        )
        .is_err());
        assert!(ValueData::new_with_seq(
            0,
            vec![0; ValueData::MAX_LEN + 1],
            PublicKey { bytes: [0; 32] }
        )
        .is_err());
    }
}
