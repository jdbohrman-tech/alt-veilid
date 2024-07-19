use super::*;

#[derive(ThisError, Debug, Copy, Clone, PartialEq, Eq)]
#[error("Already started")]
pub struct StartupLockAlreadyStartedError;

#[derive(ThisError, Debug, Copy, Clone, PartialEq, Eq)]
#[error("Already shut down")]
pub struct StartupLockAlreadyShutDownError;

#[derive(ThisError, Debug, Copy, Clone, PartialEq, Eq)]
#[error("Not started")]
pub struct StartupLockNotStartedError;

/// RAII-style lock for startup and shutdown operations
/// Must call 'success()' on this lock to report a successful startup or shutdown
/// Dropping this lock without calling 'success()' first indicates a failed
/// startup or shutdown operation
#[derive(Debug)]
pub struct StartupLockGuard<'a> {
    guard: AsyncRwLockWriteGuard<'a, bool>,
    success_value: bool,
}

impl<'a> StartupLockGuard<'a> {
    /// Call this function at the end of a successful startup or shutdown
    /// operation to switch the state of the StartupLock.
    pub fn success(mut self) {
        *self.guard = self.success_value;
    }
}

/// RAII-style lock for entry operations on a started-up region of code.
#[derive(Debug)]
pub struct StartupLockEnterGuard<'a> {
    _guard: AsyncRwLockReadGuard<'a, bool>,
}

/// Synchronization mechanism that tracks the startup and shutdown of a region of code.
/// Guarantees that some code can only be started up once and shut down only if it is
/// already started.
/// Also tracks if the code is in-use and will wait for all 'entered' code to finish
/// before shutting down. Once a shutdown is requested, future calls to 'enter' will
/// fail, ensuring that nothing is 'entered' at the time of shutdown. This allows an
/// asynchronous shutdown to wait for operations to finish before proceeding.
#[derive(Debug)]
pub struct StartupLock {
    rwlock: AsyncRwLock<bool>,
}

impl StartupLock {
    pub fn new() -> Self {
        Self {
            rwlock: AsyncRwLock::new(false),
        }
    }

    /// Start up if things are not already started up
    /// One must call 'success()' on the returned startup lock guard if startup was successful
    /// otherwise the startup lock will not shift to the 'started' state.
    pub fn startup(&self) -> Result<StartupLockGuard, StartupLockAlreadyStartedError> {
        let guard = asyncrwlock_try_write!(self.rwlock).ok_or(StartupLockAlreadyStartedError)?;
        if *guard {
            return Err(StartupLockAlreadyStartedError);
        }
        Ok(StartupLockGuard {
            guard,
            success_value: true,
        })
    }

    /// Check if this StartupLock is currently in a started state
    /// Returns false is the state is in transition
    pub fn is_started(&self) -> bool {
        let Some(guard) = asyncrwlock_try_read!(self.rwlock) else {
            return false;
        };
        *guard
    }

    /// Check if this StartupLock is currently in a shut down state
    /// Returns false is the state is in transition
    pub fn is_shut_down(&self) -> bool {
        let Some(guard) = asyncrwlock_try_read!(self.rwlock) else {
            return false;
        };
        !*guard
    }

    /// Wait for all 'entered' operations to finish before shutting down
    /// One must call 'success()' on the returned startup lock guard if shutdown was successful
    /// otherwise the startup lock will not shift to the 'stopped' state.
    pub async fn shutdown(&self) -> Result<StartupLockGuard, StartupLockAlreadyShutDownError> {
        let guard = self.rwlock.write().await;
        if !*guard {
            return Err(StartupLockAlreadyShutDownError);
        }
        Ok(StartupLockGuard {
            guard,
            success_value: false,
        })
    }

    /// Enter an operation in a started-up module.
    /// If this module has not yet started up or is in the process of startup or shutdown
    /// this will fail.
    pub fn enter(&self) -> Result<StartupLockEnterGuard, StartupLockNotStartedError> {
        let guard = asyncrwlock_try_read!(self.rwlock).ok_or(StartupLockNotStartedError)?;
        if !*guard {
            return Err(StartupLockNotStartedError);
        }
        Ok(StartupLockEnterGuard { _guard: guard })
    }
}

impl Default for StartupLock {
    fn default() -> Self {
        Self::new()
    }
}
