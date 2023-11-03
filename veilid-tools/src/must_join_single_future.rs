use super::*;

use core::task::Poll;
use futures_util::poll;

#[derive(Debug)]
struct MustJoinSingleFutureInner<T>
where
    T: 'static,
{
    locked: bool,
    join_handle: Option<MustJoinHandle<T>>,
}

/// Spawns a single background processing task idempotently, possibly returning the return value of the previously executed background task
/// This does not queue, just ensures that no more than a single copy of the task is running at a time, but allowing tasks to be retriggered
#[derive(Debug, Clone)]
pub struct MustJoinSingleFuture<T>
where
    T: 'static,
{
    inner: Arc<Mutex<MustJoinSingleFutureInner<T>>>,
}

impl<T> Default for MustJoinSingleFuture<T>
where
    T: 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> MustJoinSingleFuture<T>
where
    T: 'static,
{
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(MustJoinSingleFutureInner {
                locked: false,
                join_handle: None,
            })),
        }
    }

    fn try_lock(&self) -> Result<Option<MustJoinHandle<T>>, ()> {
        let mut inner = self.inner.lock();
        if inner.locked {
            // If already locked error out
            return Err(());
        }
        inner.locked = true;
        // If we got the lock, return what we have for a join handle if anything
        Ok(inner.join_handle.take())
    }

    fn unlock(&self, jh: Option<MustJoinHandle<T>>) {
        let mut inner = self.inner.lock();
        assert!(inner.locked);
        assert!(inner.join_handle.is_none());
        inner.locked = false;
        inner.join_handle = jh;
    }

    /// Check the result and take it if there is one
    pub async fn check(&self) -> Result<Option<T>, ()> {
        let mut out: Option<T> = None;

        // See if we have a result we can return
        let maybe_jh = match self.try_lock() {
            Ok(v) => v,
            Err(_) => {
                // If we are already polling somewhere else, don't hand back a result
                return Err(());
            }
        };
        if let Some(mut jh) = maybe_jh {
            // See if we finished, if so, return the value of the last execution
            if let Poll::Ready(r) = poll!(&mut jh) {
                out = Some(r);
                // Task finished, unlock with nothing
                self.unlock(None);
            } else {
                // Still running put the join handle back so we can check on it later
                self.unlock(Some(jh));
            }
        } else {
            // No task, unlock with nothing
            self.unlock(None);
        }

        // Return the prior result if we have one
        Ok(out)
    }

    /// Wait for the result and take it
    pub async fn join(&self) -> Result<Option<T>, ()> {
        let mut out: Option<T> = None;

        // See if we have a result we can return
        let maybe_jh = match self.try_lock() {
            Ok(v) => v,
            Err(_) => {
                // If we are already polling somewhere else,
                // that's an error because you can only join
                // these things once
                return Err(());
            }
        };
        if let Some(jh) = maybe_jh {
            // Wait for return value of the last execution
            out = Some(jh.await);
            // Task finished, unlock with nothing
        } else {
            // No task, unlock with nothing
        }
        self.unlock(None);

        // Return the prior result if we have one
        Ok(out)
    }

    // Possibly spawn the future possibly returning the value of the last execution
    pub async fn single_spawn_local(
        &self,
        future: impl Future<Output = T> + 'static,
    ) -> Result<(Option<T>, bool), ()> {
        let mut out: Option<T> = None;

        // See if we have a result we can return
        let maybe_jh = match self.try_lock() {
            Ok(v) => v,
            Err(_) => {
                // If we are already polling somewhere else, don't hand back a result
                return Err(());
            }
        };
        let mut run = true;

        if let Some(mut jh) = maybe_jh {
            // See if we finished, if so, return the value of the last execution
            if let Poll::Ready(r) = poll!(&mut jh) {
                out = Some(r);
                // Task finished, unlock with a new task
            } else {
                // Still running, don't run again, unlock with the current join handle
                run = false;
                self.unlock(Some(jh));
            }
        }

        // Run if we should do that
        if run {
            self.unlock(Some(spawn_local(future)));
        }

        // Return the prior result if we have one
        Ok((out, run))
    }
}

impl<T> MustJoinSingleFuture<T>
where
    T: 'static + Send,
{
    pub async fn single_spawn(
        &self,
        future: impl Future<Output = T> + Send + 'static,
    ) -> Result<(Option<T>, bool), ()> {
        let mut out: Option<T> = None;
        // See if we have a result we can return
        let maybe_jh = match self.try_lock() {
            Ok(v) => v,
            Err(_) => {
                // If we are already polling somewhere else, don't hand back a result
                return Err(());
            }
        };
        let mut run = true;
        if let Some(mut jh) = maybe_jh {
            // See if we finished, if so, return the value of the last execution
            if let Poll::Ready(r) = poll!(&mut jh) {
                out = Some(r);
                // Task finished, unlock with a new task
            } else {
                // Still running, don't run again, unlock with the current join handle
                run = false;
                self.unlock(Some(jh));
            }
        }
        // Run if we should do that
        if run {
            self.unlock(Some(spawn(future)));
        }
        // Return the prior result if we have one
        Ok((out, run))
    }
}
