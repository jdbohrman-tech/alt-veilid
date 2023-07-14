use super::*;
use crate::crypto::*;
use alloc::fmt;

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct NodeRefBaseCommon {
    routing_table: RoutingTable,
    entry: Arc<BucketEntry>,
    filter: Option<NodeRefFilter>,
    sequencing: Sequencing,
    #[cfg(feature = "tracking")]
    track_id: usize,
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub trait NodeRefBase: Sized {
    // Common field access
    fn common(&self) -> &NodeRefBaseCommon;
    fn common_mut(&mut self) -> &mut NodeRefBaseCommon;

    // Comparators
    fn same_entry<T: NodeRefBase>(&self, other: &T) -> bool {
        Arc::ptr_eq(&self.common().entry, &other.common().entry)
    }
    fn same_bucket_entry(&self, entry: &Arc<BucketEntry>) -> bool {
        Arc::ptr_eq(&self.common().entry, entry)
    }

    // Implementation-specific operators
    fn operate<T, F>(&self, f: F) -> T
    where
        F: FnOnce(&RoutingTableInner, &BucketEntryInner) -> T;
    fn operate_mut<T, F>(&self, f: F) -> T
    where
        F: FnOnce(&mut RoutingTableInner, &mut BucketEntryInner) -> T;

    // Filtering
    fn filter_ref(&self) -> Option<&NodeRefFilter> {
        self.common().filter.as_ref()
    }

    fn take_filter(&mut self) -> Option<NodeRefFilter> {
        self.common_mut().filter.take()
    }

    fn set_filter(&mut self, filter: Option<NodeRefFilter>) {
        self.common_mut().filter = filter
    }

    fn set_sequencing(&mut self, sequencing: Sequencing) {
        self.common_mut().sequencing = sequencing;
    }
    fn sequencing(&self) -> Sequencing {
        self.common().sequencing
    }

    fn merge_filter(&mut self, filter: NodeRefFilter) {
        let common_mut = self.common_mut();
        if let Some(self_filter) = common_mut.filter.take() {
            common_mut.filter = Some(self_filter.filtered(&filter));
        } else {
            common_mut.filter = Some(filter);
        }
    }

    fn is_filter_dead(&self) -> bool {
        if let Some(filter) = &self.common().filter {
            filter.is_dead()
        } else {
            false
        }
    }

    fn routing_domain_set(&self) -> RoutingDomainSet {
        self.common()
            .filter
            .as_ref()
            .map(|f| f.routing_domain_set)
            .unwrap_or(RoutingDomainSet::all())
    }

    fn dial_info_filter(&self) -> DialInfoFilter {
        self.common()
            .filter
            .as_ref()
            .map(|f| f.dial_info_filter.clone())
            .unwrap_or(DialInfoFilter::all())
    }

    fn best_routing_domain(&self) -> Option<RoutingDomain> {
        self.operate(|rti, e| {
            e.best_routing_domain(
                rti,
                self.common()
                    .filter
                    .as_ref()
                    .map(|f| f.routing_domain_set)
                    .unwrap_or(RoutingDomainSet::all()),
            )
        })
    }

    // Accessors
    fn routing_table(&self) -> RoutingTable {
        self.common().routing_table.clone()
    }
    fn node_ids(&self) -> TypedKeyGroup {
        self.operate(|_rti, e| e.node_ids())
    }
    fn best_node_id(&self) -> TypedKey {
        self.operate(|_rti, e| e.best_node_id())
    }
    fn has_updated_since_last_network_change(&self) -> bool {
        self.operate(|_rti, e| e.has_updated_since_last_network_change())
    }
    fn set_updated_since_last_network_change(&self) {
        self.operate_mut(|_rti, e| e.set_updated_since_last_network_change(true));
    }
    fn update_node_status(&self, routing_domain: RoutingDomain, node_status: NodeStatus) {
        self.operate_mut(|_rti, e| {
            e.update_node_status(routing_domain, node_status);
        });
    }
    fn envelope_support(&self) -> Vec<u8> {
        self.operate(|_rti, e| e.envelope_support())
    }
    fn add_envelope_version(&self, envelope_version: u8) {
        self.operate_mut(|_rti, e| e.add_envelope_version(envelope_version))
    }
    fn set_envelope_support(&self, envelope_support: Vec<u8>) {
        self.operate_mut(|_rti, e| e.set_envelope_support(envelope_support))
    }
    fn best_envelope_version(&self) -> Option<u8> {
        self.operate(|_rti, e| e.best_envelope_version())
    }
    fn state(&self, cur_ts: Timestamp) -> BucketEntryState {
        self.operate(|_rti, e| e.state(cur_ts))
    }
    fn peer_stats(&self) -> PeerStats {
        self.operate(|_rti, e| e.peer_stats().clone())
    }

    // Per-RoutingDomain accessors
    fn make_peer_info(&self, routing_domain: RoutingDomain) -> Option<PeerInfo> {
        self.operate(|_rti, e| e.make_peer_info(routing_domain))
    }
    fn node_info(&self, routing_domain: RoutingDomain) -> Option<NodeInfo> {
        self.operate(|_rti, e| e.node_info(routing_domain).cloned())
    }
    fn signed_node_info_has_valid_signature(&self, routing_domain: RoutingDomain) -> bool {
        self.operate(|_rti, e| {
            e.signed_node_info(routing_domain)
                .map(|sni| sni.has_any_signature())
                .unwrap_or(false)
        })
    }
    fn node_info_ts(&self, routing_domain: RoutingDomain) -> Timestamp {
        self.operate(|_rti, e| {
            e.signed_node_info(routing_domain)
                .map(|sni| sni.timestamp())
                .unwrap_or(0u64.into())
        })
    }
    fn has_seen_our_node_info_ts(
        &self,
        routing_domain: RoutingDomain,
        our_node_info_ts: Timestamp,
    ) -> bool {
        self.operate(|_rti, e| e.has_seen_our_node_info_ts(routing_domain, our_node_info_ts))
    }
    fn set_seen_our_node_info_ts(&self, routing_domain: RoutingDomain, seen_ts: Timestamp) {
        self.operate_mut(|_rti, e| e.set_seen_our_node_info_ts(routing_domain, seen_ts));
    }
    fn network_class(&self, routing_domain: RoutingDomain) -> Option<NetworkClass> {
        self.operate(|_rt, e| e.node_info(routing_domain).map(|n| n.network_class()))
    }
    fn outbound_protocols(&self, routing_domain: RoutingDomain) -> Option<ProtocolTypeSet> {
        self.operate(|_rt, e| e.node_info(routing_domain).map(|n| n.outbound_protocols()))
    }
    fn address_types(&self, routing_domain: RoutingDomain) -> Option<AddressTypeSet> {
        self.operate(|_rt, e| e.node_info(routing_domain).map(|n| n.address_types()))
    }
    fn node_info_outbound_filter(&self, routing_domain: RoutingDomain) -> DialInfoFilter {
        let mut dif = DialInfoFilter::all();
        if let Some(outbound_protocols) = self.outbound_protocols(routing_domain) {
            dif = dif.with_protocol_type_set(outbound_protocols);
        }
        if let Some(address_types) = self.address_types(routing_domain) {
            dif = dif.with_address_type_set(address_types);
        }
        dif
    }
    fn relay(&self, routing_domain: RoutingDomain) -> EyreResult<Option<NodeRef>> {
        self.operate_mut(|rti, e| {
            let Some(sni) = e.signed_node_info(routing_domain) else {
                return Ok(None);
            };
            let Some(rpi) = sni.relay_peer_info() else {
                return Ok(None);
            };
            // If relay is ourselves, then return None, because we can't relay through ourselves
            // and to contact this node we should have had an existing inbound connection
            if rti.unlocked_inner.matches_own_node_id(rpi.node_ids()) {
                bail!("Can't relay though ourselves");
            }

            // Register relay node and return noderef
            let nr =
                rti.register_node_with_peer_info(self.routing_table(), routing_domain, rpi, false)?;
            Ok(Some(nr))
        })
    }

    // Filtered accessors
    fn first_filtered_dial_info_detail(&self) -> Option<DialInfoDetail> {
        let routing_domain_set = self.routing_domain_set();
        let dial_info_filter = self.dial_info_filter();
        let sequencing = self.common().sequencing;
        let (ordered, dial_info_filter) = dial_info_filter.with_sequencing(sequencing);

        let sort = if ordered {
            Some(DialInfoDetail::ordered_sequencing_sort)
        } else {
            None
        };

        if dial_info_filter.is_dead() {
            return None;
        }

        let filter = |did: &DialInfoDetail| did.matches_filter(&dial_info_filter);

        self.operate(|_rt, e| {
            for routing_domain in routing_domain_set {
                if let Some(ni) = e.node_info(routing_domain) {
                    if let Some(did) = ni.first_filtered_dial_info_detail(sort, filter) {
                        return Some(did);
                    }
                }
            }
            None
        })
    }

    fn all_filtered_dial_info_details<F>(&self) -> Vec<DialInfoDetail> {
        let routing_domain_set = self.routing_domain_set();
        let dial_info_filter = self.dial_info_filter();

        let (sort, dial_info_filter) = match self.common().sequencing {
            Sequencing::NoPreference => (None, dial_info_filter),
            Sequencing::PreferOrdered => (
                Some(DialInfoDetail::ordered_sequencing_sort),
                dial_info_filter,
            ),
            Sequencing::EnsureOrdered => (
                Some(DialInfoDetail::ordered_sequencing_sort),
                dial_info_filter.filtered(
                    &DialInfoFilter::all().with_protocol_type_set(ProtocolType::all_ordered_set()),
                ),
            ),
        };

        let mut out = Vec::new();
        self.operate(|_rt, e| {
            for routing_domain in routing_domain_set {
                if let Some(ni) = e.node_info(routing_domain) {
                    let filter = |did: &DialInfoDetail| did.matches_filter(&dial_info_filter);
                    if let Some(did) = ni.first_filtered_dial_info_detail(sort, filter) {
                        out.push(did);
                    }
                }
            }
        });
        out.remove_duplicates();
        out
    }

    /// Get the most recent 'last connection' to this node
    /// Filtered first and then sorted by ordering preference and then by most recent
    fn last_connection(&self) -> Option<ConnectionDescriptor> {
        self.operate(|rti, e| {
            // apply sequencing to filter and get sort
            let sequencing = self.common().sequencing;
            let filter = self.common().filter.clone().unwrap_or_default();
            let (ordered, filter) = filter.with_sequencing(sequencing);
            let mut last_connections = e.last_connections(rti, true, filter);

            if ordered {
                last_connections.sort_by(|a, b| {
                    ProtocolType::ordered_sequencing_sort(a.0.protocol_type(), b.0.protocol_type())
                });
            }

            last_connections.first().map(|x| x.0)
        })
    }

    fn clear_last_connections(&self) {
        self.operate_mut(|_rti, e| e.clear_last_connections())
    }

    fn set_last_connection(&self, connection_descriptor: ConnectionDescriptor, ts: Timestamp) {
        self.operate_mut(|rti, e| {
            e.set_last_connection(connection_descriptor, ts);
            rti.touch_recent_peer(e.best_node_id(), connection_descriptor);
        })
    }

    fn clear_last_connection(&self, connection_descriptor: ConnectionDescriptor) {
        self.operate_mut(|_rti, e| {
            e.clear_last_connection(connection_descriptor);
        })
    }

    fn has_any_dial_info(&self) -> bool {
        self.operate(|_rti, e| {
            for rtd in RoutingDomain::all() {
                if let Some(sni) = e.signed_node_info(rtd) {
                    if sni.has_any_dial_info() {
                        return true;
                    }
                }
            }
            false
        })
    }

    fn stats_question_sent(&self, ts: Timestamp, bytes: Timestamp, expects_answer: bool) {
        self.operate_mut(|rti, e| {
            rti.transfer_stats_accounting().add_up(bytes);
            e.question_sent(ts, bytes, expects_answer);
        })
    }
    fn stats_question_rcvd(&self, ts: Timestamp, bytes: ByteCount) {
        self.operate_mut(|rti, e| {
            rti.transfer_stats_accounting().add_down(bytes);
            e.question_rcvd(ts, bytes);
        })
    }
    fn stats_answer_sent(&self, bytes: ByteCount) {
        self.operate_mut(|rti, e| {
            rti.transfer_stats_accounting().add_up(bytes);
            e.answer_sent(bytes);
        })
    }
    fn stats_answer_rcvd(&self, send_ts: Timestamp, recv_ts: Timestamp, bytes: ByteCount) {
        self.operate_mut(|rti, e| {
            rti.transfer_stats_accounting().add_down(bytes);
            rti.latency_stats_accounting()
                .record_latency(recv_ts.saturating_sub(send_ts));
            e.answer_rcvd(send_ts, recv_ts, bytes);
        })
    }
    fn stats_question_lost(&self) {
        self.operate_mut(|_rti, e| {
            e.question_lost();
        })
    }
    fn stats_failed_to_send(&self, ts: Timestamp, expects_answer: bool) {
        self.operate_mut(|_rti, e| {
            e.failed_to_send(ts, expects_answer);
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////

/// Reference to a routing table entry
/// Keeps entry in the routing table until all references are gone
pub struct NodeRef {
    common: NodeRefBaseCommon,
}

impl NodeRef {
    pub fn new(
        routing_table: RoutingTable,
        entry: Arc<BucketEntry>,
        filter: Option<NodeRefFilter>,
    ) -> Self {
        entry.ref_count.fetch_add(1u32, Ordering::Relaxed);

        Self {
            common: NodeRefBaseCommon {
                routing_table,
                entry,
                filter,
                sequencing: Sequencing::NoPreference,
                #[cfg(feature = "tracking")]
                track_id: entry.track(),
            },
        }
    }

    pub fn filtered_clone(&self, filter: NodeRefFilter) -> Self {
        let mut out = self.clone();
        out.merge_filter(filter);
        out
    }

    pub fn locked<'a>(&self, rti: &'a RoutingTableInner) -> NodeRefLocked<'a> {
        NodeRefLocked::new(rti, self.clone())
    }
    pub fn locked_mut<'a>(&self, rti: &'a mut RoutingTableInner) -> NodeRefLockedMut<'a> {
        NodeRefLockedMut::new(rti, self.clone())
    }
}

impl NodeRefBase for NodeRef {
    fn common(&self) -> &NodeRefBaseCommon {
        &self.common
    }

    fn common_mut(&mut self) -> &mut NodeRefBaseCommon {
        &mut self.common
    }

    fn operate<T, F>(&self, f: F) -> T
    where
        F: FnOnce(&RoutingTableInner, &BucketEntryInner) -> T,
    {
        let inner = &*self.common.routing_table.inner.read();
        self.common.entry.with(inner, f)
    }

    fn operate_mut<T, F>(&self, f: F) -> T
    where
        F: FnOnce(&mut RoutingTableInner, &mut BucketEntryInner) -> T,
    {
        let inner = &mut *self.common.routing_table.inner.write();
        self.common.entry.with_mut(inner, f)
    }
}

impl Clone for NodeRef {
    fn clone(&self) -> Self {
        self.common
            .entry
            .ref_count
            .fetch_add(1u32, Ordering::Relaxed);

        Self {
            common: NodeRefBaseCommon {
                routing_table: self.common.routing_table.clone(),
                entry: self.common.entry.clone(),
                filter: self.common.filter.clone(),
                sequencing: self.common.sequencing,
                #[cfg(feature = "tracking")]
                track_id: self.common.entry.write().track(),
            },
        }
    }
}

impl fmt::Display for NodeRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.common.entry.with_inner(|e| e.best_node_id()))
    }
}

