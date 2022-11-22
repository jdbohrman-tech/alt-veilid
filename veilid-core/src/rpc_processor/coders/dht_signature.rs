use crate::*;
use rpc_processor::*;

pub fn encode_signature(sig: &DHTSignature, builder: &mut veilid_capnp::signature512::Builder) {
    let sig = &sig.bytes;

    builder.set_u0(u64::from_be_bytes(
        sig[0..8].try_into().expect("slice with incorrect length"),
    ));
    builder.set_u1(u64::from_be_bytes(
        sig[8..16].try_into().expect("slice with incorrect length"),
    ));
    builder.set_u2(u64::from_be_bytes(
        sig[16..24].try_into().expect("slice with incorrect length"),
    ));
    builder.set_u3(u64::from_be_bytes(
        sig[24..32].try_into().expect("slice with incorrect length"),
    ));
    builder.set_u4(u64::from_be_bytes(
        sig[32..40].try_into().expect("slice with incorrect length"),
    ));
    builder.set_u5(u64::from_be_bytes(
        sig[40..48].try_into().expect("slice with incorrect length"),
    ));
    builder.set_u6(u64::from_be_bytes(
        sig[48..56].try_into().expect("slice with incorrect length"),
    ));
    builder.set_u7(u64::from_be_bytes(
        sig[56..64].try_into().expect("slice with incorrect length"),
    ));
}

pub fn decode_signature(reader: &veilid_capnp::signature512::Reader) -> DHTSignature {
    let u0 = reader.get_u0().to_be_bytes();
    let u1 = reader.get_u1().to_be_bytes();
    let u2 = reader.get_u2().to_be_bytes();
    let u3 = reader.get_u3().to_be_bytes();
    let u4 = reader.get_u4().to_be_bytes();
    let u5 = reader.get_u5().to_be_bytes();
    let u6 = reader.get_u6().to_be_bytes();
    let u7 = reader.get_u7().to_be_bytes();

    DHTSignature::new([
        u0[0], u0[1], u0[2], u0[3], u0[4], u0[5], u0[6], u0[7], // u0
        u1[0], u1[1], u1[2], u1[3], u1[4], u1[5], u1[6], u1[7], // u1
        u2[0], u2[1], u2[2], u2[3], u2[4], u2[5], u2[6], u2[7], // u2
        u3[0], u3[1], u3[2], u3[3], u3[4], u3[5], u3[6], u3[7], // u3
        u4[0], u4[1], u4[2], u4[3], u4[4], u4[5], u4[6], u4[7], // u4
        u5[0], u5[1], u5[2], u5[3], u5[4], u5[5], u5[6], u5[7], // u5
        u6[0], u6[1], u6[2], u6[3], u6[4], u6[5], u6[6], u6[7], // u6
        u7[0], u7[1], u7[2], u7[3], u7[4], u7[5], u7[6], u7[7], // u7
    ])
}
