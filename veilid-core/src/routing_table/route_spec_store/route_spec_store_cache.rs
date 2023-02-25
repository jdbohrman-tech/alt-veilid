use super::*;

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

/// Ephemeral data used to help the RouteSpecStore operate efficiently
#[derive(Debug)]
pub struct RouteSpecStoreCache {
    /// How many times nodes have been used
    used_nodes: HashMap<PublicKey, usize>,
    /// How many times nodes have been used at the terminal point of a route
    used_end_nodes: HashMap<PublicKey, usize>,
    /// Route spec hop cache, used to quickly disqualify routes
    hop_cache: HashSet<Vec<u8>>,
    /// Remote private routes we've imported and statistics
    remote_private_route_set_cache: LruCache<RemotePrivateRouteId, RemotePrivateRouteInfo>,
    /// Remote private routes indexed by public key
    remote_private_routes_by_key: HashMap<PublicKey, RemotePrivateRouteId>,
    /// Compiled route cache
    compiled_route_cache: LruCache<CompiledRouteCacheKey, SafetyRoute>,
    /// List of dead allocated routes
    dead_routes: Vec<RouteSetSpecId>,
    /// List of dead remote routes
    dead_remote_routes: Vec<RemotePrivateRouteId>,
}

impl RouteSpecStoreCache {
    /// add an allocated route set to our cache via its cache key
    pub fn add_to_cache(&mut self, rssd: &RouteSetSpecDetail) {
        let cache_key = rssd.make_cache_key();
        if !self.hop_cache.insert(cache_key) {
            panic!("route should never be inserted twice");
        }
        for (pk, rsd) in rssd.iter_route_set() {
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
    pub fn remove_from_cache(&mut self, rssd: &RouteSetSpecDetail) -> bool {
        let cache_key = rssd.make_cache_key();

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
        }

        // Mark it as dead for the update
        self.dead_routes.push(rssd.make_id());

        true
    }

    /// calculate how many times a node with a particular node id set has been used anywhere in the path of our allocated routes
    pub fn get_used_node_count(&self, node_ids: &TypedKeySet) -> usize {
        node_ids.iter().fold(0usize, |acc, k| {
            acc + self.used_nodes.get(&k.key).cloned().unwrap_or_default()
        })
    }

    /// calculate how many times a node with a particular node id set has been used at the end of the path of our allocated routes
    pub fn get_used_end_node_count(&self, node_ids: &TypedKeySet) -> usize {
        node_ids.iter().fold(0usize, |acc, k| {
            acc + self.used_end_nodes.get(&k.key).cloned().unwrap_or_default()
        })
    }

    /// generate unique remote private route set id for a remote private route set
    fn make_remote_private_route_id(private_routes: &[PrivateRoute]) -> String {
        let mut idbytes = [0u8; 16];
        for (pk, _) in &rprinfo.private_routes {
            for (i, x) in pk.bytes.iter().enumerate() {
                idbytes[i % 16] ^= *x;
            }
        }
        let id = format!(
            "{:08x}-{:04x}-{:04x}-{:04x}-{:08x}{:04x}",
            u32::from_be_bytes(idbytes[0..4].try_into().expect("32 bits")),
            u16::from_be_bytes(idbytes[4..6].try_into().expect("16 bits")),
            u16::from_be_bytes(idbytes[6..8].try_into().expect("16 bits")),
            u16::from_be_bytes(idbytes[8..10].try_into().expect("16 bits")),
            u32::from_be_bytes(idbytes[10..14].try_into().expect("32 bits")),
            u16::from_be_bytes(idbytes[14..16].try_into().expect("16 bits"))
        );
        id
    }

    /// add remote private route to caches
    /// returns a remote private route set id
    fn add_remote_private_route(
        &mut self,
        rprinfo: RemotePrivateRouteInfo,
    ) -> RemotePrivateRouteId {
        let id = Self::make_remote_private_route_id(rprinfo.get_private_routes());

        // also store in id by key table
        for (pk, _) in rprinfo.get_private_routes() {
            self.remote_private_routes_by_key.insert(*pk, id.clone());
        }
        self.remote_private_route_set_cache
            .insert(id.clone(), rprinfo, |dead_id, dead_rpri| {
                // If anything LRUs out, remove from the by-key table
                for (dead_pk, _) in dead_rpri.get_private_routes() {
                    self.remote_private_routes_by_key.remove(&dead_pk).unwrap();
                }
                self.dead_remote_routes.push(dead_id);
            });

        id
    }

    /// remote private route cache accessor
    fn get_remote_private_route(
        &mut self,
        id: &RemotePrivateRouteId,
    ) -> Option<&RemotePrivateRouteInfo> {
        self.remote_private_route_set_cache.get(id)
    }
    /// mutable remote private route cache accessor
    fn get_remote_private_route_mut(
        &mut self,
        id: &RemotePrivateRouteId,
    ) -> Option<&mut RemotePrivateRouteInfo> {
        self.remote_private_route_set_cache.get_mut(id)
    }
    /// mutable remote private route cache accessor without lru action
    fn peek_remote_private_route_mut(
        &mut self,
        id: &RemotePrivateRouteId,
    ) -> Option<&mut RemotePrivateRouteInfo> {
        self.remote_private_route_set_cache.peek_mut(id)
    }

    /// look up a remote private route id by one of the route public keys
    pub fn get_remote_private_route_id_by_key(
        &self,
        key: &PublicKey,
    ) -> Option<RemotePrivateRouteId> {
        self.remote_private_routes_by_key.get(key).cloned()
    }

    /// get or create a remote private route cache entry
    /// may LRU and/or expire other cache entries to make room for the new one
    /// or update an existing entry with the same private route set
    /// returns the route set id
    pub fn import_remote_private_route(
        &mut self,
        cur_ts: Timestamp,
        private_routes: Vec<PrivateRoute>,
    ) -> RemotePrivateRouteId {
        // get id for this route set
        let id = RouteSpecStoreCache::make_remote_private_route_id(&private_routes);
        let rpri = if let Some(rpri) = self.get_remote_private_route_mut(&id) {
            if rpri.did_expire(cur_ts) {
                // Start fresh if this had expired
                rpri.unexpire(cur_ts);
            } else {
                // If not expired, just mark as being used
                rpri.touch(cur_ts);
            }
        } else {
            let rpri = RemotePrivateRouteInfo {
                // New remote private route cache entry
                private_routes,
                last_seen_our_node_info_ts: Timestamp::new(0),
                last_touched_ts: cur_ts,
                stats: RouteStats::new(cur_ts),
            };
            let new_id = self.add_remote_private_route(rpri);
            assert_eq!(id, new_id);
            if self.get_remote_private_route_mut(&id).is_none() {
                bail!("remote private route should exist");
            };
        };
        id
    }

    /// remove a remote private route from the cache
    pub fn remove_remote_private_route(&mut self, id: &RemotePrivateRouteId) -> bool {
        let Some(rprinfo) = self.remote_private_route_set_cache.remove(id) else {
            return false;
        };
        for (pk, _) in rprinfo.get_private_routes() {
            self.remote_private_routes_by_key.remove(&pk).unwrap();
        }
        self.dead_remote_routes.push(id.clone());
        true
    }

    /// get an existing remote private route cache entry
    /// will LRU entries and may expire entries and not return them if they are stale
    /// calls a callback with the remote private route info if returned
    pub fn with_get_remote_private_route<F, R>(
        &mut self,
        cur_ts: Timestamp,
        id: &RemotePrivateRouteId,
        f: F,
    ) -> Option<R>
    where
        F: FnOnce(&mut RemotePrivateRouteInfo) -> R,
    {
        if let Some(rpri) = self.get_remote_private_route_mut(&id) {
            if !rpri.did_expire(cur_ts) {
                rpri.touch(cur_ts);
                return Some(f(rpri));
            }
        }
        self.remove_remote_private_route(&id);
        None
    }

    // peek a remote private route cache entry
    // will not LRU entries but may expire entries and not return them if they are stale
    /// calls a callback with the remote private route info if returned
    pub fn with_peek_remote_private_route<F, R>(
        &mut self,
        cur_ts: Timestamp,
        id: &RemotePrivateRouteId,
        f: F,
    ) -> Option<R>
    where
        F: FnOnce(&mut RemotePrivateRouteInfo) -> R,
    {
        if let Some(rpri) = self.peek_remote_private_route_mut(&id) {
            if !rpri.did_expire(cur_ts) {
                rpri.touch(cur_ts);
                return Some(f(rpri));
            }
        }
        self.remove_remote_private_route(&id);
        None
    }

    /// Take the dead local and remote routes so we can update clients
    pub fn take_dead_routes(&mut self) -> (Vec<RouteSetSpecId>, Vec<RemotePrivateRouteId>) {
        if self.dead_routes.is_empty() && self.dead_remote_routes.is_empty() {
            // Nothing to do
            return;
        }
        let dead_routes = core::mem::take(&mut self.dead_routes);
        let dead_remote_routes = core::mem::take(&mut self.dead_remote_routes);
        (dead_routes, dead_remote_routes)
    }

    /// Clean up imported remote routes
    /// Resets statistics for when our node info changes
    pub fn reset_details(&mut self) {
        for (_k, v) in self.remote_private_route_cache {
            // Restart stats for routes so we test the route again
            v.stats.reset();
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
