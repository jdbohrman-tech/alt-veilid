use futures_util::{
    future::{select, Either},
    stream::FuturesUnordered,
    StreamExt,
};
use stop_token::future::FutureExt as _;

use super::*;

#[derive(Debug)]
struct DeferredStreamProcessorInner {
    opt_deferred_stream_channel: Option<flume::Sender<PinBoxFutureStatic<()>>>,
    opt_stopper: Option<StopSource>,
    opt_join_handle: Option<MustJoinHandle<()>>,
}

/// Background processor for streams
/// Handles streams to completion, passing each item from the stream to a callback
#[derive(Debug)]
pub struct DeferredStreamProcessor {
    inner: Mutex<DeferredStreamProcessorInner>,
}

impl DeferredStreamProcessor {
    /// Create a new DeferredStreamProcessor
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(DeferredStreamProcessorInner {
                opt_deferred_stream_channel: None,
                opt_stopper: None,
                opt_join_handle: None,
            }),
        }
    }

    /// Initialize the processor before use
    pub fn init(&self) {
        let stopper = StopSource::new();
        let stop_token = stopper.token();

        let mut inner = self.inner.lock();
        inner.opt_stopper = Some(stopper);
        let (dsc_tx, dsc_rx) = flume::unbounded::<PinBoxFutureStatic<()>>();
        inner.opt_deferred_stream_channel = Some(dsc_tx);
        inner.opt_join_handle = Some(spawn(
            "deferred stream processor",
            Self::processor(stop_token, dsc_rx),
        ));
    }

    /// Terminate the processor and ensure all streams are closed
    pub async fn terminate(&self) {
        let opt_jh = {
            let mut inner = self.inner.lock();
            drop(inner.opt_deferred_stream_channel.take());
            drop(inner.opt_stopper.take());
            inner.opt_join_handle.take()
        };
        if let Some(jh) = opt_jh {
            jh.await;
        }
    }

    async fn processor(stop_token: StopToken, dsc_rx: flume::Receiver<PinBoxFutureStatic<()>>) {
        let mut unord = FuturesUnordered::<PinBoxFutureStatic<()>>::new();

        // Ensure the unord never finishes
        unord.push(Box::pin(std::future::pending()));

        // Processor loop
        let mut unord_fut = unord.next();
        let mut dsc_fut = dsc_rx.recv_async();
        while let Ok(res) = select(unord_fut, dsc_fut)
            .timeout_at(stop_token.clone())
            .await
        {
            match res {
                Either::Left((x, old_dsc_fut)) => {
                    // Unord future processor should never get empty
                    assert!(x.is_some());

                    // Make another unord future to process
                    unord_fut = unord.next();
                    // put back the other future and keep going
                    dsc_fut = old_dsc_fut;
                }
                Either::Right((new_proc, old_unord_fut)) => {
                    // Immediately drop the old unord future
                    // because we never care about it completing
                    drop(old_unord_fut);
                    let Ok(new_proc) = new_proc else {
                        break;
                    };

                    // Add a new stream to process
                    unord.push(new_proc);

                    // Make a new unord future because we don't care about the
                    // completion of the last unord future, they never return
                    // anything.
                    unord_fut = unord.next();
                    // Make a new receiver future
                    dsc_fut = dsc_rx.recv_async();
                }
            }
        }
    }

    /// Queue a stream to process in the background
    ///
    /// * 'receiver' is the stream to process
    /// * 'handler' is the callback to handle each item from the stream
    ///
    /// Returns 'true' if the stream was added for processing, and 'false' if the stream could not be added, possibly due to not being initialized.
    pub fn add<T: Send + 'static, S: futures_util::Stream<Item = T> + Unpin + Send + 'static>(
        &self,
        mut receiver: S,
        mut handler: impl FnMut(T) -> PinBoxFutureStatic<bool> + Send + 'static,
    ) -> bool {
        let (st, dsc_tx) = {
            let inner = self.inner.lock();
            let Some(st) = inner.opt_stopper.as_ref().map(|s| s.token()) else {
                return false;
            };
            let Some(dsc_tx) = inner.opt_deferred_stream_channel.clone() else {
                return false;
            };
            (st, dsc_tx)
        };
        let drp = Box::pin(async move {
            while let Ok(Some(res)) = receiver.next().timeout_at(st.clone()).await {
                if !handler(res).await {
                    break;
                }
            }
        });
        if dsc_tx.send(drp).is_err() {
            return false;
        }
        true
    }
}

impl Default for DeferredStreamProcessor {
    fn default() -> Self {
        Self::new()
    }
}
