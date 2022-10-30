use super::*;
use crate::crypto::*;
use alloc::fmt;

// Connectionless protocols like UDP are dependent on a NAT translation timeout
// We should ping them with some frequency and 30 seconds is typical timeout
const CONNECTIONLESS_TIMEOUT_SECS: u32 = 29;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct NodeRefFilter {
    pub routing_domain_set: RoutingDomainSet,
    pub dial_info_filter: DialInfoFilter,
}

impl Default for NodeRefFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeRefFilter {
    pub fn new() -> Self {
        Self {
            routing_domain_set: RoutingDomainSet::all(),
            dial_info_filter: DialInfoFilter::all(),
        }
    }

    pub fn with_routing_domain(mut self, routing_domain: RoutingDomain) -> Self {
        self.routing_domain_set = routing_domain.into();
        self
    }
    pub fn with_routing_domain_set(mut self, routing_domain_set: RoutingDomainSet) -> Self {
        self.routing_domain_set = routing_domain_set;
        self
    }
    pub fn with_dial_info_filter(mut self, dial_info_filter: DialInfoFilter) -> Self {
        self.dial_info_filter = dial_info_filter;
        self
    }
    pub fn with_protocol_type(mut self, protocol_type: ProtocolType) -> Self {
        self.dial_info_filter = self.dial_info_filter.with_protocol_type(protocol_type);
        self
    }
    pub fn with_protocol_type_set(mut self, protocol_set: ProtocolTypeSet) -> Self {
        self.dial_info_filter = self.dial_info_filter.with_protocol_type_set(protocol_set);
        self
    }
    pub fn with_address_type(mut self, address_type: AddressType) -> Self {
        self.dial_info_filter = self.dial_info_filter.with_address_type(address_type);
        self
    }
    pub fn with_address_type_set(mut self, address_set: AddressTypeSet) -> Self {
        self.dial_info_filter = self.dial_info_filter.with_address_type_set(address_set);
        self
    }
    pub fn filtered(mut self, other_filter: &NodeRefFilter) -> Self {
        self.routing_domain_set &= other_filter.routing_domain_set;
        self.dial_info_filter = self
            .dial_info_filter
            .filtered(&other_filter.dial_info_filter);
        self
    }
    pub fn is_dead(&self) -> bool {
        self.dial_info_filter.is_dead() || self.routing_domain_set.is_empty()
    }
}

pub struct NodeRef {
    routing_table: RoutingTable,
    node_id: DHTKey,
    entry: Arc<BucketEntry>,
    filter: Option<NodeRefFilter>,
    sequencing: Sequencing,
    #[cfg(feature = "tracking")]
    track_id: usize,
}

impl NodeRef {
    pub fn new(
        routing_table: RoutingTable,
        node_id: DHTKey,
        entry: Arc<BucketEntry>,
        filter: Option<NodeRefFilter>,
    ) -> Self {
        entry.ref_count.fetch_add(1u32, Ordering::Relaxed);

        Self {
            routing_table,
            node_id,
            entry,
            filter,
            sequencing: Sequencing::NoPreference,
            #[cfg(feature = "tracking")]
            track_id: entry.track(),
        }
    }

    // Operate on entry accessors
    pub(super) fn operate<T, F>(&self, f: F) -> T
    where
        F: FnOnce(&RoutingTableInner, &BucketEntryInner) -> T,
    {
        let inner = &*self.routing_table.inner.read();
        self.entry.with(inner, f)
    }

    pub(super) fn operate_mut<T, F>(&self, f: F) -> T
    where
        F: FnOnce(&mut RoutingTableInner, &mut BucketEntryInner) -> T,
    {
        let inner = &mut *self.routing_table.inner.write();
        self.entry.with_mut(inner, f)
    }

    // Filtering

    pub fn filter_ref(&self) -> Option<&NodeRefFilter> {
        self.filter.as_ref()
    }

    pub fn take_filter(&mut self) -> Option<NodeRefFilter> {
        self.filter.take()
    }

    pub fn set_filter(&mut self, filter: Option<NodeRefFilter>) {
        self.filter = filter
    }

    pub fn set_sequencing(&mut self, sequencing: Sequencing) {
        self.sequencing = sequencing;
    }
    pub fn sequencing(&self) -> Sequencing {
        self.sequencing
    }

    pub fn merge_filter(&mut self, filter: NodeRefFilter) {
        if let Some(self_filter) = self.filter.take() {
            self.filter = Some(self_filter.filtered(&filter));
        } else {
            self.filter = Some(filter);
        }
    }

    pub fn filtered_clone(&self, filter: NodeRefFilter) -> Self {
        let mut out = self.clone();
        out.merge_filter(filter);
        out
    }

    pub fn is_filter_dead(&self) -> bool {
        if let Some(filter) = &self.filter {
            filter.is_dead()
        } else {
            false
        }
    }

    pub fn routing_domain_set(&self) -> RoutingDomainSet {
        self.filter
            .as_ref()
            .map(|f| f.routing_domain_set)
            .unwrap_or(RoutingDomainSet::all())
    }