impl fmt::Debug for NodeRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NodeRef")
            .field("node_ids", &self.common.entry.with_inner(|e| e.node_ids()))
            .field("filter", &self.common.filter)
            .field("sequencing", &self.common.sequencing)
            .finish()
    }
}

impl Drop for NodeRef {
    fn drop(&mut self) {
        #[cfg(feature = "tracking")]
        self.common.entry.write().untrack(self.track_id);

        // drop the noderef and queue a bucket kick if it was the last one
        let new_ref_count = self
            .common
            .entry
            .ref_count
            .fetch_sub(1u32, Ordering::Relaxed)
            - 1;
        if new_ref_count == 0 {
            // get node ids with inner unlocked because nothing could be referencing this entry now
            // and we don't know when it will get dropped, possibly inside a lock
            let node_ids = self.common().entry.with_inner(|e| e.node_ids());
            self.common.routing_table.queue_bucket_kicks(node_ids);
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////

/// Locked reference to a routing table entry
/// For internal use inside the RoutingTable module where you have
/// already locked a RoutingTableInner
/// Keeps entry in the routing table until all references are gone
pub struct NodeRefLocked<'a> {
    inner: Mutex<&'a RoutingTableInner>,
    nr: NodeRef,
}

impl<'a> NodeRefLocked<'a> {
    pub fn new(inner: &'a RoutingTableInner, nr: NodeRef) -> Self {
        Self {
            inner: Mutex::new(inner),
            nr,
        }
    }

    pub fn unlocked(&self) -> NodeRef {
        self.nr.clone()
    }
}

impl<'a> NodeRefBase for NodeRefLocked<'a> {
    fn common(&self) -> &NodeRefBaseCommon {
        &self.nr.common
    }

    fn common_mut(&mut self) -> &mut NodeRefBaseCommon {
        &mut self.nr.common
    }

    fn operate<T, F>(&self, f: F) -> T
    where
        F: FnOnce(&RoutingTableInner, &BucketEntryInner) -> T,
    {
        let inner = &*self.inner.lock();
        self.nr.common.entry.with(inner, f)
    }

    fn operate_mut<T, F>(&self, _f: F) -> T
    where
        F: FnOnce(&mut RoutingTableInner, &mut BucketEntryInner) -> T,
    {
        panic!("need to locked_mut() for this operation")
    }
}

impl<'a> fmt::Display for NodeRefLocked<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.nr)
    }
}

