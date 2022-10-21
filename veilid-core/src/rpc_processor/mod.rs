mod coders;
mod destination;
mod operation_waiter;
mod rpc_app_call;
mod rpc_app_message;
mod rpc_cancel_tunnel;
mod rpc_complete_tunnel;
mod rpc_error;
mod rpc_find_block;
mod rpc_find_node;
mod rpc_get_value;
mod rpc_node_info_update;
mod rpc_return_receipt;
mod rpc_route;
mod rpc_set_value;
mod rpc_signal;
mod rpc_start_tunnel;
mod rpc_status;
mod rpc_supply_block;
mod rpc_validate_dial_info;
mod rpc_value_changed;
mod rpc_watch_value;

pub use coders::*;
pub use destination::*;
pub use operation_waiter::*;
pub use rpc_error::*;

use super::*;
use crate::dht::*;
use crate::xx::*;
use capnp::message::ReaderSegments;
use futures_util::StreamExt;
use network_manager::*;
use receipt_manager::*;
use routing_table::*;
use stop_token::future::FutureExt;

/////////////////////////////////////////////////////////////////////

type OperationId = u64;

/// The decoded header of an RPC message
#[derive(Debug, Clone)]
struct RPCMessageHeader {
    /// Time the message was received, not sent
    timestamp: u64,
    /// The decoded header of the envelope
    envelope: Envelope,
    /// The length in bytes of the rpc message body
    body_len: u64,
    /// The noderef of the peer that sent the message (not the original sender). Ensures node doesn't get evicted from routing table until we're done with it
    peer_noderef: NodeRef,
    /// The connection from the peer sent the message (not the original sender)
    connection_descriptor: ConnectionDescriptor,
    /// The routing domain the message was sent through
    routing_domain: RoutingDomain,
    // The private route the message was received through
    //private_route: Option<DHTKey>,
}

#[derive(Debug)]
struct RPCMessageData {
    contents: Vec<u8>, // rpc messages must be a canonicalized single segment
}

impl ReaderSegments for RPCMessageData {
    fn get_segment(&self, idx: u32) -> Option<&[u8]> {
        if idx > 0 {
            None
        } else {
            Some(self.contents.as_slice())
        }
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
    let wordvec = builder
        .into_reader()
        .canonicalize()
        .map_err(RPCError::protocol)
        .map_err(logthru_rpc!())?;
    Ok(capnp::Word::words_to_bytes(wordvec.as_slice()).to_vec())
}

// fn reader_to_vec<'a, T>(reader: &capnp::message::Reader<T>) -> Result<Vec<u8>, RPCError>
// where
//     T: capnp::message::ReaderSegments + 'a,
// {
//     let wordvec = reader
//         .canonicalize()
//         .map_err(RPCError::protocol)
//         .map_err(logthru_rpc!())?;
//     Ok(capnp::Word::words_to_bytes(wordvec.as_slice()).to_vec())
// }

#[derive(Debug)]
struct WaitableReply {
    handle: OperationWaitHandle<RPCMessage>,
    timeout: u64,
    node_ref: NodeRef,
    send_ts: u64,
    send_data_kind: SendDataKind,
}

/////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Default)]
pub struct Answer<T> {
    pub latency: u64, // how long it took to get this answer
    pub answer: T,    // the answer itself
}
impl<T> Answer<T> {
    pub fn new(latency: u64, answer: T) -> Self {
        Self { latency, answer }
    }
}

struct RenderedOperation {
    message: Vec<u8>,  // The rendered operation bytes
    node_id: DHTKey,   // Destination node id we're sending to
    node_ref: NodeRef, // Node to send envelope to (may not be destination node id in case of relay)
    hop_count: usize,  // Total safety + private route hop count + 1 hop for the initial send
}
/////////////////////////////////////////////////////////////////////

pub struct RPCProcessorInner {
    send_channel: Option<flume::Sender<(Option<Id>, RPCMessageEncoded)>>,
    stop_source: Option<StopSource>,
    worker_join_handles: Vec<MustJoinHandle<()>>,
}

pub struct RPCProcessorUnlockedInner {
    timeout: u64,
    queue_size: u32,
    concurrency: u32,
    max_route_hop_count: usize,
    default_route_hop_count: usize,
    validate_dial_info_receipt_time_ms: u32,
    update_callback: UpdateCallback,
    waiting_rpc_table: OperationWaiter<RPCMessage>,
    waiting_app_call_table: OperationWaiter<Vec<u8>>,
}

#[derive(Clone)]
pub struct RPCProcessor {
    crypto: Crypto,
    config: VeilidConfig,
    network_manager: NetworkManager,
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
        let timeout = ms_to_us(c.network.rpc.timeout_ms);
        let max_route_hop_count = c.network.rpc.max_route_hop_count as usize;
        let default_route_hop_count = c.network.rpc.default_route_hop_count as usize;
        if concurrency == 0 {
            concurrency = intf::get_concurrency() / 2;
            if concurrency == 0 {
                concurrency = 1;
            }
        }
        let validate_dial_info_receipt_time_ms = c.network.dht.validate_dial_info_receipt_time_ms;

