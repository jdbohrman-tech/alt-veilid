use super::*;

pub fn decode_typed_public_key(
    typed_key: &veilid_capnp::typed_key::Reader,
) -> Result<TypedPublicKey, RPCError> {
    let key_reader = typed_key
        .get_key()
        .map_err(RPCError::map_invalid_format("invalid typed key"))?;
    let kind = typed_key.get_kind();

    Ok(TypedPublicKey::new(
        CryptoKind::from(kind.to_be_bytes()),
        decode_key256(&key_reader),
    ))
}

pub fn encode_typed_public_key(
    typed_key: &TypedPublicKey,
    builder: &mut veilid_capnp::typed_key::Builder,
) {
    builder.set_kind(u32::from_be_bytes(typed_key.kind.0));
    let mut key_builder = builder.reborrow().init_key();
    encode_key256(&typed_key.value, &mut key_builder);
}

pub fn decode_typed_node_id(
    typed_key: &veilid_capnp::typed_key::Reader,
) -> Result<TypedNodeId, RPCError> {
    let key_reader = typed_key
        .get_key()
        .map_err(RPCError::map_invalid_format("invalid typed key"))?;
    let kind = typed_key.get_kind();

    Ok(TypedNodeId::new(
        CryptoKind::from(kind.to_be_bytes()),
        NodeId::new(decode_key256(&key_reader).bytes),
    ))
}

pub fn encode_typed_node_id(
    typed_key: &TypedNodeId,
    builder: &mut veilid_capnp::typed_key::Builder,
) {
    builder.set_kind(u32::from_be_bytes(typed_key.kind.0));
    let mut key_builder = builder.reborrow().init_key();
    encode_key256(&PublicKey::new(typed_key.value.bytes), &mut key_builder);
}

pub fn decode_typed_record_key(
    typed_key: &veilid_capnp::typed_key::Reader,
) -> Result<TypedRecordKey, RPCError> {
    let key_reader = typed_key
        .get_key()
        .map_err(RPCError::map_invalid_format("invalid typed key"))?;
    let kind = typed_key.get_kind();

    Ok(TypedRecordKey::new(
        CryptoKind::from(kind.to_be_bytes()),
        RecordKey::new(decode_key256(&key_reader).bytes),
    ))
}

pub fn encode_typed_record_key(
    typed_key: &TypedRecordKey,
    builder: &mut veilid_capnp::typed_key::Builder,
) {
    builder.set_kind(u32::from_be_bytes(typed_key.kind.0));
    let mut key_builder = builder.reborrow().init_key();
    encode_key256(&PublicKey::new(typed_key.value.bytes), &mut key_builder);
}
