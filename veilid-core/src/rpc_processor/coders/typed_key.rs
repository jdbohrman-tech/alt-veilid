use super::*;

pub fn decode_typed_key(typed_key: &veilid_capnp::typed_key::Reader) -> Result<TypedKey, RPCError> {
    let key_reader = typed_key
        .get_key()
        .map_err(RPCError::map_invalid_format("invalid typed key"))?;
    let kind = typed_key.get_kind();

    Ok(TypedKey::new(
        CryptoKind::from(kind.to_be_bytes()),
        decode_key256(&key_reader),
    ))
}

pub fn encode_typed_key(typed_key: &TypedKey, builder: &mut veilid_capnp::typed_key::Builder) {
    builder.set_kind(u32::from_be_bytes(typed_key.kind.0));
    let mut key_builder = builder.init_key();
    encode_key256(&typed_key.key, &mut key_builder);
}
