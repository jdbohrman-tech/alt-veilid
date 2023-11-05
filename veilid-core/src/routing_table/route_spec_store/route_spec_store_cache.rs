use super::*;

// Compiled route key for caching
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct CompiledRouteCacheKey {
    sr_pubkey: PublicKey,
    pr_pubkey: PublicKey,
}

/// Compiled route (safety route + private route)
#[derive(Clone, Debug)]
pub(crate) struct CompiledRoute {
    /// The safety route attached to the private route
    pub safety_route: SafetyRoute,
    /// The secret used to encrypt the message payload
    pub secret: SecretKey,
    /// The node ref to the first hop in the compiled route
    pub first_hop: NodeRef,
}

/// Ephemeral data used to help the RouteSpecStore operate efficiently
#[derive(Debug)]
pub(super) struct RouteSpecStoreCache {
    /// How many times nodes have been used
    used_nodes: HashMap<PublicKey, usize>,
    /// How many times nodes have been used at the terminal point of a route
    used_end_nodes: HashMap<PublicKey, usize>,
    /// Route spec hop cache, used to quickly disqualify routes
    hop_cache: HashSet<Vec<u8>>,
    /// Remote private routes we've imported and statistics
    remote_private_route_set_cache: LruCache<RouteId, RemotePrivateRouteInfo>,
    /// Remote private routes indexed by public key
    remote_private_routes_by_key: HashMap<PublicKey, RouteId>,
    /// Compiled route cache
    compiled_route_cache: LruCache<CompiledRouteCacheKey, SafetyRoute>,
    /// List of dead allocated routes
    dead_routes: Vec<RouteId>,
    /// List of dead remote routes
    dead_remote_routes: Vec<RouteId>,
}

