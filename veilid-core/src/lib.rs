//! # The Veilid Framework
//!
//! Core library used to create a Veilid node and operate it as part of an application.
//!
//! `veilid-core` contains all of the core logic for Veilid and can be used in mobile applications as well as desktop
//! and in-browser WebAssembly apps.
//!
//! The public API is accessed by getting a [VeilidAPI] object via a call to [api_startup], [api_startup_json], or
//! [api_startup_config].
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
#![allow(clippy::comparison_chain, clippy::upper_case_acronyms)]
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

mod attachment_manager;
mod core_context;
mod crypto;
mod intf;
mod logging;
mod network_manager;
mod routing_table;
mod rpc_processor;
mod storage_manager;
mod table_store;
mod veilid_api;
mod veilid_config;
mod wasm_helpers;

pub use self::core_context::{api_startup, api_startup_config, api_startup_json, UpdateCallback};
pub use self::logging::{
    ApiTracingLayer, VeilidLayerFilter, DEFAULT_LOG_FACILITIES_ENABLED_LIST,
    DEFAULT_LOG_FACILITIES_IGNORE_LIST, DURATION_LOG_FACILITIES,
};
pub use self::veilid_api::*;
pub use self::veilid_config::*;
pub use veilid_tools as tools;

/// The on-the-wire serialization format for Veilid RPC
pub mod veilid_capnp {
    include!("../proto/veilid_capnp.rs");
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
use veilid_tools::*;
use wasm_helpers::*;
