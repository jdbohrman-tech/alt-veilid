use super::*;

#[derive(Clone, Debug, RkyvArchive, RkyvSerialize, RkyvDeserialize)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct RouteSpecDetail {
    /// Crypto kind
    pub crypto_kind: CryptoKind,
    /// Secret key
    #[with(Skip)]
    pub secret_key: SecretKey,
    /// Route hops (node id keys)
    pub hops: Vec<PublicKey>,
}

#[derive(Clone, Debug, RkyvArchive, RkyvSerialize, RkyvDeserialize)]
#[archive_attr(repr(C), derive(CheckBytes))]
pub struct RouteSetSpecDetail {
    /// Route set per crypto kind
    route_set: BTreeMap<PublicKey, RouteSpecDetail>,
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

impl RouteSetSpecDetail {
    pub fn get_route_by_key(&self, key: &PublicKey) -> Option<&RouteSpecDetail> {
        self.route_set.get(key)
    }
    pub fn get_route_by_key_mut(&mut self, key: &PublicKey) -> Option<&mut RouteSpecDetail> {
        self.route_set.get_mut(key)
    }
    pub fn get_route_set_keys(&self) -> TypedKeySet {
        let mut tks = TypedKeySet::new();
        for (k, v) in &self.route_set {
            tks.add(TypedKey::new(v.crypto_kind, *k));
        }
        tks
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
        self.published = self.published;
    }
    pub fn hop_count(&self) -> usize {
        self.hop_node_refs.len()
    }
    pub fn get_stability(&self) -> Stability {
        self.stability
    }
    pub fn is_sequencing_match(&self, sequencing: Sequencing) -> bool {
        match sequencing {
            Sequencing::NoPreference => true,
            Sequencing::PreferOrdered => true,
            Sequencing::EnsureOrdered => self.can_do_sequenced,
        }
    }

    /// Generate a key for the cache that can be used to uniquely identify this route's contents
    pub fn make_cache_key(&self) -> Vec<u8> {
        let hops = &self.hop_node_refs;
        let mut cache: Vec<u8> = Vec::with_capacity(hops.len() * PUBLIC_KEY_LENGTH);
        for hop in hops {
            cache.extend_from_slice(&hop.best_node_id().key.bytes);
        }
        cache
    }

    /// Generate a user-facing identifier for this allocated route
    pub fn make_id(&self) -> RouteSetSpecId {
        let mut idbytes = [0u8; 16];
        for (pk, _) in self.route_set.iter() {
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
}
