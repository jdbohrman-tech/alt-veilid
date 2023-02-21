use super::*;

#[derive(Debug, Clone)]
pub struct RPCOperationSignal {
    pub signal_info: SignalInfo,
}

impl RPCOperationSignal {
    pub fn decode(
        reader: &veilid_capnp::operation_signal::Reader,
        crypto: Crypto,
    ) -> Result<RPCOperationSignal, RPCError> {
        let signal_info = decode_signal_info(reader, crypto)?;
        Ok(RPCOperationSignal { signal_info })
    }
    pub fn encode(
        &self,
        builder: &mut veilid_capnp::operation_signal::Builder,
    ) -> Result<(), RPCError> {
        encode_signal_info(&self.signal_info, builder)?;
        Ok(())
    }
}
