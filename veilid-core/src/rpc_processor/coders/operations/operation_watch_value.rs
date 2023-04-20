use super::*;

#[derive(Debug, Clone)]
pub struct RPCOperationWatchValueQ {
    pub key: TypedKey,
    pub subkeys: Vec<ValueSubkeyRange>,
    pub expiration: u64,
    pub count: u32,
}

impl RPCOperationWatchValueQ {
    pub fn decode(
        reader: &veilid_capnp::operation_watch_value_q::Reader,
    ) -> Result<RPCOperationWatchValueQ, RPCError> {
        let k_reader = reader.get_key().map_err(RPCError::protocol)?;
        let key = decode_typed_key(&k_reader)?;

        let sk_reader = reader.get_subkeys().map_err(RPCError::protocol)?;
        let mut subkeys = Vec::<ValueSubkeyRange>::with_capacity(
            sk_reader
                .len()
                .try_into()
                .map_err(RPCError::map_protocol("too many subkey ranges"))?,
        );
        for skr in sk_reader.iter() {
            let vskr = (skr.get_start(), skr.get_end());
            if vskr.0 > vskr.1 {
                return Err(RPCError::protocol("invalid subkey range"));
            }
            if let Some(lvskr) = subkeys.last() {
                if lvskr.1 >= vskr.0 {
                    return Err(RPCError::protocol(
                        "subkey range out of order or not merged",
                    ));
                }
            }
            subkeys.push(vskr);
        }

        let expiration = reader.get_expiration();
        let count = reader.get_count();

        Ok(RPCOperationWatchValueQ {
            key,
            subkeys,
            expiration,
            count,
        })
    }
    pub fn encode(
        &self,
        builder: &mut veilid_capnp::operation_watch_value_q::Builder,
    ) -> Result<(), RPCError> {
        let mut k_builder = builder.reborrow().init_key();
        encode_typed_key(&self.key, &mut k_builder);

        let mut sk_builder = builder.reborrow().init_subkeys(
            self.subkeys
                .len()
                .try_into()
                .map_err(RPCError::map_internal("invalid subkey range list length"))?,
        );
        for (i, skr) in self.subkeys.iter().enumerate() {
            let mut skr_builder = sk_builder.reborrow().get(i as u32);
            skr_builder.set_start(skr.0);
            skr_builder.set_end(skr.1);
        }
        builder.set_expiration(self.expiration);
        builder.set_count(self.count);
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
