use super::*;

pub fn encode_signal_info(
    signal_info: &SignalInfo,
    builder: &mut veilid_capnp::operation_signal::Builder,
) -> Result<(), RPCError> {
    match signal_info {
        SignalInfo::HolePunch { receipt, peer_info } => {
            let mut hp_builder = builder.reborrow().init_hole_punch();
            let r_builder = hp_builder
                .reborrow()
                .init_receipt(receipt.len().try_into().map_err(RPCError::map_protocol(
                    "invalid receipt length in encode_signal_info",
                ))?);
            r_builder.copy_from_slice(receipt);
            let mut pi_builder = hp_builder.init_peer_info();
            encode_peer_info(peer_info, &mut pi_builder)?;
        }
        SignalInfo::ReverseConnect { receipt, peer_info } => {
            let mut rc_builder = builder.reborrow().init_reverse_connect();
            let r_builder = rc_builder
                .reborrow()
                .init_receipt(receipt.len().try_into().map_err(RPCError::map_protocol(
                    "invalid receipt length in encode_signal_info",
                ))?);
            r_builder.copy_from_slice(receipt);
            let mut pi_builder = rc_builder.init_peer_info();
            encode_peer_info(peer_info, &mut pi_builder)?;
        }
    }

    Ok(())
}

pub fn decode_signal_info(
    reader: &veilid_capnp::operation_signal::Reader,
    crypto: Crypto,
) -> Result<SignalInfo, RPCError> {
    Ok(
        match reader
            .which()
            .map_err(RPCError::map_internal("invalid signal operation"))?
        {
            veilid_capnp::operation_signal::HolePunch(r) => {
                // Extract hole punch reader
                let r = r.map_err(RPCError::protocol)?;
                let receipt = r
                    .get_receipt()
                    .map_err(RPCError::map_protocol(
                        "invalid receipt in hole punch signal info",
                    ))?
                    .to_vec();
                let pi_reader = r.get_peer_info().map_err(RPCError::map_protocol(
                    "invalid peer info in hole punch signal info",
                ))?;
                let peer_info = decode_peer_info(&pi_reader, crypto)?;

                SignalInfo::HolePunch { receipt, peer_info }
            }
            veilid_capnp::operation_signal::ReverseConnect(r) => {
                // Extract reverse connect reader
                let r = r.map_err(RPCError::protocol)?;
                let receipt = r
                    .get_receipt()
                    .map_err(RPCError::map_protocol(
                        "invalid receipt in hole punch signal info",
                    ))?
                    .to_vec();
                let pi_reader = r.get_peer_info().map_err(RPCError::map_protocol(
                    "invalid peer info in reverse connect signal info",
                ))?;
                let peer_info = decode_peer_info(&pi_reader, crypto)?;

                SignalInfo::ReverseConnect { receipt, peer_info }
            }
        },
    )
}
