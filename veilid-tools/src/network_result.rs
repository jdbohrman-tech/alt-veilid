use super::*;

use core::fmt::{Debug, Display};
use core::result::Result;
use std::error::Error;
use std::io;

//////////////////////////////////////////////////////////////////
// Non-fallible network results conversions

pub trait NetworkResultExt<T> {
    fn into_network_result(self) -> NetworkResult<T>;
}

impl<T> NetworkResultExt<T> for Result<T, TimeoutError> {
    fn into_network_result(self) -> NetworkResult<T> {
        self.ok()
            .map(|v| NetworkResult::<T>::Value(v))
            .unwrap_or(NetworkResult::<T>::Timeout)
    }
}

pub trait IoNetworkResultExt<T> {
    fn into_network_result(self) -> io::Result<NetworkResult<T>>;
}

impl<T> IoNetworkResultExt<T> for io::Result<T> {
    fn into_network_result(self) -> io::Result<NetworkResult<T>> {
        match self {
            Ok(v) => Ok(NetworkResult::Value(v)),
            #[cfg(feature = "io_error_more")]
            Err(e) => match e.kind() {
                io::ErrorKind::TimedOut => Ok(NetworkResult::Timeout),
                io::ErrorKind::UnexpectedEof
                | io::ErrorKind::ConnectionAborted
                | io::ErrorKind::ConnectionRefused
                | io::ErrorKind::ConnectionReset
                | io::ErrorKind::HostUnreachable
                | io::ErrorKind::NetworkUnreachable => Ok(NetworkResult::NoConnection(e)),
                io::ErrorKind::AddrNotAvailable => Ok(NetworkResult::AlreadyExists(e)),
                _ => Err(e),
            },
            #[cfg(not(feature = "io_error_more"))]
            Err(e) => {
                #[cfg(not(target_arch = "wasm32"))]
                if let Some(os_err) = e.raw_os_error() {
                    if os_err == libc::EHOSTUNREACH || os_err == libc::ENETUNREACH {
                        return Ok(NetworkResult::NoConnection(e));
                    }
                }
                match e.kind() {
                    io::ErrorKind::TimedOut => Ok(NetworkResult::Timeout),
                    io::ErrorKind::UnexpectedEof
                    | io::ErrorKind::ConnectionAborted
                    | io::ErrorKind::ConnectionRefused
                    | io::ErrorKind::ConnectionReset => Ok(NetworkResult::NoConnection(e)),
                    io::ErrorKind::AddrNotAvailable => Ok(NetworkResult::AlreadyExists(e)),
                    _ => Err(e),
                }
            }
        }
    }
}

pub trait NetworkResultResultExt<T, E> {
    fn into_result_network_result(self) -> Result<NetworkResult<T>, E>;
}

impl<T, E> NetworkResultResultExt<T, E> for NetworkResult<Result<T, E>> {
    fn into_result_network_result(self) -> Result<NetworkResult<T>, E> {
        match self {
            NetworkResult::Timeout => Ok(NetworkResult::<T>::Timeout),
            NetworkResult::ServiceUnavailable(s) => Ok(NetworkResult::<T>::ServiceUnavailable(s)),
            NetworkResult::NoConnection(e) => Ok(NetworkResult::<T>::NoConnection(e)),
            NetworkResult::AlreadyExists(e) => Ok(NetworkResult::<T>::AlreadyExists(e)),
            NetworkResult::InvalidMessage(s) => Ok(NetworkResult::<T>::InvalidMessage(s)),
            NetworkResult::Value(Ok(v)) => Ok(NetworkResult::<T>::Value(v)),
            NetworkResult::Value(Err(e)) => Err(e),
        }
    }
}

pub trait FoldedNetworkResultExt<T> {
    fn folded(self) -> io::Result<NetworkResult<T>>;
}

