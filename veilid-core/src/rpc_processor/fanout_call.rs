use super::*;

struct FanoutContext<R>
where
    R: Unpin,
{
    closest_nodes: Vec<NodeRef>,
    called_nodes: HashSet<TypedKey>,
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

/// Contains the logic for generically searing the Veilid routing table for a set of nodes and applying an
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
            closest_nodes: Vec::with_capacity(node_count),
            called_nodes: HashSet::new(),
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

    fn add_new_nodes(self: Arc<Self>, new_nodes: Vec<NodeRef>) {
        let mut ctx = self.context.lock();

        for nn in new_nodes {
            // Make sure the new node isnt already in the list
            let mut dup = false;
            for cn in &ctx.closest_nodes {
                if cn.same_entry(&nn) {
                    dup = true;
                    break;
                }
            }
            if !dup {
                // Add the new node if we haven't already called it before (only one call per node ever)
                if let Some(key) = nn.node_ids().get(self.crypto_kind) {
                    if !ctx.called_nodes.contains(&key) {
                        ctx.closest_nodes.push(nn.clone());
                    }
                }
            }
        }

        self.routing_table
            .sort_and_clean_closest_noderefs(self.node_id, &mut ctx.closest_nodes);
        ctx.closest_nodes.truncate(self.node_count);
    }

    fn remove_node(self: Arc<Self>, dead_node: NodeRef) {
        let mut ctx = self.context.lock();
        for n in 0..ctx.closest_nodes.len() {
            let cn = &ctx.closest_nodes[n];
            if cn.same_entry(&dead_node) {
                ctx.closest_nodes.remove(n);
                break;
            }
        }
    }

    fn get_next_node(self: Arc<Self>) -> Option<NodeRef> {
        let mut next_node = None;
        let mut ctx = self.context.lock();
        for cn in ctx.closest_nodes.clone() {
            if let Some(key) = cn.node_ids().get(self.crypto_kind) {
                if !ctx.called_nodes.contains(&key) {
                    // New fanout call candidate found
                    next_node = Some(cn.clone());
                    ctx.called_nodes.insert(key);
                    break;
                }
            }
        }
        next_node
    }

    fn evaluate_done(self: Arc<Self>) -> bool {
        let mut ctx = self.context.lock();

        // If we have a result, then we're done
        if ctx.result.is_some() {
            return true;
        }

        // Check for a new done result
        ctx.result = (self.check_done)(&ctx.closest_nodes).map(|o| Ok(o));
        ctx.result.is_some()
    }

    async fn fanout_processor(self: Arc<Self>) {
        // Check to see if we have a result or are done
        while !self.clone().evaluate_done() {
            // Get the closest node we haven't processed yet
            let next_node = self.clone().get_next_node();

            // If we don't have a node to process, stop fanning out
            let Some(next_node) = next_node else {
                return;
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
                    // Register the returned nodes and add them to the closest nodes list in sorted order
                    let new_nodes = self
                        .routing_table
                        .register_find_node_answer(self.crypto_kind, filtered_v);
                    self.clone().add_new_nodes(new_nodes);
                }
                Ok(None) => {
                    // Call failed, remove the node so it isn't considered as part of the fanout
                    self.clone().remove_node(next_node);
                }
                Err(e) => {
                    // Error happened, abort everything and return the error
                    let mut ctx = self.context.lock();
                    ctx.result = Some(Err(e));
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
                    entry.with(rti, |_rti, e|  {
                        let Some(signed_node_info) = e.signed_node_info(RoutingDomain::PublicInternet) else {
                            return false;
                        };
                        // Ensure only things that are valid/signed in the PublicInternet domain are returned
                        if !signed_node_info.has_any_signature() {
                            return false;
                        }

                        // Check our node info ilter
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
                .find_closest_nodes(self.node_count, self.node_id, filters, transform)
                .map_err(RPCError::invalid_format)?
        };

        let mut ctx = self.context.lock();
        ctx.closest_nodes = closest_nodes;
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
        if self.clone().evaluate_done() {
            let mut ctx = self.context.lock();
            return TimeoutOr::value(ctx.result.take().transpose());
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
        timeout(timeout_ms, async {
            while let Some(_) = unord.next().await {
                if self.clone().evaluate_done() {
                    break;
                }
            }
        })
        .await
        .into_timeout_or()
        .map(|_| {
            // Finished, return whatever value we came up with
            let mut ctx = self.context.lock();
            ctx.result.take().transpose()
        })
    }
}
