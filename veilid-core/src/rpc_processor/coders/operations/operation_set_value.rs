use super::*;

#[derive(Debug, Clone)]
pub struct RPCOperationSetValueQ {
    pub key: ValueKey,
    pub value: ValueData,
}

impl RPCOperationSetValueQ {
    pub fn decode(
        reader: &veilid_capnp::operation_set_value_q::Reader,
    ) -> Result<RPCOperationSetValueQ, RPCError> {
        let k_reader = reader.get_key().map_err(RPCError::protocol)?;
        let key = decode_value_key(&k_reader)?;
        let v_reader = reader.get_value().map_err(RPCError::protocol)?;
        let value = decode_value_data(&v_reader)?;
        Ok(RPCOperationSetValueQ { key, value })
    }
    pub fn encode(
        &self,
        builder: &mut veilid_capnp::operation_set_value_q::Builder,
    ) -> Result<(), RPCError> {
        let mut k_builder = builder.reborrow().init_key();
        encode_value_key(&self.key, &mut k_builder)?;
        let mut v_builder = builder.reborrow().init_value();
        encode_value_data(&self.value, &mut v_builder)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum RPCOperationSetValueA {
    Data(ValueData),
    Peers(Vec<PeerInfo>),
}

impl RPCOperationSetValueA {
    pub fn decode(
        reader: &veilid_capnp::operation_set_value_a::Reader,
    ) -> Result<RPCOperationSetValueA, RPCError> {
        match reader.which().map_err(RPCError::protocol)? {
            veilid_capnp::operation_set_value_a::Which::Data(r) => {
                let data = decode_value_data(&r.map_err(RPCError::protocol)?)?;
                Ok(RPCOperationSetValueA::Data(data))
            }
            veilid_capnp::operation_set_value_a::Which::Peers(r) => {
                let peers_reader = r.map_err(RPCError::protocol)?;
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

                Ok(RPCOperationSetValueA::Peers(peers))
            }
        }
    }
    pub fn encode(
        &self,
        builder: &mut veilid_capnp::operation_set_value_a::Builder,
    ) -> Result<(), RPCError> {
        match self {
            RPCOperationSetValueA::Data(data) => {
                let mut d_builder = builder.reborrow().init_data();
                encode_value_data(&data, &mut d_builder)?;
            }
            RPCOperationSetValueA::Peers(peers) => {
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
