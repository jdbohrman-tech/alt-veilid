use super::*;
use core::ops::{Deref, DerefMut};
use range_set_blaze::*;

#[derive(
    Clone,
    Debug,
    Default,
    PartialOrd,
    PartialEq,
    Eq,
    Ord,
    Serialize,
    Deserialize,
    RkyvArchive,
    RkyvSerialize,
    RkyvDeserialize,
    JsonSchema,
)]
#[archive_attr(repr(C), derive(CheckBytes))]
#[serde(transparent)]
pub struct ValueSubkeyRangeSet {
    #[with(RkyvRangeSetBlaze)]
    #[serde(with = "serialize_range_set_blaze")]
    #[schemars(with = "Vec<(u32,u32)>")]
    pub data: RangeSetBlaze<ValueSubkey>,
}

impl ValueSubkeyRangeSet {
    pub fn new() -> Self {
        Self {
            data: Default::default(),
        }
    }
    pub fn single(value: ValueSubkey) -> Self {
        let mut data = RangeSetBlaze::new();
        data.insert(value);
        Self { data }
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
