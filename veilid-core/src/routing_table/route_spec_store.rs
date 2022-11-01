use super::*;
use crate::veilid_api::*;
use serde::*;

/// Compiled route (safety route + private route)
#[derive(Clone, Debug)]
pub struct CompiledRoute {
    /// The safety route attached to the private route
    pub safety_route: SafetyRoute,
    /// The secret used to encrypt the message payload
    pub secret: DHTKeySecret,
    /// The node ref to the first hop in the compiled route
    pub first_hop: NodeRef,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct RouteSpecDetail {
    /// Secret key
    #[serde(skip)]
    pub secret_key: DHTKeySecret,
    /// Route hops
    pub hops: Vec<DHTKey>,
    /// Route noderefs
    #[serde(skip)]
    hop_node_refs: Vec<NodeRef>,
    /// Transfers up and down
    transfer_stats_down_up: TransferStatsDownUp,
    /// Latency stats
    latency_stats: LatencyStats,
    /// Accounting mechanism for this route's RPC latency
    #[serde(skip)]
    latency_stats_accounting: LatencyStatsAccounting,
    /// Accounting mechanism for the bandwidth across this route
    #[serde(skip)]
    transfer_stats_accounting: TransferStatsAccounting,
    /// Published private route, do not reuse for ephemeral routes
    /// Not serialized because all routes should be re-published when restarting
    #[serde(skip)]
    published: bool,
    // Can optimize the rendering of this route, using node ids only instead of full peer info
    #[serde(skip)]
    reachable: bool,
    /// Timestamp of when the route was created
    created_ts: u64,
    /// Timestamp of when the route was last checked for validity
    last_checked_ts: Option<u64>,
    /// Timestamp of when the route was last used for anything
    last_used_ts: Option<u64>,
    /// Directions this route is guaranteed to work in
    directions: DirectionSet,
    /// Stability preference (prefer reliable nodes over faster)
    pub stability: Stability,
    /// Sequencing preference (connection oriented protocols vs datagram)
    pub sequencing: Sequencing,
}

/// The core representation of the RouteSpecStore that can be serialized
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RouteSpecStoreContent {
    /// All of the routes we have allocated so far
    details: HashMap<DHTKey, RouteSpecDetail>,
}

/// Ephemeral data used to help the RouteSpecStore operate efficiently
#[derive(Debug, Default)]
pub struct RouteSpecStoreCache {
    /// How many times nodes have been used
    used_nodes: HashMap<DHTKey, usize>,
    /// How many times nodes have been used at the terminal point of a route
    used_end_nodes: HashMap<DHTKey, usize>,
    /// Route spec hop cache, used to quickly disqualify routes
    hop_cache: HashSet<Vec<u8>>,
}

#[derive(Debug)]
pub struct RouteSpecStoreInner {
    /// Serialize RouteSpecStore content
    content: RouteSpecStoreContent,
    /// RouteSpecStore cache
    cache: RouteSpecStoreCache,
}

pub struct RouteSpecStoreUnlockedInner {
    /// Handle to routing table
    routing_table: RoutingTable,
    /// Maximum number of hops in a route
    max_route_hop_count: usize,
    /// Default number of hops in a route
    default_route_hop_count: usize,
}

impl fmt::Debug for RouteSpecStoreUnlockedInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RouteSpecStoreUnlockedInner")
            .field("max_route_hop_count", &self.max_route_hop_count)
            .field("default_route_hop_count", &self.default_route_hop_count)
            .finish()
    }
}

/// The routing table's storage for private/safety routes
#[derive(Clone, Debug)]
pub struct RouteSpecStore {
    inner: Arc<Mutex<RouteSpecStoreInner>>,
    unlocked_inner: Arc<RouteSpecStoreUnlockedInner>,
}

fn route_hops_to_hop_cache(hops: &[DHTKey]) -> Vec<u8> {
    let mut cache: Vec<u8> = Vec::with_capacity(hops.len() * DHT_KEY_LENGTH);
    for hop in hops {
        cache.extend_from_slice(&hop.bytes);
    }
    cache
}

/// get the hop cache key for a particular route permutation
fn route_permutation_to_hop_cache(nodes: &[(DHTKey, NodeInfo)], perm: &[usize]) -> Vec<u8> {
    let mut cache: Vec<u8> = Vec::with_capacity(perm.len() * DHT_KEY_LENGTH);
    for n in perm {
        cache.extend_from_slice(&nodes[*n].0.bytes)
    }
    cache
}

