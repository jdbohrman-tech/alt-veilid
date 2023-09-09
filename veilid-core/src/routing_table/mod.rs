mod bucket;
mod bucket_entry;
mod debug;
mod find_peers;
mod node_ref;
mod node_ref_filter;
mod privacy;
mod route_spec_store;
mod routing_domain_editor;
mod routing_domains;
mod routing_table_inner;
mod stats_accounting;
mod tasks;
mod types;

pub mod tests;

use super::*;

use crate::crypto::*;
use crate::network_manager::*;
use crate::rpc_processor::*;
use bucket::*;
use hashlink::LruCache;

pub use bucket_entry::*;
pub use debug::*;
pub use find_peers::*;
pub use node_ref::*;
pub use node_ref_filter::*;
pub use privacy::*;
pub use route_spec_store::*;
pub use routing_domain_editor::*;
pub use routing_domains::*;
pub use routing_table_inner::*;
pub use stats_accounting::*;
pub use types::*;

//////////////////////////////////////////////////////////////////////////

/// How frequently we tick the relay management routine
pub const RELAY_MANAGEMENT_INTERVAL_SECS: u32 = 1;

/// How frequently we tick the private route management routine
pub const PRIVATE_ROUTE_MANAGEMENT_INTERVAL_SECS: u32 = 1;

// Connectionless protocols like UDP are dependent on a NAT translation timeout
// We should ping them with some frequency and 30 seconds is typical timeout
pub const CONNECTIONLESS_TIMEOUT_SECS: u32 = 29;

// Table store keys
const ALL_ENTRY_BYTES: &[u8] = b"all_entry_bytes";
const ROUTING_TABLE: &str = "routing_table";
const SERIALIZED_BUCKET_MAP: &[u8] = b"serialized_bucket_map";
const CACHE_VALIDITY_KEY: &[u8] = b"cache_validity_key";

pub type LowLevelProtocolPorts = BTreeSet<(LowLevelProtocolType, AddressType, u16)>;
pub type ProtocolToPortMapping = BTreeMap<(ProtocolType, AddressType), (LowLevelProtocolType, u16)>;
#[derive(Clone, Debug)]
pub struct LowLevelPortInfo {
    pub low_level_protocol_ports: LowLevelProtocolPorts,
    pub protocol_to_port: ProtocolToPortMapping,
}
pub type RoutingTableEntryFilter<'t> =
    Box<dyn FnMut(&RoutingTableInner, Option<Arc<BucketEntry>>) -> bool + Send + 't>;
pub type SerializedBuckets = Vec<Vec<u8>>;
pub type SerializedBucketMap = BTreeMap<CryptoKind, SerializedBuckets>;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RoutingTableHealth {
    /// Number of reliable (responsive) entries in the routing table
    pub reliable_entry_count: usize,
    /// Number of unreliable (occasionally unresponsive) entries in the routing table
    pub unreliable_entry_count: usize,
    /// Number of dead (always unresponsive) entries in the routing table
    pub dead_entry_count: usize,
    /// If PublicInternet network class is valid yet
    pub public_internet_ready: bool,
    /// If LocalNetwork network class is valid yet
    pub local_network_ready: bool,
}

pub type BucketIndex = (CryptoKind, usize);

pub struct RoutingTableUnlockedInner {
    // Accessors
    config: VeilidConfig,
    network_manager: NetworkManager,

    /// The current node's public DHT keys
    node_id: TypedKeyGroup,
    /// The current node's public DHT secrets
    node_id_secret: TypedSecretGroup,
    /// Buckets to kick on our next kick task
    kick_queue: Mutex<BTreeSet<BucketIndex>>,
    /// Background process for computing statistics
    rolling_transfers_task: TickTask<EyreReport>,
    /// Background process to purge dead routing table entries when necessary
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

impl RoutingTableUnlockedInner {
    pub fn network_manager(&self) -> NetworkManager {
        self.network_manager.clone()
    }
    pub fn crypto(&self) -> Crypto {
        self.network_manager().crypto()
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
        f(&*self.config.get())
    }

    pub fn node_id(&self, kind: CryptoKind) -> TypedKey {
        self.node_id.get(kind).unwrap()
    }

    pub fn node_id_secret_key(&self, kind: CryptoKind) -> SecretKey {
        self.node_id_secret.get(kind).unwrap().value
    }

    pub fn node_ids(&self) -> TypedKeyGroup {
        self.node_id.clone()
    }

