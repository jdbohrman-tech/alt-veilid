use super::*;

mod answer;
mod coders;
mod destination;
mod error;
mod fanout;
mod message;
mod message_header;
mod operation_waiter;
mod rendered_operation;
mod rpc_app_call;
mod rpc_app_message;
mod rpc_find_node;
mod rpc_get_value;
mod rpc_inspect_value;
mod rpc_return_receipt;
mod rpc_route;
mod rpc_set_value;
mod rpc_signal;
mod rpc_status;
mod rpc_validate_dial_info;
mod rpc_value_changed;
mod rpc_watch_value;
mod sender_info;
mod sender_peer_info;

#[cfg(feature = "unstable-blockstore")]
mod rpc_find_block;
#[cfg(feature = "unstable-blockstore")]
mod rpc_supply_block;

#[cfg(feature = "unstable-tunnels")]
mod rpc_cancel_tunnel;
#[cfg(feature = "unstable-tunnels")]
mod rpc_complete_tunnel;
#[cfg(feature = "unstable-tunnels")]
mod rpc_start_tunnel;

pub(crate) use answer::*;
pub(crate) use coders::{
    builder_to_vec, decode_private_route, encode_node_info, encode_private_route, encode_route_hop,
    encode_signed_direct_node_info, encode_typed_key, RPCDecodeContext,
    MAX_INSPECT_VALUE_A_SEQS_LEN,
};
pub(crate) use destination::*;
pub(crate) use error::*;
pub(crate) use fanout::*;
pub(crate) use sender_info::*;

use futures_util::StreamExt;
use stop_token::future::FutureExt as _;

use coders::*;
use message::*;
use message_header::*;
use operation_waiter::*;
use rendered_operation::*;
use sender_peer_info::*;

use crypto::*;
use network_manager::*;
use routing_table::*;
use storage_manager::*;

impl_veilid_log_facility!("rpc");

/////////////////////////////////////////////////////////////////////

#[derive(Debug)]
#[must_use]
struct WaitableReplyContext {
    timeout_us: TimestampDuration,
    node_ref: NodeRef,
    send_ts: Timestamp,
    send_data_result: SendDataResult,
    safety_route: Option<PublicKey>,
    remote_private_route: Option<PublicKey>,
    reply_private_route: Option<PublicKey>,
}

#[derive(Debug)]
#[must_use]
struct WaitableReply {
    handle: OperationWaitHandle<Message, Option<QuestionContext>>,
    _opt_connection_ref_scope: Option<ConnectionRefScope>,
    context: WaitableReplyContext,
}

/////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug)]
#[must_use]
enum RPCKind {
    Question,
    Statement,
    Answer,
}

/////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
#[must_use]
pub struct RPCProcessorStartupContext {
    pub startup_lock: Arc<StartupLock>,
}
impl RPCProcessorStartupContext {
    pub fn new() -> Self {
        Self {
            startup_lock: Arc::new(StartupLock::new()),
        }
    }
}
impl Default for RPCProcessorStartupContext {
    fn default() -> Self {
        Self::new()
    }
}

/////////////////////////////////////////////////////////////////////

#[derive(Debug)]
#[must_use]
struct RPCProcessorInner {
    send_channel: Option<flume::Sender<(Span, MessageEncoded)>>,
    stop_source: Option<StopSource>,
    worker_join_handles: Vec<MustJoinHandle<()>>,
}

#[derive(Debug)]
#[must_use]
pub(crate) struct RPCProcessor {
    registry: VeilidComponentRegistry,
    inner: Mutex<RPCProcessorInner>,
    timeout_us: TimestampDuration,
    queue_size: u32,
    concurrency: u32,
    max_route_hop_count: usize,
    waiting_rpc_table: OperationWaiter<Message, Option<QuestionContext>>,
    waiting_app_call_table: OperationWaiter<Vec<u8>, ()>,
    startup_context: RPCProcessorStartupContext,
}

impl_veilid_component!(RPCProcessor);

impl RPCProcessor {
    fn new_inner() -> RPCProcessorInner {
        RPCProcessorInner {
            send_channel: None,
            stop_source: None,
            worker_join_handles: Vec::new(),
        }
    }

    pub fn new(
        registry: VeilidComponentRegistry,
        startup_context: RPCProcessorStartupContext,
    ) -> Self {
        // make local copy of node id for easy access
        let (concurrency, queue_size, max_route_hop_count, timeout_us) = {
            let config = registry.config();
            let c = config.get();

            // set up channel
            let mut concurrency = c.network.rpc.concurrency;
            let queue_size = c.network.rpc.queue_size;
            let timeout_us = TimestampDuration::new(ms_to_us(c.network.rpc.timeout_ms));
            let max_route_hop_count = c.network.rpc.max_route_hop_count as usize;
            if concurrency == 0 {
                concurrency = get_concurrency();
                if concurrency == 0 {
                    concurrency = 1;
                }

                // Default RPC concurrency is the number of CPUs * 16 rpc workers per core, as a single worker takes about 1% CPU when relaying and 16% is reasonable for baseline plus relay
                concurrency *= 16;
            }
            (concurrency, queue_size, max_route_hop_count, timeout_us)
        };

        Self {
            registry,
            inner: Mutex::new(Self::new_inner()),
            timeout_us,
            queue_size,
            concurrency,
            max_route_hop_count,
            waiting_rpc_table: OperationWaiter::new(),
            waiting_app_call_table: OperationWaiter::new(),
            startup_context,
        }
    }

    /////////////////////////////////////
    /// Initialization

    #[expect(clippy::unused_async)]
    async fn init_async(&self) -> EyreResult<()> {
        Ok(())
    }

    #[expect(clippy::unused_async)]
    async fn post_init_async(&self) -> EyreResult<()> {
        Ok(())
    }

    #[expect(clippy::unused_async)]
    async fn pre_terminate_async(&self) {
        // Ensure things have shut down
        assert!(
            self.startup_context.startup_lock.is_shut_down(),
            "should have shut down by now"
        );
    }

    #[expect(clippy::unused_async)]
    async fn terminate_async(&self) {}

    //////////////////////////////////////////////////////////////////////

    #[instrument(level = "debug", skip_all, err)]
    pub async fn startup(&self) -> EyreResult<()> {
        veilid_log!(self debug "starting rpc processor startup");

        let guard = self.startup_context.startup_lock.startup()?;
        {
            let mut inner = self.inner.lock();

            let channel = flume::bounded(self.queue_size as usize);
            inner.send_channel = Some(channel.0.clone());
            inner.stop_source = Some(StopSource::new());

            // spin up N workers
            veilid_log!(self trace "Spinning up {} RPC workers", self.concurrency);
            for task_n in 0..self.concurrency {
                let registry = self.registry();
                let receiver = channel.1.clone();
                let stop_token = inner.stop_source.as_ref().unwrap().token();
                let jh = spawn(&format!("rpc worker {}", task_n), async move {
                    let this = registry.rpc_processor();
                    Box::pin(this.rpc_worker(stop_token, receiver)).await
                });
                inner.worker_join_handles.push(jh);
            }
        }
        guard.success();

        veilid_log!(self debug "finished rpc processor startup");

        Ok(())
    }