    pub fn dial_info_filter(&self) -> DialInfoFilter {
        self.filter
            .as_ref()
            .map(|f| f.dial_info_filter.clone())
            .unwrap_or(DialInfoFilter::all())
    }

    pub fn best_routing_domain(&self) -> Option<RoutingDomain> {
        self.operate(|_rti, e| {
            e.best_routing_domain(
                self.filter
                    .as_ref()
                    .map(|f| f.routing_domain_set)
                    .unwrap_or(RoutingDomainSet::all()),
            )
        })
    }

    // Accessors
    pub fn routing_table(&self) -> RoutingTable {
        self.routing_table.clone()
    }
    pub fn node_id(&self) -> DHTKey {
        self.node_id
    }
    pub fn has_updated_since_last_network_change(&self) -> bool {
        self.operate(|_rti, e| e.has_updated_since_last_network_change())
    }
    pub fn set_updated_since_last_network_change(&self) {
        self.operate_mut(|_rti, e| e.set_updated_since_last_network_change(true));
    }
    pub fn update_node_status(&self, node_status: NodeStatus) {
        self.operate_mut(|_rti, e| {
            e.update_node_status(node_status);
        });
    }
    pub fn min_max_version(&self) -> Option<(u8, u8)> {
        self.operate(|_rti, e| e.min_max_version())
    }
    pub fn set_min_max_version(&self, min_max_version: (u8, u8)) {
        self.operate_mut(|_rti, e| e.set_min_max_version(min_max_version))
    }
    pub fn state(&self, cur_ts: u64) -> BucketEntryState {
        self.operate(|_rti, e| e.state(cur_ts))
    }
    pub fn peer_stats(&self) -> PeerStats {
        self.operate(|_rti, e| e.peer_stats().clone())
    }

    // Per-RoutingDomain accessors
    pub fn make_peer_info(&self, routing_domain: RoutingDomain) -> Option<PeerInfo> {
        self.operate(|_rti, e| e.make_peer_info(self.node_id(), routing_domain))
    }
    pub fn node_info(&self, routing_domain: RoutingDomain) -> Option<NodeInfo> {
        self.operate(|_rti, e| e.node_info(routing_domain).cloned())
    }
    pub fn signed_node_info_has_valid_signature(&self, routing_domain: RoutingDomain) -> bool {
        self.operate(|_rti, e| {
            e.signed_node_info(routing_domain)
                .map(|sni| sni.has_valid_signature())
                .unwrap_or(false)
        })
    }
    pub fn has_seen_our_node_info(&self, routing_domain: RoutingDomain) -> bool {
        self.operate(|_rti, e| e.has_seen_our_node_info(routing_domain))
    }
    pub fn set_seen_our_node_info(&self, routing_domain: RoutingDomain) {
        self.operate_mut(|_rti, e| e.set_seen_our_node_info(routing_domain, true));
    }
    pub fn network_class(&self, routing_domain: RoutingDomain) -> Option<NetworkClass> {
        self.operate(|_rt, e| e.node_info(routing_domain).map(|n| n.network_class))
    }
    pub fn outbound_protocols(&self, routing_domain: RoutingDomain) -> Option<ProtocolTypeSet> {
        self.operate(|_rt, e| e.node_info(routing_domain).map(|n| n.outbound_protocols))
    }
    pub fn address_types(&self, routing_domain: RoutingDomain) -> Option<AddressTypeSet> {
        self.operate(|_rt, e| e.node_info(routing_domain).map(|n| n.address_types))
    }
    pub fn node_info_outbound_filter(&self, routing_domain: RoutingDomain) -> DialInfoFilter {
        let mut dif = DialInfoFilter::all();
        if let Some(outbound_protocols) = self.outbound_protocols(routing_domain) {
            dif = dif.with_protocol_type_set(outbound_protocols);
        }
        if let Some(address_types) = self.address_types(routing_domain) {
            dif = dif.with_address_type_set(address_types);
        }
        dif
    }
    pub fn relay(&self, routing_domain: RoutingDomain) -> Option<NodeRef> {
        let target_rpi = self.operate(|_rti, e| {
            e.node_info(routing_domain)
                .map(|n| n.relay_peer_info.as_ref().map(|pi| pi.as_ref().clone()))
        })?;
        target_rpi.and_then(|t| {
            // If relay is ourselves, then return None, because we can't relay through ourselves
            // and to contact this node we should have had an existing inbound connection
            if t.node_id.key == self.routing_table.node_id() {
                return None;
            }

            // Register relay node and return noderef
            self.routing_table.register_node_with_signed_node_info(
                routing_domain,
                t.node_id.key,
                t.signed_node_info,
                false,
            )
        })
    }

