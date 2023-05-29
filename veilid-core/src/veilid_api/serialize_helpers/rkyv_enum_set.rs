use super::*;

pub struct RkyvEnumSet;

impl<T> rkyv::with::ArchiveWith<EnumSet<T>> for RkyvEnumSet
where
    T: EnumSetType + EnumSetTypeWithRepr,
    <T as EnumSetTypeWithRepr>::Repr: rkyv::Archive,
{
    type Archived = rkyv::Archived<<T as EnumSetTypeWithRepr>::Repr>;
    type Resolver = rkyv::Resolver<<T as EnumSetTypeWithRepr>::Repr>;

    #[inline]
    unsafe fn resolve_with(
        field: &EnumSet<T>,
        pos: usize,
        resolver: Self::Resolver,
        out: *mut Self::Archived,
    ) {
        let r = field.as_repr();
        r.resolve(pos, resolver, out);
    }
}

impl<T, S> rkyv::with::SerializeWith<EnumSet<T>, S> for RkyvEnumSet
where
    S: rkyv::Fallible + ?Sized,
    T: EnumSetType + EnumSetTypeWithRepr,
    <T as EnumSetTypeWithRepr>::Repr: rkyv::Serialize<S>,
{
    fn serialize_with(field: &EnumSet<T>, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        let r = field.as_repr();
        r.serialize(serializer)
    }
}

impl<T, D>
    rkyv::with::DeserializeWith<rkyv::Archived<<T as EnumSetTypeWithRepr>::Repr>, EnumSet<T>, D>
    for RkyvEnumSet
where
    D: rkyv::Fallible + ?Sized,
    T: EnumSetType + EnumSetTypeWithRepr,
    <T as EnumSetTypeWithRepr>::Repr: rkyv::Archive,
    rkyv::Archived<<T as EnumSetTypeWithRepr>::Repr>:
        rkyv::Deserialize<<T as EnumSetTypeWithRepr>::Repr, D>,
{
    fn deserialize_with(
        field: &rkyv::Archived<<T as EnumSetTypeWithRepr>::Repr>,
        deserializer: &mut D,
    ) -> Result<EnumSet<T>, D::Error> {
        Ok(EnumSet::<T>::from_repr(field.deserialize(deserializer)?))
    }
}
