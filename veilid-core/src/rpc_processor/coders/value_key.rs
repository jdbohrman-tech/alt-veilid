use super::*;

pub fn encode_value_key(
    value_key: &ValueKey,
    builder: &mut veilid_capnp::value_key::Builder,
) -> Result<(), RPCError> {
    let mut pk_builder = builder.reborrow().init_public_key();
    encode_dht_key(&value_key.key, &mut pk_builder)?;
    if let Some(subkey) = &value_key.subkey {
        builder.set_subkey(subkey);
    }
    Ok(())
}

pub fn decode_value_key(reader: &veilid_capnp::value_key::Reader) -> Result<ValueKey, RPCError> {
    let pk_reader = reader.get_public_key().map_err(RPCError::protocol)?;
    let key = decode_dht_key(&pk_reader);
    let subkey = if !reader.has_subkey() {
        None
    } else {
        let subkey = reader.get_subkey().map_err(RPCError::protocol)?;
        Some(subkey.to_owned())
    };
    Ok(ValueKey { key, subkey })
}
