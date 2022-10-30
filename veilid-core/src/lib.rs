#![deny(clippy::all)]
#![deny(unused_must_use)]

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
mod callback_state_machine;
mod core_context;
mod crypto;
mod intf;
mod network_manager;
mod receipt_manager;
mod routing_table;
mod rpc_processor;
mod veilid_api;
#[macro_use]
mod veilid_config;
mod veilid_layer_filter;
mod veilid_rng;

#[macro_use]
pub mod xx;

pub use self::api_tracing_layer::ApiTracingLayer;
pub use self::attachment_manager::AttachmentState;
pub use self::core_context::{api_startup, api_startup_json, UpdateCallback};
pub use self::veilid_api::*;
pub use self::veilid_config::*;
pub use self::veilid_layer_filter::*;

pub mod veilid_capnp {
    include!(concat!(env!("OUT_DIR"), "/proto/veilid_capnp.rs"));
}

pub mod tests;

pub fn veilid_version_string() -> String {
    env!("CARGO_PKG_VERSION").to_owned()
}
pub fn veilid_version() -> (u32, u32, u32) {
    (
        u32::from_str(env!("CARGO_PKG_VERSION_MAJOR")).unwrap(),
        u32::from_str(env!("CARGO_PKG_VERSION_MINOR")).unwrap(),
        u32::from_str(env!("CARGO_PKG_VERSION_PATCH")).unwrap(),
    )
}

#[cfg(target_os = "android")]
pub use intf::utils::android::{veilid_core_setup_android, veilid_core_setup_android_no_log};

pub static DEFAULT_LOG_IGNORE_LIST: [&str; 19] = [
    "mio",
    "h2",
    "hyper",
    "tower",
    "tonic",
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
];
