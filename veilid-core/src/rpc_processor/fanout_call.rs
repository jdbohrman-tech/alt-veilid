use super::*;

struct FanoutContext<R>
where
    R: Unpin,
{
    closest_nodes: Vec<NodeRef>,
    called_nodes: TypedKeySet,
    result: Option<Result<R, RPCError>>,
}

pub type FanoutCallReturnType = Result<Option<Vec<PeerInfo>>, RPCError>;

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
    count: usize,
    fanout: usize,
    timeout_us: TimestampDuration,
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
        count: usize,
        fanout: usize,
        timeout_us: TimestampDuration,
        call_routine: C,
        check_done: D,
    ) -> Arc<Self> {
        let context = Mutex::new(FanoutContext {
            closest_nodes: Vec::with_capacity(count),
            called_nodes: TypedKeySet::new(),
            result: None,
        });

        Arc::new(Self {
            routing_table,
            node_id,
            crypto_kind: node_id.kind,
            context,
            count,
            fanout,
            timeout_us,
            call_routine,
            check_done,
        })
    }

    fn add_new_nodes(self: Arc<Self>, new_nodes: Vec<NodeRef>) {
        let mut ctx = self.context.lock();

        for nn in new_nodes {
            let mut dup = false;
            for cn in &ctx.closest_nodes {
                if cn.same_entry(&nn) {
                    dup = true;
                }
            }
            if !dup {
                ctx.closest_nodes.push(nn.clone());
            }
        }

        self.routing_table
            .sort_and_clean_closest_noderefs(self.node_id, &mut ctx.closest_nodes);
        ctx.closest_nodes.truncate(self.count);
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
        for cn in &ctx.closest_nodes {
            if let Some(key) = cn.node_ids().get(self.crypto_kind) {
                if !ctx.called_nodes.contains(&key) {
                    // New fanout call candidate found
                    next_node = Some(cn.clone());
                    ctx.called_nodes.add(key);
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
                    // Call succeeded
                    // Register the returned nodes and add them to the closest nodes list in sorted order
                    let new_nodes = self
                        .routing_table
                        .register_find_node_answer(self.crypto_kind, v);
                    self.clone().add_new_nodes(new_nodes);
                }
                Ok(None) => {
                    // Call failed, remove the node so it isn't included in the output
                    self.clone().remove_node(next_node);
                }
                Err(e) => {
                    // Error happened, abort everything and return the error
                }
            };
        }
    }

    fn init_closest_nodes(self: Arc<Self>) {
        // Get the 'count' closest nodes to the key out of our routing table
        let closest_nodes = {
            let routing_table = self.routing_table.clone();

            let filter = Box::new(
                move |rti: &RoutingTableInner, opt_entry: Option<Arc<BucketEntry>>| {
                    // Exclude our own node
                    if opt_entry.is_none() {
                        return false;
                    }

                    // Ensure only things that are valid/signed in the PublicInternet domain are returned
                    rti.filter_has_valid_signed_node_info(
                        RoutingDomain::PublicInternet,
                        true,
                        opt_entry,
                    )
                },
            ) as RoutingTableEntryFilter;
            let filters = VecDeque::from([filter]);

            let transform = |_rti: &RoutingTableInner, v: Option<Arc<BucketEntry>>| {
                NodeRef::new(routing_table.clone(), v.unwrap().clone(), None)
            };

            routing_table.find_closest_nodes(self.count, self.node_id, filters, transform)
        };

        let mut ctx = self.context.lock();
        ctx.closest_nodes = closest_nodes;
    }

    pub async fn run(self: Arc<Self>) -> TimeoutOr<Result<Option<R>, RPCError>> {
        // Initialize closest nodes list
        self.clone().init_closest_nodes();

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
        timeout((self.timeout_us.as_u64() / 1000u64) as u32, async {
            while let Some(_) = unord.next().await {}
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
