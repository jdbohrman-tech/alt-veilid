use crate::*;
use core::fmt;
use crypto::*;
use futures_util::stream::{FuturesUnordered, StreamExt};
use network_manager::*;
use routing_table::*;
use stop_token::future::FutureExt;

impl_veilid_log_facility!("receipt");

#[derive(Clone, Debug)]
pub enum ReceiptEvent {
    ReturnedOutOfBand,
    ReturnedInBand {
        inbound_noderef: FilteredNodeRef,
    },
    ReturnedSafety,
    ReturnedPrivate {
        #[expect(dead_code)]
        private_route: PublicKey,
    },
    Expired,
    Cancelled,
}

#[derive(Clone, Debug)]
pub(super) enum ReceiptReturned {
    OutOfBand,
    InBand { inbound_noderef: FilteredNodeRef },
    Safety,
    Private { private_route: PublicKey },
}

pub trait ReceiptCallback: Send + 'static {
    fn call(
        &self,
        event: ReceiptEvent,
        receipt: Receipt,
        returns_so_far: u32,
        expected_returns: u32,
    ) -> PinBoxFutureStatic<()>;
}
impl<F, T> ReceiptCallback for T
where
    T: Fn(ReceiptEvent, Receipt, u32, u32) -> F + Send + 'static,
    F: Future<Output = ()> + Send + 'static,
{
    fn call(
        &self,
        event: ReceiptEvent,
        receipt: Receipt,
        returns_so_far: u32,
        expected_returns: u32,
    ) -> PinBoxFutureStatic<()> {
        Box::pin(self(event, receipt, returns_so_far, expected_returns))
    }
}

type ReceiptCallbackType = Box<dyn ReceiptCallback>;
type ReceiptSingleShotType = SingleShotEventual<ReceiptEvent>;

enum ReceiptRecordCallbackType {
    Normal(ReceiptCallbackType),
    SingleShot(Option<ReceiptSingleShotType>),
}
impl fmt::Debug for ReceiptRecordCallbackType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ReceiptRecordCallbackType::{}",
            match self {
                Self::Normal(_) => "Normal".to_owned(),
                Self::SingleShot(_) => "SingleShot".to_owned(),
            }
        )
    }
}

#[derive(Debug)]
struct ReceiptRecord {
    expiration_ts: Timestamp,
    receipt: Receipt,
    expected_returns: u32,
    returns_so_far: u32,
    receipt_callback: ReceiptRecordCallbackType,
}

impl ReceiptRecord {
    #[expect(dead_code)]
    pub fn new(
        receipt: Receipt,
        expiration_ts: Timestamp,
        expected_returns: u32,
        receipt_callback: impl ReceiptCallback,
    ) -> Self {
        Self {
            expiration_ts,
            receipt,
            expected_returns,
            returns_so_far: 0u32,
            receipt_callback: ReceiptRecordCallbackType::Normal(Box::new(receipt_callback)),
        }
    }

    pub fn new_single_shot(
        receipt: Receipt,
        expiration_ts: Timestamp,
        eventual: ReceiptSingleShotType,
    ) -> Self {
        Self {
            expiration_ts,
            receipt,
            returns_so_far: 0u32,
            expected_returns: 1u32,
            receipt_callback: ReceiptRecordCallbackType::SingleShot(Some(eventual)),
        }
    }
}

/* XXX: may be useful for O(1) timestamp expiration
#[derive(Clone, Debug)]
struct ReceiptRecordTimestampSort {
    expiration_ts: Timestamp,
    record: Arc<Mutex<ReceiptRecord>>,
}

impl PartialEq for ReceiptRecordTimestampSort {
    fn eq(&self, other: &ReceiptRecordTimestampSort) -> bool {
        self.expiration_ts == other.expiration_ts
    }
}
impl Eq for ReceiptRecordTimestampSort {}
impl Ord for ReceiptRecordTimestampSort {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.expiration_ts.cmp(&other.expiration_ts).reverse()
    }
}
impl PartialOrd for ReceiptRecordTimestampSort {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}
*/

///////////////////////////////////

