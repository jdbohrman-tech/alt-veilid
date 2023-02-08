use super::*;

pub fn encode_peer_info(
    peer_info: &PeerInfo,
    builder: &mut veilid_capnp::peer_info::Builder,
) -> Result<(), RPCError> {
    //
    let mut nids_builder = builder.reborrow().init_node_ids(
        peer_info
            .node_ids
            .len()
            .try_into()
            .map_err(RPCError::map_invalid_format("out of bound error"))?,
    );
    for (i, nid) in peer_info.node_ids.iter().enumerate() {
        encode_typed_key(
            nid,
            &mut nids_builder.get(
                i.try_into()
                    .map_err(RPCError::map_invalid_format("out of bound error"))?,
            ),
        );
    }
    let mut sni_builder = builder.reborrow().init_signed_node_info();
    encode_signed_node_info(&peer_info.signed_node_info, &mut sni_builder)?;

    Ok(())
}

pub fn decode_peer_info(
    reader: &veilid_capnp::peer_info::Reader,
    crypto: Crypto,
) -> Result<PeerInfo, RPCError> {
    let nids_reader = reader
        .reborrow()
        .get_node_ids()
        .map_err(RPCError::protocol)?;
    let sni_reader = reader
        .reborrow()
        .get_signed_node_info()
        .map_err(RPCError::protocol)?;
    let node_ids = Vec::with_capacity(nids_reader.len() as usize);
    for nid_reader in nids_reader.iter() {
        node_ids.push(decode_typed_key(&nid_reader)?);
    }
    let signed_node_info = decode_signed_node_info(&sni_reader, crypto, &node_ids)?;

    Ok(PeerInfo {
        node_ids,
        signed_node_info,
    })
}
