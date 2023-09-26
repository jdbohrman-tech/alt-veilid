use super::*;

struct FanoutContext<R>
where
    R: Unpin,
{
    fanout_queue: FanoutQueue,
    result: Option<Result<R, RPCError>>,
}

pub type FanoutCallReturnType = Result<Option<Vec<PeerInfo>>, RPCError>;
pub type FanoutNodeInfoFilter = Arc<dyn Fn(&[TypedKey], &NodeInfo) -> bool + Send + Sync>;

pub fn empty_fanout_node_info_filter() -> FanoutNodeInfoFilter {
    Arc::new(|_, _| true)
}

pub fn capability_fanout_node_info_filter(caps: Vec<Capability>) -> FanoutNodeInfoFilter {
    Arc::new(move |_, ni| ni.has_capabilities(&caps))
}

/// Contains the logic for generically searching the Veilid routing table for a set of nodes and applying an
/// RPC operation that eventually converges on satisfactory result, or times out and returns some
/// unsatisfactory but acceptable result. Or something.
///
/// The algorithm starts by creating a 'closest_nodes' working set of the nodes closest to some node id currently in our routing table
/// If has pluggable callbacks:
///  * 'check_done' - for checking for a termination condition
///  * 'call_routine' - routine to call for each node that performs an operation and may add more nodes to our closest_nodes set
/// The algorithm is parameterized by:
///  * 'node_count' - the number of nodes to keep in the closest_nodes set
///  * 'fanout' - the number of concurrent calls being processed at the same time
/// The algorithm returns early if 'check_done' returns some value, or if an error is found during the process.
/// If the algorithm times out, a Timeout result is returned, however operations will still have been performed and a
/// timeout is not necessarily indicative of an algorithmic 'failure', just that no definitive stopping condition was found
/// in the given time
pub struct FanoutCall<R, F, C, D>
where
    R: Unpin,
    F: Future<Output = FanoutCallReturnType>,
    C: Fn(NodeRef) -> F,
    D: Fn(&[NodeRef]) -> Option<R>,
{
    routing_table: RoutingTable,
    crypto_kind: CryptoKind,
    node_id: TypedKey,
    context: Mutex<FanoutContext<R>>,
    node_count: usize,
    fanout: usize,
    timeout_us: TimestampDuration,
    node_info_filter: FanoutNodeInfoFilter,
    call_routine: C,
    check_done: D,
}

impl<R, F, C, D> FanoutCall<R, F, C, D>
where
    R: Unpin,
    F: Future<Output = FanoutCallReturnType>,
    C: Fn(NodeRef) -> F,
    D: Fn(&[NodeRef]) -> Option<R>,
{
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        routing_table: RoutingTable,
        node_id: TypedKey,
        node_count: usize,
        fanout: usize,
        timeout_us: TimestampDuration,
        node_info_filter: FanoutNodeInfoFilter,
        call_routine: C,
        check_done: D,
    ) -> Arc<Self> {
        let context = Mutex::new(FanoutContext {
            fanout_queue: FanoutQueue::new(node_id.kind),
            result: None,
        });

        Arc::new(Self {
            routing_table,
            node_id,
            crypto_kind: node_id.kind,
            context,
            node_count,
            fanout,
            timeout_us,
            node_info_filter,
            call_routine,
            check_done,
        })
    }

    fn evaluate_done(self: Arc<Self>, ctx: &mut FanoutContext<R>) -> bool {
        // If we have a result, then we're done
        if ctx.result.is_some() {
            return true;
        }

        // Check for a new done result
        ctx.result = (self.check_done)(ctx.fanout_queue.nodes()).map(|o| Ok(o));
        ctx.result.is_some()
    }

    fn add_to_fanout_queue(self: Arc<Self>, new_nodes: &[NodeRef]) {
        let ctx = &mut *self.context.lock();
        let this = self.clone();
        ctx.fanout_queue.add(new_nodes, |current_nodes| {
            let mut current_nodes_vec = this
                .routing_table
                .sort_and_clean_closest_noderefs(this.node_id, current_nodes);
            current_nodes_vec.truncate(self.node_count);
            current_nodes_vec
        });
    }

    async fn fanout_processor(self: Arc<Self>) {
        // Loop until we have a result or are done
        loop {
            // Get the closest node we haven't processed yet if we're not done yet
            let next_node = {
                let mut ctx = self.context.lock();
                if self.clone().evaluate_done(&mut ctx) {
                    break;
                }
                ctx.fanout_queue.next()
            };

            // If we don't have a node to process, stop fanning out
            let Some(next_node) = next_node else {
                break;
            };

            // Do the call for this node
            match (self.call_routine)(next_node.clone()).await {
                Ok(Some(v)) => {
                    // Filter returned nodes
                    let filtered_v: Vec<PeerInfo> = v
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
                        .register_find_node_answer(self.crypto_kind, filtered_v);
                    self.clone().add_to_fanout_queue(&new_nodes);
                }
                Ok(None) => {
                    // Call failed, node will not be considered again
                }
                Err(e) => {
                    // Error happened, abort everything and return the error
                    self.context.lock().result = Some(Err(e));
                    return;
                }
            };
        }
    }

    fn init_closest_nodes(self: Arc<Self>) -> Result<(), RPCError> {
        // Get the 'node_count' closest nodes to the key out of our routing table
        let closest_nodes = {
            let routing_table = self.routing_table.clone();
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
                NodeRef::new(routing_table.clone(), v.unwrap().clone(), None)
            };

            routing_table
                .find_preferred_closest_nodes(self.node_count, self.node_id, filters, transform)
                .map_err(RPCError::invalid_format)?
        };
        self.clone().add_to_fanout_queue(&closest_nodes);
        Ok(())
    }

    pub async fn run(self: Arc<Self>) -> TimeoutOr<Result<Option<R>, RPCError>> {
        // Get timeout in milliseconds
        let timeout_ms = match us_to_ms(self.timeout_us.as_u64()).map_err(RPCError::internal) {
            Ok(v) => v,
            Err(e) => {
                return TimeoutOr::value(Err(e));
            }
        };

        // Initialize closest nodes list
        if let Err(e) = self.clone().init_closest_nodes() {
            return TimeoutOr::value(Err(e));
        }

        // Do a quick check to see if we're already done
        {
            let mut ctx = self.context.lock();
            if self.clone().evaluate_done(&mut ctx) {
                return TimeoutOr::value(ctx.result.take().transpose());
            }
        }

        // If not, do the fanout
        let mut unord = FuturesUnordered::new();
        {
            // Spin up 'fanout' tasks to process the fanout
            for _ in 0..self.fanout {
                let h = self.clone().fanout_processor();
                unord.push(h);
            }
        }
        // Wait for them to complete
        timeout(timeout_ms, async { while unord.next().await.is_some() {} })
            .await
            .into_timeout_or()
            .map(|_| {
                // Finished, return whatever value we came up with
                self.context.lock().result.take().transpose()
            })
    }
}
