#![allow(non_snake_case)]

use super::*;

// Keep member order appropriate for sorting < preference
// Must match DialInfo order
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum LowLevelProtocolType {
    UDP = 0,
    TCP = 1,
}
