use super::*;

#[derive(Debug, Clone)]
pub struct RPCOperationReturnReceipt {
    receipt: Vec<u8>,
}

impl RPCOperationReturnReceipt {
    pub fn new(receipt: &[u8]) -> Result<Self, RPCError> {
        if receipt.len() < MIN_RECEIPT_SIZE {
            return Err(RPCError::protocol("ReturnReceipt receipt too short to set"));
        }
        if receipt.len() > MAX_RECEIPT_SIZE {
            return Err(RPCError::protocol("ReturnReceipt receipt too long to set"));
        }

        Ok(Self {
            receipt: receipt.to_vec(),
        })
    }
    pub fn validate(&mut self, _validate_context: &RPCValidateContext) -> Result<(), RPCError> {
        Ok(())
    }

    pub fn decode(
        reader: &veilid_capnp::operation_return_receipt::Reader,
    ) -> Result<RPCOperationReturnReceipt, RPCError> {
        let rr = reader.get_receipt().map_err(RPCError::protocol)?;
        RPCOperationReturnReceipt::new(rr)
    }
    pub fn encode(
        &self,
        builder: &mut veilid_capnp::operation_return_receipt::Builder,
    ) -> Result<(), RPCError> {
        builder.set_receipt(&self.receipt);
        Ok(())
    }

    pub fn receipt(&self) -> &[u8] {
        &self.receipt
    }

    pub fn destructure(self) -> Vec<u8> {
        self.receipt
    }
}
