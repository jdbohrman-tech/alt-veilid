use super::*;
use crate::storage_manager::{SignedValueData, SignedValueDescriptor};

const MAX_SET_VALUE_A_PEERS_LEN: usize = 20;

#[derive(Clone)]
pub(in crate::rpc_processor) struct ValidateSetValueContext {
    pub descriptor: SignedValueDescriptor,
    pub subkey: ValueSubkey,
    pub vcrypto: CryptoSystemVersion,
}

impl fmt::Debug for ValidateSetValueContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ValidateSetValueContext")
            .field("descriptor", &self.descriptor)
            .field("subkey", &self.subkey)
            .field("vcrypto", &self.vcrypto.kind().to_string())
            .finish()
    }
}

#[derive(Debug, Clone)]
pub(in crate::rpc_processor) struct RPCOperationSetValueQ {
    key: TypedKey,
    subkey: ValueSubkey,
    value: SignedValueData,
    descriptor: Option<SignedValueDescriptor>,
}

impl RPCOperationSetValueQ {
    pub fn new(
        key: TypedKey,
        subkey: ValueSubkey,
        value: SignedValueData,
        descriptor: Option<SignedValueDescriptor>,
    ) -> Self {
        Self {
            key,
            subkey,
            value,
            descriptor,
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

    // pub fn value(&self) -> &SignedValueData {
    //     &self.value
    // }

    // pub fn descriptor(&self) -> Option<&SignedValueDescriptor> {
    //     self.descriptor.as_ref()
    // }
    pub fn destructure(
        self,
    ) -> (
        TypedKey,
        ValueSubkey,
        SignedValueData,
        Option<SignedValueDescriptor>,
    ) {
        (self.key, self.subkey, self.value, self.descriptor)
    }

    pub fn decode(reader: &veilid_capnp::operation_set_value_q::Reader) -> Result<Self, RPCError> {
        let k_reader = reader.get_key().map_err(RPCError::protocol)?;
        let key = decode_typed_key(&k_reader)?;
        let subkey = reader.get_subkey();
        let v_reader = reader.get_value().map_err(RPCError::protocol)?;
        let value = decode_signed_value_data(&v_reader)?;
        let descriptor = if reader.has_descriptor() {
            let d_reader = reader.get_descriptor().map_err(RPCError::protocol)?;
            let descriptor = decode_signed_value_descriptor(&d_reader)?;
            Some(descriptor)
        } else {
            None
        };
        Ok(Self {
            key,
            subkey,
            value,
            descriptor,
        })
    }
    pub fn encode(
        &self,
        builder: &mut veilid_capnp::operation_set_value_q::Builder,
    ) -> Result<(), RPCError> {
        let mut k_builder = builder.reborrow().init_key();
        encode_typed_key(&self.key, &mut k_builder);
        builder.set_subkey(self.subkey);
        let mut v_builder = builder.reborrow().init_value();
        encode_signed_value_data(&self.value, &mut v_builder)?;
        if let Some(descriptor) = &self.descriptor {
            let mut d_builder = builder.reborrow().init_descriptor();
            encode_signed_value_descriptor(descriptor, &mut d_builder)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub(in crate::rpc_processor) struct RPCOperationSetValueA {
    set: bool,
    value: Option<SignedValueData>,
    peers: Vec<PeerInfo>,
}

impl RPCOperationSetValueA {
    pub fn new(
        set: bool,
        value: Option<SignedValueData>,
        peers: Vec<PeerInfo>,
    ) -> Result<Self, RPCError> {
        if peers.len() > MAX_SET_VALUE_A_PEERS_LEN {
            return Err(RPCError::protocol(
                "encoded SetValueA peers length too long",
            ));
        }
        Ok(Self { set, value, peers })
    }

    pub fn validate(&mut self, validate_context: &RPCValidateContext) -> Result<(), RPCError> {
        let question_context = validate_context
            .question_context
            .as_ref()
            .expect("SetValueA requires question context");
        let QuestionContext::SetValue(set_value_context) = question_context else {
            panic!("Wrong context type for SetValueA");
        };

        if let Some(value) = &self.value {
            // Ensure the descriptor itself validates
            set_value_context
                .descriptor
                .validate(set_value_context.vcrypto.clone())
                .map_err(RPCError::protocol)?;

            // And the signed value data
            value
                .validate(
                    set_value_context.descriptor.owner(),
                    set_value_context.subkey,
                    set_value_context.vcrypto.clone(),
                )
                .map_err(RPCError::protocol)?;
        }

        PeerInfo::validate_vec(&mut self.peers, validate_context.crypto.clone());
        Ok(())
    }

    // pub fn set(&self) -> bool {
    //     self.set
    // }
    // pub fn value(&self) -> Option<&SignedValueData> {
    //     self.value.as_ref()
    // }
    // pub fn peers(&self) -> &[PeerInfo] {
    //     &self.peers
    // }
    pub fn destructure(self) -> (bool, Option<SignedValueData>, Vec<PeerInfo>) {
        (self.set, self.value, self.peers)
    }

    pub fn decode(reader: &veilid_capnp::operation_set_value_a::Reader) -> Result<Self, RPCError> {
        let set = reader.get_set();
        let value = if reader.has_value() {
            let v_reader = reader.get_value().map_err(RPCError::protocol)?;
            let value = decode_signed_value_data(&v_reader)?;
            Some(value)
        } else {
            None
        };
        let peers_reader = reader.get_peers().map_err(RPCError::protocol)?;
        if peers_reader.len() as usize > MAX_SET_VALUE_A_PEERS_LEN {
            return Err(RPCError::protocol(
                "decoded SetValueA peers length too long",
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

        Ok(Self { set, value, peers })
    }
    pub fn encode(
        &self,
        builder: &mut veilid_capnp::operation_set_value_a::Builder,
    ) -> Result<(), RPCError> {
        builder.set_set(self.set);

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

        Ok(())
    }
}
