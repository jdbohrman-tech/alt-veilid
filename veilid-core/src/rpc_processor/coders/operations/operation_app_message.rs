use super::*;

const MAX_APP_MESSAGE_MESSAGE_LEN: usize = 32768;

#[derive(Debug, Clone)]
pub(in crate::rpc_processor) struct RPCOperationAppMessage {
    message: Vec<u8>,
}

impl RPCOperationAppMessage {
    pub fn new(message: Vec<u8>) -> Result<Self, RPCError> {
        if message.len() > MAX_APP_MESSAGE_MESSAGE_LEN {
            return Err(RPCError::protocol("AppMessage message too long to set"));
        }
        Ok(Self { message })
    }

    pub fn validate(&mut self, _validate_context: &RPCValidateContext) -> Result<(), RPCError> {
        Ok(())
    }

    // pub fn message(&self) -> &[u8] {
    //     &self.message
    // }
    pub fn destructure(self) -> Vec<u8> {
        self.message
    }

    pub fn decode(reader: &veilid_capnp::operation_app_message::Reader) -> Result<Self, RPCError> {
        let mr = reader.get_message().map_err(RPCError::protocol)?;
        if mr.len() > MAX_APP_MESSAGE_MESSAGE_LEN {
            return Err(RPCError::protocol("AppMessage message too long to set"));
        }
        Ok(Self {
            message: mr.to_vec(),
        })
    }
    pub fn encode(
        &self,
        builder: &mut veilid_capnp::operation_app_message::Builder,
    ) -> Result<(), RPCError> {
        builder.set_message(&self.message);
        Ok(())
    }
}
