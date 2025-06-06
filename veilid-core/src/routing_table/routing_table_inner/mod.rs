mod routing_domains;

use super::*;
pub use routing_domains::*;

use weak_table::PtrWeakHashSet;

impl_veilid_log_facility!("rtab");

pub const RECENT_PEERS_TABLE_SIZE: usize = 64;

// Critical sections
pub const LOCK_TAG_TICK: &str = "TICK";

/// Keeping track of how many entries we have of each type we care about
#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct LiveEntryCounts {
    /// A rough count of the entries in the table per routing domain and crypto kind with any capabilities
    pub any_capabilities: EntryCounts,
    /// A rough count of the entries in the table per routing domain and crypto kind with CONNECTIVITY_CAPABILITIES
    pub connectivity_capabilities: EntryCounts,
    /// A rough count of the entries in the table per routing domain and crypto kind with DISTANCE_METRIC_CAPABILITIES
    pub distance_metric_capabilities: EntryCounts,
}
pub type EntryCounts = BTreeMap<(RoutingDomain, CryptoKind), usize>;

#[derive(Debug)]
pub struct NodeRelativePerformance {
    pub percentile: f32,
    pub node_index: usize,
    pub node_count: usize,
}

//////////////////////////////////////////////////////////////////////////

/// RoutingTable rwlock-internal data
#[must_use]
pub struct RoutingTableInner {
    /// Convenience accessor for the global component registry
    pub(super) registry: VeilidComponentRegistry,
    /// Routing table buckets that hold references to entries, per crypto kind
    pub(super) buckets: BTreeMap<CryptoKind, Vec<Bucket>>,
    /// A weak set of all the entries we have in the buckets for faster iteration
    pub(super) all_entries: PtrWeakHashSet<Weak<BucketEntry>>,
    /// Summary of how many entries we have of capability combinations we care about
    pub(super) live_entry_counts: Arc<LiveEntryCounts>,
    /// The public internet routing domain
    pub(super) public_internet_routing_domain: PublicInternetRoutingDomainDetail,
    /// The dial info we use on the local network
    pub(super) local_network_routing_domain: LocalNetworkRoutingDomainDetail,
    /// Interim accounting mechanism for this node's RPC latency to any other node
    pub(super) self_latency_stats_accounting: LatencyStatsAccounting,
    /// Interim accounting mechanism for the total bandwidth to/from this node
    pub(super) self_transfer_stats_accounting: TransferStatsAccounting,
    /// Statistics about the total bandwidth to/from this node
    pub(super) self_transfer_stats: TransferStatsDownUp,
    /// Peers we have recently communicated with
    pub(super) recent_peers: LruCache<TypedNodeId, RecentPeersEntry>,
    /// Async tagged critical sections table
    /// Tag: "tick" -> in ticker
    pub(super) critical_sections: AsyncTagLockTable<&'static str>,
    /// Last time we pinged checked the active watches
    pub(super) opt_active_watch_keepalive_ts: Option<Timestamp>,
}

impl_veilid_component_registry_accessor!(RoutingTableInner);

impl RoutingTableInner {
    pub(super) fn new(registry: VeilidComponentRegistry) -> RoutingTableInner {
        RoutingTableInner {
            registry: registry.clone(),
            buckets: BTreeMap::new(),
            public_internet_routing_domain: PublicInternetRoutingDomainDetail::new(
                registry.clone(),
            ),
            local_network_routing_domain: LocalNetworkRoutingDomainDetail::new(registry.clone()),
            all_entries: PtrWeakHashSet::new(),
            live_entry_counts: Default::default(),
            self_latency_stats_accounting: LatencyStatsAccounting::new(),
            self_transfer_stats_accounting: TransferStatsAccounting::new(),
            self_transfer_stats: TransferStatsDownUp::default(),
            recent_peers: LruCache::new(RECENT_PEERS_TABLE_SIZE),
            critical_sections: AsyncTagLockTable::new(),
            opt_active_watch_keepalive_ts: None,
        }
    }

    pub fn bucket_entry_count(&self) -> usize {
        self.all_entries.len()
    }

    pub fn transfer_stats_accounting(&mut self) -> &mut TransferStatsAccounting {
        &mut self.self_transfer_stats_accounting
    }
    pub fn latency_stats_accounting(&mut self) -> &mut LatencyStatsAccounting {
        &mut self.self_latency_stats_accounting
    }

    pub fn routing_domain_for_address(&self, address: Address) -> Option<RoutingDomain> {
        for rd in RoutingDomain::all() {
            let can_contain = self.with_routing_domain(rd, |rdd| rdd.can_contain_address(address));
            if can_contain {
                return Some(rd);
            }
        }
        None
    }

    pub fn with_routing_domain<F, R>(&self, domain: RoutingDomain, f: F) -> R
    where
        F: FnOnce(&dyn RoutingDomainDetail) -> R,
    {
        match domain {
            RoutingDomain::PublicInternet => f(&self.public_internet_routing_domain),
            RoutingDomain::LocalNetwork => f(&self.local_network_routing_domain),
        }
    }

    pub fn relay_node(&self, domain: RoutingDomain) -> Option<FilteredNodeRef> {
        self.with_routing_domain(domain, |rdd| rdd.relay_node())
    }

    pub fn relay_node_last_keepalive(&self, domain: RoutingDomain) -> Option<Timestamp> {
        self.with_routing_domain(domain, |rdd| rdd.relay_node_last_keepalive())
    }
    pub fn relay_node_last_optimized(&self, domain: RoutingDomain) -> Option<Timestamp> {
        self.with_routing_domain(domain, |rdd| rdd.relay_node_last_optimized())
    }

    pub fn set_relay_node_last_keepalive(&mut self, domain: RoutingDomain, ts: Timestamp) {
        match domain {
            RoutingDomain::PublicInternet => self
                .public_internet_routing_domain
                .set_relay_node_last_keepalive(Some(ts)),
            RoutingDomain::LocalNetwork => self
                .local_network_routing_domain
                .set_relay_node_last_keepalive(Some(ts)),
        };
    }
    pub fn set_relay_node_last_optimized(&mut self, domain: RoutingDomain, ts: Timestamp) {
        match domain {
            RoutingDomain::PublicInternet => self
                .public_internet_routing_domain
                .set_relay_node_last_optimized(Some(ts)),
            RoutingDomain::LocalNetwork => self
                .local_network_routing_domain
                .set_relay_node_last_optimized(Some(ts)),
        };
    }

    #[expect(dead_code)]
    pub fn has_dial_info(&self, domain: RoutingDomain) -> bool {
        self.with_routing_domain(domain, |rdd| !rdd.dial_info_details().is_empty())
    }

    pub fn dial_info_details(&self, domain: RoutingDomain) -> Vec<DialInfoDetail> {
        self.with_routing_domain(domain, |rdd| rdd.dial_info_details().clone())
    }

    pub fn first_filtered_dial_info_detail(
        &self,
        routing_domain_set: RoutingDomainSet,
        filter: &DialInfoFilter,
    ) -> Option<DialInfoDetail> {
        if filter.is_dead() || routing_domain_set.is_empty() {
            return None;
        }
        for routing_domain in routing_domain_set {
            let did = self.with_routing_domain(routing_domain, |rdd| {
                for did in rdd.dial_info_details() {
                    if did.matches_filter(filter) {
                        return Some(did.clone());
                    }
                }
                None
            });
            if did.is_some() {
                return did;
            }
        }
        None
    }

