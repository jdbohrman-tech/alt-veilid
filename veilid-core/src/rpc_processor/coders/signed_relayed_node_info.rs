use crate::*;
use rpc_processor::*;

pub fn encode_signed_relayed_node_info(
    signed_relayed_node_info: &SignedRelayedNodeInfo,
    builder: &mut veilid_capnp::signed_relayed_node_info::Builder,
) -> Result<(), RPCError> {
    //
    let mut ni_builder = builder.reborrow().init_node_info();
    encode_node_info(&signed_relayed_node_info.node_info, &mut ni_builder)?;

    let mut rid_builder = builder.reborrow().init_relay_id();
    encode_dht_key(&signed_relayed_node_info.relay_id.key, &mut rid_builder)?;

    let mut ri_builder = builder.reborrow().init_relay_info();
    encode_signed_direct_node_info(&signed_relayed_node_info.relay_info, &mut ri_builder)?;

    builder
        .reborrow()
        .set_timestamp(signed_relayed_node_info.timestamp);

    let mut sig_builder = builder.reborrow().init_signature();
    encode_signature(&signed_relayed_node_info.signature, &mut sig_builder);

    Ok(())
}

pub fn decode_signed_relayed_node_info(
    reader: &veilid_capnp::signed_relayed_node_info::Reader,
    node_id: &DHTKey,
) -> Result<SignedRelayedNodeInfo, RPCError> {
    let ni_reader = reader
        .reborrow()
        .get_node_info()
        .map_err(RPCError::protocol)?;
    let node_info = decode_node_info(&ni_reader)?;

    let rid_reader = reader
        .reborrow()
        .get_relay_id()
        .map_err(RPCError::protocol)?;
    let relay_id = decode_dht_key(&rid_reader);

    let ri_reader = reader
        .reborrow()
        .get_relay_info()
        .map_err(RPCError::protocol)?;
    let relay_info = decode_signed_direct_node_info(&ri_reader, &relay_id)?;

    let sig_reader = reader
        .reborrow()
        .get_signature()
        .map_err(RPCError::protocol)?;
    let timestamp = reader.reborrow().get_timestamp();

    let signature = decode_signature(&sig_reader);

    SignedRelayedNodeInfo::new(
        NodeId::new(*node_id),
        node_info,
        NodeId::new(relay_id),
        relay_info,
        timestamp,
        signature,
    )
    .map_err(RPCError::protocol)
}
