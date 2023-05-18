mod table_db;
use super::*;

#[cfg(target_arch = "wasm32")]
mod wasm;
#[cfg(target_arch = "wasm32")]
pub use wasm::*;
#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(not(target_arch = "wasm32"))]
pub use native::*;

pub static KNOWN_TABLE_NAMES: [&'static str; 7] = [
    "crypto_caches",
    "RouteSpecStore",
    "routing_table",
    "local_records",
    "local_subkeys",
    "remote_records",
    "remote_subkeys",
];

pub static KNOWN_PROTECTED_STORE_KEYS: [&'static str; 4] =
    ["node_id", "node_id_secret", "_test_key", "RouteSpecStore"];
