use super::*;
use crate::storage_manager::ValueDetail;

pub fn encode_value_detail(
    value_detail: &ValueDetail,
    builder: &mut veilid_capnp::value_detail::Builder,
) -> Result<(), RPCError> {
    let mut svdb = builder.reborrow().init_signed_value_data();
    encode_signed_value_data(value_detail.signed_value_data(), &mut svdb)?;
    if let Some(descriptor) = value_detail.descriptor() {
        let mut db = builder.reborrow().init_descriptor();
        encode_signed_value_descriptor(descriptor, &mut db)?;
    }
    Ok(())
}

pub fn decode_value_detail(
    reader: &veilid_capnp::value_detail::Reader,
) -> Result<ValueDetail, RPCError> {
    let svdr = reader.get_signed_value_data().map_err(RPCError::protocol)?;
    let signed_value_data = decode_signed_value_data(&svdr)?;

    let descriptor = if reader.has_descriptor() {
        let dr = reader
            .reborrow()
            .get_descriptor()
            .map_err(RPCError::protocol)?;
        let descriptor = decode_signed_value_descriptor(&dr)?;
        Some(descriptor)
    } else {
        None
    };

    Ok(ValueDetail {
        signed_value_data,
        descriptor,
    })
}
