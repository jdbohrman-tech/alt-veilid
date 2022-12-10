mod bucket;
mod bucket_entry;
mod debug;
mod node_ref;
mod node_ref_filter;
mod privacy;
mod route_spec_store;
mod routing_domain_editor;
mod routing_domains;
mod routing_table_inner;
mod stats_accounting;
mod tasks;

use crate::*;

use crate::crypto::*;
use crate::network_manager::*;
use crate::rpc_processor::*;
use bucket::*;
pub use bucket_entry::*;
pub use debug::*;
use hashlink::LruCache;
pub use node_ref::*;
pub use node_ref_filter::*;
pub use privacy::*;
pub use route_spec_store::*;
pub use routing_domain_editor::*;
pub use routing_domains::*;
pub use routing_table_inner::*;
pub use stats_accounting::*;

//////////////////////////////////////////////////////////////////////////

/// How frequently we tick the relay management routine
pub const RELAY_MANAGEMENT_INTERVAL_SECS: u32 = 1;

/// How frequently we tick the private route management routine
pub const PRIVATE_ROUTE_MANAGEMENT_INTERVAL_SECS: u32 = 1;

// Connectionless protocols like UDP are dependent on a NAT translation timeout
// We should ping them with some frequency and 30 seconds is typical timeout
pub const CONNECTIONLESS_TIMEOUT_SECS: u32 = 29;

pub type LowLevelProtocolPorts = BTreeSet<(LowLevelProtocolType, AddressType, u16)>;
pub type ProtocolToPortMapping = BTreeMap<(ProtocolType, AddressType), (LowLevelProtocolType, u16)>;
#[derive(Clone, Debug)]
pub struct LowLevelPortInfo {
    pub low_level_protocol_ports: LowLevelProtocolPorts,
    pub protocol_to_port: ProtocolToPortMapping,
}
pub type RoutingTableEntryFilter<'t> =
    Box<dyn FnMut(&RoutingTableInner, DHTKey, Option<Arc<BucketEntry>>) -> bool + Send + 't>;

#[derive(Clone, Debug, Default)]
pub struct RoutingTableHealth {
    /// Number of reliable (responsive) entries in the routing table
    pub reliable_entry_count: usize,
    /// Number of unreliable (occasionally unresponsive) entries in the routing table
    pub unreliable_entry_count: usize,
    /// Number of dead (always unresponsive) entries in the routing table
    pub dead_entry_count: usize,
}

pub(super) struct RoutingTableUnlockedInner {
    // Accessors
    config: VeilidConfig,
    network_manager: NetworkManager,

    /// The current node's public DHT key
    node_id: DHTKey,
    /// The current node's DHT key secret
    node_id_secret: DHTKeySecret,
    /// Buckets to kick on our next kick task
    kick_queue: Mutex<BTreeSet<usize>>,
    /// Background process for computing statistics
    rolling_transfers_task: TickTask<EyreReport>,
    /// Backgroup process to purge dead routing table entries when necessary
    kick_buckets_task: TickTask<EyreReport>,
    /// Background process to get our initial routing table
    bootstrap_task: TickTask<EyreReport>,
    /// Background process to ensure we have enough nodes in our routing table
    peer_minimum_refresh_task: TickTask<EyreReport>,
    /// Background process to check nodes to see if they are still alive and for reliability
    ping_validator_task: TickTask<EyreReport>,
    /// Background process to keep relays up
    relay_management_task: TickTask<EyreReport>,
    /// Background process to keep private routes up
    private_route_management_task: TickTask<EyreReport>,
}

#[derive(Clone)]
pub struct RoutingTable {
    inner: Arc<RwLock<RoutingTableInner>>,
    unlocked_inner: Arc<RoutingTableUnlockedInner>,
}

