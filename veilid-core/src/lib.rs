//! # The Veilid Framework
//!
//! This is the core library used to create a Veilid node and operate it as part of an application.
//!
//! `veilid-core` contains all of the core logic for Veilid and can be used in mobile applications as well as desktop
//! and in-browser WebAssembly apps.
//!
//! ## Getting started
//!
//! - [Developer Book](https://veilid.gitlab.io/developer-book/)
//! - [Examples](https://gitlab.com/veilid/veilid/-/tree/main/veilid-core/examples/)
//! - [API Documentation](https://docs.rs/veilid-core)
//!
//! The public API is accessed by getting a [VeilidAPI] object via a call to [api_startup], [api_startup_json], or
//! [api_startup_config].
//!
//! From there, a [RoutingContext] object can get you access to public and private routed operations.
//!
//! ## Cargo features
//!
//! The default `veilid-core` configurations are:
//!
//! * `default` - Uses `tokio` as the async runtime.
//!
//! If you use `--no-default-features`, you can switch to other runtimes:
//!
//! * `default-async-std` - Uses `async-std` as the async runtime.
//! * `default-wasm` - When building for the `wasm32` architecture, use this to enable `wasm-bindgen-futures` as the async runtime.
//!

#![recursion_limit = "256"]

cfg_if::cfg_if! {
    if #[cfg(all(target_arch = "wasm32", target_os = "unknown"))] {
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
mod component;
mod core_context;
mod crypto;
mod intf;
mod logging;
mod network_manager;
mod routing_table;
mod rpc_processor;
mod stats_accounting;
mod storage_manager;
mod table_store;
mod veilid_api;
mod veilid_config;

pub(crate) use self::component::*;
pub(crate) use self::core_context::RegisteredComponents;
pub(crate) use self::stats_accounting::*;

pub(crate) use self::component::VeilidComponentGuard;
pub use self::core_context::{api_startup, api_startup_config, api_startup_json, UpdateCallback};
pub use self::logging::{
    ApiTracingLayer, FmtStripFields, VeilidLayerFilter, VeilidLayerLogKeyFilter,
    DEFAULT_LOG_FACILITIES_ENABLED_LIST, DEFAULT_LOG_FACILITIES_IGNORE_LIST,
    DURATION_LOG_FACILITIES, FLAME_LOG_FACILITIES_IGNORE_LIST, VEILID_LOG_KEY_FIELD,
};
pub use self::veilid_api::*;
pub use self::veilid_config::*;
pub use veilid_tools as tools;

/// The on-the-wire serialization format for Veilid RPC.
pub(crate) mod veilid_capnp {
    #![allow(
        clippy::all,
        clippy::must_use_candidate,
        clippy::large_futures,
        clippy::large_stack_arrays,
        clippy::large_stack_frames,
        clippy::large_types_passed_by_value,
        clippy::unused_async,
        clippy::ptr_cast_constness
    )]
    include!("../proto/veilid_capnp.rs");
}

#[doc(hidden)]
pub mod tests;

/// Return the cargo package version of veilid-core in string format.
#[must_use]
pub fn veilid_version_string() -> String {
    env!("CARGO_PKG_VERSION").to_owned()
}

/// Return the cargo package version of veilid-core in tuple format.
#[must_use]
pub fn veilid_version() -> (u32, u32, u32) {
    (
        u32::from_str(env!("CARGO_PKG_VERSION_MAJOR")).unwrap(),
        u32::from_str(env!("CARGO_PKG_VERSION_MINOR")).unwrap(),
        u32::from_str(env!("CARGO_PKG_VERSION_PATCH")).unwrap(),
    )
}

#[cfg(not(docsrs))]
include!(env!("BOSION_PATH"));

/// Return the features that were enabled when veilid-core was built.
#[must_use]
pub fn veilid_features() -> Vec<String> {
    if cfg!(docsrs) {
        vec!["default".to_string()]
    } else {
        let features = Bosion::CRATE_FEATURES.to_vec();
        features.into_iter().map(String::from).collect()
    }
}

#[cfg(target_os = "android")]
pub use intf::android::veilid_core_setup_android;

use cfg_if::*;
use enumset::*;
use eyre::{bail, eyre, Report as EyreReport, Result as EyreResult, WrapErr};
#[allow(unused_imports)]
use futures_util::stream::{FuturesOrdered, FuturesUnordered};
use indent::*;
use parking_lot::*;
use schemars::JsonSchema;
use serde::*;
use stop_token::*;
use thiserror::Error as ThisError;
use tracing::*;
use veilid_tools::*;

cfg_if::cfg_if! {
    if #[cfg(all(target_arch = "wasm32", target_os = "unknown"))] {
        pub use wasm_bindgen::prelude::*;
        pub use tsify::*;
    }
}
