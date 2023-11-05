use super::*;
use crate::veilid_api::*;

mod permutation;
mod remote_private_route_info;
mod route_set_spec_detail;
mod route_spec_store_cache;
mod route_spec_store_content;
mod route_stats;

use permutation::*;
use remote_private_route_info::*;
use route_set_spec_detail::*;
use route_spec_store_cache::*;
use route_spec_store_content::*;

pub(crate) use route_spec_store_cache::CompiledRoute;
pub(crate) use route_stats::*;

/// The size of the remote private route cache
const REMOTE_PRIVATE_ROUTE_CACHE_SIZE: usize = 1024;
/// Remote private route cache entries expire in 5 minutes if they haven't been used
const REMOTE_PRIVATE_ROUTE_CACHE_EXPIRY: TimestampDuration = TimestampDuration::new(300_000_000u64);
/// Amount of time a route can remain idle before it gets tested
const ROUTE_MIN_IDLE_TIME_MS: u32 = 30_000;
/// The size of the compiled route cache
const COMPILED_ROUTE_CACHE_SIZE: usize = 256;

#[derive(Debug)]
struct RouteSpecStoreInner {
    /// Serialize RouteSpecStore content
    content: RouteSpecStoreContent,
    /// RouteSpecStore cache
    cache: RouteSpecStoreCache,
}

struct RouteSpecStoreUnlockedInner {
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
pub(crate) struct RouteSpecStore {
    inner: Arc<Mutex<RouteSpecStoreInner>>,
    unlocked_inner: Arc<RouteSpecStoreUnlockedInner>,
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
                content: RouteSpecStoreContent::new(),
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
        let content = RouteSpecStoreContent::load(routing_table.clone()).await?;

        let mut inner = RouteSpecStoreInner {
            content,
            cache: Default::default(),
        };

        // Rebuild the routespecstore cache
        let rti = &*routing_table.inner.read();
        for (_, rssd) in inner.content.iter_details() {
            inner.cache.add_to_cache(rti, rssd);
        }

