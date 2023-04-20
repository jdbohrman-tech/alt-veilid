use super::*;
use crate::storage_manager::SignedValueDescriptor;

pub fn encode_signed_value_descriptor(
    signed_value_descriptor: &SignedValueDescriptor,
    builder: &mut veilid_capnp::signed_value_descriptor::Builder,
) -> Result<(), RPCError> {
    let mut ob = builder.reborrow().init_owner();
    encode_key256(signed_value_descriptor.owner(), &mut ob);
    builder.set_schema_data(signed_value_descriptor.schema_data());
    let mut sb = builder.reborrow().init_signature();
    encode_signature512(signed_value_descriptor.signature(), &mut sb);
    Ok(())
}

pub fn decode_signed_value_descriptor(
    reader: &veilid_capnp::signed_value_descriptor::Reader,
) -> Result<SignedValueDescriptor, RPCError> {
    let or = reader.get_owner().map_err(RPCError::protocol)?;
    let owner = decode_key256(&or);
    let schema_data = reader
        .get_schema_data()
        .map_err(RPCError::protocol)?
        .to_vec();
    let sr = reader.get_signature().map_err(RPCError::protocol)?;
    let signature = decode_signature512(&sr);
    Ok(SignedValueDescriptor::new(owner, schema_data, signature))
}
