use super::*;

struct FanoutContext<R>
where
    R: Unpin,
{
    fanout_queue: FanoutQueue,
    result: Option<Result<R, RPCError>>,
}

#[derive(Debug, Copy, Clone)]
pub enum FanoutResultKind {
    Partial,
    Timeout,
    Finished,
    Exhausted,
}
impl FanoutResultKind {
    pub fn is_partial(&self) -> bool {
        matches!(self, Self::Partial)
    }
}

#[derive(Debug, Clone)]
pub struct FanoutResult {
    pub kind: FanoutResultKind,
    pub value_nodes: Vec<NodeRef>,
}

pub fn debug_fanout_result(result: &FanoutResult) -> String {
    let kc = match result.kind {
        FanoutResultKind::Partial => "P",
        FanoutResultKind::Timeout => "T",
        FanoutResultKind::Finished => "F",
        FanoutResultKind::Exhausted => "E",
    };
    format!("{}:{}", kc, result.value_nodes.len())
}

pub fn debug_fanout_results(results: &[FanoutResult]) -> String {
    let mut col = 0;
    let mut out = String::new();
    let mut left = results.len();
    for r in results {
        if col == 0 {
            out += "    ";
        }
        let sr = debug_fanout_result(r);
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
}

pub type FanoutCallResult = RPCNetworkResult<FanoutCallOutput>;
pub type FanoutNodeInfoFilter = Arc<dyn (Fn(&[TypedKey], &NodeInfo) -> bool) + Send + Sync>;
pub type FanoutCheckDone<R> = Arc<dyn (Fn(&[NodeRef]) -> Option<R>) + Send + Sync>;
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
///
/// The algorithm returns early if 'check_done' returns some value, or if an error is found during the process.
/// If the algorithm times out, a Timeout result is returned, however operations will still have been performed and a
/// timeout is not necessarily indicative of an algorithmic 'failure', just that no definitive stopping condition was found
/// in the given time
pub(crate) struct FanoutCall<'a, R>
where
    R: Unpin,
{
    routing_table: &'a RoutingTable,
    node_id: TypedKey,
    context: Mutex<FanoutContext<R>>,
    node_count: usize,
    fanout: usize,
    timeout_us: TimestampDuration,
    node_info_filter: FanoutNodeInfoFilter,
    call_routine: FanoutCallRoutine,
    check_done: FanoutCheckDone<R>,
}

impl<'a, R> FanoutCall<'a, R>
where
    R: Unpin,
{
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        routing_table: &'a RoutingTable,
        node_id: TypedKey,
        node_count: usize,
        fanout: usize,
        timeout_us: TimestampDuration,
        node_info_filter: FanoutNodeInfoFilter,
        call_routine: FanoutCallRoutine,
        check_done: FanoutCheckDone<R>,
    ) -> Self {
        let context = Mutex::new(FanoutContext {
            fanout_queue: FanoutQueue::new(node_id.kind),
            result: None,
        });

        Self {
            routing_table,
            node_id,
            context,
            node_count,
            fanout,
            timeout_us,
            node_info_filter,
            call_routine,
            check_done,
        }
    }

    #[instrument(level = "trace", target = "fanout", skip_all)]
    fn evaluate_done(&self, ctx: &mut FanoutContext<R>) -> bool {
        // If we have a result, then we're done
        if ctx.result.is_some() {
            return true;
        }

        // Check for a new done result
        ctx.result = (self.check_done)(ctx.fanout_queue.nodes()).map(|o| Ok(o));
        ctx.result.is_some()
    }

    #[instrument(level = "trace", target = "fanout", skip_all)]
    fn add_to_fanout_queue(&self, new_nodes: &[NodeRef]) {
        event!(target: "fanout", Level::DEBUG,
            "FanoutCall::add_to_fanout_queue:\n  new_nodes={{\n{}}}\n",
            new_nodes
                .iter()
                .map(|x| format!("  {}", x))
                .collect::<Vec<String>>()
                .join(",\n"),
        );

        let ctx = &mut *self.context.lock();
        ctx.fanout_queue.add(new_nodes, |current_nodes| {
            let mut current_nodes_vec = self
                .routing_table
                .sort_and_clean_closest_noderefs(self.node_id, current_nodes);
            current_nodes_vec.truncate(self.node_count);
            current_nodes_vec
        });
    }

    #[instrument(level = "trace", target = "fanout", skip_all)]
    async fn fanout_processor(&self) -> bool {
        // Loop until we have a result or are done
        loop {
            // Get the closest node we haven't processed yet if we're not done yet
            let next_node = {
                let mut ctx = self.context.lock();
                if self.evaluate_done(&mut ctx) {
                    break true;
                }
                ctx.fanout_queue.next()
            };

            // If we don't have a node to process, stop fanning out
            let Some(next_node) = next_node else {
                break false;
            };

            // Do the call for this node
            match (self.call_routine)(next_node.clone()).await {
                Ok(NetworkResult::Value(v)) => {
                    // Filter returned nodes
                    let filtered_v: Vec<Arc<PeerInfo>> = v
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
                    self.add_to_fanout_queue(&new_nodes);
                }
                #[allow(unused_variables)]
                Ok(x) => {
                    // Call failed, node will not be considered again
                    event!(target: "fanout", Level::DEBUG,
                        "Fanout result {}: {:?}", &next_node, x);
                }
                Err(e) => {
                    // Error happened, abort everything and return the error
                    self.context.lock().result = Some(Err(e));
                    break true;
                }
            };
        }
    }

    #[instrument(level = "trace", target = "fanout", skip_all)]
    fn init_closest_nodes(&self) -> Result<(), RPCError> {
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
                .find_preferred_closest_nodes(self.node_count, self.node_id, filters, transform)
                .map_err(RPCError::invalid_format)?
        };
        self.add_to_fanout_queue(&closest_nodes);
        Ok(())
    }

    #[instrument(level = "trace", target = "fanout", skip_all)]
    pub async fn run(
        &self,
        init_fanout_queue: Vec<NodeRef>,
    ) -> TimeoutOr<Result<Option<R>, RPCError>> {
        // Get timeout in milliseconds
        let timeout_ms = match us_to_ms(self.timeout_us.as_u64()).map_err(RPCError::internal) {
            Ok(v) => v,
            Err(e) => {
                return TimeoutOr::value(Err(e));
            }
        };

        // Initialize closest nodes list
        if let Err(e) = self.init_closest_nodes() {
            return TimeoutOr::value(Err(e));
        }

        // Ensure we include the most recent nodes
        self.add_to_fanout_queue(&init_fanout_queue);

        // Do a quick check to see if we're already done
        {
            let mut ctx = self.context.lock();
            if self.evaluate_done(&mut ctx) {
                return TimeoutOr::value(ctx.result.take().transpose());
            }
        }

        // If not, do the fanout
        let mut unord = FuturesUnordered::new();
        {
            // Spin up 'fanout' tasks to process the fanout
            for _ in 0..self.fanout {
                let h = self.fanout_processor();
                unord.push(h);
            }
        }
        // Wait for them to complete
        timeout(
            timeout_ms,
            async {
                while let Some(is_done) = unord.next().in_current_span().await {
                    if is_done {
                        break;
                    }
                }
            }
            .in_current_span(),
        )
        .await
        .into_timeout_or()
        .map(|_| {
            // Finished, return whatever value we came up with
            self.context.lock().result.take().transpose()
        })
    }
}
