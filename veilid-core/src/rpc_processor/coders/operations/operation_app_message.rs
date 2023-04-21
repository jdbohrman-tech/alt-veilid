use super::*;

const MAX_APP_MESSAGE_MESSAGE_LEN: usize = 32768;

#[derive(Debug, Clone)]
pub struct RPCOperationAppMessage {
    message: Vec<u8>,
}

impl RPCOperationAppMessage {
    pub fn new(message: &[u8]) -> Result<Self, RPCError> {
        if message.len() > MAX_APP_MESSAGE_MESSAGE_LEN {
            return Err(RPCError::protocol("AppMessage message too long to set"));
        }
        Ok(Self {
            message: message.to_vec(),
        })
    }

    pub fn validate(&mut self, _validate_context: &RPCValidateContext) -> Result<(), RPCError> {
        Ok(())
    }
    pub fn decode(
        reader: &veilid_capnp::operation_app_message::Reader,
    ) -> Result<RPCOperationAppMessage, RPCError> {
        let mr = reader.get_message().map_err(RPCError::protocol)?;
        RPCOperationAppMessage::new(mr)
    }
    pub fn encode(
        &self,
        builder: &mut veilid_capnp::operation_app_message::Builder,
    ) -> Result<(), RPCError> {
        builder.set_message(&self.message);
        Ok(())
    }

    pub fn message(&self) -> &[u8] {
        &self.message
    }

    pub fn destructure(self) -> Vec<u8> {
        self.message
    }
}