        RPCProcessorUnlockedInner {
            timeout,
            queue_size,
            concurrency,
            max_route_hop_count,
            default_route_hop_count,
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

    //////////////////////////////////////////////////////////////////////

    #[instrument(level = "debug", skip_all, err)]
    pub async fn startup(&self) -> EyreResult<()> {
        trace!("startup rpc processor");
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
            let jh = intf::spawn(Self::rpc_worker(
                this,
                inner.stop_source.as_ref().unwrap().token(),
                receiver,
            ));
            inner.worker_join_handles.push(jh);
        }

        Ok(())
    }

    #[instrument(level = "debug", skip_all)]
    pub async fn shutdown(&self) {
        debug!("starting rpc processor shutdown");

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

    /// Determine if a NodeInfo can be placed into the specified routing domain
    fn filter_node_info(&self, routing_domain: RoutingDomain, node_info: &NodeInfo) -> bool {
        let routing_table = self.routing_table();
        routing_table.node_info_is_valid_in_routing_domain(routing_domain, &node_info)
    }

    //////////////////////////////////////////////////////////////////////

    /// Search the DHT for a single node closest to a key and add it to the routing table and return the node reference
    /// If no node was found in the timeout, this returns None
    pub async fn search_dht_single_key(
        &self,
        _node_id: DHTKey,
        _count: u32,
        _fanout: u32,
        _timeout: Option<u64>,
    ) -> Result<Option<NodeRef>, RPCError> {
        //let routing_table = self.routing_table();

        // xxx find node but stop if we find the exact node we want
        // xxx return whatever node is closest after the timeout
        Err(RPCError::unimplemented("search_dht_single_key")).map_err(logthru_rpc!(error))
    }

    /// Search the DHT for the 'count' closest nodes to a key, adding them all to the routing table if they are not there and returning their node references
    pub async fn search_dht_multi_key(
        &self,
        _node_id: DHTKey,
        _count: u32,
        _fanout: u32,
        _timeout: Option<u64>,
    ) -> Result<Vec<NodeRef>, RPCError> {
        // xxx return closest nodes after the timeout
        Err(RPCError::unimplemented("search_dht_multi_key")).map_err(logthru_rpc!(error))
    }

    /// Search the DHT for a specific node corresponding to a key unless we have that node in our routing table already, and return the node reference
    /// Note: This routine can possible be recursive, hence the SendPinBoxFuture async form
    pub fn resolve_node(
        &self,
        node_id: DHTKey,
    ) -> SendPinBoxFuture<Result<Option<NodeRef>, RPCError>> {
        let this = self.clone();
        Box::pin(async move {
            let routing_table = this.routing_table();

            // First see if we have the node in our routing table already
            if let Some(nr) = routing_table.lookup_node_ref(node_id) {
                // ensure we have some dial info for the entry already,
                // if not, we should do the find_node anyway
                if nr.has_any_dial_info() {
                    return Ok(Some(nr));
                }
            }

            // If nobody knows where this node is, ask the DHT for it
            let (count, fanout, timeout) = {
                let c = this.config.get();
                (
                    c.network.dht.resolve_node_count,
                    c.network.dht.resolve_node_fanout,
                    c.network.dht.resolve_node_timeout_ms.map(ms_to_us),
                )
            };

            let nr = this
                .search_dht_single_key(node_id, count, fanout, timeout)
                .await?;

            if let Some(nr) = &nr {
                if nr.node_id() != node_id {
                    // found a close node, but not exact within our configured resolve_node timeout
                    return Ok(None);
                }
            }

            Ok(nr)
        })
    }

    #[instrument(level = "trace", skip(self, waitable_reply), err)]
    async fn wait_for_reply(
        &self,
        waitable_reply: WaitableReply,
    ) -> Result<TimeoutOr<(RPCMessage, u64)>, RPCError> {
        let out = self
            .unlocked_inner
            .waiting_rpc_table
            .wait_for_op(waitable_reply.handle, waitable_reply.timeout)
            .await;
        match &out {
            Err(_) | Ok(TimeoutOr::Timeout) => {
                waitable_reply.node_ref.stats_question_lost();
            }
            Ok(TimeoutOr::Value((rpcreader, _))) => {
                // Reply received
                let recv_ts = intf::get_timestamp();
                waitable_reply.node_ref.stats_answer_rcvd(
                    waitable_reply.send_ts,
                    recv_ts,
                    rpcreader.header.body_len,
                )
            }
        };

        out
    }

    // Wrap an operation with a private route inside a safety route
    pub(super) fn wrap_with_route(
        &self,
        safety_spec: Option<SafetySpec>,
        private_route: PrivateRoute,
        message_data: Vec<u8>,
    ) -> Result<NetworkResult<RenderedOperation>, RPCError> {
        let routing_table = self.routing_table();
        let pr_hop_count = private_route.hop_count;
        let pr_pubkey = private_route.public_key;

        let compiled_route: CompiledRoute =
            match self.routing_table().with_route_spec_store_mut(|rss, rti| {
                // Compile the safety route with the private route
                rss.compile_safety_route(rti, routing_table, safety_spec, private_route)
            })? {
                Some(cr) => cr,
                None => {
                    return Ok(NetworkResult::no_connection_other(
                        "private route could not be compiled at this time",
                    ))
                }
            };

        // Encrypt routed operation
        // Xmsg + ENC(Xmsg, DH(PKapr, SKbsr))
        let nonce = Crypto::get_random_nonce();
        let dh_secret = self
            .crypto
            .cached_dh(&pr_pubkey, &compiled_route.secret)
            .map_err(RPCError::map_internal("dh failed"))?;
        let enc_msg_data = Crypto::encrypt_aead(&message_data, &nonce, &dh_secret, None)
            .map_err(RPCError::map_internal("encryption failed"))?;

        // Make the routed operation
        let operation = RoutedOperation::new(nonce, enc_msg_data);

        // Prepare route operation
        let sr_hop_count = compiled_route.safety_route.hop_count;
        let route_operation = RPCOperationRoute {
            safety_route: compiled_route.safety_route,
            operation,
        };
        let operation = RPCOperation::new_statement(
            RPCStatement::new(RPCStatementDetail::Route(route_operation)),
            None,
        );

        // Convert message to bytes and return it
        let mut route_msg = ::capnp::message::Builder::new_default();
        let mut route_operation = route_msg.init_root::<veilid_capnp::operation::Builder>();
        operation.encode(&mut route_operation)?;
        let out_message = builder_to_vec(route_msg)?;

        // Get the first hop this is going to
        let out_node_id = compiled_route.first_hop.node_id();
        let out_hop_count = (1 + sr_hop_count + pr_hop_count) as usize;

        let out = RenderedOperation {
            message: out_message,
            node_id: out_node_id,
            node_ref: compiled_route.first_hop,
            hop_count: out_hop_count,
        };

        Ok(NetworkResult::value(out))
    }

    /// Produce a byte buffer that represents the wire encoding of the entire
    /// unencrypted envelope body for a RPC message. This incorporates
    /// wrapping a private and/or safety route if they are specified.
    #[instrument(level = "debug", skip(self, operation), err)]
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

        // To where are we sending the request
        match dest {
            Destination::Direct {
                target: ref node_ref,
                safety_spec,
            }
            | Destination::Relay {
                relay: ref node_ref,
                target: _,
                safety_spec,
            } => {
                // Send to a node without a private route
                // --------------------------------------

                // Get the actual destination node id accounting for relays
                let (node_ref, node_id) = if let Destination::Relay {
                    relay: _,
                    target: ref dht_key,
                    safety_spec: _,
                } = dest
                {
                    (node_ref.clone(), dht_key.clone())
                } else {
                    let node_id = node_ref.node_id();
                    (node_ref.clone(), node_id)
                };

                // Handle the existence of safety route
                match safety_spec {
                    None => {
                        // If no safety route is being used, and we're not sending to a private
                        // route, we can use a direct envelope instead of routing
                        out = NetworkResult::value(RenderedOperation {
                            message,
                            node_id,
                            node_ref,
                            hop_count: 1,
                        });
                    }
                    Some(safety_spec) => {
                        // No private route was specified for the request
                        // but we are using a safety route, so we must create an empty private route
                        let private_route = PrivateRoute::new_stub(node_id);

                        // Wrap with safety route
                        out = self.wrap_with_route(Some(safety_spec), private_route, message)?;
                    }
                };
            }
            Destination::PrivateRoute {
                private_route,
                safety_spec,
                reliable,
            } => {
                // Send to private route
                // ---------------------
                // Reply with 'route' operation
                out = self.wrap_with_route(safety_spec, private_route, message)?;
            }
        }

        Ok(out)
    }

