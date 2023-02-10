use super::*;
use crate::veilid_api::*;
use rkyv::{
    with::Skip, Archive as RkyvArchive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize,
};

/// The size of the remote private route cache
const REMOTE_PRIVATE_ROUTE_CACHE_SIZE: usize = 1024;
/// Remote private route cache entries expire in 5 minutes if they haven't been used
const REMOTE_PRIVATE_ROUTE_CACHE_EXPIRY: TimestampDuration = TimestampDuration::new(300_000_000u64);
/// Amount of time a route can remain idle before it gets tested
const ROUTE_MIN_IDLE_TIME_MS: u32 = 30_000;
/// The size of the compiled route cache
const COMPILED_ROUTE_CACHE_SIZE: usize = 256;


// Compiled route key for caching
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct CompiledRouteCacheKey {
    sr_pubkey: PublicKey,
    pr_pubkey: PublicKey,    
}

/// Compiled route (safety route + private route)
#[derive(Clone, Debug)]
pub struct CompiledRoute {
    /// The safety route attached to the private route
    pub safety_route: SafetyRoute,
    /// The secret used to encrypt the message payload
    pub secret: SecretKey,
    /// The node ref to the first hop in the compiled route
    pub first_hop: NodeRef,
}

#[derive(Clone, Debug, Default, RkyvArchive, RkyvSerialize, RkyvDeserialize)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct RouteStats {
    /// Consecutive failed to send count
    #[with(Skip)]
    pub failed_to_send: u32,
    /// Questions lost
    #[with(Skip)]
    pub questions_lost: u32,
    /// Timestamp of when the route was created
    pub created_ts: Timestamp,
    /// Timestamp of when the route was last checked for validity
    #[with(Skip)]
    pub last_tested_ts: Option<Timestamp>,
    /// Timestamp of when the route was last sent to
    #[with(Skip)]
    pub last_sent_ts: Option<Timestamp>,
    /// Timestamp of when the route was last received over
    #[with(Skip)]
    pub last_received_ts: Option<Timestamp>,
    /// Transfers up and down
    pub transfer_stats_down_up: TransferStatsDownUp,
    /// Latency stats
    pub latency_stats: LatencyStats,
    /// Accounting mechanism for this route's RPC latency
    #[with(Skip)]
    latency_stats_accounting: LatencyStatsAccounting,
    /// Accounting mechanism for the bandwidth across this route
    #[with(Skip)]
    transfer_stats_accounting: TransferStatsAccounting,
}

impl RouteStats {
    /// Make new route stats
    pub fn new(created_ts: Timestamp) -> Self {
        Self {
            created_ts,
            ..Default::default()
        }
    }
    /// Mark a route as having failed to send
    pub fn record_send_failed(&mut self) {
        self.failed_to_send += 1;
    }

    /// Mark a route as having lost a question
    pub fn record_question_lost(&mut self) {
        self.questions_lost += 1;
    }

    /// Mark a route as having received something
    pub fn record_received(&mut self, cur_ts: Timestamp, bytes: ByteCount) {
        self.last_received_ts = Some(cur_ts);
        self.last_tested_ts = Some(cur_ts);
        self.transfer_stats_accounting.add_down(bytes);
    }

    /// Mark a route as having been sent to
    pub fn record_sent(&mut self, cur_ts: Timestamp, bytes: ByteCount) {
        self.last_sent_ts = Some(cur_ts);
        self.transfer_stats_accounting.add_up(bytes);
    }

    /// Mark a route as having been sent to
    pub fn record_latency(&mut self, latency: TimestampDuration) {
        self.latency_stats = self.latency_stats_accounting.record_latency(latency);
    }

    /// Mark a route as having been tested
    pub fn record_tested(&mut self, cur_ts: Timestamp) {
        self.last_tested_ts = Some(cur_ts);

        // Reset question_lost and failed_to_send if we test clean
        self.failed_to_send = 0;
        self.questions_lost = 0;
    }

    /// Roll transfers for these route stats
    pub fn roll_transfers(&mut self, last_ts: Timestamp, cur_ts: Timestamp) {
        self.transfer_stats_accounting.roll_transfers(
            last_ts,
            cur_ts,
            &mut self.transfer_stats_down_up,
        )
    }

    /// Get the latency stats
    pub fn latency_stats(&self) -> &LatencyStats {
        &self.latency_stats
    }

    /// Get the transfer stats
    pub fn transfer_stats(&self) -> &TransferStatsDownUp {
        &self.transfer_stats_down_up
    }

    /// Reset stats when network restarts
    pub fn reset(&mut self) {
        self.last_tested_ts = None;
        self.last_sent_ts = None;
        self.last_received_ts = None;
    }

    /// Check if a route needs testing
    pub fn needs_testing(&self, cur_ts: Timestamp) -> bool {
        // Has the route had any failures lately?
        if self.questions_lost > 0 || self.failed_to_send > 0 {
            // If so, always test
            return true;
        }

        // Has the route been tested within the idle time we'd want to check things?
        // (also if we've received successfully over the route, this will get set)
        if let Some(last_tested_ts) = self.last_tested_ts {
            if cur_ts.saturating_sub(last_tested_ts)
                > TimestampDuration::new(ROUTE_MIN_IDLE_TIME_MS as u64 * 1000u64)
            {
                return true;
            }
        } else {
            // If this route has never been tested, it needs to be
            return true;
        }

        false
    }
}

