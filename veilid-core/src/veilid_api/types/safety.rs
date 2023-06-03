use super::*;

// Ordering here matters, >= is used to check strength of sequencing requirement
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
    JsonSchema,
)]
#[archive_attr(repr(u8), derive(CheckBytes))]
pub enum Sequencing {
    NoPreference,
    PreferOrdered,
    EnsureOrdered,
}

impl Default for Sequencing {
    fn default() -> Self {
        Self::NoPreference
    }
}

// Ordering here matters, >= is used to check strength of stability requirement
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
    JsonSchema,
)]
#[archive_attr(repr(u8), derive(CheckBytes))]
pub enum Stability {
    LowLatency,
    Reliable,
}

impl Default for Stability {
    fn default() -> Self {
        Self::LowLatency
    }
}

/// The choice of safety route to include in compiled routes
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
    JsonSchema,
)]
#[archive_attr(repr(u8), derive(CheckBytes))]
pub enum SafetySelection {
    /// Don't use a safety route, only specify the sequencing preference
    Unsafe(Sequencing),
    /// Use a safety route and parameters specified by a SafetySpec
    Safe(SafetySpec),
}

impl SafetySelection {
    pub fn get_sequencing(&self) -> Sequencing {
        match self {
            SafetySelection::Unsafe(seq) => *seq,
            SafetySelection::Safe(ss) => ss.sequencing,
        }
    }
}

impl Default for SafetySelection {
    fn default() -> Self {
        Self::Unsafe(Sequencing::NoPreference)
    }
}

/// Options for safety routes (sender privacy)
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
    JsonSchema,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct SafetySpec {
    /// preferred safety route set id if it still exists
    #[schemars(with = "Option<String>")]
    pub preferred_route: Option<RouteId>,
    /// must be greater than 0
    pub hop_count: usize,
    /// prefer reliability over speed
    pub stability: Stability,
    /// prefer connection-oriented sequenced protocols
    pub sequencing: Sequencing,
}
