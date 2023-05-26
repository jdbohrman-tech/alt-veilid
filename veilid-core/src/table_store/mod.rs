use super::*;

mod table_db;
mod table_store;
pub use table_db::*;
pub use table_store::*;

pub mod tests;

#[cfg(target_arch = "wasm32")]
mod wasm;
#[cfg(target_arch = "wasm32")]
use wasm::*;
#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(not(target_arch = "wasm32"))]
use native::*;
