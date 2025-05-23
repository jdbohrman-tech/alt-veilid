use super::*;

impl_veilid_log_facility!("fanout");

#[derive(Debug)]
struct FanoutContext<'a> {
    fanout_queue: FanoutQueue<'a>,
    result: FanoutResult,
    done: bool,
}

#[derive(Debug, Copy, Clone, Default)]
pub enum FanoutResultKind {
    #[default]
    Incomplete,
    Timeout,
    Consensus,
    Exhausted,
}
impl FanoutResultKind {
    pub fn is_incomplete(&self) -> bool {
        matches!(self, Self::Incomplete)
    }
}

#[derive(Clone, Debug, Default)]
pub struct FanoutResult {
    /// How the fanout completed
    pub kind: FanoutResultKind,
    /// The set of nodes that counted toward consensus
    /// (for example, had the most recent value for this subkey)
    pub consensus_nodes: Vec<NodeRef>,
    /// Which nodes accepted the request
    pub value_nodes: Vec<NodeRef>,
}

impl fmt::Display for FanoutResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let kc = match self.kind {
            FanoutResultKind::Incomplete => "I",
            FanoutResultKind::Timeout => "T",
            FanoutResultKind::Consensus => "C",
            FanoutResultKind::Exhausted => "E",
        };
        if f.alternate() {
            write!(
                f,
                "{}:{}[{}]",
                kc,
                self.consensus_nodes.len(),
                self.consensus_nodes
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
            )
        } else {
            write!(f, "{}:{}", kc, self.consensus_nodes.len())
        }
    }
}

pub fn debug_fanout_results(results: &[FanoutResult]) -> String {
    let mut col = 0;
    let mut out = String::new();
    let mut left = results.len();
    for r in results {
        if col == 0 {
            out += "    ";
        }
        let sr = format!("{}", r);
        out += &sr;
        out += ",";
        col += 1;
        left -= 1;
        if col == 32 && left != 0 {
            col = 0;
            out += "\n"
        }
    }
    out
}

#[derive(Debug)]
pub struct FanoutCallOutput {
    pub peer_info_list: Vec<Arc<PeerInfo>>,
    pub disposition: FanoutCallDisposition,
}

/// The return type of the fanout call routine
#[derive(Debug)]
pub enum FanoutCallDisposition {
    /// The call routine timed out
    Timeout,
    /// The call routine returned an invalid result
    Invalid,
    /// The called node rejected the rpc request but may have returned more nodes
    Rejected,
    /// The called node accepted the rpc request and may have returned more nodes,
    /// but we don't count the result toward our consensus
    Stale,
    /// The called node accepted the rpc request and may have returned more nodes,
    /// counting the result toward our consensus
    Accepted,
    /// The called node accepted the rpc request and may have returned more nodes,
    /// returning a newer value that indicates we should restart our consensus
    AcceptedNewerRestart,
    /// The called node accepted the rpc request and may have returned more nodes,
    /// returning a newer value that indicates our current consensus is stale and should be ignored,
    /// and counting the result toward a new consensus
    AcceptedNewer,
}

pub type FanoutCallResult = Result<FanoutCallOutput, RPCError>;
pub type FanoutNodeInfoFilter = Arc<dyn (Fn(&[TypedPublicKey], &NodeInfo) -> bool) + Send + Sync>;
pub type FanoutCheckDone = Arc<dyn (Fn(&FanoutResult) -> bool) + Send + Sync>;
pub type FanoutCallRoutine =
    Arc<dyn (Fn(NodeRef) -> PinBoxFutureStatic<FanoutCallResult>) + Send + Sync>;

pub fn empty_fanout_node_info_filter() -> FanoutNodeInfoFilter {
    Arc::new(|_, _| true)
}

pub fn capability_fanout_node_info_filter(caps: Vec<Capability>) -> FanoutNodeInfoFilter {
    Arc::new(move |_, ni| ni.has_all_capabilities(&caps))
}

