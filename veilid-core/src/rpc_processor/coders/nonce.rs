use super::*;

pub fn encode_nonce(nonce: &Nonce, builder: &mut veilid_capnp::nonce24::Builder) {
    builder.set_u0(u64::from_be_bytes(
        nonce[0..8].try_into().expect("slice with incorrect length"),
    ));
    builder.set_u1(u64::from_be_bytes(
        nonce[8..16]
            .try_into()
            .expect("slice with incorrect length"),
    ));
    builder.set_u2(u64::from_be_bytes(
        nonce[16..24]
            .try_into()
            .expect("slice with incorrect length"),
    ));
}

pub fn decode_nonce(reader: &veilid_capnp::nonce24::Reader) -> Nonce {
    let u0 = reader.get_u0().to_be_bytes();
    let u1 = reader.get_u1().to_be_bytes();
    let u2 = reader.get_u2().to_be_bytes();

    [
        u0[0], u0[1], u0[2], u0[3], u0[4], u0[5], u0[6], u0[7], // u0
        u1[0], u1[1], u1[2], u1[3], u1[4], u1[5], u1[6], u1[7], // u1
        u2[0], u2[1], u2[2], u2[3], u2[4], u2[5], u2[6], u2[7], // u2
    ]
}