    #[instrument(level = "debug", skip_all)]
    pub async fn shutdown(&self) {
        veilid_log!(self debug "starting rpc processor shutdown");
        let guard = self
            .startup_context
            .startup_lock
            .shutdown()
            .await
            .expect("should be started up");

        // Stop the rpc workers
        let mut unord = FuturesUnordered::new();
        {
            let mut inner = self.inner.lock();
            // take the join handles out
            for h in inner.worker_join_handles.drain(..) {
                unord.push(h);
            }
            // drop the stop
            drop(inner.stop_source.take());
        }
        veilid_log!(self debug "stopping {} rpc worker tasks", unord.len());

        // Wait for them to complete
        while unord.next().await.is_some() {}

        veilid_log!(self debug "resetting rpc processor state");

        // Release the rpc processor
        *self.inner.lock() = Self::new_inner();

        guard.success();
        veilid_log!(self debug "finished rpc processor shutdown");
    }

    //////////////////////////////////////////////////////////////////////

    /// Get waiting app call id for debugging purposes
    pub fn get_app_call_ids(&self) -> Vec<OperationId> {
        self.waiting_app_call_table.get_operation_ids()
    }

    /// Determine if a SignedNodeInfo can be placed into the specified routing domain
    #[instrument(level = "trace", target = "rpc", skip_all)]
    fn verify_node_info(
        &self,
        routing_domain: RoutingDomain,
        signed_node_info: &SignedNodeInfo,
        capabilities: &[Capability],
    ) -> bool {
        let routing_table = self.routing_table();
        routing_table.signed_node_info_is_valid_in_routing_domain(routing_domain, signed_node_info)
            && signed_node_info
                .node_info()
                .has_all_capabilities(capabilities)
    }

    /// Incorporate 'sender peer info' sent along with an RPC message
    #[instrument(level = "trace", target = "rpc", skip_all)]
    fn process_sender_peer_info(
        &self,
        routing_domain: RoutingDomain,
        sender_node_id: TypedKey,
        sender_peer_info: &SenderPeerInfo,
    ) -> RPCNetworkResult<Option<NodeRef>> {
        let Some(peer_info) = sender_peer_info.opt_peer_info.clone() else {
            return Ok(NetworkResult::value(None));
        };

        // Ensure the sender peer info is for the actual sender specified in the envelope
        if !peer_info.node_ids().contains(&sender_node_id) {
            // Attempted to update peer info for the wrong node id
            self.network_manager()
                .address_filter()
                .punish_node_id(sender_node_id, PunishmentReason::WrongSenderPeerInfo);

            return Ok(NetworkResult::invalid_message(
                "attempt to update peer info for non-sender node id",
            ));
        }

        // Sender PeerInfo was specified, update our routing table with it
        if !self.verify_node_info(routing_domain, peer_info.signed_node_info(), &[]) {
            veilid_log!(self debug target:"network_result", "Punishing invalid PeerInfo in {:?} for id {}: {:?}", routing_domain, sender_node_id, peer_info);

            // Punish nodes that send peer info for the wrong routing domain
            // Hairpin NAT situations where routing domain appears to be LocalNetwork
            // shoud not happen. These nodes should be using InboundRelay now to communicate
            // due to the 'node_is_on_same_ipblock' check in PublicInternetRoutigDomainDetail::get_contact_method.
            // Nodes that are actually sending LocalNetwork ip addresses over the PublicInternet domain
            // or vice-versa need to be punished.

            self.network_manager().address_filter().punish_node_id(
                sender_node_id,
                PunishmentReason::FailedToVerifySenderPeerInfo,
            );
            return Ok(NetworkResult::value(None));
        }
        let sender_nr = match self
            .routing_table()
            .register_node_with_peer_info(peer_info.clone(), false)
        {
            Ok(v) => v.unfiltered(),
            Err(e) => {
                self.network_manager().address_filter().punish_node_id(
                    sender_node_id,
                    PunishmentReason::FailedToRegisterSenderPeerInfo,
                );
                return Ok(NetworkResult::invalid_message(e));
            }
        };

        Ok(NetworkResult::value(Some(sender_nr)))
    }

    //////////////////////////////////////////////////////////////////////

    /// Search the public internet routing domain for a single node and add
    /// it to the routing table and return the node reference
    /// If no node was found in the timeout, this returns None
    #[instrument(level = "trace", target = "rpc", skip_all)]
    async fn public_internet_peer_search(
        &self,
        node_id: TypedKey,
        count: usize,
        fanout: usize,
        timeout_us: TimestampDuration,
        safety_selection: SafetySelection,
    ) -> TimeoutOr<Result<Option<NodeRef>, RPCError>> {
        let routing_table = self.routing_table();
        let routing_domain = RoutingDomain::PublicInternet;

        // Ignore own node
        if routing_table.matches_own_node_id(&[node_id]) {
            return TimeoutOr::Value(Err(RPCError::network("can't search for own node id")));
        }

        // Routine to call to generate fanout
        let registry = self.registry();
        let call_routine = Arc::new(move |next_node: NodeRef| {
            let registry = registry.clone();
            Box::pin(async move {
                let this = registry.rpc_processor();
                let v = network_result_try!(
                    this.rpc_call_find_node(
                        Destination::direct(next_node.routing_domain_filtered(routing_domain))
                            .with_safety(safety_selection),
                        node_id,
                        vec![],
                    )
                    .await?
                );
                Ok(NetworkResult::value(FanoutCallOutput {
                    peer_info_list: v.answer,
                }))
            }) as PinBoxFuture<FanoutCallResult>
        });

        // Routine to call to check if we're done at each step
        let check_done = Arc::new(move |_: &[NodeRef]| {
            let Ok(Some(nr)) = routing_table.lookup_node_ref(node_id) else {
                return None;
            };

            // ensure we have some dial info for the entry already,
            // and that the node is still alive
            // if not, we should keep looking for better info
            if nr.state(Timestamp::now()).is_alive() && nr.has_any_dial_info() {
                return Some(nr);
            }

            None
        });

        // Call the fanout
        let routing_table = self.routing_table();
        let fanout_call = FanoutCall::new(
            &routing_table,
            node_id,
            count,
            fanout,
            timeout_us,
            empty_fanout_node_info_filter(),
            call_routine,
            check_done,
        );

        fanout_call.run(vec![]).await
    }

