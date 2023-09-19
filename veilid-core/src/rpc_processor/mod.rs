mod coders;
mod destination;
mod fanout_call;
mod fanout_queue;
mod operation_waiter;
mod rpc_app_call;
mod rpc_app_message;
mod rpc_error;
mod rpc_find_node;
mod rpc_get_value;
mod rpc_return_receipt;
mod rpc_route;
mod rpc_set_value;
mod rpc_signal;
mod rpc_status;
mod rpc_validate_dial_info;
mod rpc_value_changed;
mod rpc_watch_value;

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

pub use coders::*;
pub use destination::*;
pub use fanout_call::*;
pub use fanout_queue::*;
pub use operation_waiter::*;
pub use rpc_error::*;
pub use rpc_status::*;

use super::*;

use crypto::*;
use futures_util::StreamExt;
use network_manager::*;
use receipt_manager::*;
use routing_table::*;
use stop_token::future::FutureExt;
use storage_manager::*;

/////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
struct RPCMessageHeaderDetailDirect {
    /// The decoded header of the envelope
    envelope: Envelope,
    /// The noderef of the peer that sent the message (not the original sender). Ensures node doesn't get evicted from routing table until we're done with it
    peer_noderef: NodeRef,
    /// The connection from the peer sent the message (not the original sender)
    connection_descriptor: ConnectionDescriptor,
    /// The routing domain the message was sent through
    routing_domain: RoutingDomain,
}

/// Header details for rpc messages received over only a safety route but not a private route
#[derive(Debug, Clone)]
struct RPCMessageHeaderDetailSafetyRouted {
    /// Direct header
    direct: RPCMessageHeaderDetailDirect,
    /// Remote safety route used
    remote_safety_route: PublicKey,
    /// The sequencing used for this route
    sequencing: Sequencing,
}

/// Header details for rpc messages received over a private route
#[derive(Debug, Clone)]
struct RPCMessageHeaderDetailPrivateRouted {
    /// Direct header
    direct: RPCMessageHeaderDetailDirect,
    /// Remote safety route used (or possibly node id the case of no safety route)
    remote_safety_route: PublicKey,
    /// The private route we received the rpc over
    private_route: PublicKey,
    // The safety spec for replying to this private routed rpc
    safety_spec: SafetySpec,
}

#[derive(Debug, Clone)]
enum RPCMessageHeaderDetail {
    Direct(RPCMessageHeaderDetailDirect),
    SafetyRouted(RPCMessageHeaderDetailSafetyRouted),
    PrivateRouted(RPCMessageHeaderDetailPrivateRouted),
}

/// The decoded header of an RPC message
#[derive(Debug, Clone)]
struct RPCMessageHeader {
    /// Time the message was received, not sent
    timestamp: Timestamp,
    /// The length in bytes of the rpc message body
    body_len: ByteCount,
    /// The header detail depending on which way the message was received
    detail: RPCMessageHeaderDetail,
}

impl RPCMessageHeader {
    /// The crypto kind used on the RPC
    pub fn crypto_kind(&self) -> CryptoKind {
        match &self.detail {
            RPCMessageHeaderDetail::Direct(d) => d.envelope.get_crypto_kind(),
            RPCMessageHeaderDetail::SafetyRouted(s) => s.direct.envelope.get_crypto_kind(),
            RPCMessageHeaderDetail::PrivateRouted(p) => p.direct.envelope.get_crypto_kind(),
        }
    }
    // pub fn direct_peer_noderef(&self) -> NodeRef {
    //     match &self.detail {
    //         RPCMessageHeaderDetail::Direct(d) => d.peer_noderef.clone(),
    //         RPCMessageHeaderDetail::SafetyRouted(s) => s.direct.peer_noderef.clone(),
    //         RPCMessageHeaderDetail::PrivateRouted(p) => p.direct.peer_noderef.clone(),
    //     }
    // }
    pub fn routing_domain(&self) -> RoutingDomain {
        match &self.detail {
            RPCMessageHeaderDetail::Direct(d) => d.routing_domain,
            RPCMessageHeaderDetail::SafetyRouted(s) => s.direct.routing_domain,
            RPCMessageHeaderDetail::PrivateRouted(p) => p.direct.routing_domain,
        }
    }
    pub fn direct_sender_node_id(&self) -> TypedKey {
        match &self.detail {
            RPCMessageHeaderDetail::Direct(d) => d.envelope.get_sender_typed_id(),
            RPCMessageHeaderDetail::SafetyRouted(s) => s.direct.envelope.get_sender_typed_id(),
            RPCMessageHeaderDetail::PrivateRouted(p) => p.direct.envelope.get_sender_typed_id(),
        }
    }
}

#[derive(Debug)]
pub struct RPCMessageData {
    contents: Vec<u8>, // rpc messages must be a canonicalized single segment
}

impl RPCMessageData {
    pub fn new(contents: Vec<u8>) -> Self {
        Self { contents }
    }

    pub fn get_reader(
        &self,
    ) -> Result<capnp::message::Reader<capnp::serialize::OwnedSegments>, RPCError> {
        capnp::serialize_packed::read_message(
            self.contents.as_slice(),
            capnp::message::ReaderOptions::new(),
        )
        .map_err(RPCError::protocol)
    }
}

#[derive(Debug)]
struct RPCMessageEncoded {
    header: RPCMessageHeader,
    data: RPCMessageData,
}

#[derive(Debug)]
pub(crate) struct RPCMessage {
    header: RPCMessageHeader,
    operation: RPCOperation,
    opt_sender_nr: Option<NodeRef>,
}

pub fn builder_to_vec<'a, T>(builder: capnp::message::Builder<T>) -> Result<Vec<u8>, RPCError>
where
    T: capnp::message::Allocator + 'a,
{
    let mut buffer = vec![];
    capnp::serialize_packed::write_message(&mut buffer, &builder)
        .map_err(RPCError::protocol)
        .map_err(logthru_rpc!())?;
    Ok(buffer)
}

#[derive(Debug)]
struct WaitableReply {
    handle: OperationWaitHandle<RPCMessage, Option<QuestionContext>>,
    timeout_us: TimestampDuration,
    node_ref: NodeRef,
    send_ts: Timestamp,
    send_data_kind: SendDataKind,
    safety_route: Option<PublicKey>,
    remote_private_route: Option<PublicKey>,
    reply_private_route: Option<PublicKey>,
}

