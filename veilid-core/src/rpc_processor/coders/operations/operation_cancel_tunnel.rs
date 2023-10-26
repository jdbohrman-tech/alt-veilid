use super::*;

#[cfg(feature = "unstable-tunnels")]
#[derive(Debug, Clone)]
pub(in crate::rpc_processor) struct RPCOperationCancelTunnelQ {
    id: TunnelId,
}

impl RPCOperationCancelTunnelQ {
    pub fn new(id: TunnelId) -> Self {
        Self { id }
    }
    pub fn validate(&mut self, _validate_context: &RPCValidateContext) -> Result<(), RPCError> {
        Ok(())
    }

    pub fn id(&self) -> TunnelId {
        self.id
    }

    pub fn destructure(self) -> TunnelId {
        self.id
    }

    pub fn decode(
        reader: &veilid_capnp::operation_cancel_tunnel_q::Reader,
    ) -> Result<Self, RPCError> {
        let id = TunnelId::new(reader.get_id());
        Ok(Self { id })
    }
    pub fn encode(
        &self,
        builder: &mut veilid_capnp::operation_cancel_tunnel_q::Builder,
    ) -> Result<(), RPCError> {
        builder.set_id(self.id.as_u64());

        Ok(())
    }
}

#[cfg(feature = "unstable-tunnels")]
#[derive(Debug, Clone)]
pub(in crate::rpc_processor) enum RPCOperationCancelTunnelA {
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
    ) -> Result<Self, RPCError> {
        match reader.which().map_err(RPCError::protocol)? {
            veilid_capnp::operation_cancel_tunnel_a::Which::Tunnel(r) => {
                Ok(Self::Tunnel(TunnelId::new(r)))
            }
            veilid_capnp::operation_cancel_tunnel_a::Which::Error(r) => {
                let tunnel_error = decode_tunnel_error(r.map_err(RPCError::protocol)?);
                Ok(Self::Error(tunnel_error))
            }
        }
    }
    pub fn encode(
        &self,
        builder: &mut veilid_capnp::operation_cancel_tunnel_a::Builder,
    ) -> Result<(), RPCError> {
        match self {
            Self::Tunnel(p) => {
                builder.set_tunnel(p.as_u64());
            }
            Self::Error(e) => {
                builder.set_error(encode_tunnel_error(*e));
            }
        }

        Ok(())
    }
}