impl RoutingTable {
    fn new_unlocked_inner(
        config: VeilidConfig,
        network_manager: NetworkManager,
    ) -> RoutingTableUnlockedInner {
        let c = config.get();
        RoutingTableUnlockedInner {
            config: config.clone(),
            network_manager,
            node_id: c.network.node_id.unwrap(),
            node_id_secret: c.network.node_id_secret.unwrap(),
            kick_queue: Mutex::new(BTreeSet::default()),
            rolling_transfers_task: TickTask::new(ROLLING_TRANSFERS_INTERVAL_SECS),
            kick_buckets_task: TickTask::new(1),
            bootstrap_task: TickTask::new(1),
            peer_minimum_refresh_task: TickTask::new_ms(c.network.dht.min_peer_refresh_time_ms),
            ping_validator_task: TickTask::new(1),
            relay_management_task: TickTask::new(RELAY_MANAGEMENT_INTERVAL_SECS),
            private_route_management_task: TickTask::new(PRIVATE_ROUTE_MANAGEMENT_INTERVAL_SECS),
        }
    }
    pub fn new(network_manager: NetworkManager) -> Self {
        let config = network_manager.config();
        let unlocked_inner = Arc::new(Self::new_unlocked_inner(config, network_manager));
        let inner = Arc::new(RwLock::new(RoutingTableInner::new(unlocked_inner.clone())));
        let this = Self {
            inner,
            unlocked_inner,
        };

        this.start_tasks();

        this
    }

