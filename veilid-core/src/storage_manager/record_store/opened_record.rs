use super::*;

/// The state associated with a local record when it is opened
/// This is not serialized to storage as it is ephemeral for the lifetime of the opened record
#[derive(Clone, Debug, Default)]
pub(in crate::storage_manager) struct OpenedRecord {
    /// The key pair used to perform writes to subkey on this opened record
    /// Without this, set_value() will fail regardless of which key or subkey is being written to
    /// as all writes are signed
    writer: Option<KeyPair>,

    /// The safety selection in current use
    safety_selection: SafetySelection,
}

impl OpenedRecord {
    pub fn new(writer: Option<KeyPair>, safety_selection: SafetySelection) -> Self {
        Self {
            writer,
            safety_selection,
        }
    }

    pub fn writer(&self) -> Option<&KeyPair> {
        self.writer.as_ref()
    }
    pub fn set_writer(&mut self, writer: Option<KeyPair>) {
        self.writer = writer;
    }

    pub fn safety_selection(&self) -> SafetySelection {
        self.safety_selection
    }
    pub fn set_safety_selection(&mut self, safety_selection: SafetySelection) {
        self.safety_selection = safety_selection;
    }
}
