use super::*;

#[derive(Debug, Clone)]
pub struct RPCOperationStartTunnelQ {
    pub id: TunnelId,
    pub local_mode: TunnelMode,
    pub depth: u8,
}

impl RPCOperationStartTunnelQ {
    pub fn decode(
        reader: &veilid_capnp::operation_start_tunnel_q::Reader,
    ) -> Result<RPCOperationStartTunnelQ, RPCError> {
        let id = TunnelId::new(reader.get_id());
        let local_mode = match reader.get_local_mode().map_err(RPCError::protocol)? {
            veilid_capnp::TunnelEndpointMode::Raw => TunnelMode::Raw,
            veilid_capnp::TunnelEndpointMode::Turn => TunnelMode::Turn,
        };
        let depth = reader.get_depth();

        Ok(RPCOperationStartTunnelQ {
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

#[derive(Debug, Clone)]
pub enum RPCOperationStartTunnelA {
    Partial(PartialTunnel),
    Error(TunnelError),
}

impl RPCOperationStartTunnelA {
    pub fn decode(
        reader: &veilid_capnp::operation_start_tunnel_a::Reader,
    ) -> Result<RPCOperationStartTunnelA, RPCError> {
        match reader.which().map_err(RPCError::protocol)? {
            veilid_capnp::operation_start_tunnel_a::Which::Partial(r) => {
                let pt_reader = r.map_err(RPCError::protocol)?;
                let partial_tunnel = decode_partial_tunnel(&pt_reader)?;
                Ok(RPCOperationStartTunnelA::Partial(partial_tunnel))
            }
            veilid_capnp::operation_start_tunnel_a::Which::Error(r) => {
                let tunnel_error = decode_tunnel_error(r.map_err(RPCError::protocol)?);
                Ok(RPCOperationStartTunnelA::Error(tunnel_error))
            }
        }
    }
    pub fn encode(
        &self,
        builder: &mut veilid_capnp::operation_start_tunnel_a::Builder,
    ) -> Result<(), RPCError> {
        match self {
            RPCOperationStartTunnelA::Partial(p) => {
                encode_partial_tunnel(p, &mut builder.reborrow().init_partial())?;
            }
            RPCOperationStartTunnelA::Error(e) => {
                builder.set_error(encode_tunnel_error(*e));
            }
        }

        Ok(())
    }
}