/// number of route permutations is the number of unique orderings
/// for a set of nodes, given that the first node is fixed
fn get_route_permutation_count(hop_count: usize) -> usize {
    if hop_count == 0 {
        unreachable!();
    }
    // a single node or two nodes is always fixed
    if hop_count == 1 || hop_count == 2 {
        return 1;
    }
    // more than two nodes has factorial permutation
    // hop_count = 3 -> 2! -> 2
    // hop_count = 4 -> 3! -> 6
    (3..hop_count).into_iter().fold(2usize, |acc, x| acc * x)
}

/// get the route permutation at particular 'perm' index, starting at the 'start' index
/// for a set of 'hop_count' nodes. the first node is always fixed, and the maximum
/// number of permutations is given by get_route_permutation_count()
fn with_route_permutations<F>(hop_count: usize, start: usize, mut f: F) -> bool
where
    F: FnMut(&[usize]) -> bool,
{
    if hop_count == 0 {
        unreachable!();
    }
    // initial permutation
    let mut permutation: Vec<usize> = Vec::with_capacity(hop_count);
    for n in 0..hop_count {
        permutation[n] = start + n;
    }
    // if we have one hop or two, then there's only one permutation
    if hop_count == 1 || hop_count == 2 {
        return f(&permutation);
    }

    // heaps algorithm, but skipping the first element
    fn heaps_permutation<F>(permutation: &mut [usize], size: usize, mut f: F) -> bool
    where
        F: FnMut(&[usize]) -> bool,
    {
        if size == 1 {
            if f(&permutation) {
                return true;
            }
            return false;
        }

        for i in 0..size {
            if heaps_permutation(permutation, size - 1, &mut f) {
                return true;
            }
            if size % 2 == 1 {
                permutation.swap(1, size);
            } else {
                permutation.swap(1 + i, size);
            }
        }
        false
    }

    // recurse
    heaps_permutation(&mut permutation, hop_count - 1, f)
}

impl RouteSpecStore {
    pub fn new(routing_table: RoutingTable) -> Self {
        let config = routing_table.network_manager().config();
        let c = config.get();

        Self {
            unlocked_inner: Arc::new(RouteSpecStoreUnlockedInner {
                max_route_hop_count: c.network.rpc.max_route_hop_count.into(),
                default_route_hop_count: c.network.rpc.default_route_hop_count.into(),
                routing_table,
            }),
            inner: Arc::new(Mutex::new(RouteSpecStoreInner {
                content: RouteSpecStoreContent {
                    details: HashMap::new(),
                },
                cache: Default::default(),
            })),
        }
    }

    pub async fn load(routing_table: RoutingTable) -> EyreResult<RouteSpecStore> {
        let config = routing_table.network_manager().config();
        let c = config.get();
        // Get cbor blob from table store
        let table_store = routing_table.network_manager().table_store();
        let rsstdb = table_store.open("RouteSpecStore", 1).await?;
        let mut content: RouteSpecStoreContent =
            rsstdb.load_cbor(0, b"content").await?.unwrap_or_default();

        // Load secrets from pstore
        let pstore = routing_table.network_manager().protected_store();
        let mut dead_keys = Vec::new();
        for (k, v) in &mut content.details {
            if let Some(secret_key) = pstore
                .load_user_secret(&format!("RouteSpecStore_{}", k.encode()))
                .await?
            {
                match secret_key.try_into() {
                    Ok(s) => {
                        v.secret_key = DHTKeySecret::new(s);
                    }
                    Err(_) => {
                        dead_keys.push(*k);
                    }
                }
            } else {
                dead_keys.push(*k);
            }
        }
        for k in dead_keys {
            log_rtab!(debug "killing off private route: {}", k.encode());
            content.details.remove(&k);
        }

        let mut inner = RouteSpecStoreInner {
            content,
            cache: Default::default(),
        };

        // Rebuild the routespecstore cache
        Self::rebuild_cache(&mut inner);

        let rss = RouteSpecStore {
            unlocked_inner: Arc::new(RouteSpecStoreUnlockedInner {
                max_route_hop_count: c.network.rpc.max_route_hop_count.into(),
                default_route_hop_count: c.network.rpc.default_route_hop_count.into(),
                routing_table,
            }),
            inner: Arc::new(Mutex::new(inner)),
        };

        Ok(rss)
    }

