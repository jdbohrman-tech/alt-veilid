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

    let mut sig_builder = builder.reborrow().init_signature();
    let Some(signature) = &signed_direct_node_info.signature else {
        return Err(RPCError::internal("Should not encode SignedDirectNodeInfo without signature!"));
    };
    encode_signature(signature, &mut sig_builder);

    Ok(())
}

pub fn decode_signed_direct_node_info(
    reader: &veilid_capnp::signed_direct_node_info::Reader,
    node_id: &DHTKey,
) -> Result<SignedDirectNodeInfo, RPCError> {
    let ni_reader = reader
        .reborrow()
        .get_node_info()
        .map_err(RPCError::protocol)?;
    let node_info = decode_node_info(&ni_reader)?;

    let sig_reader = reader
        .reborrow()
        .get_signature()
        .map_err(RPCError::protocol)?;

    let timestamp = reader.reborrow().get_timestamp().into();

    let signature = decode_signature(&sig_reader);

    SignedDirectNodeInfo::new(NodeId::new(*node_id), node_info, timestamp, signature)
        .map_err(RPCError::protocol)
}
