use super::*;

#[derive(Debug, Clone)]
pub struct RPCOperationAppCallQ {
    pub message: Vec<u8>,
}

impl RPCOperationAppCallQ {
    pub fn decode(
        reader: &veilid_capnp::operation_app_call_q::Reader,
    ) -> Result<RPCOperationAppCallQ, RPCError> {
        let message = reader.get_message().map_err(RPCError::protocol)?.to_vec();
        Ok(RPCOperationAppCallQ { message })
    }
    pub fn encode(
        &self,
        builder: &mut veilid_capnp::operation_app_call_q::Builder,
    ) -> Result<(), RPCError> {
        builder.set_message(&self.message);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct RPCOperationAppCallA {
    pub message: Vec<u8>,
}

impl RPCOperationAppCallA {
    pub fn decode(
        reader: &veilid_capnp::operation_app_call_a::Reader,
    ) -> Result<RPCOperationAppCallA, RPCError> {
        let message = reader.get_message().map_err(RPCError::protocol)?.to_vec();
        Ok(RPCOperationAppCallA { message })
    }
    pub fn encode(
        &self,
        builder: &mut veilid_capnp::operation_app_call_a::Builder,
    ) -> Result<(), RPCError> {
        builder.set_message(&self.message);
        Ok(())
    }
}
