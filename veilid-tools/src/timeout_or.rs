use super::*;

use core::fmt::{Debug, Display};
use core::result::Result;
use std::error::Error;
use std::io;

#[derive(ThisError, Debug, Clone, Copy, Eq, PartialEq)]
#[error("Timeout")]
pub struct TimeoutError();

impl TimeoutError {
    #[must_use]
    pub fn to_io(self) -> io::Error {
        io::Error::new(io::ErrorKind::TimedOut, self)
    }
}

cfg_if! {
    if #[cfg(feature="rt-async-std")] {
        impl From<async_std::future::TimeoutError> for TimeoutError {
            fn from(_: async_std::future::TimeoutError) -> Self {
                Self()
            }
        }
    } else if #[cfg(feature="rt-tokio")] {
        impl From<tokio::time::error::Elapsed> for TimeoutError {
            fn from(_: tokio::time::error::Elapsed) -> Self {
                Self()
            }
        }
    }
}

//////////////////////////////////////////////////////////////////
// Non-fallible timeout conversions

pub trait TimeoutOrExt<T> {
    fn into_timeout_or(self) -> TimeoutOr<T>;
}

impl<T> TimeoutOrExt<T> for Result<T, TimeoutError> {
    fn into_timeout_or(self) -> TimeoutOr<T> {
        self.ok()
            .map(|v| TimeoutOr::<T>::Value(v))
            .unwrap_or(TimeoutOr::<T>::Timeout)
    }
}

pub trait IoTimeoutOrExt<T> {
    fn into_timeout_or(self) -> io::Result<TimeoutOr<T>>;
}

impl<T> IoTimeoutOrExt<T> for io::Result<T> {
    fn into_timeout_or(self) -> io::Result<TimeoutOr<T>> {
        match self {
            Ok(v) => Ok(TimeoutOr::<T>::Value(v)),
            Err(e) if e.kind() == io::ErrorKind::TimedOut => Ok(TimeoutOr::<T>::Timeout),
            Err(e) => Err(e),
        }
    }
}

pub trait TimeoutOrResultExt<T, E> {
    fn into_result(self) -> Result<TimeoutOr<T>, E>;
}

impl<T, E> TimeoutOrResultExt<T, E> for TimeoutOr<Result<T, E>> {
    fn into_result(self) -> Result<TimeoutOr<T>, E> {
        match self {
            TimeoutOr::<Result<T, E>>::Timeout => Ok(TimeoutOr::<T>::Timeout),
            TimeoutOr::<Result<T, E>>::Value(Ok(v)) => Ok(TimeoutOr::<T>::Value(v)),
            TimeoutOr::<Result<T, E>>::Value(Err(e)) => Err(e),
        }
    }
}

//////////////////////////////////////////////////////////////////
// Non-fallible timeout

#[must_use]
pub enum TimeoutOr<T> {
    Timeout,
    Value(T),
}

impl<T> TimeoutOr<T> {
    pub fn timeout() -> Self {
        Self::Timeout
    }
    pub fn value(value: T) -> Self {
        Self::Value(value)
    }

    pub fn is_timeout(&self) -> bool {
        matches!(self, Self::Timeout)
    }

    pub fn is_value(&self) -> bool {
        matches!(self, Self::Value(_))
    }
    pub fn map<X, F: Fn(T) -> X>(self, f: F) -> TimeoutOr<X> {
        match self {
            Self::Timeout => TimeoutOr::<X>::Timeout,
            Self::Value(v) => TimeoutOr::<X>::Value(f(v)),
        }
    }
    pub fn on_timeout<F: Fn()>(self, f: F) -> Self {
        match self {
            Self::Timeout => {
                f();
                Self::Timeout
            }
            Self::Value(v) => Self::Value(v),
        }
    }
    pub fn into_timeout_error(self) -> Result<T, TimeoutError> {
        match self {
            Self::Timeout => Err(TimeoutError {}),
            Self::Value(v) => Ok(v),
        }
    }

    pub fn into_option(self) -> Option<T> {
        match self {
            Self::Timeout => None,
            Self::Value(v) => Some(v),
        }
    }
}

impl<T> From<TimeoutOr<T>> for Option<T> {
    fn from(t: TimeoutOr<T>) -> Self {
        match t {
            TimeoutOr::<T>::Timeout => None,
            TimeoutOr::<T>::Value(v) => Some(v),
        }
    }
}

impl<T: Clone> Clone for TimeoutOr<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Timeout => Self::Timeout,
            Self::Value(t) => Self::Value(t.clone()),
        }
    }
}
impl<T: Debug> Debug for TimeoutOr<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Timeout => write!(f, "Timeout"),
            Self::Value(arg0) => f.debug_tuple("Value").field(arg0).finish(),
        }
    }
}
impl<T: Display> Display for TimeoutOr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Timeout => write!(f, ""),
            Self::Value(arg0) => write!(f, "{}", arg0),
        }
    }
}
impl<T: Debug + Display> Error for TimeoutOr<T> {}

//////////////////////////////////////////////////////////////////
// Non-fallible timeoue macros

#[macro_export]
macro_rules! timeout_or_try {
    ($r: expr) => {
        match $r {
            TimeoutOr::Timeout => return Ok(TimeoutOr::Timeout),
            TimeoutOr::Value(v) => v,
        }
    };
}
