use super::*;

#[derive(Debug)]
pub(in crate::rpc_processor) struct MessageData {
    pub contents: Vec<u8>, // rpc messages must be a canonicalized single segment
}

impl MessageData {
    pub fn new(contents: Vec<u8>) -> Self {
        Self { contents }
    }

    pub fn get_reader(
        &self,
    ) -> Result<capnp::message::Reader<capnp::serialize::OwnedSegments>, RPCError> {
        capnp::serialize_packed::read_message(
            self.contents.as_slice(),
            capnp::message::ReaderOptions::new(),
        )
        .map_err(RPCError::protocol)
    }
}

#[derive(Debug)]
pub(in crate::rpc_processor) struct MessageEncoded {
    pub header: MessageHeader,
    pub data: MessageData,
}

#[derive(Debug)]
pub(in crate::rpc_processor) struct Message {
    pub header: MessageHeader,
    pub operation: RPCOperation,
    pub opt_sender_nr: Option<NodeRef>,
}