    // Get signed node info to package with RPC messages to improve
    // routing table caching when it is okay to do so
    // This is only done in the PublicInternet routing domain because
    // as far as we can tell this is the only domain that will really benefit
    fn get_sender_signed_node_info(&self, dest: &Destination) -> Option<SignedNodeInfo> {
        // Don't do this if the sender is to remain private
        // Otherwise we would be attaching the original sender's identity to the final destination,
        // thus defeating the purpose of the safety route entirely :P
        if dest.get_safety_spec().is_some() {
            return None;
        }
        // Don't do this if our own signed node info isn't valid yet
        let routing_table = self.routing_table();
        if !routing_table.has_valid_own_node_info(RoutingDomain::PublicInternet) {
            return None;
        }

        match dest {
            Destination::Direct {
                target,
                safety_spec: _,
            } => {
                // If the target has seen our node info already don't do this
                if target.has_seen_our_node_info(RoutingDomain::PublicInternet) {
                    return None;
                }
                Some(routing_table.get_own_signed_node_info(RoutingDomain::PublicInternet))
            }
            Destination::Relay {
                relay: _,
                target,
                safety_spec: _,
            } => {
                if let Some(target) = routing_table.lookup_node_ref(*target) {
                    if target.has_seen_our_node_info(RoutingDomain::PublicInternet) {
                        return None;
                    }
                    Some(routing_table.get_own_signed_node_info(RoutingDomain::PublicInternet))
                } else {
                    None
                }
            }
            Destination::PrivateRoute {
                private_route: _,
                safety_spec: _,
                reliable: _,
            } => None,
        }
    }