    pub fn all_filtered_dial_info_details(
        &self,
        routing_domain_set: RoutingDomainSet,
        filter: &DialInfoFilter,
    ) -> Vec<DialInfoDetail> {
        let mut ret = Vec::new();
        if filter.is_dead() || routing_domain_set.is_empty() {
            return ret;
        }
        for routing_domain in routing_domain_set {
            self.with_routing_domain(routing_domain, |rdd| {
                for did in rdd.dial_info_details() {
                    if did.matches_filter(filter) {
                        ret.push(did.clone());
                    }
                }
            });
        }
        ret.remove_duplicates();
        ret
    }

    pub fn node_info_is_valid_in_routing_domain(
        &self,
        routing_domain: RoutingDomain,
        node_info: &NodeInfo,
    ) -> bool {
        // Ensure all of the dial info works in this routing domain
        self.with_routing_domain(routing_domain, |rdd| {
            for did in node_info.dial_info_detail_list() {
                if !rdd.ensure_dial_info_is_valid(&did.dial_info) {
                    return false;
                }
            }
            true
        })
    }

    pub fn signed_node_info_is_valid_in_routing_domain(
        &self,
        routing_domain: RoutingDomain,
        signed_node_info: &SignedNodeInfo,
    ) -> bool {
        if !self.node_info_is_valid_in_routing_domain(routing_domain, signed_node_info.node_info())
        {
            return false;
        }
        // Ensure the relay is also valid in this routing domain if it is provided
        if let Some(relay_ni) = signed_node_info.relay_info() {
            // If there is a relay, the relay should have inbound capable network class and the node's network class should be valid
            if relay_ni.network_class() != NetworkClass::InboundCapable {
                return false;
            }
            if signed_node_info.node_info().network_class() == NetworkClass::Invalid {
                return false;
            }

            if !self.node_info_is_valid_in_routing_domain(routing_domain, relay_ni) {
                return false;
            }
        }
        true
    }

    pub fn get_contact_method(
        &self,
        routing_domain: RoutingDomain,
        peer_a: Arc<PeerInfo>,
        peer_b: Arc<PeerInfo>,
        dial_info_filter: DialInfoFilter,
        sequencing: Sequencing,
        dif_sort: Option<&DialInfoDetailSort>,
    ) -> ContactMethod {
        self.with_routing_domain(routing_domain, |rdd| {
            rdd.get_contact_method(self, peer_a, peer_b, dial_info_filter, sequencing, dif_sort)
        })
    }

    pub fn reset_all_updated_since_last_network_change(&mut self) {
        let cur_ts = Timestamp::now();
        self.with_entries_mut(cur_ts, BucketEntryState::Dead, |rti, v| {
            v.with_mut(rti, |_rti, e| {
                e.reset_updated_since_last_network_change();
            });
            Option::<()>::None
        });
    }

    /// Publish the node's current peer info to the world if it is valid
    pub fn publish_peer_info(&mut self, routing_domain: RoutingDomain) -> bool {
        self.with_routing_domain(routing_domain, |rdd| rdd.publish_peer_info(self))
    }
    /// Unpublish the node's current peer info
    pub fn unpublish_peer_info(&mut self, routing_domain: RoutingDomain) {
        self.with_routing_domain(routing_domain, |rdd| rdd.unpublish_peer_info())
    }

    /// Get the current published peer info
    pub fn get_published_peer_info(&self, routing_domain: RoutingDomain) -> Option<Arc<PeerInfo>> {
        self.with_routing_domain(routing_domain, |rdd| rdd.get_published_peer_info())
    }

    /// Return a copy of our node's current peerinfo (may not yet be published)
    pub fn get_current_peer_info(&self, routing_domain: RoutingDomain) -> Arc<PeerInfo> {
        self.with_routing_domain(routing_domain, |rdd| rdd.get_peer_info(self))
    }

    /// Return a list of the current valid bootstrap peers in a particular routing domain
    pub fn get_bootstrap_peers(&self, routing_domain: RoutingDomain) -> Vec<NodeRef> {
        self.with_routing_domain(routing_domain, |rdd| rdd.get_bootstrap_peers())
    }

    /// Return the domain's currently registered network class
    pub fn get_network_class(&self, routing_domain: RoutingDomain) -> NetworkClass {
        self.with_routing_domain(routing_domain, |rdd| rdd.network_class())
    }

    /// Return the domain's filter for what we can receivein the form of a dial info filter
    pub fn get_inbound_dial_info_filter(&self, routing_domain: RoutingDomain) -> DialInfoFilter {
        self.with_routing_domain(routing_domain, |rdd| rdd.inbound_dial_info_filter())
    }

    /// Return the domain's filter for what we can receive in the form of a node ref filter
    #[expect(dead_code)]
    pub fn get_inbound_node_ref_filter(&self, routing_domain: RoutingDomain) -> NodeRefFilter {
        let dif = self.get_inbound_dial_info_filter(routing_domain);
        NodeRefFilter::new()
            .with_routing_domain(routing_domain)
            .with_dial_info_filter(dif)
    }

    /// Return the domain's filter for what we can send out in the form of a dial info filter
    pub fn get_outbound_dial_info_filter(&self, routing_domain: RoutingDomain) -> DialInfoFilter {
        self.with_routing_domain(routing_domain, |rdd| rdd.outbound_dial_info_filter())
    }
    /// Return the domain's filter for what we can receive in the form of a node ref filter
    pub fn get_outbound_node_ref_filter(&self, routing_domain: RoutingDomain) -> NodeRefFilter {
        let dif = self.get_outbound_dial_info_filter(routing_domain);
        NodeRefFilter::new()
            .with_routing_domain(routing_domain)
            .with_dial_info_filter(dif)
    }

    fn bucket_depth(bucket_index: BucketIndex) -> usize {
        match bucket_index.1 {
            0 => 256,
            1 => 128,
            2 => 64,
            3 => 32,
            4 => 16,
            5 => 8,
            6 => 4,
            7 => 2,
            _ => 1,
        }
    }

    pub fn init_buckets(&mut self) {
        // Size the buckets (one per bit), one bucket set per crypto kind
        self.buckets.clear();
        for ck in VALID_CRYPTO_KINDS {
            let mut ckbuckets = Vec::with_capacity(PUBLIC_KEY_LENGTH * 8);
            for _ in 0..PUBLIC_KEY_LENGTH * 8 {
                let bucket = Bucket::new(self.registry(), ck);
                ckbuckets.push(bucket);
            }
            self.buckets.insert(ck, ckbuckets);
        }
    }

    /// Attempt to empty the routing table
    /// should only be performed when there are no node_refs (detached)
    pub fn purge_buckets(&mut self) {
        veilid_log!(self trace
            "Starting routing table buckets purge. Table currently has {} nodes",
            self.bucket_entry_count()
        );
        let closest_nodes = BTreeSet::new();
        for ck in VALID_CRYPTO_KINDS {
            for bucket in self.buckets.get_mut(&ck).unwrap().iter_mut() {
                bucket.kick(0, &closest_nodes);
            }
        }
        self.all_entries.remove_expired();

        veilid_log!(self debug
            "Routing table buckets purge complete. Routing table now has {} nodes",
            self.bucket_entry_count()
        );
    }