/// Contains the logic for generically searching the Veilid routing table for a set of nodes and applying an
/// RPC operation that eventually converges on satisfactory result, or times out and returns some
/// unsatisfactory but acceptable result. Or something.
///
/// The algorithm starts by creating a 'closest_nodes' working set of the nodes closest to some node id currently in our routing table
/// If has pluggable callbacks:
///  * 'check_done' - for checking for a termination condition
///  * 'call_routine' - routine to call for each node that performs an operation and may add more nodes to our closest_nodes set
///
/// The algorithm is parameterized by:
///  * 'node_count' - the number of nodes to keep in the closest_nodes set
///  * 'fanout' - the number of concurrent calls being processed at the same time
///  * 'consensus_count' - the number of nodes in the processed queue that need to be in the
///    'Accepted' state before we terminate the fanout early.
///
/// The algorithm returns early if 'check_done' returns some value, or if an error is found during the process.
/// If the algorithm times out, a Timeout result is returned, however operations will still have been performed and a
/// timeout is not necessarily indicative of an algorithmic 'failure', just that no definitive stopping condition was found
/// in the given time
pub(crate) struct FanoutCall<'a> {
    routing_table: &'a RoutingTable,
    hash_coordinate: TypedHashDigest,
    node_count: usize,
    fanout_tasks: usize,
    consensus_count: usize,
    timeout_us: TimestampDuration,
    node_info_filter: FanoutNodeInfoFilter,
    call_routine: FanoutCallRoutine,
    check_done: FanoutCheckDone,
}

impl VeilidComponentRegistryAccessor for FanoutCall<'_> {
    fn registry(&self) -> VeilidComponentRegistry {
        self.routing_table.registry()
    }
}

