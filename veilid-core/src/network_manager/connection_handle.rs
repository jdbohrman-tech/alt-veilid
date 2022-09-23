use super::*;

#[derive(Clone, Debug)]
pub struct ConnectionHandle {
    id: u64,
    descriptor: ConnectionDescriptor,
    channel: flume::Sender<Vec<u8>>,
}

#[derive(Debug)]
pub enum ConnectionHandleSendResult {
    Sent,
    NotSent(Vec<u8>),
}

impl ConnectionHandle {
    pub(super) fn new(
        id: u64,
        descriptor: ConnectionDescriptor,
        channel: flume::Sender<Vec<u8>>,
    ) -> Self {
        Self {
            id,
            descriptor,
            channel,
        }
    }

    pub fn connection_id(&self) -> u64 {
        self.id
    }

    pub fn connection_descriptor(&self) -> ConnectionDescriptor {
        self.descriptor.clone()
    }

    pub fn send(&self, message: Vec<u8>) -> ConnectionHandleSendResult {
        match self.channel.send(message) {
            Ok(()) => ConnectionHandleSendResult::Sent,
            Err(e) => ConnectionHandleSendResult::NotSent(e.0),
        }
    }
    pub async fn send_async(&self, message: Vec<u8>) -> ConnectionHandleSendResult {
        match self.channel.send_async(message).await {
            Ok(()) => ConnectionHandleSendResult::Sent,
            Err(e) => ConnectionHandleSendResult::NotSent(e.0),
        }
    }
}

impl PartialEq for ConnectionHandle {
    fn eq(&self, other: &Self) -> bool {
        self.descriptor == other.descriptor
    }
}

impl Eq for ConnectionHandle {}
