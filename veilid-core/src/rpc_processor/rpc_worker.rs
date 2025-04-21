use futures_util::StreamExt as _;
use stop_token::future::FutureExt as _;

use super::*;

impl_veilid_log_facility!("rpc");

#[derive(Debug)]
pub(super) enum RPCWorkerRequestKind {
    Message { message_encoded: MessageEncoded },
}

#[derive(Debug)]
pub(super) struct RPCWorkerRequest {
    enqueued_ts: Timestamp,
    span: Span,
    kind: RPCWorkerRequestKind,
}

impl RPCProcessor {
    pub(super) fn startup_rpc_workers(&self) -> EyreResult<()> {
        let mut inner = self.inner.lock();

        // Relay workers
        let channel = flume::bounded(self.queue_size as usize);
        inner.rpc_send_channel = Some(channel.0.clone());
        inner.rpc_stop_source = Some(StopSource::new());

        // spin up N workers
        veilid_log!(self debug "Starting {} RPC workers", self.concurrency);
        for task_n in 0..self.concurrency {
            let registry = self.registry();
            let receiver = channel.1.clone();
            let stop_token = inner.rpc_stop_source.as_ref().unwrap().token();
            let jh = spawn(&format!("relay worker {}", task_n), async move {
                let this = registry.rpc_processor();
                Box::pin(this.rpc_worker(stop_token, receiver)).await
            });
            inner.rpc_worker_join_handles.push(jh);
        }
        Ok(())
    }

    pub(super) async fn shutdown_rpc_workers(&self) {
        // Stop the rpc workers
        let mut unord = FuturesUnordered::new();
        {
            let mut inner = self.inner.lock();
            // take the join handles out
            for h in inner.rpc_worker_join_handles.drain(..) {
                unord.push(h);
            }
            // drop the stop
            drop(inner.rpc_stop_source.take());
        }
        veilid_log!(self debug "Stopping {} RPC workers", unord.len());

        // Wait for them to complete
        while unord.next().await.is_some() {}
    }

    async fn rpc_worker(&self, stop_token: StopToken, receiver: flume::Receiver<RPCWorkerRequest>) {
        while let Ok(Ok(request)) = receiver.recv_async().timeout_at(stop_token.clone()).await {
            let rpc_request_span = tracing::trace_span!("rpc request");
            rpc_request_span.follows_from(request.span);

            // Measure dequeue time
            let dequeue_ts = Timestamp::now();
            let dequeue_latency = dequeue_ts.saturating_sub(request.enqueued_ts);

            // Process request kind
            match request.kind {
                // Process RPC Message
                RPCWorkerRequestKind::Message { message_encoded } => {
                    network_result_value_or_log!(self target:"network_result", match self
                        .process_rpc_message(message_encoded).instrument(rpc_request_span)
                        .await
                    {
                        Err(e) => {
                            veilid_log!(self error "couldn't process rpc message: {}", e);
                            continue;
                        }
                        Ok(v) => {
                            v
                        }
                    } => [ format!(": msg.header={:?}", message_encoded.header) ] {});
                }
            }

            // Measure process time
            let process_ts = Timestamp::now();
            let process_latency = process_ts.saturating_sub(dequeue_ts);

            // Accounting
            let mut inner = self.inner.lock();
            inner.rpc_worker_dequeue_latency = inner
                .rpc_worker_dequeue_latency_accounting
                .record_latency(dequeue_latency);
            inner.rpc_worker_process_latency = inner
                .rpc_worker_process_latency_accounting
                .record_latency(process_latency);
        }
    }

