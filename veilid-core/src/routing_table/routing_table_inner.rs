use super::*;

const RECENT_PEERS_TABLE_SIZE: usize = 64;

//////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy)]
pub struct RecentPeersEntry {
    pub last_connection: ConnectionDescriptor,
}

/// RoutingTable rwlock-internal data
pub struct RoutingTableInner {
    /// Extra pointer to unlocked members to simplify access
    pub(super) unlocked_inner: Arc<RoutingTableUnlockedInner>,
    /// Routing table buckets that hold entries
    pub(super) buckets: Vec<Bucket>,
    /// A fast counter for the number of entries in the table, total
    pub(super) bucket_entry_count: usize,
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
    pub(super) recent_peers: LruCache<DHTKey, RecentPeersEntry>,
    /// Storage for private/safety RouteSpecs
    pub(super) route_spec_store: RouteSpecStore,
}

impl RoutingTableInner {
    pub fn new(unlocked_inner: Arc<RoutingTableUnlockedInner>) -> RoutingTableInner {
        RoutingTableInner {
            unlocked_inner,
            buckets: Vec::new(),
            public_internet_routing_domain: PublicInternetRoutingDomainDetail::default(),
            local_network_routing_domain: LocalNetworkRoutingDomainDetail::default(),
            bucket_entry_count: 0,
            self_latency_stats_accounting: LatencyStatsAccounting::new(),
            self_transfer_stats_accounting: TransferStatsAccounting::new(),
            self_transfer_stats: TransferStatsDownUp::default(),
            recent_peers: LruCache::new(RECENT_PEERS_TABLE_SIZE),
            route_spec_store: RouteSpecStore::new(unlocked_inner.config.clone()),
        }
    }

    pub fn network_manager(&self) -> NetworkManager {
        self.unlocked_inner.network_manager.clone()
    }
    pub fn rpc_processor(&self) -> RPCProcessor {
        self.network_manager().rpc_processor()
    }

    pub fn node_id(&self) -> DHTKey {
        self.unlocked_inner.node_id
    }

    pub fn node_id_secret(&self) -> DHTKeySecret {
        self.unlocked_inner.node_id_secret
    }

    pub fn config(&self) -> VeilidConfig {
        self.unlocked_inner.config.clone()
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

    pub fn with_route_spec_store_mut<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut RouteSpecStore, &mut RoutingTableInner) -> R,
    {
        f(&mut self.route_spec_store, self)
    }

