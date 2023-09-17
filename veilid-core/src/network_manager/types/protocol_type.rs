#![allow(non_snake_case)]
use super::*;

// Keep member order appropriate for sorting < preference
// Must match DialInfo order
#[allow(clippy::derived_hash_with_manual_eq)]
#[derive(Debug, PartialOrd, Ord, Hash, EnumSetType, Serialize, Deserialize)]
#[enumset(repr = "u8")]
pub enum ProtocolType {
    UDP = 0,
    TCP = 1,
    WS = 2,
    WSS = 3,
}

impl ProtocolType {
    pub fn is_ordered(&self) -> bool {
        matches!(
            self,
            ProtocolType::TCP | ProtocolType::WS | ProtocolType::WSS
        )
    }
    pub fn low_level_protocol_type(&self) -> LowLevelProtocolType {
        match self {
            ProtocolType::UDP => LowLevelProtocolType::UDP,
            ProtocolType::TCP | ProtocolType::WS | ProtocolType::WSS => LowLevelProtocolType::TCP,
        }
    }
    pub fn sort_order(&self, sequencing: Sequencing) -> usize {
        match self {
            ProtocolType::UDP => {
                if sequencing != Sequencing::NoPreference {
                    3
                } else {
                    0
                }
            }
            ProtocolType::TCP => {
                if sequencing != Sequencing::NoPreference {
                    0
                } else {
                    1
                }
            }
            ProtocolType::WS => {
                if sequencing != Sequencing::NoPreference {
                    1
                } else {
                    2
                }
            }
            ProtocolType::WSS => {
                if sequencing != Sequencing::NoPreference {
                    2
                } else {
                    3
                }
            }
        }
    }
    pub fn all_ordered_set() -> ProtocolTypeSet {
        ProtocolType::TCP | ProtocolType::WS | ProtocolType::WSS
    }

    pub fn ordered_sequencing_sort(a: Self, b: Self) -> core::cmp::Ordering {
        let ca = a.sort_order(Sequencing::EnsureOrdered);
        let cb = b.sort_order(Sequencing::EnsureOrdered);
        if ca < cb {
            return core::cmp::Ordering::Less;
        }
        if ca > cb {
            return core::cmp::Ordering::Greater;
        }
        core::cmp::Ordering::Equal
    }
}

impl fmt::Display for ProtocolType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProtocolType::UDP => write!(f, "UDP"),
            ProtocolType::TCP => write!(f, "TCP"),
            ProtocolType::WS => write!(f, "WS"),
            ProtocolType::WSS => write!(f, "WSS"),
        }
    }
}

impl FromStr for ProtocolType {
    type Err = VeilidAPIError;
    fn from_str(s: &str) -> VeilidAPIResult<ProtocolType> {
        match s.to_ascii_uppercase().as_str() {
            "UDP" => Ok(ProtocolType::UDP),
            "TCP" => Ok(ProtocolType::TCP),
            "WS" => Ok(ProtocolType::WS),
            "WSS" => Ok(ProtocolType::WSS),
            _ => Err(VeilidAPIError::parse_error(
                "ProtocolType::from_str failed",
                s,
            )),
        }
    }
}

pub type ProtocolTypeSet = EnumSet<ProtocolType>;
