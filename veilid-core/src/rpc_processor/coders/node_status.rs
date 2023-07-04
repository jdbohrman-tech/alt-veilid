use super::*;

pub fn encode_node_status(
    _node_status: &NodeStatus,
    _builder: &mut veilid_capnp::node_status::Builder,
) -> Result<(), RPCError> {
    Ok(())
}

pub fn decode_node_status(
    _reader: &veilid_capnp::node_status::Reader,
) -> Result<NodeStatus, RPCError> {
    Ok(NodeStatus {})
}