#[derive(Clone, Debug, RkyvArchive, RkyvSerialize, RkyvDeserialize)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct RouteSpecDetail {
    /// Secret key
    #[with(Skip)]
    secret_key: SecretKey,
    /// Route hops
    hops: Vec<PublicKey>,
    /// Route noderefs
    #[with(Skip)]
    hop_node_refs: Vec<NodeRef>,
    /// Published private route, do not reuse for ephemeral routes
    /// Not serialized because all routes should be re-published when restarting
    #[with(Skip)]
    published: bool,
    /// Directions this route is guaranteed to work in
    #[with(RkyvEnumSet)]
    directions: DirectionSet,
    /// Stability preference (prefer reliable nodes over faster)
    stability: Stability,
    /// Sequencing capability (connection oriented protocols vs datagram)
    can_do_sequenced: bool,
    /// Stats
    stats: RouteStats,
}

impl RouteSpecDetail {
    pub fn get_stats(&self) -> &RouteStats {
        &self.stats
    }
    pub fn get_stats_mut(&mut self) -> &mut RouteStats {
        &mut self.stats
    }
    pub fn is_published(&self) -> bool {
        self.published
    }
    pub fn hop_count(&self) -> usize {
        self.hops.len()
    }
    pub fn get_secret_key(&self) -> SecretKey {
        self.secret_key
    }
    pub fn get_stability(&self) -> Stability {
        self.stability
    }
    pub fn is_sequencing_match(&self, sequencing: Sequencing) -> bool {
        match sequencing {
            Sequencing::NoPreference => true,
            Sequencing::PreferOrdered => true,
            Sequencing::EnsureOrdered => {
                self.can_do_sequenced
            }
        }
    }
}

/// The core representation of the RouteSpecStore that can be serialized
#[derive(Debug, Clone, Default, RkyvArchive, RkyvSerialize, RkyvDeserialize)]
#[archive_attr(repr(C, align(8)), derive(CheckBytes))]
pub struct RouteSpecStoreContent {
    /// All of the routes we have allocated so far
    details: HashMap<PublicKey, RouteSpecDetail>,
}

/// What remote private routes have seen
#[derive(Debug, Clone, Default)]
pub struct RemotePrivateRouteInfo {
    // The private route itself
    private_route: Option<PrivateRoute>,
    /// Did this remote private route see our node info due to no safety route in use
    last_seen_our_node_info_ts: Timestamp,
    /// Last time this remote private route was requested for any reason (cache expiration)
    last_touched_ts: Timestamp,
    /// Stats
    stats: RouteStats,
}

impl RemotePrivateRouteInfo {
    pub fn get_stats(&self) -> &RouteStats {
        &self.stats
    }
    pub fn get_stats_mut(&mut self) -> &mut RouteStats {
        &mut self.stats
    }
}

/// Ephemeral data used to help the RouteSpecStore operate efficiently
#[derive(Debug)]
pub struct RouteSpecStoreCache {
    /// How many times nodes have been used
    used_nodes: HashMap<PublicKey, usize>,
    /// How many times nodes have been used at the terminal point of a route
    used_end_nodes: HashMap<PublicKey, usize>,
    /// Route spec hop cache, used to quickly disqualify routes
    hop_cache: HashSet<Vec<u8>>,
    /// Has a remote private route responded to a question and when
    remote_private_route_cache: LruCache<PublicKey, RemotePrivateRouteInfo>,
    /// Compiled route cache
    compiled_route_cache: LruCache<CompiledRouteCacheKey, SafetyRoute>,
    /// List of dead allocated routes
    dead_routes: Vec<PublicKey>,
    /// List of dead remote routes
    dead_remote_routes: Vec<PublicKey>,
}

