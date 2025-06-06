use super::*;

const MAX_WATCH_VALUE_Q_SUBKEY_RANGES_LEN: usize = 512;
const MAX_WATCH_VALUE_A_PEERS_LEN: usize = 20;

#[derive(Debug, Clone)]
pub(in crate::rpc_processor) struct RPCOperationWatchValueQ {
    key: TypedRecordKey,
    subkeys: ValueSubkeyRangeSet,
    expiration: u64,
    count: u32,
    watch_id: Option<u64>,
    watcher: PublicKey,
    signature: Signature,
}

impl RPCOperationWatchValueQ {
    pub fn new(
        key: TypedRecordKey,
        subkeys: ValueSubkeyRangeSet,
        expiration: u64,
        count: u32,
        watch_id: Option<u64>,
        watcher: KeyPair,
        vcrypto: &CryptoSystemGuard<'_>,
    ) -> Result<Self, RPCError> {
        if subkeys.ranges_len() > MAX_WATCH_VALUE_Q_SUBKEY_RANGES_LEN {
            return Err(RPCError::protocol("WatchValueQ subkeys length too long"));
        }

        // Count is zero means cancelling, so there should always be a watch id in this case
        if count == 0 && watch_id.is_none() {
            return Err(RPCError::protocol("can't cancel zero watch id"));
        }

        let signature_data = Self::make_signature_data(&key, &subkeys, expiration, count, watch_id);
        let signature = vcrypto
            .sign(&watcher.key, &watcher.secret, &signature_data)
            .map_err(RPCError::protocol)?;

        Ok(Self {
            key,
            subkeys,
            expiration,
            count,
            watch_id,
            watcher: watcher.key,
            signature,
        })
    }

    // signature covers: key, subkeys, expiration, count, using watcher key
    fn make_signature_data(
        key: &TypedRecordKey,
        subkeys: &ValueSubkeyRangeSet,
        expiration: u64,
        count: u32,
        watch_id: Option<u64>,
    ) -> Vec<u8> {
        let subkeys_ranges_len = subkeys.ranges_len();

        let mut sig_data =
            Vec::with_capacity(PUBLIC_KEY_LENGTH + 4 + (subkeys_ranges_len * 8) + 8 + 8);
        sig_data.extend_from_slice(&key.kind.0);
        sig_data.extend_from_slice(&key.value.bytes);
        for sk in subkeys.ranges() {
            sig_data.extend_from_slice(&sk.start().to_le_bytes());
            sig_data.extend_from_slice(&sk.end().to_le_bytes());
        }
        sig_data.extend_from_slice(&expiration.to_le_bytes());
        sig_data.extend_from_slice(&count.to_le_bytes());
        if let Some(watch_id) = watch_id {
            sig_data.extend_from_slice(&watch_id.to_le_bytes());
        }
        sig_data
    }

    pub fn validate(&mut self, validate_context: &RPCValidateContext) -> Result<(), RPCError> {
        let crypto = validate_context.crypto();
        let Some(vcrypto) = crypto.get(self.key.kind) else {
            return Err(RPCError::protocol("unsupported cryptosystem"));
        };

        let sig_data = Self::make_signature_data(
            &self.key,
            &self.subkeys,
            self.expiration,
            self.count,
            self.watch_id,
        );
        if !vcrypto
            .verify(&self.watcher, &sig_data, &self.signature)
            .map_err(RPCError::protocol)?
        {
            return Err(RPCError::protocol("failed to validate watcher signature"));
        }

        // Count is zero means cancelling, so there should always be a watch id in this case
        if self.count == 0 && self.watch_id.is_none() {
            return Err(RPCError::protocol("can't cancel zero watch id"));
        }

        Ok(())
    }

    #[expect(dead_code)]
    pub fn key(&self) -> &TypedRecordKey {
        &self.key
    }

    #[expect(dead_code)]
    pub fn subkeys(&self) -> &ValueSubkeyRangeSet {
        &self.subkeys
    }

    #[expect(dead_code)]
    pub fn expiration(&self) -> u64 {
        self.expiration
    }

    #[expect(dead_code)]
    pub fn count(&self) -> u32 {
        self.count
    }

    #[expect(dead_code)]
    pub fn watch_id(&self) -> Option<u64> {
        self.watch_id
    }

    #[expect(dead_code)]
    pub fn watcher(&self) -> &PublicKey {
        &self.watcher
    }
    #[expect(dead_code)]
    pub fn signature(&self) -> &Signature {
        &self.signature
    }
    pub fn destructure(
        self,
    ) -> (
        TypedRecordKey,
        ValueSubkeyRangeSet,
        u64,
        u32,
        Option<u64>,
        PublicKey,
        Signature,
    ) {
        (
            self.key,
            self.subkeys,
            self.expiration,
            self.count,
            self.watch_id,
            self.watcher,
            self.signature,
        )
    }