    /// Attempt to remove last_connections from entries
    pub fn purge_last_connections(&mut self) {
        veilid_log!(self trace "Starting routing table last_connections purge.");
        for ck in VALID_CRYPTO_KINDS {
            for bucket in &self.buckets[&ck] {
                for entry in bucket.entries() {
                    entry.1.with_mut_inner(|e| {
                        e.clear_last_flows(DialInfoFilter::all());
                    });
                }
            }
        }
        veilid_log!(self debug "Routing table last_connections purge complete.");
    }

    /// Attempt to settle buckets and remove entries down to the desired number
    /// which may not be possible due extant NodeRefs
    pub fn kick_bucket(&mut self, bucket_index: BucketIndex, exempt_peers: &BTreeSet<NodeId>) {
        let bucket = self.get_bucket_mut(bucket_index);
        let bucket_depth = Self::bucket_depth(bucket_index);

        if let Some(dead_node_ids) = bucket.kick(bucket_depth, exempt_peers) {
            // Remove expired entries
            self.all_entries.remove_expired();

            veilid_log!(self debug "Bucket {}:{} kicked Routing table now has {} nodes\nKicked nodes:{:#?}", bucket_index.0, bucket_index.1, self.bucket_entry_count(), dead_node_ids);
        }
    }

    /// Build the counts of entries per routing domain and crypto kind and cache them
    /// Only considers entries that have valid signed node info
    pub fn refresh_cached_live_entry_counts(&mut self) -> Arc<LiveEntryCounts> {
        let mut live_entry_counts = LiveEntryCounts::default();

        let cur_ts = Timestamp::now();
        self.with_entries_mut(cur_ts, BucketEntryState::Unreliable, |_rti, entry| {
            entry.with_inner(|e| {
                // Tally per routing domain and crypto kind
                for rd in RoutingDomain::all() {
                    if let Some(sni) = e.signed_node_info(rd) {
                        // Only consider entries that have valid signed node info in this domain
                        if sni.has_any_signature() {
                            // Tally
                            for crypto_kind in e.crypto_kinds() {
                                live_entry_counts
                                    .any_capabilities
                                    .entry((rd, crypto_kind))
                                    .and_modify(|x| *x += 1)
                                    .or_insert(1);
                                if e.has_all_capabilities(rd, CONNECTIVITY_CAPABILITIES) {
                                    live_entry_counts
                                        .connectivity_capabilities
                                        .entry((rd, crypto_kind))
                                        .and_modify(|x| *x += 1)
                                        .or_insert(1);
                                }
                                if e.has_all_capabilities(rd, DISTANCE_METRIC_CAPABILITIES) {
                                    live_entry_counts
                                        .distance_metric_capabilities
                                        .entry((rd, crypto_kind))
                                        .and_modify(|x| *x += 1)
                                        .or_insert(1);
                                }
                            }
                        }
                    }
                }
            });
            Option::<()>::None
        });
        self.live_entry_counts = Arc::new(live_entry_counts);
        self.live_entry_counts.clone()
    }

    /// Return the last cached entry counts
    /// Only considers entries that have valid signed node info
    pub fn cached_live_entry_counts(&self) -> Arc<LiveEntryCounts> {
        self.live_entry_counts.clone()
    }

    /// Count entries that match some criteria
    pub fn get_entry_count(
        &self,
        routing_domain_set: RoutingDomainSet,
        min_state: BucketEntryState,
        crypto_kinds: &[CryptoKind],
    ) -> usize {
        let mut count = 0usize;
        let cur_ts = Timestamp::now();
        self.with_entries(cur_ts, min_state, |rti, e| {
            if e.with_inner(|e| {
                e.best_routing_domain(rti, routing_domain_set).is_some()
                    && !common_crypto_kinds(&e.crypto_kinds(), crypto_kinds).is_empty()
            }) {
                count += 1;
            }
            Option::<()>::None
        });
        count
    }

    /// Iterate entries with a filter
    pub fn with_entries<T, F: FnMut(&RoutingTableInner, Arc<BucketEntry>) -> Option<T>>(
        &self,
        cur_ts: Timestamp,
        min_state: BucketEntryState,
        mut f: F,
    ) -> Option<T> {
        for entry in &self.all_entries {
            if entry.with_inner(|e| e.state(cur_ts) >= min_state) {
                if let Some(out) = f(self, entry) {
                    return Some(out);
                }
            }
        }

        None
    }

    /// Iterate entries with a filter mutably
    pub fn with_entries_mut<T, F: FnMut(&mut RoutingTableInner, Arc<BucketEntry>) -> Option<T>>(
        &mut self,
        cur_ts: Timestamp,
        min_state: BucketEntryState,
        mut f: F,
    ) -> Option<T> {
        let mut entries = Vec::with_capacity(self.all_entries.len());
        for entry in self.all_entries.iter() {
            if entry.with_inner(|e| e.state(cur_ts) >= min_state) {
                entries.push(entry);
            }
        }
        for entry in entries {
            if let Some(out) = f(self, entry) {
                return Some(out);
            }
        }
        None
    }

    // Collect all entries that are 'needs_ping' and have some node info making them reachable somehow
    pub(super) fn get_nodes_needing_ping(
        &self,
        routing_domain: RoutingDomain,
        cur_ts: Timestamp,
    ) -> Vec<FilteredNodeRef> {
        let opt_own_node_info_ts = self
            .get_published_peer_info(routing_domain)
            .map(|pi| pi.signed_node_info().timestamp());

        let mut filters = VecDeque::new();

        // Remove our own node from the results
        let filter_self =
            Box::new(move |_rti: &RoutingTableInner, v: Option<Arc<BucketEntry>>| v.is_some())
                as RoutingTableEntryFilter;
        filters.push_back(filter_self);

        let filter_ping = Box::new(
            move |rti: &RoutingTableInner, v: Option<Arc<BucketEntry>>| {
                let entry = v.unwrap();
                entry.with_inner(|e| {
                    // If this entry isn't in the routing domain we are checking, don't include it
                    if !e.exists_in_routing_domain(rti, routing_domain) {
                        return false;
                    }

                    // If we don't have node status for this node, then we should ping it to get some node status
                    if e.has_node_info(routing_domain.into())
                        && e.node_status(routing_domain).is_none()
                    {
                        return true;
                    }

                    // If this entry needs a ping because this node hasn't seen our latest node info, then do it
                    if opt_own_node_info_ts.is_some()
                        && !e.has_seen_our_node_info_ts(
                            routing_domain,
                            opt_own_node_info_ts.unwrap(),
                        )
                    {
                        return true;
                    }

                    // If this entry needs need a ping by non-routing-domain-specific metrics then do it
                    if e.needs_ping(cur_ts) {
                        return true;
                    }

                    false
                })
            },
        ) as RoutingTableEntryFilter;
        filters.push_back(filter_ping);

        // Sort by least recently contacted
        let compare = |_rti: &RoutingTableInner,
                       a_entry: &Option<Arc<BucketEntry>>,
                       b_entry: &Option<Arc<BucketEntry>>| {
            // same nodes are always the same
            if let Some(a_entry) = a_entry {
                if let Some(b_entry) = b_entry {
                    if Arc::ptr_eq(a_entry, b_entry) {
                        return core::cmp::Ordering::Equal;
                    }
                }
            } else if b_entry.is_none() {
                return core::cmp::Ordering::Equal;
            }

            // our own node always comes last (should not happen, here for completeness)
            if a_entry.is_none() {
                return core::cmp::Ordering::Greater;
            }
            if b_entry.is_none() {
                return core::cmp::Ordering::Less;
            }
            // Sort by least recently contacted regardless of reliability
            // If something needs a ping it should get it in the order of need
            let ae = a_entry.as_ref().unwrap();
            let be = b_entry.as_ref().unwrap();
            ae.with_inner(|ae| {
                be.with_inner(|be| {
                    let ca = ae
                        .peer_stats()
                        .rpc_stats
                        .last_question_ts
                        .unwrap_or(Timestamp::new(0))
                        .as_u64();
                    let cb = be
                        .peer_stats()
                        .rpc_stats
                        .last_question_ts
                        .unwrap_or(Timestamp::new(0))
                        .as_u64();

                    ca.cmp(&cb)
                })
            })
        };

        let transform = |_rti: &RoutingTableInner, v: Option<Arc<BucketEntry>>| {
            FilteredNodeRef::new(
                self.registry.clone(),
                v.unwrap().clone(),
                NodeRefFilter::new().with_routing_domain(routing_domain),
                Sequencing::default(),
            )
        };

        self.find_peers_with_sort_and_filter(usize::MAX, cur_ts, filters, compare, transform)
    }

