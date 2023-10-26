use super::*;

const MAX_FIND_BLOCK_A_DATA_LEN: usize = 32768;
const MAX_FIND_BLOCK_A_SUPPLIERS_LEN: usize = 10;
const MAX_FIND_BLOCK_A_PEERS_LEN: usize = 10;

#[derive(Debug, Clone)]
pub(in crate::rpc_processor) struct RPCOperationFindBlockQ {
    block_id: TypedKey,
}

impl RPCOperationFindBlockQ {
    pub fn new(block_id: TypedKey) -> Self {
        Self { block_id }
    }
    pub fn validate(&mut self, _validate_context: &RPCValidateContext) -> Result<(), RPCError> {
        Ok(())
    }

    pub fn block_id(&self) -> TypedKey {
        self.block_id
    }

    pub fn destructure(self) -> TypedKey {
        self.block_id
    }

    pub fn decode(
        reader: &veilid_capnp::operation_find_block_q::Reader,
    ) -> Result<RPCOperationFindBlockQ, RPCError> {
        let bi_reader = reader.get_block_id().map_err(RPCError::protocol)?;
        let block_id = decode_typed_key(&bi_reader)?;

        Ok(Self { block_id })
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
pub(in crate::rpc_processor) struct RPCOperationFindBlockA {
    data: Vec<u8>,
    suppliers: Vec<PeerInfo>,
    peers: Vec<PeerInfo>,
}

impl RPCOperationFindBlockA {
    pub fn new(
        data: Vec<u8>,
        suppliers: Vec<PeerInfo>,
        peers: Vec<PeerInfo>,
    ) -> Result<Self, RPCError> {
        if data.len() > MAX_FIND_BLOCK_A_DATA_LEN {
            return Err(RPCError::protocol("find block data length too long"));
        }
        if suppliers.len() > MAX_FIND_BLOCK_A_SUPPLIERS_LEN {
            return Err(RPCError::protocol("find block suppliers length too long"));
        }
        if peers.len() > MAX_FIND_BLOCK_A_PEERS_LEN {
            return Err(RPCError::protocol("find block peers length too long"));
        }

        Ok(Self {
            data,
            suppliers,
            peers,
        })
    }
    pub fn validate(&mut self, validate_context: &RPCValidateContext) -> Result<(), RPCError> {
        PeerInfo::validate_vec(&mut self.suppliers, validate_context.crypto.clone());
        PeerInfo::validate_vec(&mut self.peers, validate_context.crypto.clone());
        Ok(())
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
    pub fn suppliers(&self) -> &[PeerInfo] {
        &self.suppliers
    }
    pub fn peers(&self) -> &[PeerInfo] {
        &self.peers
    }

    pub fn destructure(self) -> (Vec<u8>, Vec<PeerInfo>, Vec<PeerInfo>) {
        (self.data, self.suppliers, self.peers)
    }
    pub fn decode(reader: &veilid_capnp::operation_find_block_a::Reader) -> Result<Self, RPCError> {
        let data = reader.get_data().map_err(RPCError::protocol)?;
        if data.len() > MAX_FIND_BLOCK_A_DATA_LEN {
            return Err(RPCError::protocol("find block data length too long"));
        }

        let suppliers_reader = reader.get_suppliers().map_err(RPCError::protocol)?;
        if suppliers_reader.len() as usize > MAX_FIND_BLOCK_A_SUPPLIERS_LEN {
            return Err(RPCError::protocol("find block suppliers length too long"));
        }

        let peers_reader = reader.get_peers().map_err(RPCError::protocol)?;
        if peers_reader.len() as usize > MAX_FIND_BLOCK_A_PEERS_LEN {
            return Err(RPCError::protocol("find block peers length too long"));
        }

        let mut suppliers = Vec::<PeerInfo>::with_capacity(
            suppliers_reader
                .len()
                .try_into()
                .map_err(RPCError::map_internal("too many suppliers"))?,
        );
        for s in suppliers_reader.iter() {
            let peer_info = decode_peer_info(&s)?;
            suppliers.push(peer_info);
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

        Ok(Self {
            data: data.to_vec(),
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
