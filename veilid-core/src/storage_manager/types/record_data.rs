use super::*;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub(in crate::storage_manager) struct RecordData {
    signed_value_data: Arc<SignedValueData>,
}

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
        mem::size_of::<Self>() + self.signed_value_data.total_size()
    }
}
