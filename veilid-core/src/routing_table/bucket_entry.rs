use super::*;
use core::sync::atomic::{AtomicU32, Ordering};

/// Reliable pings are done with increased spacing between pings
///
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
/// - Number of consecutive lost answers on an unordered protocol we will
///   tolerate before we call something unreliable
const UNRELIABLE_LOST_ANSWERS_UNORDERED: u32 = 1;
/// - Number of consecutive lost answers on an ordered protocol we will
///   tolerate before we call something unreliable
const UNRELIABLE_LOST_ANSWERS_ORDERED: u32 = 0;

/// Dead nodes are unreachable nodes, not 'never reached' nodes
///
/// How many times do we try to ping a never-reached node before we call it dead
const NEVER_SEEN_PING_COUNT: u32 = 3;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum BucketEntryDeadReason {
    CanNotSend,
    TooManyLostAnswers,
    NoPingResponse,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum BucketEntryUnreliableReason {
    FailedToSend,
    LostAnswers,
    NotSeenConsecutively,
    InUnreliablePingSpan,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum BucketEntryStateReason {
    Punished(PunishmentReason),
    Dead(BucketEntryDeadReason),
    Unreliable(BucketEntryUnreliableReason),
    Reliable,
}

// Do not change order here, it will mess up other sorts
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum BucketEntryState {
    Punished,
    Dead,
    Unreliable,
    Reliable,
}

impl BucketEntryState {
    pub fn is_alive(&self) -> bool {
        match self {
            BucketEntryState::Punished => false,
            BucketEntryState::Dead => false,
            BucketEntryState::Unreliable => true,
            BucketEntryState::Reliable => true,
        }
    }
    pub fn ordering(&self) -> usize {
        match self {
            BucketEntryState::Punished => 0,
            BucketEntryState::Dead => 1,
            BucketEntryState::Unreliable => 2,
            BucketEntryState::Reliable => 3,
        }
    }
}

impl From<BucketEntryStateReason> for BucketEntryState {
    fn from(value: BucketEntryStateReason) -> Self {
        match value {
            BucketEntryStateReason::Punished(_) => BucketEntryState::Punished,
            BucketEntryStateReason::Dead(_) => BucketEntryState::Dead,
            BucketEntryStateReason::Unreliable(_) => BucketEntryState::Unreliable,
            BucketEntryStateReason::Reliable => BucketEntryState::Reliable,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub(crate) struct LastFlowKey(pub ProtocolType, pub AddressType);

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub(crate) struct LastSenderInfoKey(pub RoutingDomain, pub ProtocolType, pub AddressType);

/// Bucket entry information specific to the LocalNetwork RoutingDomain
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct BucketEntryPublicInternet {
    /// The PublicInternet node info
    signed_node_info: Option<Box<SignedNodeInfo>>,
    /// The last node info timestamp of ours that this entry has seen
    last_seen_our_node_info_ts: Timestamp,
    /// Last known node status
    node_status: Option<NodeStatus>,
}

impl fmt::Display for BucketEntryPublicInternet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(sni) = &self.signed_node_info {
            writeln!(f, "signed_node_info:")?;
            write!(f, "    {}", indent_string(sni))?;
        } else {
            writeln!(f, "signed_node_info: None")?;
        }
        writeln!(
            f,
            "last_seen_our_node_info_ts: {}",
            self.last_seen_our_node_info_ts
        )?;
        writeln!(f, "node_status: {:?}", self.node_status)?;
        Ok(())
    }
}

/// Bucket entry information specific to the LocalNetwork RoutingDomain
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct BucketEntryLocalNetwork {
    /// The LocalNetwork node info
    signed_node_info: Option<Box<SignedNodeInfo>>,
    /// The last node info timestamp of ours that this entry has seen
    last_seen_our_node_info_ts: Timestamp,
    /// Last known node status
    node_status: Option<NodeStatus>,
}

impl fmt::Display for BucketEntryLocalNetwork {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(sni) = &self.signed_node_info {
            writeln!(f, "signed_node_info:")?;
            write!(f, "    {}", indent_string(sni))?;
        } else {
            writeln!(f, "signed_node_info: None")?;
        }
        writeln!(
            f,
            "last_seen_our_node_info_ts: {}",
            self.last_seen_our_node_info_ts
        )?;
        writeln!(f, "node_status: {:?}", self.node_status)?;
        Ok(())
    }
}

/// The data associated with each bucket entry
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct BucketEntryInner {
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
    /// The last flows used to contact this node, per protocol type
    #[serde(skip)]
    last_flows: BTreeMap<LastFlowKey, (Flow, Timestamp)>,
    /// Last seen senderinfo per protocol/address type
    #[serde(skip)]
    last_sender_info: HashMap<LastSenderInfoKey, SenderInfo>,
    /// The node info for this entry on the publicinternet routing domain
    public_internet: BucketEntryPublicInternet,
    /// Node location
    #[cfg(feature = "geolocation")]
    #[serde(skip)]
    geolocation_info: GeolocationInfo,
    /// The node info for this entry on the localnetwork routing domain
    local_network: BucketEntryLocalNetwork,
    /// Statistics gathered for the peer
    peer_stats: PeerStats,
    /// The peer info cache for speedy access to fully encapsulated per-routing-domain peer info
    #[serde(skip)]
    peer_info_cache: Mutex<BTreeMap<RoutingDomain, Option<Arc<PeerInfo>>>>,
    /// The accounting for the latency statistics
    #[serde(skip)]
    latency_stats_accounting: LatencyStatsAccounting,
    /// The accounting for the transfer statistics
    #[serde(skip)]
    transfer_stats_accounting: TransferStatsAccounting,
    /// The account for the state and reason statistics
    #[serde(skip)]
    state_stats_accounting: Mutex<StateStatsAccounting>,
    /// RPC answer stats accounting for unordered protocols
    #[serde(skip)]
    answer_stats_accounting_unordered: AnswerStatsAccounting,
    /// RPC answer stats accounting for ordered protocols
    #[serde(skip)]
    answer_stats_accounting_ordered: AnswerStatsAccounting,
    /// If the entry is being punished and should be considered dead
    #[serde(skip)]
    punishment: Option<PunishmentReason>,
    /// Tracking identifier for NodeRef debugging
    #[cfg(feature = "tracking")]
    #[serde(skip)]
    next_track_id: usize,
    /// Backtraces for NodeRef debugging
    #[cfg(feature = "tracking")]
    #[serde(skip)]
    node_ref_tracks: HashMap<usize, backtrace::Backtrace>,
}

impl fmt::Display for BucketEntryInner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "validated_node_ids: {}", self.validated_node_ids)?;
        writeln!(f, "unsupported_node_ids: {}", self.unsupported_node_ids)?;
        writeln!(f, "envelope_support: {:?}", self.envelope_support)?;
        writeln!(
            f,
            "updated_since_last_network_change: {:?}",
            self.updated_since_last_network_change
        )?;
        writeln!(f, "last_flows:")?;
        for lf in &self.last_flows {
            writeln!(
                f,
                "    {:?}/{:?}: {} @ {}",
                lf.0 .0, lf.0 .1, lf.1 .0, lf.1 .1
            )?;
        }
        writeln!(f, "last_sender_info:")?;
        for lsi in &self.last_sender_info {
            writeln!(
                f,
                "    {:?}/{:?}/{:?}: {}",
                lsi.0 .0, lsi.0 .1, lsi.0 .2, lsi.1.socket_address
            )?;
        }
        writeln!(f, "public_internet:")?;
        write!(f, "{}", indent_all_string(&self.public_internet))?;
        writeln!(f, "local_network:")?;
        write!(f, "{}", indent_all_string(&self.local_network))?;
        writeln!(f, "peer_stats:")?;
        write!(f, "{}", indent_all_string(&self.peer_stats))?;
        writeln!(
            f,
            "punishment: {}",
            if let Some(punishment) = self.punishment {
                format!("{:?}", punishment)
            } else {
                "None".to_owned()
            }
        )?;

        Ok(())
    }
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
    /// Returns Ok(None) if no previous existing node id was associated with that crypto kind, or one existed but nothing changed.
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
            // Won't change number of crypto kinds, but the node id changed
            node_ids.add(node_id);

            // Also clear the peerinfo cache since the node ids changed
            let mut pi_cache = self.peer_info_cache.lock();
            pi_cache.clear();

            return Ok(Some(old_node_id));
        }
        // Check to ensure we aren't adding more crypto kinds than we support
        if total_node_id_count == MAX_CRYPTO_KINDS {
            bail!("too many crypto kinds for this node");
        }
        node_ids.add(node_id);

        // Also clear the peerinfo cache since the node ids changed
        let mut pi_cache = self.peer_info_cache.lock();
        pi_cache.clear();

        Ok(None)
    }

    /// Remove a node id for a particular crypto kind.
    /// Returns Some(node) any previous existing node id associated with that crypto kind
    /// Returns None if no previous existing node id was associated with that crypto kind
    pub fn remove_node_id(&mut self, crypto_kind: CryptoKind) -> Option<TypedKey> {
        let node_ids = if VALID_CRYPTO_KINDS.contains(&crypto_kind) {
            &mut self.validated_node_ids
        } else {
            &mut self.unsupported_node_ids
        };

        let opt_dead_id = node_ids.remove(crypto_kind);
        if opt_dead_id.is_some() {
            // Also clear the peerinfo cache since the node ids changed
            let mut pi_cache = self.peer_info_cache.lock();
            pi_cache.clear();
        }

        opt_dead_id
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

    /// All-of capability check
    pub fn has_all_capabilities(
        &self,
        routing_domain: RoutingDomain,
        capabilities: &[Capability],
    ) -> bool {
        let Some(ni) = self.node_info(routing_domain) else {
            return false;
        };
        ni.has_all_capabilities(capabilities)
    }

    /// Any-of capability check
    pub fn has_any_capabilities(
        &self,
        routing_domain: RoutingDomain,
        capabilities: &[Capability],
    ) -> bool {
        let Some(ni) = self.node_info(routing_domain) else {
            return false;
        };
        ni.has_any_capabilities(capabilities)
    }

    // Less is faster
    pub fn cmp_fastest(
        e1: &Self,
        e2: &Self,
        metric: impl Fn(&LatencyStats) -> TimestampDuration,
    ) -> std::cmp::Ordering {
        // Lower latency to the front
        if let Some(e1_latency) = &e1.peer_stats.latency {
            if let Some(e2_latency) = &e2.peer_stats.latency {
                metric(e1_latency).cmp(&metric(e2_latency))
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
    pub fn cmp_fastest_reliable(
        cur_ts: Timestamp,
        e1: &Self,
        e2: &Self,
        metric: impl Fn(&LatencyStats) -> TimestampDuration,
    ) -> std::cmp::Ordering {
        // Reverse compare so most reliable is at front
        let ret = e2.state(cur_ts).cmp(&e1.state(cur_ts));
        if ret != std::cmp::Ordering::Equal {
            return ret;
        }

        // Lower latency to the front
        Self::cmp_fastest(e1, e2, metric)
    }

    // Less is more reliable then older
    pub fn cmp_oldest_reliable(cur_ts: Timestamp, e1: &Self, e2: &Self) -> std::cmp::Ordering {
        // Reverse compare so most reliable is at front
        let ret = e2.state(cur_ts).cmp(&e1.state(cur_ts));
        if ret != std::cmp::Ordering::Equal {
            return ret;
        }

        // Lower timestamp to the front, recent or no timestamp is at the end
        // First check consecutive-ping reliability timestamp
        if let Some(e1_ts) = &e1.peer_stats.rpc_stats.first_consecutive_seen_ts {
            if let Some(e2_ts) = &e2.peer_stats.rpc_stats.first_consecutive_seen_ts {
                e1_ts.cmp(e2_ts)
            } else {
                std::cmp::Ordering::Less
            }
        } else if e2.peer_stats.rpc_stats.first_consecutive_seen_ts.is_some() {
            std::cmp::Ordering::Greater
        } else {
            // Then check 'since added to routing table' timestamp
            e1.peer_stats.time_added.cmp(&e2.peer_stats.time_added)
        }
    }

    pub fn update_signed_node_info(
        &mut self,
        routing_domain: RoutingDomain,
        signed_node_info: &SignedNodeInfo,
    ) -> bool {
        // Get the correct signed_node_info for the chosen routing domain
        let opt_current_sni = match routing_domain {
            RoutingDomain::LocalNetwork => &mut self.local_network.signed_node_info,
            RoutingDomain::PublicInternet => &mut self.public_internet.signed_node_info,
        };

        // See if we have an existing signed_node_info to update or not
        let mut node_info_changed = false;
        let mut had_previous_node_info = false;
        if let Some(current_sni) = opt_current_sni {
            had_previous_node_info = true;

            // Always allow overwriting unsigned node (bootstrap)
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
                        // Let the node try to live again but don't mark it as seen yet
                        self.updated_since_last_network_change = true;
                        self.make_not_dead(Timestamp::now());
                    }
                    return false;
                }

                // See if anything has changed in this update beside the timestamp
                if !signed_node_info.equivalent(current_sni) {
                    node_info_changed = true;
                }
            }
        }

        // Update the envelope version support we have to use
        let envelope_support = signed_node_info.node_info().envelope_support().to_vec();

        // Update the signed node info
        // Let the node try to live again but don't mark it as seen yet
        *opt_current_sni = Some(Box::new(signed_node_info.clone()));
        self.set_envelope_support(envelope_support);
        self.updated_since_last_network_change = true;
        self.make_not_dead(Timestamp::now());

        // Update geolocation info
        #[cfg(feature = "geolocation")]
        {
            self.geolocation_info = signed_node_info.get_geolocation_info(routing_domain);
        }

        // If we're updating an entry's node info, purge all
        // but the last connection in our last connections list
        // because the dial info could have changed and it's safer to just reconnect.
        // The latest connection would have been the one we got the new node info
        // over so that connection is still valid.
        if node_info_changed {
            self.clear_last_flows_except_latest();
        }

        // Clear the peerinfo cache since the node info changed or was added
        if node_info_changed || !had_previous_node_info {
            let mut pi_cache = self.peer_info_cache.lock();
            pi_cache.remove(&routing_domain);
        }

        node_info_changed
    }

    #[cfg(feature = "geolocation")]
    pub(super) fn update_geolocation_info(&mut self) {
        if let Some(ref sni) = self.public_internet.signed_node_info {
            self.geolocation_info = sni.get_geolocation_info(RoutingDomain::PublicInternet);
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
        let last_flows = self.last_flows(rti, true, NodeRefFilter::from(routing_domain));
        !last_flows.is_empty()
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

    pub fn get_peer_info(&self, routing_domain: RoutingDomain) -> Option<Arc<PeerInfo>> {
        // Return cached peer info if we have it
        let mut pi_cache = self.peer_info_cache.lock();
        if let Some(opt_pi) = pi_cache.get(&routing_domain).cloned() {
            return opt_pi;
        }

        // Create a new peerinfo
        let opt_current_sni = match routing_domain {
            RoutingDomain::LocalNetwork => &self.local_network.signed_node_info,
            RoutingDomain::PublicInternet => &self.public_internet.signed_node_info,
        };
        // Peer info includes all node ids, even unvalidated ones
        let node_ids = self.node_ids();
        let opt_pi = opt_current_sni
            .as_ref()
            .map(|s| Arc::new(PeerInfo::new(routing_domain, node_ids, *s.clone())));

        // Cache the peerinfo
        pi_cache.insert(routing_domain, opt_pi.clone());

        // Return the peerinfo
        opt_pi
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
        let last_connections = self.last_flows(rti, true, NodeRefFilter::from(routing_domain_set));
        for lc in last_connections {
            if let Some(rd) = rti.routing_domain_for_address(lc.0.remote_address().address()) {
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

    fn flow_to_key(&self, last_flow: Flow) -> LastFlowKey {
        LastFlowKey(last_flow.protocol_type(), last_flow.address_type())
    }

    // Stores a flow in this entry's table of last flows
    pub(super) fn set_last_flow(&mut self, last_flow: Flow, timestamp: Timestamp) {
        if self.punishment.is_some() {
            // Don't record connection if this entry is currently punished
            return;
        }
        let key = self.flow_to_key(last_flow);
        self.last_flows.insert(key, (last_flow, timestamp));
    }

    // Removes a flow in this entry's table of last flows
    pub(super) fn remove_last_flow(&mut self, last_flow: Flow) {
        let key = self.flow_to_key(last_flow);
        self.last_flows.remove(&key);
    }

    // Clears the table of last flows to ensure we create new ones and drop any existing ones
    // With a DialInfo::all filter specified, only clear the flows that match the filter
    pub(super) fn clear_last_flows(&mut self, dial_info_filter: DialInfoFilter) {
        if dial_info_filter != DialInfoFilter::all() {
            self.last_flows.retain(|k, _v| {
                !(dial_info_filter.protocol_type_set.contains(k.0)
                    && dial_info_filter.address_type_set.contains(k.1))
            })
        } else {
            self.last_flows.clear();
        }
    }

    // Clears the table of last flows except the most recent one
    pub(super) fn clear_last_flows_except_latest(&mut self) {
        if self.last_flows.is_empty() {
            // No last_connections
            return;
        }
        let mut dead_keys = Vec::with_capacity(self.last_flows.len() - 1);
        let mut most_recent_flow = None;
        let mut most_recent_flow_time = 0u64;
        for (k, v) in &self.last_flows {
            let lct = v.1.as_u64();
            if lct > most_recent_flow_time {
                most_recent_flow = Some(k);
                most_recent_flow_time = lct;
            }
        }
        let Some(most_recent_flow) = most_recent_flow else {
            return;
        };
        for k in self.last_flows.keys() {
            if k != most_recent_flow {
                dead_keys.push(k.clone());
            }
        }
        for dk in dead_keys {
            self.last_flows.remove(&dk);
        }
    }

    // Gets all the 'last flows' that match a particular filter, and their accompanying timestamps of last use
    pub(super) fn last_flows(
        &self,
        rti: &RoutingTableInner,
        only_live: bool,
        filter: NodeRefFilter,
    ) -> Vec<(Flow, Timestamp)> {
        let opt_connection_manager = rti.network_manager().opt_connection_manager();

        let mut out: Vec<(Flow, Timestamp)> = self
            .last_flows
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
                    return Some(*v);
                }

                // Check if the connection is still considered live
                let alive =
                    // Should we check the connection table?
                    if v.0.protocol_type().is_ordered() {
                        // Look the connection up in the connection manager and see if it's still there
                        if let Some(connection_manager) = &opt_connection_manager {
                            connection_manager.get_connection(v.0).is_some()
                        } else {
                            false
                        }
                    } else {
                        // If this is not connection oriented, then we check our last seen time
                        // to see if this mapping has expired (beyond our timeout)
                        let cur_ts = Timestamp::now();
                        (v.1 + TimestampDuration::new(CONNECTIONLESS_TIMEOUT_SECS as u64 * 1_000_000u64)) >= cur_ts
                    };

                if alive {
                    Some(*v)
                } else {
                    None
                }
            })
            .collect();
        // Sort with newest timestamps
        out.sort_by(|a, b| b.1.cmp(&a.1));
        out
    }

    pub(super) fn add_envelope_version(&mut self, envelope_version: u8) {
        if self.envelope_support.contains(&envelope_version) {
            return;
        }
        self.envelope_support.push(envelope_version);
        self.envelope_support.sort();
        self.envelope_support.dedup();
    }

    pub(super) fn set_envelope_support(&mut self, mut envelope_support: Vec<u8>) {
        envelope_support.sort();
        envelope_support.dedup();
        self.envelope_support = envelope_support;
    }

    #[expect(dead_code)]
    pub fn envelope_support(&self) -> Vec<u8> {
        self.envelope_support.clone()
    }

    pub fn best_envelope_version(&self) -> Option<u8> {
        self.envelope_support
            .iter()
            .rev()
            .find(|x| VALID_ENVELOPE_VERSIONS.contains(x))
            .copied()
    }

    pub fn state_reason(&self, cur_ts: Timestamp) -> BucketEntryStateReason {
        let reason = if let Some(punished_reason) = self.punishment {
            BucketEntryStateReason::Punished(punished_reason)
        } else if let Some(dead_reason) = self.check_dead(cur_ts) {
            BucketEntryStateReason::Dead(dead_reason)
        } else if let Some(unreliable_reason) = self.check_unreliable(cur_ts) {
            BucketEntryStateReason::Unreliable(unreliable_reason)
        } else {
            BucketEntryStateReason::Reliable
        };

        // record this reason
        self.state_stats_accounting
            .lock()
            .record_state_reason(cur_ts, reason);

        reason
    }

    pub fn state(&self, cur_ts: Timestamp) -> BucketEntryState {
        self.state_reason(cur_ts).into()
    }

    #[cfg(feature = "geolocation")]
    pub fn geolocation_info(&self) -> &GeolocationInfo {
        &self.geolocation_info
    }

    pub fn set_punished(&mut self, punished: Option<PunishmentReason>) {
        self.punishment = punished;
        if punished.is_some() {
            self.clear_last_flows(DialInfoFilter::all());
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
            RoutingDomain::LocalNetwork => self.local_network.node_status.as_ref().cloned(),
            RoutingDomain::PublicInternet => self.public_internet.node_status.as_ref().cloned(),
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

    pub fn reset_updated_since_last_network_change(&mut self) {
        self.updated_since_last_network_change = false;
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

    // Called every UPDATE_STATE_STATS_SECS seconds
    pub(super) fn update_state_stats(&mut self) {
        if let Some(state_stats) = self.state_stats_accounting.lock().take_stats() {
            self.peer_stats.state = state_stats;
        }
    }

    // called every ROLLING_ANSWERS_INTERVAL_SECS seconds
    pub(super) fn roll_answer_stats(&mut self, cur_ts: Timestamp) {
        self.peer_stats.rpc_stats.answer_unordered =
            self.answer_stats_accounting_unordered.roll_answers(cur_ts);
        self.peer_stats.rpc_stats.answer_ordered =
            self.answer_stats_accounting_ordered.roll_answers(cur_ts);
    }

    ///// state machine handling
    pub(super) fn check_unreliable(
        &self,
        cur_ts: Timestamp,
    ) -> Option<BucketEntryUnreliableReason> {
        // If we have had any failures to send, this is not reliable
        if self.peer_stats.rpc_stats.failed_to_send > 0 {
            return Some(BucketEntryUnreliableReason::FailedToSend);
        }

        // If we have had more than UNRELIABLE_LOST_ANSWERS_UNORDERED lost answers recently on an unordered protocol, this is not reliable
        if self.peer_stats.rpc_stats.recent_lost_answers_unordered
            > UNRELIABLE_LOST_ANSWERS_UNORDERED
        {
            return Some(BucketEntryUnreliableReason::LostAnswers);
        }
        // If we have had more than UNRELIABLE_LOST_ANSWERS_ORDERED lost answers recently on an ordered protocol, this is not reliable
        if self.peer_stats.rpc_stats.recent_lost_answers_ordered > UNRELIABLE_LOST_ANSWERS_ORDERED {
            return Some(BucketEntryUnreliableReason::LostAnswers);
        }

        match self.peer_stats.rpc_stats.first_consecutive_seen_ts {
            // If we have not seen seen a node consecutively, it can't be reliable
            None => return Some(BucketEntryUnreliableReason::NotSeenConsecutively),
            // If not have seen the node consistently for longer than UNRELIABLE_PING_SPAN_SECS then it is unreliable
            Some(ts) => {
                let seen_consecutively = cur_ts.saturating_sub(ts)
                    >= TimestampDuration::new(UNRELIABLE_PING_SPAN_SECS as u64 * 1_000_000u64);
                if !seen_consecutively {
                    return Some(BucketEntryUnreliableReason::InUnreliablePingSpan);
                }
            }
        }

        None
    }
    pub(super) fn check_dead(&self, cur_ts: Timestamp) -> Option<BucketEntryDeadReason> {
        // If we have failed to send NEVER_REACHED_PING_COUNT times in a row, the node is dead
        if self.peer_stats.rpc_stats.failed_to_send >= NEVER_SEEN_PING_COUNT {
            return Some(BucketEntryDeadReason::CanNotSend);
        }

        match self.peer_stats.rpc_stats.last_seen_ts {
            // a node is not dead if we haven't heard from it yet,
            // but we give it NEVER_REACHED_PING_COUNT chances to ping before we say it's dead
            None => {
                let no_answers = self.peer_stats.rpc_stats.recent_lost_answers_unordered
                    + self.peer_stats.rpc_stats.recent_lost_answers_ordered
                    >= NEVER_SEEN_PING_COUNT;
                if no_answers {
                    return Some(BucketEntryDeadReason::TooManyLostAnswers);
                }
            }

            // return dead if we have not heard from the node at all for the duration of the unreliable ping span
            // and we have tried to reach it and failed the entire time of unreliable ping span
            Some(ts) => {
                let not_seen = cur_ts.saturating_sub(ts)
                    >= TimestampDuration::new(UNRELIABLE_PING_SPAN_SECS as u64 * 1_000_000u64);
                let no_answers = self.peer_stats.rpc_stats.recent_lost_answers_unordered
                    + self.peer_stats.rpc_stats.recent_lost_answers_ordered
                    >= (UNRELIABLE_PING_SPAN_SECS / UNRELIABLE_PING_INTERVAL_SECS);
                if not_seen && no_answers {
                    return Some(BucketEntryDeadReason::NoPingResponse);
                }
            }
        }

        None
    }

    /// Return the last time we asked a node a question
    fn last_outbound_contact_time(&self) -> Option<Timestamp> {
        // This is outbound and inbound contact time which may be a reasonable optimization for nodes that have
        // a very low rate of 'lost answers', but for now we are reverting this to ensure outbound connectivity before
        // we claim a node is reliable
        //
        // self.peer_stats
        //     .rpc_stats
        //     .last_seen_ts
        //     .max(self.peer_stats.rpc_stats.last_question_ts)

        self.peer_stats.rpc_stats.last_question_ts
    }

    /// Return the last time we asked a node a question
    // fn last_question_time(&self) -> Option<Timestamp> {
    //     self.peer_stats.rpc_stats.last_question_ts
    // }

    fn needs_constant_ping(&self, cur_ts: Timestamp, interval_us: TimestampDuration) -> bool {
        // If we have not either seen the node in the last 'interval' then we should ping it
        let latest_contact_time = self.last_outbound_contact_time();

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

        match state {
            BucketEntryState::Reliable => {
                // If we are in a reliable state, we need a ping on an exponential scale
                let latest_contact_time = self.last_outbound_contact_time();

                match latest_contact_time {
                    None => {
                        error!("Peer is reliable, but not seen!");
                        true
                    }
                    Some(latest_contact_time) => {
                        let first_consecutive_seen_ts =
                            self.peer_stats.rpc_stats.first_consecutive_seen_ts.unwrap();
                        let start_of_reliable_time = first_consecutive_seen_ts
                            + TimestampDuration::new_secs(
                                UNRELIABLE_PING_SPAN_SECS - UNRELIABLE_PING_INTERVAL_SECS,
                            );
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
                self.needs_constant_ping(
                    cur_ts,
                    TimestampDuration::new(UNRELIABLE_PING_INTERVAL_SECS as u64 * 1_000_000u64),
                )
            }
            BucketEntryState::Dead => {
                error!("Should not be asking this for dead nodes");
                false
            }
            BucketEntryState::Punished => {
                error!("Should not be asking this for punished nodes");
                false
            }
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

    pub(super) fn make_not_dead(&mut self, cur_ts: Timestamp) {
        self.peer_stats.rpc_stats.last_seen_ts = None;
        self.peer_stats.rpc_stats.failed_to_send = 0;
        self.peer_stats.rpc_stats.recent_lost_answers_unordered = 0;
        self.peer_stats.rpc_stats.recent_lost_answers_ordered = 0;
        assert!(self.check_dead(cur_ts).is_none());
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
            self.state_reason(cur_ts),
            first_consecutive_seen_ts,
            last_seen_ts_str
        )
    }

    ////////////////////////////////////////////////////////////////
    /// Called when rpc processor things happen

    pub(super) fn question_sent(
        &mut self,
        ts: Timestamp,
        bytes: ByteCount,
        expects_answer: bool,
        ordered: bool,
    ) {
        self.transfer_stats_accounting.add_up(bytes);
        if ordered {
            self.answer_stats_accounting_ordered.record_question(ts);
        } else {
            self.answer_stats_accounting_unordered.record_question(ts);
        }
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
    pub(super) fn answer_rcvd(
        &mut self,
        send_ts: Timestamp,
        recv_ts: Timestamp,
        bytes: ByteCount,
        ordered: bool,
    ) {
        self.transfer_stats_accounting.add_down(bytes);
        if ordered {
            self.answer_stats_accounting_ordered.record_answer(recv_ts);
            self.peer_stats.rpc_stats.recent_lost_answers_ordered = 0;
        } else {
            self.answer_stats_accounting_unordered
                .record_answer(recv_ts);
            self.peer_stats.rpc_stats.recent_lost_answers_unordered = 0;
        }
        self.peer_stats.rpc_stats.messages_rcvd += 1;
        self.peer_stats.rpc_stats.questions_in_flight -= 1;
        self.record_latency(recv_ts.saturating_sub(send_ts));
        self.touch_last_seen(recv_ts);
    }
    pub(super) fn lost_answer(&mut self, ordered: bool) {
        let cur_ts = Timestamp::now();
        if ordered {
            self.answer_stats_accounting_ordered
                .record_lost_answer(cur_ts);
            self.peer_stats.rpc_stats.recent_lost_answers_ordered += 1;
            if self.peer_stats.rpc_stats.recent_lost_answers_ordered
                > UNRELIABLE_LOST_ANSWERS_ORDERED
            {
                self.peer_stats.rpc_stats.first_consecutive_seen_ts = None;
            }
        } else {
            self.answer_stats_accounting_unordered
                .record_lost_answer(cur_ts);
            self.peer_stats.rpc_stats.recent_lost_answers_unordered += 1;
            if self.peer_stats.rpc_stats.recent_lost_answers_unordered
                > UNRELIABLE_LOST_ANSWERS_UNORDERED
            {
                self.peer_stats.rpc_stats.first_consecutive_seen_ts = None;
            }
        }
        self.peer_stats.rpc_stats.questions_in_flight -= 1;
    }
    pub(super) fn failed_to_send(&mut self, ts: Timestamp, expects_answer: bool) {
        if expects_answer {
            self.peer_stats.rpc_stats.last_question_ts = Some(ts);
        }
        self.peer_stats.rpc_stats.failed_to_send += 1;
        self.peer_stats.rpc_stats.first_consecutive_seen_ts = None;
    }
    pub(super) fn report_sender_info(
        &mut self,
        key: LastSenderInfoKey,
        sender_info: SenderInfo,
    ) -> Option<SenderInfo> {
        let last_sender_info = self.last_sender_info.insert(key, sender_info);
        if last_sender_info != Some(sender_info) {
            // Return last senderinfo if this new one is different
            last_sender_info
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub(crate) struct BucketEntry {
    pub(super) ref_count: AtomicU32,
    inner: RwLock<BucketEntryInner>,
}

impl BucketEntry {
    pub(super) fn new(first_node_id: TypedKey) -> Self {
        // First node id should always be one we support since TypedKeySets are sorted and we must have at least one supported key
        assert!(VALID_CRYPTO_KINDS.contains(&first_node_id.kind));

        let now = Timestamp::now();
        let inner = BucketEntryInner {
            validated_node_ids: TypedKeyGroup::from(first_node_id),
            unsupported_node_ids: TypedKeyGroup::new(),
            envelope_support: Vec::new(),
            updated_since_last_network_change: false,
            last_flows: BTreeMap::new(),
            last_sender_info: HashMap::new(),
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
            #[cfg(feature = "geolocation")]
            geolocation_info: Default::default(),
            peer_stats: PeerStats {
                time_added: now,
                rpc_stats: RPCStats::default(),
                latency: None,
                transfer: TransferStatsDownUp::default(),
                state: StateStats::default(),
            },
            peer_info_cache: Mutex::new(BTreeMap::new()),
            latency_stats_accounting: LatencyStatsAccounting::new(),
            transfer_stats_accounting: TransferStatsAccounting::new(),
            state_stats_accounting: Mutex::new(StateStatsAccounting::new()),
            answer_stats_accounting_ordered: AnswerStatsAccounting::new(),
            answer_stats_accounting_unordered: AnswerStatsAccounting::new(),
            punishment: None,
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
        f(rti, &inner)
    }

    // Note, that this requires -also- holding the RoutingTable write lock, as a
    // mutable reference to RoutingTableInner must be passed in to get this
    pub fn with_mut<F, R>(&self, rti: &mut RoutingTableInner, f: F) -> R
    where
        F: FnOnce(&mut RoutingTableInner, &mut BucketEntryInner) -> R,
    {
        let mut inner = self.inner.write();
        f(rti, &mut inner)
    }

    // Internal inner access for RoutingTableInner only
    pub(super) fn with_inner<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&BucketEntryInner) -> R,
    {
        let inner = self.inner.read();
        f(&inner)
    }

    // Internal inner access for RoutingTableInner only
    pub(super) fn with_mut_inner<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut BucketEntryInner) -> R,
    {
        let mut inner = self.inner.write();
        f(&mut inner)
    }
}

impl Drop for BucketEntry {
    fn drop(&mut self) {
        if self.ref_count.load(Ordering::Acquire) != 0 {
            #[cfg(feature = "tracking")]
            {
                veilid_log!(self info "NodeRef Tracking");
                for (id, bt) in &mut self.node_ref_tracks {
                    bt.resolve();
                    veilid_log!(self info "Id: {}\n----------------\n{:#?}", id, bt);
                }
            }

            panic!(
                "bucket entry dropped with non-zero refcount: {:#?}",
                &*self.inner.read()
            )
        }
    }
}