    #[expect(dead_code)]
    pub fn get_all_alive_nodes(&self, cur_ts: Timestamp) -> Vec<NodeRef> {
        let mut node_refs = Vec::<NodeRef>::with_capacity(self.bucket_entry_count());
        self.with_entries(cur_ts, BucketEntryState::Unreliable, |_rti, entry| {
            node_refs.push(NodeRef::new(self.registry(), entry));
            Option::<()>::None
        });
        node_refs
    }

    fn get_bucket_mut(&mut self, bucket_index: BucketIndex) -> &mut Bucket {
        self.buckets
            .get_mut(&bucket_index.0)
            .unwrap()
            .get_mut(bucket_index.1)
            .unwrap()
    }

    fn get_bucket(&self, bucket_index: BucketIndex) -> &Bucket {
        self.buckets
            .get(&bucket_index.0)
            .unwrap()
            .get(bucket_index.1)
            .unwrap()
    }

    // Update buckets with new node ids we may have learned belong to this entry
    fn update_bucket_entry_node_ids(
        &mut self,
        entry: Arc<BucketEntry>,
        node_ids: &[TypedNodeId],
    ) -> EyreResult<()> {
        let routing_table = self.routing_table();

        entry.with_mut_inner(|e| {
            let mut existing_node_ids = e.node_ids();

            // Peer infos for all routing domains we have
            let mut old_peer_infos = vec![];

            for node_id in node_ids {
                let ck = node_id.kind;
                let is_existing_node_id = existing_node_ids.contains(node_id);

                existing_node_ids.remove(ck);

                // Skip node ids that exist already
                if is_existing_node_id {
                    continue;
                }

                // New node id, get the old peer info if we don't have it yet
                if old_peer_infos.is_empty() {
                    for rd in RoutingDomainSet::all() {
                        if let Some(old_peer_info) = e.get_peer_info(rd) {
                            old_peer_infos.push(old_peer_info);
                        }
                    }
                }

                // Add new node id to entry
                if let Some(old_node_id) = e.add_node_id(*node_id)? {
                    // Remove any old node id for this crypto kind
                    if VALID_CRYPTO_KINDS.contains(&ck) {
                        let bucket_index = routing_table.calculate_bucket_index(&old_node_id);
                        let bucket = self.get_bucket_mut(bucket_index);
                        bucket.remove_entry(&old_node_id.value);
                        routing_table.kick_queue.lock().insert(bucket_index);
                    }
                }

                // Bucket the entry appropriately
                if VALID_CRYPTO_KINDS.contains(&ck) {
                    let bucket_index = routing_table.calculate_bucket_index(node_id);
                    let bucket = self.get_bucket_mut(bucket_index);
                    bucket.add_existing_entry(node_id.value, entry.clone());

                    // Kick bucket
                    routing_table.kick_queue.lock().insert(bucket_index);
                }
            }

            // Remove from buckets if node id wasn't seen in new peer info list
            for node_id in existing_node_ids.iter() {
                let ck = node_id.kind;
                if VALID_CRYPTO_KINDS.contains(&ck) {
                    let bucket_index = routing_table.calculate_bucket_index(node_id);
                    let bucket = self.get_bucket_mut(bucket_index);
                    bucket.remove_entry(&node_id.value);
                    entry.with_mut_inner(|e| e.remove_node_id(ck));
                }
            }

            // New node id, get the old peer info if we don't have it yet
            if !old_peer_infos.is_empty() {
                let mut new_peer_infos = vec![];
                for rd in RoutingDomainSet::all() {
                    if let Some(new_peer_info) = e.get_peer_info(rd) {
                        new_peer_infos.push(new_peer_info);
                    }
                }

                // adding a node id should never change what routing domains peers are in
                // so we should have a 1:1 ordered mapping here to update with the new nodeids
                assert_eq!(old_peer_infos.len(), new_peer_infos.len());
                for (old_pi, new_pi) in old_peer_infos.into_iter().zip(new_peer_infos.into_iter()) {
                    assert_eq!(old_pi.routing_domain(), new_pi.routing_domain());
                    self.on_entry_peer_info_updated(Some(old_pi), Some(new_pi));
                }
            }
            Ok(())
        })
    }

    /// Create a node reference, possibly creating a bucket entry
    /// the 'update_func' closure is called on the node, and, if created,
    /// in a locked fashion as to ensure the bucket entry state is always valid
    #[instrument(level = "trace", skip_all, err)]
    fn create_node_ref<F>(
        &mut self,
        node_ids: &TypedNodeIdGroup,
        update_func: F,
    ) -> EyreResult<NodeRef>
    where
        F: FnOnce(&mut RoutingTableInner, &mut BucketEntryInner),
    {
        let routing_table = self.routing_table();

        // Ensure someone isn't trying register this node itself
        if routing_table.matches_own_node_id(node_ids) {
            bail!("can't register own node");
        }

        // Look up all bucket entries and make sure we only have zero or one
        // If we have more than one, pick the one with the best cryptokind to add node ids to
        let mut best_entry: Option<Arc<BucketEntry>> = None;
        for node_id in node_ids.iter() {
            // Ignore node ids we don't support
            if !VALID_CRYPTO_KINDS.contains(&node_id.kind) {
                continue;
            }
            // Find the first in crypto sort order
            let bucket_index = routing_table.calculate_bucket_index(node_id);
            let bucket = self.get_bucket(bucket_index);
            if let Some(entry) = bucket.entry(&node_id.value) {
                // Best entry is the first one in sorted order that exists from the node id list
                // Everything else that matches will be overwritten in the bucket and the
                // existing noderefs will eventually unref and drop the old unindexed bucketentry
                // We do this instead of merging for now. We could 'kill' entries and have node_refs
                // rewrite themselves to point to the merged entry upon dereference. The use case for this
                // may not be worth the effort.
                best_entry = Some(entry);
                break;
            };
        }

        // If the entry does exist already, update it
        if let Some(best_entry) = best_entry {
            // Update the entry with all of the node ids
            if let Err(e) = self.update_bucket_entry_node_ids(best_entry.clone(), node_ids) {
                bail!("Not registering new ids for existing node: {}", e);
            }

            // Make a noderef to return
            let nr = NodeRef::new(self.registry(), best_entry.clone());

            // Update the entry with the update func
            best_entry.with_mut_inner(|e| update_func(self, e));

            // Return the noderef
            return Ok(nr);
        }

        // If no entry exists yet, add the first entry to a bucket, possibly evicting a bucket member
        let first_node_id = node_ids[0];
        let bucket_entry = routing_table.calculate_bucket_index(&first_node_id);
        let bucket = self.get_bucket_mut(bucket_entry);
        let new_entry = bucket.add_new_entry(first_node_id.value);
        self.all_entries.insert(new_entry.clone());
        routing_table.kick_queue.lock().insert(bucket_entry);

        // Update the other bucket entries with the remaining node ids
        if let Err(e) = self.update_bucket_entry_node_ids(new_entry.clone(), node_ids) {
            bail!("Not registering new node: {}", e);
        }

        // Make node ref to return
        let nr = NodeRef::new(self.registry(), new_entry.clone());

        // Update the entry with the update func
        new_entry.with_mut_inner(|e| update_func(self, e));

        // Kick the bucket
        veilid_log!(self debug "Routing table now has {} nodes, {} live", self.bucket_entry_count(), self.get_entry_count(RoutingDomainSet::all(), BucketEntryState::Unreliable, &VALID_CRYPTO_KINDS));

        Ok(nr)
    }