        // Return the loaded RouteSpecStore
        let rss = RouteSpecStore {
            unlocked_inner: Arc::new(RouteSpecStoreUnlockedInner {
                max_route_hop_count,
                default_route_hop_count,
                routing_table: routing_table.clone(),
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

        // Save our content
        content
            .save(self.unlocked_inner.routing_table.clone())
            .await?;

        Ok(())
    }

    #[instrument(level = "trace", skip(self))]
    pub fn send_route_update(&self) {
        let (dead_routes, dead_remote_routes) = {
            let mut inner = self.inner.lock();
            let Some(dr) = inner.cache.take_dead_routes() else {
                // Nothing to do
                return;
            };
            dr
        };

        let update = VeilidUpdate::RouteChange(Box::new(VeilidRouteChange {
            dead_routes,
            dead_remote_routes,
        }));

        let update_callback = self.unlocked_inner.routing_table.update_callback();
        update_callback(update);
    }

    /// Purge the route spec store
    pub async fn purge(&self) -> VeilidAPIResult<()> {
        {
            let inner = &mut *self.inner.lock();
            inner.content = Default::default();
            inner.cache = Default::default();
        }
        self.save().await.map_err(VeilidAPIError::internal)
    }

    /// Create a new route
    /// Prefers nodes that are not currently in use by another route
    /// The route is not yet tested for its reachability
    /// Returns Err(VeilidAPIError::TryAgain) if no route could be allocated at this time
    /// Returns other errors on failure
    /// Returns Ok(route id string) on success
    #[instrument(level = "trace", skip(self), ret, err(level=Level::TRACE))]
    #[allow(clippy::too_many_arguments)]
    pub fn allocate_route(
        &self,
        crypto_kinds: &[CryptoKind],
        stability: Stability,
        sequencing: Sequencing,
        hop_count: usize,
        directions: DirectionSet,
        avoid_nodes: &[TypedKey],
        automatic: bool,
    ) -> VeilidAPIResult<RouteId> {
        let inner = &mut *self.inner.lock();
        let routing_table = self.unlocked_inner.routing_table.clone();
        let rti = &mut *routing_table.inner.write();

        self.allocate_route_inner(
            inner,
            rti,
            crypto_kinds,
            stability,
            sequencing,
            hop_count,
            directions,
            avoid_nodes,
            automatic,
        )
    }

    #[instrument(level = "trace", skip(self, inner, rti), ret, err(level=Level::TRACE))]
    #[allow(clippy::too_many_arguments)]
    fn allocate_route_inner(
        &self,
        inner: &mut RouteSpecStoreInner,
        rti: &mut RoutingTableInner,
        crypto_kinds: &[CryptoKind],
        stability: Stability,
        sequencing: Sequencing,
        hop_count: usize,
        directions: DirectionSet,
        avoid_nodes: &[TypedKey],
        automatic: bool,
    ) -> VeilidAPIResult<RouteId> {
        use core::cmp::Ordering;

        if hop_count < 1 {
            apibail_invalid_argument!(
                "Not allocating route less than one hop in length",
                "hop_count",
                hop_count
            );
        }

        if hop_count > self.unlocked_inner.max_route_hop_count {
            apibail_invalid_argument!(
                "Not allocating route longer than max route hop count",
                "hop_count",
                hop_count
            );
        }

        // Ensure we have a valid network class so our peer info is useful
        if !rti.has_valid_network_class(RoutingDomain::PublicInternet) {
            apibail_try_again!(
                "unable to allocate route until we have a valid PublicInternet network class"
            );
        };

        // Get our peer info
        let our_peer_info = rti.get_own_peer_info(RoutingDomain::PublicInternet);

        // Get relay node if we have one
        let opt_own_relay_nr = rti
            .relay_node(RoutingDomain::PublicInternet)
            .map(|nr| nr.locked(rti));

        // Get list of all nodes, and sort them for selection
        let cur_ts = get_aligned_timestamp();
        let filter = Box::new(
            |_rti: &RoutingTableInner, entry: Option<Arc<BucketEntry>>| -> bool {
                // Exclude our own node from routes
                if entry.is_none() {
                    return false;
                }
                let entry = entry.unwrap();

                // Exclude our relay if we have one
                if let Some(own_relay_nr) = &opt_own_relay_nr {
                    if own_relay_nr.same_bucket_entry(&entry) {
                        return false;
                    }
                }

                // Process node info exclusions
                let keep = entry.with_inner(|e| {
                    // Exclude nodes that don't have our requested crypto kinds
                    let common_ck = e.common_crypto_kinds(crypto_kinds);
                    if common_ck.len() != crypto_kinds.len() {
                        return false;
                    }

                    // Exclude nodes we have specifically chosen to avoid
                    if e.node_ids().contains_any(avoid_nodes) {
                        return false;
                    }

                    // Exclude nodes on our local network
                    if e.node_info(RoutingDomain::LocalNetwork).is_some() {
                        return false;
                    }

                    // Exclude nodes that have no publicinternet signednodeinfo
                    let Some(sni) = e.signed_node_info(RoutingDomain::PublicInternet) else {
                        return false;
                    };

                    // Relay check
                    let relay_ids = sni.relay_ids();
                    if !relay_ids.is_empty() {
                        // Exclude nodes whose relays we have chosen to avoid
                        if relay_ids.contains_any(avoid_nodes) {
                            return false;
                        }
                        // Exclude nodes whose relay is our own relay if we have one
                        if let Some(own_relay_nr) = &opt_own_relay_nr {
                            if relay_ids.contains_any(&own_relay_nr.node_ids()) {
                                return false;
                            }
                        }
                    }
                    true
                });
                if !keep {
                    return false;
                }

                // Exclude nodes with no publicinternet nodeinfo, or incompatible nodeinfo or node status won't route
                entry.with_inner(|e| {
                    e.signed_node_info(RoutingDomain::PublicInternet)
                        .map(|sni| {
                            sni.has_sequencing_matched_dial_info(sequencing)
                                && sni.node_info().has_capability(CAP_ROUTE)
                        })
                        .unwrap_or(false)
                })
            },
        ) as RoutingTableEntryFilter;
        let filters = VecDeque::from([filter]);
        let compare = |_rti: &RoutingTableInner,
                       entry1: &Option<Arc<BucketEntry>>,
                       entry2: &Option<Arc<BucketEntry>>|
         -> Ordering {
            // Our own node is filtered out
            let entry1 = entry1.as_ref().unwrap().clone();
            let entry2 = entry2.as_ref().unwrap().clone();
            let entry1_node_ids = entry1.with_inner(|e| e.node_ids());
            let entry2_node_ids = entry2.with_inner(|e| e.node_ids());

            // deprioritize nodes that we have already used as end points
            let e1_used_end = inner.cache.get_used_end_node_count(&entry1_node_ids);
            let e2_used_end = inner.cache.get_used_end_node_count(&entry2_node_ids);
            let cmp_used_end = e1_used_end.cmp(&e2_used_end);
            if !matches!(cmp_used_end, Ordering::Equal) {
                return cmp_used_end;
            }

            // deprioritize nodes we have used already anywhere
            let e1_used = inner.cache.get_used_node_count(&entry1_node_ids);
            let e2_used = inner.cache.get_used_node_count(&entry2_node_ids);
            let cmp_used = e1_used.cmp(&e2_used);
            if !matches!(cmp_used, Ordering::Equal) {
                return cmp_used;
            }

            // apply sequencing preference
            // ensureordered will be taken care of by filter
            // and nopreference doesn't care
            if matches!(sequencing, Sequencing::PreferOrdered) {
                let cmp_seq = entry1.with_inner(|e1| {
                    entry2.with_inner(|e2| {
                        let e1_can_do_ordered = e1
                            .signed_node_info(RoutingDomain::PublicInternet)
                            .map(|sni| sni.has_sequencing_matched_dial_info(sequencing))
                            .unwrap_or(false);
                        let e2_can_do_ordered = e2
                            .signed_node_info(RoutingDomain::PublicInternet)
                            .map(|sni| sni.has_sequencing_matched_dial_info(sequencing))
                            .unwrap_or(false);
                        e2_can_do_ordered.cmp(&e1_can_do_ordered)
                    })
                });
                if !matches!(cmp_seq, Ordering::Equal) {
                    return cmp_seq;
                }
            }

            // always prioritize reliable nodes, but sort by oldest or fastest

            entry1.with_inner(|e1| {
                entry2.with_inner(|e2| match stability {
                    Stability::LowLatency => BucketEntryInner::cmp_fastest_reliable(cur_ts, e1, e2),
                    Stability::Reliable => BucketEntryInner::cmp_oldest_reliable(cur_ts, e1, e2),
                })
            })
        };

        let routing_table = self.unlocked_inner.routing_table.clone();
        let transform = |_rti: &RoutingTableInner, entry: Option<Arc<BucketEntry>>| -> NodeRef {
            NodeRef::new(routing_table.clone(), entry.unwrap(), None)
        };

        // Pull the whole routing table in sorted order
        let nodes: Vec<NodeRef> =
            rti.find_peers_with_sort_and_filter(usize::MAX, cur_ts, filters, compare, transform);

        // If we couldn't find enough nodes, wait until we have more nodes in the routing table
        if nodes.len() < hop_count {
            apibail_try_again!("not enough nodes to construct route at this time");
        }

        // Get peer info for everything
        let nodes_pi: Vec<PeerInfo> = nodes
            .iter()
            .map(|nr| {
                nr.locked(rti)
                    .make_peer_info(RoutingDomain::PublicInternet)
                    .unwrap()
            })
            .collect();

        // Now go through nodes and try to build a route we haven't seen yet
        let mut perm_func = Box::new(|permutation: &[usize]| {
            // Get the hop cache key for a particular route permutation
            // uses the same algorithm as RouteSetSpecDetail::make_cache_key
            let route_permutation_to_hop_cache =
                |_rti: &RoutingTableInner, nodes: &[NodeRef], perm: &[usize]| -> Vec<u8> {
                    let mut cache: Vec<u8> = Vec::with_capacity(perm.len() * PUBLIC_KEY_LENGTH);
                    for n in perm {
                        cache.extend_from_slice(&nodes[*n].locked(rti).best_node_id().value.bytes)
                    }
                    cache
                };
            let cache_key = route_permutation_to_hop_cache(rti, &nodes, permutation);

            // Skip routes we have already seen
            if inner.cache.contains_route(&cache_key) {
                return None;
            }

            // Ensure the route doesn't contain both a node and its relay
            let mut seen_nodes: HashSet<TypedKey> = HashSet::new();
            for n in permutation {
                let node = nodes.get(*n).unwrap();
                if !seen_nodes.insert(node.locked(rti).best_node_id()) {
                    // Already seen this node, should not be in the route twice
                    return None;
                }
                let opt_relay = match node.locked_mut(rti).relay(RoutingDomain::PublicInternet) {
                    Ok(r) => r,
                    Err(_) => {
                        // Not selecting a relay through ourselves
                        return None;
                    }
                };
                if let Some(relay) = opt_relay {
                    let relay_id = relay.locked(rti).best_node_id();
                    if !seen_nodes.insert(relay_id) {
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
                    let current_node = nodes_pi.get(*n).unwrap();
                    let cm = rti.get_contact_method(
                        RoutingDomain::PublicInternet,
                        previous_node,
                        current_node,
                        DialInfoFilter::all(),
                        sequencing,
                        None,
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
                            None,
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
                    let current_node = nodes_pi.get(*n).unwrap();
                    let cm = rti.get_contact_method(
                        RoutingDomain::PublicInternet,
                        next_node,
                        current_node,
                        DialInfoFilter::all(),
                        sequencing,
                        None,
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
                            None,
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
            Some((route_nodes, can_do_sequenced))
        }) as PermFunc;

        let mut route_nodes: Vec<usize> = Vec::new();
        let mut can_do_sequenced: bool = true;

        for start in 0..(nodes.len() - hop_count) {
            // Try the permutations available starting with 'start'
            if let Some((rn, cds)) = with_route_permutations(hop_count, start, &mut perm_func) {
                route_nodes = rn;
                can_do_sequenced = cds;
                break;
            }
        }
        if route_nodes.is_empty() {
            apibail_try_again!("unable to find unique route at this time");
        }

        drop(perm_func);

        // Got a unique route, lets build the details, register it, and return it
        let hop_node_refs: Vec<NodeRef> = route_nodes.iter().map(|k| nodes[*k].clone()).collect();
        let mut route_set = BTreeMap::<PublicKey, RouteSpecDetail>::new();
        for crypto_kind in crypto_kinds.iter().copied() {
            let vcrypto = self
                .unlocked_inner
                .routing_table
                .crypto()
                .get(crypto_kind)
                .unwrap();
            let keypair = vcrypto.generate_keypair();
            let hops: Vec<PublicKey> = route_nodes
                .iter()
                .map(|v| {
                    nodes[*v]
                        .locked(rti)
                        .node_ids()
                        .get(crypto_kind)
                        .unwrap()
                        .value
                })
                .collect();

            route_set.insert(
                keypair.key,
                RouteSpecDetail {
                    crypto_kind,
                    secret_key: keypair.secret,
                    hops,
                },
            );
        }

        let rssd = RouteSetSpecDetail::new(
            cur_ts,
            route_set,
            hop_node_refs,
            directions,
            stability,
            can_do_sequenced,
            automatic,
        );

        // make id
        let id = self.generate_allocated_route_id(&rssd)?;

        // Add to cache
        inner.cache.add_to_cache(rti, &rssd);

        // Keep route in spec store
        inner.content.add_detail(id, rssd);

        Ok(id)
    }

    /// validate data using a private route's key and signature chain
    #[cfg_attr(
        feature = "verbose-tracing",
        instrument(level = "trace", skip(self, data, callback), ret)
    )]
    pub fn with_signature_validated_route<F, R>(
        &self,
        public_key: &TypedKey,
        signatures: &[Signature],
        data: &[u8],
        last_hop_id: PublicKey,
        callback: F,
    ) -> Option<R>
    where
        F: FnOnce(&RouteSetSpecDetail, &RouteSpecDetail) -> R,
        R: fmt::Debug,
    {
        let inner = &*self.inner.lock();
        let crypto = self.unlocked_inner.routing_table.crypto();
        let Some(vcrypto) = crypto.get(public_key.kind) else {
            log_rpc!(debug "can't handle route with public key: {:?}", public_key);
            return None;
        };

        let Some(rsid) = inner.content.get_id_by_key(&public_key.value) else {
            log_rpc!(debug "route id does not exist: {:?}", public_key.value);
            return None;
        };
        let Some(rssd) = inner.content.get_detail(&rsid) else {
            log_rpc!(debug "route detail does not exist: {:?}", rsid);
            return None;
        };
        let Some(rsd) = rssd.get_route_by_key(&public_key.value) else {
            log_rpc!(debug "route set {:?} does not have key: {:?}", rsid, public_key.value);
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
                if let Err(e) = vcrypto.verify(hop_public_key, data, &signatures[hop_n]) {
                    log_rpc!(debug "failed to verify signature for hop {} at {} on private route {}: {}", hop_n, hop_public_key, public_key, e);
                    return None;
                }
            }
        }
        // We got the correct signatures, return a key and response safety spec
        Some(callback(rssd, rsd))
    }

    #[cfg_attr(
        feature = "verbose-tracing",
        instrument(level = "trace", skip(self), ret, err)
    )]
    async fn test_allocated_route(&self, private_route_id: RouteId) -> VeilidAPIResult<bool> {
        // Make loopback route to test with
        let (dest, hops) = {
            // Get the best allocated route for this id
            let (key, hops) = {
                let inner = &mut *self.inner.lock();
                let Some(rssd) = inner.content.get_detail(&private_route_id) else {
                    apibail_invalid_argument!(
                        "route id not allocated",
                        "private_route_id",
                        private_route_id
                    );
                };
                let Some(key) = rssd.get_best_route_set_key() else {
                    apibail_internal!("no best key to test allocated route");
                };
                // Get the hops so we can match the route's hop length for safety
                // route length as well as marking nodes as unreliable if this fails
                let hops = rssd.hops_node_refs();
                (key, hops)
            };

            // Get the private route to send to
            let private_route = self.assemble_private_route(&key, None)?;
            // Always test routes with safety routes that are more likely to succeed
            let stability = Stability::Reliable;
            // Routes should test with the most likely to succeed sequencing they are capable of
            let sequencing = Sequencing::PreferOrdered;
            // Hop count for safety spec should match the private route spec
            let hop_count = hops.len();

            let safety_spec = SafetySpec {
                preferred_route: Some(private_route_id),
                hop_count,
                stability,
                sequencing,
            };
            let safety_selection = SafetySelection::Safe(safety_spec);

            (
                Destination::PrivateRoute {
                    private_route,
                    safety_selection,
                },
                hops,
            )
        };

        // Test with double-round trip ping to self
        let rpc_processor = self.unlocked_inner.routing_table.rpc_processor();
        let _res = match rpc_processor.rpc_call_status(dest).await? {
            NetworkResult::Value(v) => v,
            _ => {
                // Did not error, but did not come back, mark the nodes as failed to send, and then return false
                // This will prevent those node from immediately being included in the next allocated route,
                // avoiding the same route being constructed to replace this one when it is removed.
                for hop in hops {
                    hop.report_failed_route_test();
                }
                return Ok(false);
            }
        };

        Ok(true)
    }

    #[instrument(level = "trace", skip(self), ret, err)]
    async fn test_remote_route(&self, private_route_id: RouteId) -> VeilidAPIResult<bool> {
        // Make private route test
        let dest = {
            // Get the route to test
            let Some(private_route) = self.best_remote_private_route(&private_route_id) else {
                apibail_internal!("no best key to test remote route");
            };

            // Always test routes with safety routes that are more likely to succeed
            let stability = Stability::Reliable;
            // Routes should test with the most likely to succeed sequencing they are capable of
            let sequencing = Sequencing::PreferOrdered;

            // Get a safety route that is good enough
            let safety_spec = SafetySpec {
                preferred_route: None,
                hop_count: self.unlocked_inner.default_route_hop_count,
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

    /// Release an allocated route that is no longer in use
    #[instrument(level = "trace", skip(self), ret)]
    fn release_allocated_route(&self, id: RouteId) -> bool {
        let mut inner = self.inner.lock();
        let Some(rssd) = inner.content.remove_detail(&id) else {
            return false;
        };

        // Remove from hop cache
        let rti = &*self.unlocked_inner.routing_table.inner.read();
        if !inner.cache.remove_from_cache(rti, id, &rssd) {
            panic!("hop cache should have contained cache key");
        }

        true
    }

    /// Check if a route id is remote or not
    pub fn is_route_id_remote(&self, id: &RouteId) -> bool {
        let inner = &mut *self.inner.lock();
        let cur_ts = get_aligned_timestamp();
        inner
            .cache
            .peek_remote_private_route_mut(cur_ts, id)
            .is_some()
    }

    /// Test an allocated route for continuity
    #[cfg_attr(
        feature = "verbose-tracing",
        instrument(level = "trace", skip(self), ret, err)
    )]
    pub async fn test_route(&self, id: RouteId) -> VeilidAPIResult<bool> {
        let is_remote = self.is_route_id_remote(&id);
        if is_remote {
            self.test_remote_route(id).await
        } else {
            self.test_allocated_route(id).await
        }
    }

    /// Release an allocated or remote route that is no longer in use
    #[instrument(level = "trace", skip(self), ret)]
    pub fn release_route(&self, id: RouteId) -> bool {
        let is_remote = self.is_route_id_remote(&id);
        if is_remote {
            self.release_remote_private_route(id)
        } else {
            self.release_allocated_route(id)
        }
    }

    /// Find first matching unpublished route that fits into the selection criteria
    /// Don't pick any routes that have failed and haven't been tested yet
    #[allow(clippy::too_many_arguments)]
    fn first_available_route_inner(
        inner: &RouteSpecStoreInner,
        crypto_kind: CryptoKind,
        min_hop_count: usize,
        max_hop_count: usize,
        stability: Stability,
        sequencing: Sequencing,
        directions: DirectionSet,
        avoid_nodes: &[TypedKey],
    ) -> Option<RouteId> {
        let cur_ts = get_aligned_timestamp();

        let mut routes = Vec::new();

        // Get all valid routes, allow routes that need testing
        // but definitely prefer routes that have been recently tested
        for (id, rssd) in inner.content.iter_details() {
            if rssd.get_stability() >= stability
                && rssd.is_sequencing_match(sequencing)
                && rssd.hop_count() >= min_hop_count
                && rssd.hop_count() <= max_hop_count
                && rssd.get_directions().is_superset(directions)
                && rssd.get_route_set_keys().kinds().contains(&crypto_kind)
                && !rssd.is_published()
                && !rssd.contains_nodes(avoid_nodes)
            {
                routes.push((id, rssd));
            }
        }

        // Sort the routes by preference
        routes.sort_by(|a, b| {
            let a_needs_testing = a.1.get_stats().needs_testing(cur_ts);
            let b_needs_testing = b.1.get_stats().needs_testing(cur_ts);
            if !a_needs_testing && b_needs_testing {
                return cmp::Ordering::Less;
            }
            if !b_needs_testing && a_needs_testing {
                return cmp::Ordering::Greater;
            }
            let a_latency = a.1.get_stats().latency_stats().average;
            let b_latency = b.1.get_stats().latency_stats().average;

            a_latency.cmp(&b_latency)
        });

        // Return the best one if we got one
        routes.first().map(|r| *r.0)
    }

    /// List all allocated routes
    pub fn list_allocated_routes<F, R>(&self, mut filter: F) -> Vec<R>
    where
        F: FnMut(&RouteId, &RouteSetSpecDetail) -> Option<R>,
    {
        let inner = self.inner.lock();
        let mut out = Vec::with_capacity(inner.content.get_detail_count());
        for detail in inner.content.iter_details() {
            if let Some(x) = filter(detail.0, detail.1) {
                out.push(x);
            }
        }
        out
    }

    /// List all allocated routes
    pub fn list_remote_routes<F, R>(&self, mut filter: F) -> Vec<R>
    where
        F: FnMut(&RouteId, &RemotePrivateRouteInfo) -> Option<R>,
    {
        let inner = self.inner.lock();
        let mut out = Vec::with_capacity(inner.cache.get_remote_private_route_count());
        for info in inner.cache.iter_remote_private_routes() {
            if let Some(x) = filter(info.0, info.1) {
                out.push(x);
            }
        }
        out
    }

    /// Get the debug description of a route
    pub fn debug_route(&self, id: &RouteId) -> Option<String> {
        let inner = &mut *self.inner.lock();
        let cur_ts = get_aligned_timestamp();
        if let Some(rpri) = inner.cache.peek_remote_private_route_mut(cur_ts, id) {
            return Some(format!("{:#?}", rpri));
        }
        if let Some(rssd) = inner.content.get_detail(id) {
            return Some(format!("{:#?}", rssd));
        }
        None
    }

    //////////////////////////////////////////////////////////////////////

    /// Choose the best private route from a private route set to communicate with
    pub fn best_remote_private_route(&self, id: &RouteId) -> Option<PrivateRoute> {
        let inner = &mut *self.inner.lock();
        let cur_ts = get_aligned_timestamp();
        let rpri = inner.cache.get_remote_private_route(cur_ts, id)?;
        rpri.best_private_route()
    }

    /// Compiles a safety route to the private route, with caching
    /// Returns Err(VeilidAPIError::TryAgain) if no allocation could happen at this time (not an error)
    /// Returns other Err() if the parameters are wrong
    /// Returns Ok(compiled route) on success
    pub fn compile_safety_route(
        &self,
        safety_selection: SafetySelection,
        mut private_route: PrivateRoute,
    ) -> VeilidAPIResult<CompiledRoute> {
        // let profile_start_ts = get_timestamp();
        let inner = &mut *self.inner.lock();
        let routing_table = self.unlocked_inner.routing_table.clone();
        let rti = &mut *routing_table.inner.write();

        // Get useful private route properties
        let crypto_kind = private_route.crypto_kind();
        let crypto = routing_table.crypto();
        let Some(vcrypto) = crypto.get(crypto_kind) else {
            apibail_generic!("crypto not supported for route");
        };
        let pr_pubkey = private_route.public_key.value;
        let pr_hopcount = private_route.hop_count as usize;
        let max_route_hop_count = self.unlocked_inner.max_route_hop_count;

        // Check private route hop count isn't larger than the max route hop count plus one for the 'first hop' header
        if pr_hopcount > (max_route_hop_count + 1) {
            apibail_invalid_argument!(
                "private route hop count too long",
                "private_route.hop_count",
                pr_hopcount
            );
        }
        // See if we are using a safety route, if not, short circuit this operation
        let safety_spec = match safety_selection {
            // Safety route spec to use
            SafetySelection::Safe(safety_spec) => safety_spec,
            // Safety route stub with the node's public key as the safety route key since it's the 0th hop
            SafetySelection::Unsafe(sequencing) => {
                let Some(pr_first_hop_node) = private_route.pop_first_hop() else {
                    apibail_generic!("compiled private route should have first hop");
                };

                let opt_first_hop = match pr_first_hop_node {
                    RouteNode::NodeId(id) => rti
                        .lookup_node_ref(routing_table.clone(), TypedKey::new(crypto_kind, id))
                        .map_err(VeilidAPIError::internal)?,
                    RouteNode::PeerInfo(pi) => Some(
                        rti.register_node_with_peer_info(
                            routing_table.clone(),
                            RoutingDomain::PublicInternet,
                            *pi,
                            false,
                        )
                        .map_err(VeilidAPIError::internal)?,
                    ),
                };
                if opt_first_hop.is_none() {
                    // Can't reach this private route any more
                    apibail_generic!("can't reach private route any more");
                }
                let mut first_hop = opt_first_hop.unwrap();

                // Set sequencing requirement
                first_hop.set_sequencing(sequencing);

                // Return the compiled safety route
                //println!("compile_safety_route profile (stub): {} us", (get_timestamp() - profile_start_ts));
                return Ok(CompiledRoute {
                    safety_route: SafetyRoute::new_stub(
                        routing_table.node_id(crypto_kind),
                        private_route,
                    ),
                    secret: routing_table.node_id_secret_key(crypto_kind),
                    first_hop,
                });
            }
        };

        // If the safety route requested is also the private route, this is a loopback test, just accept it
        let opt_private_route_id = inner.content.get_id_by_key(&pr_pubkey);
        let sr_pubkey = if opt_private_route_id.is_some()
            && safety_spec.preferred_route == opt_private_route_id
        {
            // Private route is also safety route during loopback test
            pr_pubkey
        } else {
            let Some(avoid_node_id) = private_route.first_hop_node_id() else {
                apibail_generic!("compiled private route should have first hop");
            };
            self.get_route_for_safety_spec_inner(
                inner,
                rti,
                crypto_kind,
                &safety_spec,
                Direction::Outbound.into(),
                &[avoid_node_id],
            )?
        };

        // Look up a few things from the safety route detail we want for the compiled route and don't borrow inner
        let Some(safety_route_id) = inner.content.get_id_by_key(&sr_pubkey) else {
            apibail_generic!("safety route id missing");
        };
        let Some(safety_rssd) = inner.content.get_detail(&safety_route_id) else {
            apibail_internal!("safety route set detail missing");
        };
        let Some(safety_rsd) = safety_rssd.get_route_by_key(&sr_pubkey) else {
            apibail_internal!("safety route detail missing");
        };

        // We can optimize the peer info in this safety route if it has been successfully
        // communicated over either via an outbound test, or used as a private route inbound
        // and we are replying over the same route as our safety route outbound
        let optimize = safety_rssd.get_stats().last_tested_ts.is_some()
            || safety_rssd.get_stats().last_received_ts.is_some();

        // Get the first hop noderef of the safety route
        let mut first_hop = safety_rssd.hop_node_ref(0).unwrap();

        // Ensure sequencing requirement is set on first hop
        first_hop.set_sequencing(safety_spec.sequencing);

        // Get the safety route secret key
        let secret = safety_rsd.secret_key;

        // See if we have a cached route we can use
        if optimize {
            if let Some(safety_route) = inner
                .cache
                .lookup_compiled_route_cache(sr_pubkey, pr_pubkey)
            {
                // Build compiled route
                let compiled_route = CompiledRoute {
                    safety_route,
                    secret,
                    first_hop,
                };
                // Return compiled route
                //println!("compile_safety_route profile (cached): {} us", (get_timestamp() - profile_start_ts));
                return Ok(compiled_route);
            }
        }

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
            let mut nonce = vcrypto.random_nonce();
            // Forward order (safety route), but inside-out
            for h in (1..safety_rsd.hops.len()).rev() {
                // Get blob to encrypt for next hop
                blob_data = {
                    // Encrypt the previous blob ENC(nonce, DH(PKhop,SKsr))
                    let dh_secret = vcrypto
                        .cached_dh(&safety_rsd.hops[h], &safety_rsd.secret_key)
                        .map_err(VeilidAPIError::internal)?;
                    let enc_msg_data = vcrypto
                        .encrypt_aead(blob_data.as_slice(), &nonce, &dh_secret, None)
                        .map_err(VeilidAPIError::internal)?;

                    // Make route hop data
                    let route_hop_data = RouteHopData {
                        nonce,
                        blob: enc_msg_data,
                    };

                    // Make route hop
                    let route_hop = RouteHop {
                        node: if optimize {
                            // Optimized, no peer info, just the dht key
                            RouteNode::NodeId(safety_rsd.hops[h])
                        } else {
                            // Full peer info, required until we are sure the route has been fully established
                            let node_id = TypedKey::new(safety_rsd.crypto_kind, safety_rsd.hops[h]);
                            let pi = rti
                                .with_node_entry(node_id, |entry| {
                                    entry.with(rti, |_rti, e| {
                                        e.make_peer_info(RoutingDomain::PublicInternet)
                                    })
                                })
                                .flatten();
                            if pi.is_none() {
                                apibail_internal!("peer info should exist for route but doesn't");
                            }
                            RouteNode::PeerInfo(Box::new(pi.unwrap()))
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
                nonce = vcrypto.random_nonce();
            }

            // Encode first RouteHopData
            let dh_secret = vcrypto
                .cached_dh(&safety_rsd.hops[0], &safety_rsd.secret_key)
                .map_err(VeilidAPIError::internal)?;
            let enc_msg_data = vcrypto
                .encrypt_aead(blob_data.as_slice(), &nonce, &dh_secret, None)
                .map_err(VeilidAPIError::internal)?;

            let route_hop_data = RouteHopData {
                nonce,
                blob: enc_msg_data,
            };

            SafetyRouteHops::Data(route_hop_data)
        };

        // Build safety route
        let safety_route = SafetyRoute {
            public_key: TypedKey::new(crypto_kind, sr_pubkey),
            hop_count: safety_spec.hop_count as u8,
            hops,
        };

        // Add to cache but only if we have an optimized route
        if optimize {
            inner
                .cache
                .add_to_compiled_route_cache(pr_pubkey, safety_route.clone());
        }

        // Build compiled route
        let compiled_route = CompiledRoute {
            safety_route,
            secret,
            first_hop,
        };

        // Return compiled route
        //println!("compile_safety_route profile (uncached): {} us", (get_timestamp() - profile_start_ts));
        Ok(compiled_route)
    }

    /// Get an allocated route that matches a particular safety spec
    #[cfg_attr(
        feature = "verbose-tracing",
        instrument(level = "trace", skip(self, inner, rti), ret, err)
    )]
    fn get_route_for_safety_spec_inner(
        &self,
        inner: &mut RouteSpecStoreInner,
        rti: &mut RoutingTableInner,
        crypto_kind: CryptoKind,
        safety_spec: &SafetySpec,
        direction: DirectionSet,
        avoid_nodes: &[TypedKey],
    ) -> VeilidAPIResult<PublicKey> {
        // Ensure the total hop count isn't too long for our config
        let max_route_hop_count = self.unlocked_inner.max_route_hop_count;
        if safety_spec.hop_count == 0 {
            apibail_invalid_argument!(
                "safety route hop count is zero",
                "safety_spec.hop_count",
                safety_spec.hop_count
            );
        }
        if safety_spec.hop_count > max_route_hop_count {
            apibail_invalid_argument!(
                "safety route hop count too long",
                "safety_spec.hop_count",
                safety_spec.hop_count
            );
        }

        // See if the preferred route is here
        if let Some(preferred_route) = safety_spec.preferred_route {
            if let Some(preferred_rssd) = inner.content.get_detail(&preferred_route) {
                // Only use the preferred route if it has the desired crypto kind
                if let Some(preferred_key) = preferred_rssd.get_route_set_keys().get(crypto_kind) {
                    // Only use the preferred route if it doesn't contain the avoid nodes
                    if !preferred_rssd.contains_nodes(avoid_nodes) {
                        return Ok(preferred_key.value);
                    }
                }
            }
        }

        // Select a safety route from the pool or make one if we don't have one that matches
        let sr_route_id = if let Some(sr_route_id) = Self::first_available_route_inner(
            inner,
            crypto_kind,
            safety_spec.hop_count,
            safety_spec.hop_count,
            safety_spec.stability,
            safety_spec.sequencing,
            direction,
            avoid_nodes,
        ) {
            // Found a route to use
            sr_route_id
        } else {
            // No route found, gotta allocate one
            self.allocate_route_inner(
                inner,
                rti,
                &[crypto_kind],
                safety_spec.stability,
                safety_spec.sequencing,
                safety_spec.hop_count,
                direction,
                avoid_nodes,
                true,
            )?
        };

        let sr_pubkey = inner
            .content
            .get_detail(&sr_route_id)
            .unwrap()
            .get_route_set_keys()
            .get(crypto_kind)
            .unwrap()
            .value;

        Ok(sr_pubkey)
    }

    /// Get a private route to use for the answer to question
    #[cfg_attr(
        feature = "verbose-tracing",
        instrument(level = "trace", skip(self), ret, err)
    )]
    pub fn get_private_route_for_safety_spec(
        &self,
        crypto_kind: CryptoKind,
        safety_spec: &SafetySpec,
        avoid_nodes: &[TypedKey],
    ) -> VeilidAPIResult<PublicKey> {
        let inner = &mut *self.inner.lock();
        let routing_table = self.unlocked_inner.routing_table.clone();
        let rti = &mut *routing_table.inner.write();

        self.get_route_for_safety_spec_inner(
            inner,
            rti,
            crypto_kind,
            safety_spec,
            Direction::Inbound.into(),
            avoid_nodes,
        )
    }

    fn assemble_private_route_inner(
        &self,
        key: &PublicKey,
        rsd: &RouteSpecDetail,
        optimized: bool,
    ) -> VeilidAPIResult<PrivateRoute> {
        let routing_table = self.unlocked_inner.routing_table.clone();
        let rti = &*routing_table.inner.read();

        // Ensure we get the crypto for it
        let crypto = routing_table.network_manager().crypto();
        let Some(vcrypto) = crypto.get(rsd.crypto_kind) else {
            apibail_invalid_argument!(
                "crypto not supported for route",
                "rsd.crypto_kind",
                rsd.crypto_kind
            );
        };

        // Ensure our network class is valid before attempting to assemble any routes
        if !rti.has_valid_network_class(RoutingDomain::PublicInternet) {
            apibail_try_again!(
                "unable to assemble route until we have a valid PublicInternet network class"
            );
        }

        // Make innermost route hop to our own node
        let mut route_hop = RouteHop {
            node: if optimized {
                let Some(node_id) = routing_table.node_ids().get(rsd.crypto_kind) else {
                    apibail_invalid_argument!(
                        "missing node id for crypto kind",
                        "rsd.crypto_kind",
                        rsd.crypto_kind
                    );
                };
                RouteNode::NodeId(node_id.value)
            } else {
                let pi = rti.get_own_peer_info(RoutingDomain::PublicInternet);
                RouteNode::PeerInfo(Box::new(pi))
            },
            next_hop: None,
        };

        // Loop for each hop
        let hop_count = rsd.hops.len();
        // iterate hops in private route order (reverse, but inside out)
        for h in 0..hop_count {
            let nonce = vcrypto.random_nonce();

            let blob_data = {
                let mut rh_message = ::capnp::message::Builder::new_default();
                let mut rh_builder = rh_message.init_root::<veilid_capnp::route_hop::Builder>();
                encode_route_hop(&route_hop, &mut rh_builder)?;
                builder_to_vec(rh_message)?
            };

            // Encrypt the previous blob ENC(nonce, DH(PKhop,SKpr))
            let dh_secret = vcrypto.cached_dh(&rsd.hops[h], &rsd.secret_key)?;
            let enc_msg_data =
                vcrypto.encrypt_aead(blob_data.as_slice(), &nonce, &dh_secret, None)?;
            let route_hop_data = RouteHopData {
                nonce,
                blob: enc_msg_data,
            };

            route_hop = RouteHop {
                node: if optimized {
                    // Optimized, no peer info, just the dht key
                    RouteNode::NodeId(rsd.hops[h])
                } else {
                    // Full peer info, required until we are sure the route has been fully established
                    let node_id = TypedKey::new(rsd.crypto_kind, rsd.hops[h]);
                    let pi = rti
                        .with_node_entry(node_id, |entry| {
                            entry.with(rti, |_rti, e| {
                                e.make_peer_info(RoutingDomain::PublicInternet)
                            })
                        })
                        .flatten();
                    if pi.is_none() {
                        apibail_internal!("peer info should exist for route but doesn't");
                    }
                    RouteNode::PeerInfo(Box::new(pi.unwrap()))
                },
                next_hop: Some(route_hop_data),
            }
        }

        let private_route = PrivateRoute {
            public_key: TypedKey::new(rsd.crypto_kind, *key),
            // add hop for 'FirstHop'
            hop_count: (hop_count + 1).try_into().unwrap(),
            hops: PrivateRouteHops::FirstHop(Box::new(route_hop)),
        };
        Ok(private_route)
    }

    /// Assemble a single private route for publication
    /// Returns a PrivateRoute object for an allocated private route key
    #[cfg_attr(
        feature = "verbose-tracing",
        instrument(level = "trace", skip(self), err)
    )]
    pub fn assemble_private_route(
        &self,
        key: &PublicKey,
        optimized: Option<bool>,
    ) -> VeilidAPIResult<PrivateRoute> {
        let inner = &*self.inner.lock();
        let Some(rsid) = inner.content.get_id_by_key(key) else {
            apibail_invalid_argument!("route key does not exist", "key", key);
        };
        let Some(rssd) = inner.content.get_detail(&rsid) else {
            apibail_internal!("route id does not exist");
        };

        // See if we can optimize this compilation yet
        // We don't want to include full nodeinfo if we don't have to
        let optimized = optimized.unwrap_or(
            rssd.get_stats().last_tested_ts.is_some()
                || rssd.get_stats().last_received_ts.is_some(),
        );

        let rsd = rssd
            .get_route_by_key(key)
            .expect("route key index is broken");

        self.assemble_private_route_inner(key, rsd, optimized)
    }

