use super::*;

pub fn encode_signed_relayed_node_info(
    signed_relayed_node_info: &SignedRelayedNodeInfo,
    builder: &mut veilid_capnp::signed_relayed_node_info::Builder,
) -> Result<(), RPCError> {
    //
    let mut ni_builder = builder.reborrow().init_node_info();
    encode_node_info(&signed_relayed_node_info.node_info, &mut ni_builder)?;

    let mut rids_builder = builder.reborrow().init_relay_ids(
        signed_relayed_node_info
            .relay_ids
            .len()
            .try_into()
            .map_err(RPCError::map_invalid_format("out of bound error"))?,
    );
    for (i, typed_key) in signed_relayed_node_info.relay_ids.iter().enumerate() {
        encode_typed_key(
            typed_key,
            &mut rids_builder.reborrow().get(
                i.try_into()
                    .map_err(RPCError::map_invalid_format("out of bound error"))?,
            ),
        );
    }

    let mut ri_builder = builder.reborrow().init_relay_info();
    encode_signed_direct_node_info(&signed_relayed_node_info.relay_info, &mut ri_builder)?;

    builder
        .reborrow()
        .set_timestamp(signed_relayed_node_info.timestamp.into());

    let mut sigs_builder = builder.reborrow().init_signatures(
        signed_relayed_node_info
            .signatures
            .len()
            .try_into()
            .map_err(RPCError::map_invalid_format("out of bound error"))?,
    );
    for (i, typed_signature) in signed_relayed_node_info.signatures.iter().enumerate() {
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

pub fn decode_signed_relayed_node_info(
    reader: &veilid_capnp::signed_relayed_node_info::Reader,
    crypto: Crypto,
    node_ids: &mut TypedKeySet,
) -> Result<SignedRelayedNodeInfo, RPCError> {
    let ni_reader = reader
        .reborrow()
        .get_node_info()
        .map_err(RPCError::protocol)?;
    let node_info = decode_node_info(&ni_reader)?;

    let rids_reader = reader
        .reborrow()
        .get_relay_ids()
        .map_err(RPCError::protocol)?;
    let rid_count = rids_reader.len() as usize;
    if rid_count > MAX_CRYPTO_KINDS {
        return Err(RPCError::protocol("too many relay ids"));
    }
    let mut relay_ids = TypedKeySet::with_capacity(rid_count);
    for rid_reader in rids_reader {
        let relay_id = decode_typed_key(&rid_reader)?;
        relay_ids.add(relay_id);
    }

    let ri_reader = reader
        .reborrow()
        .get_relay_info()
        .map_err(RPCError::protocol)?;
    let relay_info = decode_signed_direct_node_info(&ri_reader, crypto.clone(), &mut relay_ids)?;

    // Ensure the relay info for the node has a superset of the crypto kinds of the node it is relaying
    if common_crypto_kinds(
        &node_info.crypto_support,
        &relay_info.node_info.crypto_support,
    )
    .len()
        != node_info.crypto_support.len()
    {
        return Err(RPCError::protocol(
            "relay should have superset of node crypto kinds",
        ));
    }

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
    SignedRelayedNodeInfo::new(
        crypto,
        node_ids,
        node_info,
        relay_ids,
        relay_info,
        timestamp,
        typed_signatures,
    )
    .map_err(RPCError::protocol)
}
