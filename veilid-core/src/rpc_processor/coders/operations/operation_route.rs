use super::*;

#[derive(Clone)]
pub(in crate::rpc_processor) struct RoutedOperation {
    sequencing: Sequencing,
    signatures: Vec<Signature>,
    nonce: Nonce,
    data: Vec<u8>,
}

impl fmt::Debug for RoutedOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RoutedOperation")
            .field("sequencing", &self.sequencing)
            .field("signatures.len", &self.signatures.len())
            .field("nonce", &self.nonce)
            .field("data(len)", &self.data.len())
            .finish()
    }
}

impl RoutedOperation {
    pub fn new(sequencing: Sequencing, nonce: Nonce, data: Vec<u8>) -> Self {
        Self {
            sequencing,
            signatures: Vec::new(),
            nonce,
            data,
        }
    }
    pub fn validate(&mut self, _validate_context: &RPCValidateContext) -> Result<(), RPCError> {
        //xxx
        Ok(())
    }
    pub fn sequencing(&self) -> Sequencing {
        self.sequencing
    }
    pub fn signatures(&self) -> &[Signature] {
        &self.signatures
    }

    pub fn add_signature(&mut self, signature: Signature) {
        self.signatures.push(signature);
    }

    pub fn nonce(&self) -> &Nonce {
        &self.nonce
    }
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    // pub fn destructure(self) -> (Sequencing, Vec<Signature>, Nonce, Vec<u8>) {
    //     (self.sequencing, self.signatures, self.nonce, self.data)
    // }

    pub fn decode(reader: &veilid_capnp::routed_operation::Reader) -> Result<Self, RPCError> {
        let sigs_reader = reader.get_signatures().map_err(RPCError::protocol)?;
        let mut signatures = Vec::<Signature>::with_capacity(
            sigs_reader
                .len()
                .try_into()
                .map_err(RPCError::map_internal("too many signatures"))?,
        );
        for s in sigs_reader.iter() {
            let sig = decode_signature512(&s);
            signatures.push(sig);
        }

        let sequencing = decode_sequencing(reader.get_sequencing().map_err(RPCError::protocol)?);
        let n_reader = reader.get_nonce().map_err(RPCError::protocol)?;
        let nonce = decode_nonce(&n_reader);
        let data = reader.get_data().map_err(RPCError::protocol)?;

        Ok(Self {
            sequencing,
            signatures,
            nonce,
            data: data.to_vec(),
        })
    }

    pub fn encode(
        &self,
        builder: &mut veilid_capnp::routed_operation::Builder,
    ) -> Result<(), RPCError> {
        builder
            .reborrow()
            .set_sequencing(encode_sequencing(self.sequencing));
        let mut sigs_builder = builder.reborrow().init_signatures(
            self.signatures
                .len()
                .try_into()
                .map_err(RPCError::map_internal("invalid signatures list length"))?,
        );
        for (i, sig) in self.signatures.iter().enumerate() {
            let mut sig_builder = sigs_builder.reborrow().get(i as u32);
            encode_signature512(sig, &mut sig_builder);
        }
        let mut n_builder = builder.reborrow().init_nonce();
        encode_nonce(&self.nonce, &mut n_builder);
        builder.set_data(&self.data);

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub(in crate::rpc_processor) struct RPCOperationRoute {
    safety_route: SafetyRoute,
    operation: RoutedOperation,
}

impl RPCOperationRoute {
    pub fn new(safety_route: SafetyRoute, operation: RoutedOperation) -> Self {
        Self {
            safety_route,
            operation,
        }
    }
    pub fn validate(&mut self, validate_context: &RPCValidateContext) -> Result<(), RPCError> {
        self.operation.validate(validate_context)
    }

    pub fn safety_route(&self) -> &SafetyRoute {
        &self.safety_route
    }
    // pub fn operation(&self) -> &RoutedOperation {
    //     &self.operation
    // }
    pub fn destructure(self) -> (SafetyRoute, RoutedOperation) {
        (self.safety_route, self.operation)
    }

    pub fn decode(reader: &veilid_capnp::operation_route::Reader) -> Result<Self, RPCError> {
        let sr_reader = reader.get_safety_route().map_err(RPCError::protocol)?;
        let safety_route = decode_safety_route(&sr_reader)?;

        let o_reader = reader.get_operation().map_err(RPCError::protocol)?;
        let operation = RoutedOperation::decode(&o_reader)?;

        Ok(Self {
            safety_route,
            operation,
        })
    }
    pub fn encode(
        &self,
        builder: &mut veilid_capnp::operation_route::Builder,
    ) -> Result<(), RPCError> {
        let mut sr_builder = builder.reborrow().init_safety_route();
        encode_safety_route(&self.safety_route, &mut sr_builder)?;
        let mut o_builder = builder.reborrow().init_operation();
        self.operation.encode(&mut o_builder)?;
        Ok(())
    }
}
