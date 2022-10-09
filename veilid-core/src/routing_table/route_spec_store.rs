use super::*;
use crate::veilid_api::*;
use serde::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct RouteSpecDetail {
    /// The actual route spec
    #[serde(with = "arc_serialize")]
    route_spec: Arc<RouteSpec>,
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
    #[serde(skip)]
    published: bool,
    /// Timestamp of when the route was created
    timestamp: u64,
}

/// The core representation of the RouteSpecStore that can be serialized
#[derive(Debug, Serialize, Deserialize)]
pub struct RouteSpecStoreContent {
    /// All of the routes we have allocated so far
    details: HashMap<DHTKey, RouteSpecDetail>,
}

/// Ephemeral data used to help the RouteSpecStore operate efficiently
#[derive(Debug, Default)]
pub struct RouteSpecStoreCache {
    /// The fastest routes by latency
    fastest_routes: Vec<DHTKey>,
    /// The most reliable routes by node lifetime longevity
    reliable_routes: Vec<DHTKey>,
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

fn route_spec_to_hop_cache(spec: Arc<RouteSpec>) -> Vec<u8> {
    let mut cache: Vec<u8> = Vec::with_capacity(spec.hops.len() * DHT_KEY_LENGTH);
    for hop in spec.hops {
        cache.extend_from_slice(&hop.dial_info.node_id.key.bytes);
    }
    cache
}

fn node_sublist_to_hop_cache(
    nodes: &[(DHTKey, Arc<BucketEntry>)],
    start: usize,
    len: usize,
) -> Vec<u8> {
    let mut cache: Vec<u8> = Vec::with_capacity(len * DHT_KEY_LENGTH);
    for node in &nodes[start..start + len] {
        cache.extend_from_slice(&node.0.bytes)
    }
    cache
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

    pub fn from_cbor(
        routing_table: RoutingTable,
        cbor: &[u8],
    ) -> Result<RouteSpecStore, VeilidAPIError> {
        let content: RouteSpecStoreContent = serde_cbor::from_slice(cbor)
            .map_err(|e| VeilidAPIError::parse_error("invalid route spec store content", e))?;
        let rss = RouteSpecStore {
            content,
            cache: Default::default(),
        };
        rss.rebuild_cache();
        Ok(rss)
    }

    pub fn to_cbor(&self) -> Vec<u8> {
        serde_cbor::to_vec(&self.content).unwrap()
    }

    fn rebuild_cache(&mut self) {
        //
    }

    fn detail_mut(&mut self, spec: Arc<RouteSpec>) -> &mut RouteSpecDetail {
        self.content.details.get_mut(&spec.public_key).unwrap()
    }

    /// Create a new route
    /// Prefers nodes that are not currently in use by another route
    /// The route is not yet tested for its reachability
    /// Returns None if no route could be allocated at this time
    pub fn allocate_route(
        &mut self,
        routing_table: RoutingTable,
        reliable: bool,
        hop_count: usize,
    ) -> Option<Arc<RouteSpec>> {
        use core::cmp::Ordering;

        let max_route_hop_count = {
            let config = routing_table.network_manager().config();
            let c = config.get();
            let max_route_hop_count = c.network.rpc.max_route_hop_count;
            max_route_hop_count.into()
        };

        if hop_count < 2 {
            log_rtab!(error "Not allocating route less than two hops in length");
            return None;
        }

        if hop_count > max_route_hop_count {
            log_rtab!(error "Not allocating route longer than max route hop count");
            return None;
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
            let cmpout = v1.1.unwrap().with(rti, |rti, e1| {
                v2.1.unwrap().with(rti, |_rti, e2| {
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
        let mut nodes = routing_table
            .find_peers_with_sort_and_filter(node_count, cur_ts, filter, compare, transform);

        // If we couldn't find enough nodes, wait until we have more nodes in the routing table
        if nodes.len() < hop_count {
            log_rtab!(debug "Not enough nodes to construct route at this time. Try again later.");
            return None;
        }

        // Now go through nodes and try to build a route we haven't seen yet
        let mut route_nodes = None;
        for start in 0..(nodes.len() - hop_count) {
            // Get the route cache key
            let key = node_sublist_to_hop_cache(&nodes, start, hop_count);

            // try each route until we find a unique one
            if !self.cache.hop_cache.contains(&key) {
                route_nodes = Some(&nodes[start..start + hop_count]);
                break;
            }
        }
        if route_nodes.is_none() {
            return None;
        }
        let route_node = route_nodes.unwrap();

        // Got a unique route, lets build the detail, register it, and return it
        let hops: Vec<RouteHopSpec> = route_node
            .into_iter()
            .map(|v| RouteHopSpec {
                dial_info: NodeDialInfo {
                    node_id: NodeId::new(v.0),
                    dial_info: xxx,
                },
            })
            .collect();

        let (public_key, secret_key) = generate_secret();
        let route_spec = Arc::new(RouteSpec {
            public_key,
            secret_key,
            hops,
        });

        let rsd = RouteSpecDetail {
            route_spec,
            transfer_stats_down_up: Default::default(),
            latency_stats: Default::default(),
            latency_stats_accounting: Default::default(),
            transfer_stats_accounting: Default::default(),
            published: false,
            timestamp: cur_ts,
        };

        None
    }

    pub fn release_route(&mut self, spec: Arc<RouteSpec>) {}

    pub fn best_route(&mut self, reliable: bool) -> Arc<RouteSpec> {}

    /// Mark route as published
    /// When first deserialized, routes must be re-published in order to ensure they remain
    /// in the RouteSpecStore.
    pub fn publish_route(&mut self, spec: Arc<RouteSpec>) {
        //compile private route here?
    }

    pub fn record_latency(
        &mut self,
        spec: Arc<RouteSpec>,
        latency: u64,
    ) -> veilid_api::LatencyStats {
    }

    pub fn add_down(&mut self, spec: Arc<RouteSpec>, bytes: u64) {
        self.current_transfer.down += bytes;
    }

    pub fn add_up(&mut self, spec: Arc<RouteSpec>, bytes: u64) {}

    pub fn roll_transfers(&mut self) {
        //
    }
}