struct ReceiptManagerInner {
    records_by_nonce: BTreeMap<Nonce, Arc<Mutex<ReceiptRecord>>>,
    next_oldest_ts: Option<Timestamp>,
    stop_source: Option<StopSource>,
    timeout_task: MustJoinSingleFuture<()>,
}

struct ReceiptManagerUnlockedInner {
    startup_lock: StartupLock,
}

#[derive(Clone)]
pub(super) struct ReceiptManager {
    registry: VeilidComponentRegistry,
    inner: Arc<Mutex<ReceiptManagerInner>>,
    unlocked_inner: Arc<ReceiptManagerUnlockedInner>,
}

impl_veilid_component_registry_accessor!(ReceiptManager);

impl ReceiptManager {
    fn new_inner() -> ReceiptManagerInner {
        ReceiptManagerInner {
            records_by_nonce: BTreeMap::new(),
            next_oldest_ts: None,
            stop_source: None,
            timeout_task: MustJoinSingleFuture::new(),
        }
    }

    pub fn new(registry: VeilidComponentRegistry) -> Self {
        Self {
            registry,
            inner: Arc::new(Mutex::new(Self::new_inner())),
            unlocked_inner: Arc::new(ReceiptManagerUnlockedInner {
                startup_lock: StartupLock::new(),
            }),
        }
    }

    pub fn startup(&self) -> EyreResult<()> {
        let guard = self.unlocked_inner.startup_lock.startup()?;
        veilid_log!(self debug "startup receipt manager");

        let mut inner = self.inner.lock();
        inner.stop_source = Some(StopSource::new());

        guard.success();
        Ok(())
    }

    #[instrument(level = "trace", target = "receipt", skip_all)]
    fn perform_callback(
        evt: ReceiptEvent,
        record_mut: &mut ReceiptRecord,
    ) -> Option<PinBoxFutureStatic<()>> {
        match &mut record_mut.receipt_callback {
            ReceiptRecordCallbackType::Normal(callback) => Some(callback.call(
                evt,
                record_mut.receipt.clone(),
                record_mut.returns_so_far,
                record_mut.expected_returns,
            )),
            ReceiptRecordCallbackType::SingleShot(eventual) => {
                // resolve this eventual with the receiptevent
                // don't need to wait for the instance to receive it
                // because this can only happen once
                if let Some(eventual) = eventual.take() {
                    eventual.resolve(evt);
                }
                None
            }
        }
    }

    #[instrument(level = "trace", target = "receipt", skip_all)]
    async fn timeout_task_routine(self, now: Timestamp, stop_token: StopToken) {
        // Go through all receipts and build a list of expired nonces
        let mut new_next_oldest_ts: Option<Timestamp> = None;
        let mut expired_records = Vec::new();
        {
            let mut inner = self.inner.lock();
            let mut expired_nonces = Vec::new();
            for (k, v) in &inner.records_by_nonce {
                let receipt_inner = v.lock();
                if receipt_inner.expiration_ts <= now {
                    // Expire this receipt
                    expired_nonces.push(*k);
                } else if new_next_oldest_ts.is_none()
                    || receipt_inner.expiration_ts < new_next_oldest_ts.unwrap()
                {
                    // Mark the next oldest timestamp we would need to take action on as we go through everything
                    new_next_oldest_ts = Some(receipt_inner.expiration_ts);
                }
            }
            if expired_nonces.is_empty() {
                return;
            }
            // Now remove the expired receipts
            for e in expired_nonces {
                let expired_record = inner.records_by_nonce.remove(&e).expect("key should exist");
                expired_records.push(expired_record);
            }
            // Update the next oldest timestamp
            inner.next_oldest_ts = new_next_oldest_ts;
        }
        let mut callbacks = FuturesUnordered::new();
        for expired_record in expired_records {
            let mut expired_record_mut = expired_record.lock();
            if let Some(callback) =
                Self::perform_callback(ReceiptEvent::Expired, &mut expired_record_mut)
            {
                callbacks.push(callback.instrument(Span::current()))
            }
        }

        // Wait on all the multi-call callbacks
        loop {
            if let Ok(None) | Err(_) = callbacks.next().timeout_at(stop_token.clone()).await {
                break;
            }
        }
    }

