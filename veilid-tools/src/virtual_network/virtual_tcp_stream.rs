use super::*;
use futures_util::FutureExt;
use serde::*;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct VirtualTcpOptions {
    linger: Option<Duration>,
    no_delay: bool,
    only_v6: bool,
    reuse_address_port: bool,
}

pub struct VirtualTcpStream {
    machine: Machine,
    socket_id: SocketId,
    local_address: SocketAddr,
    remote_address: SocketAddr,
    current_recv_fut: Option<SendPinBoxFuture<Result<Vec<u8>, VirtualNetworkError>>>,
    current_send_fut: Option<SendPinBoxFuture<Result<usize, VirtualNetworkError>>>,
    current_tcp_shutdown_fut: Option<SendPinBoxFuture<Result<(), VirtualNetworkError>>>,
}

impl fmt::Debug for VirtualTcpStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VirtualTcpStream")
            .field("machine", &self.machine)
            .field("socket_id", &self.socket_id)
            .field("local_address", &self.local_address)
            .field("remote_address", &self.remote_address)
            .field(
                "current_recv_fut",
                if self.current_recv_fut.is_some() {
                    &"Some(...)"
                } else {
                    &"None"
                },
            )
            .field(
                "current_send_fut",
                if self.current_send_fut.is_some() {
                    &"Some(...)"
                } else {
                    &"None"
                },
            )
            .field(
                "current_close_fut",
                if self.current_tcp_shutdown_fut.is_some() {
                    &"Some(...)"
                } else {
                    &"None"
                },
            )
            .finish()
    }
}

impl VirtualTcpStream {
    //////////////////////////////////////////////////////////////////////////
    // Public Interface

    pub async fn connect(
        remote_address: SocketAddr,
        local_address: Option<SocketAddr>,
        timeout_ms: u32,
        options: VirtualTcpOptions,
    ) -> VirtualNetworkResult<Self> {
        let machine = default_machine().unwrap();
        Self::connect_with_machine(machine, remote_address, local_address, timeout_ms, options)
            .await
    }

    pub async fn connect_with_machine(
        machine: Machine,
        remote_address: SocketAddr,
        local_address: Option<SocketAddr>,
        timeout_ms: u32,
        options: VirtualTcpOptions,
    ) -> VirtualNetworkResult<Self> {
        machine
            .router_client
            .clone()
            .tcp_connect(
                machine.id,
                remote_address,
                local_address,
                timeout_ms,
                options,
            )
            .await
            .map(|(socket_id, local_address)| {
                Self::new(machine, socket_id, local_address, remote_address)
            })
    }

    pub fn local_addr(&self) -> VirtualNetworkResult<SocketAddr> {
        Ok(self.local_address)
    }

    pub fn peer_addr(&self) -> VirtualNetworkResult<SocketAddr> {
        Ok(self.remote_address)
    }

    //////////////////////////////////////////////////////////////////////////
    // Private Implementation

    pub(super) fn new(
        machine: Machine,
        socket_id: SocketId,
        local_address: SocketAddr,
        remote_address: SocketAddr,
    ) -> Self {
        Self {
            machine,
            socket_id,
            local_address,
            remote_address,
            current_recv_fut: None,
            current_send_fut: None,
            current_tcp_shutdown_fut: None,
        }
    }
}

impl Drop for VirtualTcpStream {
    fn drop(&mut self) {
        self.machine
            .router_client
            .drop_tcp_stream(self.machine.id, self.socket_id);
    }
}

impl futures_util::AsyncRead for VirtualTcpStream {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        buf: &mut [u8],
    ) -> task::Poll<std::io::Result<usize>> {
        if self.current_recv_fut.is_none() {
            self.current_recv_fut = Some(Box::pin(self.machine.router_client.clone().recv(
                self.machine.id,
                self.socket_id,
                buf.len(),
            )));
        }
        let fut = self.current_recv_fut.as_mut().unwrap();
        fut.poll_unpin(cx).map(|v| match v {
            Ok(v) => {
                let len = usize::min(buf.len(), v.len());
                buf[0..len].copy_from_slice(&v[0..len]);
                self.current_recv_fut = None;
                Ok(len)
            }
            Err(e) => Err(e.into()),
        })
    }
}

impl futures_util::AsyncWrite for VirtualTcpStream {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        buf: &[u8],
    ) -> task::Poll<std::io::Result<usize>> {
        if self.current_send_fut.is_none() {
            self.current_send_fut = Some(Box::pin(self.machine.router_client.clone().send(
                self.machine.id,
                self.socket_id,
                buf.to_vec(),
            )));
        }
        let fut = self.current_send_fut.as_mut().unwrap();
        fut.poll_unpin(cx).map(|v| match v {
            Ok(v) => {
                self.current_send_fut = None;
                Ok(v)
            }
            Err(e) => Err(e.into()),
        })
    }

    fn poll_flush(
        self: Pin<&mut Self>,
        _cx: &mut task::Context<'_>,
    ) -> task::Poll<std::io::Result<()>> {
        task::Poll::Ready(Ok(()))
    }

    fn poll_close(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<std::io::Result<()>> {
        if self.current_tcp_shutdown_fut.is_none() {
            self.current_tcp_shutdown_fut = Some(Box::pin(
                self.machine
                    .router_client
                    .clone()
                    .tcp_shutdown(self.machine.id, self.socket_id),
            ));
        }
        let fut = self.current_tcp_shutdown_fut.as_mut().unwrap();
        fut.poll_unpin(cx).map(|v| match v {
            Ok(v) => {
                self.current_tcp_shutdown_fut = None;
                Ok(v)
            }
            Err(e) => Err(e.into()),
        })
    }
}
