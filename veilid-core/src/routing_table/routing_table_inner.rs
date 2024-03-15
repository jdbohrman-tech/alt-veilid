use super::*;
use weak_table::PtrWeakHashSet;

pub const RECENT_PEERS_TABLE_SIZE: usize = 64;

pub type EntryCounts = BTreeMap<(RoutingDomain, CryptoKind), usize>;
//////////////////////////////////////////////////////////////////////////

/// RoutingTable rwlock-internal data
pub(crate) struct RoutingTableInner {
    /// Extra pointer to unlocked members to simplify access
    pub(super) unlocked_inner: Arc<RoutingTableUnlockedInner>,
    /// Routing table buckets that hold references to entries, per crypto kind
    pub(super) buckets: BTreeMap<CryptoKind, Vec<Bucket>>,
    /// A weak set of all the entries we have in the buckets for faster iteration
    pub(super) all_entries: PtrWeakHashSet<Weak<BucketEntry>>,
    /// A rough count of the entries in the table per routing domain and crypto kind
    pub(super) live_entry_count: EntryCounts,
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
    pub(super) recent_peers: LruCache<TypedKey, RecentPeersEntry>,
    /// Storage for private/safety RouteSpecs
    pub(super) route_spec_store: Option<RouteSpecStore>,
    /// Async tagged critical sections table
    /// Tag: "tick" -> in ticker
    pub(super) critical_sections: AsyncTagLockTable<&'static str>,
}