impl<'a> fmt::Debug for NodeRefLocked<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NodeRefLocked")
            .field("nr", &self.nr)
            .finish()
    }
}

////////////////////////////////////////////////////////////////////////////////////

/// Mutable locked reference to a routing table entry
/// For internal use inside the RoutingTable module where you have
/// already locked a RoutingTableInner
/// Keeps entry in the routing table until all references are gone
pub struct NodeRefLockedMut<'a> {
    inner: Mutex<&'a mut RoutingTableInner>,
    nr: NodeRef,
}

impl<'a> NodeRefLockedMut<'a> {
    pub fn new(inner: &'a mut RoutingTableInner, nr: NodeRef) -> Self {
        Self {
            inner: Mutex::new(inner),
            nr,
        }
    }

    pub fn unlocked(&self) -> NodeRef {
        self.nr.clone()
    }
}

impl<'a> NodeRefBase for NodeRefLockedMut<'a> {
    fn common(&self) -> &NodeRefBaseCommon {
        &self.nr.common
    }

    fn common_mut(&mut self) -> &mut NodeRefBaseCommon {
        &mut self.nr.common
    }

    fn operate<T, F>(&self, f: F) -> T
    where
        F: FnOnce(&RoutingTableInner, &BucketEntryInner) -> T,
    {
        let inner = &*self.inner.lock();
        self.nr.common.entry.with(inner, f)
    }

    fn operate_mut<T, F>(&self, f: F) -> T
    where
        F: FnOnce(&mut RoutingTableInner, &mut BucketEntryInner) -> T,
    {
        let inner = &mut *self.inner.lock();
        self.nr.common.entry.with_mut(inner, f)
    }
}

impl<'a> fmt::Display for NodeRefLockedMut<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.nr)
    }
}

impl<'a> fmt::Debug for NodeRefLockedMut<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NodeRefLockedMut")
            .field("nr", &self.nr)
            .finish()
    }
}
