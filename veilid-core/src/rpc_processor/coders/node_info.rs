use super::*;

pub fn encode_node_info(
    node_info: &NodeInfo,
    builder: &mut veilid_capnp::node_info::Builder,
) -> Result<(), RPCError> {
    builder.set_network_class(encode_network_class(node_info.network_class));

    let mut ps_builder = builder.reborrow().init_outbound_protocols();
    encode_protocol_type_set(&node_info.outbound_protocols, &mut ps_builder)?;

    let mut ats_builder = builder.reborrow().init_address_types();
    encode_address_type_set(&node_info.address_types, &mut ats_builder)?;

    builder.set_min_version(node_info.min_version);
    builder.set_max_version(node_info.max_version);

    let mut didl_builder = builder.reborrow().init_dial_info_detail_list(
        node_info
            .dial_info_detail_list
            .len()
            .try_into()
            .map_err(RPCError::map_protocol(
                "too many dial info details in node info",
            ))?,
    );

    for idx in 0..node_info.dial_info_detail_list.len() {
        let mut did_builder = didl_builder.reborrow().get(idx as u32);
        encode_dial_info_detail(&node_info.dial_info_detail_list[idx], &mut did_builder)?;
    }

    Ok(())
}

pub fn decode_node_info(reader: &veilid_capnp::node_info::Reader) -> Result<NodeInfo, RPCError> {
    let network_class = decode_network_class(
        reader
            .reborrow()
            .get_network_class()
            .map_err(RPCError::protocol)?,
    );

    let outbound_protocols = decode_protocol_type_set(
        &reader
            .reborrow()
            .get_outbound_protocols()
            .map_err(RPCError::protocol)?,
    )?;

    let address_types = decode_address_type_set(
        &reader
            .reborrow()
            .get_address_types()
            .map_err(RPCError::protocol)?,
    )?;

    let min_version = reader.reborrow().get_min_version();
    let max_version = reader.reborrow().get_max_version();

    let didl_reader = reader
        .reborrow()
        .get_dial_info_detail_list()
        .map_err(RPCError::protocol)?;
    let mut dial_info_detail_list = Vec::<DialInfoDetail>::with_capacity(
        didl_reader
            .len()
            .try_into()
            .map_err(RPCError::map_protocol("too many dial info details"))?,
    );
    for did in didl_reader.iter() {
        dial_info_detail_list.push(decode_dial_info_detail(&did)?)
    }

    Ok(NodeInfo {
        network_class,
        outbound_protocols,
        address_types,
        min_version,
        max_version,
        dial_info_detail_list,
    })
}