    pub fn with_route_spec_store<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&RouteSpecStore, &RoutingTableInner) -> R,
    {
        f(&self.route_spec_store, self)
    }

    pub fn relay_node(&self, domain: RoutingDomain) -> Option<NodeRef> {
        self.with_routing_domain(domain, |rd| rd.common().relay_node())
    }

    pub fn has_dial_info(&self, domain: RoutingDomain) -> bool {
        self.with_routing_domain(domain, |rd| !rd.common().dial_info_details().is_empty())
    }

    pub fn dial_info_details(&self, domain: RoutingDomain) -> Vec<DialInfoDetail> {
        self.with_routing_domain(domain, |rd| rd.common().dial_info_details().clone())
    }

    pub fn first_filtered_dial_info_detail(
        &self,
        routing_domain_set: RoutingDomainSet,
        filter: &DialInfoFilter,
    ) -> Option<DialInfoDetail> {
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
        // Should not be passing around nodeinfo with an invalid network class
        if matches!(node_info.network_class, NetworkClass::Invalid) {
            return false;
        }
        // Ensure all of the dial info works in this routing domain
        for did in &node_info.dial_info_detail_list {
            if !self.ensure_dial_info_is_valid(routing_domain, &did.dial_info) {
                return false;
            }
        }
        // Ensure the relay is also valid in this routing domain if it is provided
        if let Some(relay_peer_info) = node_info.relay_peer_info.as_ref() {
            let relay_ni = &relay_peer_info.signed_node_info.node_info;
            if !self.node_info_is_valid_in_routing_domain(routing_domain, relay_ni) {
                return false;
            }
        }
        true
    }

    #[instrument(level = "trace", skip(self), ret)]
    pub fn get_contact_method(
        &self,
        routing_domain: RoutingDomain,
        node_a_id: &DHTKey,
        node_a: &NodeInfo,
        node_b_id: &DHTKey,
        node_b: &NodeInfo,
        dial_info_filter: DialInfoFilter,
        sequencing: Sequencing,
    ) -> ContactMethod {
        self.with_routing_domain(routing_domain, |rdd| {
            rdd.get_contact_method(
                self,
                node_a_id,
                node_a,
                node_b_id,
                node_b,
                dial_info_filter,
                sequencing,
            )
        })
    }

    pub fn reset_all_seen_our_node_info(&mut self, routing_domain: RoutingDomain) {
        let cur_ts = intf::get_timestamp();
        self.with_entries_mut(cur_ts, BucketEntryState::Dead, |rti, _, v| {
            v.with_mut(rti, |_rti, e| {
                e.set_seen_our_node_info(routing_domain, false);
            });
            Option::<()>::None
        });
    }

    pub fn reset_all_updated_since_last_network_change(&mut self) {
        let cur_ts = intf::get_timestamp();
        self.with_entries_mut(cur_ts, BucketEntryState::Dead, |rti, _, v| {
            v.with_mut(rti, |_rti, e| {
                e.set_updated_since_last_network_change(false)
            });
            Option::<()>::None
        });
    }

    /// Return a copy of our node's peerinfo
    pub fn get_own_peer_info(&self, routing_domain: RoutingDomain) -> PeerInfo {
        self.with_routing_domain(routing_domain, |rdd| {
            rdd.common().with_peer_info(|pi| pi.clone())
        })
    }

    /// Return a copy of our node's signednodeinfo
    pub fn get_own_signed_node_info(&self, routing_domain: RoutingDomain) -> SignedNodeInfo {
        self.with_routing_domain(routing_domain, |rdd| {
            rdd.common()
                .with_peer_info(|pi| pi.signed_node_info.clone())
        })
    }

    /// Return a copy of our node's nodeinfo
    pub fn get_own_node_info(&self, routing_domain: RoutingDomain) -> NodeInfo {
        self.with_routing_domain(routing_domain, |rdd| {
            rdd.common()
                .with_peer_info(|pi| pi.signed_node_info.node_info.clone())
        })
    }

    /// Return our currently registered network class
    pub fn has_valid_own_node_info(&self, routing_domain: RoutingDomain) -> bool {
        self.with_routing_domain(routing_domain, |rdd| rdd.common().has_valid_own_node_info())
    }

    /// Return the domain's currently registered network class
    pub fn get_network_class(&self, routing_domain: RoutingDomain) -> Option<NetworkClass> {
        self.with_routing_domain(routing_domain, |rdd| rdd.common().network_class())
    }

    /// Return the domain's filter for what we can receivein the form of a dial info filter
    pub fn get_inbound_dial_info_filter(&self, routing_domain: RoutingDomain) -> DialInfoFilter {
        self.with_routing_domain(routing_domain, |rdd| {
            rdd.common().inbound_dial_info_filter()
        })
    }

    /// Return the domain's filter for what we can receive in the form of a node ref filter
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

    fn bucket_depth(index: usize) -> usize {
        match index {
            0 => 256,
            1 => 128,
            2 => 64,
            3 => 32,
            4 => 16,
            5 => 8,
            6 => 4,
            7 => 4,
            8 => 4,
            9 => 4,
            _ => 4,
        }
    }

    pub fn init(&mut self, routing_table: RoutingTable) -> EyreResult<()> {
        // Size the buckets (one per bit)
        self.buckets.reserve(DHT_KEY_LENGTH * 8);
        for _ in 0..DHT_KEY_LENGTH * 8 {
            let bucket = Bucket::new(routing_table.clone());
            self.buckets.push(bucket);
        }
        Ok(())
    }

    pub fn terminate(&mut self) {
        //
    }

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
            let cur_ts = intf::get_timestamp();
            self.with_entries_mut(cur_ts, BucketEntryState::Dead, |rti, _, e| {
                e.with_mut(rti, |_rti, e| {
                    e.clear_signed_node_info(RoutingDomain::LocalNetwork);
                    e.set_seen_our_node_info(RoutingDomain::LocalNetwork, false);
                    e.set_updated_since_last_network_change(false);
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
            self.bucket_entry_count
        );
        for bucket in &self.buckets {
            bucket.kick(self, 0);
        }
        log_rtab!(debug
             "Routing table buckets purge complete. Routing table now has {} nodes",
            self.bucket_entry_count
        );
    }

    /// Attempt to remove last_connections from entries
    pub fn purge_last_connections(&mut self) {
        log_rtab!(
            "Starting routing table last_connections purge. Table currently has {} nodes",
            self.bucket_entry_count
        );
        for bucket in &self.buckets {
            for entry in bucket.entries() {
                entry.1.with_mut(self, |_rti, e| {
                    e.clear_last_connections();
                });
            }
        }
        log_rtab!(debug
             "Routing table last_connections purge complete. Routing table now has {} nodes",
             self.bucket_entry_count
        );
    }

    /// Attempt to settle buckets and remove entries down to the desired number
    /// which may not be possible due extant NodeRefs
    pub fn kick_bucket(&mut self, idx: usize) {
        let bucket = &mut self.buckets[idx];
        let bucket_depth = Self::bucket_depth(idx);

        if let Some(dead_node_ids) = bucket.kick(self, bucket_depth) {
            // Remove counts
            self.bucket_entry_count -= dead_node_ids.len();
            log_rtab!(debug "Routing table now has {} nodes", self.bucket_entry_count);

            // Now purge the routing table inner vectors
            //let filter = |k: &DHTKey| dead_node_ids.contains(k);
            //inner.closest_reliable_nodes.retain(filter);
            //inner.fastest_reliable_nodes.retain(filter);
            //inner.closest_nodes.retain(filter);
            //inner.fastest_nodes.retain(filter);
        }
    }

    pub fn find_bucket_index(&self, node_id: DHTKey) -> usize {
        distance(&node_id, &self.unlocked_inner.node_id)
            .first_nonzero_bit()
            .unwrap()
    }

    pub fn get_entry_count(
        &self,
        routing_domain_set: RoutingDomainSet,
        min_state: BucketEntryState,
    ) -> usize {
        let mut count = 0usize;
        let cur_ts = intf::get_timestamp();
        self.with_entries(cur_ts, min_state, |rti, _, e| {
            if e.with(rti, |_rti, e| e.best_routing_domain(routing_domain_set))
                .is_some()
            {
                count += 1;
            }
            Option::<()>::None
        });
        count
    }

    pub fn with_entries<T, F: FnMut(&RoutingTableInner, DHTKey, Arc<BucketEntry>) -> Option<T>>(
        &self,
        cur_ts: u64,
        min_state: BucketEntryState,
        mut f: F,
    ) -> Option<T> {
        for bucket in &self.buckets {
            for entry in bucket.entries() {
                if entry.1.with(self, |_rti, e| e.state(cur_ts) >= min_state) {
                    if let Some(out) = f(self, *entry.0, entry.1.clone()) {
                        return Some(out);
                    }
                }
            }
        }
        None
    }

    pub fn with_entries_mut<
        T,
        F: FnMut(&mut RoutingTableInner, DHTKey, Arc<BucketEntry>) -> Option<T>,
    >(
        &mut self,
        cur_ts: u64,
        min_state: BucketEntryState,
        mut f: F,
    ) -> Option<T> {
        for bucket in &self.buckets {
            for entry in bucket.entries() {
                if entry.1.with(self, |_rti, e| e.state(cur_ts) >= min_state) {
                    if let Some(out) = f(self, *entry.0, entry.1.clone()) {
                        return Some(out);
                    }
                }
            }
        }
        None
    }

    pub fn get_nodes_needing_updates(
        &self,
        outer_self: RoutingTable,
        routing_domain: RoutingDomain,
        cur_ts: u64,
        all: bool,
    ) -> Vec<NodeRef> {
        let mut node_refs = Vec::<NodeRef>::with_capacity(self.bucket_entry_count);
        self.with_entries(cur_ts, BucketEntryState::Unreliable, |rti, k, v| {
            // Only update nodes that haven't seen our node info yet
            if all || !v.with(rti, |_rti, e| e.has_seen_our_node_info(routing_domain)) {
                node_refs.push(NodeRef::new(
                    outer_self.clone(),
                    k,
                    v,
                    Some(NodeRefFilter::new().with_routing_domain(routing_domain)),
                ));
            }
            Option::<()>::None
        });
        node_refs
    }

    pub fn get_nodes_needing_ping(
        &self,
        outer_self: RoutingTable,
        routing_domain: RoutingDomain,
        cur_ts: u64,
    ) -> Vec<NodeRef> {
        // Collect relay nodes
        let opt_relay_id = self.with_routing_domain(routing_domain, |rd| {
            rd.common().relay_node().map(|rn| rn.node_id())
        });

        // Collect all entries that are 'needs_ping' and have some node info making them reachable somehow
        let mut node_refs = Vec::<NodeRef>::with_capacity(self.bucket_entry_count);
        self.with_entries(cur_ts, BucketEntryState::Unreliable, |rti, k, v| {
            if v.with(rti, |_rti, e| {
                e.has_node_info(routing_domain.into())
                    && e.needs_ping(cur_ts, opt_relay_id == Some(k))
            }) {
                node_refs.push(NodeRef::new(
                    outer_self.clone(),
                    k,
                    v,
                    Some(NodeRefFilter::new().with_routing_domain(routing_domain)),
                ));
            }
            Option::<()>::None
        });
        node_refs
    }

    pub fn get_all_nodes(&self, outer_self: RoutingTable, cur_ts: u64) -> Vec<NodeRef> {
        let mut node_refs = Vec::<NodeRef>::with_capacity(self.bucket_entry_count);
        self.with_entries(cur_ts, BucketEntryState::Unreliable, |_rti, k, v| {
            node_refs.push(NodeRef::new(outer_self.clone(), k, v, None));
            Option::<()>::None
        });
        node_refs
    }

    /// Create a node reference, possibly creating a bucket entry
    /// the 'update_func' closure is called on the node, and, if created,
    /// in a locked fashion as to ensure the bucket entry state is always valid
    pub fn create_node_ref<F>(
        &mut self,
        outer_self: RoutingTable,
        node_id: DHTKey,
        update_func: F,
    ) -> Option<NodeRef>
    where
        F: FnOnce(&mut RoutingTableInner, &mut BucketEntryInner),
    {
        // Ensure someone isn't trying register this node itself
        if node_id == self.node_id() {
            log_rtab!(debug "can't register own node");
            return None;
        }

        // Look up existing entry
        let idx = self.find_bucket_index(node_id);
        let noderef = {
            let bucket = &self.buckets[idx];
            let entry = bucket.entry(&node_id);
            entry.map(|e| NodeRef::new(outer_self.clone(), node_id, e, None))
        };

        // If one doesn't exist, insert into bucket, possibly evicting a bucket member
        let noderef = match noderef {
            None => {
                // Make new entry
                self.bucket_entry_count += 1;
                let cnt = self.bucket_entry_count;
                let bucket = &mut self.buckets[idx];
                let nr = bucket.add_entry(node_id);

                // Update the entry
                let entry = bucket.entry(&node_id).unwrap();
                entry.with_mut(self, update_func);

                // Kick the bucket
                self.unlocked_inner.kick_queue.lock().insert(idx);
                log_rtab!(debug "Routing table now has {} nodes, {} live", cnt, self.get_entry_count(RoutingDomainSet::all(), BucketEntryState::Unreliable));

                nr
            }
            Some(nr) => {
                // Update the entry
                let bucket = &mut self.buckets[idx];
                let entry = bucket.entry(&node_id).unwrap();
                entry.with_mut(self, update_func);

                nr
            }
        };

        Some(noderef)
    }

    /// Resolve an existing routing table entry and return a reference to it
    pub fn lookup_node_ref(&self, outer_self: RoutingTable, node_id: DHTKey) -> Option<NodeRef> {
        if node_id == self.unlocked_inner.node_id {
            log_rtab!(error "can't look up own node id in routing table");
            return None;
        }
        let idx = self.find_bucket_index(node_id);
        let bucket = &self.buckets[idx];
        bucket
            .entry(&node_id)
            .map(|e| NodeRef::new(outer_self, node_id, e, None))
    }

    /// Resolve an existing routing table entry and return a filtered reference to it
    pub fn lookup_and_filter_noderef(
        &self,
        outer_self: RoutingTable,
        node_id: DHTKey,
        routing_domain_set: RoutingDomainSet,
        dial_info_filter: DialInfoFilter,
    ) -> Option<NodeRef> {
        let nr = self.lookup_node_ref(outer_self, node_id)?;
        Some(
            nr.filtered_clone(
                NodeRefFilter::new()
                    .with_dial_info_filter(dial_info_filter)
                    .with_routing_domain_set(routing_domain_set),
            ),
        )
    }

    /// Resolve an existing routing table entry and call a function on its entry without using a noderef
    pub fn with_node_entry<F, R>(&self, node_id: DHTKey, f: F) -> Option<R>
    where
        F: FnOnce(Arc<BucketEntry>) -> R,
    {
        if node_id == self.unlocked_inner.node_id {
            log_rtab!(error "can't look up own node id in routing table");
            return None;
        }
        let idx = self.find_bucket_index(node_id);
        let bucket = &self.buckets[idx];
        if let Some(e) = bucket.entry(&node_id) {
            return Some(f(e));
        }
        None
    }

    /// Shortcut function to add a node to our routing table if it doesn't exist
    /// and add the dial info we have for it. Returns a noderef filtered to
    /// the routing domain in which this node was registered for convenience.
    pub fn register_node_with_signed_node_info(
        &mut self,
        outer_self: RoutingTable,
        routing_domain: RoutingDomain,
        node_id: DHTKey,
        signed_node_info: SignedNodeInfo,
        allow_invalid: bool,
    ) -> Option<NodeRef> {
        // validate signed node info is not something malicious
        if node_id == self.node_id() {
            log_rtab!(debug "can't register own node id in routing table");
            return None;
        }
        if let Some(rpi) = &signed_node_info.node_info.relay_peer_info {
            if rpi.node_id.key == node_id {
                log_rtab!(debug "node can not be its own relay");
                return None;
            }
        }
        if !allow_invalid {
            // verify signature
            if !signed_node_info.has_valid_signature() {
                log_rtab!(debug "signed node info for {} has invalid signature", node_id);
                return None;
            }
            // verify signed node info is valid in this routing domain
            if !self
                .node_info_is_valid_in_routing_domain(routing_domain, &signed_node_info.node_info)
            {
                log_rtab!(debug "signed node info for {} not valid in the {:?} routing domain", node_id, routing_domain);
                return None;
            }
        }

        self.create_node_ref(outer_self, node_id, |_rti, e| {
            e.update_signed_node_info(routing_domain, signed_node_info);
        })
        .map(|mut nr| {
            nr.set_filter(Some(
                NodeRefFilter::new().with_routing_domain(routing_domain),
            ));
            nr
        })
    }

    /// Shortcut function to add a node to our routing table if it doesn't exist
    /// and add the last peer address we have for it, since that's pretty common
    pub fn register_node_with_existing_connection(
        &mut self,
        outer_self: RoutingTable,
        node_id: DHTKey,
        descriptor: ConnectionDescriptor,
        timestamp: u64,
    ) -> Option<NodeRef> {
        let out = self.create_node_ref(outer_self, node_id, |_rti, e| {
            // this node is live because it literally just connected to us
            e.touch_last_seen(timestamp);
        });
        if let Some(nr) = &out {
            // set the most recent node address for connection finding and udp replies
            nr.set_last_connection(descriptor, timestamp);
        }
        out
    }

    //////////////////////////////////////////////////////////////////////
    // Routing Table Health Metrics

    pub fn get_routing_table_health(&self) -> RoutingTableHealth {
        let mut health = RoutingTableHealth::default();
        let cur_ts = intf::get_timestamp();
        for bucket in &self.buckets {
            for (_, v) in bucket.entries() {
                match v.with(self, |_rti, e| e.state(cur_ts)) {
                    BucketEntryState::Reliable => {
                        health.reliable_entry_count += 1;
                    }
                    BucketEntryState::Unreliable => {
                        health.unreliable_entry_count += 1;
                    }
                    BucketEntryState::Dead => {
                        health.dead_entry_count += 1;
                    }
                }
            }
        }
        health
    }

    pub fn get_recent_peers(
        &mut self,
        outer_self: RoutingTable,
    ) -> Vec<(DHTKey, RecentPeersEntry)> {
        let mut recent_peers = Vec::new();
        let mut dead_peers = Vec::new();
        let mut out = Vec::new();

        // collect all recent peers
        for (k, _v) in &self.recent_peers {
            recent_peers.push(*k);
        }

        // look up each node and make sure the connection is still live
        // (uses same logic as send_data, ensuring last_connection works for UDP)
        for e in &recent_peers {
            let mut dead = true;
            if let Some(nr) = self.lookup_node_ref(outer_self.clone(), *e) {
                if let Some(last_connection) = nr.last_connection() {
                    out.push((*e, RecentPeersEntry { last_connection }));
                    dead = false;
                }
            }
            if dead {
                dead_peers.push(e);
            }
        }

        // purge dead recent peers
        for d in dead_peers {
            self.recent_peers.remove(d);
        }

        out
    }

    pub fn touch_recent_peer(&self, node_id: DHTKey, last_connection: ConnectionDescriptor) {
        self.recent_peers
            .insert(node_id, RecentPeersEntry { last_connection });
    }

    //////////////////////////////////////////////////////////////////////
    // Find Nodes

    // Retrieve the fastest nodes in the routing table matching an entry filter
    pub fn find_fast_public_nodes_filtered<'a, 'b, F>(
        &self,
        outer_self: RoutingTable,
        node_count: usize,
        mut entry_filter: F,
    ) -> Vec<NodeRef>
    where
        F: FnMut(&'a RoutingTableInner, &'b BucketEntryInner) -> bool,
    {
        self.find_fastest_nodes(
            // count
            node_count,
            // filter
            |rti, _k: DHTKey, v: Option<Arc<BucketEntry>>| {
                let entry = v.unwrap();
                entry.with(rti, |rti, e| {
                    // skip nodes on local network
                    if e.node_info(RoutingDomain::LocalNetwork).is_some() {
                        return false;
                    }
                    // skip nodes not on public internet
                    if e.node_info(RoutingDomain::PublicInternet).is_none() {
                        return false;
                    }
                    // skip nodes that dont match entry filter
                    entry_filter(rti, e)
                })
            },
            // transform
            |_rti, k: DHTKey, v: Option<Arc<BucketEntry>>| {
                NodeRef::new(outer_self.clone(), k, v.unwrap().clone(), None)
            },
        )
    }

    pub fn filter_has_valid_signed_node_info(
        &self,
        routing_domain: RoutingDomain,
        has_valid_own_node_info: bool,
        v: Option<Arc<BucketEntry>>,
    ) -> bool {
        match v {
            None => has_valid_own_node_info,
            Some(entry) => entry.with(self, |_rti, e| {
                e.signed_node_info(routing_domain.into())
                    .map(|sni| sni.has_valid_signature())
                    .unwrap_or(false)
            }),
        }
    }

    pub fn transform_to_peer_info(
        &self,
        routing_domain: RoutingDomain,
        own_peer_info: PeerInfo,
        k: DHTKey,
        v: Option<Arc<BucketEntry>>,
    ) -> PeerInfo {
        match v {
            None => own_peer_info,
            Some(entry) => entry.with(self, |_rti, e| e.make_peer_info(k, routing_domain).unwrap()),
        }
    }

    pub fn find_peers_with_sort_and_filter<'a, 'b, F, C, T, O>(
        &self,
        node_count: usize,
        cur_ts: u64,
        mut filter: F,
        compare: C,
        mut transform: T,
    ) -> Vec<O>
    where
        F: FnMut(&'a RoutingTableInner, DHTKey, Option<Arc<BucketEntry>>) -> bool,
        C: FnMut(
            &'a RoutingTableInner,
            &'b (DHTKey, Option<Arc<BucketEntry>>),
            &'b (DHTKey, Option<Arc<BucketEntry>>),
        ) -> core::cmp::Ordering,
        T: FnMut(&'a RoutingTableInner, DHTKey, Option<Arc<BucketEntry>>) -> O,
    {
        // collect all the nodes for sorting
        let mut nodes =
            Vec::<(DHTKey, Option<Arc<BucketEntry>>)>::with_capacity(self.bucket_entry_count + 1);

        // add our own node (only one of there with the None entry)
        if filter(self, self.unlocked_inner.node_id, None) {
            nodes.push((self.unlocked_inner.node_id, None));
        }

        // add all nodes from buckets
        self.with_entries(cur_ts, BucketEntryState::Unreliable, |rti, k, v| {
            // Apply filter
            if filter(rti, k, Some(v.clone())) {
                nodes.push((k, Some(v.clone())));
            }
            Option::<()>::None
        });

        // sort by preference for returning nodes
        nodes.sort_by(|a, b| compare(self, a, b));

        // return transformed vector for filtered+sorted nodes
        let cnt = usize::min(node_count, nodes.len());
        let mut out = Vec::<O>::with_capacity(cnt);
        for node in nodes {
            let val = transform(self, node.0, node.1);
            out.push(val);
        }

        out
    }

    pub fn find_fastest_nodes<'a, T, F, O>(
        &self,
        node_count: usize,
        mut filter: F,
        transform: T,
    ) -> Vec<O>
    where
        F: FnMut(&'a RoutingTableInner, DHTKey, Option<Arc<BucketEntry>>) -> bool,
        T: FnMut(&'a RoutingTableInner, DHTKey, Option<Arc<BucketEntry>>) -> O,
    {
        let cur_ts = intf::get_timestamp();
        let out = self.find_peers_with_sort_and_filter(
            node_count,
            cur_ts,
            // filter
            |rti, k, v| {
                if let Some(entry) = &v {
                    // always filter out dead nodes
                    if entry.with(rti, |_rti, e| e.state(cur_ts) == BucketEntryState::Dead) {
                        false
                    } else {
                        filter(rti, k, v)
                    }
                } else {
                    // always filter out self peer, as it is irrelevant to the 'fastest nodes' search
                    false
                }
            },
            // sort
            |rti, (a_key, a_entry), (b_key, b_entry)| {
                // same nodes are always the same
                if a_key == b_key {
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
                ae.with(rti, |rti, ae| {
                    be.with(rti, |_rti, be| {
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
            },
            // transform,
            transform,
        );
        out
    }

    pub fn find_closest_nodes<'a, F, T, O>(
        &self,
        node_id: DHTKey,
        filter: F,
        mut transform: T,
    ) -> Vec<O>
    where
        F: FnMut(&'a RoutingTableInner, DHTKey, Option<Arc<BucketEntry>>) -> bool,
        T: FnMut(&'a RoutingTableInner, DHTKey, Option<Arc<BucketEntry>>) -> O,
    {
        let cur_ts = intf::get_timestamp();
        let node_count = {
            let config = self.config();
            let c = config.get();
            c.network.dht.max_find_node_count as usize
        };
        let out = self.find_peers_with_sort_and_filter(
            node_count,
            cur_ts,
            // filter
            filter,
            // sort
            |rti, (a_key, a_entry), (b_key, b_entry)| {
                // same nodes are always the same
                if a_key == b_key {
                    return core::cmp::Ordering::Equal;
                }

                // reliable nodes come first, pessimistically treating our own node as unreliable
                let ra = a_entry
                    .as_ref()
                    .map_or(false, |x| x.with(rti, |_rti, x| x.check_reliable(cur_ts)));
                let rb = b_entry
                    .as_ref()
                    .map_or(false, |x| x.with(rti, |_rti, x| x.check_reliable(cur_ts)));
                if ra != rb {
                    if ra {
                        return core::cmp::Ordering::Less;
                    } else {
                        return core::cmp::Ordering::Greater;
                    }
                }

                // distance is the next metric, closer nodes first
                let da = distance(a_key, &node_id);
                let db = distance(b_key, &node_id);
                da.cmp(&db)
            },
            // transform,
            &mut transform,
        );
        log_rtab!(">> find_closest_nodes: node count = {}", out.len());
        out
    }
}
