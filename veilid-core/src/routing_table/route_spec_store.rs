use super::*;
use crate::veilid_api::*;
use rkyv::{
    with::Skip, Archive as RkyvArchive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize,
};

/// The size of the remote private route cache
const REMOTE_PRIVATE_ROUTE_CACHE_SIZE: usize = 1024;
/// Remote private route cache entries expire in 5 minutes if they haven't been used
const REMOTE_PRIVATE_ROUTE_CACHE_EXPIRY: u64 = 300_000_000u64;

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

#[derive(Clone, Debug, RkyvArchive, RkyvSerialize, RkyvDeserialize)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct KeyPair {
    key: DHTKey,
    secret: DHTKeySecret,
}

#[derive(Clone, Debug, RkyvArchive, RkyvSerialize, RkyvDeserialize)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct RouteSpecDetail {
    /// Secret key
    #[with(Skip)]
    pub secret_key: DHTKeySecret,
    /// Route hops
    pub hops: Vec<DHTKey>,
    /// Route noderefs
    #[with(Skip)]
    hop_node_refs: Vec<NodeRef>,
    /// Transfers up and down
    transfer_stats_down_up: TransferStatsDownUp,
    /// Latency stats
    latency_stats: LatencyStats,
    /// Accounting mechanism for this route's RPC latency
    #[with(Skip)]
    latency_stats_accounting: LatencyStatsAccounting,
    /// Accounting mechanism for the bandwidth across this route
    #[with(Skip)]
    transfer_stats_accounting: TransferStatsAccounting,
    /// Published private route, do not reuse for ephemeral routes
    /// Not serialized because all routes should be re-published when restarting
    #[with(Skip)]
    published: bool,
    // Can optimize the rendering of this route, using node ids only instead of full peer info
    #[with(Skip)]
    reachable: bool,
    /// Timestamp of when the route was created
    created_ts: u64,
    /// Timestamp of when the route was last checked for validity
    last_checked_ts: Option<u64>,
    /// Timestamp of when the route was last used for anything
    last_used_ts: Option<u64>,
    /// Directions this route is guaranteed to work in
    #[with(RkyvEnumSet)]
    directions: DirectionSet,
    /// Stability preference (prefer reliable nodes over faster)
    pub stability: Stability,
    /// Sequencing preference (connection oriented protocols vs datagram)
    pub sequencing: Sequencing,
}

/// The core representation of the RouteSpecStore that can be serialized
#[derive(Debug, Clone, Default, RkyvArchive, RkyvSerialize, RkyvDeserialize)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct RouteSpecStoreContent {
    /// All of the routes we have allocated so far
    details: HashMap<DHTKey, RouteSpecDetail>,
}

/// What remote private routes have seen
#[derive(Debug, Clone, Default)]
struct RemotePrivateRouteInfo {
    // The private route itself
    private_route: Option<PrivateRoute>,
    /// Timestamp of when the route was last used for anything
    last_used_ts: u64,
    /// The time this remote private route last responded
    last_replied_ts: Option<u64>,
    /// Did this remote private route see our node info due to no safety route in use
    seen_our_node_info: bool,
}

/// Ephemeral data used to help the RouteSpecStore operate efficiently
#[derive(Debug)]
pub struct RouteSpecStoreCache {
    /// How many times nodes have been used
    used_nodes: HashMap<DHTKey, usize>,
    /// How many times nodes have been used at the terminal point of a route
    used_end_nodes: HashMap<DHTKey, usize>,
    /// Route spec hop cache, used to quickly disqualify routes
    hop_cache: HashSet<Vec<u8>>,
    /// Has a remote private route responded to a question and when
    remote_private_route_cache: LruCache<DHTKey, RemotePrivateRouteInfo>,
}

