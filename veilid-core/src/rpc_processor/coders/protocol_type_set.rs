use super::*;

pub fn encode_protocol_type_set(
    protocol_type_set: &ProtocolTypeSet,
    builder: &mut veilid_capnp::protocol_type_set::Builder,
) -> Result<(), RPCError> {
    builder.set_udp(protocol_type_set.contains(ProtocolType::UDP));
    builder.set_tcp(protocol_type_set.contains(ProtocolType::TCP));
    builder.set_ws(protocol_type_set.contains(ProtocolType::WS));
    builder.set_wss(protocol_type_set.contains(ProtocolType::WSS));

    Ok(())
}

pub fn decode_protocol_type_set(
    reader: &veilid_capnp::protocol_type_set::Reader,
) -> Result<ProtocolTypeSet, RPCError> {
    let mut out = ProtocolTypeSet::new();
    if reader.reborrow().get_udp() {
        out.insert(ProtocolType::UDP);
    }
    if reader.reborrow().get_tcp() {
        out.insert(ProtocolType::TCP);
    }
    if reader.reborrow().get_ws() {
        out.insert(ProtocolType::WS);
    }
    if reader.reborrow().get_wss() {
        out.insert(ProtocolType::WSS);
    }
    Ok(out)
}
