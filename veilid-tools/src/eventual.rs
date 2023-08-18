/// Eventual is like Dart's "Completer"
/// It is a thread-safe concurrent data future that may eventually resolve to a value
/// Three variants exist
/// Eventual, which will complete each 'instance' future to that instance's value (can be different per instance) only when 'resolve' is called.
/// EventualValue, which will complete each 'instance' future when 'resolve' is called with an owned value, and one of those instances may 'take' the value.
/// EventualValueClone, which will complete each 'instance' future when 'resolve' is called with a Clone-able value, and any of those instances may get a clone of that value.
/// The future returned from an Eventual::resolve() can also be awaited on to wait until all instances have been completed
use super::*;

use eventual_base::*;

pub struct Eventual {
    inner: Arc<Mutex<EventualBaseInner<()>>>,
}

impl core::fmt::Debug for Eventual {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Eventual").finish()
    }
}

impl Clone for Eventual {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl EventualBase for Eventual {
    type ResolvedType = ();
    fn base_inner(&self) -> MutexGuard<EventualBaseInner<Self::ResolvedType>> {
        self.inner.lock()
    }
}

impl Default for Eventual {
    fn default() -> Self {
        Self::new()
    }
}

impl Eventual {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(EventualBaseInner::new())),
        }
    }

    pub fn instance_clone<T>(&self, value: T) -> EventualFutureClone<T>
    where
        T: Clone + Unpin,
    {
        EventualFutureClone {
            id: None,
            value,
            eventual: self.clone(),
        }
    }
    pub fn instance_none<T>(&self) -> EventualFutureNone<T>
    where
        T: Unpin,
    {
        EventualFutureNone {
            id: None,
            eventual: self.clone(),
            _marker: core::marker::PhantomData {},
        }
    }
    pub fn instance_empty(&self) -> EventualFutureEmpty {
        EventualFutureEmpty {
            id: None,
            eventual: self.clone(),
        }
    }

    pub fn resolve(&self) -> EventualResolvedFuture<Self> {
        self.resolve_to_value(())
    }
}

///////

pub struct EventualFutureClone<T>
where
    T: Clone + Unpin,
{
    id: Option<usize>,
    value: T,
    eventual: Eventual,
}

impl<T> Future for EventualFutureClone<T>
where
    T: Clone + Unpin,
{
    type Output = T;
    fn poll(mut self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        let this = &mut *self;
        let out = {
            let mut inner = this.eventual.base_inner();
            inner.instance_poll(&mut this.id, cx)
        };
        match out {
            None => task::Poll::<Self::Output>::Pending,
            Some(wakers) => {
                // Wake all EventualResolvedFutures
                for w in wakers {
                    w.wake();
                }
                task::Poll::<Self::Output>::Ready(this.value.clone())
            }
        }
    }
}

impl<T> Drop for EventualFutureClone<T>
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

///////

pub struct EventualFutureNone<T>
where
    T: Unpin,
{
    id: Option<usize>,
    eventual: Eventual,
    _marker: core::marker::PhantomData<T>,
}

impl<T> Future for EventualFutureNone<T>
where
    T: Unpin,
{
    type Output = Option<T>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        let this = &mut *self;
        let out = {
            let mut inner = this.eventual.base_inner();
            inner.instance_poll(&mut this.id, cx)
        };
        match out {
            None => task::Poll::<Self::Output>::Pending,
            Some(wakers) => {
                // Wake all EventualResolvedFutures
                for w in wakers {
                    w.wake();
                }
                task::Poll::<Self::Output>::Ready(None)
            }
        }
    }
}

impl<T> Drop for EventualFutureNone<T>
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

///////

pub struct EventualFutureEmpty {
    id: Option<usize>,
    eventual: Eventual,
}

impl Future for EventualFutureEmpty {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        let this = &mut *self;
        let out = {
            let mut inner = this.eventual.base_inner();
            inner.instance_poll(&mut this.id, cx)
        };
        match out {
            None => task::Poll::<Self::Output>::Pending,
            Some(wakers) => {
                // Wake all EventualResolvedFutures
                for w in wakers {
                    w.wake();
                }
                task::Poll::<Self::Output>::Ready(())
            }
        }
    }
}

impl Drop for EventualFutureEmpty {
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
