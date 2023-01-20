use super::*;

use core::pin::Pin;
use core::task::{Context, Poll};
use futures_util::AsyncRead as Read;
use futures_util::AsyncWrite as Write;
use futures_util::Sink;
use futures_util::Stream;
use std::io;

pub struct CloneStream<T>
where
    T: Unpin,
{
    inner: Arc<Mutex<T>>,
}

impl<T> Clone for CloneStream<T>
where
    T: Unpin,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> CloneStream<T>
where
    T: Unpin,
{
    pub fn new(t: T) -> Self {
        Self {
            inner: Arc::new(Mutex::new(t)),
        }
    }
}

impl<T> Read for CloneStream<T>
where
    T: Read + Unpin,
{
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        let mut inner = self.inner.lock();
        Pin::new(&mut *inner).poll_read(cx, buf)
    }
}

impl<T> Write for CloneStream<T>
where
    T: Write + Unpin,
{
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        let mut inner = self.inner.lock();
        Pin::new(&mut *inner).poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        let mut inner = self.inner.lock();
        Pin::new(&mut *inner).poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        let mut inner = self.inner.lock();
        Pin::new(&mut *inner).poll_close(cx)
    }
}

impl<T> Stream for CloneStream<T>
where
    T: Stream + Unpin,
{
    type Item = T::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut inner = self.inner.lock();
        Pin::new(&mut *inner).poll_next(cx)
    }
}

impl<T, Item> Sink<Item> for CloneStream<T>
where
    T: Sink<Item> + Unpin,
{
    type Error = T::Error;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let mut inner = self.inner.lock();
        Pin::new(&mut *inner).poll_ready(cx)
    }
    fn start_send(self: Pin<&mut Self>, item: Item) -> Result<(), Self::Error> {
        let mut inner = self.inner.lock();
        Pin::new(&mut *inner).start_send(item)
    }
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let mut inner = self.inner.lock();
        Pin::new(&mut *inner).poll_flush(cx)
    }
    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let mut inner = self.inner.lock();
        Pin::new(&mut *inner).poll_close(cx)
    }
}
