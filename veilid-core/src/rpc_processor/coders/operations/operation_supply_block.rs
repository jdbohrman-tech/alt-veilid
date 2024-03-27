use super::*;

const MAX_SUPPLY_BLOCK_A_PEERS_LEN: usize = 20;

#[derive(Debug, Clone)]
pub(in crate::rpc_processor) struct RPCOperationSupplyBlockQ {
    block_id: TypedKey,
}

impl RPCOperationSupplyBlockQ {
    pub fn new(block_id: TypedKey) -> Self {
        Self { block_id }
    }
    pub fn validate(&mut self, _validate_context: &RPCValidateContext) -> Result<(), RPCError> {
        Ok(())
    }

    pub fn block_id(&self) -> &TypedKey {
        &self.block_id
    }

    pub fn destructure(self) -> TypedKey {
        self.block_id
    }

    pub fn decode(
        reader: &veilid_capnp::operation_supply_block_q::Reader,
    ) -> Result<Self, RPCError> {
        let bi_reader = reader.get_block_id().map_err(RPCError::protocol)?;
        let block_id = decode_typed_key(&bi_reader)?;

        Ok(Self { block_id })
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

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub(in crate::rpc_processor) struct RPCOperationSupplyBlockA {
    expiration: u64,
    peers: Vec<PeerInfo>,
}

impl RPCOperationSupplyBlockA {
    pub fn new(expiration: u64, peers: Vec<PeerInfo>) -> Result<Self, RPCError> {
        if peers.len() > MAX_SUPPLY_BLOCK_A_PEERS_LEN {
            return Err(RPCError::protocol("SupplyBlockA peers length too long"));
        }
        Ok(Self { expiration, peers })
    }
    pub fn validate(&mut self, validate_context: &RPCValidateContext) -> Result<(), RPCError> {
        PeerInfo::validate_vec(&mut self.peers, validate_context.crypto.clone());
        Ok(())
    }
    pub fn expiration(&self) -> u64 {
        self.expiration
    }
    pub fn peers(&self) -> &[PeerInfo] {
        &self.peers
    }
    pub fn destructure(self) -> (u64, Vec<PeerInfo>) {
        (self.expiration, self.peers)
    }

    pub fn decode(
        reader: &veilid_capnp::operation_supply_block_a::Reader,
    ) -> Result<Self, RPCError> {
        let expiration = reader.get_expiration();

        let peers_reader = reader.get_peers().map_err(RPCError::protocol)?;
        if peers_reader.len() as usize > MAX_SUPPLY_BLOCK_A_PEERS_LEN {
            return Err(RPCError::protocol("SupplyBlockA peers length too long"));
        }
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

        Ok(Self { expiration, peers })
    }
    pub fn encode(
        &self,
        builder: &mut veilid_capnp::operation_supply_block_a::Builder,
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
