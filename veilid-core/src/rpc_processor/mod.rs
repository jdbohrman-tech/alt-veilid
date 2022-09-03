mod coders;
mod destination;
mod private_route;
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

pub use destination::*;
pub use private_route::*;
pub use rpc_error::*;

use super::*;
use crate::dht::*;
use crate::xx::*;
use capnp::message::ReaderSegments;
use coders::*;
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

fn builder_to_vec<'a, T>(builder: capnp::message::Builder<T>) -> Result<Vec<u8>, RPCError>
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
    op_id: OperationId,
    eventual: EventualValue<(Option<Id>, RPCMessage)>,
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
    message: Vec<u8>,          // The rendered operation bytes
    node_id: DHTKey,           // Node id we're sending to
    node_ref: Option<NodeRef>, // Node to send envelope to (may not be destination node id in case of relay)
    hop_count: usize,          // Total safety + private route hop count
}
/////////////////////////////////////////////////////////////////////

pub struct RPCProcessorInner {
    network_manager: NetworkManager,
    routing_table: RoutingTable,
    node_id: DHTKey,
    node_id_secret: DHTKeySecret,
    send_channel: Option<flume::Sender<(Option<Id>, RPCMessageEncoded)>>,
    timeout: u64,
    max_route_hop_count: usize,
    waiting_rpc_table: BTreeMap<OperationId, EventualValue<(Option<Id>, RPCMessage)>>,
    stop_source: Option<StopSource>,
    worker_join_handles: Vec<MustJoinHandle<()>>,
}

#[derive(Clone)]
pub struct RPCProcessor {
    crypto: Crypto,
    config: VeilidConfig,
    inner: Arc<Mutex<RPCProcessorInner>>,
}

impl RPCProcessor {
    fn new_inner(network_manager: NetworkManager) -> RPCProcessorInner {
        RPCProcessorInner {
            network_manager: network_manager.clone(),
            routing_table: network_manager.routing_table(),
            node_id: DHTKey::default(),
            node_id_secret: DHTKeySecret::default(),
            send_channel: None,
            timeout: 10000000,
            max_route_hop_count: 7,
            waiting_rpc_table: BTreeMap::new(),
            stop_source: None,
            worker_join_handles: Vec::new(),
        }
    }
    pub fn new(network_manager: NetworkManager) -> Self {
        Self {
            crypto: network_manager.crypto(),
            config: network_manager.config(),
            inner: Arc::new(Mutex::new(Self::new_inner(network_manager))),
        }
    }

    pub fn network_manager(&self) -> NetworkManager {
        self.inner.lock().network_manager.clone()
    }

    pub fn routing_table(&self) -> RoutingTable {
        self.inner.lock().routing_table.clone()
    }

    pub fn node_id(&self) -> DHTKey {
        self.inner.lock().node_id
    }

    pub fn node_id_secret(&self) -> DHTKeySecret {
        self.inner.lock().node_id_secret
    }

    //////////////////////////////////////////////////////////////////////

