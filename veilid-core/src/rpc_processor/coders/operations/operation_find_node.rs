use crate::*;
use rpc_processor::*;

#[derive(Debug, Clone)]
pub struct RPCOperationFindNodeQ {
    pub node_id: DHTKey,
}

impl RPCOperationFindNodeQ {
    pub fn decode(
        reader: &veilid_capnp::operation_find_node_q::Reader,
    ) -> Result<RPCOperationFindNodeQ, RPCError> {
        let ni_reader = reader.get_node_id().map_err(RPCError::protocol)?;
        let node_id = decode_public_key(&ni_reader);
        Ok(RPCOperationFindNodeQ { node_id })
    }
    pub fn encode(
        &self,
        builder: &mut veilid_capnp::operation_find_node_q::Builder,
    ) -> Result<(), RPCError> {
        let mut ni_builder = builder.reborrow().init_node_id();
        encode_public_key(&self.node_id, &mut ni_builder)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct RPCOperationFindNodeA {
    pub peers: Vec<PeerInfo>,
}

impl RPCOperationFindNodeA {
    pub fn decode(
        reader: &veilid_capnp::operation_find_node_a::Reader,
    ) -> Result<RPCOperationFindNodeA, RPCError> {
        let peers_reader = reader.get_peers().map_err(RPCError::protocol)?;
        let mut peers = Vec::<PeerInfo>::with_capacity(
            peers_reader
                .len()
                .try_into()
                .map_err(RPCError::map_internal("too many peers"))?,
        );
        for p in peers_reader.iter() {
            let peer_info = decode_peer_info(&p)?;
            peers.push(peer_info);
        }

        Ok(RPCOperationFindNodeA { peers })
    }
    pub fn encode(
        &self,
        builder: &mut veilid_capnp::operation_find_node_a::Builder,
    ) -> Result<(), RPCError> {
        let mut peers_builder = builder.reborrow().init_peers(
            self.peers
                .len()
                .try_into()
                .map_err(RPCError::map_internal("invalid closest nodes list length"))?,
        );
        for (i, peer) in self.peers.iter().enumerate() {
            let mut pi_builder = peers_builder.reborrow().get(i as u32);
            encode_peer_info(peer, &mut pi_builder)?;
        }
        Ok(())
    }
}
