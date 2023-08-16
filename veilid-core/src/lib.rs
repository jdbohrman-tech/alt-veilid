//! # The Veilid Framework
//!
//! Core library used to create a Veilid node and operate it as part of an application.
//!
//! `veilid-core` contains all of the core logic for Veilid and can be used in mobile applications as well as desktop
//! and in-browser WebAssembly apps.
//!
//! The public API is accessed by getting a [VeilidAPI] object via a call to [api_startup] or [api_startup_json].
//!
//! From there, a [RoutingContext] object can get you access to public and private routed operations.
//!
//! ## Features
//!
//! The default `veilid-core` configurations are:
//!
//! * `default` - Uses `tokio` as the async runtime
//!
//! If you use `--no-default-features`, you can switch to other runtimes:
//!
//! * `default-async-std` - Uses `async-std` as the async runtime
//! * `default-wasm` - When building for the `wasm32` architecture, use this to enable `wasm-bindgen-futures` as the async runtime
//!

#![deny(clippy::all)]
#![deny(unused_must_use)]
#![recursion_limit = "256"]

cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        #[cfg(any(feature = "rt-async-std", feature = "rt-tokio"))]
        compile_error!("features \"rt-async-std\" and \"rt-tokio\" can not be specified for WASM");
    } else {
        #[cfg(all(feature = "rt-async-std", feature = "rt-tokio"))]
        compile_error!(
            "feature \"rt-async-std\" and feature \"rt-tokio\" cannot be enabled at the same time"
        );
        #[cfg(not(any(feature = "rt-async-std", feature = "rt-tokio")))]
        compile_error!("exactly one of feature \"rt-async-std\" or feature \"rt-tokio\" must be specified");
    }
}

#[macro_use]
extern crate alloc;

mod api_tracing_layer;
mod attachment_manager;
mod core_context;
mod crypto;
mod intf;
mod network_manager;
mod receipt_manager;
mod routing_table;
mod rpc_processor;
mod storage_manager;
mod table_store;
mod veilid_api;
mod veilid_config;
mod veilid_layer_filter;

pub use self::api_tracing_layer::ApiTracingLayer;
pub use self::core_context::{api_startup, api_startup_json, UpdateCallback};
pub use self::veilid_api::*;
pub use self::veilid_config::*;
pub use self::veilid_layer_filter::*;
pub use veilid_tools as tools;

/// The on-the-wire serialization format for Veilid RPC
pub mod veilid_capnp {
    include!(concat!(env!("OUT_DIR"), "/proto/veilid_capnp.rs"));
}

#[doc(hidden)]
pub mod tests;

/// Return the cargo package version of veilid-core in string format
pub fn veilid_version_string() -> String {
    env!("CARGO_PKG_VERSION").to_owned()
}

/// Return the cargo package version of veilid-core in tuple format
pub fn veilid_version() -> (u32, u32, u32) {
    (
        u32::from_str(env!("CARGO_PKG_VERSION_MAJOR")).unwrap(),
        u32::from_str(env!("CARGO_PKG_VERSION_MINOR")).unwrap(),
        u32::from_str(env!("CARGO_PKG_VERSION_PATCH")).unwrap(),
    )
}

#[cfg(target_os = "android")]
pub use intf::android::veilid_core_setup_android;

pub static DEFAULT_LOG_IGNORE_LIST: [&str; 23] = [
    "mio",
    "h2",
    "hyper",
    "tower",
    "tonic",
    "tokio",
    "runtime",
    "tokio_util",
    "want",
    "serial_test",
    "async_std",
    "async_io",
    "polling",
    "rustls",
    "async_tungstenite",
    "tungstenite",
    "netlink_proto",
    "netlink_sys",
    "trust_dns_resolver",
    "trust_dns_proto",
    "attohttpc",
    "ws_stream_wasm",
    "keyvaluedb_web",
];

use cfg_if::*;
use enumset::*;
use eyre::{bail, eyre, Report as EyreReport, Result as EyreResult, WrapErr};
#[allow(unused_imports)]
use futures_util::stream::{FuturesOrdered, FuturesUnordered};
use parking_lot::*;
use schemars::{schema_for, JsonSchema};
use serde::*;
use stop_token::*;
use thiserror::Error as ThisError;
use tracing::*;
use tsify::*;
use tsify_async::*;
use veilid_tools::*;
use wasm_bindgen::prelude::*;