    /// Determine if a NodeInfo can be placed into the specified routing domain
    fn filter_node_info(&self, routing_domain: RoutingDomain, node_info: &NodeInfo) -> bool {
        // reject attempts to include non-public addresses in results
        for did in &node_info.dial_info_detail_list {
            if !did.dial_info.is_global() {
                // non-public address causes rejection
                return false;
            }
        }
        if let Some(rpi) = &node_info.relay_peer_info {
            for did in &rpi.signed_node_info.node_info.dial_info_detail_list {
                if !did.dial_info.is_global() {
                    // non-public address causes rejection
                    return false;
                }
            }
        }
        true
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

    // set up wait for reply
    fn add_op_id_waiter(&self, op_id: OperationId) -> EventualValue<(Option<Id>, RPCMessage)> {
        let mut inner = self.inner.lock();
        let e = EventualValue::new();
        inner.waiting_rpc_table.insert(op_id, e.clone());
        e
    }

    // remove wait for reply
    fn cancel_op_id_waiter(&self, op_id: OperationId) {
        let mut inner = self.inner.lock();
        inner.waiting_rpc_table.remove(&op_id);
    }

    // complete the reply
    #[instrument(level = "trace", skip(self, msg), fields(msg.operation.op_id), err)]
    async fn complete_op_id_waiter(&self, msg: RPCMessage) -> Result<(), RPCError> {
        let op_id = msg.operation.op_id();
        let eventual = {
            let mut inner = self.inner.lock();
            inner
                .waiting_rpc_table
                .remove(&op_id)
                .ok_or_else(RPCError::else_internal(format!(
                    "Unmatched operation id: {:#?}",
                    msg
                )))?
        };
        eventual.resolve((Span::current().id(), msg)).await;
        Ok(())
    }

    // wait for reply
    async fn do_wait_for_reply(
        &self,
        waitable_reply: &WaitableReply,
    ) -> Result<TimeoutOr<(RPCMessage, u64)>, RPCError> {
        let timeout_ms = u32::try_from(waitable_reply.timeout / 1000u64)
            .map_err(RPCError::map_internal("invalid timeout"))?;
        // wait for eventualvalue
        let start_ts = intf::get_timestamp();
        let res = intf::timeout(timeout_ms, waitable_reply.eventual.instance())
            .await
            .into_timeout_or();
        Ok(res.map(|res| {
            let (span_id, rpcreader) = res.take_value().unwrap();
            let end_ts = intf::get_timestamp();

            // fixme: causes crashes? "Missing otel data span extensions"??
            //Span::current().follows_from(span_id);

            (rpcreader, end_ts - start_ts)
        }))
    }

    #[instrument(level = "trace", skip(self, waitable_reply), err)]
    async fn wait_for_reply(
        &self,
        waitable_reply: WaitableReply,
    ) -> Result<TimeoutOr<(RPCMessage, u64)>, RPCError> {
        let out = self.do_wait_for_reply(&waitable_reply).await;
        match &out {
            Err(_) | Ok(TimeoutOr::Timeout) => {
                self.cancel_op_id_waiter(waitable_reply.op_id);

                waitable_reply.node_ref.stats_question_lost();
            }
            Ok(TimeoutOr::Value((rpcreader, _))) => {
                // Note that the remote node definitely received this node info since we got a reply
                waitable_reply.node_ref.set_seen_our_node_info();

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

    /// Produce a byte buffer that represents the wire encoding of the entire
    /// unencrypted envelope body for a RPC message. This incorporates
    /// wrapping a private and/or safety route if they are specified.
    #[instrument(level = "debug", skip(self, operation), err)]
    fn render_operation(
        &self,
        dest: Destination,
        operation: &RPCOperation,
    ) -> Result<RenderedOperation, RPCError> {
        let out_node_id; // Envelope Node Id
        let mut out_node_ref: Option<NodeRef> = None; // Node to send envelope to
        let out_hop_count: usize; // Total safety + private route hop count
        let out_message; // Envelope data

        // Encode message to a builder and make a message reader for it
        // Then produce the message as an unencrypted byte buffer
        let message_vec = {
            let mut msg_builder = ::capnp::message::Builder::new_default();
            let mut op_builder = msg_builder.init_root::<veilid_capnp::operation::Builder>();
            operation.encode(&mut op_builder)?;
            builder_to_vec(msg_builder)?
        };

        // To where are we sending the request
        match dest {
            Destination::Direct {
                target: node_ref,
                routing_domain,
                safety_route_spec,
            }
            | Destination::Relay {
                relay: node_ref,
                target: _,
                routing_domain,
                safety_route_spec,
            } => {
                // Send to a node without a private route
                // --------------------------------------

                // Get the actual destination node id accounting for relays
                let (node_ref, node_id) = if let Destination::Relay {
                    relay: _,
                    target: dht_key,
                    routing_domain: _,
                    safety_route_spec: _,
                } = dest
                {
                    (node_ref.clone(), dht_key.clone())
                } else {
                    let node_id = node_ref.node_id();
                    (node_ref.clone(), node_id)
                };

                // Handle the existence of safety route
                match safety_route_spec {
                    None => {
                        // If no safety route is being used, and we're not sending to a private
                        // route, we can use a direct envelope instead of routing
                        out_message = message_vec;

                        // Message goes directly to the node
                        out_node_id = node_id;
                        out_node_ref = Some(node_ref);
                        out_hop_count = 1;
                    }
                    Some(sr) => {
                        // No private route was specified for the request
                        // but we are using a safety route, so we must create an empty private route
                        let private_route = PrivateRoute::new_stub(node_id);

                        // first
                        out_node_id = sr
                            .hops
                            .first()
                            .ok_or_else(RPCError::else_internal("no hop in safety route"))?
                            .dial_info
                            .node_id
                            .key;
                        out_message = self.wrap_with_route(Some(sr), private_route, message_vec)?;
                        out_hop_count = 1 + sr.hops.len();
                    }
                };
            }
            Destination::PrivateRoute {
                private_route,
                safety_route_spec,
            } => {
                // Send to private route
                // ---------------------
                // Reply with 'route' operation
                out_node_id = match safety_route_spec {
                    None => {
                        // If no safety route, the first node is the first hop of the private route
                        out_hop_count = private_route.hop_count as usize;
                        let out_node_id = match &private_route.hops {
                            Some(rh) => rh.dial_info.node_id.key,
                            _ => return Err(RPCError::internal("private route has no hops")),
                        };
                        out_message = self.wrap_with_route(None, private_route, message_vec)?;
                        out_node_id
                    }
                    Some(sr) => {
                        // If safety route is in use, first node is the first hop of the safety route
                        out_hop_count = 1 + sr.hops.len() + (private_route.hop_count as usize);
                        let out_node_id = sr
                            .hops
                            .first()
                            .ok_or_else(RPCError::else_internal("no hop in safety route"))?
                            .dial_info
                            .node_id
                            .key;
                        out_message = self.wrap_with_route(Some(sr), private_route, message_vec)?;
                        out_node_id
                    }
                }
            }
        }

        // Verify hop count isn't larger than out maximum routed hop count
        if out_hop_count > self.inner.lock().max_route_hop_count {
            return Err(RPCError::internal("hop count too long for route"))
                .map_err(logthru_rpc!(warn));
        }

        Ok(RenderedOperation {
            message: out_message,
            node_id: out_node_id,
            node_ref: out_node_ref,
            hop_count: out_hop_count,
        })
    }

    // Issue a question over the network, possibly using an anonymized route
    #[instrument(level = "debug", skip(self, question), err)]
    async fn question(
        &self,
        dest: Destination,
        question: RPCQuestion,
    ) -> Result<NetworkResult<WaitableReply>, RPCError> {
        
        // Get sender info if we should send that
        let opt_sender_info = if dest.safety_route_spec().is_none() && matches!(question.respond_to(), RespondTo::Sender) {
            // Sender is not private, send sender info if needed
            // Get the noderef of the eventual destination or first route hop
            if let Some(target_nr) = self.routing_table().lookup_node_ref(dest.get_target_id()) {
                if target_nr.has_seen_our_node_info(R)
            }
        }

        // Wrap question in operation
        let operation = RPCOperation::new_question(question);
        let op_id = operation.op_id();

        // Log rpc send
        debug!(target: "rpc_message", dir = "send", kind = "question", op_id, desc = operation.kind().desc(), ?dest);

        // Produce rendered operation
        let RenderedOperation {
            message,
            node_id,
            node_ref,
            hop_count,
        } = self.render_operation(dest, &operation, safety_route_spec)?;

        // If we need to resolve the first hop, do it
        let node_ref = match node_ref {
            None => match self.resolve_node(node_id).await? {
                None => {
                    return Ok(NetworkResult::no_connection_other(node_id));
                }
                Some(nr) => nr,
            },
            Some(nr) => nr,
        };

        // Calculate answer timeout
        // Timeout is number of hops times the timeout per hop
        let timeout = self.inner.lock().timeout * (hop_count as u64);

        // Set up op id eventual
        let eventual = self.add_op_id_waiter(op_id);

        // Send question
        let bytes = message.len() as u64;
        let send_ts = intf::get_timestamp();
        let send_data_kind = network_result_try!(self
            .network_manager()
            .send_envelope(node_ref.clone(), Some(node_id), message)
            .await
            .map_err(|e| {
                // If we're returning an error, clean up
                self.cancel_op_id_waiter(op_id);
                node_ref
                    .stats_failed_to_send(send_ts, true);
                RPCError::network(e)
            })? => {
                // If we couldn't send we're still cleaning up
                self.cancel_op_id_waiter(op_id);
                node_ref
                    .stats_failed_to_send(send_ts, true);
            }
        );

        // Successfully sent
        node_ref.stats_question_sent(send_ts, bytes, true);

        // Pass back waitable reply completion
        Ok(NetworkResult::value(WaitableReply {
            op_id,
            eventual,
            timeout,
            node_ref,
            send_ts,
            send_data_kind,
        }))
    }

    // Issue a statement over the network, possibly using an anonymized route
    #[instrument(level = "debug", skip(self, statement, safety_route_spec), err)]
    async fn statement(
        &self,
        dest: Destination,
        statement: RPCStatement,
    ) -> Result<NetworkResult<()>, RPCError> {
        // Wrap statement in operation
        let operation = RPCOperation::new_statement(statement);

        // Log rpc send
        debug!(target: "rpc_message", dir = "send", kind = "statement", op_id = operation.op_id(), desc = operation.kind().desc(), ?dest);

        // Produce rendered operation
        let RenderedOperation {
            message,
            node_id,
            node_ref,
            hop_count: _,
        } = self.render_operation(dest, &operation, safety_route_spec)?;

        // If we need to resolve the first hop, do it
        let node_ref = match node_ref {
            None => match self.resolve_node(node_id).await? {
                None => {
                    return Ok(NetworkResult::no_connection_other(node_id));
                }
                Some(nr) => nr,
            },
            Some(nr) => nr,
        };

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
            RespondTo::Sender(_) => {
                // Reply directly to the request's source
                let sender_id = request.header.envelope.get_sender_id();

                // This may be a different node's reference than the 'sender' in the case of a relay
                let peer_noderef = request.header.peer_noderef.clone();

                // If the sender_id is that of the peer, then this is a direct reply
                // else it is a relayed reply through the peer
                if peer_noderef.node_id() == sender_id {
                    Destination::Direct(peer_noderef)
                } else {
                    Destination::Relay(peer_noderef, sender_id)
                }
            }
            RespondTo::PrivateRoute(pr) => Destination::PrivateRoute(pr.clone()),
        }
    }

    // Issue a reply over the network, possibly using an anonymized route
    // The request must want a response, or this routine fails
    #[instrument(level = "debug", skip(self, request, answer, safety_route_spec), err)]
    async fn answer(
        &self,
        request: RPCMessage,
        answer: RPCAnswer,
    ) -> Result<NetworkResult<()>, RPCError> {
        // Wrap answer in operation
        let operation = RPCOperation::new_answer(&request.operation, answer);

        // Extract destination from respond_to
        let dest = self.get_respond_to_destination(&request);

        // Log rpc send
        debug!(target: "rpc_message", dir = "send", kind = "answer", op_id = operation.op_id(), desc = operation.kind().desc(), ?dest);

        // Produce rendered operation
        let RenderedOperation {
            message,
            node_id,
            node_ref,
            hop_count: _,
        } = self.render_operation(dest, &operation, safety_route_spec)?;

        // If we need to resolve the first hop, do it
        let node_ref = match node_ref {
            None => match self.resolve_node(node_id).await? {
                None => {
                    return Ok(NetworkResult::no_connection_other(node_id));
                }
                Some(nr) => nr,
            },
            Some(nr) => nr,
        };

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
        
        // Get the routing domain
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

        // Get the sender noderef, incorporating and 'sender node info' we have from a question
        let mut opt_sender_nr: Option<NodeRef> = None;
        match operation.kind() {
            RPCOperationKind::Question(q) => {
                match q.respond_to() {
                    RespondTo::Sender(Some(sender_ni)) => {
                        // Sender NodeInfo was specified, update our routing table with it
                        if !self.filter_node_info(&sender_ni.node_info) {
                            return Err(RPCError::invalid_format(
                                "respond_to_sender_signed_node_info has invalid peer scope",
                            ));
                        }
                        opt_sender_nr = self.routing_table().register_node_with_signed_node_info(
                            routing_domain,
                            sender_node_id,
                            sender_ni.clone(),
                            false,
                        );
                    }
                    _ => {}
                }
            }
            _ => {}
        };
        if opt_sender_nr.is_none() {
            // look up sender node, in case it's different than our peer due to relaying
            opt_sender_nr = self.routing_table().lookup_node_ref(sender_node_id)
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
            },
            RPCOperationKind::Answer(_) => self.complete_op_id_waiter(msg).await,
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
        while let Ok(Ok((span_id, msg))) =
            receiver.recv_async().timeout_at(stop_token.clone()).await
        {
            let rpc_worker_span = span!(parent: None, Level::TRACE, "rpc_worker");
            // fixme: causes crashes? "Missing otel data span extensions"??
            //rpc_worker_span.follows_from(span_id);
            let _enter = rpc_worker_span.enter();

            let _ = self
                .process_rpc_message(msg)
                .await
                .map_err(logthru_rpc!("couldn't process rpc message"));
        }
    }

    #[instrument(level = "debug", skip_all, err)]
    pub async fn startup(&self) -> EyreResult<()> {
        trace!("startup rpc processor");
        let mut inner = self.inner.lock();
        // make local copy of node id for easy access
        let c = self.config.get();
        inner.node_id = c.network.node_id;
        inner.node_id_secret = c.network.node_id_secret;

        // set up channel
        let mut concurrency = c.network.rpc.concurrency;
        let mut queue_size = c.network.rpc.queue_size;
        let mut timeout = ms_to_us(c.network.rpc.timeout_ms);
        let mut max_route_hop_count = c.network.rpc.max_route_hop_count as usize;
        if concurrency == 0 {
            concurrency = intf::get_concurrency() / 2;
            if concurrency == 0 {
                concurrency = 1;
            }
        }
        if queue_size == 0 {
            queue_size = 1024;
        }
        if timeout == 0 {
            timeout = 10000000;
        }
        if max_route_hop_count == 0 {
            max_route_hop_count = 7usize;
        }
        inner.timeout = timeout;
        inner.max_route_hop_count = max_route_hop_count;
        let channel = flume::bounded(queue_size as usize);
        inner.send_channel = Some(channel.0.clone());
        inner.stop_source = Some(StopSource::new());

        // spin up N workers
        trace!("Spinning up {} RPC workers", concurrency);
        for _ in 0..concurrency {
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
        *self.inner.lock() = Self::new_inner(self.network_manager());

        debug!("finished rpc processor shutdown");
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