    /// Resolve an existing routing table entry using any crypto kind and return a reference to it
    #[instrument(level = "trace", skip_all, err)]
    pub fn lookup_any_node_ref(&self, node_id_key: NodeId) -> EyreResult<Option<NodeRef>> {
        for ck in VALID_CRYPTO_KINDS {
            if let Some(nr) = self.lookup_node_ref(TypedNodeId::new(ck, node_id_key))? {
                return Ok(Some(nr));
            }
        }
        Ok(None)
    }

    /// Resolve an existing routing table entry and return a reference to it
    #[instrument(level = "trace", skip_all, err)]
    pub fn lookup_node_ref(&self, node_id: TypedNodeId) -> EyreResult<Option<NodeRef>> {
        if self.routing_table().matches_own_node_id(&[node_id]) {
            bail!("can't look up own node id in routing table");
        }
        if !VALID_CRYPTO_KINDS.contains(&node_id.kind) {
            bail!("can't look up node id with invalid crypto kind");
        }

        let bucket_index = self.routing_table().calculate_bucket_index(&node_id);
        let bucket = self.get_bucket(bucket_index);
        Ok(bucket
            .entry(&node_id.value)
            .map(|e| NodeRef::new(self.registry(), e)))
    }

    /// Resolve an existing routing table entry and return a filtered reference to it
    #[instrument(level = "trace", skip_all, err)]
    pub fn lookup_and_filter_noderef(
        &self,
        node_id: TypedNodeId,
        routing_domain_set: RoutingDomainSet,
        dial_info_filter: DialInfoFilter,
    ) -> EyreResult<Option<FilteredNodeRef>> {
        let nr = self.lookup_node_ref(node_id)?;
        Ok(nr.map(|nr| {
            nr.custom_filtered(
                NodeRefFilter::new()
                    .with_dial_info_filter(dial_info_filter)
                    .with_routing_domain_set(routing_domain_set),
            )
        }))
    }

    /// Resolve an existing routing table entry and call a function on its entry without using a noderef
    pub fn with_node_entry<F, R>(&self, node_id: TypedNodeId, f: F) -> Option<R>
    where
        F: FnOnce(Arc<BucketEntry>) -> R,
    {
        if self.routing_table().matches_own_node_id(&[node_id]) {
            veilid_log!(self error "can't look up own node id in routing table");
            return None;
        }
        if !VALID_CRYPTO_KINDS.contains(&node_id.kind) {
            veilid_log!(self error "can't look up node id with invalid crypto kind");
            return None;
        }
        let bucket_entry = self.routing_table().calculate_bucket_index(&node_id);
        let bucket = self.get_bucket(bucket_entry);
        bucket.entry(&node_id.value).map(f)
    }

    /// Shortcut function to add a node to our routing table if it doesn't exist
    /// and add the dial info we have for it. Returns a noderef filtered to
    /// the routing domain in which this node was registered for convenience.
    #[instrument(level = "trace", skip_all, err)]
    pub fn register_node_with_peer_info(
        &mut self,
        peer_info: Arc<PeerInfo>,
        allow_invalid: bool,
    ) -> EyreResult<FilteredNodeRef> {
        let routing_domain = peer_info.routing_domain();

        // if our own node is in the list, then ignore it as we don't add ourselves to our own routing table
        if self
            .routing_table()
            .matches_own_node_id(peer_info.node_ids())
        {
            bail!("can't register own node id in routing table");
        }

        // node can not be its own relay
        let rids = peer_info.signed_node_info().relay_ids();
        let nids = peer_info.node_ids();
        if nids.contains_any(&rids) {
            bail!("node can not be its own relay");
        }

        if !allow_invalid {
            // verify signature
            if !peer_info.signed_node_info().has_any_signature() {
                bail!(
                    "signed node info for {:?} has no valid signature",
                    peer_info.node_ids()
                );
            }
            // verify signed node info is valid in this routing domain
            if !self.signed_node_info_is_valid_in_routing_domain(
                routing_domain,
                peer_info.signed_node_info(),
            ) {
                bail!(
                    "signed node info for {:?} not valid in the {:?} routing domain",
                    peer_info.node_ids(),
                    routing_domain
                );
            }
        }

        // Register relay info first if we have that and the relay isn't us
        if let Some(relay_peer_info) = peer_info.signed_node_info().relay_peer_info(routing_domain)
        {
            if !self
                .routing_table()
                .matches_own_node_id(relay_peer_info.node_ids())
            {
                self.register_node_with_peer_info(relay_peer_info, false)?;
            }
        }

        let (_routing_domain, node_ids, signed_node_info) =
            Arc::unwrap_or_clone(peer_info).destructure();
        let mut updated = false;
        let mut old_peer_info = None;
        let nr = self.create_node_ref(&node_ids, |_rti, e| {
            old_peer_info = e.get_peer_info(routing_domain);
            updated = e.update_signed_node_info(routing_domain, &signed_node_info);
        })?;

        // Process any new or updated PeerInfo
        if old_peer_info.is_none() || updated {
            let new_peer_info = nr.locked(self).get_peer_info(routing_domain);
            self.on_entry_peer_info_updated(old_peer_info, new_peer_info);
        }

        Ok(nr.custom_filtered(NodeRefFilter::new().with_routing_domain(routing_domain)))
    }

    /// Shortcut function to add a node to our routing table if it doesn't exist
    /// Returns a noderef filtered to
    /// the routing domain in which this node was registered for convenience.
    #[instrument(level = "trace", skip_all, err)]
    pub fn register_node_with_id(
        &mut self,
        routing_domain: RoutingDomain,
        node_id: TypedNodeId,
        timestamp: Timestamp,
    ) -> EyreResult<FilteredNodeRef> {
        let nr = self.create_node_ref(&TypedNodeIdGroup::from(node_id), |_rti, e| {
            //e.make_not_dead(timestamp);
            e.touch_last_seen(timestamp);
        })?;

        // Enforce routing domain
        let nr = nr.custom_filtered(NodeRefFilter::new().with_routing_domain(routing_domain));
        Ok(nr)
    }

