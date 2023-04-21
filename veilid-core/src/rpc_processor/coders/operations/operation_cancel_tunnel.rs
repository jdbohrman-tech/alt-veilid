use super::*;

#[derive(Debug, Clone)]
pub struct RPCOperationCancelTunnelQ {
    id: TunnelId,
}

impl RPCOperationCancelTunnelQ {
    pub fn new(id: TunnelId) -> Self {
        Self { id }
    }
    pub fn validate(&mut self, _validate_context: &RPCValidateContext) -> Result<(), RPCError> {
        Ok(())
    }
    pub fn decode(
        reader: &veilid_capnp::operation_cancel_tunnel_q::Reader,
    ) -> Result<RPCOperationCancelTunnelQ, RPCError> {
        let id = TunnelId::new(reader.get_id());

        Ok(RPCOperationCancelTunnelQ { id })
    }
    pub fn encode(
        &self,
        builder: &mut veilid_capnp::operation_cancel_tunnel_q::Builder,
    ) -> Result<(), RPCError> {
        builder.set_id(self.id.as_u64());

        Ok(())
    }
    pub fn id(&self) -> TunnelId {
        self.id
    }

    pub fn destructure(self) -> TunnelId {
        self.id
    }
}

#[derive(Debug, Clone)]
pub enum RPCOperationCancelTunnelA {
    Tunnel(TunnelId),
    Error(TunnelError),
}

impl RPCOperationCancelTunnelA {
    pub fn new_tunnel(id: TunnelId) -> Self {
        Self::Tunnel(id)
    }
    pub fn new_error(error: TunnelError) -> Self {
        Self::Error(error)
    }
    pub fn validate(&mut self, _validate_context: &RPCValidateContext) -> Result<(), RPCError> {
        Ok(())
    }
    pub fn decode(
        reader: &veilid_capnp::operation_cancel_tunnel_a::Reader,
    ) -> Result<RPCOperationCancelTunnelA, RPCError> {
        match reader.which().map_err(RPCError::protocol)? {
            veilid_capnp::operation_cancel_tunnel_a::Which::Tunnel(r) => {
                Ok(RPCOperationCancelTunnelA::Tunnel(TunnelId::new(r)))
            }
            veilid_capnp::operation_cancel_tunnel_a::Which::Error(r) => {
                let tunnel_error = decode_tunnel_error(r.map_err(RPCError::protocol)?);
                Ok(RPCOperationCancelTunnelA::Error(tunnel_error))
            }
        }
    }
    pub fn encode(
        &self,
        builder: &mut veilid_capnp::operation_cancel_tunnel_a::Builder,
    ) -> Result<(), RPCError> {
        match self {
            RPCOperationCancelTunnelA::Tunnel(p) => {
                builder.set_tunnel(p.as_u64());
            }
            RPCOperationCancelTunnelA::Error(e) => {
                builder.set_error(encode_tunnel_error(*e));
            }
        }

        Ok(())
    }
}
