use super::*;

#[cfg(feature = "unstable-tunnels")]
#[derive(Debug, Clone)]
pub(in crate::rpc_processor) struct RPCOperationStartTunnelQ {
    id: TunnelId,
    local_mode: TunnelMode,
    depth: u8,
}

impl RPCOperationStartTunnelQ {
    pub fn new(id: TunnelId, local_mode: TunnelMode, depth: u8) -> Self {
        Self {
            id,
            local_mode,
            depth,
        }
    }

    pub fn validate(&mut self, _validate_context: &RPCValidateContext) -> Result<(), RPCError> {
        Ok(())
    }

    pub fn id(&self) -> TunnelId {
        self.id
    }
    pub fn local_mode(&self) -> TunnelMode {
        self.local_mode
    }
    pub fn depth(&self) -> u8 {
        self.depth
    }
    pub fn destructure(self) -> (TunnelId, TunnelMode, u8) {
        (self.id, self.local_mode, self.depth)
    }

    pub fn decode(
        reader: &veilid_capnp::operation_start_tunnel_q::Reader,
    ) -> Result<Self, RPCError> {
        let id = TunnelId::new(reader.get_id());
        let local_mode = match reader.get_local_mode().map_err(RPCError::protocol)? {
            veilid_capnp::TunnelEndpointMode::Raw => TunnelMode::Raw,
            veilid_capnp::TunnelEndpointMode::Turn => TunnelMode::Turn,
        };
        let depth = reader.get_depth();

        Ok(Self {
            id,
            local_mode,
            depth,
        })
    }
    pub fn encode(
        &self,
        builder: &mut veilid_capnp::operation_start_tunnel_q::Builder,
    ) -> Result<(), RPCError> {
        builder.set_id(self.id.as_u64());
        builder.set_local_mode(match self.local_mode {
            TunnelMode::Raw => veilid_capnp::TunnelEndpointMode::Raw,
            TunnelMode::Turn => veilid_capnp::TunnelEndpointMode::Turn,
        });
        builder.set_depth(self.depth);

        Ok(())
    }
}

#[cfg(feature = "unstable-tunnels")]
#[derive(Debug, Clone)]
pub(in crate::rpc_processor) enum RPCOperationStartTunnelA {
    Partial(PartialTunnel),
    Error(TunnelError),
}

impl RPCOperationStartTunnelA {
    pub fn new_partial(partial_tunnel: PartialTunnel) -> Self {
        Self::Partial(partial_tunnel)
    }
    pub fn new_error(tunnel_error: TunnelError) -> Self {
        Self::Error(tunnel_error)
    }
    pub fn validate(&mut self, _validate_context: &RPCValidateContext) -> Result<(), RPCError> {
        Ok(())
    }

    pub fn decode(
        reader: &veilid_capnp::operation_start_tunnel_a::Reader,
    ) -> Result<Self, RPCError> {
        match reader.which().map_err(RPCError::protocol)? {
            veilid_capnp::operation_start_tunnel_a::Which::Partial(r) => {
                let pt_reader = r.map_err(RPCError::protocol)?;
                let partial_tunnel = decode_partial_tunnel(&pt_reader)?;
                Ok(Self::Partial(partial_tunnel))
            }
            veilid_capnp::operation_start_tunnel_a::Which::Error(r) => {
                let tunnel_error = decode_tunnel_error(r.map_err(RPCError::protocol)?);
                Ok(Self::Error(tunnel_error))
            }
        }
    }
    pub fn encode(
        &self,
        builder: &mut veilid_capnp::operation_start_tunnel_a::Builder,
    ) -> Result<(), RPCError> {
        match self {
            Self::Partial(p) => {
                encode_partial_tunnel(p, &mut builder.reborrow().init_partial())?;
            }
            Self::Error(e) => {
                builder.set_error(encode_tunnel_error(*e));
            }
        }

        Ok(())
    }
}