impl<'a> FanoutCall<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        routing_table: &'a RoutingTable,
        hash_coordinate: TypedHashDigest,
        node_count: usize,
        fanout_tasks: usize,
        consensus_count: usize,
        timeout_us: TimestampDuration,
        node_info_filter: FanoutNodeInfoFilter,
        call_routine: FanoutCallRoutine,
        check_done: FanoutCheckDone,
    ) -> Self {
        Self {
            routing_table,
            hash_coordinate,
            node_count,
            fanout_tasks,
            consensus_count,
            timeout_us,
            node_info_filter,
            call_routine,
            check_done,
        }
    }

    #[instrument(level = "trace", target = "fanout", skip_all)]
    fn evaluate_done(&self, ctx: &mut FanoutContext) -> bool {
        // If we already finished, just return
        if ctx.done {
            return true;
        }

        // Calculate fanout result so far
        let fanout_result = ctx.fanout_queue.with_nodes(|nodes, sorted_nodes| {
            // Count up nodes we have seen in order and see if our closest nodes have a consensus
            let mut consensus: Option<bool> = None;
            let mut consensus_nodes: Vec<NodeRef> = vec![];
            let mut value_nodes: Vec<NodeRef> = vec![];
            for sn in sorted_nodes {
                let node = nodes.get(sn).unwrap();
                match node.status {
                    FanoutNodeStatus::Queued | FanoutNodeStatus::InProgress => {
                        // Still have a closer node to do before reaching consensus,
                        // or are doing it still, then wait until those are done
                        if consensus.is_none() {
                            consensus = Some(false);
                        }
                    }
                    FanoutNodeStatus::Timeout
                    | FanoutNodeStatus::Rejected
                    | FanoutNodeStatus::Disqualified => {
                        // Node does not count toward consensus or value node list
                    }
                    FanoutNodeStatus::Stale => {
                        // Node does not count toward consensus but does count toward value node list
                        value_nodes.push(node.node_ref.clone());
                    }
                    FanoutNodeStatus::Accepted => {
                        // Node counts toward consensus and value node list
                        value_nodes.push(node.node_ref.clone());

                        consensus_nodes.push(node.node_ref.clone());
                        if consensus.is_none() && consensus_nodes.len() >= self.consensus_count {
                            consensus = Some(true);
                        }
                    }
                }
            }

            // If we have reached sufficient consensus, return done
            match consensus {
                Some(true) => FanoutResult {
                    kind: FanoutResultKind::Consensus,
                    consensus_nodes,
                    value_nodes,
                },
                Some(false) => FanoutResult {
                    kind: FanoutResultKind::Incomplete,
                    consensus_nodes,
                    value_nodes,
                },
                None => FanoutResult {
                    kind: FanoutResultKind::Exhausted,
                    consensus_nodes,
                    value_nodes,
                },
            }
        });

        let done = (self.check_done)(&fanout_result);
        ctx.result = fanout_result;
        ctx.done = done;
        done
    }

    #[instrument(level = "trace", target = "fanout", skip_all)]
    async fn fanout_processor<'b>(
        &self,
        context: &Mutex<FanoutContext<'b>>,
    ) -> Result<bool, RPCError> {
        // Make a work request channel
        let (work_sender, work_receiver) = flume::bounded(1);

        // Loop until we have a result or are done
        loop {
            // Put in a work request
            {
                let mut context_locked = context.lock();
                context_locked
                    .fanout_queue
                    .request_work(work_sender.clone());
            }

            // Wait around for some work to do
            let Ok(next_node) = work_receiver.recv_async().await else {
                // If we don't have a node to process, stop fanning out
                break Ok(false);
            };

            // Do the call for this node
            match (self.call_routine)(next_node.clone()).await {
                Ok(output) => {
                    // Filter returned nodes
                    let filtered_v: Vec<Arc<PeerInfo>> = output
                        .peer_info_list
                        .into_iter()
                        .filter(|pi| {
                            let node_ids = pi.node_ids().to_vec();
                            if !(self.node_info_filter)(
                                &node_ids,
                                pi.signed_node_info().node_info(),
                            ) {
                                return false;
                            }
                            true
                        })
                        .collect();

                    // Call succeeded
                    // Register the returned nodes and add them to the fanout queue in sorted order
                    let new_nodes = self
                        .routing_table
                        .register_nodes_with_peer_info_list(filtered_v);

                    // Update queue
                    {
                        let mut context_locked = context.lock();
                        context_locked.fanout_queue.add(&new_nodes);

                        // Process disposition of the output of the fanout call routine
                        match output.disposition {
                            FanoutCallDisposition::Timeout => {
                                context_locked.fanout_queue.timeout(next_node);
                            }
                            FanoutCallDisposition::Rejected => {
                                context_locked.fanout_queue.rejected(next_node);
                            }
                            FanoutCallDisposition::Accepted => {
                                context_locked.fanout_queue.accepted(next_node);
                            }
                            FanoutCallDisposition::AcceptedNewerRestart => {
                                context_locked.fanout_queue.all_accepted_to_queued();
                                context_locked.fanout_queue.accepted(next_node);
                            }
                            FanoutCallDisposition::AcceptedNewer => {
                                context_locked.fanout_queue.all_accepted_to_stale();
                                context_locked.fanout_queue.accepted(next_node);
                            }
                            FanoutCallDisposition::Invalid => {
                                // Do nothing with invalid fanout calls
                            }
                            FanoutCallDisposition::Stale => {
                                context_locked.fanout_queue.stale(next_node);
                            }
                        }

                        // See if we're done before going back for more processing
                        if self.evaluate_done(&mut context_locked) {
                            break Ok(true);
                        }

                        // We modified the queue so we may have more work to do now,
                        // tell the queue it should send more work to the workers
                        context_locked.fanout_queue.send_more_work();
                    }
                }
                Err(e) => {
                    break Err(e);
                }
            };
        }
    }

    #[instrument(level = "trace", target = "fanout", skip_all)]
    fn init_closest_nodes(&self, context: &mut FanoutContext) -> Result<(), RPCError> {
        // Get the 'node_count' closest nodes to the key out of our routing table
        let closest_nodes = {
            let node_info_filter = self.node_info_filter.clone();
            let filter = Box::new(
                move |rti: &RoutingTableInner, opt_entry: Option<Arc<BucketEntry>>| {
                    // Exclude our own node
                    if opt_entry.is_none() {
                        return false;
                    }
                    let entry = opt_entry.unwrap();

                    // Filter entries
                    entry.with(rti, |_rti, e| {
                        let Some(signed_node_info) =
                            e.signed_node_info(RoutingDomain::PublicInternet)
                        else {
                            return false;
                        };
                        // Ensure only things that are valid/signed in the PublicInternet domain are returned
                        if !signed_node_info.has_any_signature() {
                            return false;
                        }

                        // Check our node info filter
                        let node_ids = e.node_ids().to_vec();
                        if !(node_info_filter)(&node_ids, signed_node_info.node_info()) {
                            return false;
                        }

                        true
                    })
                },
            ) as RoutingTableEntryFilter;
            let filters = VecDeque::from([filter]);

            let transform = |_rti: &RoutingTableInner, v: Option<Arc<BucketEntry>>| {
                NodeRef::new(self.routing_table.registry(), v.unwrap().clone())
            };

            self.routing_table
                .find_preferred_closest_nodes(
                    self.node_count,
                    self.hash_coordinate,
                    filters,
                    transform,
                )
                .map_err(RPCError::invalid_format)?
        };
        context.fanout_queue.add(&closest_nodes);

        Ok(())
    }

    #[instrument(level = "trace", target = "fanout", skip_all)]
    pub async fn run(&self, init_fanout_queue: Vec<NodeRef>) -> Result<FanoutResult, RPCError> {
        // Create context for this run
        let crypto = self.routing_table.crypto();
        let Some(vcrypto) = crypto.get(self.hash_coordinate.kind) else {
            return Err(RPCError::internal(
                "should not try this on crypto we don't support",
            ));
        };
        let node_sort = Box::new(
            |a_key: &CryptoTyped<PublicKey>,
             b_key: &CryptoTyped<PublicKey>|
             -> core::cmp::Ordering {
                let da =
                    vcrypto.distance(&HashDigest::from(a_key.value), &self.hash_coordinate.value);
                let db =
                    vcrypto.distance(&HashDigest::from(b_key.value), &self.hash_coordinate.value);
                da.cmp(&db)
            },
        );
        let context = Arc::new(Mutex::new(FanoutContext {
            fanout_queue: FanoutQueue::new(
                self.routing_table.registry(),
                self.hash_coordinate.kind,
                node_sort,
                self.consensus_count,
            ),
            result: FanoutResult {
                kind: FanoutResultKind::Incomplete,
                consensus_nodes: vec![],
                value_nodes: vec![],
            },
            done: false,
        }));

        // Get timeout in milliseconds
        let timeout_ms = us_to_ms(self.timeout_us.as_u64()).map_err(RPCError::internal)?;

        // Initialize closest nodes list
        {
            let context_locked = &mut *context.lock();
            self.init_closest_nodes(context_locked)?;

            // Ensure we include the most recent nodes
            context_locked.fanout_queue.add(&init_fanout_queue);

            // Do a quick check to see if we're already done
            if self.evaluate_done(context_locked) {
                return Ok(core::mem::take(&mut context_locked.result));
            }
        }

        // If not, do the fanout
        let mut unord = FuturesUnordered::new();
        {
            // Spin up 'fanout' tasks to process the fanout
            for _ in 0..self.fanout_tasks {
                let h = self.fanout_processor(&context);
                unord.push(h);
            }
        }
        // Wait for them to complete
        match timeout(
            timeout_ms,
            async {
                loop {
                    if let Some(res) = unord.next().in_current_span().await {
                        match res {
                            Ok(is_done) => {
                                if is_done {
                                    break Ok(());
                                }
                            }
                            Err(e) => {
                                break Err(e);
                            }
                        }
                    } else {
                        break Ok(());
                    }
                }
            }
            .in_current_span(),
        )
        .await
        {
            Ok(Ok(())) => {
                // Finished, either by exhaustion or consensus,
                // time to return whatever value we came up with
                let context_locked = &mut *context.lock();
                // Print final queue
                veilid_log!(self debug "Finished FanoutQueue: {}", context_locked.fanout_queue);
                return Ok(core::mem::take(&mut context_locked.result));
            }
            Ok(Err(e)) => {
                // Fanout died with an error
                return Err(e);
            }
            Err(_) => {
                // Timeout, do one last evaluate with remaining nodes in timeout state
                let context_locked = &mut *context.lock();
                context_locked.fanout_queue.all_unfinished_to_timeout();

                // Print final queue
                veilid_log!(self debug "Timeout FanoutQueue: {}", context_locked.fanout_queue);

                // Final evaluate
                if self.evaluate_done(context_locked) {
                    // Last-chance value returned at timeout
                    return Ok(core::mem::take(&mut context_locked.result));
                }

                // We definitely weren't done, so this is just a plain timeout
                let mut result = core::mem::take(&mut context_locked.result);
                result.kind = FanoutResultKind::Timeout;
                return Ok(result);
            }
        }
    }
}
