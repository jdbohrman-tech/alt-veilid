use super::*;

pub fn encode_signed_direct_node_info(
    signed_direct_node_info: &SignedDirectNodeInfo,
    builder: &mut veilid_capnp::signed_direct_node_info::Builder,
) -> Result<(), RPCError> {
    //
    let mut ni_builder = builder.reborrow().init_node_info();
    encode_node_info(&signed_direct_node_info.node_info, &mut ni_builder)?;

    builder
        .reborrow()
        .set_timestamp(signed_direct_node_info.timestamp.into());

    let mut sigs_builder = builder.reborrow().init_signatures(
        signed_direct_node_info
            .signatures
            .len()
            .try_into()
            .map_err(RPCError::map_invalid_format("out of bound error"))?,
    );
    for (i, typed_signature) in signed_direct_node_info.signatures.iter().enumerate() {
        encode_typed_signature(
            typed_signature,
            &mut sigs_builder.reborrow().get(
                i.try_into()
                    .map_err(RPCError::map_invalid_format("out of bound error"))?,
            ),
        );
    }

    Ok(())
}

pub fn decode_signed_direct_node_info(
    reader: &veilid_capnp::signed_direct_node_info::Reader,
    crypto: Crypto,
    node_ids: &mut TypedKeySet,
) -> Result<SignedDirectNodeInfo, RPCError> {
    let ni_reader = reader
        .reborrow()
        .get_node_info()
        .map_err(RPCError::protocol)?;
    let node_info = decode_node_info(&ni_reader)?;

    let timestamp = reader.reborrow().get_timestamp().into();

    let sigs_reader = reader
        .reborrow()
        .get_signatures()
        .map_err(RPCError::protocol)?;

    let sig_count = sigs_reader.len() as usize;
    if sig_count > MAX_CRYPTO_KINDS {
        return Err(RPCError::protocol("too many signatures"));
    }

    let mut typed_signatures = Vec::with_capacity(sig_count);
    for sig_reader in sigs_reader {
        let typed_signature = decode_typed_signature(&sig_reader)?;
        typed_signatures.push(typed_signature);
    }

    SignedDirectNodeInfo::new(crypto, node_ids, node_info, timestamp, typed_signatures)
        .map_err(RPCError::protocol)
}
