use super::*;

#[derive(Debug, Clone)]
pub struct RPCOperationSupplyBlockQ {
    pub block_id: TypedKey,
}

impl RPCOperationSupplyBlockQ {
    pub fn decode(
        reader: &veilid_capnp::operation_supply_block_q::Reader,
    ) -> Result<RPCOperationSupplyBlockQ, RPCError> {
        let bi_reader = reader.get_block_id().map_err(RPCError::protocol)?;
        let block_id = decode_typed_key(&bi_reader)?;

        Ok(RPCOperationSupplyBlockQ { block_id })
    }
    pub fn encode(
        &self,
        builder: &mut veilid_capnp::operation_supply_block_q::Builder,
    ) -> Result<(), RPCError> {
        let mut bi_builder = builder.reborrow().init_block_id();
        encode_typed_key(&self.block_id, &mut bi_builder);

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum RPCOperationSupplyBlockA {
    Expiration(u64),
    Peers(Vec<PeerInfo>),
}

impl RPCOperationSupplyBlockA {
    pub fn decode(
        reader: &veilid_capnp::operation_supply_block_a::Reader,
        crypto: Crypto,
    ) -> Result<RPCOperationSupplyBlockA, RPCError> {
        match reader.which().map_err(RPCError::protocol)? {
            veilid_capnp::operation_supply_block_a::Which::Expiration(r) => {
                Ok(RPCOperationSupplyBlockA::Expiration(r))
            }
            veilid_capnp::operation_supply_block_a::Which::Peers(r) => {
                let peers_reader = r.map_err(RPCError::protocol)?;
                let mut peers = Vec::<PeerInfo>::with_capacity(
                    peers_reader
                        .len()
                        .try_into()
                        .map_err(RPCError::map_internal("too many peers"))?,
                );
                for p in peers_reader.iter() {
                    let peer_info = decode_peer_info(&p, crypto.clone())?;
                    peers.push(peer_info);
                }

                Ok(RPCOperationSupplyBlockA::Peers(peers))
            }
        }
    }
    pub fn encode(
        &self,
        builder: &mut veilid_capnp::operation_supply_block_a::Builder,
    ) -> Result<(), RPCError> {
        match self {
            RPCOperationSupplyBlockA::Expiration(e) => {
                builder.set_expiration(*e);
            }
            RPCOperationSupplyBlockA::Peers(peers) => {
                let mut peers_builder = builder.reborrow().init_peers(
                    peers
                        .len()
                        .try_into()
                        .map_err(RPCError::map_internal("invalid peers list length"))?,
                );
                for (i, peer) in peers.iter().enumerate() {
                    let mut pi_builder = peers_builder.reborrow().get(i as u32);
                    encode_peer_info(peer, &mut pi_builder)?;
                }
            }
        }

        Ok(())
    }
}