    /// Assemble private route set for publication
    /// Returns a vec of PrivateRoute objects for an allocated private route
    #[cfg_attr(
        feature = "verbose-tracing",
        instrument(level = "trace", skip(self), err)
    )]
    pub fn assemble_private_routes(
        &self,
        id: &RouteId,
        optimized: Option<bool>,
    ) -> VeilidAPIResult<Vec<PrivateRoute>> {
        let inner = &*self.inner.lock();
        let Some(rssd) = inner.content.get_detail(id) else {
            apibail_invalid_argument!("route id does not exist", "id", id);
        };

        // See if we can optimize this compilation yet
        // We don't want to include full nodeinfo if we don't have to
        let optimized = optimized.unwrap_or(
            rssd.get_stats().last_tested_ts.is_some()
                || rssd.get_stats().last_received_ts.is_some(),
        );

        let mut out = Vec::new();
        for (key, rsd) in rssd.iter_route_set() {
            out.push(self.assemble_private_route_inner(key, rsd, optimized)?);
        }
        Ok(out)
    }

    /// Import a remote private route for compilation
    /// It is safe to import the same route more than once and it will return the same route id
    /// Returns a route set id
    #[cfg_attr(
        feature = "verbose-tracing",
        instrument(level = "trace", skip(self, blob), ret, err)
    )]
    pub fn import_remote_private_route(&self, blob: Vec<u8>) -> VeilidAPIResult<RouteId> {
        let cur_ts = get_aligned_timestamp();

        // decode the pr blob
        let private_routes = RouteSpecStore::blob_to_private_routes(
            self.unlocked_inner.routing_table.crypto(),
            blob,
        )?;

        // make the route id
        let id = self.generate_remote_route_id(&private_routes)?;

        // validate the private routes
        let inner = &mut *self.inner.lock();
        for private_route in &private_routes {
            // ensure private route has first hop
            if !matches!(private_route.hops, PrivateRouteHops::FirstHop(_)) {
                apibail_generic!("private route must have first hop");
            }

            // ensure this isn't also an allocated route
            // if inner.content.get_id_by_key(&private_route.public_key.value).is_some() {
            //     bail!("should not import allocated route");
            // }
        }

        inner
            .cache
            .cache_remote_private_route(cur_ts, id, private_routes);

        Ok(id)
    }

    /// Release a remote private route that is no longer in use
    #[cfg_attr(
        feature = "verbose-tracing",
        instrument(level = "trace", skip(self), ret)
    )]
    pub fn release_remote_private_route(&self, id: RouteId) -> bool {
        let inner = &mut *self.inner.lock();
        inner.cache.remove_remote_private_route(id)
    }

    /// Get a route id for a route's public key
    pub fn get_route_id_for_key(&self, key: &PublicKey) -> Option<RouteId> {
        let inner = &mut *self.inner.lock();
        // Check for local route
        if let Some(id) = inner.content.get_id_by_key(key) {
            return Some(id);
        }

        // Check for remote route
        if let Some(rrid) = inner.cache.get_remote_private_route_id_by_key(key) {
            return Some(rrid);
        }

        None
    }

    /// Check to see if this remote (not ours) private route has seen our current node info yet
    /// This happens when you communicate with a private route without a safety route
    pub fn has_remote_private_route_seen_our_node_info(&self, key: &PublicKey) -> bool {
        let inner = &mut *self.inner.lock();

        // Check for local route. If this is not a remote private route,
        // we may be running a test and using our own local route as the destination private route.
        // In that case we definitely have already seen our own node info
        if inner.content.get_id_by_key(key).is_some() {
            return true;
        }

        if let Some(rrid) = inner.cache.get_remote_private_route_id_by_key(key) {
            let cur_ts = get_aligned_timestamp();
            if let Some(rpri) = inner.cache.peek_remote_private_route_mut(cur_ts, &rrid) {
                let our_node_info_ts = self
                    .unlocked_inner
                    .routing_table
                    .get_own_node_info_ts(RoutingDomain::PublicInternet);
                return rpri.has_seen_our_node_info_ts(our_node_info_ts);
            }
        }

        false
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
    ) -> VeilidAPIResult<()> {
        let our_node_info_ts = self
            .unlocked_inner
            .routing_table
            .get_own_node_info_ts(RoutingDomain::PublicInternet);

        let inner = &mut *self.inner.lock();

        // Check for local route. If this is not a remote private route
        // then we just skip the recording. We may be running a test and using
        // our own local route as the destination private route.
        if inner.content.get_id_by_key(key).is_some() {
            return Ok(());
        }

        if let Some(rrid) = inner.cache.get_remote_private_route_id_by_key(key) {
            if let Some(rpri) = inner.cache.peek_remote_private_route_mut(cur_ts, &rrid) {
                rpri.set_last_seen_our_node_info_ts(our_node_info_ts);
                return Ok(());
            }
        }

        apibail_invalid_argument!("private route is missing from store", "key", key);
    }

    /// Get the route statistics for any route we know about, local or remote
    pub fn with_route_stats<F, R>(&self, cur_ts: Timestamp, key: &PublicKey, f: F) -> Option<R>
    where
        F: FnOnce(&mut RouteStats) -> R,
    {
        let inner = &mut *self.inner.lock();

        // Check for stub route
        if self
            .unlocked_inner
            .routing_table
            .matches_own_node_id_key(key)
        {
            return None;
        }

        // Check for local route
        if let Some(rsid) = inner.content.get_id_by_key(key) {
            if let Some(rsd) = inner.content.get_detail_mut(&rsid) {
                return Some(f(rsd.get_stats_mut()));
            }
        }

        // Check for remote route
        if let Some(rrid) = inner.cache.get_remote_private_route_id_by_key(key) {
            if let Some(rpri) = inner.cache.peek_remote_private_route_mut(cur_ts, &rrid) {
                return Some(f(rpri.get_stats_mut()));
            }
        }

        None
    }

    /// Clear caches when local our local node info changes
    #[instrument(level = "trace", skip(self))]
    pub fn reset(&self) {
        log_rtab!(debug "flushing route spec store");

        let inner = &mut *self.inner.lock();

        // Clean up local allocated routes
        inner.content.reset_details();

        // Reset private route cache
        inner.cache.reset_remote_private_routes();
    }

    /// Mark route as published
    /// When first deserialized, routes must be re-published in order to ensure they remain
    /// in the RouteSpecStore.
    pub fn mark_route_published(&self, id: &RouteId, published: bool) -> VeilidAPIResult<()> {
        let inner = &mut *self.inner.lock();
        let Some(rssd) = inner.content.get_detail_mut(id) else {
            apibail_invalid_argument!("route does not exist", "id", id);
        };
        rssd.set_published(published);
        Ok(())
    }

    /// Process transfer statistics to get averages
    pub fn roll_transfers(&self, last_ts: Timestamp, cur_ts: Timestamp) {
        let inner = &mut *self.inner.lock();

        // Roll transfers for locally allocated routes
        inner.content.roll_transfers(last_ts, cur_ts);

        // Roll transfers for remote private routes
        inner.cache.roll_transfers(last_ts, cur_ts);
    }

    /// Convert private route list to binary blob
    pub fn private_routes_to_blob(private_routes: &[PrivateRoute]) -> VeilidAPIResult<Vec<u8>> {
        let mut buffer = vec![];

        // Serialize count
        let pr_count = private_routes.len();
        if pr_count > MAX_CRYPTO_KINDS {
            apibail_internal!("too many crypto kinds to encode blob");
        }
        let pr_count = pr_count as u8;
        buffer.push(pr_count);

        // Serialize stream of private routes
        for private_route in private_routes {
            let mut pr_message = ::capnp::message::Builder::new_default();
            let mut pr_builder = pr_message.init_root::<veilid_capnp::private_route::Builder>();

            encode_private_route(private_route, &mut pr_builder)
                .map_err(VeilidAPIError::internal)?;

            capnp::serialize_packed::write_message(&mut buffer, &pr_message)
                .map_err(RPCError::internal)?;
        }
        Ok(buffer)
    }

    /// Convert binary blob to private route vector
    pub fn blob_to_private_routes(
        crypto: Crypto,
        blob: Vec<u8>,
    ) -> VeilidAPIResult<Vec<PrivateRoute>> {
        // Deserialize count
        if blob.is_empty() {
            apibail_invalid_argument!(
                "not deserializing empty private route blob",
                "blob.is_empty",
                true
            );
        }

        let pr_count = blob[0] as usize;
        if pr_count > MAX_CRYPTO_KINDS {
            apibail_invalid_argument!("too many crypto kinds to decode blob", "blob[0]", pr_count);
        }

        // Deserialize stream of private routes
        let mut pr_slice = &blob[1..];
        let mut out = Vec::with_capacity(pr_count);
        for _ in 0..pr_count {
            let reader = capnp::serialize_packed::read_message(
                &mut pr_slice,
                capnp::message::ReaderOptions::new(),
            )
            .map_err(|e| VeilidAPIError::invalid_argument("failed to read blob", "e", e))?;

            let pr_reader = reader
                .get_root::<veilid_capnp::private_route::Reader>()
                .map_err(VeilidAPIError::internal)?;
            let private_route = decode_private_route(&pr_reader).map_err(|e| {
                VeilidAPIError::invalid_argument("failed to decode private route", "e", e)
            })?;
            private_route.validate(crypto.clone()).map_err(|e| {
                VeilidAPIError::invalid_argument("failed to validate private route", "e", e)
            })?;

            out.push(private_route);
        }

        // Don't trust the order of the blob
        out.sort_by(|a, b| a.public_key.cmp(&b.public_key));

        Ok(out)
    }

    /// Generate RouteId from typed key set of route public keys
    fn generate_allocated_route_id(&self, rssd: &RouteSetSpecDetail) -> VeilidAPIResult<RouteId> {
        let route_set_keys = rssd.get_route_set_keys();
        let crypto = self.unlocked_inner.routing_table.crypto();

        let mut idbytes = Vec::with_capacity(PUBLIC_KEY_LENGTH * route_set_keys.len());
        let mut best_kind: Option<CryptoKind> = None;
        for tk in route_set_keys.iter() {
            if best_kind.is_none()
                || compare_crypto_kind(&tk.kind, best_kind.as_ref().unwrap()) == cmp::Ordering::Less
            {
                best_kind = Some(tk.kind);
            }
            idbytes.extend_from_slice(&tk.value.bytes);
        }
        let Some(best_kind) = best_kind else {
            apibail_internal!("no compatible crypto kinds in route");
        };
        let vcrypto = crypto.get(best_kind).unwrap();

        Ok(RouteId::new(vcrypto.generate_hash(&idbytes).bytes))
    }

    /// Generate RouteId from set of private routes    
    fn generate_remote_route_id(
        &self,
        private_routes: &[PrivateRoute],
    ) -> VeilidAPIResult<RouteId> {
        let crypto = self.unlocked_inner.routing_table.crypto();

        let mut idbytes = Vec::with_capacity(PUBLIC_KEY_LENGTH * private_routes.len());
        let mut best_kind: Option<CryptoKind> = None;
        for private_route in private_routes {
            if best_kind.is_none()
                || compare_crypto_kind(&private_route.public_key.kind, best_kind.as_ref().unwrap())
                    == cmp::Ordering::Less
            {
                best_kind = Some(private_route.public_key.kind);
            }
            idbytes.extend_from_slice(&private_route.public_key.value.bytes);
        }
        let Some(best_kind) = best_kind else {
            apibail_internal!("no compatible crypto kinds in route");
        };
        let vcrypto = crypto.get(best_kind).unwrap();

        Ok(RouteId::new(vcrypto.generate_hash(&idbytes).bytes))
    }
}
