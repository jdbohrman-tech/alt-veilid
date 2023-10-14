//! # Veilid Tools
//!
//! A collection of baseline tools for Rust development use by Veilid and Veilid-enabled Rust applications
//!
//! These are used by `veilid-core`, `veilid-server`, `veilid-cli` and may be used by any other applications
//! that link in `veilid-core` if a common baseline of functionality is desired. Extending this crate with new
//! utility functions is encouraged rather than adding 'common' functionality to `veilid-core`, allowing it to
//! remain free of boilerplate and utility classes that could be reused elsewhere.
//!
//! Everything added to this crate must be extensively unit-tested.
//!
//! ## Features
//!
//! The default `veilid-tools` configurations are:
//!
//! * `default` - Uses `tokio` as the async runtime
//!
//! If you use `--no-default-features`, you can switch to other runtimes:
//!
//! * `rt-async-std` - Uses `async-std` as the async runtime
//! * `rt-wasm-bindgen` - When building for the `wasm32` architecture, use this to enable `wasm-bindgen-futures` as the async runtime
//!
#![deny(clippy::all)]
#![allow(clippy::comparison_chain, clippy::upper_case_acronyms)]
#![deny(unused_must_use)]

// pub mod bump_port;
pub mod assembly_buffer;
pub mod async_peek_stream;
pub mod async_tag_lock;
pub mod clone_stream;
pub mod eventual;
pub mod eventual_base;
pub mod eventual_value;
pub mod eventual_value_clone;
pub mod interval;
pub mod ip_addr_port;
pub mod ip_extra;
pub mod log_thru;
pub mod must_join_handle;
pub mod must_join_single_future;
pub mod mutable_future;
#[cfg(not(target_arch = "wasm32"))]
pub mod network_interfaces;
pub mod network_result;
pub mod random;
pub mod single_shot_eventual;
pub mod sleep;
pub mod spawn;
pub mod split_url;
pub mod tick_task;
pub mod timeout;
pub mod timeout_or;
pub mod timestamp;
pub mod tools;
#[cfg(target_arch = "wasm32")]
pub mod wasm;

pub type PinBox<T> = Pin<Box<T>>;
pub type PinBoxFuture<T> = PinBox<dyn Future<Output = T> + 'static>;
pub type PinBoxFutureLifetime<'a, T> = PinBox<dyn Future<Output = T> + 'a>;
pub type SendPinBoxFuture<T> = PinBox<dyn Future<Output = T> + Send + 'static>;
pub type SendPinBoxFutureLifetime<'a, T> = PinBox<dyn Future<Output = T> + Send + 'a>;