impl Default for RouteSpecStoreCache {
    fn default() -> Self {
        Self {
            used_nodes: Default::default(),
            used_end_nodes: Default::default(),
            hop_cache: Default::default(),
            remote_private_route_cache: LruCache::new(REMOTE_PRIVATE_ROUTE_CACHE_SIZE),
        }
    }
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
fn route_permutation_to_hop_cache(nodes: &[PeerInfo], perm: &[usize]) -> Vec<u8> {
    let mut cache: Vec<u8> = Vec::with_capacity(perm.len() * DHT_KEY_LENGTH);
    for n in perm {
        cache.extend_from_slice(&nodes[*n].node_id.key.bytes)
    }
    cache
}

/// number of route permutations is the number of unique orderings
/// for a set of nodes, given that the first node is fixed
fn _get_route_permutation_count(hop_count: usize) -> usize {
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

type PermFunc<'t> = Box<dyn Fn(&[usize]) -> Option<(Vec<usize>, Vec<u8>)> + Send + 't>;

/// get the route permutation at particular 'perm' index, starting at the 'start' index
/// for a set of 'hop_count' nodes. the first node is always fixed, and the maximum
/// number of permutations is given by get_route_permutation_count()

fn with_route_permutations(
    hop_count: usize,
    start: usize,
    f: &PermFunc,
) -> Option<(Vec<usize>, Vec<u8>)> {
    if hop_count == 0 {
        unreachable!();
    }
    // initial permutation
    let mut permutation: Vec<usize> = Vec::with_capacity(hop_count);
    for n in 0..hop_count {
        permutation.push(start + n);
    }
    // if we have one hop or two, then there's only one permutation
    if hop_count == 1 || hop_count == 2 {
        return f(&permutation);
    }

    // heaps algorithm, but skipping the first element
    fn heaps_permutation(
        permutation: &mut [usize],
        size: usize,
        f: &PermFunc,
    ) -> Option<(Vec<usize>, Vec<u8>)> {
        if size == 1 {
            return f(&permutation);
        }

        for i in 0..size {
            let out = heaps_permutation(permutation, size - 1, f);
            if out.is_some() {
                return out;
            }
            if size % 2 == 1 {
                permutation.swap(1, size);
            } else {
                permutation.swap(1 + i, size);
            }
        }

        None
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
        let (max_route_hop_count, default_route_hop_count) = {
            let config = routing_table.network_manager().config();
            let c = config.get();
            (
                c.network.rpc.max_route_hop_count as usize,
                c.network.rpc.default_route_hop_count as usize,
            )
        };

        // Get frozen blob from table store
        let table_store = routing_table.network_manager().table_store();
        let rsstdb = table_store.open("RouteSpecStore", 1).await?;
        let mut content: RouteSpecStoreContent =
            rsstdb.load_rkyv(0, b"content")?.unwrap_or_default();

        // Look up all route hop noderefs since we can't serialize those
        let mut dead_keys = Vec::new();
        for (k, rsd) in &mut content.details {
            for h in &rsd.hops {
                let Some(nr) = routing_table.lookup_node_ref(*h) else {
                    dead_keys.push(*k);
                    break;
                };
                rsd.hop_node_refs.push(nr);
            }
        }
        for k in dead_keys {
            log_rtab!(debug "no entry, killing off private route: {}", k.encode());
            content.details.remove(&k);
        }

        // Load secrets from pstore
        let pstore = routing_table.network_manager().protected_store();
        let out: Vec<KeyPair> = pstore
            .load_user_secret_rkyv("RouteSpecStore")
            .await?
            .unwrap_or_default();

        let mut dead_keys = Vec::new();
        for KeyPair { key, secret } in out {
            if let Some(rsd) = content.details.get_mut(&key) {
                rsd.secret_key = secret;
            } else {
                dead_keys.push(key);
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
                max_route_hop_count,
                default_route_hop_count,
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

        // Save all the fields we care about to the frozen blob in table storage
        let table_store = self
            .unlocked_inner
            .routing_table
            .network_manager()
            .table_store();
        let rsstdb = table_store.open("RouteSpecStore", 1).await?;
        rsstdb.store_rkyv(0, b"content", &content)?;

        // // Keep secrets in protected store as well
        let pstore = self
            .unlocked_inner
            .routing_table
            .network_manager()
            .protected_store();

        let mut out: Vec<KeyPair> = Vec::with_capacity(content.details.len());
        for (k, v) in &content.details {
            out.push(KeyPair {
                key: *k,
                secret: v.secret_key,
            });
        }

        let _ = pstore.save_user_secret_rkyv("RouteSpecStore", &out).await?; // ignore if this previously existed or not

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

    /// Purge the route spec store
    pub async fn purge(&self) -> EyreResult<()> {
        {
            let inner = &mut *self.inner.lock();
            inner.content = Default::default();
            inner.cache = Default::default();
        }
        self.save().await
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
        avoid_node_ids: &[DHTKey],
    ) -> EyreResult<Option<DHTKey>> {
        let inner = &mut *self.inner.lock();
        let routing_table = self.unlocked_inner.routing_table.clone();
        let rti = &mut *routing_table.inner.write();

        self.allocate_route_inner(
            inner,
            rti,
            stability,
            sequencing,
            hop_count,
            directions,
            avoid_node_ids,
        )
    }

    fn allocate_route_inner(
        &self,
        inner: &mut RouteSpecStoreInner,
        rti: &RoutingTableInner,
        stability: Stability,
        sequencing: Sequencing,
        hop_count: usize,
        directions: DirectionSet,
        avoid_node_ids: &[DHTKey],
    ) -> EyreResult<Option<DHTKey>> {
        use core::cmp::Ordering;

        if hop_count < 1 {
            bail!("Not allocating route less than one hop in length");
        }

        if hop_count > self.unlocked_inner.max_route_hop_count {
            bail!("Not allocating route longer than max route hop count");
        }

        // Get relay node id if we have one
        let opt_relay_id = rti
            .relay_node(RoutingDomain::PublicInternet)
            .map(|nr| nr.node_id());

        // Get list of all nodes, and sort them for selection
        let cur_ts = intf::get_timestamp();
        let filter = Box::new(
            move |rti: &RoutingTableInner, k: DHTKey, v: Option<Arc<BucketEntry>>| -> bool {
                // Exclude our own node from routes
                if v.is_none() {
                    return false;
                }
                let v = v.unwrap();

                // Exclude our relay if we have one
                if let Some(relay_id) = opt_relay_id {
                    if k == relay_id {
                        return false;
                    }
                }

                // Exclude nodes on our local network
                let on_local_network = v.with(rti, |_rti, e| {
                    e.node_info(RoutingDomain::LocalNetwork).is_some()
                });
                if on_local_network {
                    return false;
                }

                // Exclude nodes we have specifically chosen to avoid
                if avoid_node_ids.contains(&k) {
                    return false;
                }

                // Exclude nodes with no publicinternet nodeinfo, or incompatible nodeinfo or node status won't route
                v.with(rti, move |_rti, e| {
                    let node_info_ok =
                        if let Some(sni) = e.signed_node_info(RoutingDomain::PublicInternet) {
                            sni.has_sequencing_matched_dial_info(sequencing)
                        } else {
                            false
                        };
                    let node_status_ok =
                        if let Some(ns) = e.node_status(RoutingDomain::PublicInternet) {
                            ns.will_route()
                        } else {
                            false
                        };

                    node_info_ok && node_status_ok
                })
            },
        ) as RoutingTableEntryFilter;
        let filters = VecDeque::from([filter]);
        let compare = |rti: &RoutingTableInner,
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
        let transform =
            |rti: &RoutingTableInner, k: DHTKey, v: Option<Arc<BucketEntry>>| -> PeerInfo {
                // Return the peerinfo for that key
                v.unwrap().with(rti, |_rti, e| {
                    e.make_peer_info(k, RoutingDomain::PublicInternet.into())
                        .unwrap()
                        .clone()
                })
            };

        // Pull the whole routing table in sorted order
        let node_count = rti.get_entry_count(
            RoutingDomain::PublicInternet.into(),
            BucketEntryState::Unreliable,
        );
        let nodes =
            rti.find_peers_with_sort_and_filter(node_count, cur_ts, filters, compare, transform);

        // If we couldn't find enough nodes, wait until we have more nodes in the routing table
        if nodes.len() < hop_count {
            log_rtab!(debug "not enough nodes to construct route at this time");
            return Ok(None);
        }

        // Now go through nodes and try to build a route we haven't seen yet
        let perm_func = Box::new(|permutation: &[usize]| {
            // Get the route cache key
            let cache_key = route_permutation_to_hop_cache(&nodes, permutation);

            // Skip routes we have already seen
            if inner.cache.hop_cache.contains(&cache_key) {
                return None;
            }

            // Ensure this route is viable by checking that each node can contact the next one
            if directions.contains(Direction::Outbound) {
                let our_peer_info = rti.get_own_peer_info(RoutingDomain::PublicInternet);
                let mut previous_node = &our_peer_info;
                let mut reachable = true;
                for n in permutation {
                    let current_node = nodes.get(*n).unwrap();
                    let cm = rti.get_contact_method(
                        RoutingDomain::PublicInternet,
                        previous_node,
                        current_node,
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
                    return None;
                }
            }
            if directions.contains(Direction::Inbound) {
                let our_peer_info = rti.get_own_peer_info(RoutingDomain::PublicInternet);
                let mut next_node = &our_peer_info;
                let mut reachable = true;
                for n in permutation.iter().rev() {
                    let current_node = nodes.get(*n).unwrap();
                    let cm = rti.get_contact_method(
                        RoutingDomain::PublicInternet,
                        next_node,
                        current_node,
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
                    return None;
                }
            }
            // Keep this route
            let route_nodes = permutation.to_vec();
            Some((route_nodes, cache_key))
        }) as PermFunc;

        let mut route_nodes: Vec<usize> = Vec::new();
        let mut cache_key: Vec<u8> = Vec::new();

        for start in 0..(nodes.len() - hop_count) {
            // Try the permutations available starting with 'start'
            if let Some((rn, ck)) = with_route_permutations(hop_count, start, &perm_func) {
                route_nodes = rn;
                cache_key = ck;
                break;
            }
        }
        if route_nodes.is_empty() {
            log_rtab!(debug "unable to find unique route at this time");
            return Ok(None);
        }

        // Got a unique route, lets build the detail, register it, and return it
        let hops: Vec<DHTKey> = route_nodes.iter().map(|v| nodes[*v].node_id.key).collect();
        let hop_node_refs = hops
            .iter()
            .map(|k| {
                rti.lookup_node_ref(self.unlocked_inner.routing_table.clone(), *k)
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

        drop(perm_func);

        // Add to cache
        Self::add_to_cache(&mut inner.cache, cache_key, &rsd);

        // Keep route in spec store
        inner.content.details.insert(public_key, rsd);

        Ok(Some(public_key))
    }

    pub fn validate_signatures(
        &self,
        public_key: &DHTKey,
        signatures: &[DHTSignature],
        data: &[u8],
        last_hop_id: DHTKey,
    ) -> EyreResult<Option<(DHTKeySecret, SafetySpec)>> {
        let inner = &*self.inner.lock();
        let rsd = Self::detail(inner, &public_key).ok_or_else(|| eyre!("route does not exist"))?;

        // Ensure we have the right number of signatures
        if signatures.len() != rsd.hops.len() - 1 {
            // Wrong number of signatures
            log_rpc!(debug "wrong number of signatures ({} should be {}) for routed operation on private route {}", signatures.len(), rsd.hops.len() - 1, public_key);
            return Ok(None);
        }
        // Validate signatures to ensure the route was handled by the nodes and not messed with
        // This is in private route (reverse) order as we are receiving over the route
        for (hop_n, hop_public_key) in rsd.hops.iter().rev().enumerate() {
            // The last hop is not signed, as the whole packet is signed
            if hop_n == signatures.len() {
                // Verify the node we received the routed operation from is the last hop in our route
                if *hop_public_key != last_hop_id {
                    log_rpc!(debug "received routed operation from the wrong hop ({} should be {}) on private route {}", hop_public_key.encode(), last_hop_id.encode(), public_key);
                    return Ok(None);
                }
            } else {
                // Verify a signature for a hop node along the route
                if let Err(e) = verify(hop_public_key, data, &signatures[hop_n]) {
                    log_rpc!(debug "failed to verify signature for hop {} at {} on private route {}: {}", hop_n, hop_public_key, public_key, e);
                    return Ok(None);
                }
            }
        }
        // We got the correct signatures, return a key and response safety spec
        Ok(Some((
            rsd.secret_key,
            SafetySpec {
                preferred_route: Some(*public_key),
                hop_count: rsd.hops.len(),
                stability: rsd.stability,
                sequencing: rsd.sequencing,
            },
        )))
    }

    /// Test an allocated route for continuity
    pub async fn test_route(&self, key: &DHTKey) -> EyreResult<bool> {
        let inner = &mut *self.inner.lock();
        let rsd = Self::detail(inner, &key).ok_or_else(|| eyre!("route does not exist"))?;
        let rpc_processor = self.unlocked_inner.routing_table.rpc_processor();

        // Target is the last hop
        let target = rsd.hop_node_refs.last().unwrap().clone();
        let hop_count = rsd.hops.len();
        let stability = rsd.stability;
        let sequencing = rsd.sequencing;

        // Test with ping to end
        let res = match rpc_processor
            .rpc_call_status(Destination::Direct {
                target,
                safety_selection: SafetySelection::Safe(SafetySpec {
                    preferred_route: Some(key.clone()),
                    hop_count,
                    stability,
                    sequencing,
                }),
            })
            .await?
        {
            NetworkResult::Value(v) => v,
            _ => {
                // Did not error, but did not come back, just return false
                return Ok(false);
            }
        };

        Ok(true)
    }

    /// Release an allocated route that is no longer in use
    pub fn release_route(&self, public_key: DHTKey) -> EyreResult<()> {
        let mut inner = self.inner.lock();
        let Some(detail) = inner.content.details.remove(&public_key) else {
            bail!("can't release route that was never allocated");
        };

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
        match inner
            .cache
            .used_end_nodes
            .entry(*detail.hops.last().unwrap())
        {
            std::collections::hash_map::Entry::Occupied(mut o) => {
                *o.get_mut() -= 1;
                if *o.get() == 0 {
                    o.remove();
                }
            }
            std::collections::hash_map::Entry::Vacant(_) => {
                panic!("used_end_nodes cache should have contained hop");
            }
        }
        Ok(())
    }

    /// Find first matching unpublished route that fits into the selection criteria
    fn first_unpublished_route_inner<'a>(
        inner: &'a RouteSpecStoreInner,
        min_hop_count: usize,
        max_hop_count: usize,
        stability: Stability,
        sequencing: Sequencing,
        directions: DirectionSet,
        avoid_node_ids: &[DHTKey],
    ) -> Option<DHTKey> {
        for detail in &inner.content.details {
            if detail.1.stability >= stability
                && detail.1.sequencing >= sequencing
                && detail.1.hops.len() >= min_hop_count
                && detail.1.hops.len() <= max_hop_count
                && detail.1.directions.is_subset(directions)
                && !detail.1.published
            {
                let mut avoid = false;
                for h in &detail.1.hops {
                    if avoid_node_ids.contains(h) {
                        avoid = true;
                        break;
                    }
                }
                if !avoid {
                    return Some(*detail.0);
                }
            }
        }
        None
    }

    /// List all routes
    pub fn list_routes(&self) -> Vec<DHTKey> {
        let inner = self.inner.lock();
        let mut out = Vec::with_capacity(inner.content.details.len());
        for detail in &inner.content.details {
            out.push(*detail.0);
        }
        out
    }

    /// Get the debug description of a route
    pub fn debug_route(&self, key: &DHTKey) -> Option<String> {
        let inner = &*self.inner.lock();
        Self::detail(inner, key).map(|rsd| format!("{:#?}", rsd))
    }

    //////////////////////////////////////////////////////////////////////

    /// Compiles a safety route to the private route, with caching
    /// Returns an Err() if the parameters are wrong
    /// Returns Ok(None) if no allocation could happen at this time (not an error)
    pub fn compile_safety_route(
        &self,
        safety_selection: SafetySelection,
        mut private_route: PrivateRoute,
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
            // Safety route spec to use
            SafetySelection::Safe(safety_spec) => safety_spec,
            // Safety route stub with the node's public key as the safety route key since it's the 0th hop
            SafetySelection::Unsafe(sequencing) => {
                let Some(pr_first_hop_node) = private_route.pop_first_hop() else {
                    bail!("compiled private route should have first hop");
                };

                let opt_first_hop = match pr_first_hop_node {
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
        };

        let PrivateRouteHops::FirstHop(pr_first_hop) = &private_route.hops else {
            bail!("compiled private route should have first hop");
        };

        // Get the safety route to use from the spec
        let avoid_node_id = match &pr_first_hop.node {
            RouteNode::NodeId(n) => n.key,
            RouteNode::PeerInfo(p) => p.node_id.key,
        };
        let Some(sr_pubkey) = self.get_route_for_safety_spec_inner(inner, rti, &safety_spec, Direction::Outbound.into(), &[avoid_node_id])? else {
            // No safety route could be found for this spec
            return Ok(None);
        };
        let safety_rsd = Self::detail_mut(inner, &sr_pubkey).unwrap();

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
            // Forward order (safety route), but inside-out
            for h in (1..safety_rsd.hops.len()).rev() {
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

    /// Get a route that matches a particular safety spec
    fn get_route_for_safety_spec_inner(
        &self,
        inner: &mut RouteSpecStoreInner,
        rti: &RoutingTableInner,
        safety_spec: &SafetySpec,
        direction: DirectionSet,
        avoid_node_ids: &[DHTKey],
    ) -> EyreResult<Option<DHTKey>> {
        // Ensure the total hop count isn't too long for our config
        let max_route_hop_count = self.unlocked_inner.max_route_hop_count;
        if safety_spec.hop_count == 0 {
            bail!("safety route hop count is zero");
        }
        if safety_spec.hop_count > max_route_hop_count {
            bail!("safety route hop count too long");
        }

        // See if the preferred route is here
        if let Some(preferred_route) = safety_spec.preferred_route {
            if inner.content.details.contains_key(&preferred_route) {
                return Ok(Some(preferred_route));
            }
        }

        // Select a safety route from the pool or make one if we don't have one that matches
        let sr_pubkey = if let Some(sr_pubkey) = Self::first_unpublished_route_inner(
            inner,
            safety_spec.hop_count,
            safety_spec.hop_count,
            safety_spec.stability,
            safety_spec.sequencing,
            direction,
            avoid_node_ids,
        ) {
            // Found a route to use
            sr_pubkey
        } else {
            // No route found, gotta allocate one
            let sr_pubkey = match self
                .allocate_route_inner(
                    inner,
                    rti,
                    safety_spec.stability,
                    safety_spec.sequencing,
                    safety_spec.hop_count,
                    direction,
                    avoid_node_ids,
                )
                .map_err(RPCError::internal)?
            {
                Some(pk) => pk,
                None => return Ok(None),
            };
            sr_pubkey
        };
        Ok(Some(sr_pubkey))
    }

    /// Get a private sroute to use for the answer to question
    pub fn get_private_route_for_safety_spec(
        &self,
        safety_spec: &SafetySpec,
        avoid_node_ids: &[DHTKey],
    ) -> EyreResult<Option<DHTKey>> {
        let inner = &mut *self.inner.lock();
        let routing_table = self.unlocked_inner.routing_table.clone();
        let rti = &*routing_table.inner.read();

        Ok(self.get_route_for_safety_spec_inner(
            inner,
            rti,
            safety_spec,
            Direction::Inbound.into(),
            avoid_node_ids,
        )?)
    }

    /// Assemble private route for publication
    pub fn assemble_private_route(
        &self,
        key: &DHTKey,
        optimize: Option<bool>,
    ) -> EyreResult<PrivateRoute> {
        let inner = &*self.inner.lock();
        let routing_table = self.unlocked_inner.routing_table.clone();
        let rti = &*routing_table.inner.read();

        let rsd = Self::detail(inner, key).ok_or_else(|| eyre!("route does not exist"))?;

        // See if we can optimize this compilation yet
        // We don't want to include full nodeinfo if we don't have to
        let optimize = optimize.unwrap_or(rsd.reachable);

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
        // iterate hops in private route order (reverse, but inside out)
        for h in 0..hop_count {
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
            // add hop for 'FirstHop'
            hop_count: (hop_count + 1).try_into().unwrap(),
            hops: PrivateRouteHops::FirstHop(route_hop),
        };
        Ok(private_route)
    }

    /// Import a remote private route for compilation
    pub fn import_remote_private_route(&self, blob: Vec<u8>) -> EyreResult<DHTKey> {
        // decode the pr blob
        let private_route = RouteSpecStore::blob_to_private_route(blob)?;

        // store the private route in our cache
        let inner = &mut *self.inner.lock();
        let cur_ts = intf::get_timestamp();

        let key = Self::with_create_remote_private_route(inner, cur_ts, private_route, |r| {
            r.private_route.as_ref().unwrap().public_key.clone()
        });
        Ok(key)
    }

    /// Retrieve an imported remote private route by its public key
    pub fn get_remote_private_route(&self, key: &DHTKey) -> EyreResult<PrivateRoute> {
        let inner = &mut *self.inner.lock();
        let cur_ts = intf::get_timestamp();
        let Some(pr) = Self::with_get_remote_private_route(inner, cur_ts, key, |r| {
            r.private_route.as_ref().unwrap().clone()
        }) else {
            bail!("remote private route not found");
        };
        Ok(pr)
    }

    // get or create a remote private route cache entry
    fn with_create_remote_private_route<F, R>(
        inner: &mut RouteSpecStoreInner,
        cur_ts: u64,
        private_route: PrivateRoute,
        f: F,
    ) -> R
    where
        F: FnOnce(&mut RemotePrivateRouteInfo) -> R,
    {
        let pr_pubkey = private_route.public_key;

        let rpr = inner
            .cache
            .remote_private_route_cache
            .entry(pr_pubkey)
            .and_modify(|rpr| {
                if cur_ts - rpr.last_used_ts >= REMOTE_PRIVATE_ROUTE_CACHE_EXPIRY {
                    // Start fresh if this had expired
                    rpr.last_used_ts = cur_ts;
                    rpr.last_replied_ts = None;
                    rpr.seen_our_node_info = false;
                } else {
                    // If not expired, just mark as being used
                    rpr.last_used_ts = cur_ts;
                }
            })
            .or_insert_with(|| RemotePrivateRouteInfo {
                // New remote private route cache entry
                private_route: Some(private_route),
                last_used_ts: cur_ts,
                last_replied_ts: None,
                seen_our_node_info: false,
            });
        f(rpr)
    }

    // get a remote private route cache entry
    fn with_get_remote_private_route<F, R>(
        inner: &mut RouteSpecStoreInner,
        cur_ts: u64,
        key: &DHTKey,
        f: F,
    ) -> Option<R>
    where
        F: FnOnce(&mut RemotePrivateRouteInfo) -> R,
    {
        let rpr = inner.cache.remote_private_route_cache.get_mut(key)?;
        if cur_ts - rpr.last_used_ts < REMOTE_PRIVATE_ROUTE_CACHE_EXPIRY {
            return Some(f(rpr));
        }
        inner.cache.remote_private_route_cache.remove(key);
        None
    }

    /// Check to see if this remote (not ours) private route has seen our node info yet
    /// This returns true if we have sent non-safety-route node info to the
    /// private route and gotten a response before
    pub fn has_remote_private_route_seen_our_node_info(&self, key: &DHTKey) -> bool {
        let inner = &mut *self.inner.lock();
        let cur_ts = intf::get_timestamp();
        Self::with_get_remote_private_route(inner, cur_ts, key, |rpr| rpr.seen_our_node_info)
            .unwrap_or_default()
    }

    /// Mark a remote private route as having seen our node info
    pub fn mark_remote_private_route_seen_our_node_info(
        &self,
        key: &DHTKey,
        cur_ts: u64,
    ) -> EyreResult<()> {
        let inner = &mut *self.inner.lock();
        if Self::with_get_remote_private_route(inner, cur_ts, key, |rpr| {
            rpr.seen_our_node_info = true;
        })
        .is_none()
        {
            bail!("private route is missing from store: {}", key);
        }
        Ok(())
    }

    /// Mark a remote private route as having replied to a question {
    pub fn mark_remote_private_route_replied(&self, key: &DHTKey, cur_ts: u64) -> EyreResult<()> {
        let inner = &mut *self.inner.lock();
        if Self::with_get_remote_private_route(inner, cur_ts, key, |rpr| {
            rpr.last_replied_ts = Some(cur_ts);
        })
        .is_none()
        {
            bail!("private route is missing from store: {}", key);
        }
        Ok(())
    }

    /// Mark a remote private route as having beed used {
    pub fn mark_remote_private_route_used(&self, key: &DHTKey, cur_ts: u64) -> EyreResult<()> {
        let inner = &mut *self.inner.lock();
        if Self::with_get_remote_private_route(inner, cur_ts, key, |rpr| {
            rpr.last_used_ts = cur_ts;
        })
        .is_none()
        {
            bail!("private route is missing from store: {}", key);
        }
        Ok(())
    }

    /// Clear caches when local our local node info changes
    pub fn reset(&self) {
        let inner = &mut *self.inner.lock();

        // Clean up local allocated routes
        for (_k, v) in &mut inner.content.details {
            // Must republish route now
            v.published = false;
            // Route is not known reachable now
            v.reachable = false;
            // We have yet to check it since local node info changed
            v.last_checked_ts = None;
        }

        // Reset private route cache
        for (_k, v) in &mut inner.cache.remote_private_route_cache {
            v.last_replied_ts = None;
            v.seen_our_node_info = false;
        }
    }

    /// Mark route as published
    /// When first deserialized, routes must be re-published in order to ensure they remain
    /// in the RouteSpecStore.
    pub fn mark_route_published(&self, key: &DHTKey, published: bool) -> EyreResult<()> {
        let inner = &mut *self.inner.lock();
        Self::detail_mut(inner, key)
            .ok_or_else(|| eyre!("route does not exist"))?
            .published = published;
        Ok(())
    }

    /// Mark route as reachable
    /// When first deserialized, routes must be re-tested for reachability
    /// This can be used to determine if routes need to be sent with full peerinfo or can just use a node id
    pub fn mark_route_reachable(&self, key: &DHTKey, reachable: bool) -> EyreResult<()> {
        let inner = &mut *self.inner.lock();
        Self::detail_mut(inner, key)
            .ok_or_else(|| eyre!("route does not exist"))?
            .reachable = reachable;
        Ok(())
    }

    /// Mark route as checked
    pub fn touch_route_checked(&self, key: &DHTKey, cur_ts: u64) -> EyreResult<()> {
        let inner = &mut *self.inner.lock();
        Self::detail_mut(inner, key)
            .ok_or_else(|| eyre!("route does not exist"))?
            .last_checked_ts = Some(cur_ts);
        Ok(())
    }

    /// Mark route as used
    pub fn touch_route_used(&self, key: &DHTKey, cur_ts: u64) -> EyreResult<()> {
        let inner = &mut *self.inner.lock();
        Self::detail_mut(inner, key)
            .ok_or_else(|| eyre!("route does not exist"))?
            .last_used_ts = Some(cur_ts);
        Ok(())
    }

    /// Record latency on the route
    pub fn record_latency(&self, key: &DHTKey, latency: u64) -> EyreResult<()> {
        let inner = &mut *self.inner.lock();

        let rsd = Self::detail_mut(inner, key).ok_or_else(|| eyre!("route does not exist"))?;
        rsd.latency_stats = rsd.latency_stats_accounting.record_latency(latency);
        Ok(())
    }

    /// Get the calculated latency stats
    pub fn latency_stats(&self, key: &DHTKey) -> EyreResult<LatencyStats> {
        let inner = &mut *self.inner.lock();
        Ok(Self::detail_mut(inner, key)
            .ok_or_else(|| eyre!("route does not exist"))?
            .latency_stats
            .clone())
    }

    /// Add download transfers to route
    pub fn add_down(&self, key: &DHTKey, bytes: u64) -> EyreResult<()> {
        let inner = &mut *self.inner.lock();
        let rsd = Self::detail_mut(inner, key).ok_or_else(|| eyre!("route does not exist"))?;
        rsd.transfer_stats_accounting.add_down(bytes);
        Ok(())
    }

    /// Add upload transfers to route
    pub fn add_up(&self, key: &DHTKey, bytes: u64) -> EyreResult<()> {
        let inner = &mut *self.inner.lock();
        let rsd = Self::detail_mut(inner, key).ok_or_else(|| eyre!("route does not exist"))?;
        rsd.transfer_stats_accounting.add_up(bytes);
        Ok(())
    }

    /// Process transfer statistics to get averages
    pub fn roll_transfers(&self, last_ts: u64, cur_ts: u64) {
        let inner = &mut *self.inner.lock();
        for rsd in inner.content.details.values_mut() {
            rsd.transfer_stats_accounting.roll_transfers(
                last_ts,
                cur_ts,
                &mut rsd.transfer_stats_down_up,
            );
        }
    }

    /// Convert private route to binary blob
    pub fn private_route_to_blob(private_route: &PrivateRoute) -> EyreResult<Vec<u8>> {
        let mut pr_message = ::capnp::message::Builder::new_default();
        let mut pr_builder = pr_message.init_root::<veilid_capnp::private_route::Builder>();
        encode_private_route(&private_route, &mut pr_builder)
            .wrap_err("failed to encode private route")?;

        let mut buffer = vec![];
        capnp::serialize_packed::write_message(&mut buffer, &pr_message)
            .map_err(RPCError::internal)
            .wrap_err("failed to convert builder to vec")?;
        Ok(buffer)
    }

    /// Convert binary blob to private route
    pub fn blob_to_private_route(blob: Vec<u8>) -> EyreResult<PrivateRoute> {
        let reader = capnp::serialize_packed::read_message(
            blob.as_slice(),
            capnp::message::ReaderOptions::new(),
        )
        .map_err(RPCError::internal)
        .wrap_err("failed to make message reader")?;

        let pr_reader = reader
            .get_root::<veilid_capnp::private_route::Reader>()
            .map_err(RPCError::internal)
            .wrap_err("failed to make reader for private_route")?;
        decode_private_route(&pr_reader).wrap_err("failed to decode private route")
    }
}
