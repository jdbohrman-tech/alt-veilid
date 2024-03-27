use super::*;
use crate::storage_manager::{SignedValueData, SignedValueDescriptor};

const MAX_GET_VALUE_A_PEERS_LEN: usize = 20;

#[derive(Clone)]
pub(in crate::rpc_processor) struct ValidateGetValueContext {
    pub last_descriptor: Option<SignedValueDescriptor>,
    pub subkey: ValueSubkey,
    pub vcrypto: CryptoSystemVersion,
}

impl fmt::Debug for ValidateGetValueContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ValidateGetValueContext")
            .field("last_descriptor", &self.last_descriptor)
            .field("subkey", &self.subkey)
            .field("vcrypto", &self.vcrypto.kind().to_string())
            .finish()
    }
}

#[derive(Debug, Clone)]
pub(in crate::rpc_processor) struct RPCOperationGetValueQ {
    key: TypedKey,
    subkey: ValueSubkey,
    want_descriptor: bool,
}

impl RPCOperationGetValueQ {
    pub fn new(key: TypedKey, subkey: ValueSubkey, want_descriptor: bool) -> Self {
        Self {
            key,
            subkey,
            want_descriptor,
        }
    }
    pub fn validate(&mut self, _validate_context: &RPCValidateContext) -> Result<(), RPCError> {
        Ok(())
    }

    // pub fn key(&self) -> &TypedKey {
    //     &self.key
    // }
    // pub fn subkey(&self) -> ValueSubkey {
    //     self.subkey
    // }
    // pub fn want_descriptor(&self) -> bool {
    //     self.want_descriptor
    // }
    pub fn destructure(self) -> (TypedKey, ValueSubkey, bool) {
        (self.key, self.subkey, self.want_descriptor)
    }

    pub fn decode(reader: &veilid_capnp::operation_get_value_q::Reader) -> Result<Self, RPCError> {
        let k_reader = reader.reborrow().get_key().map_err(RPCError::protocol)?;
        let key = decode_typed_key(&k_reader)?;
        let subkey = reader.reborrow().get_subkey();
        let want_descriptor = reader.reborrow().get_want_descriptor();
        Ok(Self {
            key,
            subkey,
            want_descriptor,
        })
    }
    pub fn encode(
        &self,
        builder: &mut veilid_capnp::operation_get_value_q::Builder,
    ) -> Result<(), RPCError> {
        let mut k_builder = builder.reborrow().init_key();
        encode_typed_key(&self.key, &mut k_builder);
        builder.set_subkey(self.subkey);
        builder.set_want_descriptor(self.want_descriptor);
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub(in crate::rpc_processor) struct RPCOperationGetValueA {
    value: Option<SignedValueData>,
    peers: Vec<PeerInfo>,
    descriptor: Option<SignedValueDescriptor>,
}

impl RPCOperationGetValueA {
    pub fn new(
        value: Option<SignedValueData>,
        peers: Vec<PeerInfo>,
        descriptor: Option<SignedValueDescriptor>,
    ) -> Result<Self, RPCError> {
        if peers.len() > MAX_GET_VALUE_A_PEERS_LEN {
            return Err(RPCError::protocol(
                "encoded GetValueA peers length too long",
            ));
        }
        Ok(Self {
            value,
            peers,
            descriptor,
        })
    }

    pub fn validate(&mut self, validate_context: &RPCValidateContext) -> Result<(), RPCError> {
        let question_context = validate_context
            .question_context
            .as_ref()
            .expect("GetValueA requires question context");
        let QuestionContext::GetValue(get_value_context) = question_context else {
            panic!("Wrong context type for GetValueA");
        };

        // Validate descriptor
        if let Some(descriptor) = &self.descriptor {
            // Ensure the descriptor itself validates
            descriptor
                .validate(get_value_context.vcrypto.clone())
                .map_err(RPCError::protocol)?;

            // Ensure descriptor matches last one
            if let Some(last_descriptor) = &get_value_context.last_descriptor {
                if descriptor.cmp_no_sig(last_descriptor) != cmp::Ordering::Equal {
                    return Err(RPCError::protocol(
                        "GetValue descriptor does not match last descriptor",
                    ));
                }
            }
        }

        // Ensure the value validates
        if let Some(value) = &self.value {
            // Get descriptor to validate with
            let Some(descriptor) = self
                .descriptor
                .as_ref()
                .or(get_value_context.last_descriptor.as_ref())
            else {
                return Err(RPCError::protocol(
                    "no last descriptor, requires a descriptor",
                ));
            };

            // And the signed value data
            value
                .validate(
                    descriptor.owner(),
                    get_value_context.subkey,
                    get_value_context.vcrypto.clone(),
                )
                .map_err(RPCError::protocol)?;
        }

        PeerInfo::validate_vec(&mut self.peers, validate_context.crypto.clone());
        Ok(())
    }

    // pub fn value(&self) -> Option<&SignedValueData> {
    //     self.value.as_ref()
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
        Option<SignedValueData>,
        Vec<PeerInfo>,
        Option<SignedValueDescriptor>,
    ) {
        (self.value, self.peers, self.descriptor)
    }

    pub fn decode(reader: &veilid_capnp::operation_get_value_a::Reader) -> Result<Self, RPCError> {
        let value = if reader.has_value() {
            let value_reader = reader.get_value().map_err(RPCError::protocol)?;
            let value = decode_signed_value_data(&value_reader)?;
            Some(value)
        } else {
            None
        };

        let peers_reader = reader.get_peers().map_err(RPCError::protocol)?;
        if peers_reader.len() as usize > MAX_GET_VALUE_A_PEERS_LEN {
            return Err(RPCError::protocol(
                "decoded GetValueA peers length too long",
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
            value,
            peers,
            descriptor,
        })
    }
    pub fn encode(
        &self,
        builder: &mut veilid_capnp::operation_get_value_a::Builder,
    ) -> Result<(), RPCError> {
        if let Some(value) = &self.value {
            let mut v_builder = builder.reborrow().init_value();
            encode_signed_value_data(value, &mut v_builder)?;
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
