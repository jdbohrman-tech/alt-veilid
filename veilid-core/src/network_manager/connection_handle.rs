use super::*;

#[derive(Clone, Debug)]
pub struct ConnectionHandle {
    _id: NetworkConnectionId,
    descriptor: ConnectionDescriptor,
    channel: flume::Sender<(Option<Id>, Vec<u8>)>,
}

#[derive(Debug)]
pub enum ConnectionHandleSendResult {
    Sent,
    NotSent(Vec<u8>),
}

impl ConnectionHandle {
    pub(super) fn new(
        id: NetworkConnectionId,
        descriptor: ConnectionDescriptor,
        channel: flume::Sender<(Option<Id>, Vec<u8>)>,
    ) -> Self {
        Self {
            _id: id,
            descriptor,
            channel,
        }
    }

    // pub fn connection_id(&self) -> NetworkConnectionId {
    //     self.id
    // }

    pub fn connection_descriptor(&self) -> ConnectionDescriptor {
        self.descriptor
    }

    // #[cfg_attr(feature="verbose-tracing", instrument(level="trace", skip(self, message), fields(message.len = message.len())))]
    // pub fn send(&self, message: Vec<u8>) -> ConnectionHandleSendResult {
    //     match self.channel.send((Span::current().id(), message)) {
    //         Ok(()) => ConnectionHandleSendResult::Sent,
    //         Err(e) => ConnectionHandleSendResult::NotSent(e.0 .1),
    //     }
    // }

    #[cfg_attr(feature="verbose-tracing", instrument(level="trace", skip(self, message), fields(message.len = message.len())))]
    pub async fn send_async(&self, message: Vec<u8>) -> ConnectionHandleSendResult {
        match self
            .channel
            .send_async((Span::current().id(), message))
            .await
        {
            Ok(()) => ConnectionHandleSendResult::Sent,
            Err(e) => ConnectionHandleSendResult::NotSent(e.0 .1),
        }
    }
}

impl PartialEq for ConnectionHandle {
    fn eq(&self, other: &Self) -> bool {
        self.descriptor == other.descriptor
    }
}

impl Eq for ConnectionHandle {}
