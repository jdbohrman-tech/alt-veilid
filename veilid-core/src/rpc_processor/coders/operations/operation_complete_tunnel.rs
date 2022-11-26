use super::*;

#[derive(Debug, Clone)]
pub struct RPCOperationCompleteTunnelQ {
    pub id: TunnelId,
    pub local_mode: TunnelMode,
    pub depth: u8,
    pub endpoint: TunnelEndpoint,
}

impl RPCOperationCompleteTunnelQ {
    pub fn decode(
        reader: &veilid_capnp::operation_complete_tunnel_q::Reader,
    ) -> Result<RPCOperationCompleteTunnelQ, RPCError> {
        let id = reader.get_id();
        let local_mode = match reader.get_local_mode().map_err(RPCError::protocol)? {
            veilid_capnp::TunnelEndpointMode::Raw => TunnelMode::Raw,
            veilid_capnp::TunnelEndpointMode::Turn => TunnelMode::Turn,
        };
        let depth = reader.get_depth();
        let te_reader = reader.get_endpoint().map_err(RPCError::protocol)?;
        let endpoint = decode_tunnel_endpoint(&te_reader)?;

        Ok(RPCOperationCompleteTunnelQ {
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
        builder.set_id(self.id);
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

#[derive(Debug, Clone)]
pub enum RPCOperationCompleteTunnelA {
    Tunnel(FullTunnel),
    Error(TunnelError),
}

impl RPCOperationCompleteTunnelA {
    pub fn decode(
        reader: &veilid_capnp::operation_complete_tunnel_a::Reader,
    ) -> Result<RPCOperationCompleteTunnelA, RPCError> {
        match reader.which().map_err(RPCError::protocol)? {
            veilid_capnp::operation_complete_tunnel_a::Which::Tunnel(r) => {
                let ft_reader = r.map_err(RPCError::protocol)?;
                let full_tunnel = decode_full_tunnel(&ft_reader)?;
                Ok(RPCOperationCompleteTunnelA::Tunnel(full_tunnel))
            }
            veilid_capnp::operation_complete_tunnel_a::Which::Error(r) => {
                let tunnel_error = decode_tunnel_error(r.map_err(RPCError::protocol)?);
                Ok(RPCOperationCompleteTunnelA::Error(tunnel_error))
            }
        }
    }
    pub fn encode(
        &self,
        builder: &mut veilid_capnp::operation_complete_tunnel_a::Builder,
    ) -> Result<(), RPCError> {
        match self {
            RPCOperationCompleteTunnelA::Tunnel(p) => {
                encode_full_tunnel(p, &mut builder.reborrow().init_tunnel())?;
            }
            RPCOperationCompleteTunnelA::Error(e) => {
                builder.set_error(encode_tunnel_error(*e));
            }
        }

        Ok(())
    }
}
