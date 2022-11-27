use super::*;

#[derive(ThisError, Debug, Clone, PartialEq, Eq)]
pub enum EventualError {
    #[error("Try failed: {0}")]
    TryFailed(String),
}

pub struct EventualBaseInner<T> {
    resolved: Option<T>,
    wakers: BTreeMap<usize, task::Waker>,
    resolved_wakers: BTreeMap<usize, task::Waker>,
    freelist: Vec<usize>,
    resolved_freelist: Vec<usize>,
}

impl<T> EventualBaseInner<T> {
    pub(super) fn new() -> Self {
        EventualBaseInner {
            resolved: None,
            wakers: BTreeMap::new(),
            resolved_wakers: BTreeMap::new(),
            freelist: Vec::new(),
            resolved_freelist: Vec::new(),
        }
    }

    pub(super) fn insert_waker(&mut self, waker: task::Waker) -> usize {
        let id = match self.freelist.pop() {
            Some(id) => id,
            None => self.wakers.len(),
        };
        self.wakers.insert(id, waker);
        id
    }

    #[must_use]
    pub(super) fn remove_waker(&mut self, id: usize) -> Vec<task::Waker> {
        self.freelist.push(id);
        self.wakers.remove(&id);
        // See if we should complete the EventualResolvedFutures
        let mut resolved_waker_list = Vec::new();
        if self.wakers.is_empty() && self.resolved.is_some() {
            for w in &self.resolved_wakers {
                resolved_waker_list.push(w.1.clone());
            }
        }
        resolved_waker_list
    }

    pub(super) fn insert_resolved_waker(&mut self, waker: task::Waker) -> usize {
        let id = match self.resolved_freelist.pop() {
            Some(id) => id,
            None => self.resolved_wakers.len(),
        };
        self.resolved_wakers.insert(id, waker);
        id
    }

    pub(super) fn remove_resolved_waker(&mut self, id: usize) {
        self.resolved_freelist.push(id);
        self.resolved_wakers.remove(&id);
    }

    #[must_use]
    pub(super) fn resolve_and_get_wakers(&mut self, value: T) -> Option<Vec<task::Waker>> {
        if self.resolved.is_some() {
            // Already resolved
            return None;
        }

        // Store resolved value
        self.resolved = Some(value);

        // Return a copy of the waker list so the caller can wake all the EventualFutures
        let mut waker_list = Vec::new();
        for w in &self.wakers {
            waker_list.push(w.1.clone());
        }
        Some(waker_list)
    }

    pub(super) fn is_resolved(&self) -> bool {
        self.resolved.is_some()
    }
    pub(super) fn resolved_value_ref(&self) -> &Option<T> {
        &self.resolved
    }
    pub(super) fn resolved_value_mut(&mut self) -> &mut Option<T> {
        &mut self.resolved
    }

    pub(super) fn reset(&mut self) {
        assert_eq!(self.wakers.len(), 0);
        assert_eq!(self.resolved_wakers.len(), 0);
        self.resolved = None;
        self.freelist.clear();
        self.resolved_freelist.clear();
    }

    pub(super) fn try_reset(&mut self) -> Result<(), EventualError> {
        if !self.wakers.is_empty() {
            return Err(EventualError::TryFailed(
                "wakers not empty during reset".to_owned(),
            ));
        }
        if !self.resolved_wakers.is_empty() {
            return Err(EventualError::TryFailed(
                "Resolved wakers not empty during reset".to_owned(),
            ));
        }
        self.reset();
        Ok(())
    }

    // Resolved future helpers
    pub(super) fn resolved_poll(
        &mut self,
        id: &mut Option<usize>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<()> {
        // If there are any instance futures still waiting, we resolution isn't finished
        if !self.wakers.is_empty() {
            if id.is_none() {
                *id = Some(self.insert_resolved_waker(cx.waker().clone()));
            }
            task::Poll::<()>::Pending
        } else {
            if let Some(id) = id.take() {
                self.remove_resolved_waker(id);
            }
            task::Poll::<()>::Ready(())
        }
    }

    // Instance future helpers
    #[must_use]
    pub(super) fn instance_poll(
        &mut self,
        id: &mut Option<usize>,
        cx: &mut task::Context<'_>,
    ) -> Option<Vec<task::Waker>> {
        // If the resolved value hasn't showed up then we can't wake the instance futures
        if self.resolved.is_none() {
            if id.is_none() {
                *id = Some(self.insert_waker(cx.waker().clone()));
            }
            None
        } else if let Some(id) = id.take() {
            Some(self.remove_waker(id))
        } else {
            Some(Vec::new())
        }
    }
}

// xxx: this would love to be 'pub(super)' instead of pub, to ensure nobody else touches resolve_to_value directly
pub trait EventualBase: Clone + Unpin {
    type ResolvedType;

    fn base_inner(&self) -> MutexGuard<EventualBaseInner<Self::ResolvedType>>;

    fn resolve_to_value(&self, value: Self::ResolvedType) -> EventualResolvedFuture<Self> {
        let wakers = {
            let mut inner = self.base_inner();
            inner.resolve_and_get_wakers(value)
        };
        if let Some(wakers) = wakers {
            for w in wakers {
                w.wake();
            }
        }
        EventualResolvedFuture {
            id: None,
            eventual: self.clone(),
        }
    }
}

pub struct EventualResolvedFuture<B: EventualBase> {
    id: Option<usize>,
    eventual: B,
}

impl<B: EventualBase> Future for EventualResolvedFuture<B> {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        let this = &mut *self;
        let mut inner = this.eventual.base_inner();
        inner.resolved_poll(&mut this.id, cx)
    }
}

impl<B: EventualBase> Drop for EventualResolvedFuture<B> {
    fn drop(&mut self) {
        if let Some(id) = self.id.take() {
            let mut inner = self.eventual.base_inner();
            inner.remove_resolved_waker(id);
        }
    }
}

pub trait EventualCommon: EventualBase {
    fn is_resolved(&self) -> bool {
        self.base_inner().is_resolved()
    }

    fn reset(&self) {
        self.base_inner().reset()
    }

    fn try_reset(&self) -> Result<(), EventualError> {
        self.base_inner().try_reset()
    }
}

impl<T> EventualCommon for T where T: EventualBase {}
