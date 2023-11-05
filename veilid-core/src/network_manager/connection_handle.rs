use super::*;

#[derive(Clone, Debug)]
pub struct ConnectionHandle {
    connection_id: NetworkConnectionId,
    flow: Flow,
    channel: flume::Sender<(Option<Id>, Vec<u8>)>,
}

#[derive(Debug)]
pub enum ConnectionHandleSendResult {
    Sent,
    NotSent(Vec<u8>),
}

impl ConnectionHandle {
    pub(super) fn new(
        connection_id: NetworkConnectionId,
        flow: Flow,
        channel: flume::Sender<(Option<Id>, Vec<u8>)>,
    ) -> Self {
        Self {
            connection_id,
            flow,
            channel,
        }
    }

    #[allow(dead_code)]
    pub fn connection_id(&self) -> NetworkConnectionId {
        self.connection_id
    }

    #[allow(dead_code)]
    pub fn flow(&self) -> Flow {
        self.flow
    }

    pub fn unique_flow(&self) -> UniqueFlow {
        UniqueFlow {
            flow: self.flow,
            connection_id: Some(self.connection_id),
        }
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
        self.connection_id == other.connection_id && self.flow == other.flow
    }
}

impl Eq for ConnectionHandle {}
