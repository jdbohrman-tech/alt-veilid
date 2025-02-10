use super::*;

#[derive(Debug, Clone)]
pub(in crate::rpc_processor) struct RPCOperationSignal {
    signal_info: SignalInfo,
}

impl RPCOperationSignal {
    pub fn new(signal_info: SignalInfo) -> Self {
        Self { signal_info }
    }
    pub fn validate(&mut self, validate_context: &RPCValidateContext) -> Result<(), RPCError> {
        let crypto = validate_context.crypto();
        self.signal_info.validate(&crypto)
    }
    // pub fn signal_info(&self) -> &SignalInfo {
    //     &self.signal_info
    // }
    pub fn destructure(self) -> SignalInfo {
        self.signal_info
    }

    pub fn decode(
        decode_context: &RPCDecodeContext,
        reader: &veilid_capnp::operation_signal::Reader,
    ) -> Result<Self, RPCError> {
        let signal_info = decode_signal_info(decode_context, reader)?;
        Ok(Self { signal_info })
    }
    pub fn encode(
        &self,
        builder: &mut veilid_capnp::operation_signal::Builder,
    ) -> Result<(), RPCError> {
        encode_signal_info(&self.signal_info, builder)?;
        Ok(())
    }
}
