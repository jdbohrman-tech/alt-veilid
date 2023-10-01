use super::*;

pub fn encode_node_info(
    node_info: &NodeInfo,
    builder: &mut veilid_capnp::node_info::Builder,
) -> Result<(), RPCError> {
    builder.set_network_class(encode_network_class(node_info.network_class()));

    let mut ps_builder = builder.reborrow().init_outbound_protocols();
    encode_protocol_type_set(&node_info.outbound_protocols(), &mut ps_builder)?;

    let mut ats_builder = builder.reborrow().init_address_types();
    encode_address_type_set(&node_info.address_types(), &mut ats_builder)?;

    let mut es_builder = builder
        .reborrow()
        .init_envelope_support(node_info.envelope_support().len() as u32);
    if let Some(s) = es_builder.as_slice() {
        s.clone_from_slice(node_info.envelope_support());
    }

    let mut cs_builder = builder
        .reborrow()
        .init_crypto_support(node_info.crypto_support().len() as u32);
    if let Some(s) = cs_builder.as_slice() {
        let csvec: Vec<u32> = node_info
            .crypto_support()
            .iter()
            .map(|x| u32::from_be_bytes(x.0))
            .collect();
        s.clone_from_slice(&csvec);
    }

    let mut cap_builder = builder
        .reborrow()
        .init_capabilities(node_info.capabilities().len() as u32);
    if let Some(s) = cap_builder.as_slice() {
        let capvec: Vec<u32> = node_info
            .capabilities()
            .iter()
            .map(|x| u32::from_be_bytes(x.0))
            .collect();

        s.clone_from_slice(&capvec);
    }
    let mut didl_builder = builder.reborrow().init_dial_info_detail_list(
        node_info
            .dial_info_detail_list()
            .len()
            .try_into()
            .map_err(RPCError::map_protocol(
                "too many dial info details in node info",
            ))?,
    );

    for idx in 0..node_info.dial_info_detail_list().len() {
        let mut did_builder = didl_builder.reborrow().get(idx as u32);
        encode_dial_info_detail(&node_info.dial_info_detail_list()[idx], &mut did_builder)?;
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

    let es_reader = reader
        .reborrow()
        .get_envelope_support()
        .map_err(RPCError::protocol)?;
    let envelope_support = es_reader.as_slice().map(|s| s.to_vec()).unwrap_or_default();

    // Ensure envelope versions are not duplicated
    // Unsorted is okay, some nodes may have a different envelope order preference
    // But nothing should show up more than once
    let mut eversions = envelope_support.clone();
    eversions.dedup();
    if eversions.len() != envelope_support.len() {
        return Err(RPCError::protocol("duplicate envelope versions"));
    }
    if envelope_support.len() > MAX_ENVELOPE_VERSIONS {
        return Err(RPCError::protocol("too many envelope versions"));
    }
    if envelope_support.is_empty() {
        return Err(RPCError::protocol("no envelope versions"));
    }

    let cs_reader = reader
        .reborrow()
        .get_crypto_support()
        .map_err(RPCError::protocol)?;

    if cs_reader.len() as usize > MAX_CRYPTO_KINDS {
        return Err(RPCError::protocol("too many crypto kinds"));
    }

    let crypto_support: Vec<CryptoKind> = cs_reader
        .as_slice()
        .map(|s| s.iter().map(|x| FourCC::from(x.to_be_bytes())).collect())
        .unwrap_or_default();

    // Ensure crypto kinds are not duplicated
    // Unsorted is okay, some nodes may have a different crypto order preference
    // But nothing should show up more than once
    let mut ckinds = crypto_support.clone();
    ckinds.dedup();
    if ckinds.len() != crypto_support.len() {
        return Err(RPCError::protocol("duplicate crypto kinds"));
    }
    if crypto_support.len() > MAX_CRYPTO_KINDS {
        return Err(RPCError::protocol("too many crypto kinds"));
    }
    if crypto_support.is_empty() {
        return Err(RPCError::protocol("no crypto kinds"));
    }

    let cap_reader = reader
        .reborrow()
        .get_capabilities()
        .map_err(RPCError::protocol)?;
    if cap_reader.len() as usize > MAX_CAPABILITIES {
        return Err(RPCError::protocol("too many capabilities"));
    }
    let capabilities = cap_reader
        .as_slice()
        .map(|s| s.iter().map(|x| FourCC::from(x.to_be_bytes())).collect())
        .unwrap_or_default();

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

    Ok(NodeInfo::new(
        network_class,
        outbound_protocols,
        address_types,
        envelope_support,
        crypto_support,
        capabilities,
        dial_info_detail_list,
    ))
}
