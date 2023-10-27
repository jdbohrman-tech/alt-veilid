use super::*;

pub(crate) fn encode_address_type_set(
    address_type_set: &AddressTypeSet,
    builder: &mut veilid_capnp::address_type_set::Builder,
) -> Result<(), RPCError> {
    builder.set_ipv4(address_type_set.contains(AddressType::IPV4));
    builder.set_ipv6(address_type_set.contains(AddressType::IPV6));

    Ok(())
}

pub(crate) fn decode_address_type_set(
    reader: &veilid_capnp::address_type_set::Reader,
) -> Result<AddressTypeSet, RPCError> {
    let mut out = AddressTypeSet::new();
    if reader.reborrow().get_ipv4() {
        out.insert(AddressType::IPV4);
    }
    if reader.reborrow().get_ipv6() {
        out.insert(AddressType::IPV6);
    }
    Ok(out)
}
