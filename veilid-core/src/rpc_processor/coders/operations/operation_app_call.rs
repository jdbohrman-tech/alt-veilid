use super::*;

const MAX_APP_CALL_Q_MESSAGE_LEN: usize = 32768;
const MAX_APP_CALL_A_MESSAGE_LEN: usize = 32768;

#[derive(Debug, Clone)]
pub(in crate::rpc_processor) struct RPCOperationAppCallQ {
    message: Vec<u8>,
}

impl RPCOperationAppCallQ {
    pub fn new(message: Vec<u8>) -> Result<Self, RPCError> {
        if message.len() > MAX_APP_CALL_Q_MESSAGE_LEN {
            return Err(RPCError::protocol("AppCallQ message too long to set"));
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

    pub fn decode(reader: &veilid_capnp::operation_app_call_q::Reader) -> Result<Self, RPCError> {
        let mr = reader.get_message().map_err(RPCError::protocol)?;
        if mr.len() > MAX_APP_CALL_Q_MESSAGE_LEN {
            return Err(RPCError::protocol("AppCallQ message too long to set"));
        }
        Ok(Self {
            message: mr.to_vec(),
        })
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
pub(in crate::rpc_processor) struct RPCOperationAppCallA {
    message: Vec<u8>,
}

impl RPCOperationAppCallA {
    pub fn new(message: Vec<u8>) -> Result<Self, RPCError> {
        if message.len() > MAX_APP_CALL_A_MESSAGE_LEN {
            return Err(RPCError::protocol("AppCallA message too long to set"));
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

    pub fn decode(reader: &veilid_capnp::operation_app_call_a::Reader) -> Result<Self, RPCError> {
        let mr = reader.get_message().map_err(RPCError::protocol)?;
        if mr.len() > MAX_APP_CALL_A_MESSAGE_LEN {
            return Err(RPCError::protocol("AppCallA message too long to set"));
        }
        Ok(Self {
            message: mr.to_vec(),
        })
    }
    pub fn encode(
        &self,
        builder: &mut veilid_capnp::operation_app_call_a::Builder,
    ) -> Result<(), RPCError> {
        builder.set_message(&self.message);
        Ok(())
    }
}
