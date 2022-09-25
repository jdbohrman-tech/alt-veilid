use crate::*;
use rpc_processor::*;

#[derive(Debug, Clone)]
pub struct RPCOperationAppMessage {
    pub message: Vec<u8>,
}

impl RPCOperationAppMessage {
    pub fn decode(
        reader: &veilid_capnp::operation_app_message::Reader,
    ) -> Result<RPCOperationAppMessage, RPCError> {
        let message = reader.get_message().map_err(RPCError::protocol)?.to_vec();
        Ok(RPCOperationAppMessage { message })
    }
    pub fn encode(
        &self,
        builder: &mut veilid_capnp::operation_app_message::Builder,
    ) -> Result<(), RPCError> {
        builder.set_message(&self.message);
        Ok(())
    }
}
