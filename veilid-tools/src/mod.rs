// mod bump_port;
mod async_peek_stream;
mod async_tag_lock;
mod clone_stream;
mod eventual;
mod eventual_base;
mod eventual_value;
mod eventual_value_clone;
mod interval;
mod ip_addr_port;
mod ip_extra;
mod log_thru;
mod must_join_handle;
mod must_join_single_future;
mod mutable_future;
mod network_result;
mod random;
mod single_shot_eventual;
mod sleep;
mod spawn;
mod split_url;
mod tick_task;
mod timeout;
mod timeout_or;
mod timestamp;
mod tools;
#[cfg(target_arch = "wasm32")]
mod wasm;

pub use cfg_if::*;
#[allow(unused_imports)]
pub use eyre::{bail, ensure, eyre, Report as EyreReport, Result as EyreResult, WrapErr};
pub use futures_util::future::{select, Either};
pub use futures_util::select;
pub use futures_util::stream::FuturesUnordered;
pub use futures_util::{AsyncRead, AsyncWrite};
pub use log_thru::*;
pub use owo_colors::OwoColorize;
pub use parking_lot::*;
pub use split_url::*;
pub use static_assertions::*;
pub use stop_token::*;
pub use thiserror::Error as ThisError;
cfg_if! {
    if #[cfg(feature = "tracing")] {
        pub use tracing::*;
    } else {
        pub use log::*;
    }
}
pub type PinBox<T> = Pin<Box<T>>;
pub type PinBoxFuture<T> = PinBox<dyn Future<Output = T> + 'static>;
pub type PinBoxFutureLifetime<'a, T> = PinBox<dyn Future<Output = T> + 'a>;
pub type SendPinBoxFuture<T> = PinBox<dyn Future<Output = T> + Send + 'static>;
pub type SendPinBoxFutureLifetime<'a, T> = PinBox<dyn Future<Output = T> + Send + 'a>;

pub use std::borrow::{Cow, ToOwned};
pub use std::boxed::Box;
pub use std::cell::RefCell;
pub use std::cmp;
pub use std::collections::btree_map::BTreeMap;
pub use std::collections::btree_set::BTreeSet;
pub use std::collections::hash_map::HashMap;
pub use std::collections::hash_set::HashSet;
pub use std::collections::LinkedList;
pub use std::collections::VecDeque;
pub use std::convert::{TryFrom, TryInto};
pub use std::fmt;
pub use std::future::Future;
pub use std::mem;
pub use std::net::{
    IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6, ToSocketAddrs,
};
pub use std::ops::{Fn, FnMut, FnOnce};
pub use std::pin::Pin;
pub use std::rc::Rc;
pub use std::string::String;
pub use std::sync::atomic::{AtomicBool, Ordering};
pub use std::sync::{Arc, Weak};
pub use std::task;
pub use std::time::Duration;
pub use std::vec::Vec;

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        pub use async_lock::Mutex as AsyncMutex;
        pub use async_lock::MutexGuard as AsyncMutexGuard;
        pub use async_lock::MutexGuardArc as AsyncMutexGuardArc;
        pub use async_executors::JoinHandle as LowLevelJoinHandle;
    } else {
        cfg_if! {
            if #[cfg(feature="rt-async-std")] {
                pub use async_std::sync::Mutex as AsyncMutex;
                pub use async_std::sync::MutexGuard as AsyncMutexGuard;
                pub use async_std::sync::MutexGuardArc as AsyncMutexGuardArc;
                pub use async_std::task::JoinHandle as LowLevelJoinHandle;
            } else if #[cfg(feature="rt-tokio")] {
                pub use tokio::sync::Mutex as AsyncMutex;
                pub use tokio::sync::MutexGuard as AsyncMutexGuard;
                pub use tokio::sync::OwnedMutexGuard as AsyncMutexGuardArc;
                pub use tokio::task::JoinHandle as LowLevelJoinHandle;
            } else {
                #[compile_error("must use an executor")]
            }
        }
    }
}

// pub use bump_port::*;
pub use async_peek_stream::*;
pub use async_tag_lock::*;
pub use clone_stream::*;
pub use eventual::*;
pub use eventual_base::{EventualCommon, EventualResolvedFuture};
pub use eventual_value::*;
pub use eventual_value_clone::*;
pub use interval::*;
pub use ip_addr_port::*;
pub use ip_extra::*;
pub use must_join_handle::*;
pub use must_join_single_future::*;
pub use mutable_future::*;
pub use network_result::*;
pub use random::*;
pub use single_shot_eventual::*;
pub use sleep::*;
pub use spawn::*;
pub use tick_task::*;
pub use timeout::*;
pub use timeout_or::*;
pub use timestamp::*;
pub use tools::*;
#[cfg(target_arch = "wasm32")]
pub use wasm::*;
