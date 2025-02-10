use super::*;

use std::io;
#[derive(ThisError, Clone, Debug, PartialEq, Eq)]
pub enum VirtualNetworkError {
    #[error("Serialization Error: {0}")]
    SerializationError(postcard::Error),
    #[error("Response Mismatch")]
    ResponseMismatch,
    #[error("Wait error")]
    WaitError,
    #[error("Invalid machine id")]
    InvalidMachineId,
    #[error("Invalid socket id")]
    InvalidSocketId,
    #[error("Missing profile")]
    MissingProfile,
    #[error("Profile complete")]
    ProfileComplete,
    #[error("Io error: {0}")]
    IoError(io::ErrorKind),
}

impl From<VirtualNetworkError> for io::Error {
    fn from(value: VirtualNetworkError) -> Self {
        match value {
            VirtualNetworkError::IoError(e) => io::Error::from(e),
            e => io::Error::other(e),
        }
    }
}

pub type VirtualNetworkResult<T> = Result<T, VirtualNetworkError>;
