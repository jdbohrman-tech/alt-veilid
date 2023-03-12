use super::*;

pub fn decode_typed_signature(
    typed_signature: &veilid_capnp::typed_signature::Reader,
) -> Result<TypedSignature, RPCError> {
    let sig_reader = typed_signature
        .get_signature()
        .map_err(RPCError::map_invalid_format("invalid typed signature"))?;
    let kind = typed_signature.get_kind();

    Ok(TypedSignature::new(
        CryptoKind::from(kind.to_be_bytes()),
        decode_signature512(&sig_reader),
    ))
}

pub fn encode_typed_signature(
    typed_signature: &TypedSignature,
    builder: &mut veilid_capnp::typed_signature::Builder,
) {
    builder.set_kind(u32::from_be_bytes(typed_signature.kind.0));
    let mut sig_builder = builder.reborrow().init_signature();
    encode_signature512(&typed_signature.value, &mut sig_builder);
}
