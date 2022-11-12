use crate::crypto::*;
use crate::*;
use core::convert::TryInto;
use rpc_processor::*;

pub fn decode_block_id(public_key: &veilid_capnp::b_l_a_k_e3_hash::Reader) -> DHTKey {
    let u0 = public_key.get_u0().to_be_bytes();
    let u1 = public_key.get_u1().to_be_bytes();
    let u2 = public_key.get_u2().to_be_bytes();
    let u3 = public_key.get_u3().to_be_bytes();

    let mut x: [u8; 32] = Default::default();
    x[0..8].copy_from_slice(&u0);
    x[8..16].copy_from_slice(&u1);
    x[16..24].copy_from_slice(&u2);
    x[24..32].copy_from_slice(&u3);

    DHTKey::new(x)
}

pub fn encode_block_id(
    key: &DHTKey,
    builder: &mut veilid_capnp::b_l_a_k_e3_hash::Builder,
) -> Result<(), RPCError> {
    builder.set_u0(u64::from_be_bytes(
        key.bytes[0..8].try_into().map_err(RPCError::internal)?,
    ));
    builder.set_u1(u64::from_be_bytes(
        key.bytes[8..16].try_into().map_err(RPCError::internal)?,
    ));
    builder.set_u2(u64::from_be_bytes(
        key.bytes[16..24].try_into().map_err(RPCError::internal)?,
    ));
    builder.set_u3(u64::from_be_bytes(
        key.bytes[24..32].try_into().map_err(RPCError::internal)?,
    ));
    Ok(())
}