    pub fn node_id_typed_key_pairs(&self) -> Vec<TypedKeyPair> {
        let mut tkps = Vec::new();
        for ck in VALID_CRYPTO_KINDS {
            tkps.push(TypedKeyPair::new(
                ck,
                KeyPair::new(self.node_id(ck).value, self.node_id_secret_key(ck)),
            ));
        }
        tkps
    }

    pub fn matches_own_node_id(&self, node_ids: &[TypedKey]) -> bool {
        for ni in node_ids {
            if let Some(v) = self.node_id.get(ni.kind) {
                if v.value == ni.value {
                    return true;
                }
            }
        }
        false
    }

    pub fn matches_own_node_id_key(&self, node_id_key: &PublicKey) -> bool {
        for tk in self.node_id.iter() {
            if tk.value == *node_id_key {
                return true;
            }
        }
        false
    }

    pub fn calculate_bucket_index(&self, node_id: &TypedKey) -> BucketIndex {
        let crypto = self.crypto();
        let self_node_id_key = self.node_id(node_id.kind).value;
        let vcrypto = crypto.get(node_id.kind).unwrap();
        (
            node_id.kind,
            vcrypto
                .distance(&node_id.value, &self_node_id_key)
                .first_nonzero_bit()
                .unwrap(),
        )
    }
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
            node_id: c.network.routing_table.node_id.clone(),
            node_id_secret: c.network.routing_table.node_id_secret.clone(),
            kick_queue: Mutex::new(BTreeSet::default()),
            rolling_transfers_task: TickTask::new(ROLLING_TRANSFERS_INTERVAL_SECS),
            kick_buckets_task: TickTask::new(1),
            bootstrap_task: TickTask::new(1),
            peer_minimum_refresh_task: TickTask::new(1),
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

        this.setup_tasks();

        this
    }

    /////////////////////////////////////
    /// Initialization