    pub async fn save(&self) -> EyreResult<()> {
        let content = {
            let inner = self.inner.lock();
            inner.content.clone()
        };

        // Save all the fields we care about to the cbor blob in table storage
        let table_store = self
            .unlocked_inner
            .routing_table
            .network_manager()
            .table_store();
        let rsstdb = table_store.open("RouteSpecStore", 1).await?;
        rsstdb.store_cbor(0, b"content", &content).await?;

        // Keep secrets in protected store as well
        let pstore = self
            .unlocked_inner
            .routing_table
            .network_manager()
            .protected_store();
        for (k, v) in &content.details {
            if pstore
                .save_user_secret(
                    &format!("RouteSpecStore_{}", k.encode()),
                    &v.secret_key.bytes,
                )
                .await?
            {
                panic!("route spec should not already have secret key saved");
            }
        }

        Ok(())
    }

    fn add_to_cache(cache: &mut RouteSpecStoreCache, cache_key: Vec<u8>, rsd: &RouteSpecDetail) {
        if !cache.hop_cache.insert(cache_key) {
            panic!("route should never be inserted twice");
        }
        for h in &rsd.hops {
            cache
                .used_nodes
                .entry(*h)
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
        cache
            .used_end_nodes
            .entry(*rsd.hops.last().unwrap())
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }

    fn rebuild_cache(inner: &mut RouteSpecStoreInner) {
        for v in inner.content.details.values() {
            let cache_key = route_hops_to_hop_cache(&v.hops);
            Self::add_to_cache(&mut inner.cache, cache_key, &v);
        }
    }

    fn detail<'a>(
        inner: &'a RouteSpecStoreInner,
        public_key: &DHTKey,
    ) -> Option<&'a RouteSpecDetail> {
        inner.content.details.get(public_key)
    }
    fn detail_mut<'a>(
        inner: &'a mut RouteSpecStoreInner,
        public_key: &DHTKey,
    ) -> Option<&'a mut RouteSpecDetail> {
        inner.content.details.get_mut(public_key)
    }

    /// Create a new route
    /// Prefers nodes that are not currently in use by another route
    /// The route is not yet tested for its reachability
    /// Returns None if no route could be allocated at this time
    pub fn allocate_route(
        &self,
        stability: Stability,
        sequencing: Sequencing,
        hop_count: usize,
        directions: DirectionSet,
    ) -> EyreResult<Option<DHTKey>> {
        let inner = &mut *self.inner.lock();
        let routing_table = self.unlocked_inner.routing_table.clone();
        let rti = &mut *routing_table.inner.write();

        self.allocate_route_inner(inner, rti, stability, sequencing, hop_count, directions)
    }

    fn allocate_route_inner(
        &self,
        inner: &mut RouteSpecStoreInner,
        rti: &RoutingTableInner,
        stability: Stability,
        sequencing: Sequencing,
        hop_count: usize,
        directions: DirectionSet,
    ) -> EyreResult<Option<DHTKey>> {
        use core::cmp::Ordering;

        if hop_count < 1 {
            bail!("Not allocating route less than one hop in length");
        }

        if hop_count > self.unlocked_inner.max_route_hop_count {
            bail!("Not allocating route longer than max route hop count");
        }

        // Get list of all nodes, and sort them for selection
        let cur_ts = intf::get_timestamp();
        let filter = |rti, _k: DHTKey, v: Option<Arc<BucketEntry>>| -> bool {
            // Exclude our own node from routes
            if v.is_none() {
                return false;
            }
            let v = v.unwrap();

            // Exclude nodes on our local network
            let on_local_network = v.with(rti, |_rti, e| {
                e.node_info(RoutingDomain::LocalNetwork).is_some()
            });
            if on_local_network {
                return false;
            }

            // Exclude nodes with no publicinternet nodeinfo, or incompatible nodeinfo or node status won't route
            v.with(rti, |_rti, e| {
                let node_info_ok = if let Some(ni) = e.node_info(RoutingDomain::PublicInternet) {
                    ni.has_sequencing_matched_dial_info(sequencing)
                } else {
                    false
                };
                let node_status_ok = if let Some(ns) = e.node_status(RoutingDomain::PublicInternet)
                {
                    ns.will_route()
                } else {
                    false
                };

                node_info_ok && node_status_ok
            })
        };
        let compare = |rti,
                       v1: &(DHTKey, Option<Arc<BucketEntry>>),
                       v2: &(DHTKey, Option<Arc<BucketEntry>>)|
         -> Ordering {
            // deprioritize nodes that we have already used as end points
            let e1_used_end = inner
                .cache
                .used_end_nodes
                .get(&v1.0)
                .cloned()
                .unwrap_or_default();
            let e2_used_end = inner
                .cache
                .used_end_nodes
                .get(&v2.0)
                .cloned()
                .unwrap_or_default();
            let cmp_used_end = e1_used_end.cmp(&e2_used_end);
            if !matches!(cmp_used_end, Ordering::Equal) {
                return cmp_used_end;
            }

            // deprioritize nodes we have used already anywhere
            let e1_used = inner
                .cache
                .used_nodes
                .get(&v1.0)
                .cloned()
                .unwrap_or_default();
            let e2_used = inner
                .cache
                .used_nodes
                .get(&v2.0)
                .cloned()
                .unwrap_or_default();
            let cmp_used = e1_used.cmp(&e2_used);
            if !matches!(cmp_used, Ordering::Equal) {
                return cmp_used;
            }

            // always prioritize reliable nodes, but sort by oldest or fastest
            let cmpout = v1.1.as_ref().unwrap().with(rti, |rti, e1| {
                v2.1.as_ref()
                    .unwrap()
                    .with(rti, |_rti, e2| match stability {
                        Stability::LowLatency => {
                            BucketEntryInner::cmp_fastest_reliable(cur_ts, e1, e2)
                        }
                        Stability::Reliable => {
                            BucketEntryInner::cmp_oldest_reliable(cur_ts, e1, e2)
                        }
                    })
            });
            cmpout
        };
        let transform = |rti, k: DHTKey, v: Option<Arc<BucketEntry>>| -> (DHTKey, NodeInfo) {
            // Return the key and the nodeinfo for that key
            (
                k,
                v.unwrap().with(rti, |_rti, e| {
                    e.node_info(RoutingDomain::PublicInternet.into())
                        .unwrap()
                        .clone()
                }),
            )
        };

        // Pull the whole routing table in sorted order
        let node_count = rti.get_entry_count(
            RoutingDomain::PublicInternet.into(),
            BucketEntryState::Unreliable,
        );
        let nodes =
            rti.find_peers_with_sort_and_filter(node_count, cur_ts, filter, compare, transform);

        // If we couldn't find enough nodes, wait until we have more nodes in the routing table
        if nodes.len() < hop_count {
            log_rtab!(debug "not enough nodes to construct route at this time");
            return Ok(None);
        }

        // Now go through nodes and try to build a route we haven't seen yet
        let mut route_nodes: Vec<usize> = Vec::new();
        let mut cache_key: Vec<u8> = Vec::new();
        for start in 0..(nodes.len() - hop_count) {
            // Try the permutations available starting with 'start'
            let done = with_route_permutations(hop_count, start, |permutation: &[usize]| {
                // Get the route cache key
                cache_key = route_permutation_to_hop_cache(&nodes, permutation);

                // Skip routes we have already seen
                if inner.cache.hop_cache.contains(&cache_key) {
                    return false;
                }

                // Ensure this route is viable by checking that each node can contact the next one
                if directions.contains(Direction::Outbound) {
                    let our_node_info = rti.get_own_node_info(RoutingDomain::PublicInternet);
                    let our_node_id = rti.node_id();
                    let mut previous_node = &(our_node_id, our_node_info);
                    let mut reachable = true;
                    for n in permutation {
                        let current_node = nodes.get(*n).unwrap();
                        let cm = rti.get_contact_method(
                            RoutingDomain::PublicInternet,
                            &previous_node.0,
                            &previous_node.1,
                            &current_node.0,
                            &current_node.1,
                            DialInfoFilter::all(),
                            sequencing,
                        );
                        if matches!(cm, ContactMethod::Unreachable) {
                            reachable = false;
                            break;
                        }
                        previous_node = current_node;
                    }
                    if !reachable {
                        return false;
                    }
                }
                if directions.contains(Direction::Inbound) {
                    let our_node_info = rti.get_own_node_info(RoutingDomain::PublicInternet);
                    let our_node_id = rti.node_id();
                    let mut next_node = &(our_node_id, our_node_info);
                    let mut reachable = true;
                    for n in permutation.iter().rev() {
                        let current_node = nodes.get(*n).unwrap();
                        let cm = rti.get_contact_method(
                            RoutingDomain::PublicInternet,
                            &next_node.0,
                            &next_node.1,
                            &current_node.0,
                            &current_node.1,
                            DialInfoFilter::all(),
                            sequencing,
                        );
                        if matches!(cm, ContactMethod::Unreachable) {
                            reachable = false;
                            break;
                        }
                        next_node = current_node;
                    }
                    if !reachable {
                        return false;
                    }
                }
                // Keep this route
                route_nodes = permutation.to_vec();
                true
            });
            if done {
                break;
            }
        }
        if route_nodes.is_empty() {
            log_rtab!(debug "unable to find unique route at this time");
            return Ok(None);
        }

        // Got a unique route, lets build the detail, register it, and return it
        let hops = route_nodes.iter().map(|v| nodes[*v].0).collect();
        let hop_node_refs = route_nodes
            .iter()
            .map(|v| {
                rti.lookup_node_ref(self.unlocked_inner.routing_table.clone(), nodes[*v].0)
                    .unwrap()
            })
            .collect();

        let (public_key, secret_key) = generate_secret();

        let rsd = RouteSpecDetail {
            secret_key,
            hops,
            hop_node_refs,
            transfer_stats_down_up: Default::default(),
            latency_stats: Default::default(),
            latency_stats_accounting: Default::default(),
            transfer_stats_accounting: Default::default(),
            published: false,
            reachable: false,
            created_ts: cur_ts,
            last_checked_ts: None,
            last_used_ts: None,
            directions,
            stability,
            sequencing,
        };

        // Add to cache
        Self::add_to_cache(&mut inner.cache, cache_key, &rsd);

        // Keep route in spec store
        inner.content.details.insert(public_key, rsd);

        Ok(Some(public_key))
    }

    pub fn with_route_spec_detail<F, R>(&self, public_key: &DHTKey, f: F) -> Option<R>
    where
        F: FnOnce(&RouteSpecDetail) -> R,
    {
        let inner = self.inner.lock();
        Self::detail(&*inner, &public_key).map(f)
    }

    pub fn release_route(&self, public_key: DHTKey) {
        let mut inner = self.inner.lock();
        if let Some(detail) = inner.content.details.remove(&public_key) {
            // Remove from hop cache
            let cache_key = route_hops_to_hop_cache(&detail.hops);
            if !inner.cache.hop_cache.remove(&cache_key) {
                panic!("hop cache should have contained cache key");
            }
            // Remove from used nodes cache
            for h in &detail.hops {
                match inner.cache.used_nodes.entry(*h) {
                    std::collections::hash_map::Entry::Occupied(mut o) => {
                        *o.get_mut() -= 1;
                        if *o.get() == 0 {
                            o.remove();
                        }
                    }
                    std::collections::hash_map::Entry::Vacant(_) => {
                        panic!("used_nodes cache should have contained hop");
                    }
                }
            }
            // Remove from end nodes cache
            match inner.cache.used_nodes.entry(*detail.hops.last().unwrap()) {
                std::collections::hash_map::Entry::Occupied(mut o) => {
                    *o.get_mut() -= 1;
                    if *o.get() == 0 {
                        o.remove();
                    }
                }
                std::collections::hash_map::Entry::Vacant(_) => {
                    panic!("used_nodes cache should have contained hop");
                }
            }
        } else {
            panic!("can't release route that was never allocated");
        }
    }

    /// Find first matching unpublished route that fits into the selection criteria
    pub fn first_unpublished_route(
        &self,
        min_hop_count: usize,
        max_hop_count: usize,
        stability: Stability,
        sequencing: Sequencing,
        directions: DirectionSet,
    ) -> Option<DHTKey> {
        let inner = self.inner.lock();

        for detail in &inner.content.details {
            if detail.1.stability >= stability
                && detail.1.sequencing >= sequencing
                && detail.1.hops.len() >= min_hop_count
                && detail.1.hops.len() <= max_hop_count
                && detail.1.directions.is_subset(directions)
                && !detail.1.published
            {
                return Some(*detail.0);
            }
        }
        None
    }

    //////////////////////////////////////////////////////////////////////

    /// Compiles a safety route to the private route, with caching
    /// Returns an Err() if the parameters are wrong
    /// Returns Ok(None) if no allocation could happen at this time (not an error)
    pub fn compile_safety_route(
        &self,
        safety_selection: SafetySelection,
        private_route: PrivateRoute,
    ) -> EyreResult<Option<CompiledRoute>> {
        let inner = &mut *self.inner.lock();
        let routing_table = self.unlocked_inner.routing_table.clone();
        let rti = &mut *routing_table.inner.write();

        let pr_hopcount = private_route.hop_count as usize;
        let max_route_hop_count = self.unlocked_inner.max_route_hop_count;
        if pr_hopcount > max_route_hop_count {
            bail!("private route hop count too long");
        }

        // See if we are using a safety route, if not, short circuit this operation
        let safety_spec = match safety_selection {
            SafetySelection::Unsafe(sequencing) => {
                // Safety route stub with the node's public key as the safety route key since it's the 0th hop
                if private_route.first_hop.is_none() {
                    bail!("can't compile zero length route");
                }
                let first_hop = private_route.first_hop.as_ref().unwrap();
                let opt_first_hop = match &first_hop.node {
                    RouteNode::NodeId(id) => rti.lookup_node_ref(routing_table.clone(), id.key),
                    RouteNode::PeerInfo(pi) => rti.register_node_with_signed_node_info(
                        routing_table.clone(),
                        RoutingDomain::PublicInternet,
                        pi.node_id.key,
                        pi.signed_node_info.clone(),
                        false,
                    ),
                };
                if opt_first_hop.is_none() {
                    // Can't reach this private route any more
                    log_rtab!(debug "can't reach private route any more");
                    return Ok(None);
                }
                let mut first_hop = opt_first_hop.unwrap();

                // Set sequencing requirement
                first_hop.set_sequencing(sequencing);

                // Return the compiled safety route
                return Ok(Some(CompiledRoute {
                    safety_route: SafetyRoute::new_stub(routing_table.node_id(), private_route),
                    secret: routing_table.node_id_secret(),
                    first_hop,
                }));
            }
            SafetySelection::Safe(safety_spec) => safety_spec,
        };

        // See if the preferred route is here
        let opt_safety_rsd: Option<(&mut RouteSpecDetail, DHTKey)> =
            if let Some(preferred_route) = safety_spec.preferred_route {
                Self::detail_mut(inner, &preferred_route).map(|rsd| (rsd, preferred_route))
            } else {
                // Preferred safety route was not requested
                None
            };
        let (safety_rsd, sr_pubkey) = if let Some(safety_rsd) = opt_safety_rsd {
            // Safety route exists
            safety_rsd
        } else {
            // Select a safety route from the pool or make one if we don't have one that matches
            if let Some(sr_pubkey) = self.first_unpublished_route(
                safety_spec.hop_count,
                safety_spec.hop_count,
                safety_spec.stability,
                safety_spec.sequencing,
                Direction::Outbound.into(),
            ) {
                // Found a route to use
                (Self::detail_mut(inner, &sr_pubkey).unwrap(), sr_pubkey)
            } else {
                // No route found, gotta allocate one
                let sr_pubkey = match self
                    .allocate_route_inner(
                        inner,
                        rti,
                        safety_spec.stability,
                        safety_spec.sequencing,
                        safety_spec.hop_count,
                        Direction::Outbound.into(),
                    )
                    .map_err(RPCError::internal)?
                {
                    Some(pk) => pk,
                    None => return Ok(None),
                };
                (Self::detail_mut(inner, &sr_pubkey).unwrap(), sr_pubkey)
            }
        };

        // Ensure the total hop count isn't too long for our config
        let sr_hopcount = safety_spec.hop_count;
        if sr_hopcount == 0 {
            bail!("safety route hop count is zero");
        }
        if sr_hopcount > max_route_hop_count {
            bail!("safety route hop count too long");
        }

        // See if we can optimize this compilation yet
        // We don't want to include full nodeinfo if we don't have to
        let optimize = safety_rsd.reachable;

        // xxx implement caching here!

        // Create hops
        let hops = {
            // start last blob-to-encrypt data off as private route
            let mut blob_data = {
                let mut pr_message = ::capnp::message::Builder::new_default();
                let mut pr_builder = pr_message.init_root::<veilid_capnp::private_route::Builder>();
                encode_private_route(&private_route, &mut pr_builder)?;
                let mut blob_data = builder_to_vec(pr_message)?;

                // append the private route tag so we know how to decode it later
                blob_data.push(1u8);
                blob_data
            };

            // Encode each hop from inside to outside
            // skips the outermost hop since that's entering the
            // safety route and does not include the dialInfo
            // (outer hop is a RouteHopData, not a RouteHop).
            // Each loop mutates 'nonce', and 'blob_data'
            let mut nonce = Crypto::get_random_nonce();
            let crypto = routing_table.network_manager().crypto();
            for h in (1..sr_hopcount).rev() {
                // Get blob to encrypt for next hop
                blob_data = {
                    // Encrypt the previous blob ENC(nonce, DH(PKhop,SKsr))
                    let dh_secret = crypto
                        .cached_dh(&safety_rsd.hops[h], &safety_rsd.secret_key)
                        .wrap_err("dh failed")?;
                    let enc_msg_data =
                        Crypto::encrypt_aead(blob_data.as_slice(), &nonce, &dh_secret, None)
                            .wrap_err("encryption failed")?;

                    // Make route hop data
                    let route_hop_data = RouteHopData {
                        nonce,
                        blob: enc_msg_data,
                    };

                    // Make route hop
                    let route_hop = RouteHop {
                        node: if optimize {
                            // Optimized, no peer info, just the dht key
                            RouteNode::NodeId(NodeId::new(safety_rsd.hops[h]))
                        } else {
                            // Full peer info, required until we are sure the route has been fully established
                            let node_id = safety_rsd.hops[h];
                            let pi = rti
                                .with_node_entry(node_id, |entry| {
                                    entry.with(rti, |_rti, e| {
                                        e.make_peer_info(node_id, RoutingDomain::PublicInternet)
                                    })
                                })
                                .flatten();
                            if pi.is_none() {
                                bail!("peer info should exist for route but doesn't");
                            }
                            RouteNode::PeerInfo(pi.unwrap())
                        },
                        next_hop: Some(route_hop_data),
                    };

                    // Make next blob from route hop
                    let mut rh_message = ::capnp::message::Builder::new_default();
                    let mut rh_builder = rh_message.init_root::<veilid_capnp::route_hop::Builder>();
                    encode_route_hop(&route_hop, &mut rh_builder)?;
                    let mut blob_data = builder_to_vec(rh_message)?;

                    // Append the route hop tag so we know how to decode it later
                    blob_data.push(0u8);
                    blob_data
                };

                // Make another nonce for the next hop
                nonce = Crypto::get_random_nonce();
            }

            // Encode first RouteHopData
            let dh_secret = crypto
                .cached_dh(&safety_rsd.hops[0], &safety_rsd.secret_key)
                .map_err(RPCError::map_internal("dh failed"))?;
            let enc_msg_data = Crypto::encrypt_aead(blob_data.as_slice(), &nonce, &dh_secret, None)
                .map_err(RPCError::map_internal("encryption failed"))?;

            let route_hop_data = RouteHopData {
                nonce,
                blob: enc_msg_data,
            };

            SafetyRouteHops::Data(route_hop_data)
        };

        // Build safety route
        let safety_route = SafetyRoute {
            public_key: sr_pubkey,
            hop_count: safety_spec.hop_count as u8,
            hops,
        };

        let mut first_hop = safety_rsd.hop_node_refs.first().unwrap().clone();

        // Ensure sequencing requirement is set on first hop
        first_hop.set_sequencing(safety_spec.sequencing);

        // Build compiled route
        let compiled_route = CompiledRoute {
            safety_route,
            secret: safety_rsd.secret_key,
            first_hop,
        };

        // xxx: add cache here

        // Return compiled route
        Ok(Some(compiled_route))
    }

    /// Assemble private route for publication
    pub fn assemble_private_route(
        &self,
        rti: &RoutingTableInner,
        routing_table: RoutingTable,
        key: &DHTKey,
    ) -> EyreResult<PrivateRoute> {
        let inner = &*self.inner.lock();

        let rsd = Self::detail(inner, &key).ok_or_else(|| eyre!("route does not exist"))?;

        // See if we can optimize this compilation yet
        // We don't want to include full nodeinfo if we don't have to
        let optimize = rsd.reachable;

        // Make innermost route hop to our own node
        let mut route_hop = RouteHop {
            node: if optimize {
                RouteNode::NodeId(NodeId::new(routing_table.node_id()))
            } else {
                RouteNode::PeerInfo(rti.get_own_peer_info(RoutingDomain::PublicInternet))
            },
            next_hop: None,
        };

        let crypto = routing_table.network_manager().crypto();
        // Loop for each hop
        let hop_count = rsd.hops.len();
        for h in (0..hop_count).rev() {
            let nonce = Crypto::get_random_nonce();

            let blob_data = {
                let mut rh_message = ::capnp::message::Builder::new_default();
                let mut rh_builder = rh_message.init_root::<veilid_capnp::route_hop::Builder>();
                encode_route_hop(&route_hop, &mut rh_builder)?;
                builder_to_vec(rh_message)?
            };

            // Encrypt the previous blob ENC(nonce, DH(PKhop,SKpr))
            let dh_secret = crypto
                .cached_dh(&rsd.hops[h], &rsd.secret_key)
                .wrap_err("dh failed")?;
            let enc_msg_data = Crypto::encrypt_aead(blob_data.as_slice(), &nonce, &dh_secret, None)
                .wrap_err("encryption failed")?;
            let route_hop_data = RouteHopData {
                nonce,
                blob: enc_msg_data,
            };

            route_hop = RouteHop {
                node: if optimize {
                    // Optimized, no peer info, just the dht key
                    RouteNode::NodeId(NodeId::new(rsd.hops[h]))
                } else {
                    // Full peer info, required until we are sure the route has been fully established
                    let node_id = rsd.hops[h];
                    let pi = rti
                        .with_node_entry(node_id, |entry| {
                            entry.with(rti, |_rti, e| {
                                e.make_peer_info(node_id, RoutingDomain::PublicInternet)
                            })
                        })
                        .flatten();
                    if pi.is_none() {
                        bail!("peer info should exist for route but doesn't",);
                    }
                    RouteNode::PeerInfo(pi.unwrap())
                },
                next_hop: Some(route_hop_data),
            }
        }

        let private_route = PrivateRoute {
            public_key: key.clone(),
            hop_count: hop_count.try_into().unwrap(),
            first_hop: Some(route_hop),
        };
        Ok(private_route)
    }

    /// Mark route as published
    /// When first deserialized, routes must be re-published in order to ensure they remain
    /// in the RouteSpecStore.
    pub fn mark_route_published(&mut self, key: &DHTKey) -> EyreResult<()> {
        let inner = &mut *self.inner.lock();
        Self::detail_mut(inner, &key)
            .ok_or_else(|| eyre!("route does not exist"))?
            .published = true;
        Ok(())
    }

    /// Mark route as reachable
    /// When first deserialized, routes must be re-tested for reachability
    /// This can be used to determine if routes need to be sent with full peerinfo or can just use a node id
    pub fn mark_route_reachable(&mut self, key: &DHTKey) -> EyreResult<()> {
        let inner = &mut *self.inner.lock();
        Self::detail_mut(inner, &key)
            .ok_or_else(|| eyre!("route does not exist"))?
            .published = true;
        Ok(())
    }

    /// Mark route as checked
    pub fn touch_route_checked(&mut self, key: &DHTKey, cur_ts: u64) -> EyreResult<()> {
        let inner = &mut *self.inner.lock();
        Self::detail_mut(inner, &key)
            .ok_or_else(|| eyre!("route does not exist"))?
            .last_checked_ts = Some(cur_ts);
        Ok(())
    }

    /// Mark route as used
    pub fn touch_route_used(&mut self, key: &DHTKey, cur_ts: u64) -> EyreResult<()> {
        let inner = &mut *self.inner.lock();
        Self::detail_mut(inner, &key)
            .ok_or_else(|| eyre!("route does not exist"))?
            .last_used_ts = Some(cur_ts);
        Ok(())
    }

    /// Record latency on the route
    pub fn record_latency(&mut self, key: &DHTKey, latency: u64) -> EyreResult<()> {
        let inner = &mut *self.inner.lock();

        let rsd = Self::detail_mut(inner, &key).ok_or_else(|| eyre!("route does not exist"))?;
        rsd.latency_stats = rsd.latency_stats_accounting.record_latency(latency);
        Ok(())
    }

    /// Get the calculated latency stats
    pub fn latency_stats(&mut self, key: &DHTKey) -> EyreResult<LatencyStats> {
        let inner = &mut *self.inner.lock();
        Ok(Self::detail_mut(inner, &key)
            .ok_or_else(|| eyre!("route does not exist"))?
            .latency_stats
            .clone())
    }

    /// Add download transfers to route
    pub fn add_down(&mut self, key: &DHTKey, bytes: u64) -> EyreResult<()> {
        let inner = &mut *self.inner.lock();
        let rsd = Self::detail_mut(inner, &key).ok_or_else(|| eyre!("route does not exist"))?;
        rsd.transfer_stats_accounting.add_down(bytes);
        Ok(())
    }

    /// Add upload transfers to route
    pub fn add_up(&mut self, key: &DHTKey, bytes: u64) -> EyreResult<()> {
        let inner = &mut *self.inner.lock();
        let rsd = Self::detail_mut(inner, &key).ok_or_else(|| eyre!("route does not exist"))?;
        rsd.transfer_stats_accounting.add_up(bytes);
        Ok(())
    }

    /// Process transfer statistics to get averages
    pub fn roll_transfers(&mut self, last_ts: u64, cur_ts: u64) {
        let inner = &mut *self.inner.lock();
        for rsd in inner.content.details.values_mut() {
            rsd.transfer_stats_accounting.roll_transfers(
                last_ts,
                cur_ts,
                &mut rsd.transfer_stats_down_up,
            );
        }
    }
}
