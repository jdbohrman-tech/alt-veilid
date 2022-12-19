use super::*;

pub fn encode_sequencing(sequencing: Sequencing) -> veilid_capnp::Sequencing {
    match sequencing {
        Sequencing::NoPreference => veilid_capnp::Sequencing::NoPreference,
        Sequencing::PreferOrdered => veilid_capnp::Sequencing::PreferOrdered,
        Sequencing::EnsureOrdered => veilid_capnp::Sequencing::EnsureOrdered,
    }
}

pub fn decode_sequencing(sequencing: veilid_capnp::Sequencing) -> Sequencing {
    match sequencing {
        veilid_capnp::Sequencing::NoPreference => Sequencing::NoPreference,
        veilid_capnp::Sequencing::PreferOrdered => Sequencing::PreferOrdered,
        veilid_capnp::Sequencing::EnsureOrdered => Sequencing::EnsureOrdered,
    }
}
