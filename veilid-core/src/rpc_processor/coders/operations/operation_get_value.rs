use super::*;
use crate::storage_manager::{SignedValueDescriptor, ValueDetail};

const MAX_GET_VALUE_A_PEERS_LEN: usize = 20;

#[derive(Clone)]
pub struct ValidateGetValueContext {
    last_descriptor: Option<SignedValueDescriptor>,
    subkey: ValueSubkey,
    vcrypto: CryptoSystemVersion,
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
pub struct RPCOperationGetValueQ {
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

    pub fn decode(
        reader: &veilid_capnp::operation_get_value_q::Reader,
    ) -> Result<RPCOperationGetValueQ, RPCError> {
        let k_reader = reader.reborrow().get_key().map_err(RPCError::protocol)?;
        let key = decode_typed_key(&k_reader)?;
        let subkey = reader.reborrow().get_subkey();
        let want_descriptor = reader.reborrow().get_want_descriptor();
        Ok(RPCOperationGetValueQ {
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
    pub fn key(&self) -> &TypedKey {
        &self.key
    }
    pub fn subkey(&self) -> ValueSubkey {
        self.subkey
    }
    pub fn want_descriptor(&self) -> bool {
        self.want_descriptor
    }
    pub fn destructure(self) -> (TypedKey, ValueSubkey, bool) {
        (self.key, self.subkey, self.want_descriptor)
    }
}

#[derive(Debug, Clone)]
pub enum RPCOperationGetValueA {
    Value(ValueDetail),
    Peers(Vec<PeerInfo>),
}

impl RPCOperationGetValueA {
    pub fn new_value(value: ValueDetail) -> Self {
        Self::Value(value)
    }
    pub fn new_peers(peers: Vec<PeerInfo>) -> Result<Self, RPCError> {
        if peers.len() > MAX_GET_VALUE_A_PEERS_LEN {
            return Err(RPCError::protocol("GetValueA peers length too long"));
        }
        Ok(Self::Peers(peers))
    }
    pub fn validate(&mut self, validate_context: &RPCValidateContext) -> Result<(), RPCError> {
        match self {
            RPCOperationGetValueA::Value(value_detail) => {
                let question_context = validate_context
                    .question_context
                    .as_ref()
                    .expect("GetValueA requires question context");
                let QuestionContext::GetValue(get_value_context) = question_context else {
                    panic!("Wrong context type for GetValueA");
                };
                value_detail
                    .validate(
                        get_value_context.last_descriptor.as_ref(),
                        get_value_context.subkey,
                        get_value_context.vcrypto.clone(),
                    )
                    .map_err(RPCError::protocol)
            }
            RPCOperationGetValueA::Peers(peers) => {
                PeerInfo::validate_vec(peers, validate_context.crypto.clone());
                Ok(())
            }
        }
    }

    pub fn decode(
        reader: &veilid_capnp::operation_get_value_a::Reader,
    ) -> Result<RPCOperationGetValueA, RPCError> {
        match reader.which().map_err(RPCError::protocol)? {
            veilid_capnp::operation_get_value_a::Which::Value(r) => {
                let value_detail = decode_value_detail(&r.map_err(RPCError::protocol)?)?;
                Ok(RPCOperationGetValueA::Value(value_detail))
            }
            veilid_capnp::operation_get_value_a::Which::Peers(r) => {
                let peers_reader = r.map_err(RPCError::protocol)?;
                if peers_reader.len() as usize > MAX_GET_VALUE_A_PEERS_LEN {
                    return Err(RPCError::protocol("GetValueA peers length too long"));
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

                Ok(RPCOperationGetValueA::Peers(peers))
            }
        }
    }
    pub fn encode(
        &self,
        builder: &mut veilid_capnp::operation_get_value_a::Builder,
    ) -> Result<(), RPCError> {
        match self {
            RPCOperationGetValueA::Value(value_detail) => {
                let mut d_builder = builder.reborrow().init_value();
                encode_value_detail(&value_detail, &mut d_builder)?;
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