    pub fn decode(
        _decode_context: &RPCDecodeContext,
        reader: &veilid_capnp::operation_watch_value_q::Reader,
    ) -> Result<Self, RPCError> {
        let k_reader = reader.get_key().map_err(RPCError::protocol)?;
        let key = decode_typed_record_key(&k_reader)?;

        let sk_reader = reader.get_subkeys().map_err(RPCError::protocol)?;
        if sk_reader.len() as usize > MAX_WATCH_VALUE_Q_SUBKEY_RANGES_LEN {
            return Err(RPCError::protocol("WatchValueQ too many subkey ranges"));
        }
        let mut subkeys = ValueSubkeyRangeSet::new();
        for skr in sk_reader.iter() {
            let vskr = (skr.get_start(), skr.get_end());
            if vskr.0 > vskr.1 {
                return Err(RPCError::protocol("invalid subkey range"));
            }
            if let Some(lvskr) = subkeys.last() {
                if lvskr >= vskr.0 {
                    return Err(RPCError::protocol(
                        "subkey range out of order or not merged",
                    ));
                }
            }
            subkeys.ranges_insert(vskr.0..=vskr.1);
        }

        let expiration = reader.get_expiration();
        let count = reader.get_count();
        let watch_id = if reader.get_watch_id() != 0 {
            Some(reader.get_watch_id())
        } else {
            None
        };

        let w_reader = reader.get_watcher().map_err(RPCError::protocol)?;
        let watcher = decode_key256(&w_reader);

        let s_reader = reader.get_signature().map_err(RPCError::protocol)?;
        let signature = decode_signature512(&s_reader);

        Ok(Self {
            key,
            subkeys,
            expiration,
            count,
            watch_id,
            watcher,
            signature,
        })
    }

    pub fn encode(
        &self,
        builder: &mut veilid_capnp::operation_watch_value_q::Builder,
    ) -> Result<(), RPCError> {
        let mut k_builder = builder.reborrow().init_key();
        encode_typed_record_key(&self.key, &mut k_builder);

        let mut sk_builder = builder.reborrow().init_subkeys(
            self.subkeys
                .ranges_len()
                .try_into()
                .map_err(RPCError::map_internal("invalid subkey range list length"))?,
        );
        for (i, skr) in self.subkeys.ranges().enumerate() {
            let mut skr_builder = sk_builder.reborrow().get(i as u32);
            skr_builder.set_start(*skr.start());
            skr_builder.set_end(*skr.end());
        }
        builder.set_expiration(self.expiration);
        builder.set_count(self.count);
        builder.set_watch_id(self.watch_id.unwrap_or(0u64));

        let mut w_builder = builder.reborrow().init_watcher();
        encode_key256(&self.watcher, &mut w_builder);

        let mut s_builder = builder.reborrow().init_signature();
        encode_signature512(&self.signature, &mut s_builder);

        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub(in crate::rpc_processor) struct RPCOperationWatchValueA {
    accepted: bool,
    expiration: u64,
    peers: Vec<Arc<PeerInfo>>,
    watch_id: u64,
}

impl RPCOperationWatchValueA {
    pub fn new(
        accepted: bool,
        expiration: u64,
        peers: Vec<Arc<PeerInfo>>,
        watch_id: u64,
    ) -> Result<Self, RPCError> {
        if peers.len() > MAX_WATCH_VALUE_A_PEERS_LEN {
            return Err(RPCError::protocol("WatchValueA peers length too long"));
        }
        Ok(Self {
            accepted,
            expiration,
            peers,
            watch_id,
        })
    }

    pub fn validate(&mut self, validate_context: &RPCValidateContext) -> Result<(), RPCError> {
        let crypto = validate_context.crypto();
        PeerInfo::validate_vec(&mut self.peers, &crypto);
        Ok(())
    }

    #[expect(dead_code)]
    pub fn accepted(&self) -> bool {
        self.accepted
    }
    #[expect(dead_code)]
    pub fn expiration(&self) -> u64 {
        self.expiration
    }
    #[expect(dead_code)]
    pub fn peers(&self) -> &[Arc<PeerInfo>] {
        &self.peers
    }
    #[expect(dead_code)]
    pub fn watch_id(&self) -> u64 {
        self.watch_id
    }
    pub fn destructure(self) -> (bool, u64, Vec<Arc<PeerInfo>>, u64) {
        (self.accepted, self.expiration, self.peers, self.watch_id)
    }

    pub fn decode(
        decode_context: &RPCDecodeContext,
        reader: &veilid_capnp::operation_watch_value_a::Reader,
    ) -> Result<Self, RPCError> {
        let accepted = reader.get_accepted();
        let expiration = reader.get_expiration();
        let peers_reader = reader.get_peers().map_err(RPCError::protocol)?;
        if peers_reader.len() as usize > MAX_WATCH_VALUE_A_PEERS_LEN {
            return Err(RPCError::protocol("WatchValueA peers length too long"));
        }
        let mut peers = Vec::<Arc<PeerInfo>>::with_capacity(
            peers_reader
                .len()
                .try_into()
                .map_err(RPCError::map_internal("too many peers"))?,
        );
        for p in peers_reader.iter() {
            let peer_info = Arc::new(decode_peer_info(decode_context, &p)?);
            peers.push(peer_info);
        }
        let watch_id = reader.get_watch_id();

        Ok(Self {
            accepted,
            expiration,
            peers,
            watch_id,
        })
    }
    pub fn encode(
        &self,
        builder: &mut veilid_capnp::operation_watch_value_a::Builder,
    ) -> Result<(), RPCError> {
        builder.set_accepted(self.accepted);
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
        builder.set_watch_id(self.watch_id);

        Ok(())
    }
}