    /// Called whenever a routing table entry is:
    /// 1. created or updated with new peer information
    /// 2. has a node id added or removed (per CryptoKind)
    ///   * by a new peer info showing up with a different overlapping node id list
    ///   * by a bucket kick removing an entry from a bucket for some cryptokind
    /// 3. (todo) is removed from some routing domain (peer info gone)
    ///
    /// It is not called when:
    /// 1. nodes are registered by id for an existing connection but have no peer info yet
    /// 2. nodes are removed that don't have any peer info
    fn on_entry_peer_info_updated(
        &mut self,
        old_peer_info: Option<Arc<PeerInfo>>,
        new_peer_info: Option<Arc<PeerInfo>>,
    ) {
        let (routing_domain, node_ids) = match (old_peer_info.as_ref(), new_peer_info.as_ref()) {
            (None, None) => {
                return;
            }
            (None, Some(new_pi)) => (new_pi.routing_domain(), new_pi.node_ids().clone()),
            (Some(old_pi), None) => (old_pi.routing_domain(), old_pi.node_ids().clone()),
            (Some(old_pi), Some(new_pi)) => {
                assert_eq!(
                    old_pi.routing_domain(),
                    new_pi.routing_domain(),
                    "routing domains should be the same here",
                );
                let mut node_ids = old_pi.node_ids().clone();
                node_ids.add_all(new_pi.node_ids());
                (new_pi.routing_domain(), node_ids)
            }
        };

        // If this is our relay, then redo our own peerinfo because
        // if we have relayed peerinfo, then changing the relay's peerinfo
        // changes our own peer info
        self.with_routing_domain(routing_domain, |rd| {
            let opt_our_relay_node_ids = rd
                .relay_node()
                .map(|relay_nr| relay_nr.locked(self).node_ids());
            if let Some(our_relay_node_ids) = opt_our_relay_node_ids {
                if our_relay_node_ids.contains_any(&node_ids) {
                    rd.refresh();
                    rd.publish_peer_info(self);
                }
            }
        });

        // Update tables that use peer info
        // if let Some(_old_pi) = old_peer_info {
        //     // Remove old info
        // }
        // if let Some(_new_pi) = new_peer_info {
        //     // Add new info
        // }
    }

    //////////////////////////////////////////////////////////////////////
    // Routing Table Health Metrics

    pub fn get_routing_table_health(&self) -> RoutingTableHealth {
        let mut _punished_entry_count: usize = 0;
        let mut reliable_entry_count: usize = 0;
        let mut unreliable_entry_count: usize = 0;
        let mut dead_entry_count: usize = 0;

        let cur_ts = Timestamp::now();
        for entry in self.all_entries.iter() {
            match entry.with_inner(|e| e.state(cur_ts)) {
                BucketEntryState::Reliable => {
                    reliable_entry_count += 1;
                }
                BucketEntryState::Unreliable => {
                    unreliable_entry_count += 1;
                }
                BucketEntryState::Dead => {
                    dead_entry_count += 1;
                }
                BucketEntryState::Punished => {
                    _punished_entry_count += 1;
                }
            }
        }

        // Public internet routing domain is ready for app use,
        // when we have proper dialinfo/networkclass and it is published
        let public_internet_ready = self
            .get_published_peer_info(RoutingDomain::PublicInternet)
            .is_some();

        // Local internet routing domain is ready for app use
        // when we have proper dialinfo/networkclass and it is published
        let local_network_ready = self
            .get_published_peer_info(RoutingDomain::LocalNetwork)
            .is_some();

        let live_entry_counts = self.cached_live_entry_counts().as_ref().clone();

        RoutingTableHealth {
            reliable_entry_count,
            unreliable_entry_count,
            dead_entry_count,
            live_entry_counts,
            public_internet_ready,
            local_network_ready,
        }
    }

    pub fn touch_recent_peer(&mut self, node_id: TypedNodeId, last_connection: Flow) {
        self.recent_peers
            .insert(node_id, RecentPeersEntry { last_connection });
    }

    //////////////////////////////////////////////////////////////////////
    // Find Nodes

    // Retrieve the fastest nodes in the routing table matching an entry filter
    #[instrument(level = "trace", skip_all)]
    pub fn find_fast_non_local_nodes_filtered(
        &self,
        registry: VeilidComponentRegistry,
        routing_domain: RoutingDomain,
        node_count: usize,
        mut filters: VecDeque<RoutingTableEntryFilter>,
    ) -> Vec<NodeRef> {
        assert_ne!(
            routing_domain,
            RoutingDomain::LocalNetwork,
            "LocalNetwork is not a valid non-local RoutingDomain"
        );
        let public_node_filter = Box::new(
            move |_rti: &RoutingTableInner, v: Option<Arc<BucketEntry>>| {
                let entry = v.unwrap();
                entry.with_inner(|e| {
                    // skip nodes on local network
                    if e.node_info(RoutingDomain::LocalNetwork).is_some() {
                        return false;
                    }
                    // skip nodes not on desired routing domain
                    if e.node_info(routing_domain).is_none() {
                        return false;
                    }
                    true
                })
            },
        ) as RoutingTableEntryFilter;
        filters.push_front(public_node_filter);

        self.find_preferred_fastest_nodes(
            node_count,
            filters,
            |_rti: &RoutingTableInner, v: Option<Arc<BucketEntry>>| {
                NodeRef::new(registry.clone(), v.unwrap().clone())
            },
        )
    }

    #[instrument(level = "trace", skip_all)]
    pub fn filter_has_valid_signed_node_info(
        &self,
        routing_domain: RoutingDomain,
        has_valid_own_node_info: bool,
        entry: Option<Arc<BucketEntry>>,
    ) -> bool {
        match entry {
            None => has_valid_own_node_info,
            Some(entry) => entry.with_inner(|e| {
                e.signed_node_info(routing_domain)
                    .map(|sni| {
                        sni.has_any_signature()
                            && !matches!(sni.node_info().network_class(), NetworkClass::Invalid)
                    })
                    .unwrap_or(false)
            }),
        }
    }

    #[instrument(level = "trace", skip_all)]
    pub fn transform_to_peer_info(
        &self,
        routing_domain: RoutingDomain,
        own_peer_info: Arc<PeerInfo>,
        entry: Option<Arc<BucketEntry>>,
    ) -> Arc<PeerInfo> {
        match entry {
            None => own_peer_info.clone(),
            Some(entry) => entry.with_inner(|e| e.get_peer_info(routing_domain).unwrap()),
        }
    }

