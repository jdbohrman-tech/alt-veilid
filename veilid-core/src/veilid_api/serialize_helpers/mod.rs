mod rkyv_enum_set;
mod rkyv_range_set_blaze;
pub mod serialize_arc;
pub mod serialize_range_set_blaze;
mod serialize_json;

use super::*;
use core::fmt::Debug;

pub use rkyv_enum_set::*;
pub use rkyv_range_set_blaze::*;
pub use serialize_json::*;

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
        for<'t> CheckBytes<rkyv::validation::validators::DefaultValidator<'t>>,
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
