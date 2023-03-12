use super::*;

#[derive(Debug, Clone)]
pub struct RPCOperationFindBlockQ {
    pub block_id: TypedKey,
}

impl RPCOperationFindBlockQ {
    pub fn decode(
        reader: &veilid_capnp::operation_find_block_q::Reader,
    ) -> Result<RPCOperationFindBlockQ, RPCError> {
        let bi_reader = reader.get_block_id().map_err(RPCError::protocol)?;
        let block_id = decode_typed_key(&bi_reader)?;

        Ok(RPCOperationFindBlockQ { block_id })
    }
    pub fn encode(
        &self,
        builder: &mut veilid_capnp::operation_find_block_q::Builder,
    ) -> Result<(), RPCError> {
        let mut bi_builder = builder.reborrow().init_block_id();
        encode_typed_key(&self.block_id, &mut bi_builder);

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct RPCOperationFindBlockA {
    pub data: Vec<u8>,
    pub suppliers: Vec<PeerInfo>,
    pub peers: Vec<PeerInfo>,
}

impl RPCOperationFindBlockA {
    pub fn decode(
        reader: &veilid_capnp::operation_find_block_a::Reader,
        crypto: Crypto,
    ) -> Result<RPCOperationFindBlockA, RPCError> {
        let data = reader.get_data().map_err(RPCError::protocol)?.to_vec();

        let suppliers_reader = reader.get_suppliers().map_err(RPCError::protocol)?;
        let mut suppliers = Vec::<PeerInfo>::with_capacity(
            suppliers_reader
                .len()
                .try_into()
                .map_err(RPCError::map_internal("too many suppliers"))?,
        );
        for s in suppliers_reader.iter() {
            let peer_info = decode_peer_info(&s, crypto.clone())?;
            suppliers.push(peer_info);
        }

        let peers_reader = reader.get_peers().map_err(RPCError::protocol)?;
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

        Ok(RPCOperationFindBlockA {
            data,
            suppliers,
            peers,
        })
    }

    pub fn encode(
        &self,
        builder: &mut veilid_capnp::operation_find_block_a::Builder,
    ) -> Result<(), RPCError> {
        builder.set_data(&self.data);

        let mut suppliers_builder = builder.reborrow().init_suppliers(
            self.suppliers
                .len()
                .try_into()
                .map_err(RPCError::map_internal("invalid suppliers list length"))?,
        );
        for (i, peer) in self.suppliers.iter().enumerate() {
            let mut pi_builder = suppliers_builder.reborrow().get(i as u32);
            encode_peer_info(peer, &mut pi_builder)?;
        }

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