    /// Search the DHT for a specific node corresponding to a key unless we
    /// have that node in our routing table already, and return the node reference
    /// Note: This routine can possibly be recursive, hence the PinBoxFuture async form
    #[instrument(level = "trace", target = "rpc", skip_all)]
    pub fn resolve_node(
        &self,
        node_id: TypedKey,
        safety_selection: SafetySelection,
    ) -> PinBoxFuture<Result<Option<NodeRef>, RPCError>> {
        let registry = self.registry();
        Box::pin(
            async move {
                let this = registry.rpc_processor();

                let _guard = this
                    .startup_context
                    .startup_lock
                    .enter()
                    .map_err(RPCError::map_try_again("not started up"))?;

                let routing_table = this.routing_table();

                // First see if we have the node in our routing table already
                let mut existing_nr = None;
                if let Some(nr) = routing_table
                    .lookup_node_ref(node_id)
                    .map_err(RPCError::internal)?
                {
                    existing_nr = Some(nr.clone());

                    // ensure we have some dial info for the entry already,
                    // and that the node is still alive
                    // if not, we should do the find_node anyway
                    if nr.state(Timestamp::now()).is_alive() && nr.has_any_dial_info() {
                        return Ok(Some(nr));
                    }
                }

                // If nobody knows where this node is, ask the DHT for it
                let (node_count, _consensus_count, fanout, timeout) = this.config().with(|c| {
                    (
                        c.network.dht.max_find_node_count as usize,
                        c.network.dht.resolve_node_count as usize,
                        c.network.dht.resolve_node_fanout as usize,
                        TimestampDuration::from(ms_to_us(c.network.dht.resolve_node_timeout_ms)),
                    )
                });

                // Search routing domains for peer
                // xxx: Eventually add other routing domains here
                let nr = match this
                    .public_internet_peer_search(
                        node_id,
                        node_count,
                        fanout,
                        timeout,
                        safety_selection,
                    )
                    .await
                {
                    TimeoutOr::Timeout => None,
                    TimeoutOr::Value(Ok(v)) => v,
                    TimeoutOr::Value(Err(e)) => {
                        return Err(e);
                    }
                };

                // Either return the node we just resolved or a dead one we found in the routing table to try again
                Ok(nr.or(existing_nr))
            }
            .in_current_span(),
        )
    }

    #[instrument(level = "trace", target = "rpc", skip_all)]
    async fn wait_for_reply(
        &self,
        waitable_reply: WaitableReply,
        debug_string: String,
    ) -> Result<TimeoutOr<(Message, TimestampDuration)>, RPCError> {
        let id = waitable_reply.handle.id();
        let out = self
            .waiting_rpc_table
            .wait_for_op(waitable_reply.handle, waitable_reply.context.timeout_us)
            .await;
        match &out {
            Err(e) => {
                veilid_log!(self debug "RPC Lost (id={} {}): {} ({}) ", id, debug_string, e, waitable_reply.context.send_data_result.unique_flow().flow);
                self.record_lost_answer(&waitable_reply.context);
            }
            Ok(TimeoutOr::Timeout) => {
                veilid_log!(self debug "RPC Lost (id={} {}): Timeout ({})", id, debug_string, waitable_reply.context.send_data_result.unique_flow().flow);
                self.record_lost_answer(&waitable_reply.context);
            }
            Ok(TimeoutOr::Value((rpcreader, _))) => {
                // Reply received
                let recv_ts = Timestamp::now();

                // Record answer received
                self.record_answer_received(
                    recv_ts,
                    rpcreader.header.body_len,
                    &waitable_reply.context,
                );

                // Ensure the reply comes over the private route that was requested
                if let Some(reply_private_route) = waitable_reply.context.reply_private_route {
                    match &rpcreader.header.detail {
                        RPCMessageHeaderDetail::Direct(_) => {
                            return Err(RPCError::protocol(
                                "should have received reply over private route or stub",
                            ));
                        }
                        RPCMessageHeaderDetail::SafetyRouted(sr) => {
                            let node_id = self
                                .routing_table()
                                .node_id(sr.direct.envelope.get_crypto_kind());
                            if node_id.value != reply_private_route {
                                return Err(RPCError::protocol(
                                    "should have received reply from safety route to a stub",
                                ));
                            }
                        }
                        RPCMessageHeaderDetail::PrivateRouted(pr) => {
                            if pr.private_route != reply_private_route {
                                return Err(RPCError::protocol(
                                    "received reply over the wrong private route",
                                ));
                            }
                        }
                    };
                }
            }
        };
        out
    }

    /// Wrap an operation with a private route inside a safety route
    #[instrument(level = "trace", target = "rpc", skip_all)]
    fn wrap_with_route(
        &self,
        routing_domain: RoutingDomain,
        safety_selection: SafetySelection,
        remote_private_route: PrivateRoute,
        reply_private_route: Option<PublicKey>,
        message_data: Vec<u8>,
    ) -> RPCNetworkResult<RenderedOperation> {
        let routing_table = self.routing_table();
        let crypto = self.crypto();
        let rss = routing_table.route_spec_store();

        // Get useful private route properties
        let pr_is_stub = remote_private_route.is_stub();
        let pr_hop_count = remote_private_route.hop_count;
        let pr_pubkey = remote_private_route.public_key.value;
        let crypto_kind = remote_private_route.crypto_kind();
        let Some(vcrypto) = crypto.get(crypto_kind) else {
            return Err(RPCError::internal(
                "crypto not available for selected private route",
            ));
        };

        // Compile the safety route with the private route
        let compiled_route: CompiledRoute = network_result_try!(rss
            .compile_safety_route(safety_selection, remote_private_route)
            .to_rpc_network_result()?);
        let sr_is_stub = compiled_route.safety_route.is_stub();
        let sr_pubkey = compiled_route.safety_route.public_key.value;

        // Encrypt routed operation
        // Xmsg + ENC(Xmsg, DH(PKapr, SKbsr))
        let nonce = vcrypto.random_nonce();
        let dh_secret = vcrypto
            .cached_dh(&pr_pubkey, &compiled_route.secret)
            .map_err(RPCError::map_internal("dh failed"))?;
        let enc_msg_data = vcrypto
            .encrypt_aead(&message_data, &nonce, &dh_secret, None)
            .map_err(RPCError::map_internal("encryption failed"))?;

        // Make the routed operation
        let operation = RoutedOperation::new(
            routing_domain,
            safety_selection.get_sequencing(),
            nonce,
            enc_msg_data,
        );

        // Prepare route operation
        let sr_hop_count = compiled_route.safety_route.hop_count;
        let route_operation = RPCOperationRoute::new(compiled_route.safety_route, operation);
        let ssni_route =
            self.get_sender_peer_info(&Destination::direct(compiled_route.first_hop.clone()));
        let operation = RPCOperation::new_statement(
            RPCStatement::new(RPCStatementDetail::Route(Box::new(route_operation))),
            ssni_route,
        );

        // Convert message to bytes and return it
        let mut route_msg = ::capnp::message::Builder::new_default();
        let mut route_operation = route_msg.init_root::<veilid_capnp::operation::Builder>();
        operation.encode(&mut route_operation)?;
        let out_message = builder_to_vec(route_msg)?;

        // Get the first hop this is going to
        let out_hop_count = (1 + sr_hop_count + pr_hop_count) as usize;

        let out = RenderedOperation {
            message: out_message,
            destination_node_ref: compiled_route.first_hop.unfiltered(),
            node_ref: compiled_route.first_hop,
            hop_count: out_hop_count,
            safety_route: if sr_is_stub { None } else { Some(sr_pubkey) },
            remote_private_route: if pr_is_stub { None } else { Some(pr_pubkey) },
            reply_private_route,
        };

        Ok(NetworkResult::value(out))
    }

