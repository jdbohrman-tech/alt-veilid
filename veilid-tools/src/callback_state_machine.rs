use super::*;
pub use rust_fsm;
pub use rust_fsm::*;

pub type StateChangeCallback<T> = Arc<
    dyn Fn(<T as StateMachineImpl>::State, <T as StateMachineImpl>::State) + Send + Sync + 'static,
>;

struct CallbackStateMachineInner<T>
where
    T: StateMachineImpl,
    T::State: Copy + Unpin + core::fmt::Debug,
{
    state: T::State,
    callback: Option<StateChangeCallback<T>>,
    eventual: EventualValueClone<T::State>,
}

impl<T> core::fmt::Debug for CallbackStateMachineInner<T>
where
    T: StateMachineImpl,
    T::State: Copy + Unpin + core::fmt::Debug,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        write!(f, "CallbackStateMachineInner(state: {:?})", self.state)
    }
}

#[derive(Debug, Clone)]
pub struct CallbackStateMachine<T>
where
    T: StateMachineImpl,
    T::State: Copy + Unpin + core::fmt::Debug,
{
    inner: Arc<Mutex<CallbackStateMachineInner<T>>>,
}

impl<T> CallbackStateMachine<T>
where
    T: StateMachineImpl,
    T::State: Copy + Unpin + core::fmt::Debug,
{
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(CallbackStateMachineInner {
                state: T::INITIAL_STATE,
                callback: None,
                eventual: EventualValueClone::new(),
            })),
        }
    }

    pub fn set_state_change_callback(&self, callback: StateChangeCallback<T>) {
        self.inner.lock().callback = Some(callback);
    }

    pub fn clear_state_change_callback(&self) {
        self.inner.lock().callback = None;
    }

    pub fn state_eventual_instance(&self) -> (T::State, EventualValueCloneFuture<T::State>) {
        let inner = self.inner.lock();
        (inner.state, inner.eventual.instance())
    }

    pub async fn consume(&self, input: &T::Input) -> Result<Option<T::Output>, ()> {
        let current_state = self.inner.lock().state;

        if let Some(new_state) = T::transition(&current_state, input) {
            let output = T::output(&current_state, input);
            let old_state = current_state;
            let (callback, eventual) = {
                let mut inner = self.inner.lock();
                inner.state = new_state;
                let eventual =
                    core::mem::replace(&mut inner.eventual, EventualValueClone::<T::State>::new());
                (inner.callback.clone(), eventual)
            };
            if let Some(cb) = callback {
                cb(old_state, new_state);
            }
            eventual.resolve(new_state).await;
            Ok(output)
        } else {
            Err(())
        }
    }

    pub fn state(&self) -> T::State {
        self.inner.lock().state
    }
}

impl<T> Default for CallbackStateMachine<T>
where
    T: StateMachineImpl,
    T::State: Copy + Unpin + core::fmt::Debug,
{
    fn default() -> Self {
        Self::new()
    }
}