#[doc(no_inline)]
pub use std::borrow::{Cow, ToOwned};
#[doc(no_inline)]
pub use std::boxed::Box;
#[doc(no_inline)]
pub use std::cell::RefCell;
#[doc(no_inline)]
pub use std::cmp;
#[doc(no_inline)]
pub use std::collections::btree_map::BTreeMap;
#[doc(no_inline)]
pub use std::collections::btree_set::BTreeSet;
#[doc(no_inline)]
pub use std::collections::hash_map::HashMap;
#[doc(no_inline)]
pub use std::collections::hash_set::HashSet;
#[doc(no_inline)]
pub use std::collections::LinkedList;
#[doc(no_inline)]
pub use std::collections::VecDeque;
#[doc(no_inline)]
pub use std::convert::{TryFrom, TryInto};
#[doc(no_inline)]
pub use std::fmt;
#[doc(no_inline)]
pub use std::future::Future;
#[doc(no_inline)]
pub use std::mem;
#[doc(no_inline)]
pub use std::net::{
    IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6, ToSocketAddrs,
};
#[doc(no_inline)]
pub use std::ops::{Fn, FnMut, FnOnce};
#[doc(no_inline)]
pub use std::pin::Pin;
#[doc(no_inline)]
pub use std::rc::Rc;
#[doc(no_inline)]
pub use std::str::FromStr;
#[doc(no_inline)]
pub use std::string::{String, ToString};
#[doc(no_inline)]
pub use std::sync::atomic::{AtomicBool, Ordering};
#[doc(no_inline)]
pub use std::sync::{Arc, Weak};
#[doc(no_inline)]
pub use std::task;
#[doc(no_inline)]
pub use std::time::Duration;
#[doc(no_inline)]
pub use std::vec::Vec;

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        #[doc(no_inline)]
        pub use async_lock::Mutex as AsyncMutex;
        #[doc(no_inline)]
        pub use async_lock::MutexGuard as AsyncMutexGuard;
        #[doc(no_inline)]
        pub use async_lock::MutexGuardArc as AsyncMutexGuardArc;
        #[doc(no_inline)]
        pub use async_executors::JoinHandle as LowLevelJoinHandle;
    } else {
        cfg_if! {
            if #[cfg(feature="rt-async-std")] {
                #[doc(no_inline)]
                pub use async_std::sync::Mutex as AsyncMutex;
                #[doc(no_inline)]
                pub use async_std::sync::MutexGuard as AsyncMutexGuard;
                #[doc(no_inline)]
                pub use async_std::sync::MutexGuardArc as AsyncMutexGuardArc;
                #[doc(no_inline)]
                pub use async_std::task::JoinHandle as LowLevelJoinHandle;
            } else if #[cfg(feature="rt-tokio")] {
                #[doc(no_inline)]
                pub use tokio::sync::Mutex as AsyncMutex;
                #[doc(no_inline)]
                pub use tokio::sync::MutexGuard as AsyncMutexGuard;
                #[doc(no_inline)]
                pub use tokio::sync::OwnedMutexGuard as AsyncMutexGuardArc;
                #[doc(no_inline)]
                pub use tokio::task::JoinHandle as LowLevelJoinHandle;
            } else {
                compile_error!("needs executor implementation")
            }
        }
    }
}

// pub use bump_port::*;
#[doc(inline)]
pub use assembly_buffer::*;
#[doc(inline)]
pub use async_peek_stream::*;
#[doc(inline)]
pub use async_tag_lock::*;
#[doc(inline)]
pub use clone_stream::*;
#[doc(inline)]
pub use eventual::*;
#[doc(inline)]
pub use eventual_base::{EventualCommon, EventualResolvedFuture};
#[doc(inline)]
pub use eventual_value::*;
#[doc(inline)]
pub use eventual_value_clone::*;
#[doc(inline)]
pub use interval::*;
#[doc(inline)]
pub use ip_addr_port::*;
#[doc(inline)]
pub use ip_extra::*;
#[doc(inline)]
pub use log_thru::*;
#[doc(inline)]
pub use must_join_handle::*;
#[doc(inline)]
pub use must_join_single_future::*;
#[doc(inline)]
pub use mutable_future::*;
#[doc(inline)]
#[cfg(not(target_arch = "wasm32"))]
pub use network_interfaces::*;
#[doc(inline)]
pub use network_result::*;
#[doc(inline)]
pub use random::*;
#[doc(inline)]
pub use single_shot_eventual::*;
#[doc(inline)]
pub use sleep::*;
#[doc(inline)]
pub use spawn::*;
#[doc(inline)]
pub use split_url::*;
#[doc(inline)]
pub use tick_task::*;
#[doc(inline)]
pub use timeout::*;
#[doc(inline)]
pub use timeout_or::*;
#[doc(inline)]
pub use timestamp::*;
#[doc(inline)]
pub use tools::*;

#[cfg(target_arch = "wasm32")]
pub use wasm::*;

// Tests must be public for wasm-pack tests
pub mod tests;

cfg_if! {
    if #[cfg(feature = "tracing")] {
        use tracing::*;
    } else {
        use log::*;
    }
}
use cfg_if::*;
use futures_util::{AsyncRead, AsyncWrite};
use parking_lot::*;
use stop_token::*;
use thiserror::Error as ThisError;

pub use fn_name;

// For iOS tests
#[no_mangle]
pub extern "C" fn main_rs() {}