    // Issue a question over the network, possibly using an anonymized route
    #[instrument(level = "debug", skip(self, question), err)]
    async fn question(
        &self,
        dest: Destination,
        question: RPCQuestion,
    ) -> Result<NetworkResult<WaitableReply>, RPCError> {
        // Get sender info if we should send that
        let opt_sender_info = self.get_sender_signed_node_info(&dest);

        // Wrap question in operation
        let operation = RPCOperation::new_question(question, opt_sender_info);
        let op_id = operation.op_id();

        // Log rpc send
        debug!(target: "rpc_message", dir = "send", kind = "question", op_id, desc = operation.kind().desc(), ?dest);

        // Produce rendered operation
        let RenderedOperation {
            message,
            node_id,
            node_ref,
            hop_count,
        } = network_result_try!(self.render_operation(dest, &operation)?);

        // Calculate answer timeout
        // Timeout is number of hops times the timeout per hop
        let timeout = self.unlocked_inner.timeout * (hop_count as u64);

        // Set up op id eventual
        let handle = self.unlocked_inner.waiting_rpc_table.add_op_waiter(op_id);

        // Send question
        let bytes = message.len() as u64;
        let send_ts = intf::get_timestamp();
        let send_data_kind = network_result_try!(self
            .network_manager()
            .send_envelope(node_ref.clone(), Some(node_id), message)
            .await
            .map_err(|e| {
                // If we're returning an error, clean up
                node_ref
                    .stats_failed_to_send(send_ts, true);
                RPCError::network(e)
            })? => {
                // If we couldn't send we're still cleaning up
                node_ref
                    .stats_failed_to_send(send_ts, true);
            }
        );

        // Successfully sent
        node_ref.stats_question_sent(send_ts, bytes, true);

        // Pass back waitable reply completion
        Ok(NetworkResult::value(WaitableReply {
            handle,
            timeout,
            node_ref,
            send_ts,
            send_data_kind,
        }))
    }

