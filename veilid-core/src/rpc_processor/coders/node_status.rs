use super::*;

pub fn encode_public_internet_node_status(
    public_internet_node_status: &PublicInternetNodeStatus,
    builder: &mut veilid_capnp::public_internet_node_status::Builder,
) -> Result<(), RPCError> {
    let mut cap_builder = builder
        .reborrow()
        .init_capabilities(public_internet_node_status.capabilities.len() as u32);
    if let Some(s) = cap_builder.as_slice() {
        let capvec: Vec<u32> = public_internet_node_status
            .capabilities
            .iter()
            .map(|x| u32::from_be_bytes(x.0))
            .collect();

        s.clone_from_slice(&capvec);
    }
    Ok(())
}

pub fn decode_public_internet_node_status(
    reader: &veilid_capnp::public_internet_node_status::Reader,
) -> Result<PublicInternetNodeStatus, RPCError> {
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

    Ok(PublicInternetNodeStatus { capabilities })
}

pub fn encode_local_network_node_status(
    local_network_node_status: &LocalNetworkNodeStatus,
    builder: &mut veilid_capnp::local_network_node_status::Builder,
) -> Result<(), RPCError> {
    let mut cap_builder = builder
        .reborrow()
        .init_capabilities(local_network_node_status.capabilities.len() as u32);
    if let Some(s) = cap_builder.as_slice() {
        let capvec: Vec<u32> = local_network_node_status
            .capabilities
            .iter()
            .map(|x| u32::from_be_bytes(x.0))
            .collect();

        s.clone_from_slice(&capvec);
    }
    Ok(())
}

pub fn decode_local_network_node_status(
    reader: &veilid_capnp::local_network_node_status::Reader,
) -> Result<LocalNetworkNodeStatus, RPCError> {
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

    Ok(LocalNetworkNodeStatus { capabilities })
}

pub fn encode_node_status(
    node_status: &NodeStatus,
    builder: &mut veilid_capnp::node_status::Builder,
) -> Result<(), RPCError> {
    match node_status {
        NodeStatus::PublicInternet(ns) => {
            let mut pi_builder = builder.reborrow().init_public_internet();
            encode_public_internet_node_status(&ns, &mut pi_builder)
        }
        NodeStatus::LocalNetwork(ns) => {
            let mut ln_builder = builder.reborrow().init_local_network();
            encode_local_network_node_status(&ns, &mut ln_builder)
        }
    }
}

pub fn decode_node_status(
    reader: &veilid_capnp::node_status::Reader,
) -> Result<NodeStatus, RPCError> {
    Ok(
        match reader
            .which()
            .map_err(RPCError::map_internal("invalid node status"))?
        {
            veilid_capnp::node_status::PublicInternet(pi) => {
                let r = pi.map_err(RPCError::protocol)?;
                let pins = decode_public_internet_node_status(&r)?;
                NodeStatus::PublicInternet(pins)
            }
            veilid_capnp::node_status::LocalNetwork(ln) => {
                let r = ln.map_err(RPCError::protocol)?;
                let lnns = decode_local_network_node_status(&r)?;
                NodeStatus::LocalNetwork(lnns)
            }
        },
    )
}
