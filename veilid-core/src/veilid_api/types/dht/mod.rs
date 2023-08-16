mod dht_record_descriptor;
mod schema;
mod value_data;
mod value_subkey_range_set;

use super::*;

pub use dht_record_descriptor::*;
pub use schema::*;
pub use value_data::*;
pub use value_subkey_range_set::*;

/// Value subkey
#[declare]
pub type ValueSubkey = u32;
/// Value sequence number
#[declare]
pub type ValueSeqNum = u32;