    // Issue a statement over the network, possibly using an anonymized route
    #[instrument(level = "debug", skip(self, statement), err)]
    async fn statement(
        &self,
        dest: Destination,
        statement: RPCStatement,
    ) -> Result<NetworkResult<()>, RPCError> {
        // Get sender info if we should send that
        let opt_sender_info = self.get_sender_signed_node_info(&dest);

        // Wrap statement in operation
        let operation = RPCOperation::new_statement(statement, opt_sender_info);

        // Log rpc send
        debug!(target: "rpc_message", dir = "send", kind = "statement", op_id = operation.op_id(), desc = operation.kind().desc(), ?dest);

        // Produce rendered operation
        let RenderedOperation {
            message,
            node_id,
            node_ref,
            hop_count: _,
        } = network_result_try!(self.render_operation(dest, &operation)?);

        // Send statement
        let bytes = message.len() as u64;
        let send_ts = intf::get_timestamp();
        let _send_data_kind = network_result_try!(self
            .network_manager()
            .send_envelope(node_ref.clone(), Some(node_id), message)
            .await
            .map_err(|e| {
                // If we're returning an error, clean up
                node_ref
                    .stats_failed_to_send(send_ts, true);
                RPCError::network(e)
            })? => {
                // If we couldn't send we're still cleaning up
                node_ref
                    .stats_failed_to_send(send_ts, true);
            }
        );

        // Successfully sent
        node_ref.stats_question_sent(send_ts, bytes, true);

        Ok(NetworkResult::value(()))
    }

    // Convert the 'RespondTo' into a 'Destination' for a response
    fn get_respond_to_destination(&self, request: &RPCMessage) -> Destination {
        // Get the question 'respond to'
        let respond_to = match request.operation.kind() {
            RPCOperationKind::Question(q) => q.respond_to(),
            _ => {
                panic!("not a question");
            }
        };

        // To where should we respond?
        match respond_to {
            RespondTo::Sender => {
                // Reply directly to the request's source
                let sender_id = request.header.envelope.get_sender_id();

                // This may be a different node's reference than the 'sender' in the case of a relay
                let peer_noderef = request.header.peer_noderef.clone();

                // If the sender_id is that of the peer, then this is a direct reply
                // else it is a relayed reply through the peer
                if peer_noderef.node_id() == sender_id {
                    Destination::direct(peer_noderef)
                } else {
                    Destination::relay(peer_noderef, sender_id)
                }
            }
            RespondTo::PrivateRoute(pr) => Destination::private_route(
                pr.clone(),
                request
                    .header
                    .connection_descriptor
                    .protocol_type()
                    .is_connection_oriented(),
            ),
        }
    }

