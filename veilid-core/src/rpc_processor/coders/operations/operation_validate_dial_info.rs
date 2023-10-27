use super::*;

#[derive(Debug, Clone)]
pub(in crate::rpc_processor) struct RPCOperationValidateDialInfo {
    dial_info: DialInfo,
    receipt: Vec<u8>,
    redirect: bool,
}

impl RPCOperationValidateDialInfo {
    pub fn new(dial_info: DialInfo, receipt: Vec<u8>, redirect: bool) -> Result<Self, RPCError> {
        if receipt.len() < MIN_RECEIPT_SIZE {
            return Err(RPCError::protocol(
                "ValidateDialInfo receipt too short to set",
            ));
        }
        if receipt.len() > MAX_RECEIPT_SIZE {
            return Err(RPCError::protocol(
                "ValidateDialInfo receipt too long to set",
            ));
        }

        Ok(Self {
            dial_info,
            receipt,
            redirect,
        })
    }

    pub fn validate(&mut self, _validate_context: &RPCValidateContext) -> Result<(), RPCError> {
        Ok(())
    }
    // pub fn dial_info(&self) -> &DialInfo {
    //     &self.dial_info
    // }
    // pub fn receipt(&self) -> &[u8] {
    //     &self.receipt
    // }
    // pub fn redirect(&self) -> bool {
    //     self.redirect
    // }
    pub fn destructure(self) -> (DialInfo, Vec<u8>, bool) {
        (self.dial_info, self.receipt, self.redirect)
    }

    pub fn decode(
        reader: &veilid_capnp::operation_validate_dial_info::Reader,
    ) -> Result<Self, RPCError> {
        let di_reader = reader.get_dial_info().map_err(RPCError::protocol)?;
        let dial_info = decode_dial_info(&di_reader)?;
        let rcpt_reader = reader.get_receipt().map_err(RPCError::protocol)?;
        if rcpt_reader.len() < MIN_RECEIPT_SIZE {
            return Err(RPCError::protocol(
                "ValidateDialInfo receipt too short to set",
            ));
        }
        if rcpt_reader.len() > MAX_RECEIPT_SIZE {
            return Err(RPCError::protocol(
                "ValidateDialInfo receipt too long to set",
            ));
        }

        let receipt = rcpt_reader.to_vec();
        let redirect = reader.get_redirect();

        Ok(Self {
            dial_info,
            receipt,
            redirect,
        })
    }
    pub fn encode(
        &self,
        builder: &mut veilid_capnp::operation_validate_dial_info::Builder,
    ) -> Result<(), RPCError> {
        let mut di_builder = builder.reborrow().init_dial_info();
        encode_dial_info(&self.dial_info, &mut di_builder)?;
        builder.set_receipt(&self.receipt);
        builder.set_redirect(self.redirect);
        Ok(())
    }
}
