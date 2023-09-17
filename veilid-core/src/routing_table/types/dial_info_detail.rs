use super::*;

// Keep member order appropriate for sorting < preference
#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Serialize, Deserialize)]
pub struct DialInfoDetail {
    pub class: DialInfoClass,
    pub dial_info: DialInfo,
}

impl MatchesDialInfoFilter for DialInfoDetail {
    fn matches_filter(&self, filter: &DialInfoFilter) -> bool {
        self.dial_info.matches_filter(filter)
    }
}

pub type DialInfoDetailSort = dyn Fn(&DialInfoDetail, &DialInfoDetail) -> core::cmp::Ordering;

impl DialInfoDetail {
    pub fn ordered_sequencing_sort(a: &DialInfoDetail, b: &DialInfoDetail) -> core::cmp::Ordering {
        let c = DialInfo::ordered_sequencing_sort(&a.dial_info, &b.dial_info);
        if c != core::cmp::Ordering::Equal {
            return c;
        }
        a.class.cmp(&b.class)
    }
    pub const NO_SORT: std::option::Option<
        for<'r, 's> fn(&'r DialInfoDetail, &'s DialInfoDetail) -> std::cmp::Ordering,
    > = None::<fn(&DialInfoDetail, &DialInfoDetail) -> core::cmp::Ordering>;
}
