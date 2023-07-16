use super::*;
use core::sync::atomic::{AtomicU32, Ordering};


/// Reliable pings are done with increased spacing between pings

/// - Start secs is the number of seconds between the first two pings
const RELIABLE_PING_INTERVAL_START_SECS: u32 = 10;
/// - Max secs is the maximum number of seconds between consecutive pings
const RELIABLE_PING_INTERVAL_MAX_SECS: u32 = 10 * 60;
/// - Multiplier changes the number of seconds between pings over time
///   making it longer as the node becomes more reliable
const RELIABLE_PING_INTERVAL_MULTIPLIER: f64 = 2.0;

/// Unreliable pings are done for a fixed amount of time while the
/// node is given a chance to come back online before it is made dead
/// If a node misses a single ping, it is marked unreliable and must
/// return reliable pings for the duration of the span before being
/// marked reliable again
///
/// - Span is the number of seconds total to attempt to validate the node
const UNRELIABLE_PING_SPAN_SECS: u32 = 60;
/// - Interval is the number of seconds between each ping
const UNRELIABLE_PING_INTERVAL_SECS: u32 = 5;

/// How many times do we try to ping a never-reached node before we call it dead
const NEVER_REACHED_PING_COUNT: u32 = 3;

// Do not change order here, it will mess up other sorts

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum BucketEntryState {
    Dead,
    Unreliable,
    Reliable,
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct LastConnectionKey(ProtocolType, AddressType);

/// Bucket entry information specific to the LocalNetwork RoutingDomain
#[derive(Debug, Serialize, Deserialize)]
pub struct BucketEntryPublicInternet {
    /// The PublicInternet node info
    signed_node_info: Option<Box<SignedNodeInfo>>,
    /// The last node info timestamp of ours that this entry has seen
    last_seen_our_node_info_ts: Timestamp,
    /// Last known node status
    node_status: Option<NodeStatus>,
}

/// Bucket entry information specific to the LocalNetwork RoutingDomain
#[derive(Debug, Serialize, Deserialize)]
pub struct BucketEntryLocalNetwork {
    /// The LocalNetwork node info
    signed_node_info: Option<Box<SignedNodeInfo>>,
    /// The last node info timestamp of ours that this entry has seen
    last_seen_our_node_info_ts: Timestamp,
    /// Last known node status
    node_status: Option<NodeStatus>,
}

/// The data associated with each bucket entry
#[derive(Debug, Serialize, Deserialize)]
pub struct BucketEntryInner {
    /// The node ids matching this bucket entry, with the cryptography versions supported by this node as the 'kind' field
    validated_node_ids: TypedKeyGroup,
    /// The node ids claimed by the remote node that use cryptography versions we do not support
    unsupported_node_ids: TypedKeyGroup,
    /// The set of envelope versions supported by the node inclusive of the requirements of any relay the node may be using
    envelope_support: Vec<u8>,
    /// If this node has updated it's SignedNodeInfo since our network
    /// and dial info has last changed, for example when our IP address changes
    /// Used to determine if we should make this entry 'live' again when we receive a signednodeinfo update that
    /// has the same timestamp, because if we change our own IP address or network class it may be possible for nodes that were
    /// unreachable may now be reachable with the same SignedNodeInfo/DialInfo
    updated_since_last_network_change: bool,
    /// The last connection descriptors used to contact this node, per protocol type
    #[serde(skip)]
    last_connections: BTreeMap<LastConnectionKey, (ConnectionDescriptor, Timestamp)>,
    /// The node info for this entry on the publicinternet routing domain
    public_internet: BucketEntryPublicInternet,
    /// The node info for this entry on the localnetwork routing domain
    local_network: BucketEntryLocalNetwork,
    /// Statistics gathered for the peer
    peer_stats: PeerStats,
    /// The accounting for the latency statistics
    #[serde(skip)]
    latency_stats_accounting: LatencyStatsAccounting,
    /// The accounting for the transfer statistics
    #[serde(skip)]
    transfer_stats_accounting: TransferStatsAccounting,
    /// If the entry is being punished and should be considered dead
    #[serde(skip)]
    is_punished: bool,
    /// Tracking identifier for NodeRef debugging
    #[cfg(feature = "tracking")]
    #[serde(skip)]
    next_track_id: usize,
    /// Backtraces for NodeRef debugging
    #[cfg(feature = "tracking")]
    #[serde(skip)]
    node_ref_tracks: HashMap<usize, backtrace::Backtrace>,
}

impl BucketEntryInner {
    #[cfg(feature = "tracking")]
    pub fn track(&mut self) -> usize {
        let track_id = self.next_track_id;
        self.next_track_id += 1;
        self.node_ref_tracks
            .insert(track_id, backtrace::Backtrace::new_unresolved());
        track_id
    }

    #[cfg(feature = "tracking")]
    pub fn untrack(&mut self, track_id: usize) {
        self.node_ref_tracks.remove(&track_id);
    }

    /// Get all node ids
    pub fn node_ids(&self) -> TypedKeyGroup {
        let mut node_ids = self.validated_node_ids.clone();
        node_ids.add_all(&self.unsupported_node_ids);
        node_ids
    }

    /// Add a node id for a particular crypto kind.
    /// Returns Ok(Some(node)) any previous existing node id associated with that crypto kind
    /// Returns Ok(None) if no previous existing node id was associated with that crypto kind
    /// Results Err() if this operation would add more crypto kinds than we support
    pub fn add_node_id(&mut self, node_id: TypedKey) -> EyreResult<Option<TypedKey>> {
        let total_node_id_count = self.validated_node_ids.len() + self.unsupported_node_ids.len();
        let node_ids = if VALID_CRYPTO_KINDS.contains(&node_id.kind) {
            &mut self.validated_node_ids
        } else {
            &mut self.unsupported_node_ids
        };

        if let Some(old_node_id) = node_ids.get(node_id.kind) {
            // If this was already there we do nothing
            if old_node_id == node_id {
                return Ok(None);
            }
            // Won't change number of crypto kinds
            node_ids.add(node_id);    
            return Ok(Some(old_node_id));
        }
        // Check to ensure we aren't adding more crypto kinds than we support
        if total_node_id_count == MAX_CRYPTO_KINDS {
            bail!("too many crypto kinds for this node");
        }
        node_ids.add(node_id);
        Ok(None)
    }
    pub fn best_node_id(&self) -> TypedKey {
        self.validated_node_ids.best().unwrap()
    }

    /// Get crypto kinds
    pub fn crypto_kinds(&self) -> Vec<CryptoKind> {
        self.validated_node_ids.kinds()
    }
    /// Compare sets of crypto kinds
    pub fn common_crypto_kinds(&self, other: &[CryptoKind]) -> Vec<CryptoKind> {
        common_crypto_kinds(&self.validated_node_ids.kinds(), other)
    }

    /// Capability check
    pub fn has_capabilities(&self, routing_domain: RoutingDomain, capabilities: &[Capability]) -> bool {
        let Some(ni) = self.node_info(routing_domain) else {
            return false;
        };
        ni.has_capabilities(capabilities)
    }

    // Less is faster
    pub fn cmp_fastest(e1: &Self, e2: &Self) -> std::cmp::Ordering {
        // Lower latency to the front
        if let Some(e1_latency) = &e1.peer_stats.latency {
            if let Some(e2_latency) = &e2.peer_stats.latency {
                e1_latency.average.cmp(&e2_latency.average)
            } else {
                std::cmp::Ordering::Less
            }
        } else if e2.peer_stats.latency.is_some() {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    }

    // Less is more reliable then faster
    pub fn cmp_fastest_reliable(cur_ts: Timestamp, e1: &Self, e2: &Self) -> std::cmp::Ordering {
        // Reverse compare so most reliable is at front
        let ret = e2.state(cur_ts).cmp(&e1.state(cur_ts));
        if ret != std::cmp::Ordering::Equal {
            return ret;
        }

        // Lower latency to the front
        if let Some(e1_latency) = &e1.peer_stats.latency {
            if let Some(e2_latency) = &e2.peer_stats.latency {
                e1_latency.average.cmp(&e2_latency.average)
            } else {
                std::cmp::Ordering::Less
            }
        } else if e2.peer_stats.latency.is_some() {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    }

    // Less is more reliable then older
    pub fn cmp_oldest_reliable(cur_ts: Timestamp, e1: &Self, e2: &Self) -> std::cmp::Ordering {
        // Reverse compare so most reliable is at front
        let ret = e2.state(cur_ts).cmp(&e1.state(cur_ts));
        if ret != std::cmp::Ordering::Equal {
            return ret;
        }

        // Lower timestamp to the front, recent or no timestamp is at the end
        if let Some(e1_ts) = &e1.peer_stats.rpc_stats.first_consecutive_seen_ts {
            if let Some(e2_ts) = &e2.peer_stats.rpc_stats.first_consecutive_seen_ts {
                e1_ts.cmp(&e2_ts)
            } else {
                std::cmp::Ordering::Less
            }
        } else if e2.peer_stats.rpc_stats.first_consecutive_seen_ts.is_some() {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    }

    pub fn sort_fastest_reliable_fn(cur_ts: Timestamp) -> impl FnMut(&Self, &Self) -> std::cmp::Ordering {
        move |e1, e2| Self::cmp_fastest_reliable(cur_ts, e1, e2)
    }

    pub fn clear_signed_node_info(&mut self, routing_domain: RoutingDomain) {
        // Get the correct signed_node_info for the chosen routing domain
        let opt_current_sni = match routing_domain {
            RoutingDomain::LocalNetwork => &mut self.local_network.signed_node_info,
            RoutingDomain::PublicInternet => &mut self.public_internet.signed_node_info,
        };
        *opt_current_sni = None;
    }

    pub fn update_signed_node_info(
        &mut self,
        routing_domain: RoutingDomain,
        signed_node_info: SignedNodeInfo,
    ) {
        // Get the correct signed_node_info for the chosen routing domain
        let opt_current_sni = match routing_domain {
            RoutingDomain::LocalNetwork => &mut self.local_network.signed_node_info,
            RoutingDomain::PublicInternet => &mut self.public_internet.signed_node_info,
        };

        // See if we have an existing signed_node_info to update or not
        let mut node_info_changed = false;
        if let Some(current_sni) = opt_current_sni {
            // Always allow overwriting invalid/unsigned node
            if current_sni.has_any_signature() {
                // If the timestamp hasn't changed or is less, ignore this update
                if signed_node_info.timestamp() <= current_sni.timestamp() {
                    // If we received a node update with the same timestamp
                    // we can make this node live again, but only if our network has recently changed
                    // which may make nodes that were unreachable now reachable with the same dialinfo
                    if !self.updated_since_last_network_change
                        && signed_node_info.timestamp() == current_sni.timestamp()
                    {
                        // No need to update the signednodeinfo though since the timestamp is the same
                        // Touch the node and let it try to live again
                        self.updated_since_last_network_change = true;
                        self.touch_last_seen(get_aligned_timestamp());
                    }
                    return;
                }

                // See if anything has changed in this update beside the timestamp
                if signed_node_info.node_info() != current_sni.node_info() {
                    node_info_changed = true;
                }
            }
        }

        // Update the envelope version support we have to use
        let envelope_support = signed_node_info.node_info().envelope_support().to_vec();
        
        // Update the signed node info
        *opt_current_sni = Some(Box::new(signed_node_info));
        self.set_envelope_support(envelope_support);
        self.updated_since_last_network_change = true;
        self.touch_last_seen(get_aligned_timestamp());

        // If we're updating an entry's node info, purge all 
        // but the last connection in our last connections list
        // because the dial info could have changed and its safer to just reconnect.
        // The latest connection would have been the once we got the new node info
        // over so that connection is still valid.
        if node_info_changed {
            self.clear_last_connections_except_latest();
        }
    }

    pub fn has_node_info(&self, routing_domain_set: RoutingDomainSet) -> bool {
        for routing_domain in routing_domain_set {
            // Get the correct signed_node_info for the chosen routing domain
            let opt_current_sni = match routing_domain {
                RoutingDomain::LocalNetwork => &self.local_network.signed_node_info,
                RoutingDomain::PublicInternet => &self.public_internet.signed_node_info,
            };
            if opt_current_sni.is_some() {
                return true;
            }
        }
        false
    }

    pub fn exists_in_routing_domain(
        &self,
        rti: &RoutingTableInner,
        routing_domain: RoutingDomain,
    ) -> bool {
        // Check node info
        if self.has_node_info(routing_domain.into()) {
            return true;
        }

        // Check connections
        let last_connections = self.last_connections(
            rti,
            true,
            NodeRefFilter::from(routing_domain),
        );
        !last_connections.is_empty()
    }

    pub fn node_info(&self, routing_domain: RoutingDomain) -> Option<&NodeInfo> {
        let opt_current_sni = match routing_domain {
            RoutingDomain::LocalNetwork => &self.local_network.signed_node_info,
            RoutingDomain::PublicInternet => &self.public_internet.signed_node_info,
        };
        opt_current_sni.as_ref().map(|s| s.node_info())
    }

    pub fn signed_node_info(&self, routing_domain: RoutingDomain) -> Option<&SignedNodeInfo> {
        let opt_current_sni = match routing_domain {
            RoutingDomain::LocalNetwork => &self.local_network.signed_node_info,
            RoutingDomain::PublicInternet => &self.public_internet.signed_node_info,
        };
        opt_current_sni.as_ref().map(|s| s.as_ref())
    }

    pub fn make_peer_info(&self, routing_domain: RoutingDomain) -> Option<PeerInfo> {
        let opt_current_sni = match routing_domain {
            RoutingDomain::LocalNetwork => &self.local_network.signed_node_info,
            RoutingDomain::PublicInternet => &self.public_internet.signed_node_info,
        };
        // Peer info includes all node ids, even unvalidated ones
        let node_ids = self.node_ids();
        opt_current_sni.as_ref().map(|s| PeerInfo::new(
            node_ids,
            *s.clone(),
        ))
    }

    pub fn best_routing_domain(
        &self,
        rti: &RoutingTableInner,
        routing_domain_set: RoutingDomainSet,
    ) -> Option<RoutingDomain> {
        // Check node info
        for routing_domain in routing_domain_set {
            let opt_current_sni = match routing_domain {
                RoutingDomain::LocalNetwork => &self.local_network.signed_node_info,
                RoutingDomain::PublicInternet => &self.public_internet.signed_node_info,
            };
            if opt_current_sni.is_some() {
                return Some(routing_domain);
            }
        }
        // Check connections
        let mut best_routing_domain: Option<RoutingDomain> = None;
        let last_connections = self.last_connections(
            rti,
            true,
            NodeRefFilter::from(routing_domain_set),
        );
        for lc in last_connections {
            if let Some(rd) =
                rti.routing_domain_for_address(lc.0.remote_address().address())
            {
                if let Some(brd) = best_routing_domain {
                    if rd < brd {
                        best_routing_domain = Some(rd);
                    }
                } else {
                    best_routing_domain = Some(rd);
                }
            }
        }
        best_routing_domain
    }

    fn descriptor_to_key(&self, last_connection: ConnectionDescriptor) -> LastConnectionKey {
        LastConnectionKey(
            last_connection.protocol_type(),
            last_connection.address_type(),
        )
    }

    // Stores a connection descriptor in this entry's table of last connections
    pub fn set_last_connection(&mut self, last_connection: ConnectionDescriptor, timestamp: Timestamp) {
        if self.is_punished {
            // Don't record connection if this entry is currently punished
            return;
        }
        let key = self.descriptor_to_key(last_connection);
        self.last_connections
            .insert(key, (last_connection, timestamp));
    }

     // Removes a connection descriptor in this entry's table of last connections
     pub fn clear_last_connection(&mut self, last_connection: ConnectionDescriptor) {
        let key = self.descriptor_to_key(last_connection);
        self.last_connections
            .remove(&key);
    }

    // Clears the table of last connections to ensure we create new ones and drop any existing ones
    pub fn clear_last_connections(&mut self) {
        self.last_connections.clear();
    }

    // Clears the table of last connections except the most recent one
    pub fn clear_last_connections_except_latest(&mut self) {
        if self.last_connections.len() == 0 {
            // No last_connections
            return;
        }
        let mut dead_keys = Vec::with_capacity(self.last_connections.len()-1);
        let mut most_recent_connection = None;
        let mut most_recent_connection_time = 0u64;
        for (k, v) in &self.last_connections {
            let lct = v.1.as_u64();
            if lct > most_recent_connection_time {
                most_recent_connection = Some(k);
                most_recent_connection_time = lct;
            }
        }
        let Some(most_recent_connection) = most_recent_connection else {
            return;
        };
        for (k, _) in &self.last_connections {
            if k != most_recent_connection {
                dead_keys.push(k.clone());
            }
        }
        for dk in dead_keys {
            self.last_connections.remove(&dk);
        }
    }

    // Gets all the 'last connections' that match a particular filter, and their accompanying timestamps of last use
    pub(super) fn last_connections(
        &self,
        rti: &RoutingTableInner,
        only_live: bool,
        filter: NodeRefFilter,
    ) -> Vec<(ConnectionDescriptor, Timestamp)> {
        let connection_manager =
            rti.unlocked_inner.network_manager.connection_manager();

        let mut out: Vec<(ConnectionDescriptor, Timestamp)> = self
            .last_connections
            .iter()
            .filter_map(|(k, v)| {
                let include = {
                    let remote_address = v.0.remote_address().address();
                    rti.routing_domain_for_address(remote_address).map(|rd| {
                        filter.routing_domain_set.contains(rd)
                            && filter.dial_info_filter.protocol_type_set.contains(k.0)
                            && filter.dial_info_filter.address_type_set.contains(k.1)
                    }).unwrap_or(false)
                };

                if !include {
                    return None;
                }

                if !only_live {
                    return Some(v.clone());
                }

                // Check if the connection is still considered live
                let alive = 
                    // Should we check the connection table?
                    if v.0.protocol_type().is_ordered() {
                        // Look the connection up in the connection manager and see if it's still there
                        connection_manager.get_connection(v.0).is_some()
                    } else {
                        // If this is not connection oriented, then we check our last seen time
                        // to see if this mapping has expired (beyond our timeout)
                        let cur_ts = get_aligned_timestamp();
                        (v.1 + TimestampDuration::new(CONNECTIONLESS_TIMEOUT_SECS as u64 * 1_000_000u64)) >= cur_ts
                    };

                if alive {
                    Some(v.clone())
                } else {
                    None
                }
            })
            .collect();
        // Sort with newest timestamps
        out.sort_by(|a, b| {
            b.1.cmp(&a.1)
        });
        out
    }

    pub fn add_envelope_version(&mut self, envelope_version: u8) {
        if self.envelope_support.contains(&envelope_version) {
            return;
        }
        self.envelope_support.push(envelope_version);
        self.envelope_support.dedup();
        self.envelope_support.sort();
    }

    pub fn set_envelope_support(&mut self, mut envelope_support: Vec<u8>) {
        envelope_support.dedup();
        envelope_support.sort();
        self.envelope_support = envelope_support;
    }

    pub fn envelope_support(&self) -> Vec<u8> {
        self.envelope_support.clone()
    }

    pub fn best_envelope_version(&self) -> Option<u8> {
        self.envelope_support.iter().rev().find(|x| VALID_ENVELOPE_VERSIONS.contains(x)).copied()
    }

    pub fn state(&self, cur_ts: Timestamp) -> BucketEntryState {
        if self.is_punished {
            return BucketEntryState::Dead;
        }
        if self.check_reliable(cur_ts) {
            BucketEntryState::Reliable
        } else if self.check_dead(cur_ts) {
            BucketEntryState::Dead
        } else {
            BucketEntryState::Unreliable
        }
    }
    pub fn set_punished(&mut self, punished: bool) {
        self.is_punished = punished;
        if punished {
            self.clear_last_connections();
        }
    }

    pub fn peer_stats(&self) -> &PeerStats {
        &self.peer_stats
    }

    pub fn update_node_status(&mut self, routing_domain: RoutingDomain, status: NodeStatus) {
        match routing_domain {
            RoutingDomain::LocalNetwork => {
                self.local_network.node_status = Some(status);
            }
            RoutingDomain::PublicInternet => {
                self.public_internet.node_status = Some(status);
            }
        }
    }
    pub fn node_status(&self, routing_domain: RoutingDomain) -> Option<NodeStatus> {
        match routing_domain {
            RoutingDomain::LocalNetwork => self
                .local_network
                .node_status
                .as_ref()
                .map(|ns| ns.clone()),
            RoutingDomain::PublicInternet => self
                .public_internet
                .node_status
                .as_ref()
                .map(|ns| ns.clone()),
        }
    }

    pub fn set_seen_our_node_info_ts(&mut self, routing_domain: RoutingDomain, seen_ts: Timestamp) {
        match routing_domain {
            RoutingDomain::LocalNetwork => {
                self.local_network.last_seen_our_node_info_ts = seen_ts;
            }
            RoutingDomain::PublicInternet => {
                self.public_internet.last_seen_our_node_info_ts = seen_ts;
            }
        }
    }

    pub fn has_seen_our_node_info_ts(
        &self,
        routing_domain: RoutingDomain,
        our_node_info_ts: Timestamp,
    ) -> bool {
        match routing_domain {
            RoutingDomain::LocalNetwork => {
                our_node_info_ts == self.local_network.last_seen_our_node_info_ts
            }
            RoutingDomain::PublicInternet => {
                our_node_info_ts == self.public_internet.last_seen_our_node_info_ts
            }
        }
    }

    pub fn set_updated_since_last_network_change(&mut self, updated: bool) {
        self.updated_since_last_network_change = updated;
    }

    pub fn has_updated_since_last_network_change(&self) -> bool {
        self.updated_since_last_network_change
    }

    ///// stats methods
    // called every ROLLING_TRANSFERS_INTERVAL_SECS seconds
    pub(super) fn roll_transfers(&mut self, last_ts: Timestamp, cur_ts: Timestamp) {
        self.transfer_stats_accounting.roll_transfers(
            last_ts,
            cur_ts,
            &mut self.peer_stats.transfer,
        );
    }

    // Called for every round trip packet we receive
    fn record_latency(&mut self, latency: TimestampDuration) {
        self.peer_stats.latency = Some(self.latency_stats_accounting.record_latency(latency));
    }

    ///// state machine handling
    pub(super) fn check_reliable(&self, cur_ts: Timestamp) -> bool {
        // If we have had any failures to send, this is not reliable
        if self.peer_stats.rpc_stats.failed_to_send > 0 {
            return false;
        }

        // if we have seen the node consistently for longer that UNRELIABLE_PING_SPAN_SECS
        match self.peer_stats.rpc_stats.first_consecutive_seen_ts {
            None => false,
            Some(ts) => {
                cur_ts.saturating_sub(ts) >= TimestampDuration::new(UNRELIABLE_PING_SPAN_SECS as u64 * 1000000u64)
            }
        }
    }
    pub(super) fn check_dead(&self, cur_ts: Timestamp) -> bool {
        // If we have failed to send NEVER_REACHED_PING_COUNT times in a row, the node is dead
        if self.peer_stats.rpc_stats.failed_to_send >= NEVER_REACHED_PING_COUNT {
            return true;
        }
        // if we have not heard from the node at all for the duration of the unreliable ping span
        // a node is not dead if we haven't heard from it yet,
        // but we give it NEVER_REACHED_PING_COUNT chances to ping before we say it's dead
        match self.peer_stats.rpc_stats.last_seen_ts {
            None => self.peer_stats.rpc_stats.recent_lost_answers < NEVER_REACHED_PING_COUNT,
            Some(ts) => {
                cur_ts.saturating_sub(ts) >= TimestampDuration::new(UNRELIABLE_PING_SPAN_SECS as u64 * 1000000u64)
            }
        }
    }

    /// Return the last time we either saw a node, or asked it a question
    fn latest_contact_time(&self) -> Option<Timestamp> {
        self.peer_stats
            .rpc_stats
            .last_seen_ts
            .max(self.peer_stats.rpc_stats.last_question_ts)
    }

    fn needs_constant_ping(&self, cur_ts: Timestamp, interval_us: TimestampDuration) -> bool {
        // If we have not either seen the node in the last 'interval' then we should ping it
        let latest_contact_time = self.latest_contact_time();

        match latest_contact_time {
            None => true,
            Some(latest_contact_time) => {
                // If we haven't done anything with this node in 'interval' seconds
                cur_ts.saturating_sub(latest_contact_time) >= interval_us
            }
        }
    }

    // Check if this node needs a ping right now to validate it is still reachable
    pub(super) fn needs_ping(&self, cur_ts: Timestamp) -> bool {
        // See which ping pattern we are to use
        let state = self.state(cur_ts);

        // If we don't have node status for this node, then we should ping it to get some node status
        for routing_domain in RoutingDomainSet::all() {
            if self.has_node_info(routing_domain.into()) {
                if self.node_status(routing_domain).is_none() {
                    return true;
                }
            }
        }

        match state {
            BucketEntryState::Reliable => {
                // If we are in a reliable state, we need a ping on an exponential scale
                let latest_contact_time = self.latest_contact_time();

                match latest_contact_time {
                    None => {
                        error!("Peer is reliable, but not seen!");
                        true
                    }
                    Some(latest_contact_time) => {
                        let first_consecutive_seen_ts =
                            self.peer_stats.rpc_stats.first_consecutive_seen_ts.unwrap();
                        let start_of_reliable_time = first_consecutive_seen_ts
                            + ((UNRELIABLE_PING_SPAN_SECS - UNRELIABLE_PING_INTERVAL_SECS) as u64
                                * 1_000_000u64);
                        let reliable_cur = cur_ts.saturating_sub(start_of_reliable_time);
                        let reliable_last =
                            latest_contact_time.saturating_sub(start_of_reliable_time);

                        retry_falloff_log(
                            reliable_last.as_u64(),
                            reliable_cur.as_u64(),
                            RELIABLE_PING_INTERVAL_START_SECS as u64 * 1_000_000u64,
                            RELIABLE_PING_INTERVAL_MAX_SECS as u64 * 1_000_000u64,
                            RELIABLE_PING_INTERVAL_MULTIPLIER,
                        )
                    }
                }
            }
            BucketEntryState::Unreliable => {
                // If we are in an unreliable state, we need a ping every UNRELIABLE_PING_INTERVAL_SECS seconds
                self.needs_constant_ping(cur_ts, TimestampDuration::new(UNRELIABLE_PING_INTERVAL_SECS as u64 * 1000000u64))
            }
            BucketEntryState::Dead => false,
        }
    }

    pub(super) fn touch_last_seen(&mut self, ts: Timestamp) {
        // Mark the node as seen
        if self
            .peer_stats
            .rpc_stats
            .first_consecutive_seen_ts
            .is_none()
        {
            self.peer_stats.rpc_stats.first_consecutive_seen_ts = Some(ts);
        }

        self.peer_stats.rpc_stats.last_seen_ts = Some(ts);
    }

    pub(super) fn _state_debug_info(&self, cur_ts: Timestamp) -> String {
        let first_consecutive_seen_ts = if let Some(first_consecutive_seen_ts) =
            self.peer_stats.rpc_stats.first_consecutive_seen_ts
        {
            format!(
                "{}s ago",
                timestamp_to_secs(cur_ts.saturating_sub(first_consecutive_seen_ts).as_u64())
            )
        } else {
            "never".to_owned()
        };
        let last_seen_ts_str = if let Some(last_seen_ts) = self.peer_stats.rpc_stats.last_seen_ts {
            format!(
                "{}s ago",
                timestamp_to_secs(cur_ts.saturating_sub(last_seen_ts).as_u64())
            )
        } else {
            "never".to_owned()
        };

        format!(
            "state: {:?}, first_consecutive_seen_ts: {}, last_seen_ts: {}",
            self.state(cur_ts),
            first_consecutive_seen_ts,
            last_seen_ts_str
        )
    }

    ////////////////////////////////////////////////////////////////
    /// Called when rpc processor things happen

    pub(super) fn question_sent(&mut self, ts: Timestamp, bytes: ByteCount, expects_answer: bool) {
        self.transfer_stats_accounting.add_up(bytes);
        self.peer_stats.rpc_stats.messages_sent += 1;
        self.peer_stats.rpc_stats.failed_to_send = 0;
        if expects_answer {
            self.peer_stats.rpc_stats.questions_in_flight += 1;
            self.peer_stats.rpc_stats.last_question_ts = Some(ts);
        }
    }
    pub(super) fn question_rcvd(&mut self, ts: Timestamp, bytes: ByteCount) {
        self.transfer_stats_accounting.add_down(bytes);
        self.peer_stats.rpc_stats.messages_rcvd += 1;
        self.touch_last_seen(ts);
    }
    pub(super) fn answer_sent(&mut self, bytes: ByteCount) {
        self.transfer_stats_accounting.add_up(bytes);
        self.peer_stats.rpc_stats.messages_sent += 1;
        self.peer_stats.rpc_stats.failed_to_send = 0;
    }
    pub(super) fn answer_rcvd(&mut self, send_ts: Timestamp, recv_ts: Timestamp, bytes: ByteCount) {
        self.transfer_stats_accounting.add_down(bytes);
        self.peer_stats.rpc_stats.messages_rcvd += 1;
        self.peer_stats.rpc_stats.questions_in_flight -= 1;
        self.record_latency(recv_ts.saturating_sub(send_ts));
        self.touch_last_seen(recv_ts);
        self.peer_stats.rpc_stats.recent_lost_answers = 0;
    }
    pub(super) fn question_lost(&mut self) {
        self.peer_stats.rpc_stats.first_consecutive_seen_ts = None;
        self.peer_stats.rpc_stats.questions_in_flight -= 1;
        self.peer_stats.rpc_stats.recent_lost_answers += 1;
    }
    pub(super) fn failed_to_send(&mut self, ts: Timestamp, expects_answer: bool) {
        if expects_answer {
            self.peer_stats.rpc_stats.last_question_ts = Some(ts);
        }
        self.peer_stats.rpc_stats.failed_to_send += 1;
        self.peer_stats.rpc_stats.first_consecutive_seen_ts = None;
    }
}

#[derive(Debug)]
pub struct BucketEntry {
    pub(super) ref_count: AtomicU32,
    inner: RwLock<BucketEntryInner>,
}

impl BucketEntry {
    pub(super) fn new(first_node_id: TypedKey) -> Self {

        // First node id should always be one we support since TypedKeySets are sorted and we must have at least one supported key
        assert!(VALID_CRYPTO_KINDS.contains(&first_node_id.kind));

        let now = get_aligned_timestamp();
        let inner = BucketEntryInner {
            validated_node_ids: TypedKeyGroup::from(first_node_id),
            unsupported_node_ids: TypedKeyGroup::new(),
            envelope_support: Vec::new(),
            updated_since_last_network_change: false,
            last_connections: BTreeMap::new(),
            local_network: BucketEntryLocalNetwork {
                last_seen_our_node_info_ts: Timestamp::new(0u64),
                signed_node_info: None,
                node_status: None,
            },
            public_internet: BucketEntryPublicInternet {
                last_seen_our_node_info_ts: Timestamp::new(0u64),
                signed_node_info: None,
                node_status: None,
            },
            peer_stats: PeerStats {
                time_added: now,
                rpc_stats: RPCStats::default(),
                latency: None,
                transfer: TransferStatsDownUp::default(),
            },
            latency_stats_accounting: LatencyStatsAccounting::new(),
            transfer_stats_accounting: TransferStatsAccounting::new(),
            is_punished: false,
            #[cfg(feature = "tracking")]
            next_track_id: 0,
            #[cfg(feature = "tracking")]
            node_ref_tracks: HashMap::new(),
        };

        Self::new_with_inner(inner)
    }

    pub(super) fn new_with_inner(inner: BucketEntryInner) -> Self {
        Self {
            ref_count: AtomicU32::new(0),
            inner: RwLock::new(inner),
        }
    }

    // Note, that this requires -also- holding the RoutingTable read lock, as an
    // immutable reference to RoutingTableInner must be passed in to get this
    // This ensures that an operation on the routing table can not change entries
    // while it is being read from
    pub fn with<F, R>(&self, rti: &RoutingTableInner, f: F) -> R
    where
        F: FnOnce(&RoutingTableInner, &BucketEntryInner) -> R,
    {
        let inner = self.inner.read();
        f(rti, &*inner)
    }

    // Note, that this requires -also- holding the RoutingTable write lock, as a
    // mutable reference to RoutingTableInner must be passed in to get this
    pub fn with_mut<F, R>(&self, rti: &mut RoutingTableInner, f: F) -> R
    where
        F: FnOnce(&mut RoutingTableInner, &mut BucketEntryInner) -> R,
    {
        let mut inner = self.inner.write();
        f(rti, &mut *inner)
    }

    // Internal inner access for RoutingTableInner only
    pub(super) fn with_inner<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&BucketEntryInner) -> R,
    {
        let inner = self.inner.read();
        f(&*inner)
    }

    // Internal inner access for RoutingTableInner only
    pub(super) fn with_mut_inner<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut BucketEntryInner) -> R,
    {
        let mut inner = self.inner.write();
        f(&mut *inner)
    }
}

impl Drop for BucketEntry {
    fn drop(&mut self) {
        if self.ref_count.load(Ordering::Relaxed) != 0 {
            #[cfg(feature = "tracking")]
            {
                println!("NodeRef Tracking");
                for (id, bt) in &mut self.node_ref_tracks {
                    bt.resolve();
                    println!("Id: {}\n----------------\n{:#?}", id, bt);
                }
            }

            panic!(
                "bucket entry dropped with non-zero refcount: {:#?}",
                &*self.inner.read()
            )
        }
    }
}
