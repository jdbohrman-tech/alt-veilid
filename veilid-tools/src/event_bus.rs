//! Event Bus

use super::*;
use futures_util::stream::{FuturesUnordered, StreamExt};
use stop_token::future::FutureExt as _;

use std::any::{Any, TypeId};

type AnyEventHandler =
    Arc<dyn Fn(Arc<dyn Any + Send + Sync>) -> SendPinBoxFuture<()> + Send + Sync>;
type SubscriptionId = u64;

#[derive(Debug)]
pub struct EventBusSubscription {
    id: SubscriptionId,
    type_id: TypeId,
}

struct QueuedEvent {
    evt: Arc<dyn Any + Send + Sync>,
    type_name: &'static str,
}

struct EventBusUnlockedInner {
    tx: flume::Sender<QueuedEvent>,
    rx: flume::Receiver<QueuedEvent>,
    startup_lock: StartupLock,
}

impl fmt::Debug for EventBusUnlockedInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EventBusUnlockedInner")
            .field("tx", &self.tx)
            .field("rx", &self.rx)
            .field("startup_lock", &self.startup_lock)
            .finish()
    }
}

struct EventBusInner {
    handlers: HashMap<TypeId, Vec<(SubscriptionId, AnyEventHandler)>>,
    next_sub_id: SubscriptionId,
    free_sub_ids: Vec<SubscriptionId>,
    stop_source: Option<StopSource>,
    bus_processor_jh: Option<MustJoinHandle<()>>,
}

impl fmt::Debug for EventBusInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EventBusInner")
            .field("handlers.len", &self.handlers.len())
            .field("next_sub_id", &self.next_sub_id)
            .field("free_sub_ids", &self.free_sub_ids)
            .finish()
    }
}

/// Event bus
///
/// Asynchronously handles events of arbitrary Any type
/// by passing them in-order to a set of registered async 'handler' functions.
/// Handlers are processes in an unordered fashion, but an event is fully handled by all handlers
/// until the next event in the posted event stream is processed.
#[derive(Debug, Clone)]
pub struct EventBus {
    unlocked_inner: Arc<EventBusUnlockedInner>,
    inner: Arc<Mutex<EventBusInner>>,
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

impl EventBus {
    ////////////////////////////////////////////////////////////////////
    // Public interface

    /// Create a new EventBus
    pub fn new() -> Self {
        let (tx, rx) = flume::unbounded();
        Self {
            unlocked_inner: Arc::new(EventBusUnlockedInner {
                tx,
                rx,
                startup_lock: StartupLock::new(),
            }),
            inner: Arc::new(Mutex::new(Self::new_inner())),
        }
    }

    /// Start up the EventBus background processor
    pub async fn startup(&self) -> Result<(), StartupLockAlreadyStartedError> {
        let guard = self.unlocked_inner.startup_lock.startup()?;
        {
            let mut inner = self.inner.lock();
            let stop_source = StopSource::new();
            let stop_token = stop_source.token();
            inner.stop_source = Some(stop_source);

            let bus_processor_jh = spawn(
                "event bus processor",
                self.clone().bus_processor(stop_token),
            );
            inner.bus_processor_jh = Some(bus_processor_jh);
        }

        guard.success();
        Ok(())
    }

    /// Shut down EventBus background processing
    /// This unregisters all handlers as well and discards any unprocessed events
    pub async fn shutdown(&self) {
        let Ok(guard) = self.unlocked_inner.startup_lock.shutdown().await else {
            return;
        };

        let opt_jh = {
            let mut inner = self.inner.lock();
            drop(inner.stop_source.take());
            inner.bus_processor_jh.take()
        };

        if let Some(jh) = opt_jh {
            jh.await;
        }

        *self.inner.lock() = Self::new_inner();

        guard.success();
    }

    /// Post an event to be processed
    pub fn post<E: Any + Send + Sync + 'static>(
        &self,
        evt: E,
    ) -> Result<(), StartupLockNotStartedError> {
        let _guard = self.unlocked_inner.startup_lock.enter()?;

        if let Err(e) = self.unlocked_inner.tx.send(QueuedEvent {
            evt: Arc::new(evt),
            type_name: std::any::type_name::<E>(),
        }) {
            error!("{}", e);
        }
        Ok(())
    }

    /// Subscribe a handler to handle all posted events of a particular type
    /// Returns an subscription object that can be used to cancel this specific subscription if desired
    pub fn subscribe<
        E: Any + Send + Sync + 'static,
        F: Fn(Arc<E>) -> SendPinBoxFuture<()> + Send + Sync + 'static,
    >(
        &self,
        handler: F,
    ) -> EventBusSubscription {
        let handler = Arc::new(handler);
        let type_id = TypeId::of::<E>();
        let mut inner = self.inner.lock();

        let id = inner.free_sub_ids.pop().unwrap_or_else(|| {
            let id = inner.next_sub_id;
            inner.next_sub_id += 1;
            id
        });

        inner.handlers.entry(type_id).or_default().push((
            id,
            Arc::new(move |any_evt| {
                let handler = handler.clone();
                Box::pin(async move {
                    handler(any_evt.downcast::<E>().unwrap()).await;
                })
            }),
        ));

        EventBusSubscription { id, type_id }
    }

    /// Given a subscription object returned from `subscribe`, removes the
    /// subscription for the EventBus. The handler will no longer be called.
    pub fn unsubscribe(&self, sub: EventBusSubscription) {
        let mut inner = self.inner.lock();

        inner.handlers.entry(sub.type_id).and_modify(|e| {
            let index = e.iter().position(|x| x.0 == sub.id).unwrap();
            e.remove(index);
        });

        inner.free_sub_ids.push(sub.id);
    }

    /// Returns the number of unprocessed events remaining
    pub fn len(&self) -> usize {
        self.unlocked_inner.rx.len()
    }

    /// Checks if the bus has no events
    pub fn is_empty(&self) -> bool {
        self.unlocked_inner.rx.is_empty()
    }

    ////////////////////////////////////////////////////////////////////
    // Internal implementation

    fn new_inner() -> EventBusInner {
        EventBusInner {
            handlers: HashMap::new(),
            next_sub_id: 0,
            free_sub_ids: vec![],
            stop_source: None,
            bus_processor_jh: None,
        }
    }

    async fn bus_processor(self, stop_token: StopToken) {
        while let Ok(Ok(qe)) = self
            .unlocked_inner
            .rx
            .recv_async()
            .timeout_at(stop_token.clone())
            .await
        {
            let Ok(_guard) = self.unlocked_inner.startup_lock.enter() else {
                break;
            };
            let type_id = (qe.evt.as_ref()).type_id();
            let type_name = qe.type_name;

            let opt_handlers: Option<FuturesUnordered<_>> = {
                let mut inner = self.inner.lock();
                match inner.handlers.entry(type_id) {
                    std::collections::hash_map::Entry::Occupied(entry) => Some(
                        entry
                            .get()
                            .iter()
                            .cloned()
                            .map(|(_id, handler)| handler(qe.evt.clone()))
                            .collect(),
                    ),
                    std::collections::hash_map::Entry::Vacant(_) => {
                        error!("no handlers for event: {}", type_name);
                        None
                    }
                }
            };

            // Process all handlers for this event simultaneously
            if let Some(mut handlers) = opt_handlers {
                while let Ok(Some(_)) = handlers.next().timeout_at(stop_token.clone()).await {}
            }
        }
    }
}
