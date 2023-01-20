use super::*;

pub struct MutableFuture<O, T: Future<Output = O>> {
    inner: Arc<Mutex<Pin<Box<T>>>>,
}

impl<O, T: Future<Output = O>> MutableFuture<O, T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner: Arc::new(Mutex::new(Box::pin(inner))),
        }
    }

    pub fn set(&self, inner: T) {
        *self.inner.lock() = Box::pin(inner);
    }
}

impl<O, T: Future<Output = O>> Clone for MutableFuture<O, T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<O, T: Future<Output = O>> Future for MutableFuture<O, T> {
    type Output = O;
    fn poll(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        let mut inner = self.inner.lock();
        T::poll(inner.as_mut(), cx)
    }
}
