#[cfg(all(target_os = "android", feature = "veilid_core_android_tests"))]
mod android;
pub mod common;
#[cfg(all(target_os = "ios", feature = "veilid_core_ios_tests"))]
mod ios;
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
mod native;

#[allow(unused_imports)]
use super::*;

pub use common::*;
pub use crypto::tests::*;
pub use network_manager::tests::*;
pub use routing_table::tests::*;
pub use table_store::tests::*;
pub use veilid_api::tests::*;
