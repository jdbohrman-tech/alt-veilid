use super::*;

#[cfg(target_arch = "wasm32")]
mod wasm;
#[cfg(target_arch = "wasm32")]
pub use wasm::*;
#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(not(target_arch = "wasm32"))]
pub use native::*;

pub static KNOWN_PROTECTED_STORE_KEYS: [&str; 2] = ["device_encryption_key", "_test_key"];