    #[instrument(level = "trace", skip_all)]
    pub fn find_peers_with_sort_and_filter<C, T, O>(
        &self,
        node_count: usize,
        cur_ts: Timestamp,
        mut filters: VecDeque<RoutingTableEntryFilter>,
        mut compare: C,
        mut transform: T,
    ) -> Vec<O>
    where
        C: for<'a, 'b> FnMut(
            &'a RoutingTableInner,
            &'b Option<Arc<BucketEntry>>,
            &'b Option<Arc<BucketEntry>>,
        ) -> core::cmp::Ordering,
        T: for<'r> FnMut(&'r RoutingTableInner, Option<Arc<BucketEntry>>) -> O,
    {
        // collect all the nodes for sorting
        let mut nodes =
            Vec::<Option<Arc<BucketEntry>>>::with_capacity(self.bucket_entry_count() + 1);

        // add our own node (only one of there with the None entry)
        let mut filtered = false;
        for filter in &mut filters {
            if !filter(self, None) {
                filtered = true;
                break;
            }
        }
        if !filtered {
            nodes.push(None);
        }

        // add all nodes that match filter
        self.with_entries(cur_ts, BucketEntryState::Unreliable, |rti, v| {
            // Apply filter
            let mut filtered = false;
            for filter in &mut filters {
                if !filter(rti, Some(v.clone())) {
                    filtered = true;
                    break;
                }
            }
            if !filtered {
                nodes.push(Some(v.clone()));
            }
            Option::<()>::None
        });

        // sort by preference for returning nodes
        nodes.sort_by(|a, b| compare(self, a, b));

        // return transformed vector for filtered+sorted nodes
        nodes.truncate(node_count);
        let mut out = Vec::<O>::with_capacity(nodes.len());
        for node in nodes {
            let val = transform(self, node);
            out.push(val);
        }

        out
    }

    #[instrument(level = "trace", skip_all)]
    pub fn find_preferred_fastest_nodes<T, O>(
        &self,
        node_count: usize,
        mut filters: VecDeque<RoutingTableEntryFilter>,
        transform: T,
    ) -> Vec<O>
    where
        T: for<'r> FnMut(&'r RoutingTableInner, Option<Arc<BucketEntry>>) -> O,
    {
        let cur_ts = Timestamp::now();

        // always filter out self peer, as it is irrelevant to the 'fastest nodes' search
        let filter_self =
            Box::new(move |_rti: &RoutingTableInner, v: Option<Arc<BucketEntry>>| v.is_some())
                as RoutingTableEntryFilter;
        filters.push_front(filter_self);

        // Fastest sort
        let sort = |_rti: &RoutingTableInner,
                    a_entry: &Option<Arc<BucketEntry>>,
                    b_entry: &Option<Arc<BucketEntry>>| {
            // same nodes are always the same
            if let Some(a_entry) = a_entry {
                if let Some(b_entry) = b_entry {
                    if Arc::ptr_eq(a_entry, b_entry) {
                        return core::cmp::Ordering::Equal;
                    }
                }
            } else if b_entry.is_none() {
                return core::cmp::Ordering::Equal;
            }

            // our own node always comes last (should not happen, here for completeness)
            if a_entry.is_none() {
                return core::cmp::Ordering::Greater;
            }
            if b_entry.is_none() {
                return core::cmp::Ordering::Less;
            }
            // reliable nodes come first
            let ae = a_entry.as_ref().unwrap();
            let be = b_entry.as_ref().unwrap();
            ae.with_inner(|ae| {
                be.with_inner(|be| {
                    let ra = ae.check_unreliable(cur_ts).is_none();
                    let rb = be.check_unreliable(cur_ts).is_none();
                    if ra != rb {
                        if ra {
                            return core::cmp::Ordering::Less;
                        } else {
                            return core::cmp::Ordering::Greater;
                        }
                    }

                    // latency is the next metric, closer nodes first
                    let a_latency = match ae.peer_stats().latency.as_ref() {
                        None => {
                            // treat unknown latency as slow
                            return core::cmp::Ordering::Greater;
                        }
                        Some(l) => l,
                    };
                    let b_latency = match be.peer_stats().latency.as_ref() {
                        None => {
                            // treat unknown latency as slow
                            return core::cmp::Ordering::Less;
                        }
                        Some(l) => l,
                    };
                    // Sort by average latency
                    a_latency.average.cmp(&b_latency.average)
                })
            })
        };

        self.find_peers_with_sort_and_filter(node_count, cur_ts, filters, sort, transform)
    }

    #[instrument(level = "trace", skip_all)]
    pub fn find_preferred_closest_nodes<T, O>(
        &self,
        node_count: usize,
        node_id: TypedHashDigest,
        mut filters: VecDeque<RoutingTableEntryFilter>,
        transform: T,
    ) -> VeilidAPIResult<Vec<O>>
    where
        T: for<'r> FnMut(&'r RoutingTableInner, Option<Arc<BucketEntry>>) -> O,
    {
        let cur_ts = Timestamp::now();
        let routing_table = self.routing_table();

        // Get the crypto kind
        let crypto_kind = node_id.kind;
        let crypto = self.crypto();
        let Some(vcrypto) = crypto.get(crypto_kind) else {
            apibail_generic!("invalid crypto kind");
        };

        // Filter to ensure entries support the crypto kind in use
        // always filter out dead and punished nodes
        let filter = Box::new(
            move |_rti: &RoutingTableInner, opt_entry: Option<Arc<BucketEntry>>| {
                if let Some(entry) = opt_entry {
                    entry.with_inner(|e| e.crypto_kinds().contains(&crypto_kind))
                } else {
                    VALID_CRYPTO_KINDS.contains(&crypto_kind)
                }
            },
        ) as RoutingTableEntryFilter;
        filters.push_front(filter);

        // Closest sort
        // Distance is done using the node id's distance metric which may vary based on crypto system
        let sort = |_rti: &RoutingTableInner,
                    a_entry: &Option<Arc<BucketEntry>>,
                    b_entry: &Option<Arc<BucketEntry>>| {
            // same nodes are always the same
            if let Some(a_entry) = a_entry {
                if let Some(b_entry) = b_entry {
                    if Arc::ptr_eq(a_entry, b_entry) {
                        return core::cmp::Ordering::Equal;
                    }
                }
            } else if b_entry.is_none() {
                return core::cmp::Ordering::Equal;
            }

            // reliable nodes come first, pessimistically treating our own node as unreliable
            let ra = a_entry
                .as_ref()
                .is_some_and(|x| x.with_inner(|x| x.check_unreliable(cur_ts).is_none()));
            let rb = b_entry
                .as_ref()
                .is_some_and(|x| x.with_inner(|x| x.check_unreliable(cur_ts).is_none()));
            if ra != rb {
                if ra {
                    return core::cmp::Ordering::Less;
                } else {
                    return core::cmp::Ordering::Greater;
                }
            }

            // get keys
            let a_key = if let Some(a_entry) = a_entry {
                a_entry.with_inner(|e| e.node_ids().get(crypto_kind).unwrap())
            } else {
                routing_table.node_id(crypto_kind)
            };
            let b_key = if let Some(b_entry) = b_entry {
                b_entry.with_inner(|e| e.node_ids().get(crypto_kind).unwrap())
            } else {
                routing_table.node_id(crypto_kind)
            };

            // distance is the next metric, closer nodes first
            let da = vcrypto.distance(&HashDigest::from(a_key.value), &node_id.value);
            let db = vcrypto.distance(&HashDigest::from(b_key.value), &node_id.value);
            da.cmp(&db)
        };

        let out =
            self.find_peers_with_sort_and_filter(node_count, cur_ts, filters, sort, transform);
        veilid_log!(self trace ">> find_closest_nodes: node count = {}", out.len());
        Ok(out)
    }

    #[instrument(level = "trace", skip_all)]
    pub fn sort_and_clean_closest_noderefs(
        &self,
        node_id: TypedHashDigest,
        closest_nodes: &[NodeRef],
    ) -> Vec<NodeRef> {
        // Lock all noderefs
        let kind = node_id.kind;
        let mut closest_nodes_locked: Vec<LockedNodeRef> = closest_nodes
            .iter()
            .filter_map(|nr| {
                let nr_locked = nr.locked(self);
                if nr_locked.node_ids().kinds().contains(&kind) {
                    Some(nr_locked)
                } else {
                    None
                }
            })
            .collect();

        // Sort closest
        let crypto = self.crypto();
        let sort = make_closest_noderef_sort(&crypto, node_id);
        closest_nodes_locked.sort_by(sort);

        // Unlock noderefs
        closest_nodes_locked.iter().map(|x| x.unlocked()).collect()
    }

    #[instrument(level = "trace", skip(self, filter, metric), ret)]
    pub fn find_fastest_node(
        &self,
        cur_ts: Timestamp,
        filter: impl Fn(&BucketEntryInner) -> bool,
        metric: impl Fn(&LatencyStats) -> TimestampDuration,
    ) -> Option<NodeRef> {
        // Go through all entries and find fastest entry that matches filter function
        let mut fastest_node: Option<Arc<BucketEntry>> = None;

        // Iterate all known nodes for candidates
        self.with_entries(cur_ts, BucketEntryState::Unreliable, |rti, entry| {
            let entry2 = entry.clone();
            entry.with(rti, |rti, e| {
                // Filter this node
                if filter(e) {
                    // Compare against previous candidate
                    if let Some(fastest_node) = fastest_node.as_mut() {
                        // Less is faster
                        let better = fastest_node.with(rti, |_rti, best| {
                            // choose low latency stability for relays
                            BucketEntryInner::cmp_fastest_reliable(cur_ts, e, best, &metric)
                                == std::cmp::Ordering::Less
                        });
                        // Now apply filter function and see if this node should be included
                        if better {
                            *fastest_node = entry2;
                        }
                    } else {
                        // Always store the first candidate
                        fastest_node = Some(entry2);
                    }
                }
            });
            // Don't end early, iterate through all entries
            Option::<()>::None
        });
        // Return the fastest node
        fastest_node.map(|e| NodeRef::new(self.registry(), e))
    }

    #[instrument(level = "trace", skip(self, filter, metric), ret)]
    pub fn find_random_fast_node(
        &self,
        cur_ts: Timestamp,
        filter: impl Fn(&BucketEntryInner) -> bool,
        percentile: f32,
        metric: impl Fn(&LatencyStats) -> TimestampDuration,
    ) -> Option<NodeRef> {
        // Go through all entries and find all entries that matches filter function
        let mut all_filtered_nodes: Vec<Arc<BucketEntry>> = Vec::new();

        // Iterate all known nodes for candidates
        self.with_entries(cur_ts, BucketEntryState::Unreliable, |rti, entry| {
            let entry2 = entry.clone();
            entry.with(rti, |_rti, e| {
                // Filter this node
                if filter(e) {
                    all_filtered_nodes.push(entry2);
                }
            });
            // Don't end early, iterate through all entries
            Option::<()>::None
        });

        // Sort by fastest tm90 reliable
        all_filtered_nodes.sort_by(|a, b| {
            a.with(self, |rti, ea| {
                b.with(rti, |_rti, eb| {
                    BucketEntryInner::cmp_fastest_reliable(cur_ts, ea, eb, &metric)
                })
            })
        });

        if all_filtered_nodes.is_empty() {
            return None;
        }

        let max_index =
            (((all_filtered_nodes.len() - 1) as f32) * (100.0 - percentile) / 100.0) as u32;
        let chosen_index = (get_random_u32() % (max_index + 1)) as usize;

        // Return the chosen node node
        Some(NodeRef::new(
            self.registry(),
            all_filtered_nodes[chosen_index].clone(),
        ))
    }

    #[instrument(level = "trace", skip(self, filter, metric), ret)]
    pub fn get_node_relative_performance(
        &self,
        node_id: TypedNodeId,
        cur_ts: Timestamp,
        filter: impl Fn(&BucketEntryInner) -> bool,
        metric: impl Fn(&LatencyStats) -> TimestampDuration,
    ) -> Option<NodeRelativePerformance> {
        // Go through all entries and find all entries that matches filter function
        let mut all_filtered_nodes: Vec<Arc<BucketEntry>> = Vec::new();

        // Iterate all known nodes for candidates
        self.with_entries(cur_ts, BucketEntryState::Unreliable, |rti, entry| {
            let entry2 = entry.clone();
            entry.with(rti, |_rti, e| {
                // Filter this node
                if filter(e) {
                    all_filtered_nodes.push(entry2);
                }
            });
            // Don't end early, iterate through all entries
            Option::<()>::None
        });

        // Sort by fastest tm90 reliable
        all_filtered_nodes.sort_by(|a, b| {
            a.with(self, |rti, ea| {
                b.with(rti, |_rti, eb| {
                    BucketEntryInner::cmp_fastest_reliable(cur_ts, ea, eb, &metric)
                })
            })
        });

        // Get position in list of requested node
        let node_count = all_filtered_nodes.len();
        let node_index = all_filtered_nodes
            .iter()
            .position(|x| x.with(self, |_rti, e| e.node_ids().contains(&node_id)))?;

        // Print faster node stats
        #[cfg(feature = "verbose-tracing")]
        for nl in 0..node_index {
            let (latency, best_node_id) = all_filtered_nodes[nl].with(self, |_rti, e| {
                (e.peer_stats().latency.clone(), e.best_node_id())
            });
            if let Some(node_id) = best_node_id {
                if let Some(latency) = latency {
                    veilid_log!(self debug "Better relay {}: {}: {}", nl, node_id, latency);
                }
            }
        }

        // Return 'percentile' position. Fastest node is 100%.
        Some(NodeRelativePerformance {
            percentile: 100.0f32 - ((node_index * 100) as f32) / (node_count as f32),
            node_index,
            node_count,
        })
    }
}

