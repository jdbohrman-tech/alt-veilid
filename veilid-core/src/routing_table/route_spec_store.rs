use super::*;
use crate::veilid_api::*;
use serde::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct RouteSpecDetail {
    /// Secret key
    #[serde(skip)]
    secret_key: DHTKeySecret,
    /// Route hops
    hops: Vec<DHTKey>,
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
    /// Timestamp of when the route was created
    created_ts: u64,
    /// Timestamp of when the route was last checked for validity
    last_checked_ts: Option<u64>,
    /// Directions this route is guaranteed to work in
    directions: DirectionSet,
    /// Reliability
    reliable: bool,
}

/// The core representation of the RouteSpecStore that can be serialized
#[derive(Debug, Default, Serialize, Deserialize)]
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
pub struct RouteSpecStore {
    /// Serialize RouteSpecStore content
    content: RouteSpecStoreContent,
    /// RouteSpecStore cache
    cache: RouteSpecStoreCache,
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
fn with_route_permutations<F>(hop_count: usize, start: usize, f: F) -> bool
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
    fn heaps_permutation<F>(permutation: &mut [usize], size: usize, f: F) -> bool
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
            if heaps_permutation(permutation, size - 1, f) {
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
    pub fn new() -> Self {
        Self {
            content: RouteSpecStoreContent {
                details: HashMap::new(),
            },
            cache: Default::default(),
        }
    }

    pub async fn load(routing_table: RoutingTable) -> EyreResult<RouteSpecStore> {
        // Get cbor blob from table store
        let table_store = routing_table.network_manager().table_store();
        let rsstdb = table_store.open("RouteSpecStore", 1).await?;
        let content = rsstdb.load_cbor(0, b"content").await?.unwrap_or_default();
        let mut rss = RouteSpecStore {
            content,
            cache: Default::default(),
        };

        // Load secrets from pstore
        let pstore = routing_table.network_manager().protected_store();
        let mut dead_keys = Vec::new();
        for (k, v) in &mut rss.content.details {
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
            rss.content.details.remove(&k);
        }

        // Rebuild the routespecstore cache
        rss.rebuild_cache(routing_table);
        Ok(rss)
    }

    pub async fn save(&self, routing_table: RoutingTable) -> EyreResult<()> {
        // Save all the fields we care about to the cbor blob in table storage
        let table_store = routing_table.network_manager().table_store();
        let rsstdb = table_store.open("RouteSpecStore", 1).await?;
        rsstdb.store_cbor(0, b"content", &self.content).await?;

        // Keep secrets in protected store as well
        let pstore = routing_table.network_manager().protected_store();
        for (k, v) in &self.content.details {
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

    fn add_to_cache(&mut self, cache_key: Vec<u8>, rsd: &RouteSpecDetail) {
        if !self.cache.hop_cache.insert(cache_key) {
            panic!("route should never be inserted twice");
        }
        for h in &rsd.hops {
            self.cache
                .used_nodes
                .entry(*h)
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
        self.cache
            .used_end_nodes
            .entry(*rsd.hops.last().unwrap())
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }

    fn rebuild_cache(&mut self, routing_table: RoutingTable) {
        for v in self.content.details.values() {
            let cache_key = route_hops_to_hop_cache(&v.hops);
            self.add_to_cache(cache_key, &v);
        }
    }

    fn detail_mut(&mut self, public_key: &DHTKey) -> &mut RouteSpecDetail {
        self.content.details.get_mut(&public_key).unwrap()
    }

    /// Create a new route
    /// Prefers nodes that are not currently in use by another route
    /// The route is not yet tested for its reachability
    /// Returns None if no route could be allocated at this time
    pub async fn allocate_route(
        &mut self,
        routing_table: RoutingTable,
        reliable: bool,
        hop_count: usize,
        directions: DirectionSet,
    ) -> EyreResult<Option<DHTKey>> {
        use core::cmp::Ordering;

        let max_route_hop_count = {
            let config = routing_table.network_manager().config();
            let c = config.get();
            let max_route_hop_count = c.network.rpc.max_route_hop_count;
            max_route_hop_count.into()
        };

        if hop_count < 2 {
            bail!("Not allocating route less than two hops in length");
        }

        if hop_count > max_route_hop_count {
            bail!("Not allocating route longer than max route hop count");
        }

        // Get list of all nodes, and sort them for selection
        let cur_ts = intf::get_timestamp();
        let dial_info_sort = if reliable {
            Some(DialInfoDetail::reliable_sort)
        } else {
            None
        };
        let filter = |rti, k: DHTKey, v: Option<Arc<BucketEntry>>| -> bool {
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
                    ni.has_any_dial_info()
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
            let e1_used_end = self
                .cache
                .used_end_nodes
                .get(&v1.0)
                .cloned()
                .unwrap_or_default();
            let e2_used_end = self
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
            let e1_used = self
                .cache
                .used_nodes
                .get(&v1.0)
                .cloned()
                .unwrap_or_default();
            let e2_used = self
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
                v2.1.as_ref().unwrap().with(rti, |_rti, e2| {
                    if reliable {
                        BucketEntryInner::cmp_oldest_reliable(cur_ts, e1, e2)
                    } else {
                        BucketEntryInner::cmp_fastest_reliable(cur_ts, e1, e2)
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
        let node_count = routing_table.get_entry_count(
            RoutingDomain::PublicInternet.into(),
            BucketEntryState::Unreliable,
        );
        let nodes = routing_table
            .find_peers_with_sort_and_filter(node_count, cur_ts, filter, compare, transform);

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
                if self.cache.hop_cache.contains(&cache_key) {
                    return false;
                }

                // Ensure this route is viable by checking that each node can contact the next one
                if directions.contains(Direction::Outbound) {
                    let our_node_info =
                        routing_table.get_own_node_info(RoutingDomain::PublicInternet);
                    let mut previous_node_info = &our_node_info;
                    let mut reachable = true;
                    for n in permutation {
                        let current_node_info = &nodes.get(*n).as_ref().unwrap().1;
                        let cm = NetworkManager::get_node_contact_method(
                            previous_node_info,
                            current_node_info,
                        );
                        if matches!(cm, ContactMethod::Unreachable) {
                            reachable = false;
                            break;
                        }
                        previous_node_info = current_node_info;
                    }
                    if !reachable {
                        return false;
                    }
                }
                if directions.contains(Direction::Inbound) {
                    let our_node_info =
                        routing_table.get_own_node_info(RoutingDomain::PublicInternet);
                    let mut next_node_info = &our_node_info;
                    let mut reachable = true;
                    for n in permutation.iter().rev() {
                        let current_node_info = &nodes.get(*n).as_ref().unwrap().1;
                        let cm = NetworkManager::get_node_contact_method(
                            current_node_info,
                            next_node_info,
                        );
                        if matches!(cm, ContactMethod::Unreachable) {
                            reachable = false;
                            break;
                        }
                        next_node_info = current_node_info;
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
            .map(|v| routing_table.lookup_node_ref(nodes[*v].0).unwrap())
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
            created_ts: cur_ts,
            last_checked_ts: None,
            directions,
            reliable,
        };

        // Add to cache
        self.add_to_cache(cache_key, &rsd);

        // Keep route in spec store
        self.content.details.insert(public_key, rsd);

        Ok(Some(public_key))
    }

    pub fn release_route(&mut self, public_key: DHTKey) {
        if let Some(detail) = self.content.details.remove(&public_key) {
            // Remove from hop cache
            let cache_key = route_hops_to_hop_cache(&detail.hops);
            if !self.cache.hop_cache.remove(&cache_key) {
                panic!("hop cache should have contained cache key");
            }
            // Remove from used nodes cache
            for h in &detail.hops {
                match self.cache.used_nodes.entry(*h) {
                    std::collections::hash_map::Entry::Occupied(o) => {
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
            match self.cache.used_nodes.entry(*detail.hops.last().unwrap()) {
                std::collections::hash_map::Entry::Occupied(o) => {
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

    pub fn first_unpublished_route(
        &mut self,
        reliable: bool,
        min_hop_count: usize,
        max_hop_count: usize,
        directions: DirectionSet,
    ) -> Option<DHTKey> {
        for detail in &self.content.details {
            if detail.1.reliable == reliable
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

    /// xxx add route compiler here
    ///

    /// Mark route as published
    /// When first deserialized, routes must be re-published in order to ensure they remain
    /// in the RouteSpecStore.
    pub fn mark_route_published(&mut self, key: &DHTKey) {
        self.detail_mut(&key).published = true;
    }

    /// Mark route as checked
    pub fn touch_route_checked(&mut self, key: &DHTKey, cur_ts: u64) {
        self.detail_mut(&key).last_checked_ts = Some(cur_ts);
    }

    /// Record latency on the route
    pub fn record_latency(&mut self, key: &DHTKey, latency: u64) {
        let lsa = &mut self.detail_mut(&key).latency_stats_accounting;
        self.detail_mut(&key).latency_stats = lsa.record_latency(latency);
    }

    /// Get the calculated latency stats
    pub fn latency_stats(&self, key: &DHTKey) -> LatencyStats {
        self.detail_mut(&key).latency_stats.clone()
    }

    /// Add download transfers to route
    pub fn add_down(&mut self, key: &DHTKey, bytes: u64) {
        let tsa = &mut self.detail_mut(&key).transfer_stats_accounting;
        tsa.add_down(bytes);
    }

    /// Add upload transfers to route
    pub fn add_up(&mut self, key: &DHTKey, bytes: u64) {
        let tsa = &mut self.detail_mut(&key).transfer_stats_accounting;
        tsa.add_up(bytes);
    }

    /// Process transfer statistics to get averages
    pub fn roll_transfers(&mut self, last_ts: u64, cur_ts: u64) {
        for rsd in self.content.details.values_mut() {
            rsd.transfer_stats_accounting.roll_transfers(
                last_ts,
                cur_ts,
                &mut rsd.transfer_stats_down_up,
            );
        }
    }
}