impl RouteSpecStoreCache {
    /// add an allocated route set to our cache via its cache key
    pub fn add_to_cache(&mut self, rti: &RoutingTableInner, rssd: &RouteSetSpecDetail) {
        let cache_key = rssd.make_cache_key(rti);
        if !self.hop_cache.insert(cache_key) {
            panic!("route should never be inserted twice");
        }
        for (_pk, rsd) in rssd.iter_route_set() {
            for h in &rsd.hops {
                self.used_nodes
                    .entry(*h)
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
            self.used_end_nodes
                .entry(*rsd.hops.last().unwrap())
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
    }

    /// checks if an allocated route is in our cache
    pub fn contains_route(&self, cache_key: &Vec<u8>) -> bool {
        self.hop_cache.contains(cache_key)
    }

    /// removes an allocated route set from our cache
    pub fn remove_from_cache(
        &mut self,
        rti: &RoutingTableInner,
        id: RouteId,
        rssd: &RouteSetSpecDetail,
    ) -> bool {
        let cache_key = rssd.make_cache_key(rti);

        // Remove from hop cache
        if !self.hop_cache.remove(&cache_key) {
            return false;
        }
        for (pk, rsd) in rssd.iter_route_set() {
            for h in &rsd.hops {
                // Remove from used nodes cache
                match self.used_nodes.entry(*h) {
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
            match self.used_end_nodes.entry(*rsd.hops.last().unwrap()) {
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

            // Invalidate compiled route cache
            self.invalidate_compiled_route_cache(pk);
        }

        // Mark it as dead for the update if it wasn't automatically created
        if !rssd.is_automatic() {
            self.dead_routes.push(id);
        }

        true
    }

    /// calculate how many times a node with a particular node id set has been used anywhere in the path of our allocated routes
    pub fn get_used_node_count(&self, node_ids: &TypedKeyGroup) -> usize {
        node_ids.iter().fold(0usize, |acc, k| {
            acc + self.used_nodes.get(&k.value).cloned().unwrap_or_default()
        })
    }

    /// calculate how many times a node with a particular node id set has been used at the end of the path of our allocated routes
    pub fn get_used_end_node_count(&self, node_ids: &TypedKeyGroup) -> usize {
        node_ids.iter().fold(0usize, |acc, k| {
            acc + self
                .used_end_nodes
                .get(&k.value)
                .cloned()
                .unwrap_or_default()
        })
    }

    /// add remote private route to caches
    /// returns a remote private route set id
    fn add_remote_private_route(
        &mut self,
        id: RouteId,
        rprinfo: RemotePrivateRouteInfo,
    ) -> RouteId {
        // also store in id by key table
        for private_route in rprinfo.get_private_routes() {
            self.remote_private_routes_by_key
                .insert(private_route.public_key.value, id);
        }

        let mut dead = None;
        self.remote_private_route_set_cache.insert_with_callback(
            id,
            rprinfo,
            |dead_id, dead_rpri| {
                dead = Some((dead_id, dead_rpri));
            },
        );

        if let Some((dead_id, dead_rpri)) = dead {
            // If anything LRUs out, remove from the by-key table
            // Follow the same logic as 'remove_remote_private_route' here
            for dead_private_route in dead_rpri.get_private_routes() {
                self.remote_private_routes_by_key
                    .remove(&dead_private_route.public_key.value)
                    .unwrap();
                self.invalidate_compiled_route_cache(&dead_private_route.public_key.value);
            }
            self.dead_remote_routes.push(dead_id);
        }

        id
    }

    /// get count of remote private routes in cache
    pub fn get_remote_private_route_count(&self) -> usize {
        self.remote_private_route_set_cache.len()
    }

    /// iterate all of the remote private routes we have in the cache
    pub fn iter_remote_private_routes(
        &self,
    ) -> hashlink::linked_hash_map::Iter<RouteId, RemotePrivateRouteInfo> {
        self.remote_private_route_set_cache.iter()
    }

    /// remote private route cache accessor
    /// will LRU entries and may expire entries and not return them if they are stale
    pub fn get_remote_private_route(
        &mut self,
        cur_ts: Timestamp,
        id: &RouteId,
    ) -> Option<&RemotePrivateRouteInfo> {
        if let Some(rpri) = self.remote_private_route_set_cache.get_mut(id) {
            if !rpri.did_expire(cur_ts) {
                rpri.touch(cur_ts);
                return Some(rpri);
            }
        }
        None
    }

    /// mutable remote private route cache accessor
    /// will LRU entries and may expire entries and not return them if they are stale
    pub fn get_remote_private_route_mut(
        &mut self,
        cur_ts: Timestamp,
        id: &RouteId,
    ) -> Option<&mut RemotePrivateRouteInfo> {
        if let Some(rpri) = self.remote_private_route_set_cache.get_mut(id) {
            if !rpri.did_expire(cur_ts) {
                rpri.touch(cur_ts);
                return Some(rpri);
            }
        }
        None
    }

    /// mutable remote private route cache accessor without lru action
    /// will not LRU entries but may expire entries and not return them if they are stale
    pub fn peek_remote_private_route_mut(
        &mut self,
        cur_ts: Timestamp,
        id: &RouteId,
    ) -> Option<&mut RemotePrivateRouteInfo> {
        if let Some(rpri) = self.remote_private_route_set_cache.peek_mut(id) {
            if !rpri.did_expire(cur_ts) {
                rpri.touch(cur_ts);
                return Some(rpri);
            }
        }
        None
    }

    /// look up a remote private route id by one of the route public keys
    pub fn get_remote_private_route_id_by_key(&self, key: &PublicKey) -> Option<RouteId> {
        self.remote_private_routes_by_key.get(key).cloned()
    }

    /// get or create a remote private route cache entry
    /// may LRU and/or expire other cache entries to make room for the new one
    /// or update an existing entry with the same private route set
    /// returns the route set id
    pub fn cache_remote_private_route(
        &mut self,
        cur_ts: Timestamp,
        id: RouteId,
        private_routes: Vec<PrivateRoute>,
    ) {
        // get id for this route set
        if let Some(rpri) = self.get_remote_private_route_mut(cur_ts, &id) {
            if rpri.did_expire(cur_ts) {
                // Start fresh if this had expired
                rpri.unexpire(cur_ts);
            } else {
                // If not expired, just mark as being used
                rpri.touch(cur_ts);
            }
        } else {
            // New remote private route cache entry
            let rpri = RemotePrivateRouteInfo::new(private_routes, cur_ts);

            self.add_remote_private_route(id, rpri);
            if self.peek_remote_private_route_mut(cur_ts, &id).is_none() {
                panic!("remote private route should exist");
            };
        };
    }

    /// remove a remote private route from the cache
    pub fn remove_remote_private_route(&mut self, id: RouteId) -> bool {
        let Some(rprinfo) = self.remote_private_route_set_cache.remove(&id) else {
            return false;
        };
        for private_route in rprinfo.get_private_routes() {
            self.remote_private_routes_by_key
                .remove(&private_route.public_key.value)
                .unwrap();
            self.invalidate_compiled_route_cache(&private_route.public_key.value);
        }
        self.dead_remote_routes.push(id);
        true
    }

    /// Stores a compiled 'safety + private' route so we don't have to compile it again later
    pub fn add_to_compiled_route_cache(&mut self, pr_pubkey: PublicKey, safety_route: SafetyRoute) {
        let key = CompiledRouteCacheKey {
            sr_pubkey: safety_route.public_key.value,
            pr_pubkey,
        };

        if let Some(v) = self.compiled_route_cache.insert(key, safety_route) {
            log_rtab!(error "route cache already contained key: sr_pubkey={:?}, pr_pubkey={:?}", v.public_key, pr_pubkey);
        }
    }

    /// Looks up an existing compiled route from the safety and private route components
    pub fn lookup_compiled_route_cache(
        &mut self,
        sr_pubkey: PublicKey,
        pr_pubkey: PublicKey,
    ) -> Option<SafetyRoute> {
        let key = CompiledRouteCacheKey {
            sr_pubkey,
            pr_pubkey,
        };
        self.compiled_route_cache.get(&key).cloned()
    }

    /// When routes are dropped, they should be removed from the compiled route cache
    fn invalidate_compiled_route_cache(&mut self, dead_key: &PublicKey) {
        let mut dead_entries = Vec::new();
        for (k, _v) in self.compiled_route_cache.iter() {
            if k.sr_pubkey == *dead_key || k.pr_pubkey == *dead_key {
                dead_entries.push(k.clone());
            }
        }
        for d in dead_entries {
            self.compiled_route_cache.remove(&d);
        }
    }

    /// Take the dead local and remote routes so we can update clients
    pub fn take_dead_routes(&mut self) -> Option<(Vec<RouteId>, Vec<RouteId>)> {
        if self.dead_routes.is_empty() && self.dead_remote_routes.is_empty() {
            // Nothing to do
            return None;
        }
        let dead_routes = core::mem::take(&mut self.dead_routes);
        let dead_remote_routes = core::mem::take(&mut self.dead_remote_routes);
        Some((dead_routes, dead_remote_routes))
    }

    /// Clean up imported remote routes
    /// Resets statistics for when our node info changes
    pub fn reset_remote_private_routes(&mut self) {
        // Restart stats for routes so we test the route again
        for (_k, v) in self.remote_private_route_set_cache.iter_mut() {
            v.get_stats_mut().reset();
        }
    }

    /// Roll transfer statistics
    pub fn roll_transfers(&mut self, last_ts: Timestamp, cur_ts: Timestamp) {
        for (_k, v) in self.remote_private_route_set_cache.iter_mut() {
            v.get_stats_mut().roll_transfers(last_ts, cur_ts);
        }
    }
}

impl Default for RouteSpecStoreCache {
    fn default() -> Self {
        Self {
            used_nodes: Default::default(),
            used_end_nodes: Default::default(),
            hop_cache: Default::default(),
            remote_private_route_set_cache: LruCache::new(REMOTE_PRIVATE_ROUTE_CACHE_SIZE),
            remote_private_routes_by_key: HashMap::new(),
            compiled_route_cache: LruCache::new(COMPILED_ROUTE_CACHE_SIZE),
            dead_routes: Default::default(),
            dead_remote_routes: Default::default(),
        }
    }
}
