use super::*;

#[derive(Debug)]
pub struct OperationWaitHandle<T, C>
where
    T: Unpin,
    C: Unpin + Clone,
{
    waiter: OperationWaiter<T, C>,
    op_id: OperationId,
    eventual_instance: Option<EventualValueFuture<(Option<Id>, T)>>,
}

impl<T, C> Drop for OperationWaitHandle<T, C>
where
    T: Unpin,
    C: Unpin + Clone,
{
    fn drop(&mut self) {
        if self.eventual_instance.is_some() {
            self.waiter.cancel_op_waiter(self.op_id);
        }
    }
}

#[derive(Debug)]
pub struct OperationWaitingOp<T, C>
where
    T: Unpin,
    C: Unpin + Clone,
{
    context: C,
    timestamp: Timestamp,
    eventual: EventualValue<(Option<Id>, T)>,
}

#[derive(Debug)]
pub struct OperationWaiterInner<T, C>
where
    T: Unpin,
    C: Unpin + Clone,
{
    waiting_op_table: HashMap<OperationId, OperationWaitingOp<T, C>>,
}

#[derive(Debug)]
pub struct OperationWaiter<T, C>
where
    T: Unpin,
    C: Unpin + Clone,
{
    inner: Arc<Mutex<OperationWaiterInner<T, C>>>,
}

impl<T, C> Clone for OperationWaiter<T, C>
where
    T: Unpin,
    C: Unpin + Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T, C> OperationWaiter<T, C>
where
    T: Unpin,
    C: Unpin + Clone,
{
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(OperationWaiterInner {
                waiting_op_table: HashMap::new(),
            })),
        }
    }

    /// Set up wait for operation to complete
    pub fn add_op_waiter(&self, op_id: OperationId, context: C) -> OperationWaitHandle<T, C> {
        let mut inner = self.inner.lock();
        let e = EventualValue::new();
        let waiting_op = OperationWaitingOp {
            context,
            timestamp: get_aligned_timestamp(),
            eventual: e.clone(),
        };
        if inner.waiting_op_table.insert(op_id, waiting_op).is_some() {
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

    /// Get all waiting operation ids
    pub fn get_operation_ids(&self) -> Vec<OperationId> {
        let inner = self.inner.lock();
        let mut opids: Vec<(OperationId, Timestamp)> = inner
            .waiting_op_table
            .iter()
            .map(|x| (*x.0, x.1.timestamp))
            .collect();
        opids.sort_by(|a, b| a.1.cmp(&b.1));
        opids.into_iter().map(|x| x.0).collect()
    }

    /// Get operation context
    pub fn get_op_context(&self, op_id: OperationId) -> Result<C, RPCError> {
        let inner = self.inner.lock();
        let Some(waiting_op) = inner.waiting_op_table.get(&op_id) else {
            return Err(RPCError::ignore("Missing operation id getting op context"));
        };
        Ok(waiting_op.context.clone())
    }

    /// Remove wait for op
    fn cancel_op_waiter(&self, op_id: OperationId) {
        let mut inner = self.inner.lock();
        inner.waiting_op_table.remove(&op_id);
    }

    /// Complete the app call
    #[cfg_attr(
        feature = "verbose-tracing",
        instrument(level = "trace", skip(self, message), err)
    )]
    pub async fn complete_op_waiter(&self, op_id: OperationId, message: T) -> Result<(), RPCError> {
        let waiting_op = {
            let mut inner = self.inner.lock();
            inner
                .waiting_op_table
                .remove(&op_id)
                .ok_or_else(RPCError::else_internal(format!(
                    "Unmatched operation id: {}",
                    op_id
                )))?
        };
        waiting_op
            .eventual
            .resolve((Span::current().id(), message))
            .await;
        Ok(())
    }

    /// Wait for operation to complete
    pub async fn wait_for_op(
        &self,
        mut handle: OperationWaitHandle<T, C>,
        timeout_us: TimestampDuration,
    ) -> Result<TimeoutOr<(T, TimestampDuration)>, RPCError> {
        let timeout_ms = us_to_ms(timeout_us.as_u64()).map_err(RPCError::internal)?;

        // Take the instance
        // After this, we must manually cancel since the cancel on handle drop is disabled
        let eventual_instance = handle.eventual_instance.take().unwrap();

        // wait for eventualvalue
        let start_ts = get_aligned_timestamp();
        let res = timeout(timeout_ms, eventual_instance)
            .await
            .into_timeout_or();
        Ok(res
            .on_timeout(|| {
                // log_rpc!(debug "op wait timed out: {}", handle.op_id);
                // debug_print_backtrace();
                self.cancel_op_waiter(handle.op_id);
            })
            .map(|res| {
                let (_span_id, ret) = res.take_value().unwrap();
                let end_ts = get_aligned_timestamp();

                //xxx: causes crash (Missing otel data span extensions)
                // Span::current().follows_from(span_id);

                (ret, end_ts.saturating_sub(start_ts))
            }))
    }
}
