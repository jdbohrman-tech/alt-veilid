mod dht_record_descriptor;
mod dht_record_report;
mod schema;
mod value_data;
mod value_subkey_range_set;

use super::*;

pub use dht_record_descriptor::*;
pub use dht_record_report::*;
pub use schema::*;
pub use value_data::*;
pub use value_subkey_range_set::*;

/// Value subkey
#[cfg_attr(target_arch = "wasm32", declare)]
pub type ValueSubkey = u32;
/// Value sequence number
#[cfg_attr(target_arch = "wasm32", declare)]
pub type ValueSeqNum = u32;
