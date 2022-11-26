use super::*;

#[derive(Debug, Clone)]
pub struct RPCStatement {
    detail: RPCStatementDetail,
}

impl RPCStatement {
    pub fn new(detail: RPCStatementDetail) -> Self {
        Self { detail }
    }
    pub fn detail(&self) -> &RPCStatementDetail {
        &self.detail
    }
    pub fn into_detail(self) -> RPCStatementDetail {
        self.detail
    }
    pub fn desc(&self) -> &'static str {
        self.detail.desc()
    }
    pub fn decode(
        reader: &veilid_capnp::statement::Reader,
        opt_sender_node_id: Option<&DHTKey>,
    ) -> Result<RPCStatement, RPCError> {
        let d_reader = reader.get_detail();
        let detail = RPCStatementDetail::decode(&d_reader, opt_sender_node_id)?;
        Ok(RPCStatement { detail })
    }
    pub fn encode(&self, builder: &mut veilid_capnp::statement::Builder) -> Result<(), RPCError> {
        self.detail.encode(&mut builder.reborrow().init_detail())?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum RPCStatementDetail {
    ValidateDialInfo(RPCOperationValidateDialInfo),
    Route(RPCOperationRoute),
    NodeInfoUpdate(RPCOperationNodeInfoUpdate),
    ValueChanged(RPCOperationValueChanged),
    Signal(RPCOperationSignal),
    ReturnReceipt(RPCOperationReturnReceipt),
    AppMessage(RPCOperationAppMessage),
}

impl RPCStatementDetail {
    pub fn desc(&self) -> &'static str {
        match self {
            RPCStatementDetail::ValidateDialInfo(_) => "ValidateDialInfo",
            RPCStatementDetail::Route(_) => "Route",
            RPCStatementDetail::NodeInfoUpdate(_) => "NodeInfoUpdate",
            RPCStatementDetail::ValueChanged(_) => "ValueChanged",
            RPCStatementDetail::Signal(_) => "Signal",
            RPCStatementDetail::ReturnReceipt(_) => "ReturnReceipt",
            RPCStatementDetail::AppMessage(_) => "AppMessage",
        }
    }
    pub fn decode(
        reader: &veilid_capnp::statement::detail::Reader,
        opt_sender_node_id: Option<&DHTKey>,
    ) -> Result<RPCStatementDetail, RPCError> {
        let which_reader = reader.which().map_err(RPCError::protocol)?;
        let out = match which_reader {
            veilid_capnp::statement::detail::ValidateDialInfo(r) => {
                let op_reader = r.map_err(RPCError::protocol)?;
                let out = RPCOperationValidateDialInfo::decode(&op_reader)?;
                RPCStatementDetail::ValidateDialInfo(out)
            }
            veilid_capnp::statement::detail::Route(r) => {
                let op_reader = r.map_err(RPCError::protocol)?;
                let out = RPCOperationRoute::decode(&op_reader)?;
                RPCStatementDetail::Route(out)
            }
            veilid_capnp::statement::detail::NodeInfoUpdate(r) => {
                let op_reader = r.map_err(RPCError::protocol)?;
                let out = RPCOperationNodeInfoUpdate::decode(&op_reader, opt_sender_node_id)?;
                RPCStatementDetail::NodeInfoUpdate(out)
            }
            veilid_capnp::statement::detail::ValueChanged(r) => {
                let op_reader = r.map_err(RPCError::protocol)?;
                let out = RPCOperationValueChanged::decode(&op_reader)?;
                RPCStatementDetail::ValueChanged(out)
            }
            veilid_capnp::statement::detail::Signal(r) => {
                let op_reader = r.map_err(RPCError::protocol)?;
                let out = RPCOperationSignal::decode(&op_reader)?;
                RPCStatementDetail::Signal(out)
            }
            veilid_capnp::statement::detail::ReturnReceipt(r) => {
                let op_reader = r.map_err(RPCError::protocol)?;
                let out = RPCOperationReturnReceipt::decode(&op_reader)?;
                RPCStatementDetail::ReturnReceipt(out)
            }
            veilid_capnp::statement::detail::AppMessage(r) => {
                let op_reader = r.map_err(RPCError::protocol)?;
                let out = RPCOperationAppMessage::decode(&op_reader)?;
                RPCStatementDetail::AppMessage(out)
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
            RPCStatementDetail::NodeInfoUpdate(d) => {
                d.encode(&mut builder.reborrow().init_node_info_update())
            }
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
