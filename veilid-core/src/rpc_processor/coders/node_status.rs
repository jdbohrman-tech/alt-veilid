use super::*;

pub fn encode_public_internet_node_status(
    public_internet_node_status: &PublicInternetNodeStatus,
    builder: &mut veilid_capnp::public_internet_node_status::Builder,
) -> Result<(), RPCError> {
    builder.set_will_route(public_internet_node_status.will_route);
    builder.set_will_tunnel(public_internet_node_status.will_tunnel);
    builder.set_will_signal(public_internet_node_status.will_signal);
    builder.set_will_relay(public_internet_node_status.will_relay);
    builder.set_will_validate_dial_info(public_internet_node_status.will_validate_dial_info);

    Ok(())
}

pub fn decode_public_internet_node_status(
    reader: &veilid_capnp::public_internet_node_status::Reader,
) -> Result<PublicInternetNodeStatus, RPCError> {
    Ok(PublicInternetNodeStatus {
        will_route: reader.reborrow().get_will_route(),
        will_tunnel: reader.reborrow().get_will_tunnel(),
        will_signal: reader.reborrow().get_will_signal(),
        will_relay: reader.reborrow().get_will_relay(),
        will_validate_dial_info: reader.reborrow().get_will_validate_dial_info(),
    })
}

pub fn encode_local_network_node_status(
    local_network_node_status: &LocalNetworkNodeStatus,
    builder: &mut veilid_capnp::local_network_node_status::Builder,
) -> Result<(), RPCError> {
    builder.set_will_relay(local_network_node_status.will_relay);
    builder.set_will_validate_dial_info(local_network_node_status.will_validate_dial_info);

    Ok(())
}

pub fn decode_local_network_node_status(
    reader: &veilid_capnp::local_network_node_status::Reader,
) -> Result<LocalNetworkNodeStatus, RPCError> {
    Ok(LocalNetworkNodeStatus {
        will_relay: reader.reborrow().get_will_relay(),
        will_validate_dial_info: reader.reborrow().get_will_validate_dial_info(),
    })
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