    /// Produce a byte buffer that represents the wire encoding of the entire
    /// unencrypted envelope body for a RPC message. This incorporates
    /// wrapping a private and/or safety route if they are specified.
    #[instrument(level = "trace", target = "rpc", skip_all)]
    fn render_operation(
        &self,
        dest: Destination,
        operation: &RPCOperation,
    ) -> RPCNetworkResult<RenderedOperation> {
        let out: NetworkResult<RenderedOperation>;

        // Encode message to a builder and make a message reader for it
        // Then produce the message as an unencrypted byte buffer
        let message = {
            let mut msg_builder = ::capnp::message::Builder::new_default();
            let mut op_builder = msg_builder.init_root::<veilid_capnp::operation::Builder>();
            operation.encode(&mut op_builder)?;
            builder_to_vec(msg_builder)?
        };

        // Get reply private route if we are asking for one to be used in our 'respond to'
        let reply_private_route = match operation.kind() {
            RPCOperationKind::Question(q) => match q.respond_to() {
                RespondTo::Sender => None,
                RespondTo::PrivateRoute(pr) => Some(pr.public_key.value),
            },
            RPCOperationKind::Statement(_) | RPCOperationKind::Answer(_) => None,
        };

        // To where are we sending the request
        match dest {
            Destination::Direct {
                node: ref node_ref,
                safety_selection,
            }
            | Destination::Relay {
                relay: ref node_ref,
                node: _,
                safety_selection,
            } => {
                // Send to a node without a private route
                // --------------------------------------

                // Get the actual destination node id accounting for relays
                let (node_ref, destination_node_ref) = if let Destination::Relay {
                    relay: _,
                    node: ref target,
                    safety_selection: _,
                } = dest
                {
                    (node_ref.clone(), target.clone())
                } else {
                    (node_ref.clone(), node_ref.unfiltered())
                };

                // Handle the existence of safety route
                match safety_selection {
                    SafetySelection::Unsafe(sequencing) => {
                        // Apply safety selection sequencing requirement if it is more strict than the node_ref's sequencing requirement
                        let mut node_ref = node_ref.clone();
                        if sequencing > node_ref.sequencing() {
                            node_ref.set_sequencing(sequencing)
                        }

                        // Reply private route should be None here, even for questions
                        assert!(reply_private_route.is_none());

                        // If no safety route is being used, and we're not sending to a private
                        // route, we can use a direct envelope instead of routing
                        out = NetworkResult::value(RenderedOperation {
                            message,
                            destination_node_ref,
                            node_ref,
                            hop_count: 1,
                            safety_route: None,
                            remote_private_route: None,
                            reply_private_route: None,
                        });
                    }
                    SafetySelection::Safe(_) => {
                        // For now we only private-route over PublicInternet
                        let routing_domain = RoutingDomain::PublicInternet;

                        // No private route was specified for the request
                        // but we are using a safety route, so we must create an empty private route
                        // Destination relay is ignored for safety routed operations
                        let peer_info = match destination_node_ref.get_peer_info(routing_domain) {
                            None => {
                                return Ok(NetworkResult::no_connection_other(
                                    "No peer info for stub private route",
                                ))
                            }
                            Some(pi) => pi,
                        };
                        let private_route = PrivateRoute::new_stub(
                            destination_node_ref.best_node_id(),
                            RouteNode::PeerInfo(peer_info),
                        );

                        // Wrap with safety route
                        out = self.wrap_with_route(
                            routing_domain,
                            safety_selection,
                            private_route,
                            reply_private_route,
                            message,
                        )?;
                    }
                };
            }
            Destination::PrivateRoute {
                private_route,
                safety_selection,
            } => {
                // For now we only private-route over PublicInternet
                let routing_domain = RoutingDomain::PublicInternet;

                // Send to private route
                // ---------------------
                // Reply with 'route' operation
                out = self.wrap_with_route(
                    routing_domain,
                    safety_selection,
                    private_route,
                    reply_private_route,
                    message,
                )?;
            }
        }

        Ok(out)
    }

    /// Get signed node info to package with RPC messages to improve
    /// routing table caching when it is okay to do so
    /// Also check target's timestamp of our own node info, to see if we should send that
    /// And send our timestamp of the target's node info so they can determine if they should update us on their next rpc
    #[instrument(level = "trace", target = "rpc", skip_all)]
    fn get_sender_peer_info(&self, dest: &Destination) -> SenderPeerInfo {
        let routing_table = self.routing_table();
        // Don't do this if the sender is to remain private
        // Otherwise we would be attaching the original sender's identity to the final destination,
        // thus defeating the purpose of the safety route entirely :P
        let Some(UnsafeRoutingInfo {
            opt_node,
            opt_relay: _,
            opt_routing_domain,
        }) = dest.get_unsafe_routing_info(&routing_table)
        else {
            return SenderPeerInfo::default();
        };
        let Some(node) = opt_node else {
            // If this is going over a private route, don't bother sending any sender peer info
            // The other side won't accept it because peer info sent over a private route
            // could be used to deanonymize the private route's endpoint
            return SenderPeerInfo::default();
        };
        let Some(routing_domain) = opt_routing_domain else {
            // No routing domain for target, no node info is safe to send here
            // Only a stale connection or no connection exists, or an unexpected
            // relay was used, possibly due to the destination switching relays
            // in a race condition with our send
            return SenderPeerInfo::default();
        };

        // Get the target's node info timestamp
        let target_node_info_ts = node.node_info_ts(routing_domain);

        // Return whatever peer info we have even if the network class is not yet valid
        // That away we overwrite any prior existing valid-network-class nodeinfo in the remote routing table
        let routing_table = self.routing_table();

        if let Some(published_peer_info) = routing_table.get_published_peer_info(routing_domain) {
            // If the target has not yet seen our published peer info, send it along if we have it
            if !node.has_seen_our_node_info_ts(routing_domain) {
                return SenderPeerInfo::new(published_peer_info, target_node_info_ts);
            }
        }
        SenderPeerInfo::new_no_peer_info(target_node_info_ts)
    }

    /// Record failure to send to node or route
    #[instrument(level = "trace", target = "rpc", skip_all)]
    fn record_send_failure(
        &self,
        rpc_kind: RPCKind,
        send_ts: Timestamp,
        node_ref: NodeRef,
        safety_route: Option<PublicKey>,
        remote_private_route: Option<PublicKey>,
    ) {
        let wants_answer = matches!(rpc_kind, RPCKind::Question);

        // Record for node if this was not sent via a route
        if safety_route.is_none() && remote_private_route.is_none() {
            node_ref.stats_failed_to_send(send_ts, wants_answer);

            // Also clear the last_connections for the entry so we make a new connection next time
            node_ref.clear_last_flows();

            return;
        }

        // If safety route was in use, record failure to send there
        if let Some(sr_pubkey) = &safety_route {
            self.routing_table()
                .route_spec_store()
                .with_route_stats_mut(send_ts, sr_pubkey, |s| s.record_send_failed());
        } else {
            // If no safety route was in use, then it's the private route's fault if we have one
            if let Some(pr_pubkey) = &remote_private_route {
                self.routing_table()
                    .route_spec_store()
                    .with_route_stats_mut(send_ts, pr_pubkey, |s| s.record_send_failed());
            }
        }
    }

