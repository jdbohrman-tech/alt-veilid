use super::*;

#[derive(Debug, Clone)]
pub struct RPCOperationStatusQ {
    node_status: Option<NodeStatus>,
}

impl RPCOperationStatusQ {
    pub fn new(node_status: Option<NodeStatus>) -> Self {
        Self { node_status }
    }
    pub fn validate(&mut self, _validate_context: &RPCValidateContext) -> Result<(), RPCError> {
        Ok(())
    }

    // pub fn node_status(&self) -> Option<&NodeStatus> {
    //     self.node_status.as_ref()
    // }
    pub fn destructure(self) -> Option<NodeStatus> {
        self.node_status
    }

    pub fn decode(reader: &veilid_capnp::operation_status_q::Reader) -> Result<Self, RPCError> {
        let node_status = if reader.has_node_status() {
            let ns_reader = reader.get_node_status().map_err(RPCError::protocol)?;
            let node_status = decode_node_status(&ns_reader)?;
            Some(node_status)
        } else {
            None
        };
        Ok(Self { node_status })
    }
    pub fn encode(
        &self,
        builder: &mut veilid_capnp::operation_status_q::Builder,
    ) -> Result<(), RPCError> {
        if let Some(ns) = &self.node_status {
            let mut ns_builder = builder.reborrow().init_node_status();
            encode_node_status(ns, &mut ns_builder)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct RPCOperationStatusA {
    node_status: Option<NodeStatus>,
    sender_info: Option<SenderInfo>,
}

impl RPCOperationStatusA {
    pub fn new(node_status: Option<NodeStatus>, sender_info: Option<SenderInfo>) -> Self {
        Self {
            node_status,
            sender_info,
        }
    }

    pub fn validate(&mut self, _validate_context: &RPCValidateContext) -> Result<(), RPCError> {
        Ok(())
    }

    // pub fn node_status(&self) -> Option<&NodeStatus> {
    //     self.node_status.as_ref()
    // }
    // pub fn sender_info(&self) -> Option<&SenderInfo> {
    //     self.sender_info.as_ref()
    // }
    pub fn destructure(self) -> (Option<NodeStatus>, Option<SenderInfo>) {
        (self.node_status, self.sender_info)
    }

    pub fn decode(reader: &veilid_capnp::operation_status_a::Reader) -> Result<Self, RPCError> {
        let node_status = if reader.has_node_status() {
            let ns_reader = reader.get_node_status().map_err(RPCError::protocol)?;
            let node_status = decode_node_status(&ns_reader)?;
            Some(node_status)
        } else {
            None
        };

        let sender_info = if reader.has_sender_info() {
            let si_reader = reader.get_sender_info().map_err(RPCError::protocol)?;
            let sender_info = decode_sender_info(&si_reader)?;
            Some(sender_info)
        } else {
            None
        };

        Ok(Self {
            node_status,
            sender_info,
        })
    }
    pub fn encode(
        &self,
        builder: &mut veilid_capnp::operation_status_a::Builder,
    ) -> Result<(), RPCError> {
        if let Some(ns) = &self.node_status {
            let mut ns_builder = builder.reborrow().init_node_status();
            encode_node_status(ns, &mut ns_builder)?;
        }
        if let Some(si) = &self.sender_info {
            let mut si_builder = builder.reborrow().init_sender_info();
            encode_sender_info(si, &mut si_builder)?;
        }
        Ok(())
    }
}
