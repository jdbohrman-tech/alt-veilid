mod address;
mod address_type_set;
mod dial_info;
mod dial_info_class;
mod dial_info_detail;
mod key256;
mod network_class;
mod node_info;
mod node_status;
mod nonce;
mod operations;
mod peer_info;
mod private_safety_route;
mod protocol_type_set;
mod sender_info;
mod sequencing;
mod signal_info;
mod signature512;
mod signed_direct_node_info;
mod signed_node_info;
mod signed_relayed_node_info;
mod signed_value_data;
mod signed_value_descriptor;
mod socket_address;
#[cfg(feature = "unstable-tunnels")]
mod tunnel;
mod typed_key;
mod typed_signature;

pub(in crate::rpc_processor) use operations::*;

pub(crate) use address::*;
pub(crate) use address_type_set::*;
pub(crate) use dial_info::*;
pub(crate) use dial_info_class::*;
pub(crate) use dial_info_detail::*;
pub(crate) use key256::*;
pub(crate) use network_class::*;
pub(crate) use node_info::*;
pub(crate) use node_status::*;
pub(crate) use nonce::*;
pub(crate) use peer_info::*;
pub(crate) use private_safety_route::*;
pub(crate) use protocol_type_set::*;
pub(crate) use sender_info::*;
pub use sequencing::*;
pub use signal_info::*;
pub use signature512::*;
pub use signed_direct_node_info::*;
pub use signed_node_info::*;
pub use signed_relayed_node_info::*;
pub use signed_value_data::*;
pub use signed_value_descriptor::*;
pub use socket_address::*;
#[cfg(feature = "unstable-tunnels")]
pub use tunnel::*;
pub use typed_key::*;
pub use typed_signature::*;

use super::*;

#[derive(Debug, Clone)]
pub(in crate::rpc_processor) enum QuestionContext {
    GetValue(ValidateGetValueContext),
    SetValue(ValidateSetValueContext),
    InspectValue(ValidateInspectValueContext),
}

#[derive(Clone)]
pub(in crate::rpc_processor) struct RPCValidateContext {
    pub crypto: Crypto,
    // pub rpc_processor: RPCProcessor,
    pub question_context: Option<QuestionContext>,
}