    /// Record question lost to node or route
    #[instrument(level = "trace", target = "rpc", skip_all)]
    fn record_lost_answer(&self, context: &WaitableReplyContext) {
        // Record for node if this was not sent via a route
        if context.safety_route.is_none() && context.remote_private_route.is_none() {
            context
                .node_ref
                .stats_lost_answer(context.send_data_result.is_ordered());

            // Also clear the last_connections for the entry so we make a new connection next time
            context.node_ref.clear_last_flows();

            return;
        }
        // Get route spec store
        let routing_table = self.routing_table();
        let rss = routing_table.route_spec_store();

        // If safety route was used, record question lost there
        if let Some(sr_pubkey) = &context.safety_route {
            rss.with_route_stats_mut(context.send_ts, sr_pubkey, |s| {
                s.record_lost_answer();
            });
        }
        // If remote private route was used, record question lost there
        if let Some(rpr_pubkey) = &context.remote_private_route {
            rss.with_route_stats_mut(context.send_ts, rpr_pubkey, |s| {
                s.record_lost_answer();
            });
        }
        // If reply private route was used, record question lost there
        if let Some(pr_pubkey) = &context.reply_private_route {
            rss.with_route_stats_mut(context.send_ts, pr_pubkey, |s| {
                s.record_lost_answer();
            });
        }
    }

    /// Record success sending to node or route
    #[instrument(level = "trace", target = "rpc", skip_all)]
    #[expect(clippy::too_many_arguments)]
    fn record_send_success(
        &self,
        rpc_kind: RPCKind,
        send_ts: Timestamp,
        bytes: ByteCount,
        node_ref: NodeRef,
        safety_route: Option<PublicKey>,
        remote_private_route: Option<PublicKey>,
        ordered: bool,
    ) {
        // Record for node if this was not sent via a route
        if safety_route.is_none() && remote_private_route.is_none() {
            let wants_answer = matches!(rpc_kind, RPCKind::Question);
            let is_answer = matches!(rpc_kind, RPCKind::Answer);

            if is_answer {
                node_ref.stats_answer_sent(bytes);
            } else {
                node_ref.stats_question_sent(send_ts, bytes, wants_answer, ordered);
            }
            return;
        }

        // Get route spec store
        let routing_table = self.routing_table();
        let rss = routing_table.route_spec_store();

        // If safety route was used, record send there
        if let Some(sr_pubkey) = &safety_route {
            rss.with_route_stats_mut(send_ts, sr_pubkey, |s| {
                s.record_sent(send_ts, bytes);
            });
        }

        // If remote private route was used, record send there
        if let Some(pr_pubkey) = &remote_private_route {
            rss.with_route_stats_mut(send_ts, pr_pubkey, |s| {
                s.record_sent(send_ts, bytes);
            });
        }
    }

    /// Record answer received from node or route
    #[allow(clippy::too_many_arguments)]
    #[instrument(level = "trace", target = "rpc", skip_all)]
    fn record_answer_received(
        &self,
        recv_ts: Timestamp,
        bytes: ByteCount,
        context: &WaitableReplyContext,
    ) {
        // Record stats for remote node if this was direct
        if context.safety_route.is_none()
            && context.remote_private_route.is_none()
            && context.reply_private_route.is_none()
        {
            context.node_ref.stats_answer_rcvd(
                context.send_ts,
                recv_ts,
                bytes,
                context.send_data_result.is_ordered(),
            );
            return;
        }
        // Get route spec store
        let routing_table = self.routing_table();
        let rss = routing_table.route_spec_store();

        // Get latency for all local routes
        let mut total_local_latency = TimestampDuration::new(0u64);
        let total_latency: TimestampDuration = recv_ts.saturating_sub(context.send_ts);

        // If safety route was used, record route there
        if let Some(sr_pubkey) = &context.safety_route {
            rss.with_route_stats_mut(context.send_ts, sr_pubkey, |s| {
                // Record received bytes
                s.record_answer_received(recv_ts, bytes);

                // If we used a safety route to send, use our last tested latency
                total_local_latency += s.latency_stats().average
            });
        }

        // If local private route was used, record route there
        if let Some(pr_pubkey) = &context.reply_private_route {
            rss.with_route_stats_mut(context.send_ts, pr_pubkey, |s| {
                // Record received bytes
                s.record_answer_received(recv_ts, bytes);

                // If we used a private route to receive, use our last tested latency
                total_local_latency += s.latency_stats().average
            });
        }

        // If remote private route was used, record there
        if let Some(rpr_pubkey) = &context.remote_private_route {
            rss.with_route_stats_mut(context.send_ts, rpr_pubkey, |s| {
                // Record received bytes
                s.record_answer_received(recv_ts, bytes);

                // The remote route latency is recorded using the total latency minus the total local latency
                let remote_latency = total_latency.saturating_sub(total_local_latency);
                s.record_latency(remote_latency);
            });

            // If we sent to a private route without a safety route
            // We need to mark our own node info as having been seen so we can optimize sending it
            if let Err(e) = rss.mark_remote_private_route_seen_our_node_info(rpr_pubkey, recv_ts) {
                veilid_log!(self error "private route missing: {}", e);
            }

            // We can't record local route latency if a remote private route was used because
            // there is no way other than the prior latency estimation to determine how much time was spent
            // in the remote private route
            // Instead, we rely on local route testing to give us latency numbers for our local routes
        } else {
            // If no remote private route was used, then record half the total latency on our local routes
            // This is fine because if we sent with a local safety route,
            // then we must have received with a local private route too, per the design rules
            if let Some(sr_pubkey) = &context.safety_route {
                rss.with_route_stats_mut(context.send_ts, sr_pubkey, |s| {
                    s.record_latency(total_latency / 2u64);
                });
            }
            if let Some(pr_pubkey) = &context.reply_private_route {
                rss.with_route_stats_mut(context.send_ts, pr_pubkey, |s| {
                    s.record_latency(total_latency / 2u64);
                });
            }
        }
    }

    /// Record question or statement received from node or route
    #[instrument(level = "trace", target = "rpc", skip_all)]
    fn record_question_received(&self, msg: &Message) {
        let recv_ts = msg.header.timestamp;
        let bytes = msg.header.body_len;

        let routing_table = self.routing_table();
        let rss = routing_table.route_spec_store();

        // Process messages based on how they were received
        match &msg.header.detail {
            // Process direct messages
            RPCMessageHeaderDetail::Direct(_) => {
                if let Some(sender_nr) = msg.opt_sender_nr.clone() {
                    sender_nr.stats_question_rcvd(recv_ts, bytes);
                }
            }
            // Process messages that arrived with no private route (private route stub)
            RPCMessageHeaderDetail::SafetyRouted(d) => {
                // This may record nothing if the remote safety route is not also
                // a remote private route that been imported, but that's okay
                rss.with_route_stats_mut(recv_ts, &d.remote_safety_route, |s| {
                    s.record_question_received(recv_ts, bytes);
                });
            }
            // Process messages that arrived to our private route
            RPCMessageHeaderDetail::PrivateRouted(d) => {
                // This may record nothing if the remote safety route is not also
                // a remote private route that been imported, but that's okay
                // it could also be a node id if no remote safety route was used
                // in which case this also will do nothing
                rss.with_route_stats_mut(recv_ts, &d.remote_safety_route, |s| {
                    s.record_question_received(recv_ts, bytes);
                });

                // Record for our local private route we received over
                rss.with_route_stats_mut(recv_ts, &d.private_route, |s| {
                    s.record_question_received(recv_ts, bytes);
                });
            }
        }
    }

