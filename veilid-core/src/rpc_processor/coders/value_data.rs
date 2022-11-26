use super::*;

pub fn encode_value_data(
    value_data: &ValueData,
    builder: &mut veilid_capnp::value_data::Builder,
) -> Result<(), RPCError> {
    builder.set_data(&value_data.data);
    builder.set_seq(value_data.seq);
    Ok(())
}

pub fn decode_value_data(reader: &veilid_capnp::value_data::Reader) -> Result<ValueData, RPCError> {
    let data = reader.get_data().map_err(RPCError::protocol)?.to_vec();
    let seq = reader.get_seq();
    Ok(ValueData { data, seq })
}
