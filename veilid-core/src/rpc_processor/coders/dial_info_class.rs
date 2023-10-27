use super::*;

pub(crate) fn encode_dial_info_class(
    dial_info_class: DialInfoClass,
) -> veilid_capnp::DialInfoClass {
    match dial_info_class {
        DialInfoClass::Direct => veilid_capnp::DialInfoClass::Direct,
        DialInfoClass::Mapped => veilid_capnp::DialInfoClass::Mapped,
        DialInfoClass::FullConeNAT => veilid_capnp::DialInfoClass::FullConeNAT,
        DialInfoClass::Blocked => veilid_capnp::DialInfoClass::Blocked,
        DialInfoClass::AddressRestrictedNAT => veilid_capnp::DialInfoClass::AddressRestrictedNAT,
        DialInfoClass::PortRestrictedNAT => veilid_capnp::DialInfoClass::PortRestrictedNAT,
    }
}

pub(crate) fn decode_dial_info_class(
    dial_info_class: veilid_capnp::DialInfoClass,
) -> DialInfoClass {
    match dial_info_class {
        veilid_capnp::DialInfoClass::Direct => DialInfoClass::Direct,
        veilid_capnp::DialInfoClass::Mapped => DialInfoClass::Mapped,
        veilid_capnp::DialInfoClass::FullConeNAT => DialInfoClass::FullConeNAT,
        veilid_capnp::DialInfoClass::Blocked => DialInfoClass::Blocked,
        veilid_capnp::DialInfoClass::AddressRestrictedNAT => DialInfoClass::AddressRestrictedNAT,
        veilid_capnp::DialInfoClass::PortRestrictedNAT => DialInfoClass::PortRestrictedNAT,
    }
}
