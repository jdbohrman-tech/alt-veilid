use crate::*;
use rpc_processor::*;

pub fn encode_sender_info(
    sender_info: &SenderInfo,
    builder: &mut veilid_capnp::sender_info::Builder,
) -> Result<(), RPCError> {
    let mut sab = builder.reborrow().init_socket_address();
    encode_socket_address(&sender_info.socket_address, &mut sab)?;
    Ok(())
}

pub fn decode_sender_info(
    reader: &veilid_capnp::sender_info::Reader,
) -> Result<SenderInfo, RPCError> {
    let sa_reader = reader
        .reborrow()
        .get_socket_address()
        .map_err(RPCError::map_internal(
            "invalid socket address in sender_info",
        ))?;
    let socket_address = decode_socket_address(&sa_reader)?;

    Ok(SenderInfo { socket_address })
}
