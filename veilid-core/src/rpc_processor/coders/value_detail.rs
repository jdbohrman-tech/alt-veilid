use super::*;
use crate::storage_manager::ValueDetail;

pub fn encode_value_detail(
    value_detail: &ValueDetail,
    builder: &mut veilid_capnp::value_detail::Builder,
) -> Result<(), RPCError> {
    let mut svdb = builder.reborrow().init_signed_value_data();

    Ok(())
}

pub fn decode_value_detail(
    reader: &veilid_capnp::value_detail::Reader,
) -> Result<ValueDetail, RPCError> {
    Ok(ValueDetail {
        signed_value_data,
        descriptor,
    })
}
