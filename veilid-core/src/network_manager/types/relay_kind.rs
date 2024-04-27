use super::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RelayKind {
    Inbound = 0,
    Outbound = 1,
}

impl Default for RelayKind {
    fn default() -> Self {
        Self::Inbound
    }
}
