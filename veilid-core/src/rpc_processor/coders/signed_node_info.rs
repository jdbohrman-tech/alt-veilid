use super::*;

pub fn encode_signed_node_info(
    signed_node_info: &SignedNodeInfo,
    builder: &mut veilid_capnp::signed_node_info::Builder,
) -> Result<(), RPCError> {
    match signed_node_info {
        SignedNodeInfo::Direct(d) => {
            let mut d_builder = builder.reborrow().init_direct();
            encode_signed_direct_node_info(d, &mut d_builder)?;
        }
        SignedNodeInfo::Relayed(r) => {
            let mut r_builder = builder.reborrow().init_relayed();
            encode_signed_relayed_node_info(r, &mut r_builder)?;
        }
    }

    Ok(())
}

pub fn decode_signed_node_info(
    reader: &veilid_capnp::signed_node_info::Reader,
) -> Result<SignedNodeInfo, RPCError> {
    match reader
        .which()
        .map_err(RPCError::map_internal("invalid signed node info"))?
    {
        veilid_capnp::signed_node_info::Direct(d) => {
            let d_reader = d.map_err(RPCError::protocol)?;
            let sdni = decode_signed_direct_node_info(&d_reader)?;
            Ok(SignedNodeInfo::Direct(sdni))
        }
        veilid_capnp::signed_node_info::Relayed(r) => {
            let r_reader = r.map_err(RPCError::protocol)?;
            let srni = decode_signed_relayed_node_info(&r_reader)?;
            Ok(SignedNodeInfo::Relayed(srni))
        }
    }
}