impl RoutingTableInner {
    pub(super) fn new(unlocked_inner: Arc<RoutingTableUnlockedInner>) -> RoutingTableInner {
        RoutingTableInner {
            unlocked_inner,
            buckets: BTreeMap::new(),
            public_internet_routing_domain: PublicInternetRoutingDomainDetail::default(),
            local_network_routing_domain: LocalNetworkRoutingDomainDetail::default(),
            all_entries: PtrWeakHashSet::new(),
            live_entry_count: BTreeMap::new(),
            self_latency_stats_accounting: LatencyStatsAccounting::new(),
            self_transfer_stats_accounting: TransferStatsAccounting::new(),
            self_transfer_stats: TransferStatsDownUp::default(),
            recent_peers: LruCache::new(RECENT_PEERS_TABLE_SIZE),
            route_spec_store: None,
            critical_sections: AsyncTagLockTable::new(),
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

    pub fn with_routing_domain_mut<F, R>(&mut self, domain: RoutingDomain, f: F) -> R
    where
        F: FnOnce(&mut dyn RoutingDomainDetail) -> R,
    {
        match domain {
            RoutingDomain::PublicInternet => f(&mut self.public_internet_routing_domain),
            RoutingDomain::LocalNetwork => f(&mut self.local_network_routing_domain),
        }
    }

    pub fn relay_node(&self, domain: RoutingDomain) -> Option<NodeRef> {
        self.with_routing_domain(domain, |rd| rd.common().relay_node())
    }

    pub fn relay_node_last_keepalive(&self, domain: RoutingDomain) -> Option<Timestamp> {
        self.with_routing_domain(domain, |rd| rd.common().relay_node_last_keepalive())
    }

    #[allow(dead_code)]
    pub fn has_dial_info(&self, domain: RoutingDomain) -> bool {
        self.with_routing_domain(domain, |rd| !rd.common().dial_info_details().is_empty())
    }

    pub fn dial_info_details(&self, domain: RoutingDomain) -> Vec<DialInfoDetail> {
        self.with_routing_domain(domain, |rd| rd.common().dial_info_details().clone())
    }

    #[cfg_attr(target_arch = "wasm32", allow(dead_code))]
    pub fn first_filtered_dial_info_detail(
        &self,
        routing_domain_set: RoutingDomainSet,
        filter: &DialInfoFilter,
    ) -> Option<DialInfoDetail> {
        if filter.is_dead() || routing_domain_set.is_empty() {
            return None;
        }
        for routing_domain in routing_domain_set {
            let did = self.with_routing_domain(routing_domain, |rd| {
                for did in rd.common().dial_info_details() {
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
            self.with_routing_domain(routing_domain, |rd| {
                for did in rd.common().dial_info_details() {
                    if did.matches_filter(filter) {
                        ret.push(did.clone());
                    }
                }
            });
        }
        ret.remove_duplicates();
        ret
    }

    pub fn ensure_dial_info_is_valid(&self, domain: RoutingDomain, dial_info: &DialInfo) -> bool {
        let address = dial_info.socket_address().address();
        let can_contain_address =
            self.with_routing_domain(domain, |rd| rd.can_contain_address(address));

        if !can_contain_address {
            log_rtab!(debug "can not add dial info to this routing domain");
            return false;
        }
        if !dial_info.is_valid() {
            log_rtab!(debug
                "shouldn't be registering invalid addresses: {:?}",
                dial_info
            );
            return false;
        }
        true
    }

    pub fn node_info_is_valid_in_routing_domain(
        &self,
        routing_domain: RoutingDomain,
        node_info: &NodeInfo,
    ) -> bool {
        // Ensure all of the dial info works in this routing domain
        for did in node_info.dial_info_detail_list() {
            if !self.ensure_dial_info_is_valid(routing_domain, &did.dial_info) {
                return false;
            }
        }
        true
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
        peer_a: &PeerInfo,
        peer_b: &PeerInfo,
        dial_info_filter: DialInfoFilter,
        sequencing: Sequencing,
        dif_sort: Option<Arc<DialInfoDetailSort>>,
    ) -> ContactMethod {
        self.with_routing_domain(routing_domain, |rdd| {
            rdd.get_contact_method(self, peer_a, peer_b, dial_info_filter, sequencing, dif_sort)
        })
    }

    pub fn reset_all_updated_since_last_network_change(&mut self) {
        let cur_ts = get_aligned_timestamp();
        self.with_entries_mut(cur_ts, BucketEntryState::Dead, |rti, v| {
            v.with_mut(rti, |_rti, e| {
                e.reset_updated_since_last_network_change();
            });
            Option::<()>::None
        });
    }

    /// Return if this routing domain has a valid network class
    pub fn has_valid_network_class(&self, routing_domain: RoutingDomain) -> bool {
        self.with_routing_domain(routing_domain, |rdd| rdd.common().has_valid_network_class())
    }

    /// Return a copy of our node's peerinfo
    pub fn get_own_peer_info(&self, routing_domain: RoutingDomain) -> PeerInfo {
        self.with_routing_domain(routing_domain, |rdd| {
            rdd.common().with_peer_info(self, |pi| pi.clone())
        })
    }

    /// Return our current node info timestamp
    pub fn get_own_node_info_ts(&self, routing_domain: RoutingDomain) -> Timestamp {
        self.with_routing_domain(routing_domain, |rdd| {
            rdd.common()
                .with_peer_info(self, |pi| pi.signed_node_info().timestamp())
        })
    }

    /// Return the domain's currently registered network class
    pub fn get_network_class(&self, routing_domain: RoutingDomain) -> Option<NetworkClass> {
        self.with_routing_domain(routing_domain, |rdd| rdd.common().network_class())
    }

    /// Return the domain's filter for what we can receivein the form of a dial info filter
    #[allow(dead_code)]
    pub fn get_inbound_dial_info_filter(&self, routing_domain: RoutingDomain) -> DialInfoFilter {
        self.with_routing_domain(routing_domain, |rdd| {
            rdd.common().inbound_dial_info_filter()
        })
    }

    /// Return the domain's filter for what we can receive in the form of a node ref filter
    #[allow(dead_code)]
    pub fn get_inbound_node_ref_filter(&self, routing_domain: RoutingDomain) -> NodeRefFilter {
        let dif = self.get_inbound_dial_info_filter(routing_domain);
        NodeRefFilter::new()
            .with_routing_domain(routing_domain)
            .with_dial_info_filter(dif)
    }

    /// Return the domain's filter for what we can send out in the form of a dial info filter
    pub fn get_outbound_dial_info_filter(&self, routing_domain: RoutingDomain) -> DialInfoFilter {
        self.with_routing_domain(routing_domain, |rdd| {
            rdd.common().outbound_dial_info_filter()
        })
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
                let bucket = Bucket::new(ck);
                ckbuckets.push(bucket);
            }
            self.buckets.insert(ck, ckbuckets);
        }
    }

    #[cfg_attr(target_arch = "wasm32", allow(dead_code))]
    pub fn configure_local_network_routing_domain(
        &mut self,
        local_networks: Vec<(IpAddr, IpAddr)>,
    ) {
        log_net!(debug "configure_local_network_routing_domain: {:#?}", local_networks);

        let changed = self
            .local_network_routing_domain
            .set_local_networks(local_networks);

        // If the local network topology has changed, nuke the existing local node info and let new local discovery happen
        if changed {
            let cur_ts = get_aligned_timestamp();
            self.with_entries_mut(cur_ts, BucketEntryState::Dead, |rti, e| {
                e.with_mut(rti, |_rti, e| {
                    e.clear_signed_node_info(RoutingDomain::LocalNetwork);
                    e.reset_updated_since_last_network_change();
                });
                Option::<()>::None
            });
        }
    }

    /// Attempt to empty the routing table
    /// should only be performed when there are no node_refs (detached)
    pub fn purge_buckets(&mut self) {
        log_rtab!(
            "Starting routing table buckets purge. Table currently has {} nodes",
            self.bucket_entry_count()
        );
        for ck in VALID_CRYPTO_KINDS {
            for bucket in self.buckets.get_mut(&ck).unwrap().iter_mut() {
                bucket.kick(0);
            }
        }
        self.all_entries.remove_expired();

        log_rtab!(debug
            "Routing table buckets purge complete. Routing table now has {} nodes",
            self.bucket_entry_count()
        );
    }

    /// Attempt to remove last_connections from entries
    pub fn purge_last_connections(&mut self) {
        log_rtab!(
            "Starting routing table last_connections purge. Table currently has {} nodes",
            self.bucket_entry_count()
        );
        for ck in VALID_CRYPTO_KINDS {
            for bucket in &self.buckets[&ck] {
                for entry in bucket.entries() {
                    entry.1.with_mut_inner(|e| {
                        e.clear_last_flows();
                    });
                }
            }
        }
        self.all_entries.remove_expired();

        log_rtab!(debug
            "Routing table last_connections purge complete. Routing table now has {} nodes",
            self.bucket_entry_count()
        );
    }

    /// Attempt to settle buckets and remove entries down to the desired number
    /// which may not be possible due extant NodeRefs
    pub fn kick_bucket(&mut self, bucket_index: BucketIndex) {
        let bucket = self.get_bucket_mut(bucket_index);
        let bucket_depth = Self::bucket_depth(bucket_index);

        if let Some(_dead_node_ids) = bucket.kick(bucket_depth) {
            // Remove expired entries
            self.all_entries.remove_expired();

            log_rtab!(debug "Bucket {}:{} kicked Routing table now has {} nodes", bucket_index.0, bucket_index.1, self.bucket_entry_count());

            // Now purge the routing table inner vectors
            //let filter = |k: &DHTKey| dead_node_ids.contains(k);
            //inner.closest_reliable_nodes.retain(filter);
            //inner.fastest_reliable_nodes.retain(filter);
            //inner.closest_nodes.retain(filter);
            //inner.fastest_nodes.retain(filter);
        }
    }

    /// Build the counts of entries per routing domain and crypto kind and cache them
    /// Only considers entries that have valid signed node info
    pub fn refresh_cached_entry_counts(&mut self) -> EntryCounts {
        self.live_entry_count.clear();
        let cur_ts = get_aligned_timestamp();
        self.with_entries_mut(cur_ts, BucketEntryState::Unreliable, |rti, entry| {
            entry.with_inner(|e| {
                // Tally per routing domain and crypto kind
                for rd in RoutingDomain::all() {
                    if let Some(sni) = e.signed_node_info(rd) {
                        // Only consider entries that have valid signed node info in this domain
                        if sni.has_any_signature() {
                            // Tally
                            for crypto_kind in e.crypto_kinds() {
                                rti.live_entry_count
                                    .entry((rd, crypto_kind))
                                    .and_modify(|x| *x += 1)
                                    .or_insert(1);
                            }
                        }
                    }
                }
            });
            Option::<()>::None
        });
        self.live_entry_count.clone()
    }

    /// Return the last cached entry counts
    /// Only considers entries that have valid signed node info
    pub fn cached_entry_counts(&self) -> EntryCounts {
        self.live_entry_count.clone()
    }

    /// Count entries that match some criteria
    pub fn get_entry_count(
        &self,
        routing_domain_set: RoutingDomainSet,
        min_state: BucketEntryState,
        crypto_kinds: &[CryptoKind],
    ) -> usize {
        let mut count = 0usize;
        let cur_ts = get_aligned_timestamp();
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

    pub(super) fn get_nodes_needing_ping(
        &self,
        outer_self: RoutingTable,
        routing_domain: RoutingDomain,
        cur_ts: Timestamp,
    ) -> Vec<NodeRef> {
        let own_node_info_ts = self.get_own_node_info_ts(routing_domain);

        // Collect all entries that are 'needs_ping' and have some node info making them reachable somehow
        let mut node_refs = Vec::<NodeRef>::with_capacity(self.bucket_entry_count());
        self.with_entries(cur_ts, BucketEntryState::Unreliable, |rti, entry| {
            let entry_needs_ping = |e: &BucketEntryInner| {
                // If this entry isn't in the routing domain we are checking, don't include it
                if !e.exists_in_routing_domain(rti, routing_domain) {
                    return false;
                }

                // If we don't have node status for this node, then we should ping it to get some node status
                if e.has_node_info(routing_domain.into()) && e.node_status(routing_domain).is_none()
                {
                    return true;
                }

                // If this entry needs a ping because this node hasn't seen our latest node info, then do it
                if !e.has_seen_our_node_info_ts(routing_domain, own_node_info_ts) {
                    return true;
                }

                // If this entry needs need a ping by non-routing-domain-specific metrics then do it
                if e.needs_ping(cur_ts) {
                    return true;
                }

                false
            };

            if entry.with_inner(entry_needs_ping) {
                node_refs.push(NodeRef::new(
                    outer_self.clone(),
                    entry,
                    Some(NodeRefFilter::new().with_routing_domain(routing_domain)),
                ));
            }
            Option::<()>::None
        });
        node_refs
    }

    #[allow(dead_code)]
    pub fn get_all_nodes(&self, outer_self: RoutingTable, cur_ts: Timestamp) -> Vec<NodeRef> {
        let mut node_refs = Vec::<NodeRef>::with_capacity(self.bucket_entry_count());
        self.with_entries(cur_ts, BucketEntryState::Unreliable, |_rti, entry| {
            node_refs.push(NodeRef::new(outer_self.clone(), entry, None));
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
    fn update_bucket_entries(
        &mut self,
        entry: Arc<BucketEntry>,
        node_ids: &[TypedKey],
    ) -> EyreResult<()> {
        entry.with_mut_inner(|e| {
            let existing_node_ids = e.node_ids();
            for node_id in node_ids {
                // Skip node ids that exist already
                if existing_node_ids.contains(node_id) {
                    continue;
                }

                // Add new node id to entry
                let ck = node_id.kind;
                if let Some(old_node_id) = e.add_node_id(*node_id)? {
                    // Remove any old node id for this crypto kind
                    if VALID_CRYPTO_KINDS.contains(&ck) {
                        let bucket_index = self.unlocked_inner.calculate_bucket_index(&old_node_id);
                        let bucket = self.get_bucket_mut(bucket_index);
                        bucket.remove_entry(&old_node_id.value);
                        self.unlocked_inner.kick_queue.lock().insert(bucket_index);
                    }
                }

                // Bucket the entry appropriately
                if VALID_CRYPTO_KINDS.contains(&ck) {
                    let bucket_index = self.unlocked_inner.calculate_bucket_index(node_id);
                    let bucket = self.get_bucket_mut(bucket_index);
                    bucket.add_existing_entry(node_id.value, entry.clone());

                    // Kick bucket
                    self.unlocked_inner.kick_queue.lock().insert(bucket_index);
                }
            }
            Ok(())
        })
    }

    /// Create a node reference, possibly creating a bucket entry
    /// the 'update_func' closure is called on the node, and, if created,
    /// in a locked fashion as to ensure the bucket entry state is always valid
    fn create_node_ref<F>(
        &mut self,
        outer_self: RoutingTable,
        node_ids: &TypedKeyGroup,
        update_func: F,
    ) -> EyreResult<NodeRef>
    where
        F: FnOnce(&mut RoutingTableInner, &mut BucketEntryInner),
    {
        // Ensure someone isn't trying register this node itself
        if self.unlocked_inner.matches_own_node_id(node_ids) {
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
            let bucket_index = self.unlocked_inner.calculate_bucket_index(node_id);
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
            if let Err(e) = self.update_bucket_entries(best_entry.clone(), node_ids) {
                bail!("Not registering new ids for existing node: {}", e);
            }

            // Make a noderef to return
            let nr = NodeRef::new(outer_self.clone(), best_entry.clone(), None);

            // Update the entry with the update func
            best_entry.with_mut_inner(|e| update_func(self, e));

            // Return the noderef
            return Ok(nr);
        }

        // If no entry exists yet, add the first entry to a bucket, possibly evicting a bucket member
        let first_node_id = node_ids[0];
        let bucket_entry = self.unlocked_inner.calculate_bucket_index(&first_node_id);
        let bucket = self.get_bucket_mut(bucket_entry);
        let new_entry = bucket.add_new_entry(first_node_id.value);
        self.all_entries.insert(new_entry.clone());
        self.unlocked_inner.kick_queue.lock().insert(bucket_entry);

        // Update the other bucket entries with the remaining node ids
        if let Err(e) = self.update_bucket_entries(new_entry.clone(), node_ids) {
            bail!("Not registering new node: {}", e);
        }

        // Make node ref to return
        let nr = NodeRef::new(outer_self.clone(), new_entry.clone(), None);

        // Update the entry with the update func
        new_entry.with_mut_inner(|e| update_func(self, e));

        // Kick the bucket
        log_rtab!(debug "Routing table now has {} nodes, {} live", self.bucket_entry_count(), self.get_entry_count(RoutingDomainSet::all(), BucketEntryState::Unreliable, &VALID_CRYPTO_KINDS));

        Ok(nr)
    }

    /// Resolve an existing routing table entry using any crypto kind and return a reference to it
    pub fn lookup_any_node_ref(
        &self,
        outer_self: RoutingTable,
        node_id_key: PublicKey,
    ) -> EyreResult<Option<NodeRef>> {
        for ck in VALID_CRYPTO_KINDS {
            if let Some(nr) =
                self.lookup_node_ref(outer_self.clone(), TypedKey::new(ck, node_id_key))?
            {
                return Ok(Some(nr));
            }
        }
        Ok(None)
    }

    /// Resolve an existing routing table entry and return a reference to it
    pub fn lookup_node_ref(
        &self,
        outer_self: RoutingTable,
        node_id: TypedKey,
    ) -> EyreResult<Option<NodeRef>> {
        if self.unlocked_inner.matches_own_node_id(&[node_id]) {
            bail!("can't look up own node id in routing table");
        }
        if !VALID_CRYPTO_KINDS.contains(&node_id.kind) {
            bail!("can't look up node id with invalid crypto kind");
        }

        let bucket_index = self.unlocked_inner.calculate_bucket_index(&node_id);
        let bucket = self.get_bucket(bucket_index);
        Ok(bucket
            .entry(&node_id.value)
            .map(|e| NodeRef::new(outer_self, e, None)))
    }

    /// Resolve an existing routing table entry and return a filtered reference to it
    pub fn lookup_and_filter_noderef(
        &self,
        outer_self: RoutingTable,
        node_id: TypedKey,
        routing_domain_set: RoutingDomainSet,
        dial_info_filter: DialInfoFilter,
    ) -> EyreResult<Option<NodeRef>> {
        let nr = self.lookup_node_ref(outer_self, node_id)?;
        Ok(nr.map(|nr| {
            nr.filtered_clone(
                NodeRefFilter::new()
                    .with_dial_info_filter(dial_info_filter)
                    .with_routing_domain_set(routing_domain_set),
            )
        }))
    }

    /// Resolve an existing routing table entry and call a function on its entry without using a noderef
    pub fn with_node_entry<F, R>(&self, node_id: TypedKey, f: F) -> Option<R>
    where
        F: FnOnce(Arc<BucketEntry>) -> R,
    {
        if self.unlocked_inner.matches_own_node_id(&[node_id]) {
            log_rtab!(error "can't look up own node id in routing table");
            return None;
        }
        if !VALID_CRYPTO_KINDS.contains(&node_id.kind) {
            log_rtab!(error "can't look up node id with invalid crypto kind");
            return None;
        }
        let bucket_entry = self.unlocked_inner.calculate_bucket_index(&node_id);
        let bucket = self.get_bucket(bucket_entry);
        bucket.entry(&node_id.value).map(f)
    }

    /// Shortcut function to add a node to our routing table if it doesn't exist
    /// and add the dial info we have for it. Returns a noderef filtered to
    /// the routing domain in which this node was registered for convenience.
    pub fn register_node_with_peer_info(
        &mut self,
        outer_self: RoutingTable,
        routing_domain: RoutingDomain,
        peer_info: PeerInfo,
        allow_invalid: bool,
    ) -> EyreResult<NodeRef> {
        // if our own node is in the list, then ignore it as we don't add ourselves to our own routing table
        if self
            .unlocked_inner
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
        if let Some(relay_peer_info) = peer_info.signed_node_info().relay_peer_info() {
            if !self
                .unlocked_inner
                .matches_own_node_id(relay_peer_info.node_ids())
            {
                self.register_node_with_peer_info(
                    outer_self.clone(),
                    routing_domain,
                    relay_peer_info,
                    false,
                )?;
            }
        }

        let (node_ids, signed_node_info) = peer_info.destructure();
        let mut nr = self.create_node_ref(outer_self, &node_ids, |_rti, e| {
            e.update_signed_node_info(routing_domain, signed_node_info);
        })?;

        nr.set_filter(Some(
            NodeRefFilter::new().with_routing_domain(routing_domain),
        ));

        Ok(nr)
    }

    /// Shortcut function to add a node to our routing table if it doesn't exist
    /// and add the last peer address we have for it, since that's pretty common
    pub fn register_node_with_existing_connection(
        &mut self,
        outer_self: RoutingTable,
        node_id: TypedKey,
        flow: Flow,
        timestamp: Timestamp,
    ) -> EyreResult<NodeRef> {
        let nr = self.create_node_ref(outer_self, &TypedKeyGroup::from(node_id), |_rti, e| {
            // this node is live because it literally just connected to us
            e.touch_last_seen(timestamp);
        })?;
        // set the most recent node address for connection finding and udp replies
        nr.locked_mut(self).set_last_flow(flow, timestamp);
        Ok(nr)
    }

    //////////////////////////////////////////////////////////////////////
    // Routing Table Health Metrics

    pub fn get_routing_table_health(&self) -> RoutingTableHealth {
        let mut reliable_entry_count: usize = 0;
        let mut unreliable_entry_count: usize = 0;
        let mut dead_entry_count: usize = 0;

        let cur_ts = get_aligned_timestamp();
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
            }
        }

        // Public internet routing domain is ready for app use,
        // when we have proper dialinfo/networkclass
        let public_internet_ready = !matches!(
            self.get_network_class(RoutingDomain::PublicInternet)
                .unwrap_or_default(),
            NetworkClass::Invalid
        );

        // Local internet routing domain is ready for app use
        // when we have proper dialinfo/networkclass
        let local_network_ready = !matches!(
            self.get_network_class(RoutingDomain::LocalNetwork)
                .unwrap_or_default(),
            NetworkClass::Invalid
        );

        let live_entry_counts = self.cached_entry_counts();

        RoutingTableHealth {
            reliable_entry_count,
            unreliable_entry_count,
            dead_entry_count,
            live_entry_counts,
            public_internet_ready,
            local_network_ready,
        }
    }

    pub fn touch_recent_peer(&mut self, node_id: TypedKey, last_connection: Flow) {
        self.recent_peers
            .insert(node_id, RecentPeersEntry { last_connection });
    }

    //////////////////////////////////////////////////////////////////////
    // Find Nodes

    // Retrieve the fastest nodes in the routing table matching an entry filter
    pub fn find_fast_public_nodes_filtered(
        &self,
        outer_self: RoutingTable,
        node_count: usize,
        mut filters: VecDeque<RoutingTableEntryFilter>,
    ) -> Vec<NodeRef> {
        let public_node_filter =
            Box::new(|_rti: &RoutingTableInner, v: Option<Arc<BucketEntry>>| {
                let entry = v.unwrap();
                entry.with_inner(|e| {
                    // skip nodes on local network
                    if e.node_info(RoutingDomain::LocalNetwork).is_some() {
                        return false;
                    }
                    // skip nodes not on public internet
                    if e.node_info(RoutingDomain::PublicInternet).is_none() {
                        return false;
                    }
                    true
                })
            }) as RoutingTableEntryFilter;
        filters.push_front(public_node_filter);

        self.find_preferred_fastest_nodes(
            node_count,
            filters,
            |_rti: &RoutingTableInner, v: Option<Arc<BucketEntry>>| {
                NodeRef::new(outer_self.clone(), v.unwrap().clone(), None)
            },
        )
    }

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

    pub fn transform_to_peer_info(
        &self,
        routing_domain: RoutingDomain,
        own_peer_info: &PeerInfo,
        entry: Option<Arc<BucketEntry>>,
    ) -> PeerInfo {
        match entry {
            None => own_peer_info.clone(),
            Some(entry) => entry.with_inner(|e| e.make_peer_info(routing_domain).unwrap()),
        }
    }

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
        T: for<'r, 't> FnMut(&'r RoutingTableInner, Option<Arc<BucketEntry>>) -> O,
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

    pub fn find_preferred_fastest_nodes<T, O>(
        &self,
        node_count: usize,
        mut filters: VecDeque<RoutingTableEntryFilter>,
        transform: T,
    ) -> Vec<O>
    where
        T: for<'r> FnMut(&'r RoutingTableInner, Option<Arc<BucketEntry>>) -> O,
    {
        let cur_ts = get_aligned_timestamp();

        // Add filter to remove dead nodes always
        let filter_dead = Box::new(
            move |_rti: &RoutingTableInner, v: Option<Arc<BucketEntry>>| {
                if let Some(entry) = &v {
                    // always filter out dead nodes
                    !entry.with_inner(|e| e.state(cur_ts) == BucketEntryState::Dead)
                } else {
                    // always filter out self peer, as it is irrelevant to the 'fastest nodes' search
                    false
                }
            },
        ) as RoutingTableEntryFilter;
        filters.push_front(filter_dead);

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
                    let ra = ae.check_reliable(cur_ts);
                    let rb = be.check_reliable(cur_ts);
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

    pub fn find_preferred_closest_nodes<T, O>(
        &self,
        node_count: usize,
        node_id: TypedKey,
        mut filters: VecDeque<RoutingTableEntryFilter>,
        transform: T,
    ) -> VeilidAPIResult<Vec<O>>
    where
        T: for<'r> FnMut(&'r RoutingTableInner, Option<Arc<BucketEntry>>) -> O,
    {
        let cur_ts = get_aligned_timestamp();

        // Get the crypto kind
        let crypto_kind = node_id.kind;
        let Some(vcrypto) = self.unlocked_inner.crypto().get(crypto_kind) else {
            apibail_generic!("invalid crypto kind");
        };

        // Filter to ensure entries support the crypto kind in use
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
                .map_or(false, |x| x.with_inner(|x| x.check_reliable(cur_ts)));
            let rb = b_entry
                .as_ref()
                .map_or(false, |x| x.with_inner(|x| x.check_reliable(cur_ts)));
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
                self.unlocked_inner.node_id(crypto_kind)
            };
            let b_key = if let Some(b_entry) = b_entry {
                b_entry.with_inner(|e| e.node_ids().get(crypto_kind).unwrap())
            } else {
                self.unlocked_inner.node_id(crypto_kind)
            };

            // distance is the next metric, closer nodes first
            let da = vcrypto.distance(&a_key.value, &node_id.value);
            let db = vcrypto.distance(&b_key.value, &node_id.value);
            da.cmp(&db)
        };

        let out =
            self.find_peers_with_sort_and_filter(node_count, cur_ts, filters, sort, transform);
        log_rtab!(">> find_closest_nodes: node count = {}", out.len());
        Ok(out)
    }

    pub fn sort_and_clean_closest_noderefs(
        &self,
        node_id: TypedKey,
        closest_nodes: &[NodeRef],
    ) -> Vec<NodeRef> {
        // Lock all noderefs
        let kind = node_id.kind;
        let mut closest_nodes_locked: Vec<NodeRefLocked> = closest_nodes
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
        let sort = make_closest_noderef_sort(self.unlocked_inner.crypto(), node_id);
        closest_nodes_locked.sort_by(sort);

        // Unlock noderefs
        closest_nodes_locked.iter().map(|x| x.unlocked()).collect()
    }
}

fn make_closest_noderef_sort(
    crypto: Crypto,
    node_id: TypedKey,
) -> impl Fn(&NodeRefLocked, &NodeRefLocked) -> core::cmp::Ordering {
    let kind = node_id.kind;
    // Get cryptoversion to check distance with
    let vcrypto = crypto.get(kind).unwrap();

    move |a: &NodeRefLocked, b: &NodeRefLocked| -> core::cmp::Ordering {
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
                let da = vcrypto.distance(&a_key.value, &node_id.value);
                let db = vcrypto.distance(&b_key.value, &node_id.value);
                da.cmp(&db)
            })
        })
    }
}
