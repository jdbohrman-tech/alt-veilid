use super::*;
use crate::storage_manager::SignedValueDescriptor;

const MAX_INSPECT_VALUE_Q_SUBKEY_RANGES_LEN: usize = 512;
pub(crate) const MAX_INSPECT_VALUE_A_SEQS_LEN: usize = 512;
const MAX_INSPECT_VALUE_A_PEERS_LEN: usize = 20;

#[derive(Clone)]
pub(in crate::rpc_processor) struct ValidateInspectValueContext {
    pub last_descriptor: Option<SignedValueDescriptor>,
    pub subkeys: ValueSubkeyRangeSet,
    pub vcrypto: CryptoSystemVersion,
}

impl fmt::Debug for ValidateInspectValueContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ValidateInspectValueContext")
            .field("last_descriptor", &self.last_descriptor)
            .field("vcrypto", &self.vcrypto.kind().to_string())
            .finish()
    }
}

#[derive(Debug, Clone)]
pub(in crate::rpc_processor) struct RPCOperationInspectValueQ {
    key: TypedKey,
    subkeys: ValueSubkeyRangeSet,
    want_descriptor: bool,
}

impl RPCOperationInspectValueQ {
    pub fn new(
        key: TypedKey,
        subkeys: ValueSubkeyRangeSet,
        want_descriptor: bool,
    ) -> Result<Self, RPCError> {
        Ok(Self {
            key,
            subkeys,
            want_descriptor,
        })
    }
    pub fn validate(&mut self, _validate_context: &RPCValidateContext) -> Result<(), RPCError> {
        Ok(())
    }

    // pub fn key(&self) -> &TypedKey {
    //     &self.key
    // }
    // pub fn subkeys(&self) -> &ValueSubkeyRangeSet {
    //     &self.subkeys
    // }
    // pub fn want_descriptor(&self) -> bool {
    //     self.want_descriptor
    // }
    pub fn destructure(self) -> (TypedKey, ValueSubkeyRangeSet, bool) {
        (self.key, self.subkeys, self.want_descriptor)
    }