    // Issue a reply over the network, possibly using an anonymized route
    // The request must want a response, or this routine fails
    #[instrument(level = "debug", skip(self, request, answer), err)]
    async fn answer(
        &self,
        request: RPCMessage,
        answer: RPCAnswer,
    ) -> Result<NetworkResult<()>, RPCError> {
        // Extract destination from respond_to
        let dest = self.get_respond_to_destination(&request);

        // Get sender info if we should send that
        let opt_sender_info = self.get_sender_signed_node_info(&dest);

        // Wrap answer in operation
        let operation = RPCOperation::new_answer(&request.operation, answer, opt_sender_info);

        // Log rpc send
        debug!(target: "rpc_message", dir = "send", kind = "answer", op_id = operation.op_id(), desc = operation.kind().desc(), ?dest);

        // Produce rendered operation
        let RenderedOperation {
            message,
            node_id,
            node_ref,
            hop_count: _,
        } = network_result_try!(self.render_operation(dest, &operation)?);

        // Send the reply
        let bytes = message.len() as u64;
        let send_ts = intf::get_timestamp();
        network_result_try!(self.network_manager()
            .send_envelope(node_ref.clone(), Some(node_id), message)
            .await
            .map_err(|e| {
                // If we're returning an error, clean up
                node_ref
                    .stats_failed_to_send(send_ts, true);
                RPCError::network(e)
            })? => {
                // If we couldn't send we're still cleaning up
                node_ref
                    .stats_failed_to_send(send_ts, false);
            }
        );

        // Reply successfully sent
        node_ref.stats_answer_sent(bytes);

        Ok(NetworkResult::value(()))
    }