    /// Issue a question over the network, possibly using an anonymized route
    /// Optionally keeps a context to be passed to the answer processor when an answer is received
    #[instrument(level = "trace", target = "rpc", skip_all)]
    async fn question(
        &self,
        dest: Destination,
        question: RPCQuestion,
        context: Option<QuestionContext>,
    ) -> RPCNetworkResult<WaitableReply> {
        // Get sender peer info if we should send that
        let spi = self.get_sender_peer_info(&dest);

        // Wrap question in operation
        let operation = RPCOperation::new_question(question, spi);
        let op_id = operation.op_id();

        // Log rpc send
        veilid_log!(self debug target: "rpc_message", dir = "send", kind = "question", op_id = op_id.as_u64(), desc = operation.kind().desc(), ?dest);

        // Produce rendered operation
        let RenderedOperation {
            message,
            destination_node_ref,
            node_ref,
            hop_count,
            safety_route,
            remote_private_route,
            reply_private_route,
        } = network_result_try!(self.render_operation(dest.clone(), &operation)?);

        // Calculate answer timeout
        // Timeout is number of hops times the timeout per hop
        let timeout_us = self.timeout_us * (hop_count as u64);

        // Set up op id eventual
        let handle = self.waiting_rpc_table.add_op_waiter(op_id, context);

        // Send question
        let bytes: ByteCount = (message.len() as u64).into();
        #[allow(unused_variables)]
        let message_len = message.len();
        let res = self
            .network_manager()
            .send_envelope(
                node_ref.clone(),
                Some(destination_node_ref.clone()),
                message,
            )
            .await
            .map_err(|e| {
                // If we're returning an error, clean up
                let send_ts = Timestamp::now();
                self.record_send_failure(
                    RPCKind::Question,
                    send_ts,
                    node_ref.unfiltered(),
                    safety_route,
                    remote_private_route,
                );
                RPCError::network(e)
            })?;
        // Take send timestamp -after- send is attempted to exclude TCP connection time which
        // may unfairly punish some nodes, randomly, based on their being in the connection table or not
        let send_ts = Timestamp::now();
        let send_data_result = network_result_value_or_log!(self res => [ format!(": node_ref={}, destination_node_ref={}, message.len={}", node_ref, destination_node_ref, message_len) ] {
                // If we couldn't send we're still cleaning up
                self.record_send_failure(RPCKind::Question, send_ts, node_ref.unfiltered(), safety_route, remote_private_route);
                network_result_raise!(res);
            }
        );

        // Successfully sent
        self.record_send_success(
            RPCKind::Question,
            send_ts,
            bytes,
            node_ref.unfiltered(),
            safety_route,
            remote_private_route,
            send_data_result.is_ordered(),
        );

        // Ref the connection so it doesn't go away until we're done with the waitable reply
        let opt_connection_ref_scope =
            send_data_result.unique_flow().connection_id.and_then(|id| {
                self.network_manager()
                    .connection_manager()
                    .try_connection_ref_scope(id)
            });

        // Pass back waitable reply completion
        Ok(NetworkResult::value(WaitableReply {
            handle,
            _opt_connection_ref_scope: opt_connection_ref_scope,
            context: WaitableReplyContext {
                timeout_us,
                node_ref: node_ref.unfiltered(),
                send_ts,
                send_data_result,
                safety_route,
                remote_private_route,
                reply_private_route,
            },
        }))
    }

    /// Issue a statement over the network, possibly using an anonymized route
    #[instrument(level = "trace", target = "rpc", skip_all)]
    async fn statement(&self, dest: Destination, statement: RPCStatement) -> RPCNetworkResult<()> {
        // Get sender peer info if we should send that
        let spi = self.get_sender_peer_info(&dest);

        // Wrap statement in operation
        let operation = RPCOperation::new_statement(statement, spi);

        // Log rpc send
        veilid_log!(self debug target: "rpc_message", dir = "send", kind = "statement", op_id = operation.op_id().as_u64(), desc = operation.kind().desc(), ?dest);

        // Produce rendered operation
        let RenderedOperation {
            message,
            destination_node_ref,
            node_ref,
            hop_count: _,
            safety_route,
            remote_private_route,
            reply_private_route: _,
        } = network_result_try!(self.render_operation(dest, &operation)?);

        // Send statement
        let bytes: ByteCount = (message.len() as u64).into();
        #[allow(unused_variables)]
        let message_len = message.len();
        let res = self
            .network_manager()
            .send_envelope(
                node_ref.clone(),
                Some(destination_node_ref.clone()),
                message,
            )
            .await
            .map_err(|e| {
                // If we're returning an error, clean up
                let send_ts = Timestamp::now();
                self.record_send_failure(
                    RPCKind::Statement,
                    send_ts,
                    node_ref.unfiltered(),
                    safety_route,
                    remote_private_route,
                );
                RPCError::network(e)
            })?;
        // Take send timestamp -after- send is attempted to exclude TCP connection time which
        // may unfairly punish some nodes, randomly, based on their being in the connection table or not
        let send_ts = Timestamp::now();
        let send_data_result = network_result_value_or_log!(self res => [ format!(": node_ref={}, destination_node_ref={}, message.len={}", node_ref, destination_node_ref, message_len) ] {
                // If we couldn't send we're still cleaning up
                self.record_send_failure(RPCKind::Statement, send_ts, node_ref.unfiltered(), safety_route, remote_private_route);
                network_result_raise!(res);
            }
        );

        // Successfully sent
        self.record_send_success(
            RPCKind::Statement,
            send_ts,
            bytes,
            node_ref.unfiltered(),
            safety_route,
            remote_private_route,
            send_data_result.is_ordered(),
        );

        Ok(NetworkResult::value(()))
    }
    /// Issue a reply over the network, possibly using an anonymized route
    /// The request must want a response, or this routine fails
    #[instrument(level = "trace", target = "rpc", skip_all)]
    async fn answer(&self, request: Message, answer: RPCAnswer) -> RPCNetworkResult<()> {
        // Extract destination from respond_to
        let dest = network_result_try!(self.get_respond_to_destination(&request));

        // Get sender signed node info if we should send that
        let spi = self.get_sender_peer_info(&dest);

        // Wrap answer in operation
        let operation = RPCOperation::new_answer(&request.operation, answer, spi);

        // Log rpc send
        veilid_log!(self debug target: "rpc_message", dir = "send", kind = "answer", op_id = operation.op_id().as_u64(), desc = operation.kind().desc(), ?dest);

        // Produce rendered operation
        let RenderedOperation {
            message,
            destination_node_ref,
            node_ref,
            hop_count: _,
            safety_route,
            remote_private_route,
            reply_private_route: _,
        } = network_result_try!(self.render_operation(dest, &operation)?);

        // Send the reply
        let bytes: ByteCount = (message.len() as u64).into();
        #[allow(unused_variables)]
        let message_len = message.len();
        let res = self
            .network_manager()
            .send_envelope(
                node_ref.clone(),
                Some(destination_node_ref.clone()),
                message,
            )
            .await
            .map_err(|e| {
                // If we're returning an error, clean up
                let send_ts = Timestamp::now();
                self.record_send_failure(
                    RPCKind::Answer,
                    send_ts,
                    node_ref.unfiltered(),
                    safety_route,
                    remote_private_route,
                );
                RPCError::network(e)
            })?;
        // Take send timestamp -after- send is attempted to exclude TCP connection time which
        // may unfairly punish some nodes, randomly, based on their being in the connection table or not
        let send_ts = Timestamp::now();
        let send_data_result = network_result_value_or_log!(self res => [ format!(": node_ref={}, destination_node_ref={}, message.len={}", node_ref, destination_node_ref, message_len) ] {
                // If we couldn't send we're still cleaning up
                self.record_send_failure(RPCKind::Answer, send_ts, node_ref.unfiltered(), safety_route, remote_private_route);
                network_result_raise!(res);
            }
        );

        // Reply successfully sent
        self.record_send_success(
            RPCKind::Answer,
            send_ts,
            bytes,
            node_ref.unfiltered(),
            safety_route,
            remote_private_route,
            send_data_result.is_ordered(),
        );

        Ok(NetworkResult::value(()))
    }

