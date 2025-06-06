use crate::*;
use futures_util::stream::FuturesUnordered;
use futures_util::AsyncRead as FuturesAsyncRead;
use futures_util::AsyncWrite as FuturesAsyncWrite;
use futures_util::Stream;
use std::path::PathBuf;
use std::{io, path::Path};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use tokio::net::windows::named_pipe::{
    ClientOptions, NamedPipeClient, NamedPipeServer, ServerOptions,
};
/////////////////////////////////////////////////////////////

enum IpcStreamInternal {
    Client(NamedPipeClient),
    Server(NamedPipeServer),
}

pub struct IpcStream {
    internal: IpcStreamInternal,
}

impl IpcStream {
    #[expect(clippy::unused_async)]
    pub async fn connect<P: AsRef<Path>>(path: P) -> io::Result<IpcStream> {
        Ok(IpcStream {
            internal: IpcStreamInternal::Client(
                ClientOptions::new().open(path.as_ref().to_path_buf().as_os_str())?,
            ),
        })
    }
}

impl FuturesAsyncRead for IpcStream {
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut [u8],
    ) -> std::task::Poll<io::Result<usize>> {
        match &mut self.internal {
            IpcStreamInternal::Client(client) => {
                let mut rb = ReadBuf::new(buf);
                match <NamedPipeClient as AsyncRead>::poll_read(
                    std::pin::Pin::new(client),
                    cx,
                    &mut rb,
                ) {
                    std::task::Poll::Ready(r) => {
                        std::task::Poll::Ready(r.map(|_| rb.filled().len()))
                    }
                    std::task::Poll::Pending => std::task::Poll::Pending,
                }
            }
            IpcStreamInternal::Server(server) => {
                let mut rb = ReadBuf::new(buf);
                match <NamedPipeServer as AsyncRead>::poll_read(
                    std::pin::Pin::new(server),
                    cx,
                    &mut rb,
                ) {
                    std::task::Poll::Ready(r) => {
                        std::task::Poll::Ready(r.map(|_| rb.filled().len()))
                    }
                    std::task::Poll::Pending => std::task::Poll::Pending,
                }
            }
        }
    }
}

impl FuturesAsyncWrite for IpcStream {
    fn poll_write(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<io::Result<usize>> {
        match &mut self.internal {
            IpcStreamInternal::Client(client) => {
                <NamedPipeClient as AsyncWrite>::poll_write(std::pin::Pin::new(client), cx, buf)
            }
            IpcStreamInternal::Server(server) => {
                <NamedPipeServer as AsyncWrite>::poll_write(std::pin::Pin::new(server), cx, buf)
            }
        }
    }

    fn poll_flush(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<io::Result<()>> {
        match &mut self.internal {
            IpcStreamInternal::Client(client) => {
                <NamedPipeClient as AsyncWrite>::poll_flush(std::pin::Pin::new(client), cx)
            }
            IpcStreamInternal::Server(server) => {
                <NamedPipeServer as AsyncWrite>::poll_flush(std::pin::Pin::new(server), cx)
            }
        }
    }

    fn poll_close(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<io::Result<()>> {
        match &mut self.internal {
            IpcStreamInternal::Client(client) => {
                <NamedPipeClient as AsyncWrite>::poll_shutdown(std::pin::Pin::new(client), cx)
            }
            IpcStreamInternal::Server(server) => {
                <NamedPipeServer as AsyncWrite>::poll_shutdown(std::pin::Pin::new(server), cx)
            }
        }
    }
}

/////////////////////////////////////////////////////////////

pub struct IpcIncoming<'a> {
    listener: IpcListener,
    unord: FuturesUnordered<PinBoxFutureStatic<io::Result<IpcStream>>>,
    phantom: std::marker::PhantomData<&'a ()>,
}

impl Stream for IpcIncoming<'_> {
    type Item = io::Result<IpcStream>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        if self.unord.is_empty() {
            self.unord.push(Box::pin(self.listener.accept()));
        }
        match Pin::new(&mut self.unord).poll_next(cx) {
            task::Poll::Ready(ro) => {
                self.unord.push(Box::pin(self.listener.accept()));
                std::task::Poll::Ready(ro)
            }
            task::Poll::Pending => std::task::Poll::Pending,
        }
    }
}

/////////////////////////////////////////////////////////////

pub struct IpcListener {
    path: Option<PathBuf>,
    internal: Option<Mutex<Option<NamedPipeServer>>>,
}

impl IpcListener {
    /// Creates a new `IpcListener` bound to the specified path.
    #[expect(clippy::unused_async)]
    pub async fn bind<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path = path.as_ref().to_path_buf();
        let server = ServerOptions::new()
            .first_pipe_instance(true)
            .create(&path)?;
        Ok(Self {
            path: Some(path),
            internal: Some(Mutex::new(Some(server))),
        })
    }

    /// Accepts a new incoming connection to this listener.
    #[must_use]
    pub fn accept(&self) -> PinBoxFutureStatic<io::Result<IpcStream>> {
        if self.path.is_none() {
            return Box::pin(std::future::ready(Err(io::Error::from(
                io::ErrorKind::NotConnected,
            ))));
        }
        let internal = self.internal.as_ref().unwrap();
        let mut opt_server = internal.lock();
        let server = opt_server.take().unwrap();
        let path = self.path.clone().unwrap();
        *opt_server = match ServerOptions::new().create(path) {
            Ok(v) => Some(v),
            Err(e) => return Box::pin(std::future::ready(Err(e))),
        };

        Box::pin(async move {
            server.connect().await?;

            Ok(IpcStream {
                internal: IpcStreamInternal::Server(server),
            })
        })
    }

    /// Returns a stream of incoming connections.
    pub fn incoming(&mut self) -> io::Result<IpcIncoming<'_>> {
        if self.path.is_none() {
            return Err(io::Error::from(io::ErrorKind::NotConnected));
        }
        Ok(IpcIncoming {
            listener: IpcListener {
                path: self.path.take(),
                internal: self.internal.take(),
            },
            unord: FuturesUnordered::new(),
            phantom: std::marker::PhantomData,
        })
    }
}
