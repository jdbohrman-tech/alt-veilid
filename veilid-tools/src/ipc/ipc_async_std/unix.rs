use crate::*;
use async_std::io::Read as AsyncRead;
use async_std::io::Write as AsyncWrite;
use async_std::os::unix::net::{Incoming, UnixListener, UnixStream};
use futures_util::AsyncRead as FuturesAsyncRead;
use futures_util::AsyncWrite as FuturesAsyncWrite;
use futures_util::Stream;
use std::path::PathBuf;
use std::{io, path::Path};

/////////////////////////////////////////////////////////////

pub struct IpcStream {
    internal: UnixStream,
}

impl IpcStream {
    pub async fn connect<P: AsRef<Path>>(path: P) -> io::Result<IpcStream> {
        Ok(IpcStream {
            internal: UnixStream::connect(path.as_ref()).await?,
        })
    }
}

impl FuturesAsyncRead for IpcStream {
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut [u8],
    ) -> std::task::Poll<io::Result<usize>> {
        <UnixStream as AsyncRead>::poll_read(std::pin::Pin::new(&mut self.internal), cx, buf)
    }
}

impl FuturesAsyncWrite for IpcStream {
    fn poll_write(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<io::Result<usize>> {
        <UnixStream as AsyncWrite>::poll_write(std::pin::Pin::new(&mut self.internal), cx, buf)
    }

    fn poll_flush(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<io::Result<()>> {
        <UnixStream as AsyncWrite>::poll_flush(std::pin::Pin::new(&mut self.internal), cx)
    }

    fn poll_close(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<io::Result<()>> {
        <UnixStream as AsyncWrite>::poll_close(std::pin::Pin::new(&mut self.internal), cx)
    }
}

/////////////////////////////////////////////////////////////

pub struct IpcIncoming<'a> {
    path: PathBuf,
    internal: Incoming<'a>,
}

impl<'a> Drop for IpcIncoming<'a> {
    fn drop(&mut self) {
        // Clean up IPC path
        if let Err(e) = std::fs::remove_file(&self.path) {
            warn!("Unable to remove IPC socket: {}", e);
        }
    }
}

impl<'a> Stream for IpcIncoming<'a> {
    type Item = io::Result<IpcStream>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        match <Incoming as Stream>::poll_next(std::pin::Pin::new(&mut self.internal), cx) {
            std::task::Poll::Ready(ro) => {
                std::task::Poll::Ready(ro.map(|rr| rr.map(|s| IpcStream { internal: s })))
            }
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
    }
}

/////////////////////////////////////////////////////////////

pub struct IpcListener {
    path: Option<PathBuf>,
    internal: Option<Arc<UnixListener>>,
}

impl IpcListener {
    /// Creates a new `IpcListener` bound to the specified path.
    pub async fn bind<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        Ok(Self {
            path: Some(path.as_ref().to_path_buf()),
            internal: Some(Arc::new(UnixListener::bind(path.as_ref()).await?)),
        })
    }

    /// Accepts a new incoming connection to this listener.
    pub fn accept(&self) -> SendPinBoxFuture<io::Result<IpcStream>> {
        if self.path.is_none() {
            return Box::pin(std::future::ready(Err(io::Error::from(
                io::ErrorKind::NotConnected,
            ))));        
        }
        let this = IpcListener {
            path: self.path.clone(),
            internal: self.internal.clone(),
        };
        Box::pin(async move {
            Ok(IpcStream {
                internal: this.internal.as_ref().unwrap().accept().await?.0,
            })
        })
    }

    /// Returns a stream of incoming connections.
    pub fn incoming<'a>(&'a mut self) -> io::Result<IpcIncoming<'a>> {
        if self.path.is_none() {
            return Err(io::Error::from(io::ErrorKind::NotConnected));
        }
        Ok(IpcIncoming {
            path: self.path.take().unwrap(),
            internal: self.internal.as_ref().unwrap().incoming(),
        })
    }
}

impl Drop for IpcListener {
    fn drop(&mut self) {
        // Clean up IPC path
        if let Some(path) = &self.path {
            if let Err(e) = std::fs::remove_file(path) {
                warn!("Unable to remove IPC socket: {}", e);
            }
        }
    }
}
