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

impl StartupLockGuard<'_> {
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
    #[cfg(feature = "debug-locks")]
    id: usize,
    #[cfg(feature = "debug-locks")]
    active_guards: Arc<Mutex<HashMap<usize, backtrace::Backtrace>>>,
}

#[cfg(feature = "debug-locks")]
impl<'a> Drop for StartupLockEnterGuard<'a> {
    fn drop(&mut self) {
        self.active_guards.lock().remove(&self.id);
    }
}

/// RAII-style lock for entry operations on a started-up region of code.
#[derive(Debug)]
pub struct StartupLockEnterGuardArc {
    _guard: AsyncRwLockReadGuardArc<bool>,
    #[cfg(feature = "debug-locks")]
    id: usize,
    #[cfg(feature = "debug-locks")]
    active_guards: Arc<Mutex<HashMap<usize, backtrace::Backtrace>>>,
}

#[cfg(feature = "debug-locks")]
impl Drop for StartupLockEnterGuardArc {
    fn drop(&mut self) {
        self.active_guards.lock().remove(&self.id);
    }
}

#[cfg(feature = "debug-locks")]
static GUARD_ID: AtomicUsize = AtomicUsize::new(0);

/// Synchronization mechanism that tracks the startup and shutdown of a region of code.
/// Guarantees that some code can only be started up once and shut down only if it is
/// already started.
/// Also tracks if the code is in-use and will wait for all 'entered' code to finish
/// before shutting down. Once a shutdown is requested, future calls to 'enter' will
/// fail, ensuring that nothing is 'entered' at the time of shutdown. This allows an
/// asynchronous shutdown to wait for operations to finish before proceeding.
#[derive(Debug)]
pub struct StartupLock {
    startup_state: Arc<AsyncRwLock<bool>>,
    stop_source: Mutex<Option<StopSource>>,
    #[cfg(feature = "debug-locks")]
    active_guards: Arc<Mutex<HashMap<usize, backtrace::Backtrace>>>,
}

impl StartupLock {
    #[must_use]
    pub fn new() -> Self {
        Self {
            startup_state: Arc::new(AsyncRwLock::new(false)),
            stop_source: Mutex::new(None),
            #[cfg(feature = "debug-locks")]
            active_guards: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Start up if things are not already started up
    /// One must call 'success()' on the returned startup lock guard if startup was successful
    /// otherwise the startup lock will not shift to the 'started' state.
    pub fn startup(&self) -> Result<StartupLockGuard, StartupLockAlreadyStartedError> {
        let guard =
            asyncrwlock_try_write!(self.startup_state).ok_or(StartupLockAlreadyStartedError)?;
        if *guard {
            return Err(StartupLockAlreadyStartedError);
        }
        *self.stop_source.lock() = Some(StopSource::new());

        Ok(StartupLockGuard {
            guard,
            success_value: true,
        })
    }

    /// Get a stop token for this lock
    /// One can wait on this to timeout operations when a shutdown is requested
    pub fn stop_token(&self) -> Option<StopToken> {
        self.stop_source.lock().as_ref().map(|ss| ss.token())
    }

    /// Check if this StartupLock is currently in a started state
    /// Returns false is the state is in transition
    pub fn is_started(&self) -> bool {
        let Some(guard) = asyncrwlock_try_read!(self.startup_state) else {
            return false;
        };
        *guard
    }

    /// Check if this StartupLock is currently in a shut down state
    /// Returns false is the state is in transition
    pub fn is_shut_down(&self) -> bool {
        let Some(guard) = asyncrwlock_try_read!(self.startup_state) else {
            return false;
        };
        !*guard
    }

    /// Wait for all 'entered' operations to finish before shutting down
    /// One must call 'success()' on the returned startup lock guard if shutdown was successful
    /// otherwise the startup lock will not shift to the 'stopped' state.
    pub async fn shutdown(&self) -> Result<StartupLockGuard, StartupLockAlreadyShutDownError> {
        // Drop the stop source to ensure we can detect shutdown has been requested
        *self.stop_source.lock() = None;

        cfg_if! {
            if #[cfg(feature = "debug-locks")] {
                let guard = match timeout(30000, self.startup_state.write()).await {
                    Ok(v) => v,
                    Err(_) => {
                        eprintln!("active guards: {:#?}", self.active_guards.lock().values().collect::<Vec<_>>());
                        panic!("shutdown deadlock");
                    }
                };
            } else {
                let guard = self.startup_state.write().await;
            }
        }
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
        let guard = asyncrwlock_try_read!(self.startup_state).ok_or(StartupLockNotStartedError)?;
        if !*guard {
            return Err(StartupLockNotStartedError);
        }
        let out = StartupLockEnterGuard {
            _guard: guard,
            #[cfg(feature = "debug-locks")]
            id: GUARD_ID.fetch_add(1, Ordering::AcqRel),
            #[cfg(feature = "debug-locks")]
            active_guards: self.active_guards.clone(),
        };

        #[cfg(feature = "debug-locks")]
        self.active_guards
            .lock()
            .insert(out.id, backtrace::Backtrace::new());

        Ok(out)
    }

    /// Enter an operation in a started-up module, using an owned lock.
    /// If this module has not yet started up or is in the process of startup or shutdown
    /// this will fail.
    pub fn enter_arc(&self) -> Result<StartupLockEnterGuardArc, StartupLockNotStartedError> {
        let guard =
            asyncrwlock_try_read_arc!(self.startup_state).ok_or(StartupLockNotStartedError)?;
        if !*guard {
            return Err(StartupLockNotStartedError);
        }
        let out = StartupLockEnterGuardArc {
            _guard: guard,
            #[cfg(feature = "debug-locks")]
            id: GUARD_ID.fetch_add(1, Ordering::AcqRel),
            #[cfg(feature = "debug-locks")]
            active_guards: self.active_guards.clone(),
        };

        #[cfg(feature = "debug-locks")]
        self.active_guards
            .lock()
            .insert(out.id, backtrace::Backtrace::new());

        Ok(out)
    }
}

impl Default for StartupLock {
    fn default() -> Self {
        Self::new()
    }
}
