use super::*;

#[derive(Debug, Clone)]
pub(in crate::rpc_processor) struct RPCQuestion {
    respond_to: RespondTo,
    detail: RPCQuestionDetail,
}

impl RPCQuestion {
    pub fn new(respond_to: RespondTo, detail: RPCQuestionDetail) -> Self {
        Self { respond_to, detail }
    }
    pub fn validate(&mut self, validate_context: &RPCValidateContext) -> Result<(), RPCError> {
        self.respond_to.validate(validate_context.crypto.clone())?;
        self.detail.validate(validate_context)
    }
    pub fn respond_to(&self) -> &RespondTo {
        &self.respond_to
    }
    pub fn detail(&self) -> &RPCQuestionDetail {
        &self.detail
    }
    pub fn desc(&self) -> &'static str {
        self.detail.desc()
    }
    pub fn destructure(self) -> (RespondTo, RPCQuestionDetail) {
        (self.respond_to, self.detail)
    }
    pub fn decode(reader: &veilid_capnp::question::Reader) -> Result<RPCQuestion, RPCError> {
        let rt_reader = reader.get_respond_to();
        let respond_to = RespondTo::decode(&rt_reader)?;
        let d_reader = reader.get_detail();
        let detail = RPCQuestionDetail::decode(&d_reader)?;
        Ok(RPCQuestion { respond_to, detail })
    }
    pub fn encode(&self, builder: &mut veilid_capnp::question::Builder) -> Result<(), RPCError> {
        self.respond_to
            .encode(&mut builder.reborrow().init_respond_to())?;
        self.detail.encode(&mut builder.reborrow().init_detail())?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub(in crate::rpc_processor) enum RPCQuestionDetail {
    StatusQ(Box<RPCOperationStatusQ>),
    FindNodeQ(Box<RPCOperationFindNodeQ>),
    AppCallQ(Box<RPCOperationAppCallQ>),
    GetValueQ(Box<RPCOperationGetValueQ>),
    SetValueQ(Box<RPCOperationSetValueQ>),
    WatchValueQ(Box<RPCOperationWatchValueQ>),
    #[cfg(feature = "unstable-blockstore")]
    SupplyBlockQ(Box<RPCOperationSupplyBlockQ>),
    #[cfg(feature = "unstable-blockstore")]
    FindBlockQ(Box<RPCOperationFindBlockQ>),
    #[cfg(feature = "unstable-tunnels")]
    StartTunnelQ(Box<RPCOperationStartTunnelQ>),
    #[cfg(feature = "unstable-tunnels")]
    CompleteTunnelQ(Box<RPCOperationCompleteTunnelQ>),
    #[cfg(feature = "unstable-tunnels")]
    CancelTunnelQ(Box<RPCOperationCancelTunnelQ>),
}

impl RPCQuestionDetail {
    pub fn desc(&self) -> &'static str {
        match self {
            RPCQuestionDetail::StatusQ(_) => "StatusQ",
            RPCQuestionDetail::FindNodeQ(_) => "FindNodeQ",
            RPCQuestionDetail::AppCallQ(_) => "AppCallQ",
            RPCQuestionDetail::GetValueQ(_) => "GetValueQ",
            RPCQuestionDetail::SetValueQ(_) => "SetValueQ",
            RPCQuestionDetail::WatchValueQ(_) => "WatchValueQ",
            #[cfg(feature = "unstable-blockstore")]
            RPCQuestionDetail::SupplyBlockQ(_) => "SupplyBlockQ",
            #[cfg(feature = "unstable-blockstore")]
            RPCQuestionDetail::FindBlockQ(_) => "FindBlockQ",
            #[cfg(feature = "unstable-tunnels")]
            RPCQuestionDetail::StartTunnelQ(_) => "StartTunnelQ",
            #[cfg(feature = "unstable-tunnels")]
            RPCQuestionDetail::CompleteTunnelQ(_) => "CompleteTunnelQ",
            #[cfg(feature = "unstable-tunnels")]
            RPCQuestionDetail::CancelTunnelQ(_) => "CancelTunnelQ",
        }
    }
    pub fn validate(&mut self, validate_context: &RPCValidateContext) -> Result<(), RPCError> {
        match self {
            RPCQuestionDetail::StatusQ(r) => r.validate(validate_context),
            RPCQuestionDetail::FindNodeQ(r) => r.validate(validate_context),
            RPCQuestionDetail::AppCallQ(r) => r.validate(validate_context),
            RPCQuestionDetail::GetValueQ(r) => r.validate(validate_context),
            RPCQuestionDetail::SetValueQ(r) => r.validate(validate_context),
            RPCQuestionDetail::WatchValueQ(r) => r.validate(validate_context),
            #[cfg(feature = "unstable-blockstore")]
            RPCQuestionDetail::SupplyBlockQ(r) => r.validate(validate_context),
            #[cfg(feature = "unstable-blockstore")]
            RPCQuestionDetail::FindBlockQ(r) => r.validate(validate_context),
            #[cfg(feature = "unstable-tunnels")]
            RPCQuestionDetail::StartTunnelQ(r) => r.validate(validate_context),
            #[cfg(feature = "unstable-tunnels")]
            RPCQuestionDetail::CompleteTunnelQ(r) => r.validate(validate_context),
            #[cfg(feature = "unstable-tunnels")]
            RPCQuestionDetail::CancelTunnelQ(r) => r.validate(validate_context),
        }
    }

    pub fn decode(
        reader: &veilid_capnp::question::detail::Reader,
    ) -> Result<RPCQuestionDetail, RPCError> {
        let which_reader = reader.which().map_err(RPCError::protocol)?;
        let out = match which_reader {
            veilid_capnp::question::detail::StatusQ(r) => {
                let op_reader = r.map_err(RPCError::protocol)?;
                let out = RPCOperationStatusQ::decode(&op_reader)?;
                RPCQuestionDetail::StatusQ(Box::new(out))
            }
            veilid_capnp::question::detail::FindNodeQ(r) => {
                let op_reader = r.map_err(RPCError::protocol)?;
                let out = RPCOperationFindNodeQ::decode(&op_reader)?;
                RPCQuestionDetail::FindNodeQ(Box::new(out))
            }
            veilid_capnp::question::detail::Which::AppCallQ(r) => {
                let op_reader = r.map_err(RPCError::protocol)?;
                let out = RPCOperationAppCallQ::decode(&op_reader)?;
                RPCQuestionDetail::AppCallQ(Box::new(out))
            }
            veilid_capnp::question::detail::GetValueQ(r) => {
                let op_reader = r.map_err(RPCError::protocol)?;
                let out = RPCOperationGetValueQ::decode(&op_reader)?;
                RPCQuestionDetail::GetValueQ(Box::new(out))
            }
            veilid_capnp::question::detail::SetValueQ(r) => {
                let op_reader = r.map_err(RPCError::protocol)?;
                let out = RPCOperationSetValueQ::decode(&op_reader)?;
                RPCQuestionDetail::SetValueQ(Box::new(out))
            }
            veilid_capnp::question::detail::WatchValueQ(r) => {
                let op_reader = r.map_err(RPCError::protocol)?;
                let out = RPCOperationWatchValueQ::decode(&op_reader)?;
                RPCQuestionDetail::WatchValueQ(Box::new(out))
            }
            #[cfg(feature = "unstable-blockstore")]
            veilid_capnp::question::detail::SupplyBlockQ(r) => {
                let op_reader = r.map_err(RPCError::protocol)?;
                let out = RPCOperationSupplyBlockQ::decode(&op_reader)?;
                RPCQuestionDetail::SupplyBlockQ(Box::new(out))
            }
            #[cfg(feature = "unstable-blockstore")]
            veilid_capnp::question::detail::FindBlockQ(r) => {
                let op_reader = r.map_err(RPCError::protocol)?;
                let out = RPCOperationFindBlockQ::decode(&op_reader)?;
                RPCQuestionDetail::FindBlockQ(Box::new(out))
            }
            #[cfg(feature = "unstable-tunnels")]
            veilid_capnp::question::detail::StartTunnelQ(r) => {
                let op_reader = r.map_err(RPCError::protocol)?;
                let out = RPCOperationStartTunnelQ::decode(&op_reader)?;
                RPCQuestionDetail::StartTunnelQ(Box::new(out))
            }
            #[cfg(feature = "unstable-tunnels")]
            veilid_capnp::question::detail::CompleteTunnelQ(r) => {
                let op_reader = r.map_err(RPCError::protocol)?;
                let out = RPCOperationCompleteTunnelQ::decode(&op_reader)?;
                RPCQuestionDetail::CompleteTunnelQ(Box::new(out))
            }
            #[cfg(feature = "unstable-tunnels")]
            veilid_capnp::question::detail::CancelTunnelQ(r) => {
                let op_reader = r.map_err(RPCError::protocol)?;
                let out = RPCOperationCancelTunnelQ::decode(&op_reader)?;
                RPCQuestionDetail::CancelTunnelQ(Box::new(out))
            }
        };
        Ok(out)
    }
    pub fn encode(
        &self,
        builder: &mut veilid_capnp::question::detail::Builder,
    ) -> Result<(), RPCError> {
        match self {
            RPCQuestionDetail::StatusQ(d) => d.encode(&mut builder.reborrow().init_status_q()),
            RPCQuestionDetail::FindNodeQ(d) => d.encode(&mut builder.reborrow().init_find_node_q()),
            RPCQuestionDetail::AppCallQ(d) => d.encode(&mut builder.reborrow().init_app_call_q()),
            RPCQuestionDetail::GetValueQ(d) => d.encode(&mut builder.reborrow().init_get_value_q()),
            RPCQuestionDetail::SetValueQ(d) => d.encode(&mut builder.reborrow().init_set_value_q()),
            RPCQuestionDetail::WatchValueQ(d) => {
                d.encode(&mut builder.reborrow().init_watch_value_q())
            }
            #[cfg(feature = "unstable-blockstore")]
            RPCQuestionDetail::SupplyBlockQ(d) => {
                d.encode(&mut builder.reborrow().init_supply_block_q())
            }
            #[cfg(feature = "unstable-blockstore")]
            RPCQuestionDetail::FindBlockQ(d) => {
                d.encode(&mut builder.reborrow().init_find_block_q())
            }
            #[cfg(feature = "unstable-tunnels")]
            RPCQuestionDetail::StartTunnelQ(d) => {
                d.encode(&mut builder.reborrow().init_start_tunnel_q())
            }
            #[cfg(feature = "unstable-tunnels")]
            RPCQuestionDetail::CompleteTunnelQ(d) => {
                d.encode(&mut builder.reborrow().init_complete_tunnel_q())
            }
            #[cfg(feature = "unstable-tunnels")]
            RPCQuestionDetail::CancelTunnelQ(d) => {
                d.encode(&mut builder.reborrow().init_cancel_tunnel_q())
            }
        }
    }
}
