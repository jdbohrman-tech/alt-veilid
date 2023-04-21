use super::*;

const MAX_FIND_NODE_A_PEERS_LEN: usize = 20;

#[derive(Debug, Clone)]
pub struct RPCOperationFindNodeQ {
    node_id: TypedKey,
}

impl RPCOperationFindNodeQ {
    pub fn new(node_id: TypedKey) -> Self {
        Self { node_id }
    }
    pub fn validate(&mut self, _validate_context: &RPCValidateContext) -> Result<(), RPCError> {
        Ok(())
    }
    pub fn decode(
        reader: &veilid_capnp::operation_find_node_q::Reader,
    ) -> Result<RPCOperationFindNodeQ, RPCError> {
        let ni_reader = reader.get_node_id().map_err(RPCError::protocol)?;
        let node_id = decode_typed_key(&ni_reader)?;
        Ok(RPCOperationFindNodeQ { node_id })
    }
    pub fn encode(
        &self,
        builder: &mut veilid_capnp::operation_find_node_q::Builder,
    ) -> Result<(), RPCError> {
        let mut ni_builder = builder.reborrow().init_node_id();
        encode_typed_key(&self.node_id, &mut ni_builder);
        Ok(())
    }

    pub fn node_id(&self) -> &TypedKey {
        &self.node_id
    }

    pub fn destructure(self) -> TypedKey {
        self.node_id
    }
}

#[derive(Debug, Clone)]
pub struct RPCOperationFindNodeA {
    peers: Vec<PeerInfo>,
}

impl RPCOperationFindNodeA {
    pub fn new(peers: Vec<PeerInfo>) -> Result<Self, RPCError> {
        if peers.len() > MAX_FIND_NODE_A_PEERS_LEN {
            return Err(RPCError::protocol("find node peers length too long"));
        }

        Ok(Self { peers })
    }

    pub fn validate(&mut self, validate_context: &RPCValidateContext) -> Result<(), RPCError> {
        PeerInfo::validate_vec(&mut self.peers, validate_context.crypto.clone());
        Ok(())
    }
    pub fn decode(
        reader: &veilid_capnp::operation_find_node_a::Reader,
    ) -> Result<RPCOperationFindNodeA, RPCError> {
        let peers_reader = reader.get_peers().map_err(RPCError::protocol)?;

        if peers_reader.len() as usize > MAX_FIND_NODE_A_PEERS_LEN {
            return Err(RPCError::protocol("find node peers length too long"));
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

        RPCOperationFindNodeA::new(peers)
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

    pub fn peers(&self) -> &[PeerInfo] {
        &self.peers
    }

    pub fn destructure(self) -> Vec<PeerInfo> {
        self.peers
    }
}
