use futures_util::StreamExt as _;
use stop_token::future::FutureExt as _;

use super::*;

#[derive(Debug)]
pub(super) enum RelayWorkerRequestKind {
    Relay {
        relay_nr: FilteredNodeRef,
        data: Vec<u8>,
    },
}

#[derive(Debug)]
pub(super) struct RelayWorkerRequest {
    enqueued_ts: Timestamp,
    span: Span,
    kind: RelayWorkerRequestKind,
}

impl NetworkManager {
    pub(super) fn startup_relay_workers(&self) -> EyreResult<()> {
        let mut inner = self.inner.lock();

        // Relay workers
        let channel = flume::bounded(self.queue_size as usize);
        inner.relay_send_channel = Some(channel.0.clone());
        inner.relay_stop_source = Some(StopSource::new());

        // spin up N workers
        veilid_log!(self debug "Starting {} relay workers", self.concurrency);
        for task_n in 0..self.concurrency {
            let registry = self.registry();
            let receiver = channel.1.clone();
            let stop_token = inner.relay_stop_source.as_ref().unwrap().token();
            let jh = spawn(&format!("relay worker {}", task_n), async move {
                let this = registry.network_manager();
                Box::pin(this.relay_worker(stop_token, receiver)).await
            });
            inner.relay_worker_join_handles.push(jh);
        }
        Ok(())
    }

    pub(super) async fn shutdown_relay_workers(&self) {
        // Stop the relay workers
        let mut unord = FuturesUnordered::new();
        {
            let mut inner = self.inner.lock();
            // take the join handles out
            for h in inner.relay_worker_join_handles.drain(..) {
                unord.push(h);
            }
            // drop the stop
            drop(inner.relay_stop_source.take());
        }
        veilid_log!(self debug "Stopping {} relay workers", unord.len());

        // Wait for them to complete
        while unord.next().await.is_some() {}
    }

    pub(super) async fn relay_worker(
        &self,
        stop_token: StopToken,
        receiver: flume::Receiver<RelayWorkerRequest>,
    ) {
        while let Ok(Ok(request)) = receiver.recv_async().timeout_at(stop_token.clone()).await {
            let relay_request_span = tracing::trace_span!("relay request");
            relay_request_span.follows_from(request.span);

            // Measure dequeue time
            let dequeue_ts = Timestamp::now();
            let dequeue_latency = dequeue_ts.saturating_sub(request.enqueued_ts);

            // Process request kind
            match request.kind {
                RelayWorkerRequestKind::Relay { relay_nr, data } => {
                    // Relay the packet to the desired destination
                    veilid_log!(self trace "relaying {} bytes to {}", data.len(), relay_nr);
                    if let Err(e) = pin_future!(self.send_data(relay_nr, data.to_vec())).await {
                        veilid_log!(self debug "failed to relay envelope: {}" ,e);
                    }
                }
            }

            // Measure process time
            let process_ts = Timestamp::now();
            let process_latency = process_ts.saturating_sub(dequeue_ts);

            // Accounting
            self.stats_relay_processed(dequeue_latency, process_latency)
        }
    }

    #[instrument(level = "trace", target = "rpc", skip_all)]
    pub(super) fn enqueue_relay(&self, relay_nr: FilteredNodeRef, data: Vec<u8>) -> EyreResult<()> {
        let _guard = self
            .startup_context
            .startup_lock
            .enter()
            .wrap_err("not started up")?;

        let send_channel = {
            let inner = self.inner.lock();
            let Some(send_channel) = inner.relay_send_channel.as_ref().cloned() else {
                bail!("send channel is closed");
            };
            send_channel
        };
        send_channel
            .try_send(RelayWorkerRequest {
                enqueued_ts: Timestamp::now(),
                span: Span::current(),
                kind: RelayWorkerRequestKind::Relay { relay_nr, data },
            })
            .map_err(|e| eyre!("failed to enqueue relay: {}", e))?;
        Ok(())
    }
}
