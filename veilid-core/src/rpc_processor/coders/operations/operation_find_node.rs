use super::*;

const MAX_FIND_NODE_A_PEERS_LEN: usize = 20;

#[derive(Debug, Clone)]
pub struct RPCOperationFindNodeQ {
    node_id: TypedKey,
    capabilities: Vec<Capability>,
}

impl RPCOperationFindNodeQ {
    pub fn new(node_id: TypedKey, capabilities: Vec<Capability>) -> Self {
        Self {
            node_id,
            capabilities,
        }
    }
    pub fn validate(&mut self, _validate_context: &RPCValidateContext) -> Result<(), RPCError> {
        Ok(())
    }

    // pub fn node_id(&self) -> &TypedKey {
    //     &self.node_id
    // }
    // pub fn capabilities(&self) -> &[Capability] {
    //     &self.capabilities
    // }

    pub fn destructure(self) -> (TypedKey, Vec<Capability>) {
        (self.node_id, self.capabilities)
    }

    pub fn decode(reader: &veilid_capnp::operation_find_node_q::Reader) -> Result<Self, RPCError> {
        let ni_reader = reader.get_node_id().map_err(RPCError::protocol)?;
        let node_id = decode_typed_key(&ni_reader)?;
        let cap_reader = reader
            .reborrow()
            .get_capabilities()
            .map_err(RPCError::protocol)?;
        if cap_reader.len() as usize > MAX_CAPABILITIES {
            return Err(RPCError::protocol("too many capabilities"));
        }
        let capabilities = cap_reader
            .as_slice()
            .map(|s| s.iter().map(|x| FourCC::from(x.to_be_bytes())).collect())
            .unwrap_or_default();

        Ok(Self {
            node_id,
            capabilities,
        })
    }
    pub fn encode(
        &self,
        builder: &mut veilid_capnp::operation_find_node_q::Builder,
    ) -> Result<(), RPCError> {
        let mut ni_builder = builder.reborrow().init_node_id();
        encode_typed_key(&self.node_id, &mut ni_builder);

        let mut cap_builder = builder
            .reborrow()
            .init_capabilities(self.capabilities.len() as u32);
        if let Some(s) = cap_builder.as_slice() {
            let capvec: Vec<u32> = self
                .capabilities
                .iter()
                .map(|x| u32::from_be_bytes(x.0))
                .collect();

            s.clone_from_slice(&capvec);
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct RPCOperationFindNodeA {
    peers: Vec<PeerInfo>,
}

impl RPCOperationFindNodeA {
    pub fn new(peers: Vec<PeerInfo>) -> Result<Self, RPCError> {
        if peers.len() > MAX_FIND_NODE_A_PEERS_LEN {
            return Err(RPCError::protocol(
                "encoded find node peers length too long",
            ));
        }

        Ok(Self { peers })
    }

    pub fn validate(&mut self, validate_context: &RPCValidateContext) -> Result<(), RPCError> {
        PeerInfo::validate_vec(&mut self.peers, validate_context.crypto.clone());
        Ok(())
    }

    // pub fn peers(&self) -> &[PeerInfo] {
    //     &self.peers
    // }

    pub fn destructure(self) -> Vec<PeerInfo> {
        self.peers
    }

    pub fn decode(
        reader: &veilid_capnp::operation_find_node_a::Reader,
    ) -> Result<RPCOperationFindNodeA, RPCError> {
        let peers_reader = reader.get_peers().map_err(RPCError::protocol)?;

        if peers_reader.len() as usize > MAX_FIND_NODE_A_PEERS_LEN {
            return Err(RPCError::protocol(
                "decoded find node peers length too long",
            ));
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

        Ok(Self { peers })
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
