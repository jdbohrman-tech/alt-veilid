use super::*;

#[cfg(feature = "unstable-tunnels")]
#[derive(Debug, Clone)]
pub(in crate::rpc_processor) struct RPCOperationCompleteTunnelQ {
    id: TunnelId,
    local_mode: TunnelMode,
    depth: u8,
    endpoint: TunnelEndpoint,
}

impl RPCOperationCompleteTunnelQ {
    pub fn new(id: TunnelId, local_mode: TunnelMode, depth: u8, endpoint: TunnelEndpoint) -> Self {
        Self {
            id,
            local_mode,
            depth,
            endpoint,
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
    pub fn endpoint(&self) -> &TunnelEndpoint {
        &self.endpoint
    }
    pub fn destructure(self) -> (TunnelId, TunnelMode, u8, TunnelEndpoint) {
        (self.id, self.local_mode, self.depth, self.endpoint)
    }

    pub fn decode(
        reader: &veilid_capnp::operation_complete_tunnel_q::Reader,
    ) -> Result<Self, RPCError> {
        let id = TunnelId::new(reader.get_id());
        let local_mode = match reader.get_local_mode().map_err(RPCError::protocol)? {
            veilid_capnp::TunnelEndpointMode::Raw => TunnelMode::Raw,
            veilid_capnp::TunnelEndpointMode::Turn => TunnelMode::Turn,
        };
        let depth = reader.get_depth();
        let te_reader = reader.get_endpoint().map_err(RPCError::protocol)?;
        let endpoint = decode_tunnel_endpoint(&te_reader)?;

        Ok(Self {
            id,
            local_mode,
            depth,
            endpoint,
        })
    }
    pub fn encode(
        &self,
        builder: &mut veilid_capnp::operation_complete_tunnel_q::Builder,
    ) -> Result<(), RPCError> {
        builder.set_id(self.id.as_u64());
        builder.set_local_mode(match self.local_mode {
            TunnelMode::Raw => veilid_capnp::TunnelEndpointMode::Raw,
            TunnelMode::Turn => veilid_capnp::TunnelEndpointMode::Turn,
        });
        builder.set_depth(self.depth);
        let mut te_builder = builder.reborrow().init_endpoint();
        encode_tunnel_endpoint(&self.endpoint, &mut te_builder)?;

        Ok(())
    }
}

#[cfg(feature = "unstable-tunnels")]
#[derive(Debug, Clone)]
pub(in crate::rpc_processor) enum RPCOperationCompleteTunnelA {
    Tunnel(FullTunnel),
    Error(TunnelError),
}

impl RPCOperationCompleteTunnelA {
    pub fn new_tunnel(tunnel: FullTunnel) -> Self {
        Self::Tunnel(tunnel)
    }
    pub fn new_error(error: TunnelError) -> Self {
        Self::Error(error)
    }
    pub fn validate(&mut self, _validate_context: &RPCValidateContext) -> Result<(), RPCError> {
        Ok(())
    }

    pub fn decode(
        reader: &veilid_capnp::operation_complete_tunnel_a::Reader,
    ) -> Result<Self, RPCError> {
        match reader.which().map_err(RPCError::protocol)? {
            veilid_capnp::operation_complete_tunnel_a::Which::Tunnel(r) => {
                let ft_reader = r.map_err(RPCError::protocol)?;
                let full_tunnel = decode_full_tunnel(&ft_reader)?;
                Ok(Self::Tunnel(full_tunnel))
            }
            veilid_capnp::operation_complete_tunnel_a::Which::Error(r) => {
                let tunnel_error = decode_tunnel_error(r.map_err(RPCError::protocol)?);
                Ok(Self::Error(tunnel_error))
            }
        }
    }
    pub fn encode(
        &self,
        builder: &mut veilid_capnp::operation_complete_tunnel_a::Builder,
    ) -> Result<(), RPCError> {
        match self {
            Self::Tunnel(p) => {
                encode_full_tunnel(p, &mut builder.reborrow().init_tunnel())?;
            }
            Self::Error(e) => {
                builder.set_error(encode_tunnel_error(*e));
            }
        }

        Ok(())
    }
}
