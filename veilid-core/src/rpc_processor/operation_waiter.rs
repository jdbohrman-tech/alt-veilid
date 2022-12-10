use super::*;

#[derive(Debug)]
pub struct OperationWaitHandle<T>
where
    T: Unpin,
{
    waiter: OperationWaiter<T>,
    op_id: OperationId,
    eventual_instance: Option<EventualValueFuture<(Option<Id>, T)>>,
}

impl<T> Drop for OperationWaitHandle<T>
where
    T: Unpin,
{
    fn drop(&mut self) {
        if self.eventual_instance.is_some() {
            self.waiter.cancel_op_waiter(self.op_id);
        }
    }
}

#[derive(Debug)]
pub struct OperationWaiterInner<T>
where
    T: Unpin,
{
    waiting_op_table: HashMap<OperationId, EventualValue<(Option<Id>, T)>>,
}

#[derive(Debug)]
pub struct OperationWaiter<T>
where
    T: Unpin,
{
    inner: Arc<Mutex<OperationWaiterInner<T>>>,
}

impl<T> Clone for OperationWaiter<T>
where
    T: Unpin,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> OperationWaiter<T>
where
    T: Unpin,
{
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(OperationWaiterInner {
                waiting_op_table: HashMap::new(),
            })),
        }
    }

    // set up wait for op
    pub fn add_op_waiter(&self, op_id: OperationId) -> OperationWaitHandle<T> {
        let mut inner = self.inner.lock();
        let e = EventualValue::new();
        if inner.waiting_op_table.insert(op_id, e.clone()).is_some() {
            error!(
                "add_op_waiter collision should not happen for op_id {}",
                op_id
            );
        }

        OperationWaitHandle {
            waiter: self.clone(),
            op_id,
            eventual_instance: Some(e.instance()),
        }
    }

    // remove wait for op
    fn cancel_op_waiter(&self, op_id: OperationId) {
        let mut inner = self.inner.lock();
        inner.waiting_op_table.remove(&op_id);
    }

    // complete the app call
    #[instrument(level = "trace", skip(self, message), err)]
    pub async fn complete_op_waiter(&self, op_id: OperationId, message: T) -> Result<(), RPCError> {
        let eventual = {
            let mut inner = self.inner.lock();
            inner
                .waiting_op_table
                .remove(&op_id)
                .ok_or_else(RPCError::else_internal(format!(
                    "Unmatched app call id, possibly too late for timeout: {}",
                    op_id
                )))?
        };
        eventual.resolve((Span::current().id(), message)).await;
        Ok(())
    }

    pub async fn wait_for_op(
        &self,
        mut handle: OperationWaitHandle<T>,
        timeout_us: u64,
    ) -> Result<TimeoutOr<(T, u64)>, RPCError> {
        let timeout_ms = u32::try_from(timeout_us / 1000u64)
            .map_err(|e| RPCError::map_internal("invalid timeout")(e))?;

        // Take the instance
        // After this, we must manually cancel since the cancel on handle drop is disabled
        let eventual_instance = handle.eventual_instance.take().unwrap();

        // wait for eventualvalue
        let start_ts = get_timestamp();
        let res = timeout(timeout_ms, eventual_instance)
            .await
            .into_timeout_or();
        Ok(res
            .on_timeout(|| {
                log_rpc!(debug "op wait timed out: {}", handle.op_id);
                self.cancel_op_waiter(handle.op_id);
            })
            .map(|res| {
                let (_span_id, ret) = res.take_value().unwrap();
                let end_ts = get_timestamp();

                //xxx: causes crash (Missing otel data span extensions)
                // Span::current().follows_from(span_id);

                (ret, end_ts.saturating_sub(start_ts))
            }))
    }
}
