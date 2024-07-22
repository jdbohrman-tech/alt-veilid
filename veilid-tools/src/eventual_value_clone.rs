use super::*;

use eventual_base::*;

pub struct EventualValueClone<T: Unpin + Clone> {
    inner: Arc<Mutex<EventualBaseInner<T>>>,
}

impl<T: Unpin + Clone> core::fmt::Debug for EventualValueClone<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("EventualValueClone").finish()
    }
}

impl<T: Unpin + Clone> Clone for EventualValueClone<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T: Unpin + Clone> EventualBase for EventualValueClone<T> {
    type ResolvedType = T;
    fn base_inner(&self) -> MutexGuard<EventualBaseInner<Self::ResolvedType>> {
        self.inner.lock()
    }
}

impl<T: Unpin + Clone> Default for EventualValueClone<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Unpin + Clone> EventualValueClone<T> {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(EventualBaseInner::new())),
        }
    }

    pub fn instance(&self) -> EventualValueCloneFuture<T>
    where
        T: Clone + Unpin,
    {
        EventualValueCloneFuture {
            id: None,
            eventual: self.clone(),
        }
    }

    pub fn resolve(&self, value: T) -> EventualResolvedFuture<Self> {
        self.resolve_to_value(value)
    }

    pub fn value(&self) -> Option<T> {
        let inner = self.inner.lock();
        inner.resolved_value_ref().clone()
    }
}

pub struct EventualValueCloneFuture<T: Unpin + Clone> {
    id: Option<usize>,
    eventual: EventualValueClone<T>,
}

impl<T: Unpin + Clone> Future for EventualValueCloneFuture<T> {
    type Output = T;
    fn poll(mut self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        let this = &mut *self;
        let (out, some_value) = {
            let mut inner = this.eventual.base_inner();
            let out = inner.instance_poll(&mut this.id, cx);
            (out, inner.resolved_value_ref().clone())
        };
        match out {
            None => task::Poll::<Self::Output>::Pending,
            Some(wakers) => {
                // Wake all other instance futures
                for w in wakers {
                    w.wake();
                }
                task::Poll::<Self::Output>::Ready(some_value.unwrap())
            }
        }
    }
}

impl<T> Drop for EventualValueCloneFuture<T>
where
    T: Clone + Unpin,
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