    pub fn decode(
        reader: &veilid_capnp::operation_inspect_value_q::Reader,
    ) -> Result<Self, RPCError> {
        let k_reader = reader.reborrow().get_key().map_err(RPCError::protocol)?;
        let key = decode_typed_key(&k_reader)?;
        let sk_reader = reader.get_subkeys().map_err(RPCError::protocol)?;
        // Maximum number of ranges that can hold the maximum number of subkeys is one subkey per range
        if sk_reader.len() as usize > MAX_INSPECT_VALUE_Q_SUBKEY_RANGES_LEN {
            return Err(RPCError::protocol("InspectValueQ too many subkey ranges"));
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

        let want_descriptor = reader.reborrow().get_want_descriptor();
        Ok(Self {
            key,
            subkeys,
            want_descriptor,
        })
    }
    pub fn encode(
        &self,
        builder: &mut veilid_capnp::operation_inspect_value_q::Builder,
    ) -> Result<(), RPCError> {
        let mut k_builder = builder.reborrow().init_key();
        encode_typed_key(&self.key, &mut k_builder);

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
        builder.set_want_descriptor(self.want_descriptor);
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub(in crate::rpc_processor) struct RPCOperationInspectValueA {
    seqs: Vec<ValueSeqNum>,
    peers: Vec<PeerInfo>,
    descriptor: Option<SignedValueDescriptor>,
}

impl RPCOperationInspectValueA {
    pub fn new(
        seqs: Vec<ValueSeqNum>,
        peers: Vec<PeerInfo>,
        descriptor: Option<SignedValueDescriptor>,
    ) -> Result<Self, RPCError> {
        if seqs.len() > MAX_INSPECT_VALUE_A_SEQS_LEN {
            return Err(RPCError::protocol(
                "encoded InspectValueA seqs length too long",
            ));
        }
        if peers.len() > MAX_INSPECT_VALUE_A_PEERS_LEN {
            return Err(RPCError::protocol(
                "encoded InspectValueA peers length too long",
            ));
        }
        Ok(Self {
            seqs,
            peers,
            descriptor,
        })
    }

    pub fn validate(&mut self, validate_context: &RPCValidateContext) -> Result<(), RPCError> {
        let question_context = validate_context
            .question_context
            .as_ref()
            .expect("InspectValueA requires question context");
        let QuestionContext::InspectValue(inspect_value_context) = question_context else {
            panic!("Wrong context type for InspectValueA");
        };

        // Ensure seqs returned does not exceeed subkeys requested
        #[allow(clippy::unnecessary_cast)]
        if self.seqs.len() > inspect_value_context.subkeys.len() as usize {
            return Err(RPCError::protocol(
                "InspectValue seqs length is greater than subkeys requested",
            ));
        }

        // Validate descriptor
        if let Some(descriptor) = &self.descriptor {
            // Ensure the descriptor itself validates
            descriptor
                .validate(inspect_value_context.vcrypto.clone())
                .map_err(RPCError::protocol)?;

            // Ensure descriptor matches last one
            if let Some(last_descriptor) = &inspect_value_context.last_descriptor {
                if descriptor.cmp_no_sig(last_descriptor) != cmp::Ordering::Equal {
                    return Err(RPCError::protocol(
                        "InspectValue descriptor does not match last descriptor",
                    ));
                }
            }
        }

        PeerInfo::validate_vec(&mut self.peers, validate_context.crypto.clone());
        Ok(())
    }

    // pub fn seqs(&self) -> &[ValueSeqNum] {
    //     &self.seqs
    // }
    // pub fn peers(&self) -> &[PeerInfo] {
    //     &self.peers
    // }
    // pub fn descriptor(&self) -> Option<&SignedValueDescriptor> {
    //     self.descriptor.as_ref()
    // }
    pub fn destructure(
        self,
    ) -> (
        Vec<ValueSeqNum>,
        Vec<PeerInfo>,
        Option<SignedValueDescriptor>,
    ) {
        (self.seqs, self.peers, self.descriptor)
    }

    pub fn decode(
        reader: &veilid_capnp::operation_inspect_value_a::Reader,
    ) -> Result<Self, RPCError> {
        let seqs = if reader.has_seqs() {
            let seqs_reader = reader.get_seqs().map_err(RPCError::protocol)?;
            if seqs_reader.len() as usize > MAX_INSPECT_VALUE_A_SEQS_LEN {
                return Err(RPCError::protocol(
                    "decoded InspectValueA seqs length too long",
                ));
            }
            let Some(seqs) = seqs_reader.as_slice().map(|s| s.to_vec()) else {
                return Err(RPCError::protocol("invalid decoded InspectValueA seqs"));
            };
            seqs
        } else {
            return Err(RPCError::protocol("missing decoded InspectValueA seqs"));
        };

        let peers_reader = reader.get_peers().map_err(RPCError::protocol)?;
        if peers_reader.len() as usize > MAX_INSPECT_VALUE_A_PEERS_LEN {
            return Err(RPCError::protocol(
                "decoded InspectValueA peers length too long",
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

        let descriptor = if reader.has_descriptor() {
            let d_reader = reader.get_descriptor().map_err(RPCError::protocol)?;
            let descriptor = decode_signed_value_descriptor(&d_reader)?;
            Some(descriptor)
        } else {
            None
        };

        Ok(Self {
            seqs,
            peers,
            descriptor,
        })
    }
    pub fn encode(
        &self,
        builder: &mut veilid_capnp::operation_inspect_value_a::Builder,
    ) -> Result<(), RPCError> {
        let mut seqs_builder = builder.reborrow().init_seqs(
            self.seqs
                .len()
                .try_into()
                .map_err(RPCError::map_internal("invalid seqs list length"))?,
        );
        for (i, seq) in self.seqs.iter().enumerate() {
            seqs_builder.set(i as u32, *seq);
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

        if let Some(descriptor) = &self.descriptor {
            let mut d_builder = builder.reborrow().init_descriptor();
            encode_signed_value_descriptor(descriptor, &mut d_builder)?;
        }

        Ok(())
    }
}
