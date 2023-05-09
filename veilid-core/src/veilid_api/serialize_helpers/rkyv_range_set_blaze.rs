use super::*;

use range_set_blaze::*;

pub struct RkyvRangeSetBlaze;

impl<T> rkyv::with::ArchiveWith<RangeSetBlaze<T>> for RkyvRangeSetBlaze
where
    T: rkyv::Archive + Integer,
{
    type Archived = rkyv::Archived<Vec<T>>;
    type Resolver = rkyv::Resolver<Vec<T>>;

    #[inline]
    unsafe fn resolve_with(
        field: &RangeSetBlaze<T>,
        pos: usize,
        resolver: Self::Resolver,
        out: *mut Self::Archived,
    ) {
        let mut r = Vec::<T>::with_capacity(field.ranges_len() * 2);
        for range in field.ranges() {
            r.push(*range.start());
            r.push(*range.end());
        }
        r.resolve(pos, resolver, out);
    }
}

impl<T, S> rkyv::with::SerializeWith<RangeSetBlaze<T>, S> for RkyvRangeSetBlaze
where
    S: rkyv::Fallible + ?Sized,
    Vec<T>: rkyv::Serialize<S>,
    T: rkyv::Archive + Integer,
{
    fn serialize_with(
        field: &RangeSetBlaze<T>,
        serializer: &mut S,
    ) -> Result<Self::Resolver, S::Error> {
        let mut r = Vec::<T>::with_capacity(field.ranges_len() * 2);
        for range in field.ranges() {
            r.push(*range.start());
            r.push(*range.end());
        }
        r.serialize(serializer)
    }
}

impl<T, D> rkyv::with::DeserializeWith<rkyv::Archived<Vec<T>>, RangeSetBlaze<T>, D>
    for RkyvRangeSetBlaze
where
    D: rkyv::Fallible + ?Sized,
    T: rkyv::Archive + Integer,
    rkyv::Archived<T>: rkyv::Deserialize<T, D>,
    // D::Error: From<String>, // xxx this doesn't work
{
    fn deserialize_with(
        field: &rkyv::Archived<Vec<T>>,
        deserializer: &mut D,
    ) -> Result<RangeSetBlaze<T>, D::Error> {
        let mut out = RangeSetBlaze::<T>::new();
        // if field.len() % 2 == 1 {
        //     return Err("invalid range set length".to_owned().into());
        // }
        let f = field.as_slice();
        for i in 0..field.len() / 2 {
            let l: T = f[i * 2].deserialize(deserializer)?;
            let u: T = f[i * 2 + 1].deserialize(deserializer)?;
            out.ranges_insert(l..=u);
        }
        Ok(out)
    }
}
