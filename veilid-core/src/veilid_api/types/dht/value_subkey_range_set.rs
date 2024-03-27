use super::*;
use core::ops::{Deref, DerefMut};
use range_set_blaze::*;

#[derive(
    Clone, Default, Hash, PartialOrd, PartialEq, Eq, Ord, Serialize, Deserialize, JsonSchema,
)]
#[cfg_attr(
    target_arch = "wasm32",
    derive(Tsify),
    tsify(from_wasm_abi, into_wasm_abi)
)]
#[serde(transparent)]
pub struct ValueSubkeyRangeSet {
    #[serde(with = "serialize_range_set_blaze")]
    #[schemars(with = "Vec<(u32,u32)>")]
    data: RangeSetBlaze<ValueSubkey>,
}

impl ValueSubkeyRangeSet {
    pub fn new() -> Self {
        Self {
            data: Default::default(),
        }
    }
    pub fn full() -> Self {
        let mut data = RangeSetBlaze::new();
        data.ranges_insert(u32::MIN..=u32::MAX);
        Self { data }
    }
    pub fn new_with_data(data: RangeSetBlaze<ValueSubkey>) -> Self {
        Self { data }
    }
    pub fn single(value: ValueSubkey) -> Self {
        let mut data = RangeSetBlaze::new();
        data.insert(value);
        Self { data }
    }
    pub fn single_range(low: ValueSubkey, high: ValueSubkey) -> Self {
        let mut data = RangeSetBlaze::new();
        data.ranges_insert(low..=high);
        Self { data }
    }

    pub fn intersect(&self, other: &ValueSubkeyRangeSet) -> ValueSubkeyRangeSet {
        Self::new_with_data(&self.data & &other.data)
    }
    pub fn difference(&self, other: &ValueSubkeyRangeSet) -> ValueSubkeyRangeSet {
        Self::new_with_data(&self.data - &other.data)
    }
    pub fn union(&self, other: &ValueSubkeyRangeSet) -> ValueSubkeyRangeSet {
        Self::new_with_data(&self.data | &other.data)
    }

    pub fn data(&self) -> &RangeSetBlaze<ValueSubkey> {
        &self.data
    }
    pub fn into_data(self) -> RangeSetBlaze<ValueSubkey> {
        self.data
    }

    pub fn nth_subkey(&self, idx: usize) -> Option<ValueSubkey> {
        let mut idxleft = idx;
        for range in self.data.ranges() {
            let range_len = (*range.end() - *range.start() + 1) as usize;
            if idxleft < range_len {
                return Some(*range.start() + idxleft as u32);
            }
            idxleft -= range_len;
        }
        None
    }

    pub fn idx_of_subkey(&self, subkey: ValueSubkey) -> Option<usize> {
        let mut idx = 0usize;
        for range in self.data.ranges() {
            if range.contains(&subkey) {
                idx += (subkey - *range.start()) as usize;
                return Some(idx);
            } else {
                idx += (*range.end() - *range.start() + 1) as usize;
            }
        }
        None
    }
}

// impl TryFrom<Box<[Box<[ValueSubkey]>]>> for ValueSubkeyRangeSet {
//     type Error = VeilidAPIError;

//     fn try_from(value: Box<[Box<[ValueSubkey]>]>) -> Result<Self, Self::Error> {
//         let mut data = RangeSetBlaze::<ValueSubkey>::new();

//         let mut last = None;

//         for r in value.iter() {
//             if r.len() != 2 {
//                 apibail_generic!("not a pair");
//             }
//             let start = r[0];
//             let end = r[1];
//             if let Some(last) = last {
//                 if start >= last {
//                     apibail_generic!("pair out of order");
//                 }
//             }
//             if start > end {
//                 apibail_generic!("invalid pair");
//             }
//             last = Some(end);
//             data.ranges_insert(start..=end);
//         }

//         Ok(Self::new_with_data(data))
//     }
// }

// impl From<ValueSubkeyRangeSet> for Box<[Box<[ValueSubkey]>]> {
//     fn from(value: ValueSubkeyRangeSet) -> Self {
//         value
//             .ranges()
//             .map(|r| Box::new([*r.start(), *r.end()]) as Box<[ValueSubkey]>)
//             .collect()
//     }
// }

impl FromStr for ValueSubkeyRangeSet {
    type Err = VeilidAPIError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut data = RangeSetBlaze::<ValueSubkey>::new();

        for r in value.split(',') {
            let r = r.trim();
            let Some((ss, es)) = r.split_once("..=") else {
                return Err(VeilidAPIError::parse_error(
                    "can not parse ValueSubkeyRangeSet",
                    r,
                ));
            };
            let sn = ValueSubkey::from_str(ss)
                .map_err(|e| VeilidAPIError::parse_error("could not parse ValueSubkey", e))?;
            let en = ValueSubkey::from_str(es)
                .map_err(|e| VeilidAPIError::parse_error("could not parse ValueSubkey", e))?;
            data.ranges_insert(sn..=en);
        }

        Ok(ValueSubkeyRangeSet { data })
    }
}

impl Deref for ValueSubkeyRangeSet {
    type Target = RangeSetBlaze<ValueSubkey>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for ValueSubkeyRangeSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl fmt::Debug for ValueSubkeyRangeSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.data)
    }
}

impl fmt::Display for ValueSubkeyRangeSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}
