use super::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) fn encode_route_hop_data(
    route_hop_data: &RouteHopData,
    builder: &mut veilid_capnp::route_hop_data::Builder,
) -> Result<(), RPCError> {
    //
    let mut nonce_builder = builder.reborrow().init_nonce();
    encode_nonce(&route_hop_data.nonce, &mut nonce_builder);
    let blob_builder = builder
        .reborrow()
        .init_blob(
            route_hop_data
                .blob
                .len()
                .try_into()
                .map_err(RPCError::map_protocol(
                    "invalid blob length in route hop data",
                ))?,
        );
    blob_builder.copy_from_slice(route_hop_data.blob.as_slice());
    Ok(())
}

pub(crate) fn decode_route_hop_data(
    reader: &veilid_capnp::route_hop_data::Reader,
) -> Result<RouteHopData, RPCError> {
    let nonce = decode_nonce(
        &reader
            .reborrow()
            .get_nonce()
            .map_err(RPCError::map_protocol("invalid nonce in route hop data"))?,
    );

    let blob = reader
        .reborrow()
        .get_blob()
        .map_err(RPCError::map_protocol("invalid blob in route hop data"))?
        .to_vec();

    Ok(RouteHopData { nonce, blob })
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) fn encode_route_hop(
    route_hop: &RouteHop,
    builder: &mut veilid_capnp::route_hop::Builder,
) -> Result<(), RPCError> {
    let node_builder = builder.reborrow().init_node();
    match &route_hop.node {
        RouteNode::NodeId(ni) => {
            let mut ni_builder = node_builder.init_node_id();
            encode_key256(ni, &mut ni_builder);
        }
        RouteNode::PeerInfo(pi) => {
            let mut pi_builder = node_builder.init_peer_info();
            encode_peer_info(pi, &mut pi_builder)?;
        }
    }
    if let Some(rhd) = &route_hop.next_hop {
        let mut rhd_builder = builder.reborrow().init_next_hop();
        encode_route_hop_data(rhd, &mut rhd_builder)?;
    }
    Ok(())
}

pub(crate) fn decode_route_hop(
    reader: &veilid_capnp::route_hop::Reader,
) -> Result<RouteHop, RPCError> {
    let n_reader = reader.reborrow().get_node();
    let node = match n_reader.which().map_err(RPCError::protocol)? {
        veilid_capnp::route_hop::node::Which::NodeId(ni) => {
            let ni_reader = ni.map_err(RPCError::protocol)?;
            RouteNode::NodeId(decode_key256(&ni_reader))
        }
        veilid_capnp::route_hop::node::Which::PeerInfo(pi) => {
            let pi_reader = pi.map_err(RPCError::protocol)?;
            RouteNode::PeerInfo(Box::new(
                decode_peer_info(&pi_reader)
                    .map_err(RPCError::map_protocol("invalid peer info in route hop"))?,
            ))
        }
    };

    let next_hop = if reader.has_next_hop() {
        let rhd_reader = reader
            .get_next_hop()
            .map_err(RPCError::map_protocol("invalid next hop in route hop"))?;
        Some(decode_route_hop_data(&rhd_reader)?)
    } else {
        None
    };

    Ok(RouteHop { node, next_hop })
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) fn encode_private_route(
    private_route: &PrivateRoute,
    builder: &mut veilid_capnp::private_route::Builder,
) -> Result<(), RPCError> {
    encode_typed_key(
        &private_route.public_key,
        &mut builder.reborrow().init_public_key(),
    );
    builder.set_hop_count(private_route.hop_count);
    let mut h_builder = builder.reborrow().init_hops();
    match &private_route.hops {
        PrivateRouteHops::FirstHop(first_hop) => {
            let mut rh_builder = h_builder.init_first_hop();
            encode_route_hop(first_hop, &mut rh_builder)?;
        }
        PrivateRouteHops::Data(data) => {
            let mut rhd_builder = h_builder.init_data();
            encode_route_hop_data(data, &mut rhd_builder)?;
        }
        PrivateRouteHops::Empty => {
            h_builder.set_empty(());
        }
    };
    Ok(())
}

pub(crate) fn decode_private_route(
    reader: &veilid_capnp::private_route::Reader,
) -> Result<PrivateRoute, RPCError> {
    let public_key = decode_typed_key(&reader.get_public_key().map_err(
        RPCError::map_protocol("invalid public key in private route"),
    )?)?;
    let hop_count = reader.get_hop_count();

    let hops = match reader.get_hops().which().map_err(RPCError::protocol)? {
        veilid_capnp::private_route::hops::Which::FirstHop(rh_reader) => {
            let rh_reader = rh_reader.map_err(RPCError::protocol)?;
            PrivateRouteHops::FirstHop(Box::new(decode_route_hop(&rh_reader)?))
        }
        veilid_capnp::private_route::hops::Which::Data(rhd_reader) => {
            let rhd_reader = rhd_reader.map_err(RPCError::protocol)?;
            PrivateRouteHops::Data(decode_route_hop_data(&rhd_reader)?)
        }
        veilid_capnp::private_route::hops::Which::Empty(_) => PrivateRouteHops::Empty,
    };

    Ok(PrivateRoute {
        public_key,
        hop_count,
        hops,
    })
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) fn encode_safety_route(
    safety_route: &SafetyRoute,
    builder: &mut veilid_capnp::safety_route::Builder,
) -> Result<(), RPCError> {
    encode_typed_key(
        &safety_route.public_key,
        &mut builder.reborrow().init_public_key(),
    );
    builder.set_hop_count(safety_route.hop_count);
    let h_builder = builder.reborrow().init_hops();
    match &safety_route.hops {
        SafetyRouteHops::Data(rhd) => {
            let mut rhd_builder = h_builder.init_data();
            encode_route_hop_data(rhd, &mut rhd_builder)?;
        }
        SafetyRouteHops::Private(pr) => {
            let mut pr_builder = h_builder.init_private();
            encode_private_route(pr, &mut pr_builder)?;
        }
    };

    Ok(())
}

pub(crate) fn decode_safety_route(
    reader: &veilid_capnp::safety_route::Reader,
) -> Result<SafetyRoute, RPCError> {
    let public_key = decode_typed_key(
        &reader
            .get_public_key()
            .map_err(RPCError::map_protocol("invalid public key in safety route"))?,
    )?;
    let hop_count = reader.get_hop_count();
    let hops = match reader.get_hops().which().map_err(RPCError::protocol)? {
        veilid_capnp::safety_route::hops::Which::Data(rhd_reader) => {
            let rhd_reader = rhd_reader.map_err(RPCError::protocol)?;
            SafetyRouteHops::Data(decode_route_hop_data(&rhd_reader)?)
        }
        veilid_capnp::safety_route::hops::Which::Private(pr_reader) => {
            let pr_reader = pr_reader.map_err(RPCError::protocol)?;
            SafetyRouteHops::Private(decode_private_route(&pr_reader)?)
        }
    };

    Ok(SafetyRoute {
        public_key,
        hop_count,
        hops,
    })
}