    #[instrument(
        level = "trace",
        target = "receipt",
        name = "ReceiptManager::tick",
        skip_all,
        err
    )]
    pub async fn tick(&self) -> EyreResult<()> {
        let Ok(_guard) = self.unlocked_inner.startup_lock.enter() else {
            return Ok(());
        };

        let (next_oldest_ts, timeout_task, stop_token) = {
            let inner = self.inner.lock();
            let stop_token = match inner.stop_source.as_ref() {
                Some(ss) => ss.token(),
                None => {
                    // Do nothing if we're shutting down
                    return Ok(());
                }
            };
            (inner.next_oldest_ts, inner.timeout_task.clone(), stop_token)
        };
        let now = Timestamp::now();
        // If we have at least one timestamp to expire, lets do it
        if let Some(next_oldest_ts) = next_oldest_ts {
            if now >= next_oldest_ts {
                // Single-spawn the timeout task routine
                let _ = timeout_task
                    .single_spawn(
                        "receipt timeout",
                        self.clone()
                            .timeout_task_routine(now, stop_token)
                            .instrument(trace_span!(parent: None, "receipt timeout task")),
                    )
                    .in_current_span()
                    .await;
            }
        }
        Ok(())
    }

    pub async fn cancel_tasks(&self) {
        // Stop all tasks
        let timeout_task = {
            let mut inner = self.inner.lock();
            // Drop the stop
            drop(inner.stop_source.take());
            inner.timeout_task.clone()
        };

        // Wait for everything to stop
        veilid_log!(self debug "waiting for timeout task to stop");
        if timeout_task.join().await.is_err() {
            panic!("joining timeout task failed");
        }
    }

    pub async fn shutdown(&self) {
        veilid_log!(self debug "starting receipt manager shutdown");
        let Ok(guard) = self.unlocked_inner.startup_lock.shutdown().await else {
            veilid_log!(self debug "receipt manager is already shut down");
            return;
        };

        *self.inner.lock() = Self::new_inner();

        guard.success();
        veilid_log!(self debug "finished receipt manager shutdown");
    }

    #[instrument(level = "trace", target = "receipt", skip_all)]
    pub fn record_receipt(
        &self,
        receipt: Receipt,
        expiration: Timestamp,
        expected_returns: u32,
        callback: impl ReceiptCallback,
    ) {
        let Ok(_guard) = self.unlocked_inner.startup_lock.enter() else {
            veilid_log!(self debug "ignoring due to not started up");
            return;
        };
        let receipt_nonce = receipt.get_nonce();
        event!(target: "receipt", Level::DEBUG, "== New Multiple Receipt ({}) {} ", expected_returns, receipt_nonce.encode());
        let record = Arc::new(Mutex::new(ReceiptRecord::new(
            receipt,
            expiration,
            expected_returns,
            callback,
        )));
        let mut inner = self.inner.lock();
        inner.records_by_nonce.insert(receipt_nonce, record);

        Self::update_next_oldest_timestamp(&mut inner);
    }

    #[instrument(level = "trace", target = "receipt", skip_all)]
    pub fn record_single_shot_receipt(
        &self,
        receipt: Receipt,
        expiration: Timestamp,
        eventual: ReceiptSingleShotType,
    ) {
        let Ok(_guard) = self.unlocked_inner.startup_lock.enter() else {
            veilid_log!(self debug "ignoring due to not started up");
            return;
        };
        let receipt_nonce = receipt.get_nonce();
        event!(target: "receipt", Level::DEBUG, "== New SingleShot Receipt {}", receipt_nonce.encode());

        let record = Arc::new(Mutex::new(ReceiptRecord::new_single_shot(
            receipt, expiration, eventual,
        )));
        let mut inner = self.inner.lock();
        inner.records_by_nonce.insert(receipt_nonce, record);

        Self::update_next_oldest_timestamp(&mut inner);
    }

    fn update_next_oldest_timestamp(inner: &mut ReceiptManagerInner) {
        // Update the next oldest timestamp
        let mut new_next_oldest_ts: Option<Timestamp> = None;
        for v in inner.records_by_nonce.values() {
            let receipt_inner = v.lock();
            if new_next_oldest_ts.is_none()
                || receipt_inner.expiration_ts < new_next_oldest_ts.unwrap()
            {
                // Mark the next oldest timestamp we would need to take action on as we go through everything
                new_next_oldest_ts = Some(receipt_inner.expiration_ts);
            }
        }

        inner.next_oldest_ts = new_next_oldest_ts;
    }

    #[expect(dead_code)]
    pub async fn cancel_receipt(&self, nonce: &Nonce) -> EyreResult<()> {
        event!(target: "receipt", Level::DEBUG, "== Cancel Receipt {}", nonce.encode());

        let _guard = self.unlocked_inner.startup_lock.enter()?;

        // Remove the record
        let record = {
            let mut inner = self.inner.lock();
            let record = match inner.records_by_nonce.remove(nonce) {
                Some(r) => r,
                None => {
                    bail!("receipt not recorded");
                }
            };
            Self::update_next_oldest_timestamp(&mut inner);
            record
        };

        // Generate a cancelled callback
        let callback_future = {
            let mut record_mut = record.lock();
            Self::perform_callback(ReceiptEvent::Cancelled, &mut record_mut)
        };

        // Issue the callback
        if let Some(callback_future) = callback_future {
            callback_future.await;
        }

        Ok(())
    }

    pub async fn handle_receipt(
        &self,
        receipt: Receipt,
        receipt_returned: ReceiptReturned,
    ) -> NetworkResult<()> {
        let Ok(_guard) = self.unlocked_inner.startup_lock.enter() else {
            return NetworkResult::service_unavailable("receipt manager not started");
        };

        let receipt_nonce = receipt.get_nonce();
        let extra_data = receipt.get_extra_data();

        event!(target: "receipt", Level::DEBUG, "<<== RECEIPT {} <- {}{}",
            receipt_nonce.encode(),
            match receipt_returned {
                ReceiptReturned::OutOfBand => "OutOfBand".to_owned(),
                ReceiptReturned::InBand { ref inbound_noderef } => format!("InBand({})", inbound_noderef),
                ReceiptReturned::Safety => "Safety".to_owned(),
                ReceiptReturned::Private { ref private_route } => format!("Private({})", private_route),
            },
            if extra_data.is_empty() {
                "".to_owned()
            } else {
                format!("[{} extra]", extra_data.len())
            }
        );

        // Increment return count
        let (callback_future, stop_token) = {
            // Look up the receipt record from the nonce
            let mut inner = self.inner.lock();
            let stop_token = match inner.stop_source.as_ref() {
                Some(ss) => ss.token(),
                None => {
                    // If we're stopping do nothing here
                    return NetworkResult::value(());
                }
            };
            let record = match inner.records_by_nonce.get(&receipt_nonce) {
                Some(r) => r.clone(),
                None => {
                    return NetworkResult::invalid_message("receipt not recorded");
                }
            };
            // Generate the callback future
            let mut record_mut = record.lock();
            record_mut.returns_so_far += 1;

            // Get the receipt event to return
            let receipt_event = match receipt_returned {
                ReceiptReturned::OutOfBand => ReceiptEvent::ReturnedOutOfBand,
                ReceiptReturned::Safety => ReceiptEvent::ReturnedSafety,
                ReceiptReturned::InBand { inbound_noderef } => {
                    ReceiptEvent::ReturnedInBand { inbound_noderef }
                }
                ReceiptReturned::Private { private_route } => {
                    ReceiptEvent::ReturnedPrivate { private_route }
                }
            };

            let callback_future = Self::perform_callback(receipt_event, &mut record_mut);

            // Remove the record if we're done
            if record_mut.returns_so_far == record_mut.expected_returns {
                inner.records_by_nonce.remove(&receipt_nonce);

                Self::update_next_oldest_timestamp(&mut inner);
            }
            (callback_future, stop_token)
        };

        // Issue the callback
        if let Some(callback_future) = callback_future {
            let _ = callback_future.timeout_at(stop_token).await;
        }

        NetworkResult::value(())
    }
}