impl<T> FoldedNetworkResultExt<T> for io::Result<TimeoutOr<T>> {
    fn folded(self) -> io::Result<NetworkResult<T>> {
        match self {
            Ok(TimeoutOr::Timeout) => Ok(NetworkResult::Timeout),
            Ok(TimeoutOr::Value(v)) => Ok(NetworkResult::Value(v)),
            #[cfg(feature = "io_error_more")]
            Err(e) => match e.kind() {
                io::ErrorKind::TimedOut => Ok(NetworkResult::Timeout),
                io::ErrorKind::ConnectionAborted
                | io::ErrorKind::ConnectionRefused
                | io::ErrorKind::ConnectionReset
                | io::ErrorKind::HostUnreachable
                | io::ErrorKind::NetworkUnreachable => Ok(NetworkResult::NoConnection(e)),
                io::ErrorKind::AddrNotAvailable => Ok(NetworkResult::AlreadyExists(e)),
                _ => Err(e),
            },
            #[cfg(not(feature = "io_error_more"))]
            Err(e) => {
                #[cfg(not(target_arch = "wasm32"))]
                if let Some(os_err) = e.raw_os_error() {
                    if os_err == libc::EHOSTUNREACH || os_err == libc::ENETUNREACH {
                        return Ok(NetworkResult::NoConnection(e));
                    }
                }
                match e.kind() {
                    io::ErrorKind::TimedOut => Ok(NetworkResult::Timeout),
                    io::ErrorKind::ConnectionAborted
                    | io::ErrorKind::ConnectionRefused
                    | io::ErrorKind::ConnectionReset => Ok(NetworkResult::NoConnection(e)),
                    io::ErrorKind::AddrNotAvailable => Ok(NetworkResult::AlreadyExists(e)),
                    _ => Err(e),
                }
            }
        }
    }
}

impl<T> FoldedNetworkResultExt<T> for io::Result<NetworkResult<T>> {
    fn folded(self) -> io::Result<NetworkResult<T>> {
        match self {
            Ok(v) => Ok(v),
            #[cfg(feature = "io_error_more")]
            Err(e) => match e.kind() {
                io::ErrorKind::TimedOut => Ok(NetworkResult::Timeout),
                io::ErrorKind::ConnectionAborted
                | io::ErrorKind::ConnectionRefused
                | io::ErrorKind::ConnectionReset
                | io::ErrorKind::HostUnreachable
                | io::ErrorKind::NetworkUnreachable => Ok(NetworkResult::NoConnection(e)),
                io::ErrorKind::AddrNotAvailable => Ok(NetworkResult::AlreadyExists(e)),
                _ => Err(e),
            },
            #[cfg(not(feature = "io_error_more"))]
            Err(e) => {
                #[cfg(not(target_arch = "wasm32"))]
                if let Some(os_err) = e.raw_os_error() {
                    if os_err == libc::EHOSTUNREACH || os_err == libc::ENETUNREACH {
                        return Ok(NetworkResult::NoConnection(e));
                    }
                }
                match e.kind() {
                    io::ErrorKind::TimedOut => Ok(NetworkResult::Timeout),
                    io::ErrorKind::ConnectionAborted
                    | io::ErrorKind::ConnectionRefused
                    | io::ErrorKind::ConnectionReset => Ok(NetworkResult::NoConnection(e)),
                    io::ErrorKind::AddrNotAvailable => Ok(NetworkResult::AlreadyExists(e)),
                    _ => Err(e),
                }
            }
        }
    }
}

//////////////////////////////////////////////////////////////////
// Non-fallible network result

#[must_use]
pub enum NetworkResult<T> {
    Timeout,
    ServiceUnavailable(String),
    NoConnection(io::Error),
    AlreadyExists(io::Error),
    InvalidMessage(String),
    Value(T),
}

impl<T> NetworkResult<T> {
    pub fn timeout() -> Self {
        Self::Timeout
    }
    pub fn service_unavailable<S: ToString>(s: S) -> Self {
        Self::ServiceUnavailable(s.to_string())
    }
    pub fn no_connection(e: io::Error) -> Self {
        Self::NoConnection(e)
    }
    pub fn no_connection_other<S: ToString>(s: S) -> Self {
        Self::NoConnection(io::Error::new(io::ErrorKind::Other, s.to_string()))
    }
    pub fn invalid_message<S: ToString>(s: S) -> Self {
        Self::InvalidMessage(s.to_string())
    }
    pub fn already_exists(e: io::Error) -> Self {
        Self::AlreadyExists(e)
    }
    pub fn value(value: T) -> Self {
        Self::Value(value)
    }