/////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Default)]
pub struct Answer<T> {
    pub latency: TimestampDuration, // how long it took to get this answer
    pub answer: T,                  // the answer itself
}
impl<T> Answer<T> {
    pub fn new(latency: TimestampDuration, answer: T) -> Self {
        Self { latency, answer }
    }
}

/// An operation that has been fully prepared for envelope
struct RenderedOperation {
    /// The rendered operation bytes
    message: Vec<u8>,
    /// Destination node we're sending to
    destination_node_ref: NodeRef,
    /// Node to send envelope to (may not be destination node in case of relay)
    node_ref: NodeRef,
    /// Total safety + private route hop count + 1 hop for the initial send
    hop_count: usize,
    /// The safety route used to send the message
    safety_route: Option<PublicKey>,
    /// The private route used to send the message
    remote_private_route: Option<PublicKey>,
    /// The private route requested to receive the reply
    reply_private_route: Option<PublicKey>,
}

impl fmt::Debug for RenderedOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RenderedOperation")
            .field("message(len)", &self.message.len())
            .field("destination_node_ref", &self.destination_node_ref)
            .field("node_ref", &self.node_ref)
            .field("hop_count", &self.hop_count)
            .field("safety_route", &self.safety_route)
            .field("remote_private_route", &self.remote_private_route)
            .field("reply_private_route", &self.reply_private_route)
            .finish()
    }
}

