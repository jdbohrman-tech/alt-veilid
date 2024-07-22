use super::*;

use eventual_base::*;

pub struct EventualValue<T: Unpin> {
    inner: Arc<Mutex<EventualBaseInner<T>>>,
}

impl<T: Unpin> core::fmt::Debug for EventualValue<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("EventualValue").finish()
    }
}

impl<T: Unpin> Clone for EventualValue<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T: Unpin> EventualBase for EventualValue<T> {
    type ResolvedType = T;
    fn base_inner(&self) -> MutexGuard<EventualBaseInner<Self::ResolvedType>> {
        self.inner.lock()
    }
}

impl<T: Unpin> Default for EventualValue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Unpin> EventualValue<T> {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(EventualBaseInner::new())),
        }
    }

    pub fn instance(&self) -> EventualValueFuture<T> {
        EventualValueFuture {
            id: None,
            eventual: self.clone(),
        }
    }

    pub fn resolve(&self, value: T) -> EventualResolvedFuture<Self> {
        self.resolve_to_value(value)
    }

    pub fn take_value(&self) -> Option<T> {
        let mut inner = self.inner.lock();
        inner.resolved_value_mut().take()
    }
}

pub struct EventualValueFuture<T: Unpin> {
    id: Option<usize>,
    eventual: EventualValue<T>,
}

impl<T: Unpin> core::fmt::Debug for EventualValueFuture<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("EventualValueFuture")
            .field("id", &self.id)
            .finish()
    }
}

impl<T: Unpin> Future for EventualValueFuture<T> {
    type Output = EventualValue<T>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        let this = &mut *self;
        let out = {
            let mut inner = this.eventual.base_inner();
            inner.instance_poll(&mut this.id, cx)
        };
        match out {
            None => task::Poll::<Self::Output>::Pending,
            Some(wakers) => {
                // Wake all other instance futures
                for w in wakers {
                    w.wake();
                }
                task::Poll::<Self::Output>::Ready(this.eventual.clone())
            }
        }
    }
}

impl<T> Drop for EventualValueFuture<T>
where
    T: Unpin,
{
    fn drop(&mut self) {
        if let Some(id) = self.id.take() {
            let wakers = {
                let mut inner = self.eventual.base_inner();
                inner.remove_waker(id)
            };
            for w in wakers {
                w.wake();
            }
        }
    }
}
