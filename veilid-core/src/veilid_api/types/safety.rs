use super::*;

// Ordering here matters, >= is used to check strength of sequencing requirement
#[derive(
    Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, JsonSchema,
)]
#[cfg_attr(
    all(target_arch = "wasm32", target_os = "unknown"),
    derive(Tsify),
    tsify(from_wasm_abi, into_wasm_abi, namespace)
)]
#[must_use]
pub enum Sequencing {
    NoPreference = 0,
    PreferOrdered = 1,
    EnsureOrdered = 2,
}

impl Default for Sequencing {
    fn default() -> Self {
        // This is the default for veilid-core, and should not be the default used in any API-level code
        Self::NoPreference
    }
}

// Ordering here matters, >= is used to check strength of stability requirement
#[derive(
    Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, JsonSchema,
)]
#[cfg_attr(
    all(target_arch = "wasm32", target_os = "unknown"),
    derive(Tsify),
    tsify(from_wasm_abi, into_wasm_abi, namespace)
)]
#[must_use]
pub enum Stability {
    LowLatency = 0,
    Reliable = 1,
}

impl Default for Stability {
    fn default() -> Self {
        // This is the default for veilid-core, and should not be the default used in any API-level code
        Self::LowLatency
    }
}

/// The choice of safety route to include in compiled routes.
#[derive(
    Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, JsonSchema,
)]
#[cfg_attr(
    all(target_arch = "wasm32", target_os = "unknown"),
    derive(Tsify),
    tsify(from_wasm_abi, into_wasm_abi, namespace)
)]
#[must_use]
pub enum SafetySelection {
    /// Don't use a safety route, only specify the sequencing preference.
    Unsafe(Sequencing),
    /// Use a safety route and parameters specified by a SafetySpec.
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

/// Options for safety routes (sender privacy).
#[derive(
    Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, JsonSchema,
)]
#[cfg_attr(all(target_arch = "wasm32", target_os = "unknown"), derive(Tsify))]
#[must_use]
pub struct SafetySpec {
    /// Preferred safety route set id if it still exists.
    #[schemars(with = "Option<String>")]
    #[cfg_attr(
        all(target_arch = "wasm32", target_os = "unknown"),
        tsify(optional, type = "string")
    )]
    pub preferred_route: Option<RouteId>,
    /// Must be greater than 0.
    pub hop_count: usize,
    /// Prefer reliability over speed.
    pub stability: Stability,
    /// Prefer connection-oriented sequenced protocols.
    pub sequencing: Sequencing,
}
