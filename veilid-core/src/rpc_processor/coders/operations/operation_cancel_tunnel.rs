use super::*;

#[derive(Debug, Clone)]
pub struct RPCOperationCancelTunnelQ {
    pub id: TunnelId,
}

impl RPCOperationCancelTunnelQ {
    pub fn decode(
        reader: &veilid_capnp::operation_cancel_tunnel_q::Reader,
    ) -> Result<RPCOperationCancelTunnelQ, RPCError> {
        let id = reader.get_id();

        Ok(RPCOperationCancelTunnelQ { id })
    }
    pub fn encode(
        &self,
        builder: &mut veilid_capnp::operation_cancel_tunnel_q::Builder,
    ) -> Result<(), RPCError> {
        builder.set_id(self.id);

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum RPCOperationCancelTunnelA {
    Tunnel(TunnelId),
    Error(TunnelError),
}

impl RPCOperationCancelTunnelA {
    pub fn decode(
        reader: &veilid_capnp::operation_cancel_tunnel_a::Reader,
    ) -> Result<RPCOperationCancelTunnelA, RPCError> {
        match reader.which().map_err(RPCError::protocol)? {
            veilid_capnp::operation_cancel_tunnel_a::Which::Tunnel(r) => {
                Ok(RPCOperationCancelTunnelA::Tunnel(r))
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
                builder.set_tunnel(*p);
            }
            RPCOperationCancelTunnelA::Error(e) => {
                builder.set_error(encode_tunnel_error(*e));
            }
        }

        Ok(())
    }
}