    /// Decoding RPC from the wire
    /// This performs a capnp decode on the data, and if it passes the capnp schema
    /// it performs the cryptographic validation required to pass the operation up for processing
    #[instrument(level = "trace", target = "rpc", skip_all)]
    fn decode_rpc_operation(&self, encoded_msg: &MessageEncoded) -> Result<RPCOperation, RPCError> {
        let reader = encoded_msg.data.get_reader()?;
        let op_reader = reader
            .get_root::<veilid_capnp::operation::Reader>()
            .map_err(RPCError::protocol)?;
        let decode_context = RPCDecodeContext {
            routing_domain: encoded_msg.header.routing_domain(),
        };
        let mut operation = RPCOperation::decode(&decode_context, &op_reader)?;

        // Validate the RPC message
        self.validate_rpc_operation(&mut operation)?;

        Ok(operation)
    }

    /// Cryptographic RPC validation and sanitization
    ///
    /// This code may modify the RPC operation to remove elements that are inappropriate for this node
    /// or reject the RPC operation entirely. For example, PeerInfo in fanout peer lists may be
    /// removed if they are deemed inappropriate for this node, without rejecting the entire operation.
    ///
    /// We do this as part of the RPC network layer to ensure that any RPC operations that are
    /// processed have already been validated cryptographically and it is not the job of the
    /// caller or receiver. This does not mean the operation is 'semantically correct'. For
    /// complex operations that require stateful validation and a more robust context than
    /// 'signatures', the caller must still perform whatever validation is necessary
    #[instrument(level = "trace", target = "rpc", skip_all)]
    fn validate_rpc_operation(&self, operation: &mut RPCOperation) -> Result<(), RPCError> {
        // If this is an answer, get the question context for this answer
        // If we received an answer for a question we did not ask, this will return an error
        let question_context = if let RPCOperationKind::Answer(_) = operation.kind() {
            let op_id = operation.op_id();
            self.waiting_rpc_table.get_op_context(op_id)?
        } else {
            None
        };

        // Validate the RPC operation
        let validate_context = RPCValidateContext {
            registry: self.registry(),
            // rpc_processor: self.clone(),
            question_context,
        };
        operation.validate(&validate_context)?;

        Ok(())
    }

