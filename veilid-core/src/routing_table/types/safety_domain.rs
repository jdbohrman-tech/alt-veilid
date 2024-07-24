use super::*;

#[allow(clippy::derived_hash_with_manual_eq)]
#[derive(Debug, PartialOrd, Ord, Hash, Serialize, Deserialize, EnumSetType)]
#[enumset(repr = "u8")]
pub enum SafetyDomain {
    Unsafe = 0,
    Safe = 1,
}
//pub type SafetyDomainSet = EnumSet<SafetyDomain>;

impl From<SafetySelection> for SafetyDomain {
    fn from(value: SafetySelection) -> Self {
        match value {
            SafetySelection::Unsafe(_) => SafetyDomain::Unsafe,
            SafetySelection::Safe(_) => SafetyDomain::Safe,
        }
    }
}

impl SafetyDomain {
    pub fn print(&self) -> String {
        if *self == SafetyDomain::Unsafe.into() {
            "*UNSAFE".to_string()
        } else {
            "*SAFE".to_string()
        }
    }
    // pub fn print_set(set: SafetyDomainSet) -> String {
    //     if *set == SafetyDomainSet::all() {
    //         "*ALL".to_string()
    //     } else if *set == SafetyDomain::Unsafe.into() {
    //         "*UNSAFE".to_string()
    //     } else if *set == SafetyDomain::Safe.into() {
    //         "*SAFE".to_string()
    //     } else {
    //         "*NONE".to_string()
    //     }
    // }
}
