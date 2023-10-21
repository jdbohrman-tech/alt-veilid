use super::*;

#[allow(unused_macros)]
#[macro_export]
macro_rules! apibail_not_initialized {
    () => {
        return Err(VeilidAPIError::not_initialized())
    };
}

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
    ($x:expr) => {
        return Err(VeilidAPIError::try_again($x))
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
macro_rules! apibail_invalid_target {
    ($x:expr) => {
        return Err(VeilidAPIError::invalid_target($x))
    };
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! apibail_route_not_found {
    ($x:expr) => {
        return Err(VeilidAPIError::route_not_found($x))
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
    ThisError, Clone, Debug, PartialOrd, PartialEq, Eq, Ord, Serialize, Deserialize, JsonSchema,
)]
#[cfg_attr(target_arch = "wasm32", derive(Tsify), tsify(into_wasm_abi))]
#[serde(tag = "kind")]
pub enum VeilidAPIError {
    #[error("Not initialized")]
    NotInitialized,
    #[error("Already initialized")]
    AlreadyInitialized,
    #[error("Timeout")]
    Timeout,
    #[error("TryAgain: {message}")]
    TryAgain { message: String },
    #[error("Shutdown")]
    Shutdown,
    #[error("Invalid target: {message}")]
    InvalidTarget { message: String },
    #[error("No connection: {message}")]
    NoConnection { message: String },
    #[error("Key not found: {key}")]
    KeyNotFound {
        #[schemars(with = "String")]
        key: TypedKey,
    },
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
from_impl_to_jsvalue!(VeilidAPIError);

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
    pub fn try_again<T: ToString>(msg: T) -> Self {
        Self::TryAgain {
            message: msg.to_string(),
        }
    }
    pub fn shutdown() -> Self {
        Self::Shutdown
    }
    pub fn invalid_target<T: ToString>(msg: T) -> Self {
        Self::InvalidTarget {
            message: msg.to_string(),
        }
    }
    pub fn no_connection<T: ToString>(msg: T) -> Self {
        Self::NoConnection {
            message: msg.to_string(),
        }
    }
    pub fn key_not_found(key: TypedKey) -> Self {
        Self::KeyNotFound { key }
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

    pub(crate) fn from_network_result<T>(nr: NetworkResult<T>) -> Result<T, Self> {
        match nr {
            NetworkResult::Timeout => Err(VeilidAPIError::timeout()),
            NetworkResult::ServiceUnavailable(m) => Err(VeilidAPIError::invalid_target(m)),
            NetworkResult::NoConnection(m) => Err(VeilidAPIError::no_connection(m)),
            NetworkResult::AlreadyExists(m) => {
                Err(VeilidAPIError::generic(format!("Already exists: {}", m)))
            }
            NetworkResult::InvalidMessage(m) => {
                Err(VeilidAPIError::parse_error("Invalid message", m))
            }
            NetworkResult::Value(v) => Ok(v),
        }
    }
}

pub type VeilidAPIResult<T> = Result<T, VeilidAPIError>;

impl From<std::io::Error> for VeilidAPIError {
    fn from(e: std::io::Error) -> Self {
        match e.kind() {
            std::io::ErrorKind::TimedOut => VeilidAPIError::timeout(),
            std::io::ErrorKind::ConnectionRefused => VeilidAPIError::no_connection(e.to_string()),
            std::io::ErrorKind::ConnectionReset => VeilidAPIError::no_connection(e.to_string()),
            #[cfg(feature = "io_error_more")]
            std::io::ErrorKind::HostUnreachable => VeilidAPIError::no_connection(e.to_string()),
            #[cfg(feature = "io_error_more")]
            std::io::ErrorKind::NetworkUnreachable => VeilidAPIError::no_connection(e.to_string()),
            std::io::ErrorKind::ConnectionAborted => VeilidAPIError::no_connection(e.to_string()),
            std::io::ErrorKind::NotConnected => VeilidAPIError::no_connection(e.to_string()),
            std::io::ErrorKind::AddrInUse => VeilidAPIError::no_connection(e.to_string()),
            std::io::ErrorKind::AddrNotAvailable => VeilidAPIError::no_connection(e.to_string()),
            #[cfg(feature = "io_error_more")]
            std::io::ErrorKind::NetworkDown => VeilidAPIError::no_connection(e.to_string()),
            #[cfg(feature = "io_error_more")]
            std::io::ErrorKind::ReadOnlyFilesystem => VeilidAPIError::internal(e.to_string()),
            #[cfg(feature = "io_error_more")]
            std::io::ErrorKind::NotSeekable => VeilidAPIError::internal(e.to_string()),
            #[cfg(feature = "io_error_more")]
            std::io::ErrorKind::FilesystemQuotaExceeded => VeilidAPIError::internal(e.to_string()),
            #[cfg(feature = "io_error_more")]
            std::io::ErrorKind::Deadlock => VeilidAPIError::internal(e.to_string()),
            std::io::ErrorKind::Unsupported => VeilidAPIError::internal(e.to_string()),
            std::io::ErrorKind::OutOfMemory => VeilidAPIError::internal(e.to_string()),
            _ => VeilidAPIError::generic(e.to_string()),
        }
    }
}