    //////////////////////////////////////////////////////////////////////
    #[instrument(level = "trace", target = "rpc", skip_all)]
    async fn process_rpc_message(&self, encoded_msg: MessageEncoded) -> RPCNetworkResult<()> {
        // Decode operation appropriately based on header detail
        let msg = match &encoded_msg.header.detail {
            RPCMessageHeaderDetail::Direct(detail) => {
                // Get sender node id
                let sender_node_id = detail.envelope.get_sender_typed_id();

                // Decode and validate the RPC operation
                let decode_res = self.decode_rpc_operation(&encoded_msg);
                let operation = match decode_res {
                    Ok(v) => v,
                    Err(e) => {
                        match e {
                            // Invalid messages that should be punished
                            RPCError::Protocol(_) | RPCError::InvalidFormat(_) => {
                                veilid_log!(self debug "Invalid RPC Operation: {}", e);

                                // Punish nodes that send direct undecodable crap
                                self.network_manager().address_filter().punish_node_id(
                                    sender_node_id,
                                    PunishmentReason::FailedToDecodeOperation,
                                );
                            }
                            // Ignored messages that should be dropped
                            RPCError::Ignore(_) | RPCError::Network(_) | RPCError::TryAgain(_) => {
                                veilid_log!(self trace "Dropping RPC Operation: {}", e);
                            }
                            // Internal errors that deserve louder logging
                            RPCError::Unimplemented(_) | RPCError::Internal(_) => {
                                veilid_log!(self error "Error decoding RPC operation: {}", e);
                            }
                        };
                        return Ok(NetworkResult::invalid_message(e));
                    }
                };

                // Get the routing domain this message came over
                let routing_domain = detail.routing_domain;

                // Get the sender noderef, incorporating sender's peer info
                let sender_peer_info = operation.sender_peer_info();
                let mut opt_sender_nr: Option<NodeRef> = network_result_try!(self
                .process_sender_peer_info(routing_domain, sender_node_id, &sender_peer_info)? => {
                    veilid_log!(self debug target:"network_result", "Sender PeerInfo: {:?}", sender_peer_info);
                    veilid_log!(self debug target:"network_result", "From Operation: {:?}", operation.kind());
                    veilid_log!(self debug target:"network_result", "With Detail: {:?}", detail);
                });
                // look up sender node, in case it's different than our peer due to relaying
                if opt_sender_nr.is_none() {
                    opt_sender_nr = match self.routing_table().lookup_node_ref(sender_node_id) {
                        Ok(v) => v,
                        Err(e) => {
                            // If this fails it's not the other node's fault. We should be able to look up a
                            // node ref for a registered sender node id that just sent a message to us
                            // because it is registered with an existing connection at the very least.
                            return Ok(NetworkResult::no_connection_other(e));
                        }
                    }
                }

                // Update the 'seen our node info' timestamp to determine if this node needs a
                // 'node info update' ping
                if let Some(sender_nr) = &opt_sender_nr {
                    sender_nr.set_seen_our_node_info_ts(
                        routing_domain,
                        sender_peer_info.target_node_info_ts,
                    );
                }

                // Make the RPC message
                Message {
                    header: encoded_msg.header,
                    operation,
                    opt_sender_nr,
                }
            }
            RPCMessageHeaderDetail::SafetyRouted(_) | RPCMessageHeaderDetail::PrivateRouted(_) => {
                // Decode and validate the RPC operation
                let operation = match self.decode_rpc_operation(&encoded_msg) {
                    Ok(v) => v,
                    Err(e) => {
                        // Debug on error
                        veilid_log!(self debug "Dropping routed RPC: {}", e);

                        // XXX: Punish routes that send routed undecodable crap
                        // self.network_manager().address_filter().punish_route_id(xxx, PunishmentReason::FailedToDecodeRoutedMessage);
                        return Ok(NetworkResult::invalid_message(e));
                    }
                };

                // Make the RPC message
                Message {
                    header: encoded_msg.header,
                    operation,
                    opt_sender_nr: None,
                }
            }
        };

        // Process stats for questions/statements received
        match msg.operation.kind() {
            RPCOperationKind::Question(_) => {
                self.record_question_received(&msg);

                if let Some(sender_nr) = msg.opt_sender_nr.clone() {
                    sender_nr.stats_question_rcvd(msg.header.timestamp, msg.header.body_len);
                }

                // Log rpc receive
                veilid_log!(self debug target: "rpc_message", dir = "recv", kind = "question", op_id = msg.operation.op_id().as_u64(), desc = msg.operation.kind().desc(), header = ?msg.header, operation = ?msg.operation.kind());
            }
            RPCOperationKind::Statement(_) => {
                if let Some(sender_nr) = msg.opt_sender_nr.clone() {
                    sender_nr.stats_question_rcvd(msg.header.timestamp, msg.header.body_len);
                }

                // Log rpc receive
                veilid_log!(self debug target: "rpc_message", dir = "recv", kind = "statement", op_id = msg.operation.op_id().as_u64(), desc = msg.operation.kind().desc(), header = ?msg.header, operation = ?msg.operation.kind());
            }
            RPCOperationKind::Answer(_) => {
                // Answer stats are processed in wait_for_reply

                // Log rpc receive
                veilid_log!(self debug target: "rpc_message", dir = "recv", kind = "answer", op_id = msg.operation.op_id().as_u64(), desc = msg.operation.kind().desc(), header = ?msg.header, operation = ?msg.operation.kind());
            }
        };

        // Process specific message kind
        match msg.operation.kind() {
            RPCOperationKind::Question(q) => {
                let res = match q.detail() {
                    RPCQuestionDetail::StatusQ(_) => {
                        pin_dyn_future_closure!(self.process_status_q(msg))
                    }
                    RPCQuestionDetail::FindNodeQ(_) => {
                        pin_dyn_future_closure!(self.process_find_node_q(msg))
                    }
                    RPCQuestionDetail::AppCallQ(_) => {
                        pin_dyn_future_closure!(self.process_app_call_q(msg))
                    }
                    RPCQuestionDetail::GetValueQ(_) => {
                        pin_dyn_future_closure!(self.process_get_value_q(msg))
                    }
                    RPCQuestionDetail::SetValueQ(_) => {
                        pin_dyn_future_closure!(self.process_set_value_q(msg))
                    }
                    RPCQuestionDetail::WatchValueQ(_) => {
                        pin_dyn_future_closure!(self.process_watch_value_q(msg))
                    }
                    RPCQuestionDetail::InspectValueQ(_) => {
                        pin_dyn_future_closure!(self.process_inspect_value_q(msg))
                    }
                    #[cfg(feature = "unstable-blockstore")]
                    RPCQuestionDetail::SupplyBlockQ(_) => {
                        pin_dyn_future_closure!(self.process_supply_block_q(msg))
                    }
                    #[cfg(feature = "unstable-blockstore")]
                    RPCQuestionDetail::FindBlockQ(_) => {
                        pin_dyn_future_closure!(self.process_find_block_q(msg))
                    }
                    #[cfg(feature = "unstable-tunnels")]
                    RPCQuestionDetail::StartTunnelQ(_) => {
                        pin_dyn_future_closure!(self.process_start_tunnel_q(msg))
                    }
                    #[cfg(feature = "unstable-tunnels")]
                    RPCQuestionDetail::CompleteTunnelQ(_) => {
                        pin_dyn_future_closure!(self.process_complete_tunnel_q(msg))
                    }
                    #[cfg(feature = "unstable-tunnels")]
                    RPCQuestionDetail::CancelTunnelQ(_) => {
                        pin_dyn_future_closure!(self.process_cancel_tunnel_q(msg))
                    }
                };
                res.await
            }
            RPCOperationKind::Statement(s) => {
                let res = match s.detail() {
                    RPCStatementDetail::ValidateDialInfo(_) => {
                        pin_dyn_future_closure!(self.process_validate_dial_info(msg))
                    }
                    RPCStatementDetail::Route(_) => {
                        pin_dyn_future_closure!(self.process_route(msg))
                    }
                    RPCStatementDetail::ValueChanged(_) => {
                        pin_dyn_future_closure!(self.process_value_changed(msg))
                    }
                    RPCStatementDetail::Signal(_) => {
                        pin_dyn_future_closure!(self.process_signal(msg))
                    }
                    RPCStatementDetail::ReturnReceipt(_) => {
                        pin_dyn_future_closure!(self.process_return_receipt(msg))
                    }
                    RPCStatementDetail::AppMessage(_) => {
                        pin_dyn_future_closure!(self.process_app_message(msg))
                    }
                };
                res.await
            }
            RPCOperationKind::Answer(_) => {
                let op_id = msg.operation.op_id();
                if let Err(e) = self.waiting_rpc_table.complete_op_waiter(op_id, msg) {
                    match e {
                        RPCError::Unimplemented(_) | RPCError::Internal(_) => {
                            veilid_log!(self error "Could not complete rpc operation: id = {}: {}", op_id, e);
                        }
                        RPCError::InvalidFormat(_)
                        | RPCError::Protocol(_)
                        | RPCError::Network(_)
                        | RPCError::TryAgain(_) => {
                            veilid_log!(self debug "Could not complete rpc operation: id = {}: {}", op_id, e);
                        }
                        RPCError::Ignore(_) => {
                            veilid_log!(self debug "Answer late: id = {}", op_id);
                        }
                    };
                    // Don't throw an error here because it's okay if the original operation timed out
                }
                Ok(NetworkResult::value(()))
            }
        }
    }

    async fn rpc_worker(
        &self,
        stop_token: StopToken,
        receiver: flume::Receiver<(Span, MessageEncoded)>,
    ) {
        while let Ok(Ok((prev_span, msg))) =
            receiver.recv_async().timeout_at(stop_token.clone()).await
        {
            let rpc_message_span = tracing::trace_span!("rpc message");
            rpc_message_span.follows_from(prev_span);

            network_result_value_or_log!(self match self
                .process_rpc_message(msg).instrument(rpc_message_span)
                .await
            {
                Err(e) => {
                    veilid_log!(self error "couldn't process rpc message: {}", e);
                    continue;
                }

                Ok(v) => {
                    v
                }
            } => [ format!(": msg.header={:?}", msg.header) ] {});
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

        let msg = MessageEncoded {
            header,
            data: MessageData { contents: body },
        };

        let send_channel = {
            let inner = self.inner.lock();
            let Some(send_channel) = inner.send_channel.as_ref().cloned() else {
                bail!("send channel is closed");
            };
            send_channel
        };
        send_channel
            .try_send((Span::current(), msg))
            .map_err(|e| eyre!("failed to enqueue direct RPC message: {}", e))?;
        Ok(())
    }

    #[instrument(level = "trace", target = "rpc", skip_all)]
    fn enqueue_safety_routed_message(
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

        let msg = MessageEncoded {
            header,
            data: MessageData { contents: body },
        };
        let send_channel = {
            let inner = self.inner.lock();
            let Some(send_channel) = inner.send_channel.as_ref().cloned() else {
                bail!("send channel is closed");
            };
            send_channel
        };
        send_channel
            .try_send((Span::current(), msg))
            .map_err(|e| eyre!("failed to enqueue safety routed RPC message: {}", e))?;
        Ok(())
    }

    #[instrument(level = "trace", target = "rpc", skip_all)]
    fn enqueue_private_routed_message(
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

        let msg = MessageEncoded {
            header,
            data: MessageData { contents: body },
        };

        let send_channel = {
            let inner = self.inner.lock();
            let Some(send_channel) = inner.send_channel.as_ref().cloned() else {
                bail!("send channel is closed");
            };
            send_channel
        };
        send_channel
            .try_send((Span::current(), msg))
            .map_err(|e| eyre!("failed to enqueue private routed RPC message: {}", e))?;
        Ok(())
    }
}
