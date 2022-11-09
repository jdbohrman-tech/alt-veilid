use crate::*;
use rpc_processor::*;

#[derive(Debug, Clone)]
pub struct RPCOperationNodeInfoUpdate {
    pub signed_node_info: SignedNodeInfo,
}

impl RPCOperationNodeInfoUpdate {
    pub fn decode(
        reader: &veilid_capnp::operation_node_info_update::Reader,
        opt_sender_node_id: Option<&DHTKey>,
    ) -> Result<RPCOperationNodeInfoUpdate, RPCError> {
        if opt_sender_node_id.is_none() {
            return Err(RPCError::protocol(
                "can't decode node info update without sender node id",
            ));
        }
        let sender_node_id = opt_sender_node_id.unwrap();
        let sni_reader = reader.get_signed_node_info().map_err(RPCError::protocol)?;
        let signed_node_info = decode_signed_node_info(&sni_reader, sender_node_id)?;

        Ok(RPCOperationNodeInfoUpdate { signed_node_info })
    }
    pub fn encode(
        &self,
        builder: &mut veilid_capnp::operation_node_info_update::Builder,
    ) -> Result<(), RPCError> {
        let mut sni_builder = builder.reborrow().init_signed_node_info();
        encode_signed_node_info(&self.signed_node_info, &mut sni_builder)?;
        Ok(())
    }
}
