use super::*;

#[derive(Debug, Clone)]
pub struct RPCOperationGetValueQ {
    pub key: TypedKey,
    pub subkey: ValueSubkey,
}

impl RPCOperationGetValueQ {
    pub fn decode(
        reader: &veilid_capnp::operation_get_value_q::Reader,
    ) -> Result<RPCOperationGetValueQ, RPCError> {
        let k_reader = reader.get_key().map_err(RPCError::protocol)?;
        let key = decode_typed_key(&k_reader)?;
        let subkey = reader.get_subkey();
        Ok(RPCOperationGetValueQ { key, subkey })
    }
    pub fn encode(
        &self,
        builder: &mut veilid_capnp::operation_get_value_q::Builder,
    ) -> Result<(), RPCError> {
        let mut k_builder = builder.reborrow().init_key();
        encode_typed_key(&self.key, &mut k_builder);
        builder.set_subkey(self.subkey);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum RPCOperationGetValueA {
    Data(ValueData),
    Peers(Vec<PeerInfo>),
}

impl RPCOperationGetValueA {
    pub fn decode(
        reader: &veilid_capnp::operation_get_value_a::Reader,
        crypto: Crypto,
    ) -> Result<RPCOperationGetValueA, RPCError> {
        match reader.which().map_err(RPCError::protocol)? {
            veilid_capnp::operation_get_value_a::Which::Data(r) => {
                let data = decode_value_data(&r.map_err(RPCError::protocol)?)?;
                Ok(RPCOperationGetValueA::Data(data))
            }
            veilid_capnp::operation_get_value_a::Which::Peers(r) => {
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

                Ok(RPCOperationGetValueA::Peers(peers))
            }
        }
    }
    pub fn encode(
        &self,
        builder: &mut veilid_capnp::operation_get_value_a::Builder,
    ) -> Result<(), RPCError> {
        match self {
            RPCOperationGetValueA::Data(data) => {
                let mut d_builder = builder.reborrow().init_data();
                encode_value_data(&data, &mut d_builder)?;
            }
            RPCOperationGetValueA::Peers(peers) => {
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
