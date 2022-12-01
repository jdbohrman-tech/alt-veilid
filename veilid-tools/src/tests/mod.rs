#[cfg(all(target_os = "android", feature = "veilid_tools_android_tests"))]
mod android;
pub mod common;
#[cfg(all(target_os = "ios", feature = "veilid_tools_ios_tests"))]
mod ios;
#[cfg(not(target_arch = "wasm32"))]
mod native;

#[allow(unused_imports)]
use super::*;

pub use common::*;
