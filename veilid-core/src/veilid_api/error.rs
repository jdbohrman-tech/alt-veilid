use super::*;

#[allow(unused_macros)]
#[macro_export]
macro_rules! apibail_timeout {
    () => {
        return Err(VeilidAPIError::timeout())
    };
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! apibail_try_again {
    () => {
        return Err(VeilidAPIError::try_again())
    };
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! apibail_generic {
    ($x:expr) => {
        return Err(VeilidAPIError::generic($x))
    };
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! apibail_internal {
    ($x:expr) => {
        return Err(VeilidAPIError::internal($x))
    };
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! apibail_parse_error {
    ($x:expr, $y:expr) => {
        return Err(VeilidAPIError::parse_error($x, $y))
    };
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! apibail_missing_argument {
    ($x:expr, $y:expr) => {
        return Err(VeilidAPIError::missing_argument($x, $y))
    };
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! apibail_invalid_argument {
    ($x:expr, $y:expr, $z:expr) => {
        return Err(VeilidAPIError::invalid_argument($x, $y, $z))
    };
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! apibail_no_connection {
    ($x:expr) => {
        return Err(VeilidAPIError::no_connection($x))
    };
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! apibail_key_not_found {
    ($x:expr) => {
        return Err(VeilidAPIError::key_not_found($x))
    };
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! apibail_already_initialized {
    () => {
        return Err(VeilidAPIError::already_initialized())
    };
}

#[derive(
    ThisError,
    Clone,
    Debug,
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
#[archive_attr(repr(u8), derive(CheckBytes))]
#[serde(tag = "kind")]
pub enum VeilidAPIError {
    #[error("Not initialized")]
    NotInitialized,
    #[error("Already initialized")]
    AlreadyInitialized,
    #[error("Timeout")]
    Timeout,
    #[error("TryAgain")]
    TryAgain,
    #[error("Shutdown")]
    Shutdown,
    #[error("Key not found: {key}")]
    KeyNotFound { key: PublicKey },
    #[error("No connection: {message}")]
    NoConnection { message: String },
    #[error("No peer info: {node_id}")]
    NoPeerInfo { node_id: NodeId },
    #[error("Internal: {message}")]
    Internal { message: String },
    #[error("Unimplemented: {message}")]
    Unimplemented { message: String },
    #[error("Parse error: '{message}' with value '{value}'")]
    ParseError { message: String, value: String },
    #[error("Invalid argument: '{argument}' for '{context}' with value '{value}'")]
    InvalidArgument {
        context: String,
        argument: String,
        value: String,
    },
    #[error("Missing argument: '{argument}' for '{context}'")]
    MissingArgument { context: String, argument: String },
    #[error("Generic: {message}")]
    Generic { message: String },
}

impl VeilidAPIError {
    pub fn not_initialized() -> Self {
        Self::NotInitialized
    }
    pub fn already_initialized() -> Self {
        Self::AlreadyInitialized
    }
    pub fn timeout() -> Self {
        Self::Timeout
    }
    pub fn try_again() -> Self {
        Self::TryAgain
    }
    pub fn shutdown() -> Self {
        Self::Shutdown
    }
    pub fn key_not_found(key: PublicKey) -> Self {
        Self::KeyNotFound { key }
    }
    pub fn no_connection<T: ToString>(msg: T) -> Self {
        Self::NoConnection {
            message: msg.to_string(),
        }
    }
    pub fn no_peer_info(node_id: NodeId) -> Self {
        Self::NoPeerInfo { node_id }
    }
    pub fn internal<T: ToString>(msg: T) -> Self {
        Self::Internal {
            message: msg.to_string(),
        }
    }
    pub fn unimplemented<T: ToString>(msg: T) -> Self {
        Self::Unimplemented {
            message: msg.to_string(),
        }
    }
    pub fn parse_error<T: ToString, S: ToString>(msg: T, value: S) -> Self {
        Self::ParseError {
            message: msg.to_string(),
            value: value.to_string(),
        }
    }
    pub fn invalid_argument<T: ToString, S: ToString, R: ToString>(
        context: T,
        argument: S,
        value: R,
    ) -> Self {
        Self::InvalidArgument {
            context: context.to_string(),
            argument: argument.to_string(),
            value: value.to_string(),
        }
    }
    pub fn missing_argument<T: ToString, S: ToString>(context: T, argument: S) -> Self {
        Self::MissingArgument {
            context: context.to_string(),
            argument: argument.to_string(),
        }
    }
    pub fn generic<T: ToString>(msg: T) -> Self {
        Self::Generic {
            message: msg.to_string(),
        }
    }
}
