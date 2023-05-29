use super::*;

// Keep member order appropriate for sorting < preference
#[derive(
    Debug,
    Clone,
    PartialEq,
    PartialOrd,
    Ord,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct DialInfoDetail {
    pub class: DialInfoClass,
    pub dial_info: DialInfo,
}

impl MatchesDialInfoFilter for DialInfoDetail {
    fn matches_filter(&self, filter: &DialInfoFilter) -> bool {
        self.dial_info.matches_filter(filter)
    }
}

impl DialInfoDetail {
    pub fn ordered_sequencing_sort(a: &DialInfoDetail, b: &DialInfoDetail) -> core::cmp::Ordering {
        if a.class < b.class {
            return core::cmp::Ordering::Less;
        }
        if a.class > b.class {
            return core::cmp::Ordering::Greater;
        }
        DialInfo::ordered_sequencing_sort(&a.dial_info, &b.dial_info)
    }
    pub const NO_SORT: std::option::Option<
        for<'r, 's> fn(&'r DialInfoDetail, &'s DialInfoDetail) -> std::cmp::Ordering,
    > = None::<fn(&DialInfoDetail, &DialInfoDetail) -> core::cmp::Ordering>;
}