    #[instrument(level = "trace", target = "rpc", skip_all)]
    pub fn enqueue_direct_message(
        &self,
        envelope: Envelope,
        sender_noderef: FilteredNodeRef,
        flow: Flow,
        routing_domain: RoutingDomain,
        body: Vec<u8>,
    ) -> EyreResult<()> {
        let _guard = self
            .startup_context
            .startup_lock
            .enter()
            .wrap_err("not started up")?;

        if sender_noderef.routing_domain_set() != routing_domain {
            bail!("routing domain should match peer noderef filter");
        }

        let header = MessageHeader {
            detail: RPCMessageHeaderDetail::Direct(RPCMessageHeaderDetailDirect {
                envelope,
                sender_noderef,
                flow,
                routing_domain,
            }),
            timestamp: Timestamp::now(),
            body_len: ByteCount::new(body.len() as u64),
        };

        let message_encoded = MessageEncoded {
            header,
            data: MessageData { contents: body },
        };

        let send_channel = {
            let inner = self.inner.lock();
            let Some(send_channel) = inner.rpc_send_channel.as_ref().cloned() else {
                bail!("send channel is closed");
            };
            send_channel
        };
        send_channel
            .try_send(RPCWorkerRequest {
                enqueued_ts: Timestamp::now(),
                span: Span::current(),
                kind: RPCWorkerRequestKind::Message { message_encoded },
            })
            .map_err(|e| eyre!("failed to enqueue direct RPC message: {}", e))?;
        Ok(())
    }

    #[instrument(level = "trace", target = "rpc", skip_all)]
    pub(super) fn enqueue_safety_routed_message(
        &self,
        direct: RPCMessageHeaderDetailDirect,
        remote_safety_route: PublicKey,
        sequencing: Sequencing,
        body: Vec<u8>,
    ) -> EyreResult<()> {
        let _guard = self
            .startup_context
            .startup_lock
            .enter()
            .wrap_err("not started up")?;

        let header = MessageHeader {
            detail: RPCMessageHeaderDetail::SafetyRouted(RPCMessageHeaderDetailSafetyRouted {
                direct,
                remote_safety_route,
                sequencing,
            }),
            timestamp: Timestamp::now(),
            body_len: (body.len() as u64).into(),
        };

        let message_encoded = MessageEncoded {
            header,
            data: MessageData { contents: body },
        };
        let send_channel = {
            let inner = self.inner.lock();
            let Some(send_channel) = inner.rpc_send_channel.as_ref().cloned() else {
                bail!("send channel is closed");
            };
            send_channel
        };
        send_channel
            .try_send(RPCWorkerRequest {
                enqueued_ts: Timestamp::now(),
                span: Span::current(),
                kind: RPCWorkerRequestKind::Message { message_encoded },
            })
            .map_err(|e| eyre!("failed to enqueue safety routed RPC message: {}", e))?;
        Ok(())
    }

    #[instrument(level = "trace", target = "rpc", skip_all)]
    pub(super) fn enqueue_private_routed_message(
        &self,
        direct: RPCMessageHeaderDetailDirect,
        remote_safety_route: PublicKey,
        private_route: PublicKey,
        safety_spec: SafetySpec,
        body: Vec<u8>,
    ) -> EyreResult<()> {
        let _guard = self
            .startup_context
            .startup_lock
            .enter()
            .wrap_err("not started up")?;

        let header = MessageHeader {
            detail: RPCMessageHeaderDetail::PrivateRouted(RPCMessageHeaderDetailPrivateRouted {
                direct,
                remote_safety_route,
                private_route,
                safety_spec,
            }),
            timestamp: Timestamp::now(),
            body_len: (body.len() as u64).into(),
        };

        let message_encoded = MessageEncoded {
            header,
            data: MessageData { contents: body },
        };

        let send_channel = {
            let inner = self.inner.lock();
            let Some(send_channel) = inner.rpc_send_channel.as_ref().cloned() else {
                bail!("send channel is closed");
            };
            send_channel
        };
        send_channel
            .try_send(RPCWorkerRequest {
                enqueued_ts: Timestamp::now(),
                span: Span::current(),
                kind: RPCWorkerRequestKind::Message { message_encoded },
            })
            .map_err(|e| eyre!("failed to enqueue private routed RPC message: {}", e))?;
        Ok(())
    }
}
