mod dht_record_descriptor;
mod schema;
mod value_data;

use super::*;

pub use dht_record_descriptor::*;
pub use schema::*;
pub use value_data::*;

/// Value subkey
pub type ValueSubkey = u32;
/// Value subkey range
pub type ValueSubkeyRange = (u32, u32);
/// Value sequence number
pub type ValueSeqNum = u32;
