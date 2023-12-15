use crate::*;
use futures_util::AsyncRead as FuturesAsyncRead;
use futures_util::AsyncWrite as FuturesAsyncWrite;
use futures_util::Stream;
use std::path::PathBuf;
use std::{io, path::Path};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use tokio::net::{UnixListener, UnixStream};
use tokio_stream::wrappers::UnixListenerStream;
/////////////////////////////////////////////////////////////

pub struct IpcStream {
    internal: UnixStream,
}

impl IpcStream {
    pub async fn connect<P: AsRef<Path>>(path: P) -> io::Result<IpcStream> {
        Ok(IpcStream {
            internal: UnixStream::connect(path).await?,
        })
    }
}

impl FuturesAsyncRead for IpcStream {
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut [u8],
    ) -> std::task::Poll<io::Result<usize>> {
        let mut rb = ReadBuf::new(buf);
        match <UnixStream as AsyncRead>::poll_read(
            std::pin::Pin::new(&mut self.internal),
            cx,
            &mut rb,
        ) {
            std::task::Poll::Ready(r) => std::task::Poll::Ready(r.map(|_| rb.filled().len())),
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
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
        <UnixStream as AsyncWrite>::poll_shutdown(std::pin::Pin::new(&mut self.internal), cx)
    }
}

/////////////////////////////////////////////////////////////

pub struct IpcIncoming {
    path: PathBuf,
    internal: UnixListenerStream,
}

impl Stream for IpcIncoming {
    type Item = io::Result<IpcStream>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        match <UnixListenerStream as Stream>::poll_next(std::pin::Pin::new(&mut self.internal), cx)
        {
            std::task::Poll::Ready(ro) => {
                std::task::Poll::Ready(ro.map(|rr| rr.map(|s| IpcStream { internal: s })))
            }
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
    }
}

impl Drop for IpcIncoming {
    fn drop(&mut self) {
        // Clean up IPC path
        if let Err(e) = std::fs::remove_file(&self.path) {
            warn!("Unable to remove IPC socket: {}", e);
        }
    }
}

/////////////////////////////////////////////////////////////

pub struct IpcListener {
    path: Option<PathBuf>,
    internal: Option<UnixListener>,
}

impl IpcListener {
    /// Creates a new `IpcListener` bound to the specified path.
    pub async fn bind<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        Ok(Self {
            path: Some(path.as_ref().to_path_buf()),
            internal: Some(UnixListener::bind(path)?),
        })
    }

    /// Accepts a new incoming connection to this listener.
    pub async fn accept(&self) -> io::Result<IpcStream> {
        Ok(IpcStream {
            internal: self.internal.as_ref().unwrap().accept().await?.0,
        })
    }

    /// Returns a stream of incoming connections.
    pub fn incoming(mut self) -> IpcIncoming {
        IpcIncoming {
            path: self.path.take().unwrap(),
            internal: UnixListenerStream::new(self.internal.take().unwrap()),
        }
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
