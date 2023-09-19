use crate::*;
use core::fmt;
use crypto::*;
use futures_util::stream::{FuturesUnordered, StreamExt};
use network_manager::*;
use routing_table::*;
use stop_token::future::FutureExt;

#[derive(Clone, Debug)]
pub enum ReceiptEvent {
    ReturnedOutOfBand,
    ReturnedInBand { inbound_noderef: NodeRef },
    ReturnedSafety,
    ReturnedPrivate { private_route: PublicKey },
    Expired,
    Cancelled,
}

#[derive(Clone, Debug)]
pub enum ReceiptReturned {
    OutOfBand,
    InBand { inbound_noderef: NodeRef },
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
    ) -> SendPinBoxFuture<()>;
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
    ) -> SendPinBoxFuture<()> {
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

pub struct ReceiptRecord {
    expiration_ts: Timestamp,
    receipt: Receipt,
    expected_returns: u32,
    returns_so_far: u32,
    receipt_callback: ReceiptRecordCallbackType,
}

impl fmt::Debug for ReceiptRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ReceiptRecord")
            .field("expiration_ts", &self.expiration_ts)
            .field("receipt", &self.receipt)
            .field("expected_returns", &self.expected_returns)
            .field("returns_so_far", &self.returns_so_far)
            .field("receipt_callback", &self.receipt_callback)
            .finish()
    }
}

impl ReceiptRecord {
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

pub struct ReceiptManagerInner {
    network_manager: NetworkManager,
    records_by_nonce: BTreeMap<Nonce, Arc<Mutex<ReceiptRecord>>>,
    next_oldest_ts: Option<Timestamp>,
    stop_source: Option<StopSource>,
    timeout_task: MustJoinSingleFuture<()>,
}

#[derive(Clone)]
pub struct ReceiptManager {
    inner: Arc<Mutex<ReceiptManagerInner>>,
}

impl ReceiptManager {
    fn new_inner(network_manager: NetworkManager) -> ReceiptManagerInner {
        ReceiptManagerInner {
            network_manager,
            records_by_nonce: BTreeMap::new(),
            next_oldest_ts: None,
            stop_source: None,
            timeout_task: MustJoinSingleFuture::new(),
        }
    }

    pub fn new(network_manager: NetworkManager) -> Self {
        Self {
            inner: Arc::new(Mutex::new(Self::new_inner(network_manager))),
        }
    }

    pub fn network_manager(&self) -> NetworkManager {
        self.inner.lock().network_manager.clone()
    }

    pub async fn startup(&self) -> EyreResult<()> {
        trace!("startup receipt manager");
        // Retrieve config

        {
            // let config = self.core().config();
            // let c = config.get();
            let mut inner = self.inner.lock();
            inner.stop_source = Some(StopSource::new());
        }

        Ok(())
    }

    fn perform_callback(
        evt: ReceiptEvent,
        record_mut: &mut ReceiptRecord,
    ) -> Option<SendPinBoxFuture<()>> {
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

    #[instrument(level = "trace", skip(self))]
    pub async fn timeout_task_routine(self, now: Timestamp, stop_token: StopToken) {
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

    pub async fn tick(&self) -> EyreResult<()> {
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
        let now = get_aligned_timestamp();
        // If we have at least one timestamp to expire, lets do it
        if let Some(next_oldest_ts) = next_oldest_ts {
            if now >= next_oldest_ts {
                // Single-spawn the timeout task routine
                let _ = timeout_task
                    .single_spawn(self.clone().timeout_task_routine(now, stop_token))
                    .await;
            }
        }
        Ok(())
    }

    pub async fn shutdown(&self) {
        debug!("starting receipt manager shutdown");
        let network_manager = self.network_manager();

        // Stop all tasks
        let timeout_task = {
            let mut inner = self.inner.lock();
            // Drop the stop
            drop(inner.stop_source.take());
            inner.timeout_task.clone()
        };

        // Wait for everything to stop
        debug!("waiting for timeout task to stop");
        if timeout_task.join().await.is_err() {
            panic!("joining timeout task failed");
        }

        *self.inner.lock() = Self::new_inner(network_manager);
        debug!("finished receipt manager shutdown");
    }

    pub fn record_receipt(
        &self,
        receipt: Receipt,
        expiration: Timestamp,
        expected_returns: u32,
        callback: impl ReceiptCallback,
    ) {
        let receipt_nonce = receipt.get_nonce();
        log_rpc!(debug "== New Multiple Receipt ({}) {} ", expected_returns, receipt_nonce.encode());
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

    pub fn record_single_shot_receipt(
        &self,
        receipt: Receipt,
        expiration: Timestamp,
        eventual: ReceiptSingleShotType,
    ) {
        let receipt_nonce = receipt.get_nonce();
        log_rpc!(debug "== New SingleShot Receipt {}", receipt_nonce.encode());

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

    pub async fn cancel_receipt(&self, nonce: &Nonce) -> EyreResult<()> {
        log_rpc!(debug "== Cancel Receipt {}", nonce.encode());

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
        let receipt_nonce = receipt.get_nonce();
        let extra_data = receipt.get_extra_data();

        log_rpc!(debug "<<== RECEIPT {} <- {}{}",
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
