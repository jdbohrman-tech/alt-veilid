use super::*;

pub type RouterOpId = u64;

#[derive(Debug, Clone, PartialEq, Eq, ThisError)]
pub enum RouterOpWaitError<T> {
    #[error("Send error: {0}")]
    SendError(flume::SendError<T>),
    #[error("Recv error: {0}")]
    RecvError(flume::RecvError),
    #[error("Unmatched operation id: {0}")]
    UnmatchedOpId(RouterOpId),
    #[error("Missing operation id: {0}")]
    MissingOpId(RouterOpId),
}

#[derive(Debug)]
pub struct RouterOpWaitHandle<T, C>
where
    T: Unpin,
    C: Unpin + Clone,
{
    waiter: RouterOpWaiter<T, C>,
    op_id: RouterOpId,
    result_receiver: Option<flume::Receiver<T>>,
}

impl<T, C> Drop for RouterOpWaitHandle<T, C>
where
    T: Unpin,
    C: Unpin + Clone,
{
    fn drop(&mut self) {
        if self.result_receiver.is_some() {
            self.waiter.cancel_op_waiter(self.op_id);
        }
    }
}

#[derive(Debug)]
struct RouterWaitingOp<T, C>
where
    T: Unpin,
    C: Unpin + Clone,
{
    context: C,
    result_sender: flume::Sender<T>,
}

#[derive(Debug)]
struct RouterOpWaiterInner<T, C>
where
    T: Unpin,
    C: Unpin + Clone,
{
    waiting_op_table: HashMap<RouterOpId, RouterWaitingOp<T, C>>,
}

#[derive(Debug)]
pub(super) struct RouterOpWaiter<T, C>
where
    T: Unpin,
    C: Unpin + Clone,
{
    inner: Arc<Mutex<RouterOpWaiterInner<T, C>>>,
}

impl<T, C> Clone for RouterOpWaiter<T, C>
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

impl<T, C> RouterOpWaiter<T, C>
where
    T: Unpin,
    C: Unpin + Clone,
{
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(RouterOpWaiterInner {
                waiting_op_table: HashMap::new(),
            })),
        }
    }

    /// Set up wait for operation to complete
    pub fn add_op_waiter(&self, op_id: RouterOpId, context: C) -> RouterOpWaitHandle<T, C> {
        let mut inner = self.inner.lock();
        let (result_sender, result_receiver) = flume::bounded(1);
        let waiting_op = RouterWaitingOp {
            context,
            result_sender,
        };
        if inner.waiting_op_table.insert(op_id, waiting_op).is_some() {
            error!(
                "add_op_waiter collision should not happen for op_id {}",
                op_id
            );
        }

        RouterOpWaitHandle {
            waiter: self.clone(),
            op_id,
            result_receiver: Some(result_receiver),
        }
    }

    /// Get operation context
    #[expect(dead_code)]
    pub fn get_op_context(&self, op_id: RouterOpId) -> Result<C, RouterOpWaitError<T>> {
        let inner = self.inner.lock();
        let Some(waiting_op) = inner.waiting_op_table.get(&op_id) else {
            return Err(RouterOpWaitError::MissingOpId(op_id));
        };
        Ok(waiting_op.context.clone())
    }

    /// Remove wait for op
    fn cancel_op_waiter(&self, op_id: RouterOpId) {
        let mut inner = self.inner.lock();
        inner.waiting_op_table.remove(&op_id);
    }

    /// Complete the waiting op
    pub fn complete_op_waiter(
        &self,
        op_id: RouterOpId,
        message: T,
    ) -> Result<(), RouterOpWaitError<T>> {
        let waiting_op = {
            let mut inner = self.inner.lock();
            inner
                .waiting_op_table
                .remove(&op_id)
                .ok_or_else(|| RouterOpWaitError::UnmatchedOpId(op_id))?
        };
        waiting_op
            .result_sender
            .send(message)
            .map_err(RouterOpWaitError::SendError)
    }

    /// Wait for operation to complete
    pub async fn wait_for_op(
        &self,
        mut handle: RouterOpWaitHandle<T, C>,
    ) -> Result<T, RouterOpWaitError<T>> {
        // Take the receiver
        // After this, we must manually cancel since the cancel on handle drop is disabled
        let result_receiver = handle.result_receiver.take().unwrap();
        let result_fut = result_receiver.recv_async();

        // wait for eventualvalue
        result_fut.await.map_err(RouterOpWaitError::RecvError)
    }
}