    pub fn is_timeout(&self) -> bool {
        matches!(self, Self::Timeout)
    }
    pub fn is_no_connection(&self) -> bool {
        matches!(self, Self::NoConnection(_))
    }
    pub fn is_already_exists(&self) -> bool {
        matches!(self, Self::AlreadyExists(_))
    }
    pub fn is_invalid_message(&self) -> bool {
        matches!(self, Self::InvalidMessage(_))
    }
    pub fn is_value(&self) -> bool {
        matches!(self, Self::Value(_))
    }
    pub fn map<X, F: Fn(T) -> X>(self, f: F) -> NetworkResult<X> {
        match self {
            Self::Timeout => NetworkResult::<X>::Timeout,
            Self::ServiceUnavailable(s) => NetworkResult::<X>::ServiceUnavailable(s),
            Self::NoConnection(e) => NetworkResult::<X>::NoConnection(e),
            Self::AlreadyExists(e) => NetworkResult::<X>::AlreadyExists(e),
            Self::InvalidMessage(s) => NetworkResult::<X>::InvalidMessage(s),
            Self::Value(v) => NetworkResult::<X>::Value(f(v)),
        }
    }
    pub fn into_io_result(self) -> Result<T, io::Error> {
        match self {
            Self::Timeout => Err(io::Error::new(io::ErrorKind::TimedOut, "Timed out")),
            Self::ServiceUnavailable(s) => Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Service unavailable: {}", s),
            )),
            Self::NoConnection(e) => Err(e),
            Self::AlreadyExists(e) => Err(e),
            Self::InvalidMessage(s) => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid message: {}", s),
            )),
            Self::Value(v) => Ok(v),
        }
    }
}

impl<T> From<NetworkResult<T>> for Option<T> {
    fn from(t: NetworkResult<T>) -> Self {
        match t {
            NetworkResult::Value(v) => Some(v),
            _ => None,
        }
    }
}

impl<T: Debug> Debug for NetworkResult<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Timeout => write!(f, "Timeout"),
            Self::ServiceUnavailable(s) => f.debug_tuple("ServiceUnavailable").field(s).finish(),
            Self::NoConnection(e) => f.debug_tuple("NoConnection").field(e).finish(),
            Self::AlreadyExists(e) => f.debug_tuple("AlreadyExists").field(e).finish(),
            Self::InvalidMessage(s) => f.debug_tuple("InvalidMessage").field(s).finish(),
            Self::Value(v) => f.debug_tuple("Value").field(v).finish(),
        }
    }
}

impl<T> Display for NetworkResult<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Timeout => write!(f, "Timeout"),
            Self::ServiceUnavailable(s) => write!(f, "ServiceUnavailable({})", s),
            Self::NoConnection(e) => write!(f, "NoConnection({})", e.kind()),
            Self::AlreadyExists(e) => write!(f, "AlreadyExists({})", e.kind()),
            Self::InvalidMessage(s) => write!(f, "InvalidMessage({})", s),
            Self::Value(_) => write!(f, "Value"),
        }
    }
}

impl<T: Debug + Display> Error for NetworkResult<T> {}

//////////////////////////////////////////////////////////////////
// Non-fallible network result macros

#[macro_export]
macro_rules! network_result_raise {
    ($r: expr) => {
        match $r {
            NetworkResult::Timeout => return Ok(NetworkResult::Timeout),
            NetworkResult::ServiceUnavailable(s) => return Ok(NetworkResult::ServiceUnavailable(s)),
            NetworkResult::NoConnection(e) => return Ok(NetworkResult::NoConnection(e)),
            NetworkResult::AlreadyExists(e) => return Ok(NetworkResult::AlreadyExists(e)),
            NetworkResult::InvalidMessage(s) => return Ok(NetworkResult::InvalidMessage(s)),
            NetworkResult::Value(_) => panic!("Can not raise value"),
        }
    };
}