    // Filtered accessors
    pub fn first_filtered_dial_info_detail(&self) -> Option<DialInfoDetail> {
        let routing_domain_set = self.routing_domain_set();
        let dial_info_filter = self.dial_info_filter();

        let (sort, dial_info_filter) = match self.sequencing {
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

        self.operate(|_rt, e| {
            for routing_domain in routing_domain_set {
                if let Some(ni) = e.node_info(routing_domain) {
                    let filter = |did: &DialInfoDetail| did.matches_filter(&dial_info_filter);
                    if let Some(did) = ni.first_filtered_dial_info_detail(sort, filter) {
                        return Some(did);
                    }
                }
            }
            None
        })
    }

    pub fn all_filtered_dial_info_details<F>(&self) -> Vec<DialInfoDetail> {
        let routing_domain_set = self.routing_domain_set();
        let dial_info_filter = self.dial_info_filter();

        let (sort, dial_info_filter) = match self.sequencing {
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

    pub fn last_connection(&self) -> Option<ConnectionDescriptor> {
        // Get the last connections and the last time we saw anything with this connection
        // Filtered first and then sorted by most recent
        let last_connections = self.operate(|rti, e| e.last_connections(rti, self.filter.clone()));

        // Do some checks to ensure these are possibly still 'live'
        for (last_connection, last_seen) in last_connections {
            // Should we check the connection table?
            if last_connection.protocol_type().is_connection_oriented() {
                // Look the connection up in the connection manager and see if it's still there
                let connection_manager = self.routing_table.network_manager().connection_manager();
                if connection_manager.get_connection(last_connection).is_some() {
                    return Some(last_connection);
                }
            } else {
                // If this is not connection oriented, then we check our last seen time
                // to see if this mapping has expired (beyond our timeout)
                let cur_ts = intf::get_timestamp();
                if (last_seen + (CONNECTIONLESS_TIMEOUT_SECS as u64 * 1_000_000u64)) >= cur_ts {
                    return Some(last_connection);
                }
            }
        }
        None
    }

    pub fn clear_last_connections(&self) {
        self.operate_mut(|_rti, e| e.clear_last_connections())
    }

    pub fn set_last_connection(&self, connection_descriptor: ConnectionDescriptor, ts: u64) {
        self.operate_mut(|_rti, e| e.set_last_connection(connection_descriptor, ts));
        self.routing_table
            .touch_recent_peer(self.node_id(), connection_descriptor);
    }

    pub fn has_any_dial_info(&self) -> bool {
        self.operate(|_rti, e| {
            for rtd in RoutingDomain::all() {
                if let Some(ni) = e.node_info(rtd) {
                    if ni.has_any_dial_info() {
                        return true;
                    }
                }
            }
            false
        })
    }

    pub fn stats_question_sent(&self, ts: u64, bytes: u64, expects_answer: bool) {
        self.operate_mut(|rti, e| {
            rti.transfer_stats_accounting().add_up(bytes);
            e.question_sent(ts, bytes, expects_answer);
        })
    }
    pub fn stats_question_rcvd(&self, ts: u64, bytes: u64) {
        self.operate_mut(|rti, e| {
            rti.transfer_stats_accounting().add_down(bytes);
            e.question_rcvd(ts, bytes);
        })
    }
    pub fn stats_answer_sent(&self, bytes: u64) {
        self.operate_mut(|rti, e| {
            rti.transfer_stats_accounting().add_up(bytes);
            e.answer_sent(bytes);
        })
    }
    pub fn stats_answer_rcvd(&self, send_ts: u64, recv_ts: u64, bytes: u64) {
        self.operate_mut(|rti, e| {
            rti.transfer_stats_accounting().add_down(bytes);
            rti.latency_stats_accounting()
                .record_latency(recv_ts - send_ts);
            e.answer_rcvd(send_ts, recv_ts, bytes);
        })
    }
    pub fn stats_question_lost(&self) {
        self.operate_mut(|_rti, e| {
            e.question_lost();
        })
    }
    pub fn stats_failed_to_send(&self, ts: u64, expects_answer: bool) {
        self.operate_mut(|_rti, e| {
            e.failed_to_send(ts, expects_answer);
        })
    }
}

impl Clone for NodeRef {
    fn clone(&self) -> Self {
        self.entry.ref_count.fetch_add(1u32, Ordering::Relaxed);

        Self {
            routing_table: self.routing_table.clone(),
            node_id: self.node_id,
            entry: self.entry.clone(),
            filter: self.filter.clone(),
            sequencing: self.sequencing,
            #[cfg(feature = "tracking")]
            track_id: e.track(),
        }
    }
}

// impl PartialEq for NodeRef {
//     fn eq(&self, other: &Self) -> bool {
//         self.node_id == other.node_id
//     }
// }

// impl Eq for NodeRef {}

impl fmt::Display for NodeRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.node_id.encode())
    }
}

impl fmt::Debug for NodeRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NodeRef")
            .field("node_id", &self.node_id)
            .field("filter", &self.filter)
            .field("sequencing", &self.sequencing)
            .finish()
    }
}

impl Drop for NodeRef {
    fn drop(&mut self) {
        #[cfg(feature = "tracking")]
        self.operate(|e| e.untrack(self.track_id));

        // drop the noderef and queue a bucket kick if it was the last one
        let new_ref_count = self.entry.ref_count.fetch_sub(1u32, Ordering::Relaxed) - 1;
        if new_ref_count == 0 {
            self.routing_table.queue_bucket_kick(self.node_id);
        }
    }
}