    /// Called to initialize the routing table after it is created
    pub async fn init(&self) -> EyreResult<()> {
        debug!("starting routing table init");

        // Set up routing buckets
        {
            let mut inner = self.inner.write();
            inner.init_buckets();
        }

        // Load bucket entries from table db if possible
        debug!("loading routing table entries");
        if let Err(e) = self.load_buckets().await {
            log_rtab!(debug "Error loading buckets from storage: {:#?}. Resetting.", e);
            let mut inner = self.inner.write();
            inner.init_buckets();
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
        self.cancel_tasks().await;

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

    /// Serialize the routing table.
    fn serialized_buckets(&self) -> (SerializedBucketMap, SerializedBuckets) {
        // Since entries are shared by multiple buckets per cryptokind
        // we need to get the list of all unique entries when serializing
        let mut all_entries: Vec<Arc<BucketEntry>> = Vec::new();

        // Serialize all buckets and get map of entries
        let mut serialized_bucket_map: SerializedBucketMap = BTreeMap::new();
        {
            let mut entry_map: HashMap<*const BucketEntry, u32> = HashMap::new();
            let inner = &*self.inner.read();
            for ck in VALID_CRYPTO_KINDS {
                let buckets = inner.buckets.get(&ck).unwrap();
                let mut serialized_buckets = Vec::new();
                for bucket in buckets.iter() {
                    serialized_buckets.push(bucket.save_bucket(&mut all_entries, &mut entry_map))
                }
                serialized_bucket_map.insert(ck, serialized_buckets);
            }
        }

        // Serialize all the entries
        let mut all_entry_bytes = Vec::with_capacity(all_entries.len());
        for entry in all_entries {
            // Serialize entry
            let entry_bytes = entry.with_inner(|e| serialize_json_bytes(e));
            all_entry_bytes.push(entry_bytes);
        }

        (serialized_bucket_map, all_entry_bytes)
    }

    /// Write the serialized routing table to the table store.
    async fn save_buckets(&self) -> EyreResult<()> {
        let (serialized_bucket_map, all_entry_bytes) = self.serialized_buckets();

        let table_store = self.unlocked_inner.network_manager().table_store();
        let tdb = table_store.open(ROUTING_TABLE, 1).await?;
        let dbx = tdb.transact();
        if let Err(e) = dbx.store_json(0, SERIALIZED_BUCKET_MAP, &serialized_bucket_map) {
            dbx.rollback();
            return Err(e.into());
        }
        if let Err(e) = dbx.store_json(0, ALL_ENTRY_BYTES, &all_entry_bytes) {
            dbx.rollback();
            return Err(e.into());
        }
        dbx.commit().await?;
        Ok(())
    }
    /// Deserialize routing table from table store
    async fn load_buckets(&self) -> EyreResult<()> {
        // Make a cache validity key of all our node ids and our bootstrap choice
        let mut cache_validity_key: Vec<u8> = Vec::new();
        {
            let c = self.unlocked_inner.config.get();
            for ck in VALID_CRYPTO_KINDS {
                if let Some(nid) = c.network.routing_table.node_id.get(ck) {
                    cache_validity_key.append(&mut nid.value.bytes.to_vec());
                }
            }
            for b in &c.network.routing_table.bootstrap {
                cache_validity_key.append(&mut b.as_bytes().to_vec());
            }
        };

        // Deserialize bucket map and all entries from the table store
        let table_store = self.unlocked_inner.network_manager().table_store();
        let db = table_store.open(ROUTING_TABLE, 1).await?;

        let caches_valid = match db.load(0, CACHE_VALIDITY_KEY).await? {
            Some(v) => v == cache_validity_key,
            None => false,
        };
        if !caches_valid {
            // Caches not valid, start over
            log_rtab!(debug "cache validity key changed, emptying routing table");
            drop(db);
            table_store.delete(ROUTING_TABLE).await?;
            let db = table_store.open(ROUTING_TABLE, 1).await?;
            db.store(0, CACHE_VALIDITY_KEY, &cache_validity_key).await?;
            return Ok(());
        }

        // Caches valid, load saved routing table
        let Some(serialized_bucket_map): Option<SerializedBucketMap> = db.load_json(0, SERIALIZED_BUCKET_MAP).await? else {
            log_rtab!(debug "no bucket map in saved routing table");
            return Ok(());
        };
        let Some(all_entry_bytes): Option<SerializedBuckets> = db.load_json(0, ALL_ENTRY_BYTES).await? else {
            log_rtab!(debug "no all_entry_bytes in saved routing table");
            return Ok(());
        };

        // Reconstruct all entries
        let inner = &mut *self.inner.write();
        self.populate_routing_table(inner, serialized_bucket_map, all_entry_bytes)?;

        Ok(())
    }

    /// Write the deserialized table store data to the routing table.
    pub fn populate_routing_table(
        &self,
        inner: &mut RoutingTableInner,
        serialized_bucket_map: SerializedBucketMap,
        all_entry_bytes: SerializedBuckets,
    ) -> EyreResult<()> {
        let mut all_entries: Vec<Arc<BucketEntry>> = Vec::with_capacity(all_entry_bytes.len());
        for entry_bytes in all_entry_bytes {
            let entryinner = deserialize_json_bytes(&entry_bytes)
                .wrap_err("failed to deserialize bucket entry")?;
            let entry = Arc::new(BucketEntry::new_with_inner(entryinner));

            // Keep strong reference in table
            all_entries.push(entry.clone());

            // Keep all entries in weak table too
            inner.all_entries.insert(entry);
        }

        // Validate serialized bucket map
        for (k, v) in &serialized_bucket_map {
            if !VALID_CRYPTO_KINDS.contains(k) {
                warn!("crypto kind is not valid, not loading routing table");
                return Ok(());
            }
            if v.len() != PUBLIC_KEY_LENGTH * 8 {
                warn!("bucket count is different, not loading routing table");
                return Ok(());
            }
        }

        // Recreate buckets
        for (k, v) in serialized_bucket_map {
            let buckets = inner.buckets.get_mut(&k).unwrap();

            for n in 0..v.len() {
                buckets[n].load_bucket(v[n].clone(), &all_entries)?;
            }
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

    pub fn relay_node_last_keepalive(&self, domain: RoutingDomain) -> Option<Timestamp> {
        self.inner.read().relay_node_last_keepalive(domain)
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
    pub fn get_contact_method(
        &self,
        routing_domain: RoutingDomain,
        peer_a: &PeerInfo,
        peer_b: &PeerInfo,
        dial_info_filter: DialInfoFilter,
        sequencing: Sequencing,
        dif_sort: Option<Arc<dyn Fn(&DialInfoDetail, &DialInfoDetail) -> core::cmp::Ordering>>,
    ) -> ContactMethod {
        self.inner.read().get_contact_method(
            routing_domain,
            peer_a,
            peer_b,
            dial_info_filter,
            sequencing,
            dif_sort,
        )
    }

    #[instrument(level = "debug", skip(self))]
    pub fn edit_routing_domain(&self, domain: RoutingDomain) -> RoutingDomainEditor {
        RoutingDomainEditor::new(self.clone(), domain)
    }

    /// Return a copy of our node's peerinfo
    pub fn get_own_peer_info(&self, routing_domain: RoutingDomain) -> PeerInfo {
        self.inner.read().get_own_peer_info(routing_domain)
    }

    /// If we have a valid network class in this routing domain, then our 'NodeInfo' is valid
    /// If this is true, we can get our final peer info, otherwise we only have a 'best effort' peer info
    pub fn has_valid_network_class(&self, routing_domain: RoutingDomain) -> bool {
        self.inner.read().has_valid_network_class(routing_domain)
    }

    /// Return our current node info timestamp
    pub fn get_own_node_info_ts(&self, routing_domain: RoutingDomain) -> Timestamp {
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
    /// May not empty buckets completely if there are existing node_refs
    pub fn purge_buckets(&self) {
        self.inner.write().purge_buckets();
    }

    /// Attempt to remove last_connections from entries
    pub fn purge_last_connections(&self) {
        self.inner.write().purge_last_connections();
    }

    pub fn get_entry_count(
        &self,
        routing_domain_set: RoutingDomainSet,
        min_state: BucketEntryState,
        crypto_kinds: &[CryptoKind],
    ) -> usize {
        self.inner
            .read()
            .get_entry_count(routing_domain_set, min_state, crypto_kinds)
    }

    pub fn get_entry_count_per_crypto_kind(
        &self,
        routing_domain_set: RoutingDomainSet,
        min_state: BucketEntryState,
    ) -> BTreeMap<CryptoKind, usize> {
        self.inner
            .read()
            .get_entry_count_per_crypto_kind(routing_domain_set, min_state)
    }

    pub fn get_nodes_needing_ping(
        &self,
        routing_domain: RoutingDomain,
        cur_ts: Timestamp,
    ) -> Vec<NodeRef> {
        self.inner
            .read()
            .get_nodes_needing_ping(self.clone(), routing_domain, cur_ts)
    }

    pub fn get_all_nodes(&self, cur_ts: Timestamp) -> Vec<NodeRef> {
        let inner = self.inner.read();
        inner.get_all_nodes(self.clone(), cur_ts)
    }

    fn queue_bucket_kicks(&self, node_ids: TypedKeyGroup) {
        for node_id in node_ids.iter() {
            // Skip node ids we didn't add to buckets
            if !VALID_CRYPTO_KINDS.contains(&node_id.kind) {
                continue;
            }

            // Put it in the kick queue
            let x = self.unlocked_inner.calculate_bucket_index(node_id);
            self.unlocked_inner.kick_queue.lock().insert(x);
        }
    }

    /// Resolve an existing routing table entry using any crypto kind and return a reference to it
    pub fn lookup_any_node_ref(&self, node_id_key: PublicKey) -> EyreResult<Option<NodeRef>> {
        self.inner
            .read()
            .lookup_any_node_ref(self.clone(), node_id_key)
    }

    /// Resolve an existing routing table entry and return a reference to it
    pub fn lookup_node_ref(&self, node_id: TypedKey) -> EyreResult<Option<NodeRef>> {
        self.inner.read().lookup_node_ref(self.clone(), node_id)
    }

    /// Resolve an existing routing table entry and return a filtered reference to it
    pub fn lookup_and_filter_noderef(
        &self,
        node_id: TypedKey,
        routing_domain_set: RoutingDomainSet,
        dial_info_filter: DialInfoFilter,
    ) -> EyreResult<Option<NodeRef>> {
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
    pub fn register_node_with_peer_info(
        &self,
        routing_domain: RoutingDomain,
        peer_info: PeerInfo,
        allow_invalid: bool,
    ) -> EyreResult<NodeRef> {
        self.inner.write().register_node_with_peer_info(
            self.clone(),
            routing_domain,
            peer_info,
            allow_invalid,
        )
    }

    /// Shortcut function to add a node to our routing table if it doesn't exist
    /// and add the last peer address we have for it, since that's pretty common
    pub fn register_node_with_existing_connection(
        &self,
        node_id: TypedKey,
        descriptor: ConnectionDescriptor,
        timestamp: Timestamp,
    ) -> EyreResult<NodeRef> {
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

    pub fn get_recent_peers(&self) -> Vec<(TypedKey, RecentPeersEntry)> {
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
            if let Ok(Some(nr)) = self.lookup_node_ref(*e) {
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

    pub fn touch_recent_peer(&self, node_id: TypedKey, last_connection: ConnectionDescriptor) {
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
        Box::new(move |rti, e| {
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
        Box::new(move |rti, e| {
            if let Some(e) = e {
                e.with(rti, |_rti, e| {
                    if let Some(ni) = e.node_info(routing_domain) {
                        let dif = DialInfoFilter::all()
                            .with_protocol_type_set(ni.outbound_protocols())
                            .with_address_type_set(ni.address_types());
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

    /// Retrieve up to N of each type of protocol capable nodes for a single crypto kind
    fn find_bootstrap_nodes_filtered_per_crypto_kind(
        &self,
        crypto_kind: CryptoKind,
        max_per_type: usize,
    ) -> Vec<NodeRef> {
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
            move |rti: &RoutingTableInner, entry: Option<Arc<BucketEntry>>| {
                let entry = entry.unwrap();
                entry.with(rti, |_rti, e| {
                    // skip nodes on our local network here
                    if e.has_node_info(RoutingDomain::LocalNetwork.into()) {
                        return false;
                    }

                    // Ensure crypto kind is supported
                    if !e.crypto_kinds().contains(&crypto_kind) {
                        return false;
                    }

                    // does it have some dial info we need?
                    let filter = |n: &NodeInfo| {
                        let mut keep = false;
                        for did in n.dial_info_detail_list() {
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

        self.find_preferred_fastest_nodes(
            protocol_types_len * 2 * max_per_type,
            filters,
            |_rti, entry: Option<Arc<BucketEntry>>| {
                NodeRef::new(self.clone(), entry.unwrap().clone(), None)
            },
        )
    }

    /// Retrieve up to N of each type of protocol capable nodes for all crypto kinds
    pub fn find_bootstrap_nodes_filtered(&self, max_per_type: usize) -> Vec<NodeRef> {
        let mut out =
            self.find_bootstrap_nodes_filtered_per_crypto_kind(VALID_CRYPTO_KINDS[0], max_per_type);

        // Merge list of nodes so we don't have duplicates
        for crypto_kind in &VALID_CRYPTO_KINDS[1..] {
            let nrs =
                self.find_bootstrap_nodes_filtered_per_crypto_kind(*crypto_kind, max_per_type);
            'nrloop: for nr in nrs {
                for nro in &out {
                    if nro.same_entry(&nr) {
                        continue 'nrloop;
                    }
                }
                out.push(nr);
            }
        }
        out
    }

    pub fn find_peers_with_sort_and_filter<C, T, O>(
        &self,
        node_count: usize,
        cur_ts: Timestamp,
        filters: VecDeque<RoutingTableEntryFilter>,
        compare: C,
        transform: T,
    ) -> Vec<O>
    where
        C: for<'a, 'b> FnMut(
            &'a RoutingTableInner,
            &'b Option<Arc<BucketEntry>>,
            &'b Option<Arc<BucketEntry>>,
        ) -> core::cmp::Ordering,
        T: for<'r> FnMut(&'r RoutingTableInner, Option<Arc<BucketEntry>>) -> O + Send,
    {
        self.inner
            .read()
            .find_peers_with_sort_and_filter(node_count, cur_ts, filters, compare, transform)
    }

    pub fn find_preferred_fastest_nodes<'a, T, O>(
        &self,
        node_count: usize,
        filters: VecDeque<RoutingTableEntryFilter>,
        transform: T,
    ) -> Vec<O>
    where
        T: for<'r> FnMut(&'r RoutingTableInner, Option<Arc<BucketEntry>>) -> O + Send,
    {
        self.inner
            .read()
            .find_preferred_fastest_nodes(node_count, filters, transform)
    }

    pub fn find_preferred_closest_nodes<'a, T, O>(
        &self,
        node_count: usize,
        node_id: TypedKey,
        filters: VecDeque<RoutingTableEntryFilter>,
        transform: T,
    ) -> VeilidAPIResult<Vec<O>>
    where
        T: for<'r> FnMut(&'r RoutingTableInner, Option<Arc<BucketEntry>>) -> O + Send,
    {
        self.inner
            .read()
            .find_preferred_closest_nodes(node_count, node_id, filters, transform)
    }

    pub fn sort_and_clean_closest_noderefs(
        &self,
        node_id: TypedKey,
        closest_nodes: &[NodeRef],
    ) -> Vec<NodeRef> {
        self.inner
            .read()
            .sort_and_clean_closest_noderefs(node_id, closest_nodes)
    }

    #[instrument(level = "trace", skip(self, peers))]
    pub fn register_find_node_answer(
        &self,
        crypto_kind: CryptoKind,
        peers: Vec<PeerInfo>,
    ) -> Vec<NodeRef> {
        // Register nodes we'd found
        let mut out = Vec::<NodeRef>::with_capacity(peers.len());
        for p in peers {
            // Ensure we're getting back nodes we asked for
            if !p.node_ids().kinds().contains(&crypto_kind) {
                continue;
            }

            // Don't register our own node
            if self.matches_own_node_id(p.node_ids()) {
                continue;
            }

            // Register the node if it's new
            match self.register_node_with_peer_info(RoutingDomain::PublicInternet, p, false) {
                Ok(nr) => out.push(nr),
                Err(e) => {
                    log_rtab!(debug "failed to register node with peer info from find node answer: {}", e);
                }
            }
        }
        out
    }

    #[instrument(level = "trace", skip(self), err)]
    pub async fn find_node(
        &self,
        node_ref: NodeRef,
        node_id: TypedKey,
    ) -> EyreResult<NetworkResult<Vec<NodeRef>>> {
        let rpc_processor = self.rpc_processor();

        let res = network_result_try!(
            rpc_processor
                .clone()
                .rpc_call_find_node(Destination::direct(node_ref), node_id, vec![])
                .await?
        );

        // register nodes we'd found
        Ok(NetworkResult::value(
            self.register_find_node_answer(node_id.kind, res.answer),
        ))
    }

    /// Ask a remote node to list the nodes it has around the current node
    #[instrument(level = "trace", skip(self), err)]
    pub async fn find_self(
        &self,
        crypto_kind: CryptoKind,
        node_ref: NodeRef,
    ) -> EyreResult<NetworkResult<Vec<NodeRef>>> {
        let self_node_id = self.node_id(crypto_kind);
        self.find_node(node_ref, self_node_id).await
    }

    /// Ask a remote node to list the nodes it has around itself
    #[instrument(level = "trace", skip(self), err)]
    pub async fn find_target(
        &self,
        crypto_kind: CryptoKind,
        node_ref: NodeRef,
    ) -> EyreResult<NetworkResult<Vec<NodeRef>>> {
        let Some(target_node_id) = node_ref.node_ids().get(crypto_kind) else {
            bail!("no target node ids for this crypto kind");
        };
        self.find_node(node_ref, target_node_id).await
    }

    #[instrument(level = "trace", skip(self))]
    pub async fn reverse_find_node(&self, crypto_kind: CryptoKind, node_ref: NodeRef, wide: bool) {
        // Ask node to 'find node' on own node so we can get some more nodes near ourselves
        // and then contact those nodes to inform -them- that we exist

        // Ask node for nodes closest to our own node
        let closest_nodes = network_result_value_or_log!(match self.find_self(crypto_kind, node_ref.clone()).await {
            Err(e) => {
                log_rtab!(error
                    "find_self failed for {:?}: {:?}",
                    &node_ref, e
                );
                return;
            }
            Ok(v) => v,
        } => [ format!(": crypto_kind={} node_ref={} wide={}", crypto_kind, node_ref, wide) ] {
            return;
        });

        // Ask each node near us to find us as well
        if wide {
            for closest_nr in closest_nodes {
                network_result_value_or_log!(match self.find_self(crypto_kind, closest_nr.clone()).await {
                    Err(e) => {
                        log_rtab!(error
                            "find_self failed for {:?}: {:?}",
                            &closest_nr, e
                        );
                        continue;
                    }
                    Ok(v) => v,
                } => [ format!(": crypto_kind={} closest_nr={} wide={}", crypto_kind, closest_nr, wide) ] {
                    // Do nothing with non-values
                    continue;
                });
            }
        }
    }
}

impl core::ops::Deref for RoutingTable {
    type Target = RoutingTableUnlockedInner;

    fn deref(&self) -> &Self::Target {
        &self.unlocked_inner
    }
}
