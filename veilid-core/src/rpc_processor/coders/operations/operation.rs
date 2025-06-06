use super::*;

#[derive(Debug, Clone)]
pub(in crate::rpc_processor) enum RPCOperationKind {
    Question(Box<RPCQuestion>),
    Statement(Box<RPCStatement>),
    Answer(Box<RPCAnswer>),
}

impl RPCOperationKind {
    pub fn desc(&self) -> &'static str {
        match self {
            RPCOperationKind::Question(q) => q.desc(),
            RPCOperationKind::Statement(s) => s.desc(),
            RPCOperationKind::Answer(a) => a.desc(),
        }
    }

    pub fn validate(&mut self, validate_context: &RPCValidateContext) -> Result<(), RPCError> {
        match self {
            RPCOperationKind::Question(r) => r.validate(validate_context),
            RPCOperationKind::Statement(r) => r.validate(validate_context),
            RPCOperationKind::Answer(r) => r.validate(validate_context),
        }
    }

    pub fn decode(
        decode_context: &RPCDecodeContext,
        kind_reader: &veilid_capnp::operation::kind::Reader,
    ) -> Result<Self, RPCError> {
        let which_reader = kind_reader.which().map_err(RPCError::protocol)?;
        let out = match which_reader {
            veilid_capnp::operation::kind::Which::Question(r) => {
                let q_reader = r.map_err(RPCError::protocol)?;
                let out = RPCQuestion::decode(decode_context, &q_reader)?;
                RPCOperationKind::Question(Box::new(out))
            }
            veilid_capnp::operation::kind::Which::Statement(r) => {
                let q_reader = r.map_err(RPCError::protocol)?;
                let out = RPCStatement::decode(decode_context, &q_reader)?;
                RPCOperationKind::Statement(Box::new(out))
            }
            veilid_capnp::operation::kind::Which::Answer(r) => {
                let q_reader = r.map_err(RPCError::protocol)?;
                let out = RPCAnswer::decode(decode_context, &q_reader)?;
                RPCOperationKind::Answer(Box::new(out))
            }
        };

        Ok(out)
    }

    pub fn encode(
        &self,
        builder: &mut veilid_capnp::operation::kind::Builder,
    ) -> Result<(), RPCError> {
        match self {
            RPCOperationKind::Question(k) => k.encode(&mut builder.reborrow().init_question()),
            RPCOperationKind::Statement(k) => k.encode(&mut builder.reborrow().init_statement()),
            RPCOperationKind::Answer(k) => k.encode(&mut builder.reborrow().init_answer()),
        }
    }
}

#[derive(Debug, Clone)]
pub(in crate::rpc_processor) struct RPCOperation {
    op_id: OperationId,
    sender_peer_info: SenderPeerInfo,
    kind: RPCOperationKind,
}

impl RPCOperation {
    pub fn new_question(question: RPCQuestion, sender_peer_info: SenderPeerInfo) -> Self {
        Self {
            op_id: OperationId::new(get_random_u64()),
            sender_peer_info,
            kind: RPCOperationKind::Question(Box::new(question)),
        }
    }
    pub fn new_statement(statement: RPCStatement, sender_peer_info: SenderPeerInfo) -> Self {
        Self {
            op_id: OperationId::new(get_random_u64()),
            sender_peer_info,
            kind: RPCOperationKind::Statement(Box::new(statement)),
        }
    }

    pub fn new_answer(
        request: &RPCOperation,
        answer: RPCAnswer,
        sender_peer_info: SenderPeerInfo,
    ) -> Self {
        Self {
            op_id: request.op_id,
            sender_peer_info,
            kind: RPCOperationKind::Answer(Box::new(answer)),
        }
    }

    pub fn validate(&mut self, validate_context: &RPCValidateContext) -> Result<(), RPCError> {
        // Validate sender peer info
        if let Some(sender_peer_info) = &self.sender_peer_info.opt_peer_info {
            let crypto = validate_context.crypto();
            sender_peer_info
                .validate(&crypto)
                .map_err(RPCError::protocol)?;
        }

        // Validate operation kind
        self.kind.validate(validate_context)
    }

    pub fn op_id(&self) -> OperationId {
        self.op_id
    }

    pub fn sender_peer_info(&self) -> SenderPeerInfo {
        self.sender_peer_info.clone()
    }

    pub fn kind(&self) -> &RPCOperationKind {
        &self.kind
    }

    pub fn destructure(self) -> (OperationId, SenderPeerInfo, RPCOperationKind) {
        (self.op_id, self.sender_peer_info, self.kind)
    }

    pub fn decode(
        decode_context: &RPCDecodeContext,
        operation_reader: &veilid_capnp::operation::Reader,
    ) -> Result<Self, RPCError> {
        let op_id = OperationId::new(operation_reader.get_op_id());

        let opt_peer_info = if operation_reader.has_sender_peer_info() {
            let pi_reader = operation_reader
                .get_sender_peer_info()
                .map_err(RPCError::protocol)?;
            let pi = Arc::new(decode_peer_info(decode_context, &pi_reader)?);
            Some(pi)
        } else {
            None
        };

        let target_node_info_ts = Timestamp::new(operation_reader.get_target_node_info_ts());

        let kind_reader = operation_reader.get_kind();
        let kind = RPCOperationKind::decode(decode_context, &kind_reader)?;

        Ok(RPCOperation {
            op_id,
            sender_peer_info: SenderPeerInfo {
                opt_peer_info,
                target_node_info_ts,
            },
            kind,
        })
    }

    pub fn encode(&self, builder: &mut veilid_capnp::operation::Builder) -> Result<(), RPCError> {
        builder.set_op_id(self.op_id.as_u64());
        if let Some(sender_peer_info) = &self.sender_peer_info.opt_peer_info {
            let mut pi_builder = builder.reborrow().init_sender_peer_info();
            encode_peer_info(sender_peer_info, &mut pi_builder)?;
        }
        builder.set_target_node_info_ts(self.sender_peer_info.target_node_info_ts.as_u64());
        let mut k_builder = builder.reborrow().init_kind();
        self.kind.encode(&mut k_builder)?;
        Ok(())
    }
}
