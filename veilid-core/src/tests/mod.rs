#[cfg(all(target_os = "android", feature = "veilid_core_android_tests"))]
mod android;
pub mod common;
#[cfg(all(target_os = "ios", feature = "veilid_core_ios_tests"))]
mod ios;
#[cfg(not(target_arch = "wasm32"))]
mod native;

#[allow(unused_imports)]
use super::*;

pub use common::*;
pub use crypto::tests::*;
pub use network_manager::tests::*;
pub use veilid_api::tests::*;
