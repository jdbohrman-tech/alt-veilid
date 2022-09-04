use super::*;
use crate::*;
use rpc_processor::*;

#[derive(Debug, Clone)]
pub struct RPCQuestion {
    respond_to: RespondTo,
    detail: RPCQuestionDetail,
}

impl RPCQuestion {
    pub fn new(respond_to: RespondTo, detail: RPCQuestionDetail) -> Self {
        Self { respond_to, detail }
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
pub enum RPCQuestionDetail {
    StatusQ(RPCOperationStatusQ),
    FindNodeQ(RPCOperationFindNodeQ),
    GetValueQ(RPCOperationGetValueQ),
    SetValueQ(RPCOperationSetValueQ),
    WatchValueQ(RPCOperationWatchValueQ),
    SupplyBlockQ(RPCOperationSupplyBlockQ),
    FindBlockQ(RPCOperationFindBlockQ),
    StartTunnelQ(RPCOperationStartTunnelQ),
    CompleteTunnelQ(RPCOperationCompleteTunnelQ),
    CancelTunnelQ(RPCOperationCancelTunnelQ),
}

impl RPCQuestionDetail {
    pub fn desc(&self) -> &'static str {
        match self {
            RPCQuestionDetail::StatusQ(_) => "StatusQ",
            RPCQuestionDetail::FindNodeQ(_) => "FindNodeQ",
            RPCQuestionDetail::GetValueQ(_) => "GetValueQ",
            RPCQuestionDetail::SetValueQ(_) => "SetValueQ",
            RPCQuestionDetail::WatchValueQ(_) => "WatchValueQ",
            RPCQuestionDetail::SupplyBlockQ(_) => "SupplyBlockQ",
            RPCQuestionDetail::FindBlockQ(_) => "FindBlockQ",
            RPCQuestionDetail::StartTunnelQ(_) => "StartTunnelQ",
            RPCQuestionDetail::CompleteTunnelQ(_) => "CompleteTunnelQ",
            RPCQuestionDetail::CancelTunnelQ(_) => "CancelTunnelQ",
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
                RPCQuestionDetail::StatusQ(out)
            }
            veilid_capnp::question::detail::FindNodeQ(r) => {
                let op_reader = r.map_err(RPCError::protocol)?;
                let out = RPCOperationFindNodeQ::decode(&op_reader)?;
                RPCQuestionDetail::FindNodeQ(out)
            }
            veilid_capnp::question::detail::GetValueQ(r) => {
                let op_reader = r.map_err(RPCError::protocol)?;
                let out = RPCOperationGetValueQ::decode(&op_reader)?;
                RPCQuestionDetail::GetValueQ(out)
            }
            veilid_capnp::question::detail::SetValueQ(r) => {
                let op_reader = r.map_err(RPCError::protocol)?;
                let out = RPCOperationSetValueQ::decode(&op_reader)?;
                RPCQuestionDetail::SetValueQ(out)
            }
            veilid_capnp::question::detail::WatchValueQ(r) => {
                let op_reader = r.map_err(RPCError::protocol)?;
                let out = RPCOperationWatchValueQ::decode(&op_reader)?;
                RPCQuestionDetail::WatchValueQ(out)
            }
            veilid_capnp::question::detail::SupplyBlockQ(r) => {
                let op_reader = r.map_err(RPCError::protocol)?;
                let out = RPCOperationSupplyBlockQ::decode(&op_reader)?;
                RPCQuestionDetail::SupplyBlockQ(out)
            }
            veilid_capnp::question::detail::FindBlockQ(r) => {
                let op_reader = r.map_err(RPCError::protocol)?;
                let out = RPCOperationFindBlockQ::decode(&op_reader)?;
                RPCQuestionDetail::FindBlockQ(out)
            }
            veilid_capnp::question::detail::StartTunnelQ(r) => {
                let op_reader = r.map_err(RPCError::protocol)?;
                let out = RPCOperationStartTunnelQ::decode(&op_reader)?;
                RPCQuestionDetail::StartTunnelQ(out)
            }
            veilid_capnp::question::detail::CompleteTunnelQ(r) => {
                let op_reader = r.map_err(RPCError::protocol)?;
                let out = RPCOperationCompleteTunnelQ::decode(&op_reader)?;
                RPCQuestionDetail::CompleteTunnelQ(out)
            }
            veilid_capnp::question::detail::CancelTunnelQ(r) => {
                let op_reader = r.map_err(RPCError::protocol)?;
                let out = RPCOperationCancelTunnelQ::decode(&op_reader)?;
                RPCQuestionDetail::CancelTunnelQ(out)
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
            RPCQuestionDetail::GetValueQ(d) => d.encode(&mut builder.reborrow().init_get_value_q()),
            RPCQuestionDetail::SetValueQ(d) => d.encode(&mut builder.reborrow().init_set_value_q()),
            RPCQuestionDetail::WatchValueQ(d) => {
                d.encode(&mut builder.reborrow().init_watch_value_q())
            }
            RPCQuestionDetail::SupplyBlockQ(d) => {
                d.encode(&mut builder.reborrow().init_supply_block_q())
            }
            RPCQuestionDetail::FindBlockQ(d) => {
                d.encode(&mut builder.reborrow().init_find_block_q())
            }
            RPCQuestionDetail::StartTunnelQ(d) => {
                d.encode(&mut builder.reborrow().init_start_tunnel_q())
            }
            RPCQuestionDetail::CompleteTunnelQ(d) => {
                d.encode(&mut builder.reborrow().init_complete_tunnel_q())
            }
            RPCQuestionDetail::CancelTunnelQ(d) => {
                d.encode(&mut builder.reborrow().init_cancel_tunnel_q())
            }
        }
    }
}
