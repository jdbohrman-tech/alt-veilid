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

pub(crate) fn debug_seqs(seqs: &[ValueSeqNum]) -> String {
    let mut col = 0;
    let mut out = String::new();
    let mut left = seqs.len();
    for s in seqs {
        if col == 0 {
            out += "    ";
        }
        let sc = if *s == ValueSeqNum::MAX {
            "-".to_owned()
        } else {
            s.to_string()
        };
        out += &sc;
        out += ",";
        col += 1;
        left -= 1;
        if col == 32 && left != 0 {
            col = 0;
            out += "\n"
        }
    }
    out
}