/// Node information exchanged during every RPC message
#[derive(Default, Debug, Clone)]
pub struct SenderPeerInfo {
    /// The current peer info of the sender if required
    opt_sender_peer_info: Option<PeerInfo>,
    /// The last timestamp of the target's node info to assist remote node with sending its latest node info
    target_node_info_ts: Timestamp,
}
impl SenderPeerInfo {
    pub fn new_no_peer_info(target_node_info_ts: Timestamp) -> Self {
        Self {
            opt_sender_peer_info: None,
            target_node_info_ts,
        }
    }
    pub fn new(sender_peer_info: PeerInfo, target_node_info_ts: Timestamp) -> Self {
        Self {
            opt_sender_peer_info: Some(sender_peer_info),
            target_node_info_ts,
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum RPCKind {
    Question,
    Statement,
    Answer,
}

/////////////////////////////////////////////////////////////////////

pub struct RPCProcessorInner {
    send_channel: Option<flume::Sender<(Option<Id>, RPCMessageEncoded)>>,
    stop_source: Option<StopSource>,
    worker_join_handles: Vec<MustJoinHandle<()>>,
}

pub struct RPCProcessorUnlockedInner {
    timeout_us: TimestampDuration,
    queue_size: u32,
    concurrency: u32,
    max_route_hop_count: usize,
    validate_dial_info_receipt_time_ms: u32,
    update_callback: UpdateCallback,
    waiting_rpc_table: OperationWaiter<RPCMessage, Option<QuestionContext>>,
    waiting_app_call_table: OperationWaiter<Vec<u8>, ()>,
}

#[derive(Clone)]
pub struct RPCProcessor {
    crypto: Crypto,
    config: VeilidConfig,
    network_manager: NetworkManager,
    storage_manager: StorageManager,
    routing_table: RoutingTable,
    inner: Arc<Mutex<RPCProcessorInner>>,
    unlocked_inner: Arc<RPCProcessorUnlockedInner>,
}

impl RPCProcessor {
    fn new_inner() -> RPCProcessorInner {
        RPCProcessorInner {
            send_channel: None,
            stop_source: None,
            worker_join_handles: Vec::new(),
        }
    }
    fn new_unlocked_inner(
        config: VeilidConfig,
        update_callback: UpdateCallback,
    ) -> RPCProcessorUnlockedInner {
        // make local copy of node id for easy access
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
        let validate_dial_info_receipt_time_ms = c.network.dht.validate_dial_info_receipt_time_ms;

        RPCProcessorUnlockedInner {
            timeout_us,
            queue_size,
            concurrency,
            max_route_hop_count,
            validate_dial_info_receipt_time_ms,
            update_callback,
            waiting_rpc_table: OperationWaiter::new(),
            waiting_app_call_table: OperationWaiter::new(),
        }
    }
    pub fn new(network_manager: NetworkManager, update_callback: UpdateCallback) -> Self {
        let config = network_manager.config();
        Self {
            crypto: network_manager.crypto(),
            config: config.clone(),
            network_manager: network_manager.clone(),
            routing_table: network_manager.routing_table(),
            storage_manager: network_manager.storage_manager(),
            inner: Arc::new(Mutex::new(Self::new_inner())),
            unlocked_inner: Arc::new(Self::new_unlocked_inner(config, update_callback)),
        }
    }

    pub fn network_manager(&self) -> NetworkManager {
        self.network_manager.clone()
    }

    pub fn routing_table(&self) -> RoutingTable {
        self.routing_table.clone()
    }

    pub fn storage_manager(&self) -> StorageManager {
        self.storage_manager.clone()
    }

    //////////////////////////////////////////////////////////////////////

    #[instrument(level = "debug", skip_all, err)]
    pub async fn startup(&self) -> EyreResult<()> {
        debug!("startup rpc processor");
        {
            let mut inner = self.inner.lock();

            let channel = flume::bounded(self.unlocked_inner.queue_size as usize);
            inner.send_channel = Some(channel.0.clone());
            inner.stop_source = Some(StopSource::new());

            // spin up N workers
            trace!(
                "Spinning up {} RPC workers",
                self.unlocked_inner.concurrency
            );
            for _ in 0..self.unlocked_inner.concurrency {
                let this = self.clone();
                let receiver = channel.1.clone();
                let jh = spawn(Self::rpc_worker(
                    this,
                    inner.stop_source.as_ref().unwrap().token(),
                    receiver,
                ));
                inner.worker_join_handles.push(jh);
            }
        }

        // Inform storage manager we are up
        self.storage_manager
            .set_rpc_processor(Some(self.clone()))
            .await;

        Ok(())
    }

    #[instrument(level = "debug", skip_all)]
    pub async fn shutdown(&self) {
        debug!("starting rpc processor shutdown");

        // Stop storage manager from using us
        self.storage_manager.set_rpc_processor(None).await;

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
        debug!("stopping {} rpc worker tasks", unord.len());

        // Wait for them to complete
        while unord.next().await.is_some() {}

        debug!("resetting rpc processor state");

        // Release the rpc processor
        *self.inner.lock() = Self::new_inner();

        debug!("finished rpc processor shutdown");
    }

    //////////////////////////////////////////////////////////////////////

    /// Get waiting app call id for debugging purposes
    pub fn get_app_call_ids(&self) -> Vec<OperationId> {
        self.unlocked_inner.waiting_app_call_table.get_operation_ids()
    }

    /// Determine if a SignedNodeInfo can be placed into the specified routing domain
    fn verify_node_info(
        &self,
        routing_domain: RoutingDomain,
        signed_node_info: &SignedNodeInfo,
        capabilities: &[Capability],
    ) -> bool {
        let routing_table = self.routing_table();
        routing_table.signed_node_info_is_valid_in_routing_domain(routing_domain, signed_node_info)
            && signed_node_info.node_info().has_capabilities(capabilities)
    }

    //////////////////////////////////////////////////////////////////////

    /// Search the network for a single node and add it to the routing table and return the node reference
    /// If no node was found in the timeout, this returns None
    async fn search_for_node_id(
        &self,
        node_id: TypedKey,
        count: usize,
        fanout: usize,
        timeout_us: TimestampDuration,
        safety_selection: SafetySelection,
    ) -> TimeoutOr<Result<Option<NodeRef>, RPCError>> {
        let routing_table = self.routing_table();

        // Routine to call to generate fanout
        let call_routine = |next_node: NodeRef| {
            let this = self.clone();
            async move {
                match this
                    .clone()
                    .rpc_call_find_node(
                        Destination::direct(next_node).with_safety(safety_selection),
                        node_id,
                        vec![],
                    )
                    .await
                {
                    Ok(v) => {
                        let v = network_result_value_or_log!(v => [ format!(": node_id={} count={} fanout={} fanout={} safety_selection={:?}", node_id, count, fanout, timeout_us, safety_selection) ] {
                            // Any other failures, just try the next node
                            return Ok(None);
                        });
                        Ok(Some(v.answer))
                    }
                    Err(e) => Err(e),
                }
            }
        };

        // Routine to call to check if we're done at each step
        let check_done = |_:&[NodeRef]| {
            let Ok(Some(nr)) = routing_table
                .lookup_node_ref(node_id) else {
                    return None;
                };
        
            // ensure we have some dial info for the entry already,
            // and that the node is still alive
            // if not, we should keep looking for better info
            if !matches!(nr.state(get_aligned_timestamp()),BucketEntryState::Dead) &&
                nr.has_any_dial_info() {
                return Some(nr);
            }
    
            None
        };

        // Call the fanout
        let fanout_call = FanoutCall::new(
            routing_table.clone(),
            node_id,
            count,
            fanout,
            timeout_us,
            empty_fanout_node_info_filter(),
            call_routine,
            check_done,
        );

        fanout_call.run().await
    }

    /// Search the DHT for a specific node corresponding to a key unless we have that node in our routing table already, and return the node reference
    /// Note: This routine can possible be recursive, hence the SendPinBoxFuture async form
    pub fn resolve_node(
        &self,
        node_id: TypedKey,
        safety_selection: SafetySelection,
    ) -> SendPinBoxFuture<Result<Option<NodeRef>, RPCError>> {
        let this = self.clone();
        Box::pin(async move {
            let routing_table = this.routing_table();

            // First see if we have the node in our routing table already
            if let Some(nr) = routing_table
                .lookup_node_ref(node_id)
                .map_err(RPCError::internal)?
            {
                // ensure we have some dial info for the entry already,
                // and that the node is still alive
                // if not, we should do the find_node anyway
                if !matches!(nr.state(get_aligned_timestamp()),BucketEntryState::Dead) &&
                    nr.has_any_dial_info() {
                    return Ok(Some(nr));
                }
            }

            // If nobody knows where this node is, ask the DHT for it
            let (node_count, _consensus_count, fanout, timeout) = {
                let c = this.config.get();
                (
                    c.network.dht.max_find_node_count as usize,
                    c.network.dht.resolve_node_count as usize,
                    c.network.dht.resolve_node_fanout as usize,
                    TimestampDuration::from(ms_to_us(c.network.dht.resolve_node_timeout_ms)),
                )
            };

            // Search in preferred cryptosystem order
            let nr = match this
                .search_for_node_id(node_id, node_count, fanout, timeout, safety_selection)
                .await
            {
                TimeoutOr::Timeout => None,
                TimeoutOr::Value(Ok(v)) => v,
                TimeoutOr::Value(Err(e)) => {
                    return Err(e);
                }
            };

            Ok(nr)
        })
    }

    #[cfg_attr(
        feature = "verbose-tracing",
        instrument(level = "trace", skip(self, waitable_reply), err)
    )]
    async fn wait_for_reply(
        &self,
        waitable_reply: WaitableReply,
        debug_string: String,
    ) -> Result<TimeoutOr<(RPCMessage, TimestampDuration)>, RPCError> {
        let out = self
            .unlocked_inner
            .waiting_rpc_table
            .wait_for_op(waitable_reply.handle, waitable_reply.timeout_us)
            .await;
        match &out {
            Err(e) => {
                log_rpc!(debug "RPC Lost ({}): {}", debug_string, e);
                self.record_question_lost(
                    waitable_reply.send_ts,
                    waitable_reply.node_ref.clone(),
                    waitable_reply.safety_route,
                    waitable_reply.remote_private_route,
                    waitable_reply.reply_private_route,
                );
            }
            Ok(TimeoutOr::Timeout) => {
                log_rpc!(debug "RPC Lost ({}): Timeout", debug_string);
                self.record_question_lost(
                    waitable_reply.send_ts,
                    waitable_reply.node_ref.clone(),
                    waitable_reply.safety_route,
                    waitable_reply.remote_private_route,
                    waitable_reply.reply_private_route,
                );
            }
            Ok(TimeoutOr::Value((rpcreader, _))) => {
                // Reply received
                let recv_ts = get_aligned_timestamp();

                // Record answer received
                self.record_answer_received(
                    waitable_reply.send_ts,
                    recv_ts,
                    rpcreader.header.body_len,
                    waitable_reply.node_ref.clone(),
                    waitable_reply.safety_route,
                    waitable_reply.remote_private_route,
                    waitable_reply.reply_private_route,
                )
            }
        };
        out
    }

    /// Wrap an operation with a private route inside a safety route
    fn wrap_with_route(
        &self,
        safety_selection: SafetySelection,
        remote_private_route: PrivateRoute,
        reply_private_route: Option<PublicKey>,
        message_data: Vec<u8>,
    ) -> Result<NetworkResult<RenderedOperation>, RPCError> {
        let routing_table = self.routing_table();
        let rss = routing_table.route_spec_store();

        // Get useful private route properties
        let pr_is_stub = remote_private_route.is_stub();
        let pr_hop_count = remote_private_route.hop_count;
        let pr_pubkey = remote_private_route.public_key.value;
        let crypto_kind = remote_private_route.crypto_kind();
        let Some(vcrypto) = self.crypto.get(crypto_kind) else {
            return Err(RPCError::internal("crypto not available for selected private route"));
        };

        // Compile the safety route with the private route
        let compiled_route: CompiledRoute = match rss
            .compile_safety_route(safety_selection, remote_private_route)
            .map_err(RPCError::internal)?
        {
            Some(cr) => cr,
            None => {
                return Ok(NetworkResult::no_connection_other(
                    "private route could not be compiled at this time",
                ))
            }
        };
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
        let operation =
            RoutedOperation::new(safety_selection.get_sequencing(), nonce, enc_msg_data);

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
            destination_node_ref: compiled_route.first_hop.clone(),
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
    #[cfg_attr(
        feature = "verbose-tracing",
        instrument(level = "debug", skip(self, operation), err)
    )]
    fn render_operation(
        &self,
        dest: Destination,
        operation: &RPCOperation,
    ) -> Result<NetworkResult<RenderedOperation>, RPCError> {
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
                target: ref node_ref,
                safety_selection,
            }
            | Destination::Relay {
                relay: ref node_ref,
                target: _,
                safety_selection,
            } => {
                // Send to a node without a private route
                // --------------------------------------

                // Get the actual destination node id accounting for relays
                let (node_ref, destination_node_ref) = if let Destination::Relay {
                    relay: _,
                    ref target,
                    safety_selection: _,
                } = dest
                {
                    (node_ref.clone(), target.clone())
                } else {
                    (node_ref.clone(), node_ref.clone())
                };

                // Handle the existence of safety route
                match safety_selection {
                    SafetySelection::Unsafe(sequencing) => {
                        // Apply safety selection sequencing requirement if it is more strict than the node_ref's sequencing requirement
                        let mut node_ref = node_ref.clone();
                        if sequencing > node_ref.sequencing() {
                            node_ref.set_sequencing(sequencing)
                        }
                        let mut destination_node_ref = destination_node_ref.clone();
                        if sequencing > destination_node_ref.sequencing() {
                            destination_node_ref.set_sequencing(sequencing)
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
                        // No private route was specified for the request
                        // but we are using a safety route, so we must create an empty private route
                        // Destination relay is ignored for safety routed operations
                        let peer_info = match destination_node_ref
                            .make_peer_info(RoutingDomain::PublicInternet)
                        {
                            None => {
                                return Ok(NetworkResult::no_connection_other(
                                    "No PublicInternet peer info for stub private route",
                                ))
                            }
                            Some(pi) => pi,
                        };
                        let private_route = PrivateRoute::new_stub(
                            destination_node_ref.best_node_id(),
                            RouteNode::PeerInfo(Box::new(peer_info)),
                        );

                        // Wrap with safety route
                        out = self.wrap_with_route(
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
                // Send to private route
                // ---------------------
                // Reply with 'route' operation
                out = self.wrap_with_route(
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
    #[cfg_attr(
        feature = "verbose-tracing",
        instrument(level = "trace", skip(self), ret)
    )]
    fn get_sender_peer_info(&self, dest: &Destination) -> SenderPeerInfo {
        // Don't do this if the sender is to remain private
        // Otherwise we would be attaching the original sender's identity to the final destination,
        // thus defeating the purpose of the safety route entirely :P
        match dest.get_safety_selection() {
            SafetySelection::Unsafe(_) => {}
            SafetySelection::Safe(_) => {
                return SenderPeerInfo::default();
            }
        }

        // Get the target we're sending to
        let routing_table = self.routing_table();
        let target = match dest {
            Destination::Direct {
                target,
                safety_selection: _,
            } => target.clone(),
            Destination::Relay {
                relay: _,
                target,
                safety_selection: _,
            } => target.clone(),
            Destination::PrivateRoute {
                private_route: _,
                safety_selection: _,
            } => {
                return SenderPeerInfo::default();
            }
        };

        let Some(routing_domain) = target.best_routing_domain() else {
            // No routing domain for target, no node info
            // Only a stale connection or no connection exists
            return SenderPeerInfo::default();
        };

        // Get the target's node info timestamp
        let target_node_info_ts = target.node_info_ts(routing_domain);

        // Return whatever peer info we have even if the network class is not yet valid
        // That away we overwrite any prior existing valid-network-class nodeinfo in the remote routing table
        let own_peer_info = routing_table.get_own_peer_info(routing_domain);

        // Get our node info timestamp
        let our_node_info_ts = own_peer_info.signed_node_info().timestamp();

        // If the target has seen our node info already don't send it again
        if target.has_seen_our_node_info_ts(routing_domain, our_node_info_ts) {
            return SenderPeerInfo::new_no_peer_info(target_node_info_ts);
        }

        SenderPeerInfo::new(own_peer_info, target_node_info_ts)
    }

    /// Record failure to send to node or route
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
            node_ref.clear_last_connections();

            return;
        }

        // If safety route was in use, record failure to send there
        if let Some(sr_pubkey) = &safety_route {
            let rss = self.routing_table.route_spec_store();
            rss.with_route_stats(send_ts, sr_pubkey, |s| s.record_send_failed());
        } else {
            // If no safety route was in use, then it's the private route's fault if we have one
            if let Some(pr_pubkey) = &remote_private_route {
                let rss = self.routing_table.route_spec_store();
                rss.with_route_stats(send_ts, pr_pubkey, |s| s.record_send_failed());
            }
        }
    }

    /// Record question lost to node or route
    fn record_question_lost(
        &self,
        send_ts: Timestamp,
        node_ref: NodeRef,
        safety_route: Option<PublicKey>,
        remote_private_route: Option<PublicKey>,
        private_route: Option<PublicKey>,
    ) {
        // Record for node if this was not sent via a route
        if safety_route.is_none() && remote_private_route.is_none() {
            node_ref.stats_question_lost();

            // Also clear the last_connections for the entry so we make a new connection next time
            node_ref.clear_last_connections();

            return;
        }
        // Get route spec store
        let rss = self.routing_table.route_spec_store();

        // If safety route was used, record question lost there
        if let Some(sr_pubkey) = &safety_route {
            let rss = self.routing_table.route_spec_store();
            rss.with_route_stats(send_ts, sr_pubkey, |s| {
                s.record_question_lost();
            });
        }
        // If remote private route was used, record question lost there
        if let Some(rpr_pubkey) = &remote_private_route {
            rss.with_route_stats(send_ts, rpr_pubkey, |s| {
                s.record_question_lost();
            });
        }
        // If private route was used, record question lost there
        if let Some(pr_pubkey) = &private_route {
            rss.with_route_stats(send_ts, pr_pubkey, |s| {
                s.record_question_lost();
            });
        }
    }

    /// Record success sending to node or route
    fn record_send_success(
        &self,
        rpc_kind: RPCKind,
        send_ts: Timestamp,
        bytes: ByteCount,
        node_ref: NodeRef,
        safety_route: Option<PublicKey>,
        remote_private_route: Option<PublicKey>,
    ) {
        let wants_answer = matches!(rpc_kind, RPCKind::Question);

        // Record for node if this was not sent via a route
        if safety_route.is_none() && remote_private_route.is_none() {
            node_ref.stats_question_sent(send_ts, bytes, wants_answer);
            return;
        }

        // Get route spec store
        let rss = self.routing_table.route_spec_store();

        // If safety route was used, record send there
        if let Some(sr_pubkey) = &safety_route {
            rss.with_route_stats(send_ts, sr_pubkey, |s| {
                s.record_sent(send_ts, bytes);
            });
        }

        // If remote private route was used, record send there
        if let Some(pr_pubkey) = &remote_private_route {
            let rss = self.routing_table.route_spec_store();
            rss.with_route_stats(send_ts, pr_pubkey, |s| {
                s.record_sent(send_ts, bytes);
            });
        }
    }

    /// Record answer received from node or route
    #[allow(clippy::too_many_arguments)]
    fn record_answer_received(
        &self,
        send_ts: Timestamp,
        recv_ts: Timestamp,
        bytes: ByteCount,
        node_ref: NodeRef,
        safety_route: Option<PublicKey>,
        remote_private_route: Option<PublicKey>,
        reply_private_route: Option<PublicKey>,
    ) {
        // Record stats for remote node if this was direct
        if safety_route.is_none() && remote_private_route.is_none() && reply_private_route.is_none()
        {
            node_ref.stats_answer_rcvd(send_ts, recv_ts, bytes);
            return;
        }
        // Get route spec store
        let rss = self.routing_table.route_spec_store();

        // Get latency for all local routes
        let mut total_local_latency = TimestampDuration::new(0u64);
        let total_latency: TimestampDuration = recv_ts.saturating_sub(send_ts);

        // If safety route was used, record route there
        if let Some(sr_pubkey) = &safety_route {
            rss.with_route_stats(send_ts, sr_pubkey, |s| {
                // If we received an answer, the safety route we sent over can be considered tested
                s.record_tested(recv_ts);

                // If we used a safety route to send, use our last tested latency
                total_local_latency += s.latency_stats().average
            });
        }

        // If local private route was used, record route there
        if let Some(pr_pubkey) = &reply_private_route {
            rss.with_route_stats(send_ts, pr_pubkey, |s| {
                // Record received bytes
                s.record_received(recv_ts, bytes);

                // If we used a private route to receive, use our last tested latency
                total_local_latency += s.latency_stats().average
            });
        }

        // If remote private route was used, record there
        if let Some(rpr_pubkey) = &remote_private_route {
            rss.with_route_stats(send_ts, rpr_pubkey, |s| {
                // Record received bytes
                s.record_received(recv_ts, bytes);

                // The remote route latency is recorded using the total latency minus the total local latency
                let remote_latency = total_latency.saturating_sub(total_local_latency);
                s.record_latency(remote_latency);
            });

            // If we sent to a private route without a safety route
            // We need to mark our own node info as having been seen so we can optimize sending it
            if let Err(e) = rss.mark_remote_private_route_seen_our_node_info(rpr_pubkey, recv_ts) {
                log_rpc!(error "private route missing: {}", e);
            }

            // We can't record local route latency if a remote private route was used because
            // there is no way other than the prior latency estimation to determine how much time was spent
            // in the remote private route
            // Instead, we rely on local route testing to give us latency numbers for our local routes
        } else {
            // If no remote private route was used, then record half the total latency on our local routes
            // This is fine because if we sent with a local safety route,
            // then we must have received with a local private route too, per the design rules
            if let Some(sr_pubkey) = &safety_route {
                let rss = self.routing_table.route_spec_store();
                rss.with_route_stats(send_ts, sr_pubkey, |s| {
                    s.record_latency(total_latency / 2u64);
                });
            }
            if let Some(pr_pubkey) = &reply_private_route {
                rss.with_route_stats(send_ts, pr_pubkey, |s| {
                    s.record_latency(total_latency / 2u64);
                });
            }
        }
    }

    /// Record question or statement received from node or route
    fn record_question_received(&self, msg: &RPCMessage) {
        let recv_ts = msg.header.timestamp;
        let bytes = msg.header.body_len;

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
                let rss = self.routing_table.route_spec_store();

                // This may record nothing if the remote safety route is not also
                // a remote private route that been imported, but that's okay
                rss.with_route_stats(recv_ts, &d.remote_safety_route, |s| {
                    s.record_received(recv_ts, bytes);
                });
            }
            // Process messages that arrived to our private route
            RPCMessageHeaderDetail::PrivateRouted(d) => {
                let rss = self.routing_table.route_spec_store();

                // This may record nothing if the remote safety route is not also
                // a remote private route that been imported, but that's okay
                // it could also be a node id if no remote safety route was used
                // in which case this also will do nothing
                rss.with_route_stats(recv_ts, &d.remote_safety_route, |s| {
                    s.record_received(recv_ts, bytes);
                });

                // Record for our local private route we received over
                rss.with_route_stats(recv_ts, &d.private_route, |s| {
                    s.record_received(recv_ts, bytes);
                });
            }
        }
    }