impl Default for RouteSpecStoreCache {
    fn default() -> Self {
        Self {
            used_nodes: Default::default(),
            used_end_nodes: Default::default(),
            hop_cache: Default::default(),
            remote_private_route_cache: LruCache::new(REMOTE_PRIVATE_ROUTE_CACHE_SIZE),
            compiled_route_cache: LruCache::new(COMPILED_ROUTE_CACHE_SIZE),
            dead_routes: Default::default(),
            dead_remote_routes: Default::default(),
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

fn route_hops_to_hop_cache(hops: &[PublicKey]) -> Vec<u8> {
    let mut cache: Vec<u8> = Vec::with_capacity(hops.len() * PUBLIC_KEY_LENGTH);
    for hop in hops {
        cache.extend_from_slice(&hop.bytes);
    }
    cache
}

/// get the hop cache key for a particular route permutation
fn route_permutation_to_hop_cache(nodes: &[PeerInfo], perm: &[usize]) -> Vec<u8> {
    let mut cache: Vec<u8> = Vec::with_capacity(perm.len() * PUBLIC_KEY_LENGTH);
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
type PermReturnType = (Vec<usize>, Vec<u8>, bool);
type PermFunc<'t> = Box<dyn Fn(&[usize]) -> Option<PermReturnType> + Send + 't>;

/// get the route permutation at particular 'perm' index, starting at the 'start' index
/// for a set of 'hop_count' nodes. the first node is always fixed, and the maximum
/// number of permutations is given by get_route_permutation_count()

fn with_route_permutations(
    hop_count: usize,
    start: usize,
    f: &PermFunc,
) -> Option<PermReturnType> {
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
    ) -> Option<PermReturnType> {
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

    #[instrument(level = "trace", skip(routing_table), err)]
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

    #[instrument(level = "trace", skip(self), err)]
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
        rsstdb.store_rkyv(0, b"content", &content).await?;

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

    #[instrument(level = "trace", skip(self))]
    pub fn send_route_update(&self) {
        let update_callback = self.unlocked_inner.routing_table.update_callback();

        let (dead_routes, dead_remote_routes) = {
            let mut inner = self.inner.lock();
            if inner.cache.dead_routes.is_empty() && inner.cache.dead_remote_routes.is_empty() {
                // Nothing to do
                return;
            }
            let dead_routes = core::mem::take(&mut inner.cache.dead_routes);
            let dead_remote_routes = core::mem::take(&mut inner.cache.dead_remote_routes);
            (dead_routes, dead_remote_routes)
        };

        let update = VeilidUpdate::Route(VeilidStateRoute {
            dead_routes,
            dead_remote_routes,
        });

        update_callback(update);
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
        public_key: &PublicKey,
    ) -> Option<&'a RouteSpecDetail> {
        inner.content.details.get(public_key)
    }
    fn detail_mut<'a>(
        inner: &'a mut RouteSpecStoreInner,
        public_key: &PublicKey,
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
    #[instrument(level = "trace", skip(self), ret, err)]
    pub fn allocate_route(
        &self,
        stability: Stability,
        sequencing: Sequencing,
        hop_count: usize,
        directions: DirectionSet,
        avoid_node_ids: &[PublicKey],
    ) -> EyreResult<Option<PublicKey>> {
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

    #[instrument(level = "trace", skip(self, inner, rti), ret, err)]
    fn allocate_route_inner(
        &self,
        inner: &mut RouteSpecStoreInner,
        rti: &RoutingTableInner,
        stability: Stability,
        sequencing: Sequencing,
        hop_count: usize,
        directions: DirectionSet,
        avoid_node_ids: &[PublicKey],
    ) -> EyreResult<Option<PublicKey>> {
        use core::cmp::Ordering;

        if hop_count < 1 {
            bail!("Not allocating route less than one hop in length");
        }

        if hop_count > self.unlocked_inner.max_route_hop_count {
            bail!("Not allocating route longer than max route hop count");
        }

        let Some(our_peer_info) = rti.get_own_peer_info(RoutingDomain::PublicInternet) else {
            bail!("Can't allocate route until we have our own peer info");
        };

        // Get relay node id if we have one
        let opt_relay_id = rti
            .relay_node(RoutingDomain::PublicInternet)
            .map(|nr| nr.node_id());

        // Get list of all nodes, and sort them for selection
        let cur_ts = get_aligned_timestamp();
        let filter = Box::new(
            move |rti: &RoutingTableInner, k: PublicKey, v: Option<Arc<BucketEntry>>| -> bool {
                // Exclude our own node from routes
                if v.is_none() {
                    return false;
                }
                let v = v.unwrap();

                // Exclude our relay if we have one
                if let Some(own_relay_id) = opt_relay_id {
                    if k == own_relay_id {
                        return false;
                    }
                }

                // Exclude nodes we have specifically chosen to avoid
                if avoid_node_ids.contains(&k) {
                    return false;
                }

                // Process node info exclusions
                let keep = v.with(rti, |_rti, e| {
                    // Exclude nodes on our local network
                    if e.node_info(RoutingDomain::LocalNetwork).is_some() {
                        return false;
                    }
                    // Exclude nodes that have no publicinternet signednodeinfo
                    let Some(sni) = e.signed_node_info(RoutingDomain::PublicInternet) else {    
                        return false;
                    };
                    // Relay check
                    if let Some(relay_id) = sni.relay_id() {
                        // Exclude nodes whose relays we have chosen to avoid
                        if avoid_node_ids.contains(&relay_id.key) {
                            return false;
                        }
                        // Exclude nodes whose relay is our own relay if we have one
                        if let Some(own_relay_id) = opt_relay_id {
                            if own_relay_id == relay_id.key {
                                return false;
                            }
                        }
                    }
                    return true;
                });
                if !keep {
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
                       v1: &(PublicKey, Option<Arc<BucketEntry>>),
                       v2: &(PublicKey, Option<Arc<BucketEntry>>)|
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

            // apply sequencing preference
            // ensureordered will be taken care of by filter
            // and nopreference doesn't care
            if matches!(sequencing, Sequencing::PreferOrdered) {
                let cmp_seq = v1.1.as_ref().unwrap().with(rti, |rti, e1| {
                    v2.1.as_ref()
                        .unwrap()
                        .with(rti, |_rti, e2| {
                            let e1_can_do_ordered = e1.signed_node_info(RoutingDomain::PublicInternet).map(|sni| sni.has_sequencing_matched_dial_info(sequencing)).unwrap_or(false);
                            let e2_can_do_ordered = e2.signed_node_info(RoutingDomain::PublicInternet).map(|sni| sni.has_sequencing_matched_dial_info(sequencing)).unwrap_or(false);
                            e2_can_do_ordered.cmp(&e1_can_do_ordered)
                        })
                });
                if !matches!(cmp_seq, Ordering::Equal) {
                    return cmp_seq;
                }
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
            |rti: &RoutingTableInner, k: PublicKey, v: Option<Arc<BucketEntry>>| -> PeerInfo {
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

            // Ensure the route doesn't contain both a node and its relay
            let mut seen_nodes: HashSet<PublicKey> = HashSet::new();
            for n in permutation {
                let node = nodes.get(*n).unwrap();
                if !seen_nodes.insert(node.node_id.key) {
                    // Already seen this node, should not be in the route twice
                    return None;
                }
                if let Some(relay_id) = node.signed_node_info.relay_id() {
                    if !seen_nodes.insert(relay_id.key) {
                        // Already seen this node, should not be in the route twice
                        return None;
                    }
                }
            }

            // Ensure this route is viable by checking that each node can contact the next one
            let mut can_do_sequenced = true;
            if directions.contains(Direction::Outbound) {
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

                    // Check if we can do sequenced specifically
                    if can_do_sequenced {
                        let cm = rti.get_contact_method(
                            RoutingDomain::PublicInternet,
                            previous_node,
                            current_node,
                            DialInfoFilter::all(),
                            Sequencing::EnsureOrdered,
                        );
                        if matches!(cm, ContactMethod::Unreachable) {
                            can_do_sequenced = false;
                        }
                    }

                    previous_node = current_node;
                }
                if !reachable {
                    return None;
                }
            }
            if directions.contains(Direction::Inbound) {
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

                    // Check if we can do sequenced specifically
                    if can_do_sequenced {
                        let cm = rti.get_contact_method(
                            RoutingDomain::PublicInternet,
                            next_node,
                            current_node,
                            DialInfoFilter::all(),
                            Sequencing::EnsureOrdered,
                        );
                        if matches!(cm, ContactMethod::Unreachable) {
                            can_do_sequenced = false;
                        }
                    }
                    next_node = current_node;
                }
                if !reachable {
                    return None;
                }
            }
            // Keep this route
            let route_nodes = permutation.to_vec();
            Some((route_nodes, cache_key, can_do_sequenced))
        }) as PermFunc;

        let mut route_nodes: Vec<usize> = Vec::new();
        let mut cache_key: Vec<u8> = Vec::new();
        let mut can_do_sequenced: bool = true;

        for start in 0..(nodes.len() - hop_count) {
            // Try the permutations available starting with 'start'
            if let Some((rn, ck, cds)) = with_route_permutations(hop_count, start, &perm_func) {
                route_nodes = rn;
                cache_key = ck;
                can_do_sequenced = cds;
                break;
            }
        }
        if route_nodes.is_empty() {
            log_rtab!(debug "unable to find unique route at this time");
            return Ok(None);
        }

        // Got a unique route, lets build the detail, register it, and return it
        let hops: Vec<PublicKey> = route_nodes.iter().map(|v| nodes[*v].node_id.key).collect();
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
            published: false,
            directions,
            stability,
            can_do_sequenced,
            stats: RouteStats::new(cur_ts),
        };

        drop(perm_func);

        // Add to cache
        Self::add_to_cache(&mut inner.cache, cache_key, &rsd);

        // Keep route in spec store
        inner.content.details.insert(public_key, rsd);

        Ok(Some(public_key))
    }

    #[instrument(level = "trace", skip(self, data, callback), ret)]
    pub fn with_signature_validated_route<F,R>(
        &self,
        public_key: &PublicKey,
        signatures: &[Signature],
        data: &[u8],
        last_hop_id: PublicKey,
        callback: F,
    ) -> Option<R> 
    where F: FnOnce(&RouteSpecDetail) -> R, 
    R: fmt::Debug,
    {
        let inner = &*self.inner.lock();
        let Some(rsd) = Self::detail(inner, &public_key) else {
            log_rpc!(debug "route does not exist: {:?}", public_key);
            return None;
        };

        // Ensure we have the right number of signatures
        if signatures.len() != rsd.hops.len() - 1 {
            // Wrong number of signatures
            log_rpc!(debug "wrong number of signatures ({} should be {}) for routed operation on private route {}", signatures.len(), rsd.hops.len() - 1, public_key);
            return None;
        }
        // Validate signatures to ensure the route was handled by the nodes and not messed with
        // This is in private route (reverse) order as we are receiving over the route
        for (hop_n, hop_public_key) in rsd.hops.iter().rev().enumerate() {
            // The last hop is not signed, as the whole packet is signed
            if hop_n == signatures.len() {
                // Verify the node we received the routed operation from is the last hop in our route
                if *hop_public_key != last_hop_id {
                    log_rpc!(debug "received routed operation from the wrong hop ({} should be {}) on private route {}", hop_public_key.encode(), last_hop_id.encode(), public_key);
                    return None;
                }
            } else {
                // Verify a signature for a hop node along the route
                if let Err(e) = verify(hop_public_key, data, &signatures[hop_n]) {
                    log_rpc!(debug "failed to verify signature for hop {} at {} on private route {}: {}", hop_n, hop_public_key, public_key, e);
                    return None;
                }
            }
        }
        // We got the correct signatures, return a key and response safety spec     
        Some(callback(rsd))
    }

    #[instrument(level = "trace", skip(self), ret, err)]
    async fn test_allocated_route(&self, key: &PublicKey) -> EyreResult<bool> {
        // Make loopback route to test with
        let dest = {
            let private_route = self.assemble_private_route(key, None)?;

            let inner = &mut *self.inner.lock();
            let rsd = Self::detail(inner, &key).ok_or_else(|| eyre!("route does not exist"))?;

            // Match the private route's hop length for safety route length
            let hop_count = rsd.hops.len();
            // Always test routes with safety routes that are more likely to succeed
            let stability = Stability::Reliable;
            // Routes can test with whatever sequencing they were allocated with
            let sequencing = Sequencing::NoPreference;

            let safety_spec = SafetySpec {
                preferred_route: Some(key.clone()),
                hop_count,
                stability,
                sequencing,
            };
            let safety_selection = SafetySelection::Safe(safety_spec);

            Destination::PrivateRoute {
                private_route,
                safety_selection,
            }
        };

        // Test with double-round trip ping to self
        let rpc_processor = self.unlocked_inner.routing_table.rpc_processor();
        let _res = match rpc_processor.rpc_call_status(dest).await? {
            NetworkResult::Value(v) => v,
            _ => {
                // Did not error, but did not come back, just return false
                return Ok(false);
            }
        };

        Ok(true)
    }

    #[instrument(level = "trace", skip(self), ret, err)]
    async fn test_remote_route(&self, key: &PublicKey) -> EyreResult<bool> {
        // Make private route test
        let dest = {
            // Get the route to test
            let private_route = match self.peek_remote_private_route(key) {
                Some(pr) => pr,
                None => return Ok(false),
            };

            // Get a safety route that is good enough
            let safety_spec = SafetySpec {
                preferred_route: None,
                hop_count: self.unlocked_inner.default_route_hop_count,
                stability: Stability::default(),
                sequencing: Sequencing::default(),
            };

            let safety_selection = SafetySelection::Safe(safety_spec);

            Destination::PrivateRoute {
                private_route,
                safety_selection,
            }
        };

        // Test with double-round trip ping to self
        let rpc_processor = self.unlocked_inner.routing_table.rpc_processor();
        let _res = match rpc_processor.rpc_call_status(dest).await? {
            NetworkResult::Value(v) => v,
            _ => {
                // Did not error, but did not come back, just return false
                return Ok(false);
            }
        };

        Ok(true)
    }

    /// Test an allocated route for continuity
    #[instrument(level = "trace", skip(self), ret, err)]
    pub async fn test_route(&self, key: &PublicKey) -> EyreResult<bool> {
        let is_remote = {
            let inner = &mut *self.inner.lock();
            let cur_ts = get_aligned_timestamp();
            Self::with_peek_remote_private_route(inner, cur_ts, key, |_| {}).is_some()
        };
        if is_remote {
            self.test_remote_route(key).await
        } else {
            self.test_allocated_route(key).await
        }
    }

    /// Release an allocated route that is no longer in use
    #[instrument(level = "trace", skip(self), ret)]
    fn release_allocated_route(&self, public_key: &PublicKey) -> bool {
        let mut inner = self.inner.lock();
        let Some(detail) = inner.content.details.remove(public_key) else {
            return false;
        };

        // Mark it as dead for the update
        inner.cache.dead_routes.push(*public_key);

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
        true
    }

    /// Release an allocated or remote route that is no longer in use
    #[instrument(level = "trace", skip(self), ret)]
    pub fn release_route(&self, key: &PublicKey) -> bool {

        let is_remote = {
            let inner = &mut *self.inner.lock();

            // Release from compiled route cache if it's used there
            self.invalidate_compiled_route_cache(inner, key);

            // Check to see if this is a remote route
            let cur_ts = get_aligned_timestamp();
            Self::with_peek_remote_private_route(inner, cur_ts, key, |_| {}).is_some()
        };

        if is_remote {
            self.release_remote_private_route(key)
        } else {
            self.release_allocated_route(key)
        }
    }

    /// Find first matching unpublished route that fits into the selection criteria
    /// Don't pick any routes that have failed and haven't been tested yet
    fn first_available_route_inner<'a>(
        inner: &'a RouteSpecStoreInner,
        min_hop_count: usize,
        max_hop_count: usize,
        stability: Stability,
        sequencing: Sequencing,
        directions: DirectionSet,
        avoid_node_ids: &[PublicKey],
    ) -> Option<PublicKey> {
        let cur_ts = get_aligned_timestamp();

        let mut routes = Vec::new();

        // Get all valid routes, allow routes that need testing
        // but definitely prefer routes that have been recently tested
        for detail in &inner.content.details {
            if detail.1.stability >= stability
                && detail.1.is_sequencing_match(sequencing)
                && detail.1.hops.len() >= min_hop_count
                && detail.1.hops.len() <= max_hop_count
                && detail.1.directions.is_superset(directions)
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
                    routes.push(detail);
                }
            }
        }

        // Sort the routes by preference
        routes.sort_by(|a, b| {
            let a_needs_testing = a.1.stats.needs_testing(cur_ts);
            let b_needs_testing = b.1.stats.needs_testing(cur_ts);
            if !a_needs_testing && b_needs_testing {
                return cmp::Ordering::Less;
            }
            if !b_needs_testing && a_needs_testing {
                return cmp::Ordering::Greater;
            }
            let a_latency = a.1.stats.latency_stats().average;
            let b_latency = b.1.stats.latency_stats().average;

            a_latency.cmp(&b_latency)
        });

        // Return the best one if we got one
        routes.first().map(|r| *r.0)
    }

    /// List all allocated routes
    pub fn list_allocated_routes<F, R>(&self, mut filter: F) -> Vec<R>
    where
        F: FnMut(&PublicKey, &RouteSpecDetail) -> Option<R>,
    {
        let inner = self.inner.lock();
        let mut out = Vec::with_capacity(inner.content.details.len());
        for detail in &inner.content.details {
            if let Some(x) = filter(detail.0, detail.1) {
                out.push(x);
            }
        }
        out
    }

    /// List all allocated routes
    pub fn list_remote_routes<F, R>(&self, mut filter: F) -> Vec<R>
    where
        F: FnMut(&PublicKey, &RemotePrivateRouteInfo) -> Option<R>,
    {
        let inner = self.inner.lock();
        let mut out = Vec::with_capacity(inner.cache.remote_private_route_cache.len());
        for info in &inner.cache.remote_private_route_cache {
            if let Some(x) = filter(info.0, info.1) {
                out.push(x);
            }
        }
        out
    }

    /// Get the debug description of a route
    pub fn debug_route(&self, key: &PublicKey) -> Option<String> {
        let inner = &mut *self.inner.lock();
        let cur_ts = get_aligned_timestamp();
        // If this is a remote route, print it
        if let Some(s) =
            Self::with_peek_remote_private_route(inner, cur_ts, key, |rpi| format!("{:#?}", rpi))
        {
            return Some(s);
        }
        // Otherwise check allocated routes
        Self::detail(inner, key).map(|rsd| format!("{:#?}", rsd))
    }

    //////////////////////////////////////////////////////////////////////

    // Route cache
    fn add_to_compiled_route_cache(&self, inner: &mut RouteSpecStoreInner, pr_pubkey: PublicKey, safety_route: SafetyRoute)
    {
        let key = CompiledRouteCacheKey {
            sr_pubkey: safety_route.public_key,
            pr_pubkey,
        };

        if let Some(v) = inner.cache.compiled_route_cache.insert(key, safety_route) {
            log_rtab!(error "route cache already contained key: sr_pubkey={:?}, pr_pubkey={:?}", v.public_key, pr_pubkey);
        }
    }

    fn lookup_compiled_route_cache(&self, inner: &mut RouteSpecStoreInner, sr_pubkey: PublicKey, pr_pubkey: PublicKey) -> Option<SafetyRoute> {

        let key = CompiledRouteCacheKey {
            sr_pubkey,
            pr_pubkey,
        };

        inner.cache.compiled_route_cache.get(&key).cloned()
    }

    fn invalidate_compiled_route_cache(&self, inner: &mut RouteSpecStoreInner, dead_key: &PublicKey) {
        let mut dead_entries = Vec::new();
        for (k, _v) in inner.cache.compiled_route_cache.iter() {
            if k.sr_pubkey == *dead_key || k.pr_pubkey == *dead_key {
                dead_entries.push(k.clone());
            }
        }
        for d in dead_entries {
            inner.cache.compiled_route_cache.remove(&d);
        }
    }

    /// Compiles a safety route to the private route, with caching
    /// Returns an Err() if the parameters are wrong
    /// Returns Ok(None) if no allocation could happen at this time (not an error)
    pub fn compile_safety_route(
        &self,
        safety_selection: SafetySelection,
        mut private_route: PrivateRoute,
    ) -> EyreResult<Option<CompiledRoute>> {
        // let profile_start_ts = get_timestamp();

        let inner = &mut *self.inner.lock();
        let routing_table = self.unlocked_inner.routing_table.clone();
        let rti = &mut *routing_table.inner.write();

        let pr_pubkey = private_route.public_key;
        let pr_hopcount = private_route.hop_count as usize;
        let max_route_hop_count = self.unlocked_inner.max_route_hop_count;
        // Check private route hop count isn't larger than the max route hop count plus one for the 'first hop' header
        if pr_hopcount > (max_route_hop_count + 1) {
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
                    RouteNode::PeerInfo(pi) => rti.register_node_with_peer_info(
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
                //println!("compile_safety_route profile (stub): {} us", (get_timestamp() - profile_start_ts));
                return Ok(Some(CompiledRoute {
                    safety_route: SafetyRoute::new_stub(routing_table.node_id(), private_route),
                    secret: routing_table.node_id_secret(),
                    first_hop,
                }));
            }
        };

        // If the safety route requested is also the private route, this is a loopback test, just accept it
        let sr_pubkey = if safety_spec.preferred_route == Some(private_route.public_key) {
            // Private route is also safety route during loopback test
            private_route.public_key
        } else {
            let Some(avoid_node_id) = private_route.first_hop_node_id() else {
                bail!("compiled private route should have first hop");
            };
            let Some(sr_pubkey) = self.get_route_for_safety_spec_inner(inner, rti, &safety_spec, Direction::Outbound.into(), &[avoid_node_id])? else {
                // No safety route could be found for this spec
                return Ok(None);
            };
            sr_pubkey
        };
        
        // Look up a few things from the safety route detail we want for the compiled route and don't borrow inner
        let (optimize, first_hop, secret) = {
            let safety_rsd = Self::detail(inner, &sr_pubkey).ok_or_else(|| eyre!("route missing"))?;
            
            // We can optimize the peer info in this safety route if it has been successfully
            // communicated over either via an outbound test, or used as a private route inbound
            // and we are replying over the same route as our safety route outbound
            let optimize = safety_rsd.stats.last_tested_ts.is_some() || safety_rsd.stats.last_received_ts.is_some();

            // Get the first hop noderef of the safety route
            let mut first_hop = safety_rsd.hop_node_refs.first().unwrap().clone();
            // Ensure sequencing requirement is set on first hop
            first_hop.set_sequencing(safety_spec.sequencing);

            // Get the safety route secret key
            let secret = safety_rsd.secret_key;

            (optimize, first_hop, secret)
        };

        // See if we have a cached route we can use
        if optimize {
            if let Some(safety_route) = self.lookup_compiled_route_cache(inner, sr_pubkey, pr_pubkey) {
                // Build compiled route
                let compiled_route = CompiledRoute {
                    safety_route,
                    secret,
                    first_hop,
                };
                // Return compiled route
                //println!("compile_safety_route profile (cached): {} us", (get_timestamp() - profile_start_ts));
                return Ok(Some(compiled_route));
            }
        }

        // Create hops
        let hops = {
            let safety_rsd = Self::detail(inner, &sr_pubkey).ok_or_else(|| eyre!("route missing"))?;

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

        // Add to cache but only if we have an optimized route
        if optimize {
            self.add_to_compiled_route_cache(inner, pr_pubkey, safety_route.clone());
        }

        // Build compiled route
        let compiled_route = CompiledRoute {
            safety_route,
            secret,
            first_hop,
        };

        // Return compiled route
        //println!("compile_safety_route profile (uncached): {} us", (get_timestamp() - profile_start_ts));
        Ok(Some(compiled_route))
    }

    /// Get a route that matches a particular safety spec
    #[instrument(level = "trace", skip(self, inner, rti), ret, err)]
    fn get_route_for_safety_spec_inner(
        &self,
        inner: &mut RouteSpecStoreInner,
        rti: &RoutingTableInner,
        safety_spec: &SafetySpec,
        direction: DirectionSet,
        avoid_node_ids: &[PublicKey],
    ) -> EyreResult<Option<PublicKey>> {
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
            if let Some(preferred_rsd) = inner.content.details.get(&preferred_route) {
                // Only use the preferred route if it doesn't end with the avoid nodes
                if !avoid_node_ids.contains(preferred_rsd.hops.last().unwrap()) {
                    return Ok(Some(preferred_route));
                }
            }
        }

        // Select a safety route from the pool or make one if we don't have one that matches
        let sr_pubkey = if let Some(sr_pubkey) = Self::first_available_route_inner(
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
    #[instrument(level = "trace", skip(self), ret, err)]
    pub fn get_private_route_for_safety_spec(
        &self,
        safety_spec: &SafetySpec,
        avoid_node_ids: &[PublicKey],
    ) -> EyreResult<Option<PublicKey>> {
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
    #[instrument(level = "trace", skip(self), err)]
    pub fn assemble_private_route(
        &self,
        key: &PublicKey,
        optimized: Option<bool>,
    ) -> EyreResult<PrivateRoute> {
        let inner = &*self.inner.lock();
        let routing_table = self.unlocked_inner.routing_table.clone();
        let rti = &*routing_table.inner.read();

        let rsd = Self::detail(inner, key).ok_or_else(|| eyre!("route does not exist"))?;

        // See if we can optimize this compilation yet
        // We don't want to include full nodeinfo if we don't have to
        let optimized = optimized
            .unwrap_or(rsd.stats.last_tested_ts.is_some() || rsd.stats.last_received_ts.is_some());

        // Make innermost route hop to our own node
        let mut route_hop = RouteHop {
            node: if optimized {
                if !rti.has_valid_own_node_info(RoutingDomain::PublicInternet) {
                    bail!("can't make private routes until our node info is valid");
                }
                RouteNode::NodeId(NodeId::new(routing_table.node_id()))
            } else {
                let Some(pi) = rti.get_own_peer_info(RoutingDomain::PublicInternet) else {
                    bail!("can't make private routes until our node info is valid");
                };
                RouteNode::PeerInfo(pi)
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
                node: if optimized {
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
    #[instrument(level = "trace", skip(self, blob), ret, err)]
    pub fn import_remote_private_route(&self, blob: Vec<u8>) -> EyreResult<PublicKey> {
        // decode the pr blob
        let private_route = RouteSpecStore::blob_to_private_route(blob)?;

        // ensure private route has first hop
        if !matches!(private_route.hops, PrivateRouteHops::FirstHop(_)) {
            bail!("private route must have first hop");
        }

        // ensure this isn't also an allocated route
        let inner = &mut *self.inner.lock();
        if Self::detail(inner, &private_route.public_key).is_some() {
            bail!("should not import allocated route");
        }

        // store the private route in our cache
        let cur_ts = get_aligned_timestamp();
        let key = Self::with_create_remote_private_route(inner, cur_ts, private_route, |r| {
            r.private_route.as_ref().unwrap().public_key.clone()
        });
        Ok(key)
    }

    /// Release a remote private route that is no longer in use
    #[instrument(level = "trace", skip(self), ret)]
    fn release_remote_private_route(&self, key: &PublicKey) -> bool {
        let inner = &mut *self.inner.lock();
        if inner.cache.remote_private_route_cache.remove(key).is_some() {
            // Mark it as dead for the update
            inner.cache.dead_remote_routes.push(*key);
            true
        } else {
            false
        }
    }

    /// Retrieve an imported remote private route by its public key
    pub fn get_remote_private_route(&self, key: &PublicKey) -> Option<PrivateRoute> {
        let inner = &mut *self.inner.lock();
        let cur_ts = get_aligned_timestamp();
        Self::with_get_remote_private_route(inner, cur_ts, key, |r| {
            r.private_route.as_ref().unwrap().clone()
        })
    }

    /// Retrieve an imported remote private route by its public key but don't 'touch' it
    pub fn peek_remote_private_route(&self, key: &PublicKey) -> Option<PrivateRoute> {
        let inner = &mut *self.inner.lock();
        let cur_ts = get_aligned_timestamp();
        Self::with_peek_remote_private_route(inner, cur_ts, key, |r| {
            r.private_route.as_ref().unwrap().clone()
        })
    }

    // get or create a remote private route cache entry
    fn with_create_remote_private_route<F, R>(
        inner: &mut RouteSpecStoreInner,
        cur_ts: Timestamp,
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
                if cur_ts.saturating_sub(rpr.last_touched_ts) >= REMOTE_PRIVATE_ROUTE_CACHE_EXPIRY {
                    // Start fresh if this had expired
                    rpr.last_seen_our_node_info_ts = Timestamp::new(0);
                    rpr.last_touched_ts = cur_ts;
                    rpr.stats = RouteStats::new(cur_ts);
                } else {
                    // If not expired, just mark as being used
                    rpr.last_touched_ts = cur_ts;
                }
            })
            .or_insert_with(|| RemotePrivateRouteInfo {
                // New remote private route cache entry
                private_route: Some(private_route),
                last_seen_our_node_info_ts: Timestamp::new(0),
                last_touched_ts: cur_ts,
                stats: RouteStats::new(cur_ts),
            });

        let out = f(rpr);

        // Ensure we LRU out items
        if inner.cache.remote_private_route_cache.len()
            > inner.cache.remote_private_route_cache.capacity()
        {
            let (dead_k, _) = inner.cache.remote_private_route_cache.remove_lru().unwrap();
            // Mark it as dead for the update
            inner.cache.dead_remote_routes.push(dead_k);
        }

        out
    }

    // get a remote private route cache entry
    fn with_get_remote_private_route<F, R>(
        inner: &mut RouteSpecStoreInner,
        cur_ts: Timestamp,
        key: &PublicKey,
        f: F,
    ) -> Option<R>
    where
        F: FnOnce(&mut RemotePrivateRouteInfo) -> R,
    {
        let rpr = inner.cache.remote_private_route_cache.get_mut(key)?;
        if cur_ts.saturating_sub(rpr.last_touched_ts) < REMOTE_PRIVATE_ROUTE_CACHE_EXPIRY {
            rpr.last_touched_ts = cur_ts;
            return Some(f(rpr));
        }
        inner.cache.remote_private_route_cache.remove(key);
        inner.cache.dead_remote_routes.push(*key);
        None
    }

    // peek a remote private route cache entry
    fn with_peek_remote_private_route<F, R>(
        inner: &mut RouteSpecStoreInner,
        cur_ts: Timestamp,
        key: &PublicKey,
        f: F,
    ) -> Option<R>
    where
        F: FnOnce(&mut RemotePrivateRouteInfo) -> R,
    {
        match inner.cache.remote_private_route_cache.entry(*key) {
            hashlink::lru_cache::Entry::Occupied(mut o) => {
                let rpr = o.get_mut();
                if cur_ts.saturating_sub(rpr.last_touched_ts) < REMOTE_PRIVATE_ROUTE_CACHE_EXPIRY {
                    return Some(f(rpr));
                }
                o.remove();
                inner.cache.dead_remote_routes.push(*key);
                None
            }
            hashlink::lru_cache::Entry::Vacant(_) => None,
        }
    }

    /// Check to see if this remote (not ours) private route has seen our current node info yet
    /// This happens when you communicate with a private route without a safety route
    pub fn has_remote_private_route_seen_our_node_info(&self, key: &PublicKey) -> bool {
        let our_node_info_ts = {
            let rti = &*self.unlocked_inner.routing_table.inner.read();
            let Some(ts) = rti.get_own_node_info_ts(RoutingDomain::PublicInternet) else {
                return false;
            };
            ts
        };

        let opt_rpr_node_info_ts = {
            let inner = &mut *self.inner.lock();
            let cur_ts = get_aligned_timestamp();
            Self::with_peek_remote_private_route(inner, cur_ts, key, |rpr| {
                rpr.last_seen_our_node_info_ts
            })
        };

        let Some(rpr_node_info_ts) = opt_rpr_node_info_ts else {
            return false;
        };

        our_node_info_ts == rpr_node_info_ts
    }

    /// Mark a remote private route as having seen our current node info
    /// PRIVACY:
    /// We do not accept node info timestamps from remote private routes because this would
    /// enable a deanonymization attack, whereby a node could be 'pinged' with a doctored node_info with a
    /// special 'timestamp', which then may be sent back over a private route, identifying that it
    /// was that node that had the private route.
    pub fn mark_remote_private_route_seen_our_node_info(
        &self,
        key: &PublicKey,
        cur_ts: Timestamp,
    ) -> EyreResult<()> {
        let our_node_info_ts = {
            let rti = &*self.unlocked_inner.routing_table.inner.read();
            let Some(ts) = rti.get_own_node_info_ts(RoutingDomain::PublicInternet) else {
                // Node info is invalid, skipping this
                return Ok(());
            };
            ts
        };

        let inner = &mut *self.inner.lock();
        // Check for local route. If this is not a remote private route
        // then we just skip the recording. We may be running a test and using
        // our own local route as the destination private route.
        if let Some(_) = Self::detail_mut(inner, key) {
            return Ok(());
        }
        if Self::with_get_remote_private_route(inner, cur_ts, key, |rpr| {
            rpr.last_seen_our_node_info_ts = our_node_info_ts;
        })
        .is_none()
        {
            bail!("private route is missing from store: {}", key);
        }
        Ok(())
    }

    /// Get the route statistics for any route we know about, local or remote
    pub fn with_route_stats<F, R>(&self, cur_ts: Timestamp, key: &PublicKey, f: F) -> Option<R>
    where
        F: FnOnce(&mut RouteStats) -> R,
    {
        let inner = &mut *self.inner.lock();

        // Check for stub route
        if *key == self.unlocked_inner.routing_table.node_id() {
            return None;
        }
        // Check for local route
        if let Some(rsd) = Self::detail_mut(inner, key) {
            return Some(f(&mut rsd.stats));
        }
        // Check for remote route
        if let Some(res) =
            Self::with_peek_remote_private_route(inner, cur_ts, key, |rpr| f(&mut rpr.stats))
        {
            return Some(res);
        }

        None
    }

    /// Clear caches when local our local node info changes
    #[instrument(level = "trace", skip(self))]
    pub fn reset(&self) {
        let inner = &mut *self.inner.lock();

        // Clean up local allocated routes
        for (_k, v) in &mut inner.content.details {
            // Must republish route now
            v.published = false;
            // Restart stats for routes so we test the route again
            v.stats.reset();
        }

        // Reset private route cache
        for (_k, v) in &mut inner.cache.remote_private_route_cache {
            // Restart stats for routes so we test the route again
            v.stats.reset();
        }
    }

    /// Mark route as published
    /// When first deserialized, routes must be re-published in order to ensure they remain
    /// in the RouteSpecStore.
    pub fn mark_route_published(&self, key: &PublicKey, published: bool) -> EyreResult<()> {
        let inner = &mut *self.inner.lock();
        Self::detail_mut(inner, key)
            .ok_or_else(|| eyre!("route does not exist"))?
            .published = published;
        Ok(())
    }

    /// Process transfer statistics to get averages
    pub fn roll_transfers(&self, last_ts: Timestamp, cur_ts: Timestamp) {
        let inner = &mut *self.inner.lock();

        // Roll transfers for locally allocated routes
        for rsd in inner.content.details.values_mut() {
            rsd.stats.roll_transfers(last_ts, cur_ts);
        }
        // Roll transfers for remote private routes
        for (_k, v) in inner.cache.remote_private_route_cache.iter_mut() {
            v.stats.roll_transfers(last_ts, cur_ts);
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