    pub fn network_manager(&self) -> NetworkManager {
        self.unlocked_inner.network_manager.clone()
    }
    pub fn rpc_processor(&self) -> RPCProcessor {
        self.network_manager().rpc_processor()
    }
    pub fn update_callback(&self) -> UpdateCallback {
        self.network_manager().update_callback()
    }
    pub fn with_config<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&VeilidConfigInner) -> R,
    {
        f(&*self.unlocked_inner.config.get())
    }

    pub fn node_id(&self) -> DHTKey {
        self.unlocked_inner.node_id
    }

    pub fn node_id_secret(&self) -> DHTKeySecret {
        self.unlocked_inner.node_id_secret
    }

    /////////////////////////////////////
    /// Initialization

    /// Called to initialize the routing table after it is created
    pub async fn init(&self) -> EyreResult<()> {
        debug!("starting routing table init");

        // Set up routing buckets
        {
            let mut inner = self.inner.write();
            inner.init_buckets(self.clone());
        }

        // Load bucket entries from table db if possible
        debug!("loading routing table entries");
        if let Err(e) = self.load_buckets().await {
            log_rtab!(debug "Error loading buckets from storage: {:#?}. Resetting.", e);
            let mut inner = self.inner.write();
            inner.init_buckets(self.clone());
        }

        // Set up routespecstore
        debug!("starting route spec store init");
        let route_spec_store = match RouteSpecStore::load(self.clone()).await {
            Ok(v) => v,
            Err(e) => {
                log_rtab!(debug "Error loading route spec store: {:#?}. Resetting.", e);
                RouteSpecStore::new(self.clone())
            }
        };
        debug!("finished route spec store init");

        {
            let mut inner = self.inner.write();
            inner.route_spec_store = Some(route_spec_store);
        }

        debug!("finished routing table init");
        Ok(())
    }

    /// Called to shut down the routing table
    pub async fn terminate(&self) {
        debug!("starting routing table terminate");

        // Stop tasks
        self.stop_tasks().await;

        // Load bucket entries from table db if possible
        debug!("saving routing table entries");
        if let Err(e) = self.save_buckets().await {
            error!("failed to save routing table entries: {}", e);
        }

        debug!("saving route spec store");
        let rss = {
            let mut inner = self.inner.write();
            inner.route_spec_store.take()
        };
        if let Some(rss) = rss {
            if let Err(e) = rss.save().await {
                error!("couldn't save route spec store: {}", e);
            }
        }
        debug!("shutting down routing table");

        let mut inner = self.inner.write();
        *inner = RoutingTableInner::new(self.unlocked_inner.clone());

        debug!("finished routing table terminate");
    }

    async fn save_buckets(&self) -> EyreResult<()> {
        // Serialize all entries
        let mut bucketvec: Vec<Vec<u8>> = Vec::new();
        {
            let inner = &*self.inner.read();
            for bucket in &inner.buckets {
                bucketvec.push(bucket.save_bucket()?)
            }
        }
        let table_store = self.network_manager().table_store();
        let tdb = table_store.open("routing_table", 1).await?;
        let bucket_count = bucketvec.len();
        let mut dbx = tdb.transact();
        if let Err(e) = dbx.store_rkyv(0, b"bucket_count", &bucket_count) {
            dbx.rollback();
            return Err(e);
        }

        for (n, b) in bucketvec.iter().enumerate() {
            dbx.store(0, format!("bucket_{}", n).as_bytes(), b)
        }
        dbx.commit()?;
        Ok(())
    }

    async fn load_buckets(&self) -> EyreResult<()> {
        // Deserialize all entries
        let tstore = self.network_manager().table_store();
        let tdb = tstore.open("routing_table", 1).await?;
        let Some(bucket_count): Option<usize> = tdb.load_rkyv(0, b"bucket_count")? else {
            log_rtab!(debug "no bucket count in saved routing table");
            return Ok(());
        };
        let inner = &mut *self.inner.write();
        if bucket_count != inner.buckets.len() {
            // Must have the same number of buckets
            warn!("bucket count is different, not loading routing table");
            return Ok(());
        }
        let mut bucketdata_vec: Vec<Vec<u8>> = Vec::new();
        for n in 0..bucket_count {
            let Some(bucketdata): Option<Vec<u8>> =
                tdb.load(0, format!("bucket_{}", n).as_bytes())? else {
                    warn!("bucket data not loading, skipping loading routing table");
                    return Ok(());
                };
            bucketdata_vec.push(bucketdata);
        }
        for (n, bucketdata) in bucketdata_vec.into_iter().enumerate() {
            inner.buckets[n].load_bucket(bucketdata)?;
        }

        Ok(())
    }

    /// Set up the local network routing domain with our local routing table configuration
    pub fn configure_local_network_routing_domain(&self, local_networks: Vec<(IpAddr, IpAddr)>) {
        log_net!(debug "configure_local_network_routing_domain: {:#?}", local_networks);
        self.inner
            .write()
            .configure_local_network_routing_domain(local_networks);
    }

    /////////////////////////////////////
    /// Locked operations

    pub fn routing_domain_for_address(&self, address: Address) -> Option<RoutingDomain> {
        self.inner.read().routing_domain_for_address(address)
    }

    pub fn route_spec_store(&self) -> RouteSpecStore {
        self.inner.read().route_spec_store.as_ref().unwrap().clone()
    }

    pub fn relay_node(&self, domain: RoutingDomain) -> Option<NodeRef> {
        self.inner.read().relay_node(domain)
    }

    pub fn has_dial_info(&self, domain: RoutingDomain) -> bool {
        self.inner.read().has_dial_info(domain)
    }

    pub fn dial_info_details(&self, domain: RoutingDomain) -> Vec<DialInfoDetail> {
        self.inner.read().dial_info_details(domain)
    }

    pub fn first_filtered_dial_info_detail(
        &self,
        routing_domain_set: RoutingDomainSet,
        filter: &DialInfoFilter,
    ) -> Option<DialInfoDetail> {
        self.inner
            .read()
            .first_filtered_dial_info_detail(routing_domain_set, filter)
    }

    pub fn all_filtered_dial_info_details(
        &self,
        routing_domain_set: RoutingDomainSet,
        filter: &DialInfoFilter,
    ) -> Vec<DialInfoDetail> {
        self.inner
            .read()
            .all_filtered_dial_info_details(routing_domain_set, filter)
    }

    pub fn ensure_dial_info_is_valid(&self, domain: RoutingDomain, dial_info: &DialInfo) -> bool {
        self.inner
            .read()
            .ensure_dial_info_is_valid(domain, dial_info)
    }

    pub fn node_info_is_valid_in_routing_domain(
        &self,
        routing_domain: RoutingDomain,
        node_info: &NodeInfo,
    ) -> bool {
        self.inner
            .read()
            .node_info_is_valid_in_routing_domain(routing_domain, node_info)
    }

    pub fn signed_node_info_is_valid_in_routing_domain(
        &self,
        routing_domain: RoutingDomain,
        signed_node_info: &SignedNodeInfo,
    ) -> bool {
        self.inner
            .read()
            .signed_node_info_is_valid_in_routing_domain(routing_domain, signed_node_info)
    }

    /// Look up the best way for two nodes to reach each other over a specific routing domain
    #[instrument(level = "trace", skip(self), ret)]
    pub fn get_contact_method(
        &self,
        routing_domain: RoutingDomain,
        peer_a: &PeerInfo,
        peer_b: &PeerInfo,
        dial_info_filter: DialInfoFilter,
        sequencing: Sequencing,
    ) -> ContactMethod {
        self.inner.read().get_contact_method(
            routing_domain,
            peer_a,
            peer_b,
            dial_info_filter,
            sequencing,
        )
    }

    #[instrument(level = "debug", skip(self))]
    pub fn edit_routing_domain(&self, domain: RoutingDomain) -> RoutingDomainEditor {
        RoutingDomainEditor::new(self.clone(), domain)
    }

    /// Return a copy of our node's peerinfo
    pub fn get_own_peer_info(&self, routing_domain: RoutingDomain) -> Option<PeerInfo> {
        self.inner.read().get_own_peer_info(routing_domain)
    }

    /// Return the best effort copy of our node's peerinfo
    /// This may be invalid and should not be passed to other nodes,
    /// but may be used for contact method calculation
    pub fn get_best_effort_own_peer_info(&self, routing_domain: RoutingDomain) -> PeerInfo {
        self.inner
            .read()
            .get_best_effort_own_peer_info(routing_domain)
    }

    /// If we have a valid network class in this routing domain, then our 'NodeInfo' is valid
    /// If this is true, we can get our final peer info, otherwise we only have a 'best effort' peer info
    pub fn has_valid_own_node_info(&self, routing_domain: RoutingDomain) -> bool {
        self.inner.read().has_valid_own_node_info(routing_domain)
    }

    /// Return our current node info timestamp
    pub fn get_own_node_info_ts(&self, routing_domain: RoutingDomain) -> Option<u64> {
        self.inner.read().get_own_node_info_ts(routing_domain)
    }

    /// Return the domain's currently registered network class
    pub fn get_network_class(&self, routing_domain: RoutingDomain) -> Option<NetworkClass> {
        self.inner.read().get_network_class(routing_domain)
    }

    /// Return the domain's filter for what we can receivein the form of a dial info filter
    pub fn get_inbound_dial_info_filter(&self, routing_domain: RoutingDomain) -> DialInfoFilter {
        self.inner
            .read()
            .get_inbound_dial_info_filter(routing_domain)
    }

    /// Return the domain's filter for what we can receive in the form of a node ref filter
    pub fn get_inbound_node_ref_filter(&self, routing_domain: RoutingDomain) -> NodeRefFilter {
        self.inner
            .read()
            .get_inbound_node_ref_filter(routing_domain)
    }

    /// Return the domain's filter for what we can send out in the form of a dial info filter
    pub fn get_outbound_dial_info_filter(&self, routing_domain: RoutingDomain) -> DialInfoFilter {
        self.inner
            .read()
            .get_outbound_dial_info_filter(routing_domain)
    }
    /// Return the domain's filter for what we can receive in the form of a node ref filter
    pub fn get_outbound_node_ref_filter(&self, routing_domain: RoutingDomain) -> NodeRefFilter {
        self.inner
            .read()
            .get_outbound_node_ref_filter(routing_domain)
    }

    /// Attempt to empty the routing table
    /// should only be performed when there are no node_refs (detached)
    pub fn purge_buckets(&self) {
        self.inner.write().purge_buckets();
    }

    /// Attempt to remove last_connections from entries
    pub fn purge_last_connections(&self) {
        self.inner.write().purge_last_connections();
    }

    fn find_bucket_index(&self, node_id: DHTKey) -> usize {
        distance(&node_id, &self.unlocked_inner.node_id)
            .first_nonzero_bit()
            .unwrap()
    }

    pub fn get_entry_count(
        &self,
        routing_domain_set: RoutingDomainSet,
        min_state: BucketEntryState,
    ) -> usize {
        self.inner
            .read()
            .get_entry_count(routing_domain_set, min_state)
    }

    pub fn get_nodes_needing_ping(
        &self,
        routing_domain: RoutingDomain,
        cur_ts: u64,
    ) -> Vec<NodeRef> {
        self.inner
            .read()
            .get_nodes_needing_ping(self.clone(), routing_domain, cur_ts)
    }

    pub fn get_all_nodes(&self, cur_ts: u64) -> Vec<NodeRef> {
        let inner = self.inner.read();
        inner.get_all_nodes(self.clone(), cur_ts)
    }

    fn queue_bucket_kick(&self, node_id: DHTKey) {
        let idx = self.find_bucket_index(node_id);
        self.unlocked_inner.kick_queue.lock().insert(idx);
    }

    /// Create a node reference, possibly creating a bucket entry
    /// the 'update_func' closure is called on the node, and, if created,
    /// in a locked fashion as to ensure the bucket entry state is always valid
    pub fn create_node_ref<F>(&self, node_id: DHTKey, update_func: F) -> Option<NodeRef>
    where
        F: FnOnce(&mut RoutingTableInner, &mut BucketEntryInner),
    {
        self.inner
            .write()
            .create_node_ref(self.clone(), node_id, update_func)
    }

    /// Resolve an existing routing table entry and return a reference to it
    pub fn lookup_node_ref(&self, node_id: DHTKey) -> Option<NodeRef> {
        self.inner.read().lookup_node_ref(self.clone(), node_id)
    }

    /// Resolve an existing routing table entry and return a filtered reference to it
    pub fn lookup_and_filter_noderef(
        &self,
        node_id: DHTKey,
        routing_domain_set: RoutingDomainSet,
        dial_info_filter: DialInfoFilter,
    ) -> Option<NodeRef> {
        self.inner.read().lookup_and_filter_noderef(
            self.clone(),
            node_id,
            routing_domain_set,
            dial_info_filter,
        )
    }

    /// Shortcut function to add a node to our routing table if it doesn't exist
    /// and add the dial info we have for it. Returns a noderef filtered to
    /// the routing domain in which this node was registered for convenience.
    pub fn register_node_with_signed_node_info(
        &self,
        routing_domain: RoutingDomain,
        node_id: DHTKey,
        signed_node_info: SignedNodeInfo,
        allow_invalid: bool,
    ) -> Option<NodeRef> {
        self.inner.write().register_node_with_signed_node_info(
            self.clone(),
            routing_domain,
            node_id,
            signed_node_info,
            allow_invalid,
        )
    }

    /// Shortcut function to add a node to our routing table if it doesn't exist
    /// and add the last peer address we have for it, since that's pretty common
    pub fn register_node_with_existing_connection(
        &self,
        node_id: DHTKey,
        descriptor: ConnectionDescriptor,
        timestamp: u64,
    ) -> Option<NodeRef> {
        self.inner.write().register_node_with_existing_connection(
            self.clone(),
            node_id,
            descriptor,
            timestamp,
        )
    }

    //////////////////////////////////////////////////////////////////////
    // Routing Table Health Metrics

    pub fn get_routing_table_health(&self) -> RoutingTableHealth {
        self.inner.read().get_routing_table_health()
    }

    pub fn get_recent_peers(&self) -> Vec<(DHTKey, RecentPeersEntry)> {
        let mut recent_peers = Vec::new();
        let mut dead_peers = Vec::new();
        let mut out = Vec::new();

        // collect all recent peers
        {
            let inner = self.inner.read();
            for (k, _v) in &inner.recent_peers {
                recent_peers.push(*k);
            }
        }

        // look up each node and make sure the connection is still live
        // (uses same logic as send_data, ensuring last_connection works for UDP)
        for e in &recent_peers {
            let mut dead = true;
            if let Some(nr) = self.lookup_node_ref(*e) {
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
        {
            let mut inner = self.inner.write();
            for d in dead_peers {
                inner.recent_peers.remove(d);
            }
        }

        out
    }

    pub fn touch_recent_peer(&self, node_id: DHTKey, last_connection: ConnectionDescriptor) {
        self.inner
            .write()
            .touch_recent_peer(node_id, last_connection)
    }

    //////////////////////////////////////////////////////////////////////
    // Find Nodes

    /// Build a map of protocols to low level ports
    /// This way we can get the set of protocols required to keep our NAT mapping alive for keepalive pings
    /// Only one protocol per low level protocol/port combination is required
    /// For example, if WS/WSS and TCP protocols are on the same low-level TCP port, only TCP keepalives will be required
    /// and we do not need to do WS/WSS keepalive as well. If they are on different ports, then we will need WS/WSS keepalives too.
    pub fn get_low_level_port_info(&self) -> LowLevelPortInfo {
        let mut low_level_protocol_ports =
            BTreeSet::<(LowLevelProtocolType, AddressType, u16)>::new();
        let mut protocol_to_port =
            BTreeMap::<(ProtocolType, AddressType), (LowLevelProtocolType, u16)>::new();
        let our_dids = self.all_filtered_dial_info_details(
            RoutingDomain::PublicInternet.into(),
            &DialInfoFilter::all(),
        );
        for did in our_dids {
            low_level_protocol_ports.insert((
                did.dial_info.protocol_type().low_level_protocol_type(),
                did.dial_info.address_type(),
                did.dial_info.socket_address().port(),
            ));
            protocol_to_port.insert(
                (did.dial_info.protocol_type(), did.dial_info.address_type()),
                (
                    did.dial_info.protocol_type().low_level_protocol_type(),
                    did.dial_info.socket_address().port(),
                ),
            );
        }
        LowLevelPortInfo {
            low_level_protocol_ports,
            protocol_to_port,
        }
    }

    /// Makes a filter that finds nodes with a matching inbound dialinfo
    pub fn make_inbound_dial_info_entry_filter<'a>(
        routing_domain: RoutingDomain,
        dial_info_filter: DialInfoFilter,
    ) -> RoutingTableEntryFilter<'a> {
        // does it have matching public dial info?
        Box::new(move |rti, _k, e| {
            if let Some(e) = e {
                e.with(rti, |_rti, e| {
                    if let Some(ni) = e.node_info(routing_domain) {
                        if ni
                            .first_filtered_dial_info_detail(DialInfoDetail::NO_SORT, |did| {
                                did.matches_filter(&dial_info_filter)
                            })
                            .is_some()
                        {
                            return true;
                        }
                    }
                    false
                })
            } else {
                rti.first_filtered_dial_info_detail(routing_domain.into(), &dial_info_filter)
                    .is_some()
            }
        })
    }

    /// Makes a filter that finds nodes capable of dialing a particular outbound dialinfo
    pub fn make_outbound_dial_info_entry_filter<'a>(
        routing_domain: RoutingDomain,
        dial_info: DialInfo,
    ) -> RoutingTableEntryFilter<'a> {
        // does the node's outbound capabilities match the dialinfo?
        Box::new(move |rti, _k, e| {
            if let Some(e) = e {
                e.with(rti, |_rti, e| {
                    if let Some(ni) = e.node_info(routing_domain) {
                        let dif = DialInfoFilter::all()
                            .with_protocol_type_set(ni.outbound_protocols)
                            .with_address_type_set(ni.address_types);
                        if dial_info.matches_filter(&dif) {
                            return true;
                        }
                    }
                    false
                })
            } else {
                let dif = rti.get_outbound_dial_info_filter(routing_domain);
                dial_info.matches_filter(&dif)
            }
        })
    }

    pub fn find_fast_public_nodes_filtered(
        &self,
        node_count: usize,
        filters: VecDeque<RoutingTableEntryFilter>,
    ) -> Vec<NodeRef> {
        self.inner
            .read()
            .find_fast_public_nodes_filtered(self.clone(), node_count, filters)
    }

    /// Retrieve up to N of each type of protocol capable nodes
    pub fn find_bootstrap_nodes_filtered(&self, max_per_type: usize) -> Vec<NodeRef> {
        let protocol_types = vec![
            ProtocolType::UDP,
            ProtocolType::TCP,
            ProtocolType::WS,
            ProtocolType::WSS,
        ];
        let protocol_types_len = protocol_types.len();
        let mut nodes_proto_v4 = vec![0usize, 0usize, 0usize, 0usize];
        let mut nodes_proto_v6 = vec![0usize, 0usize, 0usize, 0usize];

        let filter = Box::new(
            move |rti: &RoutingTableInner, _k: DHTKey, v: Option<Arc<BucketEntry>>| {
                let entry = v.unwrap();
                entry.with(rti, |_rti, e| {
                    // skip nodes on our local network here
                    if e.has_node_info(RoutingDomain::LocalNetwork.into()) {
                        return false;
                    }

                    // does it have some dial info we need?
                    let filter = |n: &NodeInfo| {
                        let mut keep = false;
                        for did in &n.dial_info_detail_list {
                            if matches!(did.dial_info.address_type(), AddressType::IPV4) {
                                for (n, protocol_type) in protocol_types.iter().enumerate() {
                                    if nodes_proto_v4[n] < max_per_type
                                        && did.dial_info.protocol_type() == *protocol_type
                                    {
                                        nodes_proto_v4[n] += 1;
                                        keep = true;
                                    }
                                }
                            } else if matches!(did.dial_info.address_type(), AddressType::IPV6) {
                                for (n, protocol_type) in protocol_types.iter().enumerate() {
                                    if nodes_proto_v6[n] < max_per_type
                                        && did.dial_info.protocol_type() == *protocol_type
                                    {
                                        nodes_proto_v6[n] += 1;
                                        keep = true;
                                    }
                                }
                            }
                        }
                        keep
                    };

                    e.node_info(RoutingDomain::PublicInternet)
                        .map(filter)
                        .unwrap_or(false)
                })
            },
        ) as RoutingTableEntryFilter;

        let filters = VecDeque::from([filter]);

        self.find_fastest_nodes(
            protocol_types_len * 2 * max_per_type,
            filters,
            |_rti, k: DHTKey, v: Option<Arc<BucketEntry>>| {
                NodeRef::new(self.clone(), k, v.unwrap().clone(), None)
            },
        )
    }

    pub fn find_peers_with_sort_and_filter<C, T, O>(
        &self,
        node_count: usize,
        cur_ts: u64,
        filters: VecDeque<RoutingTableEntryFilter>,
        compare: C,
        transform: T,
    ) -> Vec<O>
    where
        C: for<'a, 'b> FnMut(
            &'a RoutingTableInner,
            &'b (DHTKey, Option<Arc<BucketEntry>>),
            &'b (DHTKey, Option<Arc<BucketEntry>>),
        ) -> core::cmp::Ordering,
        T: for<'r> FnMut(&'r RoutingTableInner, DHTKey, Option<Arc<BucketEntry>>) -> O + Send,
    {
        self.inner
            .read()
            .find_peers_with_sort_and_filter(node_count, cur_ts, filters, compare, transform)
    }

    pub fn find_fastest_nodes<'a, T, O>(
        &self,
        node_count: usize,
        filters: VecDeque<RoutingTableEntryFilter>,
        transform: T,
    ) -> Vec<O>
    where
        T: for<'r> FnMut(&'r RoutingTableInner, DHTKey, Option<Arc<BucketEntry>>) -> O + Send,
    {
        self.inner
            .read()
            .find_fastest_nodes(node_count, filters, transform)
    }

    pub fn find_closest_nodes<'a, T, O>(
        &self,
        node_id: DHTKey,
        filters: VecDeque<RoutingTableEntryFilter>,
        transform: T,
    ) -> Vec<O>
    where
        T: for<'r> FnMut(&'r RoutingTableInner, DHTKey, Option<Arc<BucketEntry>>) -> O + Send,
    {
        self.inner
            .read()
            .find_closest_nodes(node_id, filters, transform)
    }

    #[instrument(level = "trace", skip(self), ret)]
    pub fn register_find_node_answer(&self, peers: Vec<PeerInfo>) -> Vec<NodeRef> {
        let node_id = self.node_id();

        // register nodes we'd found
        let mut out = Vec::<NodeRef>::with_capacity(peers.len());
        for p in peers {
            // if our own node if is in the list then ignore it, as we don't add ourselves to our own routing table
            if p.node_id.key == node_id {
                continue;
            }

            // node can not be its own relay
            if let Some(rid) = &p.signed_node_info.relay_id() {
                if rid.key == p.node_id.key {
                    continue;
                }
            }

            // register the node if it's new
            if let Some(nr) = self.register_node_with_signed_node_info(
                RoutingDomain::PublicInternet,
                p.node_id.key,
                p.signed_node_info.clone(),
                false,
            ) {
                out.push(nr);
            }
        }
        out
    }

    #[instrument(level = "trace", skip(self), ret, err)]
    pub async fn find_node(
        &self,
        node_ref: NodeRef,
        node_id: DHTKey,
    ) -> EyreResult<NetworkResult<Vec<NodeRef>>> {
        let rpc_processor = self.rpc_processor();

        let res = network_result_try!(
            rpc_processor
                .clone()
                .rpc_call_find_node(Destination::direct(node_ref), node_id)
                .await?
        );

        // register nodes we'd found
        Ok(NetworkResult::value(
            self.register_find_node_answer(res.answer),
        ))
    }

    #[instrument(level = "trace", skip(self), ret, err)]
    pub async fn find_self(&self, node_ref: NodeRef) -> EyreResult<NetworkResult<Vec<NodeRef>>> {
        let node_id = self.node_id();
        self.find_node(node_ref, node_id).await
    }

    #[instrument(level = "trace", skip(self), ret, err)]
    pub async fn find_target(&self, node_ref: NodeRef) -> EyreResult<NetworkResult<Vec<NodeRef>>> {
        let node_id = node_ref.node_id();
        self.find_node(node_ref, node_id).await
    }

    #[instrument(level = "trace", skip(self))]
    pub async fn reverse_find_node(&self, node_ref: NodeRef, wide: bool) {
        // Ask bootstrap node to 'find' our own node so we can get some more nodes near ourselves
        // and then contact those nodes to inform -them- that we exist

        // Ask bootstrap server for nodes closest to our own node
        let closest_nodes = network_result_value_or_log!(match self.find_self(node_ref.clone()).await {
            Err(e) => {
                log_rtab!(error
                    "find_self failed for {:?}: {:?}",
                    &node_ref, e
                );
                return;
            }
            Ok(v) => v,
        } => {
            return;
        });

        // Ask each node near us to find us as well
        if wide {
            for closest_nr in closest_nodes {
                network_result_value_or_log!(match self.find_self(closest_nr.clone()).await {
                    Err(e) => {
                        log_rtab!(error
                            "find_self failed for {:?}: {:?}",
                            &closest_nr, e
                        );
                        continue;
                    }
                    Ok(v) => v,
                } => {
                    // Do nothing with non-values
                    continue;
                });
            }
        }
    }

    pub fn make_public_internet_relay_node_filter(&self) -> impl Fn(&BucketEntryInner) -> bool {
        // Get all our outbound protocol/address types
        let outbound_dif = self.get_outbound_dial_info_filter(RoutingDomain::PublicInternet);
        let mapped_port_info = self.get_low_level_port_info();

        move |e: &BucketEntryInner| {
            // Ensure this node is not on the local network
            if e.has_node_info(RoutingDomain::LocalNetwork.into()) {
                return false;
            }

            // Disqualify nodes that don't cover all our inbound ports for tcp and udp
            // as we need to be able to use the relay for keepalives for all nat mappings
            let mut low_level_protocol_ports = mapped_port_info.low_level_protocol_ports.clone();

            let can_serve_as_relay = e
                .node_info(RoutingDomain::PublicInternet)
                .map(|n| {
                    let dids = n.all_filtered_dial_info_details(
                        Some(DialInfoDetail::ordered_sequencing_sort), // By default, choose connection-oriented protocol for relay
                        |did| did.matches_filter(&outbound_dif),
                    );
                    for did in &dids {
                        let pt = did.dial_info.protocol_type();
                        let at = did.dial_info.address_type();
                        if let Some((llpt, port)) = mapped_port_info.protocol_to_port.get(&(pt, at))
                        {
                            low_level_protocol_ports.remove(&(*llpt, at, *port));
                        }
                    }
                    low_level_protocol_ports.is_empty()
                })
                .unwrap_or(false);
            if !can_serve_as_relay {
                return false;
            }

            true
        }
    }

    #[instrument(level = "trace", skip(self), ret)]
    pub fn find_inbound_relay(
        &self,
        routing_domain: RoutingDomain,
        cur_ts: u64,
    ) -> Option<NodeRef> {
        // Get relay filter function
        let relay_node_filter = match routing_domain {
            RoutingDomain::PublicInternet => self.make_public_internet_relay_node_filter(),
            RoutingDomain::LocalNetwork => {
                unimplemented!();
            }
        };

        // Go through all entries and find fastest entry that matches filter function
        let inner = self.inner.read();
        let inner = &*inner;
        let mut best_inbound_relay: Option<(DHTKey, Arc<BucketEntry>)> = None;

        // Iterate all known nodes for candidates
        inner.with_entries(cur_ts, BucketEntryState::Unreliable, |rti, k, v| {
            let v2 = v.clone();
            v.with(rti, |rti, e| {
                // Ensure we have the node's status
                if let Some(node_status) = e.node_status(routing_domain) {
                    // Ensure the node will relay
                    if node_status.will_relay() {
                        // Compare against previous candidate
                        if let Some(best_inbound_relay) = best_inbound_relay.as_mut() {
                            // Less is faster
                            let better = best_inbound_relay.1.with(rti, |_rti, best| {
                                // choose low latency stability for relays
                                BucketEntryInner::cmp_fastest_reliable(cur_ts, e, best)
                                    == std::cmp::Ordering::Less
                            });
                            // Now apply filter function and see if this node should be included
                            if better && relay_node_filter(e) {
                                *best_inbound_relay = (k, v2);
                            }
                        } else if relay_node_filter(e) {
                            // Always store the first candidate
                            best_inbound_relay = Some((k, v2));
                        }
                    }
                }
            });
            // Don't end early, iterate through all entries
            Option::<()>::None
        });
        // Return the best inbound relay noderef
        best_inbound_relay.map(|(k, e)| NodeRef::new(self.clone(), k, e, None))
    }
}
