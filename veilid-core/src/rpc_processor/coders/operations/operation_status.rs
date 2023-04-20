use super::*;

#[derive(Debug, Clone)]
pub struct RPCOperationStatusQ {
    pub node_status: Option<NodeStatus>,
}

impl RPCOperationStatusQ {
    pub fn validate(&self, crypto: Crypto) -> Result<(), RPCError> {
        Ok(())
    }

    pub fn decode(
        reader: &veilid_capnp::operation_status_q::Reader,
    ) -> Result<RPCOperationStatusQ, RPCError> {
        let node_status = if reader.has_node_status() {
            let ns_reader = reader.get_node_status().map_err(RPCError::protocol)?;
            let node_status = decode_node_status(&ns_reader)?;
            Some(node_status)
        } else {
            None
        };
        Ok(RPCOperationStatusQ { node_status })
    }
    pub fn encode(
        &self,
        builder: &mut veilid_capnp::operation_status_q::Builder,
    ) -> Result<(), RPCError> {
        if let Some(ns) = &self.node_status {
            let mut ns_builder = builder.reborrow().init_node_status();
            encode_node_status(&ns, &mut ns_builder)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct RPCOperationStatusA {
    pub node_status: Option<NodeStatus>,
    pub sender_info: Option<SenderInfo>,
}

impl RPCOperationStatusA {
    pub fn validate(&self, crypto: Crypto) -> Result<(), RPCError> {
        Ok(())
    }
    pub fn decode(
        reader: &veilid_capnp::operation_status_a::Reader,
    ) -> Result<RPCOperationStatusA, RPCError> {
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

        Ok(RPCOperationStatusA {
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
            encode_node_status(&ns, &mut ns_builder)?;
        }
        if let Some(si) = &self.sender_info {
            let mut si_builder = builder.reborrow().init_sender_info();
            encode_sender_info(&si, &mut si_builder)?;
        }
        Ok(())
    }
}
