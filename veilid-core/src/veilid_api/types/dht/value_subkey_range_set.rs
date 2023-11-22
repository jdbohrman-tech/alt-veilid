use super::*;
use core::ops::{Deref, DerefMut};
use range_set_blaze::*;

#[derive(Clone, Default, PartialOrd, PartialEq, Eq, Ord, Serialize, Deserialize, JsonSchema)]
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

    pub fn interset(&self, other: &ValueSubkeyRangeSet) -> ValueSubkeyRangeSet {
        Self::new_with_data(self.data & other.data)
    }
    pub fn difference(&self, other: &ValueSubkeyRangeSet) -> ValueSubkeyRangeSet {
        Self::new_with_data(self.data - other.data)
    }
    pub fn union(&self, other: &ValueSubkeyRangeSet) -> ValueSubkeyRangeSet {
        Self::new_with_data(self.data | other.data)
    }

    pub fn data(&self) -> RangeSetBlaze<ValueSubkey> {
        self.data().clone()
    }
    pub fn into_data(self) -> RangeSetBlaze<ValueSubkey> {
        self.data()
    }
}

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
