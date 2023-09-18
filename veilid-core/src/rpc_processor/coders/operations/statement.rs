use super::*;

#[derive(Debug, Clone)]
pub struct RPCStatement {
    detail: RPCStatementDetail,
}

impl RPCStatement {
    pub fn new(detail: RPCStatementDetail) -> Self {
        Self { detail }
    }
    pub fn validate(&mut self, validate_context: &RPCValidateContext) -> Result<(), RPCError> {
        self.detail.validate(validate_context)
    }
    pub fn detail(&self) -> &RPCStatementDetail {
        &self.detail
    }
    pub fn desc(&self) -> &'static str {
        self.detail.desc()
    }
    pub fn destructure(self) -> RPCStatementDetail {
        self.detail
    }
    pub fn decode(reader: &veilid_capnp::statement::Reader) -> Result<RPCStatement, RPCError> {
        let d_reader = reader.get_detail();
        let detail = RPCStatementDetail::decode(&d_reader)?;
        Ok(RPCStatement { detail })
    }
    pub fn encode(&self, builder: &mut veilid_capnp::statement::Builder) -> Result<(), RPCError> {
        self.detail.encode(&mut builder.reborrow().init_detail())?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum RPCStatementDetail {
    ValidateDialInfo(Box<RPCOperationValidateDialInfo>),
    Route(Box<RPCOperationRoute>),
    ValueChanged(Box<RPCOperationValueChanged>),
    Signal(Box<RPCOperationSignal>),
    ReturnReceipt(Box<RPCOperationReturnReceipt>),
    AppMessage(Box<RPCOperationAppMessage>),
}

impl RPCStatementDetail {
    pub fn desc(&self) -> &'static str {
        match self {
            RPCStatementDetail::ValidateDialInfo(_) => "ValidateDialInfo",
            RPCStatementDetail::Route(_) => "Route",
            RPCStatementDetail::ValueChanged(_) => "ValueChanged",
            RPCStatementDetail::Signal(_) => "Signal",
            RPCStatementDetail::ReturnReceipt(_) => "ReturnReceipt",
            RPCStatementDetail::AppMessage(_) => "AppMessage",
        }
    }
    pub fn validate(&mut self, validate_context: &RPCValidateContext) -> Result<(), RPCError> {
        match self {
            RPCStatementDetail::ValidateDialInfo(r) => r.validate(validate_context),
            RPCStatementDetail::Route(r) => r.validate(validate_context),
            RPCStatementDetail::ValueChanged(r) => r.validate(validate_context),
            RPCStatementDetail::Signal(r) => r.validate(validate_context),
            RPCStatementDetail::ReturnReceipt(r) => r.validate(validate_context),
            RPCStatementDetail::AppMessage(r) => r.validate(validate_context),
        }
    }
    pub fn decode(
        reader: &veilid_capnp::statement::detail::Reader,
    ) -> Result<RPCStatementDetail, RPCError> {
        let which_reader = reader.which().map_err(RPCError::protocol)?;
        let out = match which_reader {
            veilid_capnp::statement::detail::ValidateDialInfo(r) => {
                let op_reader = r.map_err(RPCError::protocol)?;
                let out = RPCOperationValidateDialInfo::decode(&op_reader)?;
                RPCStatementDetail::ValidateDialInfo(Box::new(out))
            }
            veilid_capnp::statement::detail::Route(r) => {
                let op_reader = r.map_err(RPCError::protocol)?;
                let out = RPCOperationRoute::decode(&op_reader)?;
                RPCStatementDetail::Route(Box::new(out))
            }
            veilid_capnp::statement::detail::ValueChanged(r) => {
                let op_reader = r.map_err(RPCError::protocol)?;
                let out = RPCOperationValueChanged::decode(&op_reader)?;
                RPCStatementDetail::ValueChanged(Box::new(out))
            }
            veilid_capnp::statement::detail::Signal(r) => {
                let op_reader = r.map_err(RPCError::protocol)?;
                let out = RPCOperationSignal::decode(&op_reader)?;
                RPCStatementDetail::Signal(Box::new(out))
            }
            veilid_capnp::statement::detail::ReturnReceipt(r) => {
                let op_reader = r.map_err(RPCError::protocol)?;
                let out = RPCOperationReturnReceipt::decode(&op_reader)?;
                RPCStatementDetail::ReturnReceipt(Box::new(out))
            }
            veilid_capnp::statement::detail::AppMessage(r) => {
                let op_reader = r.map_err(RPCError::protocol)?;
                let out = RPCOperationAppMessage::decode(&op_reader)?;
                RPCStatementDetail::AppMessage(Box::new(out))
            }
        };
        Ok(out)
    }
    pub fn encode(
        &self,
        builder: &mut veilid_capnp::statement::detail::Builder,
    ) -> Result<(), RPCError> {
        match self {
            RPCStatementDetail::ValidateDialInfo(d) => {
                d.encode(&mut builder.reborrow().init_validate_dial_info())
            }
            RPCStatementDetail::Route(d) => d.encode(&mut builder.reborrow().init_route()),
            RPCStatementDetail::ValueChanged(d) => {
                d.encode(&mut builder.reborrow().init_value_changed())
            }
            RPCStatementDetail::Signal(d) => d.encode(&mut builder.reborrow().init_signal()),
            RPCStatementDetail::ReturnReceipt(d) => {
                d.encode(&mut builder.reborrow().init_return_receipt())
            }
            RPCStatementDetail::AppMessage(d) => {
                d.encode(&mut builder.reborrow().init_app_message())
            }
        }
    }
}