    //////////////////////////////////////////////////////////////////////
    #[instrument(level = "trace", skip(self, encoded_msg), err)]
    async fn process_rpc_message_version_0(
        &self,
        encoded_msg: RPCMessageEncoded,
    ) -> Result<(), RPCError> {
        // Get the routing domain this message came over
        let routing_domain = encoded_msg.header.routing_domain;

        // Decode the operation
        let sender_node_id = encoded_msg.header.envelope.get_sender_id();

        // Decode the RPC message
        let operation = {
            let reader = capnp::message::Reader::new(encoded_msg.data, Default::default());
            let op_reader = reader
                .get_root::<veilid_capnp::operation::Reader>()
                .map_err(RPCError::protocol)
                .map_err(logthru_rpc!())?;
            RPCOperation::decode(&op_reader, &sender_node_id)?
        };

        // Get the sender noderef, incorporating and 'sender node info'
        let mut opt_sender_nr: Option<NodeRef> = None;
        if let Some(sender_node_info) = operation.sender_node_info() {
            // Sender NodeInfo was specified, update our routing table with it
            if !self.filter_node_info(routing_domain, &sender_node_info.node_info) {
                return Err(RPCError::invalid_format(
                    "sender signednodeinfo has invalid peer scope",
                ));
            }
            opt_sender_nr = self.routing_table().register_node_with_signed_node_info(
                routing_domain,
                sender_node_id,
                sender_node_info.clone(),
                false,
            );
        }

        // look up sender node, in case it's different than our peer due to relaying
        if opt_sender_nr.is_none() {
            opt_sender_nr = self.routing_table().lookup_node_ref(sender_node_id)
        }

        // Mark this sender as having seen our node info over this routing domain
        // because it managed to reach us over that routing domain
        if let Some(sender_nr) = &opt_sender_nr {
            sender_nr.set_seen_our_node_info(routing_domain);
        }

        // Make the RPC message
        let msg = RPCMessage {
            header: encoded_msg.header,
            operation,
            opt_sender_nr,
        };

        // Process stats
        let kind = match msg.operation.kind() {
            RPCOperationKind::Question(_) => {
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
        debug!(target: "rpc_message", dir = "recv", kind, op_id = msg.operation.op_id(), desc = msg.operation.kind().desc(), sender_id = ?sender_node_id);

        // Process specific message kind
        match msg.operation.kind() {
            RPCOperationKind::Question(q) => match q.detail() {
                RPCQuestionDetail::StatusQ(_) => self.process_status_q(msg).await,
                RPCQuestionDetail::FindNodeQ(_) => self.process_find_node_q(msg).await,
                RPCQuestionDetail::AppCallQ(_) => self.process_app_call_q(msg).await,
                RPCQuestionDetail::GetValueQ(_) => self.process_get_value_q(msg).await,
                RPCQuestionDetail::SetValueQ(_) => self.process_set_value_q(msg).await,
                RPCQuestionDetail::WatchValueQ(_) => self.process_watch_value_q(msg).await,
                RPCQuestionDetail::SupplyBlockQ(_) => self.process_supply_block_q(msg).await,
                RPCQuestionDetail::FindBlockQ(_) => self.process_find_block_q(msg).await,
                RPCQuestionDetail::StartTunnelQ(_) => self.process_start_tunnel_q(msg).await,
                RPCQuestionDetail::CompleteTunnelQ(_) => self.process_complete_tunnel_q(msg).await,
                RPCQuestionDetail::CancelTunnelQ(_) => self.process_cancel_tunnel_q(msg).await,
            },
            RPCOperationKind::Statement(s) => match s.detail() {
                RPCStatementDetail::ValidateDialInfo(_) => {
                    self.process_validate_dial_info(msg).await
                }
                RPCStatementDetail::Route(_) => self.process_route(msg).await,
                RPCStatementDetail::NodeInfoUpdate(_) => self.process_node_info_update(msg).await,
                RPCStatementDetail::ValueChanged(_) => self.process_value_changed(msg).await,
                RPCStatementDetail::Signal(_) => self.process_signal(msg).await,
                RPCStatementDetail::ReturnReceipt(_) => self.process_return_receipt(msg).await,
                RPCStatementDetail::AppMessage(_) => self.process_app_message(msg).await,
            },
            RPCOperationKind::Answer(_) => {
                self.unlocked_inner
                    .waiting_rpc_table
                    .complete_op_waiter(msg.operation.op_id(), msg)
                    .await
            }
        }
    }

    #[instrument(level = "trace", skip(self, msg), err)]
    async fn process_rpc_message(&self, msg: RPCMessageEncoded) -> Result<(), RPCError> {
        if msg.header.envelope.get_version() == 0 {
            self.process_rpc_message_version_0(msg).await
        } else {
            Err(RPCError::Internal(format!(
                "unsupported envelope version: {}, newest supported is version 0",
                msg.header.envelope.get_version()
            )))
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
            let _ = self
                .process_rpc_message(msg)
                .instrument(rpc_worker_span)
                .await
                .map_err(logthru_rpc!("couldn't process rpc message"));
        }
    }

    #[instrument(level = "trace", skip(self, body), err)]
    pub fn enqueue_message(
        &self,
        envelope: Envelope,
        body: Vec<u8>,
        peer_noderef: NodeRef,
        connection_descriptor: ConnectionDescriptor,
        routing_domain: RoutingDomain,
    ) -> EyreResult<()> {
        let msg = RPCMessageEncoded {
            header: RPCMessageHeader {
                timestamp: intf::get_timestamp(),
                envelope,
                body_len: body.len() as u64,
                peer_noderef,
                connection_descriptor,
                routing_domain,
            },
            data: RPCMessageData { contents: body },
        };
        let send_channel = {
            let inner = self.inner.lock();
            inner.send_channel.as_ref().unwrap().clone()
        };
        let span_id = Span::current().id();
        send_channel
            .try_send((span_id, msg))
            .wrap_err("failed to enqueue received RPC message")?;
        Ok(())
    }
}
