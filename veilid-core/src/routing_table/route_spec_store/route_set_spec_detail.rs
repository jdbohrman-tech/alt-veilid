use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct RouteSpecDetail {
    /// Crypto kind
    pub crypto_kind: CryptoKind,
    /// Secret key
    pub secret_key: SecretKey,
    /// Route hops (node id keys)
    pub hops: Vec<PublicKey>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct RouteSetSpecDetail {
    /// Route set per crypto kind
    route_set: BTreeMap<PublicKey, RouteSpecDetail>,
    /// Route noderefs
    #[serde(skip)]
    hop_node_refs: Vec<NodeRef>,
    /// Published private route, do not reuse for ephemeral routes
    /// Not serialized because all routes should be re-published when restarting
    #[serde(skip)]
    published: bool,
    /// Directions this route is guaranteed to work in
    directions: DirectionSet,
    /// Stability preference (prefer reliable nodes over faster)
    stability: Stability,
    /// Sequencing capability (connection oriented protocols vs datagram)
    can_do_sequenced: bool,
    /// Stats
    stats: RouteStats,
}

impl RouteSetSpecDetail {
    pub fn new(
        cur_ts: Timestamp,
        route_set: BTreeMap<PublicKey, RouteSpecDetail>,
        hop_node_refs: Vec<NodeRef>,
        directions: DirectionSet,
        stability: Stability,
        can_do_sequenced: bool,
    ) -> Self {
        Self {
            route_set,
            hop_node_refs,
            published: false,
            directions,
            stability,
            can_do_sequenced,
            stats: RouteStats::new(cur_ts),
        }
    }
    pub fn get_route_by_key(&self, key: &PublicKey) -> Option<&RouteSpecDetail> {
        self.route_set.get(key)
    }
    pub fn get_route_set_keys(&self) -> TypedKeyGroup {
        let mut tks = TypedKeyGroup::new();
        for (k, v) in &self.route_set {
            tks.add(TypedKey::new(v.crypto_kind, *k));
        }
        tks
    }
    pub fn get_best_route_set_key(&self) -> Option<PublicKey> {
        self.get_route_set_keys().best().map(|k| k.value)
    }
    pub fn set_hop_node_refs(&mut self, node_refs: Vec<NodeRef>) {
        self.hop_node_refs = node_refs;
    }
    pub fn iter_route_set(
        &self,
    ) -> alloc::collections::btree_map::Iter<PublicKey, RouteSpecDetail> {
        self.route_set.iter()
    }
    pub fn get_stats(&self) -> &RouteStats {
        &self.stats
    }
    pub fn get_stats_mut(&mut self) -> &mut RouteStats {
        &mut self.stats
    }
    pub fn is_published(&self) -> bool {
        self.published
    }
    pub fn set_published(&mut self, published: bool) {
        self.published = published;
    }
    pub fn hop_count(&self) -> usize {
        self.hop_node_refs.len()
    }
    pub fn hop_node_ref(&self, idx: usize) -> Option<NodeRef> {
        self.hop_node_refs.get(idx).cloned()
    }
    pub fn get_stability(&self) -> Stability {
        self.stability
    }
    pub fn get_directions(&self) -> DirectionSet {
        self.directions
    }
    pub fn is_sequencing_match(&self, sequencing: Sequencing) -> bool {
        match sequencing {
            Sequencing::NoPreference => true,
            Sequencing::PreferOrdered => true,
            Sequencing::EnsureOrdered => self.can_do_sequenced,
        }
    }
    pub fn contains_nodes(&self, nodes: &[TypedKey]) -> bool {
        for tk in nodes {
            for rsd in self.route_set.values() {
                if rsd.crypto_kind == tk.kind && rsd.hops.contains(&tk.value) {
                    return true;
                }
            }
        }
        false
    }

    /// Generate a key for the cache that can be used to uniquely identify this route's contents
    pub fn make_cache_key(&self, rti: &RoutingTableInner) -> Vec<u8> {
        let hops = &self.hop_node_refs;
        let mut cache: Vec<u8> = Vec::with_capacity(hops.len() * PUBLIC_KEY_LENGTH);
        for hop in hops {
            cache.extend_from_slice(&hop.locked(rti).best_node_id().value.bytes);
        }
        cache
    }
}
