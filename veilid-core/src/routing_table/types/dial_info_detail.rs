use super::*;

// Keep member order appropriate for sorting < preference
#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize)]
pub struct DialInfoDetail {
    pub class: DialInfoClass,
    pub dial_info: DialInfo,
}

impl fmt::Display for DialInfoDetail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}:{}", self.class, self.dial_info)
    }
}

impl MatchesDialInfoFilter for DialInfoDetail {
    fn matches_filter(&self, filter: &DialInfoFilter) -> bool {
        self.dial_info.matches_filter(filter)
    }
}

pub type DialInfoDetailSort<'a> =
    Box<dyn Fn(&DialInfoDetail, &DialInfoDetail) -> core::cmp::Ordering + 'a>;

impl DialInfoDetail {
    pub fn ordered_sequencing_sort(a: &DialInfoDetail, b: &DialInfoDetail) -> core::cmp::Ordering {
        let c = DialInfo::ordered_sequencing_sort(&a.dial_info, &b.dial_info);
        if c != core::cmp::Ordering::Equal {
            return c;
        }
        a.class.cmp(&b.class)
    }
    pub const NO_SORT: std::option::Option<&DialInfoDetailSort<'static>> = None::<_>;
}
