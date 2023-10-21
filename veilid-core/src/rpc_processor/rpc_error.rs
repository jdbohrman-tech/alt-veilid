use super::*;

#[derive(ThisError, Debug, Clone, PartialOrd, PartialEq, Eq, Ord)]
#[must_use]
pub enum RPCError {
    #[error("[RPCError: Unimplemented({0})]")]
    Unimplemented(String),
    #[error("[RPCError: InvalidFormat({0})]")]
    InvalidFormat(String),
    #[error("[RPCError: Protocol({0})]")]
    Protocol(String),
    #[error("[RPCError: Internal({0})]")]
    Internal(String),
    #[error("[RPCError: Network({0})]")]
    Network(String),
    #[error("[RPCError: TryAgain({0})]")]
    TryAgain(String),
}

impl RPCError {
    pub fn unimplemented<X: ToString>(x: X) -> Self {
        Self::Unimplemented(x.to_string())
    }
    pub fn invalid_format<X: ToString>(x: X) -> Self {
        Self::InvalidFormat(x.to_string())
    }
    pub fn map_invalid_format<M: ToString, X: ToString>(message: M) -> impl FnOnce(X) -> Self {
        move |x| Self::InvalidFormat(format!("{}: {}", message.to_string(), x.to_string()))
    }
    pub fn protocol<X: ToString>(x: X) -> Self {
        Self::Protocol(x.to_string())
    }
    pub fn map_protocol<M: ToString, X: ToString>(message: M) -> impl FnOnce(X) -> Self {
        move |x| Self::Protocol(format!("{}: {}", message.to_string(), x.to_string()))
    }
    pub fn internal<X: ToString>(x: X) -> Self {
        Self::Internal(x.to_string())
    }
    pub fn map_internal<M: ToString, X: ToString>(message: M) -> impl FnOnce(X) -> Self {
        move |x| Self::Internal(format!("{}: {}", message.to_string(), x.to_string()))
    }
    pub fn else_internal<M: ToString>(message: M) -> impl FnOnce() -> Self {
        move || Self::Internal(message.to_string())
    }
    pub fn network<X: ToString>(x: X) -> Self {
        Self::Network(x.to_string())
    }
    pub fn map_network<M: ToString, X: ToString>(message: M) -> impl FnOnce(X) -> Self {
        move |x| Self::Network(format!("{}: {}", message.to_string(), x.to_string()))
    }
}

impl From<RPCError> for VeilidAPIError {
    fn from(e: RPCError) -> Self {
        match e {
            RPCError::Unimplemented(message) => VeilidAPIError::Unimplemented { message },
            RPCError::InvalidFormat(message) => VeilidAPIError::Generic { message },
            RPCError::Protocol(message) => VeilidAPIError::Generic { message },
            RPCError::Internal(message) => VeilidAPIError::Internal { message },
            RPCError::Network(message) => VeilidAPIError::Generic { message },
            RPCError::TryAgain(message) => VeilidAPIError::TryAgain { message },
        }
    }
}

pub(crate) type RPCNetworkResult<T> = Result<NetworkResult<T>, RPCError>;

pub(crate) trait ToRPCNetworkResult<T> {
    fn to_rpc_network_result(self) -> RPCNetworkResult<T>;
}

impl<T> ToRPCNetworkResult<T> for VeilidAPIResult<T> {
    fn to_rpc_network_result(self) -> RPCNetworkResult<T> {
        match self {
            Err(VeilidAPIError::TryAgain { message }) => Err(RPCError::TryAgain(message)),
            Err(VeilidAPIError::Timeout) => Ok(NetworkResult::timeout()),
            Err(VeilidAPIError::Unimplemented { message }) => Err(RPCError::Unimplemented(message)),
            Err(e) => Err(RPCError::internal(e)),
            Ok(v) => Ok(NetworkResult::value(v)),
        }
    }
}
