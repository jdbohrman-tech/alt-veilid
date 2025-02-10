use super::*;

use core::pin::Pin;
use core::task::{Context, Poll};
use futures_util::{stream::Stream, FutureExt};
use std::io;

/// A wrapper around [`VirtualTcpListener`] that implements [`Stream`].
///
/// [`VirtualTcpListener`]: struct@crate::VirtualTcpListener
/// [`Stream`]: trait@futures_util::stream::Stream
pub struct VirtualTcpListenerStream {
    inner: VirtualTcpListener,
    current_accept_fut: Option<SendPinBoxFuture<VirtualNetworkResult<(SocketId, SocketAddr)>>>,
}

impl fmt::Debug for VirtualTcpListenerStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VirtualTcpListenerStream")
            .field("inner", &self.inner)
            .field(
                "current_accept_fut",
                if self.current_accept_fut.is_some() {
                    &"Some(...)"
                } else {
                    &"None"
                },
            )
            .finish()
    }
}

impl VirtualTcpListenerStream {
    /// Create a new `VirtualTcpListenerStream`.
    pub fn new(listener: VirtualTcpListener) -> Self {
        Self {
            inner: listener,
            current_accept_fut: None,
        }
    }

    /// Get back the inner `VirtualTcpListener`.
    pub fn into_inner(self) -> VirtualTcpListener {
        self.inner
    }
}

impl Stream for VirtualTcpListenerStream {
    type Item = io::Result<VirtualTcpStream>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<io::Result<VirtualTcpStream>>> {
        if self.current_accept_fut.is_none() {
            let machine_id = self.inner.machine.id;
            let router_client = self.inner.machine.router_client.clone();
            let socket_id = self.inner.socket_id;

            self.current_accept_fut =
                Some(Box::pin(router_client.tcp_accept(machine_id, socket_id)));
        }
        let fut = self.current_accept_fut.as_mut().unwrap();
        fut.poll_unpin(cx).map(|v| match v {
            Ok(v) => Some(Ok(VirtualTcpStream::new(
                self.inner.machine.clone(),
                v.0,
                self.inner.local_address,
                v.1,
            ))),
            Err(e) => Some(Err(e.into())),
        })
    }
}

impl AsRef<VirtualTcpListener> for VirtualTcpListenerStream {
    fn as_ref(&self) -> &VirtualTcpListener {
        &self.inner
    }
}

impl AsMut<VirtualTcpListener> for VirtualTcpListenerStream {
    fn as_mut(&mut self) -> &mut VirtualTcpListener {
        &mut self.inner
    }
}