#[instrument(level = "trace", skip_all)]
pub fn make_closest_noderef_sort<'a>(
    crypto: &'a Crypto,
    node_id: TypedHashDigest,
) -> impl Fn(&LockedNodeRef, &LockedNodeRef) -> core::cmp::Ordering + 'a {
    let kind = node_id.kind;
    // Get cryptoversion to check distance with
    let vcrypto = crypto.get(kind).unwrap();

    move |a: &LockedNodeRef, b: &LockedNodeRef| -> core::cmp::Ordering {
        // same nodes are always the same
        if a.same_entry(b) {
            return core::cmp::Ordering::Equal;
        }

        a.operate(|_rti, a_entry| {
            b.operate(|_rti, b_entry| {
                // get keys
                let a_key = a_entry.node_ids().get(kind).unwrap();
                let b_key = b_entry.node_ids().get(kind).unwrap();

                // distance is the next metric, closer nodes first
                let da = vcrypto.distance(&HashDigest::from(a_key.value), &node_id.value);
                let db = vcrypto.distance(&HashDigest::from(b_key.value), &node_id.value);
                da.cmp(&db)
            })
        })
    }
}

pub fn make_closest_node_id_sort(
    crypto: &Crypto,
    node_id: TypedNodeId,
) -> impl Fn(&NodeId, &NodeId) -> core::cmp::Ordering + '_ {
    let kind = node_id.kind;
    // Get cryptoversion to check distance with
    let vcrypto = crypto.get(kind).unwrap();

    move |a: &NodeId, b: &NodeId| -> core::cmp::Ordering {
        // distance is the next metric, closer nodes first
        let da = vcrypto.distance(&HashDigest::from(*a), &HashDigest::from(node_id.value));
        let db = vcrypto.distance(&HashDigest::from(*b), &HashDigest::from(node_id.value));
        da.cmp(&db)
    }
}