    /// Issue a question over the network, possibly using an anonymized route
    /// Optionally keeps a context to be passed to the answer processor when an answer is received
    #[cfg_attr(
        feature = "verbose-tracing",
        instrument(level = "debug", skip(self, question), err)
    )]
    async fn question(
        &self,
        dest: Destination,
        question: RPCQuestion,
        context: Option<QuestionContext>,
    ) -> Result<NetworkResult<WaitableReply>, RPCError> {
        // Get sender peer info if we should send that
        let spi = self.get_sender_peer_info(&dest);

        // Wrap question in operation
        let operation = RPCOperation::new_question(question, spi);
        let op_id = operation.op_id();

        // Log rpc send
        #[cfg(feature = "verbose-tracing")]
        debug!(target: "rpc_message", dir = "send", kind = "question", op_id = op_id.as_u64(), desc = operation.kind().desc(), ?dest);

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
        let timeout_us = self.unlocked_inner.timeout_us * (hop_count as u64);

        // Set up op id eventual
        let handle = self
            .unlocked_inner
            .waiting_rpc_table
            .add_op_waiter(op_id, context);

        // Send question
        let bytes: ByteCount = (message.len() as u64).into();
        let send_ts = get_aligned_timestamp();
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
                self.record_send_failure(
                    RPCKind::Question,
                    send_ts,
                    node_ref.clone(),
                    safety_route,
                    remote_private_route,
                );
                RPCError::network(e)
            })?;
        let send_data_kind = network_result_value_or_log!( res => [ format!(": node_ref={}, destination_node_ref={}, message.len={}", node_ref, destination_node_ref, message_len) ] {
                // If we couldn't send we're still cleaning up
                self.record_send_failure(RPCKind::Question, send_ts, node_ref.clone(), safety_route, remote_private_route);
                network_result_raise!(res);
            }
        );

        // Successfully sent
        self.record_send_success(
            RPCKind::Question,
            send_ts,
            bytes,
            node_ref.clone(),
            safety_route,
            remote_private_route,
        );

        // Pass back waitable reply completion
        Ok(NetworkResult::value(WaitableReply {
            handle,
            timeout_us,
            node_ref,
            send_ts,
            send_data_kind,
            safety_route,
            remote_private_route,
            reply_private_route,
        }))
    }

    /// Issue a statement over the network, possibly using an anonymized route
    #[cfg_attr(
        feature = "verbose-tracing",
        instrument(level = "debug", skip(self, statement), err)
    )]
    async fn statement(
        &self,
        dest: Destination,
        statement: RPCStatement,
    ) -> Result<NetworkResult<()>, RPCError> {
        // Get sender peer info if we should send that
        let spi = self.get_sender_peer_info(&dest);

        // Wrap statement in operation
        let operation = RPCOperation::new_statement(statement, spi);

        // Log rpc send
        #[cfg(feature = "verbose-tracing")]
        debug!(target: "rpc_message", dir = "send", kind = "statement", op_id = operation.op_id().as_u64(), desc = operation.kind().desc(), ?dest);

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
        let send_ts = get_aligned_timestamp();
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
                self.record_send_failure(
                    RPCKind::Statement,
                    send_ts,
                    node_ref.clone(),
                    safety_route,
                    remote_private_route,
                );
                RPCError::network(e)
            })?;
        let _send_data_kind = network_result_value_or_log!( res => [ format!(": node_ref={}, destination_node_ref={}, message.len={}", node_ref, destination_node_ref, message_len) ] {
                // If we couldn't send we're still cleaning up
                self.record_send_failure(RPCKind::Statement, send_ts, node_ref.clone(), safety_route, remote_private_route);
                network_result_raise!(res);
            }
        );

        // Successfully sent
        self.record_send_success(
            RPCKind::Statement,
            send_ts,
            bytes,
            node_ref,
            safety_route,
            remote_private_route,
        );

        Ok(NetworkResult::value(()))
    }
    /// Issue a reply over the network, possibly using an anonymized route
    /// The request must want a response, or this routine fails
    #[cfg_attr(
        feature = "verbose-tracing",
        instrument(level = "debug", skip(self, request, answer), err)
    )]
    async fn answer(
        &self,
        request: RPCMessage,
        answer: RPCAnswer,
    ) -> Result<NetworkResult<()>, RPCError> {
        // Extract destination from respond_to
        let dest = network_result_try!(self.get_respond_to_destination(&request));

        // Get sender signed node info if we should send that
        let spi = self.get_sender_peer_info(&dest);

        // Wrap answer in operation
        let operation = RPCOperation::new_answer(&request.operation, answer, spi);

        // Log rpc send
        #[cfg(feature = "verbose-tracing")]
        debug!(target: "rpc_message", dir = "send", kind = "answer", op_id = operation.op_id().as_u64(), desc = operation.kind().desc(), ?dest);

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
        let send_ts = get_aligned_timestamp();
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
                self.record_send_failure(
                    RPCKind::Answer,
                    send_ts,
                    node_ref.clone(),
                    safety_route,
                    remote_private_route,
                );
                RPCError::network(e)
            })?;
        let _send_data_kind = network_result_value_or_log!( res => [ format!(": node_ref={}, destination_node_ref={}, message.len={}", node_ref, destination_node_ref, message_len) ] {
                // If we couldn't send we're still cleaning up
                self.record_send_failure(RPCKind::Answer, send_ts, node_ref.clone(), safety_route, remote_private_route);
                network_result_raise!(res);
            }
        );

        // Reply successfully sent
        self.record_send_success(
            RPCKind::Answer,
            send_ts,
            bytes,
            node_ref,
            safety_route,
            remote_private_route,
        );

        Ok(NetworkResult::value(()))
    }

    /// Decoding RPC from the wire
    /// This performs a capnp decode on the data, and if it passes the capnp schema
    /// it performs the cryptographic validation required to pass the operation up for processing
    fn decode_rpc_operation(
        &self,
        encoded_msg: &RPCMessageEncoded,
    ) -> Result<RPCOperation, RPCError> {
        let reader = encoded_msg.data.get_reader()?;
        let op_reader = reader
            .get_root::<veilid_capnp::operation::Reader>()
            .map_err(RPCError::protocol)
            .map_err(logthru_rpc!())?;
        let mut operation = RPCOperation::decode(&op_reader)?;

        // Validate the RPC message
        self.validate_rpc_operation(&mut operation)?;

        Ok(operation)
    }

    /// Cryptographic RPC validation
    /// We do this as part of the RPC network layer to ensure that any RPC operations that are
    /// processed have already been validated cryptographically and it is not the job of the
    /// caller or receiver. This does not mean the operation is 'semantically correct'. For
    /// complex operations that require stateful validation and a more robust context than
    /// 'signatures', the caller must still perform whatever validation is necessary
    fn validate_rpc_operation(&self, operation: &mut RPCOperation) -> Result<(), RPCError> {
        // If this is an answer, get the question context for this answer
        // If we received an answer for a question we did not ask, this will return an error
        let question_context = if let RPCOperationKind::Answer(_) = operation.kind() {
            let op_id = operation.op_id();
            self.unlocked_inner
                .waiting_rpc_table
                .get_op_context(op_id)?
        } else {
            None
        };

        // Validate the RPC operation
        let validate_context = RPCValidateContext {
            crypto: self.crypto.clone(),
            rpc_processor: self.clone(),
            question_context,
        };
        operation.validate(&validate_context)?;

        Ok(())
    }

    //////////////////////////////////////////////////////////////////////
    #[cfg_attr(
        feature = "verbose-tracing",
        instrument(level = "trace", skip(self, encoded_msg), err)
    )]
    async fn process_rpc_message(
        &self,
        encoded_msg: RPCMessageEncoded,
    ) -> Result<NetworkResult<()>, RPCError> {
        let address_filter = self.network_manager.address_filter();

        // Decode operation appropriately based on header detail
        let msg = match &encoded_msg.header.detail {
            RPCMessageHeaderDetail::Direct(detail) => {
                // Get sender node id
                let sender_node_id = detail.envelope.get_sender_typed_id();

                // Decode and validate the RPC operation
                let operation = match self.decode_rpc_operation(&encoded_msg) {
                    Ok(v) => v,
                    Err(e) => {
                        // Punish nodes that send direct undecodable crap
                        if matches!(e, RPCError::Protocol(_) | RPCError::InvalidFormat(_)) {
                            address_filter.punish_node_id(sender_node_id);
                        }
                        return Ok(NetworkResult::invalid_message(e));
                    }
                };

                // Get the routing domain this message came over
                let routing_domain = detail.routing_domain;

                // Get the sender noderef, incorporating sender's peer info
                let mut opt_sender_nr: Option<NodeRef> = None;
                if let Some(sender_peer_info) = operation.sender_peer_info() {
                    // Ensure the sender peer info is for the actual sender specified in the envelope
                    if !sender_peer_info.node_ids().contains(&sender_node_id) {
                        // Attempted to update peer info for the wrong node id
                        address_filter.punish_node_id(sender_node_id);
                        return Ok(NetworkResult::invalid_message(
                            "attempt to update peer info for non-sender node id",
                        ));
                    }

                    // Sender PeerInfo was specified, update our routing table with it
                    if !self.verify_node_info(
                        routing_domain,
                        sender_peer_info.signed_node_info(),
                        &[],
                    ) {
                        address_filter.punish_node_id(sender_node_id);
                        return Ok(NetworkResult::invalid_message(
                            "sender peerinfo has invalid peer scope",
                        ));
                    }
                    opt_sender_nr = match self.routing_table().register_node_with_peer_info(
                        routing_domain,
                        sender_peer_info.clone(),
                        false,
                    ) {
                        Ok(v) => Some(v),
                        Err(e) => {
                            address_filter.punish_node_id(sender_node_id);
                            return Ok(NetworkResult::invalid_message(e));
                        } 
                    }
                }

                // look up sender node, in case it's different than our peer due to relaying
                if opt_sender_nr.is_none() {
                    opt_sender_nr = match self.routing_table().lookup_node_ref(sender_node_id) {
                        Ok(v) => v,
                        Err(e) => {
                            address_filter.punish_node_id(sender_node_id);
                            return Ok(NetworkResult::invalid_message(e));
                        }
                    }
                }

                // Update the 'seen our node info' timestamp to determine if this node needs a
                // 'node info update' ping
                if let Some(sender_nr) = &opt_sender_nr {
                    sender_nr
                        .set_seen_our_node_info_ts(routing_domain, operation.target_node_info_ts());
                }

                // Make the RPC message
                RPCMessage {
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
                        // Punish routes that send routed undecodable crap
                        // address_filter.punish_route_id(xxx);
                        return Ok(NetworkResult::invalid_message(e));
                    }
                };

                // Make the RPC message
                RPCMessage {
                    header: encoded_msg.header,
                    operation,
                    opt_sender_nr: None,
                }
            }
        };

        // Process stats for questions/statements received
        let kind = match msg.operation.kind() {
            RPCOperationKind::Question(_) => {
                self.record_question_received(&msg);

                if let Some(sender_nr) = msg.opt_sender_nr.clone() {
                    sender_nr.stats_question_rcvd(msg.header.timestamp, msg.header.body_len);
                }
                "question"
            }
            RPCOperationKind::Statement(_) => {
                if let Some(sender_nr) = msg.opt_sender_nr.clone() {
                    sender_nr.stats_question_rcvd(msg.header.timestamp, msg.header.body_len);
                }
                "statement"
            }
            RPCOperationKind::Answer(_) => {
                // Answer stats are processed in wait_for_reply
                "answer"
            }
        };

        // Log rpc receive
        trace!(target: "rpc_message", dir = "recv", kind, op_id = msg.operation.op_id().as_u64(), desc = msg.operation.kind().desc(), header = ?msg.header);

        // Process specific message kind
        match msg.operation.kind() {
            RPCOperationKind::Question(q) => match q.detail() {
                RPCQuestionDetail::StatusQ(_) => self.process_status_q(msg).await,
                RPCQuestionDetail::FindNodeQ(_) => self.process_find_node_q(msg).await,
                RPCQuestionDetail::AppCallQ(_) => self.process_app_call_q(msg).await,
                RPCQuestionDetail::GetValueQ(_) => self.process_get_value_q(msg).await,
                RPCQuestionDetail::SetValueQ(_) => self.process_set_value_q(msg).await,
                RPCQuestionDetail::WatchValueQ(_) => self.process_watch_value_q(msg).await,
                #[cfg(feature = "unstable-blockstore")]
                RPCQuestionDetail::SupplyBlockQ(_) => self.process_supply_block_q(msg).await,
                #[cfg(feature = "unstable-blockstore")]
                RPCQuestionDetail::FindBlockQ(_) => self.process_find_block_q(msg).await,
                #[cfg(feature = "unstable-tunnels")]
                RPCQuestionDetail::StartTunnelQ(_) => self.process_start_tunnel_q(msg).await,
                #[cfg(feature = "unstable-tunnels")]
                RPCQuestionDetail::CompleteTunnelQ(_) => self.process_complete_tunnel_q(msg).await,
                #[cfg(feature = "unstable-tunnels")]
                RPCQuestionDetail::CancelTunnelQ(_) => self.process_cancel_tunnel_q(msg).await,
            },
            RPCOperationKind::Statement(s) => match s.detail() {
                RPCStatementDetail::ValidateDialInfo(_) => {
                    self.process_validate_dial_info(msg).await
                }
                RPCStatementDetail::Route(_) => self.process_route(msg).await,
                RPCStatementDetail::ValueChanged(_) => self.process_value_changed(msg).await,
                RPCStatementDetail::Signal(_) => self.process_signal(msg).await,
                RPCStatementDetail::ReturnReceipt(_) => self.process_return_receipt(msg).await,
                RPCStatementDetail::AppMessage(_) => self.process_app_message(msg).await,
            },
            RPCOperationKind::Answer(_) => {
                self.unlocked_inner
                    .waiting_rpc_table
                    .complete_op_waiter(msg.operation.op_id(), msg)
                    .await?;
                Ok(NetworkResult::value(()))
            }
        }
    }

    async fn rpc_worker(
        self,
        stop_token: StopToken,
        receiver: flume::Receiver<(Option<Id>, RPCMessageEncoded)>,
    ) {
        while let Ok(Ok((_span_id, msg))) =
            receiver.recv_async().timeout_at(stop_token.clone()).await
        {
            let rpc_worker_span = span!(parent: None, Level::TRACE, "rpc_worker recv");
            // xxx: causes crash (Missing otel data span extensions)
            // rpc_worker_span.follows_from(span_id);
                    
            network_result_value_or_log!(match self
                .process_rpc_message(msg)
                .instrument(rpc_worker_span)
                .await
            {
                Err(e) => {
                    log_rpc!(error "couldn't process rpc message: {}", e);
                    continue;
                }

                Ok(v) => { 
                    v
                }
            } => [ format!(": msg.header={:?}", msg.header) ] {});
        }
    }

    #[cfg_attr(
        feature = "verbose-tracing",
        instrument(level = "trace", skip(self, body), err)
    )]
    pub fn enqueue_direct_message(
        &self,
        envelope: Envelope,
        peer_noderef: NodeRef,
        connection_descriptor: ConnectionDescriptor,
        routing_domain: RoutingDomain,
        body: Vec<u8>,
    ) -> EyreResult<()> {
        let header = RPCMessageHeader {
            detail: RPCMessageHeaderDetail::Direct(RPCMessageHeaderDetailDirect {
                envelope,
                peer_noderef,
                connection_descriptor,
                routing_domain,
            }),
            timestamp: get_aligned_timestamp(),
            body_len: ByteCount::new(body.len() as u64),
        };

        let msg = RPCMessageEncoded {
            header,
            data: RPCMessageData { contents: body },
        };

        let send_channel = {
            let inner = self.inner.lock();
            let Some(send_channel) = inner.send_channel.as_ref().cloned() else {
                bail!("send channel is closed");
            };
            send_channel
        };
        let span_id = Span::current().id();
        send_channel
            .try_send((span_id, msg))
            .map_err(|e| eyre!("failed to enqueue direct RPC message: {}", e))?;
        Ok(())
    }

    #[cfg_attr(
        feature = "verbose-tracing",
        instrument(level = "trace", skip(self, body), err)
    )]
    fn enqueue_safety_routed_message(
        &self,
        direct: RPCMessageHeaderDetailDirect,
        remote_safety_route: PublicKey,
        sequencing: Sequencing,
        body: Vec<u8>,
    ) -> EyreResult<()> {
        let header = RPCMessageHeader {
            detail: RPCMessageHeaderDetail::SafetyRouted(RPCMessageHeaderDetailSafetyRouted {
                direct,
                remote_safety_route,
                sequencing,
            }),
            timestamp: get_aligned_timestamp(),
            body_len: (body.len() as u64).into(),
        };

        let msg = RPCMessageEncoded {
            header,
            data: RPCMessageData { contents: body },
        };
        let send_channel = {
            let inner = self.inner.lock();
            let Some(send_channel) = inner.send_channel.as_ref().cloned() else {
                bail!("send channel is closed");
            };
            send_channel
        };
        let span_id = Span::current().id();
        send_channel
            .try_send((span_id, msg))
            .map_err(|e| eyre!("failed to enqueue safety routed RPC message: {}", e))?;
        Ok(())
    }

    #[cfg_attr(
        feature = "verbose-tracing",
        instrument(level = "trace", skip(self, body), err)
    )]
    fn enqueue_private_routed_message(
        &self,
        direct: RPCMessageHeaderDetailDirect,
        remote_safety_route: PublicKey,
        private_route: PublicKey,
        safety_spec: SafetySpec,
        body: Vec<u8>,
    ) -> EyreResult<()> {
        let header = RPCMessageHeader {
            detail: RPCMessageHeaderDetail::PrivateRouted(RPCMessageHeaderDetailPrivateRouted {
                direct,
                remote_safety_route,
                private_route,
                safety_spec,
            }),
            timestamp: get_aligned_timestamp(),
            body_len: (body.len() as u64).into(),
        };

        let msg = RPCMessageEncoded {
            header,
            data: RPCMessageData { contents: body },
        };

        let send_channel = {
            let inner = self.inner.lock();
            let Some(send_channel) = inner.send_channel.as_ref().cloned() else {
                bail!("send channel is closed");
            };
            send_channel
        };
        let span_id = Span::current().id();
        send_channel
            .try_send((span_id, msg))
            .map_err(|e| eyre!("failed to enqueue private routed RPC message: {}", e))?;
        Ok(())
    }
}
