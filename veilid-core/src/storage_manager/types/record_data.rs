use super::*;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct RecordData {
    signed_value_data: Arc<SignedValueData>,
}

xxx continue here, use arc everywhere to avoid copies

impl RecordData {
    pub fn new(signed_value_data: Arc<SignedValueData>) -> Self {
        Self { signed_value_data }
    }
    pub fn signed_value_data(&self) -> Arc<SignedValueData> {
        self.signed_value_data.clone()
    }
    pub fn data_size(&self) -> usize {
        self.signed_value_data.data_size()
    }
    pub fn total_size(&self) -> usize {
        (mem::size_of::<Self>() - mem::size_of::<SignedValueData>())
            + self.signed_value_data.total_size()
    }
}