#[macro_export]
macro_rules! network_result_try {
    ($r: expr) => {
        match $r {
            NetworkResult::Timeout => return Ok(NetworkResult::Timeout),
            NetworkResult::ServiceUnavailable(s) => return Ok(NetworkResult::ServiceUnavailable(s)),
            NetworkResult::NoConnection(e) => return Ok(NetworkResult::NoConnection(e)),
            NetworkResult::AlreadyExists(e) => return Ok(NetworkResult::AlreadyExists(e)),
            NetworkResult::InvalidMessage(s) => return Ok(NetworkResult::InvalidMessage(s)),
            NetworkResult::Value(v) => v,
        }
    };
    ($r:expr => $f:tt) => {
        match $r {
            NetworkResult::Timeout => {
                $f;
                return Ok(NetworkResult::Timeout);
            }
            NetworkResult::ServiceUnavailable(s) => {
                $f;
                return Ok(NetworkResult::ServiceUnavailable(s));
            }
            NetworkResult::NoConnection(e) => {
                $f;
                return Ok(NetworkResult::NoConnection(e));
            }
            NetworkResult::AlreadyExists(e) => {
                $f;
                return Ok(NetworkResult::AlreadyExists(e));
            }
            NetworkResult::InvalidMessage(s) => {
                $f;
                return Ok(NetworkResult::InvalidMessage(s));
            }
            NetworkResult::Value(v) => v,
        }
    };
}

#[macro_export]
macro_rules! log_network_result {
    ($text:expr) => {
        cfg_if::cfg_if! {
            if #[cfg(feature="network-result-extra")] {
                info!(target: "network_result", "{}", format!("{}", $text))
            } else {
                debug!(target: "network_result", "{}", format!("{}", $text))
            }
        }
    };
    ($fmt:literal, $($arg:expr),+) => {
        cfg_if::cfg_if! {
            if #[cfg(feature="network-result-extra")] {
                info!(target: "network_result", "{}", format!($fmt, $($arg),+));
            } else {
                debug!(target: "network_result", "{}", format!($fmt, $($arg),+));
            }
        }
    };
}

#[macro_export]
macro_rules! network_result_value_or_log {
    ($r:expr => $f:expr) => {
        network_result_value_or_log!($r => [ "" ] $f )
    };
    ($r:expr => [ $d:expr ] $f:expr) => { {
        #[cfg(feature="network-result-extra")]
        let __extra_message = $d;
        #[cfg(not(feature="network-result-extra"))]
        let __extra_message = "";
        match $r {
            NetworkResult::Timeout => {
                log_network_result!(
                    "{} at {}@{}:{} in {}{}",
                    "Timeout",
                    file!(),
                    line!(),
                    column!(),
                    fn_name::uninstantiated!(),
                    __extra_message
                );
                $f
            }
            NetworkResult::ServiceUnavailable(ref s) => {
                log_network_result!(
                    "{}({}) at {}@{}:{} in {}{}",
                    "ServiceUnavailable",
                    s,
                    file!(),
                    line!(),
                    column!(),
                    fn_name::uninstantiated!(),
                    __extra_message
                );
                $f
            }
            NetworkResult::NoConnection(ref e) => {
                log_network_result!(
                    "{}({}) at {}@{}:{} in {}{}",
                    "No connection",
                    e.to_string(),
                    file!(),
                    line!(),
                    column!(),
                    fn_name::uninstantiated!(),
                    __extra_message
                );
                $f
            }
            NetworkResult::AlreadyExists(ref e) => {
                log_network_result!(
                    "{}({}) at {}@{}:{} in {}{}",
                    "Already exists",
                    e.to_string(),
                    file!(),
                    line!(),
                    column!(),
                    fn_name::uninstantiated!(),
                    __extra_message
                );
                $f
            }
            NetworkResult::InvalidMessage(ref s) => {
                log_network_result!(
                    "{}({}) at {}@{}:{} in {}{}",
                    "Invalid message",
                    s,
                    file!(),
                    line!(),
                    column!(),
                    fn_name::uninstantiated!(),
                    __extra_message
                );
                $f
            }
            NetworkResult::Value(v) => v,
        }
    } };

}
