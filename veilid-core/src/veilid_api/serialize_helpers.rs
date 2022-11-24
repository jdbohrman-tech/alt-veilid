use super::*;
pub use bytecheck::CheckBytes;
use core::fmt::Debug;
use rkyv::Archive as RkyvArchive;
use rkyv::Deserialize as RkyvDeserialize;
use rkyv::Serialize as RkyvSerialize;

// Don't trace these functions as they are used in the transfer of API logs, which will recurse!

// #[instrument(level = "trace", ret, err)]
pub fn deserialize_json<'a, T: de::Deserialize<'a> + Debug>(
    arg: &'a str,
) -> Result<T, VeilidAPIError> {
    serde_json::from_str(arg).map_err(|e| VeilidAPIError::ParseError {
        message: e.to_string(),
        value: format!(
            "deserialize_json:\n---\n{}\n---\n to type {}",
            arg,
            std::any::type_name::<T>()
        ),
    })
}

// #[instrument(level = "trace", ret, err)]
pub fn deserialize_opt_json<T: de::DeserializeOwned + Debug>(
    arg: Option<String>,
) -> Result<T, VeilidAPIError> {
    let arg = arg.as_ref().ok_or_else(|| VeilidAPIError::ParseError {
        message: "invalid null string".to_owned(),
        value: format!(
            "deserialize_json_opt: null to type {}",
            std::any::type_name::<T>()
        ),
    })?;
    deserialize_json(arg)
}

// #[instrument(level = "trace", ret)]
pub fn serialize_json<T: Serialize + Debug>(val: T) -> String {
    match serde_json::to_string(&val) {
        Ok(v) => v,
        Err(e) => {
            panic!("failed to serialize json value: {}\nval={:?}", e, val);
        }
    }
}

pub mod json_as_base64 {
    use data_encoding::BASE64URL_NOPAD;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer>(v: &Vec<u8>, s: S) -> Result<S::Ok, S::Error> {
        let base64 = BASE64URL_NOPAD.encode(v);
        String::serialize(&base64, s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<u8>, D::Error> {
        let base64 = String::deserialize(d)?;
        BASE64URL_NOPAD
            .decode(base64.as_bytes())
            .map_err(|e| serde::de::Error::custom(e))
    }
}

pub mod json_as_string {
    use std::fmt::Display;
    use std::str::FromStr;

    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: Display,
        S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: FromStr,
        T::Err: Display,
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(de::Error::custom)
    }
}

pub mod opt_json_as_string {
    use std::fmt::Display;
    use std::str::FromStr;

    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<T, S>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: Display,
        S: Serializer,
    {
        match value {
            Some(v) => serializer.collect_str(v),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        T: FromStr,
        T::Err: Display,
        D: Deserializer<'de>,
    {
        match Option::<String>::deserialize(deserializer)? {
            None => Ok(None),
            Some(v) => Ok(Some(v.parse::<T>().map_err(de::Error::custom)?)),
        }
    }
}

pub mod arc_serialize {
    use alloc::sync::Arc;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<T: Serialize, S: Serializer>(v: &Arc<T>, s: S) -> Result<S::Ok, S::Error> {
        T::serialize(v.as_ref(), s)
    }

    pub fn deserialize<'de, T: Deserialize<'de>, D: Deserializer<'de>>(
        d: D,
    ) -> Result<Arc<T>, D::Error> {
        Ok(Arc::new(T::deserialize(d)?))
    }
}

pub fn to_rkyv<T>(v: &T) -> EyreResult<Vec<u8>>
where
    T: RkyvSerialize<rkyv::ser::serializers::AllocSerializer<1024>>,
{
    Ok(rkyv::to_bytes::<T, 1024>(v)
        .wrap_err("failed to freeze object")?
        .to_vec())
}

pub fn from_rkyv<T>(v: Vec<u8>) -> EyreResult<T>
where
    T: RkyvArchive,
    <T as RkyvArchive>::Archived:
        for<'t> bytecheck::CheckBytes<rkyv::validation::validators::DefaultValidator<'t>>,
    <T as RkyvArchive>::Archived:
        rkyv::Deserialize<T, rkyv::de::deserializers::SharedDeserializeMap>,
{
    match rkyv::from_bytes::<T>(&v) {
        Ok(v) => Ok(v),
        Err(e) => {
            bail!("failed to deserialize frozen object: {}", e);
        }
    }
}

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
