use crate::*;
use rpc_processor::*;

#[derive(Debug, Clone)]
pub struct RPCOperationWatchValueQ {
    pub key: ValueKey,
}

impl RPCOperationWatchValueQ {
    pub fn decode(
        reader: &veilid_capnp::operation_watch_value_q::Reader,
    ) -> Result<RPCOperationWatchValueQ, RPCError> {
        let k_reader = reader.get_key().map_err(RPCError::protocol)?;
        let key = decode_value_key(&k_reader)?;
        Ok(RPCOperationWatchValueQ { key })
    }
    pub fn encode(
        &self,
        builder: &mut veilid_capnp::operation_watch_value_q::Builder,
    ) -> Result<(), RPCError> {
        let mut k_builder = builder.reborrow().init_key();
        encode_value_key(&self.key, &mut k_builder)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct RPCOperationWatchValueA {
    pub expiration: u64,
    pub peers: Vec<PeerInfo>,
}

impl RPCOperationWatchValueA {
    pub fn decode(
        reader: &veilid_capnp::operation_watch_value_a::Reader,
    ) -> Result<RPCOperationWatchValueA, RPCError> {
        let expiration = reader.get_expiration();
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

        Ok(RPCOperationWatchValueA { expiration, peers })
    }
    pub fn encode(
        &self,
        builder: &mut veilid_capnp::operation_watch_value_a::Builder,
    ) -> Result<(), RPCError> {
        builder.set_expiration(self.expiration);

        let mut peers_builder = builder.reborrow().init_peers(
            self.peers
                .len()
                .try_into()
                .map_err(RPCError::map_internal("invalid peers list length"))?,
        );
        for (i, peer) in self.peers.iter().enumerate() {
            let mut pi_builder = peers_builder.reborrow().get(i as u32);
            encode_peer_info(peer, &mut pi_builder)?;
        }

        Ok(())
    }
}
