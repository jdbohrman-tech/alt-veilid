use super::*;
use crate::storage_manager::*;

pub fn encode_signed_value_data(
    signed_value_data: &SignedValueData,
    builder: &mut veilid_capnp::signed_value_data::Builder,
) -> Result<(), RPCError> {
    builder.set_seq(signed_value_data.value_data().seq());
    builder.set_data(signed_value_data.value_data().data());
    let mut wb = builder.reborrow().init_writer();
    encode_key256(signed_value_data.value_data().writer(), &mut wb);
    let mut sb = builder.reborrow().init_signature();
    encode_signature512(signed_value_data.signature(), &mut sb);
    Ok(())
}

pub fn decode_signed_value_data(
    reader: &veilid_capnp::signed_value_data::Reader,
) -> Result<SignedValueData, RPCError> {
    let seq = reader.get_seq();
    let data = reader.get_data().map_err(RPCError::protocol)?.to_vec();
    let wr = reader.get_writer().map_err(RPCError::protocol)?;
    let writer = decode_key256(&wr);
    let sr = reader.get_signature().map_err(RPCError::protocol)?;
    let signature = decode_signature512(&sr);

    Ok(SignedValueData::new(
        ValueData::new_with_seq(seq, data, writer).map_err(RPCError::protocol)?,
        signature,
    ))
}
