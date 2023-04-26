use super::*;

/// The state associated with a local record when it is opened
/// This is not serialized to storage as it is ephemeral for the lifetime of the opened record
#[derive(Clone, Debug, Default)]
pub struct OpenedRecord {
    /// The key pair used to perform writes to subkey on this opened record
    /// Without this, set_value() will fail regardless of which key or subkey is being written to
    /// as all writes are signed
    writer: Option<KeyPair>,
}

impl OpenedRecord {
    pub fn new(writer: Option<KeyPair>) -> Self {
        Self { writer }
    }

    pub fn writer(&self) -> Option<&KeyPair> {
        self.writer.as_ref()
    }
}
