use super::*;
use crate::dht::*;
use alloc::fmt;

// Connectionless protocols like UDP are dependent on a NAT translation timeout
// We should ping them with some frequency and 30 seconds is typical timeout
const CONNECTIONLESS_TIMEOUT_SECS: u32 = 29;

pub struct NodeRef {
    routing_table: RoutingTable,
    node_id: DHTKey,
    entry: Arc<BucketEntry>,
    filter: Option<DialInfoFilter>,
    #[cfg(feature = "tracking")]
    track_id: usize,
}

impl NodeRef {
    pub fn new(
        routing_table: RoutingTable,
        node_id: DHTKey,
        entry: Arc<BucketEntry>,
        filter: Option<DialInfoFilter>,
    ) -> Self {
        entry.ref_count.fetch_add(1u32, Ordering::Relaxed);

        Self {
            routing_table,
            node_id,
            entry,
            filter,
            #[cfg(feature = "tracking")]
            track_id: entry.track(),
        }
    }

    pub fn node_id(&self) -> DHTKey {
        self.node_id
    }

    pub fn filter_ref(&self) -> Option<&DialInfoFilter> {
        self.filter.as_ref()
    }

    pub fn take_filter(&mut self) -> Option<DialInfoFilter> {
        self.filter.take()
    }

    pub fn set_filter(&mut self, filter: Option<DialInfoFilter>) {
        self.filter = filter
    }

    pub fn merge_filter(&mut self, filter: DialInfoFilter) {
        if let Some(self_filter) = self.filter.take() {
            self.filter = Some(self_filter.filtered(filter));
        } else {
            self.filter = Some(filter);
        }
    }

    pub fn filtered_clone(&self, filter: DialInfoFilter) -> Self {
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

    // Returns true if some protocols can still pass the filter and false if no protocols remain
    // pub fn filter_protocols(&mut self, protocol_set: ProtocolSet) -> bool {
    //     if protocol_set != ProtocolSet::all() {
    //         let mut dif = self.filter.clone().unwrap_or_default();
    //         dif.protocol_set &= protocol_set;
    //         self.filter = Some(dif);
    //     }
    //     self.filter
    //         .as_ref()
    //         .map(|f| !f.protocol_set.is_empty())
    //         .unwrap_or(true)
    // }

    pub(super) fn operate<T, F>(&self, f: F) -> T
    where
        F: FnOnce(&RoutingTableInner, &BucketEntryInner) -> T,
    {
        let inner = &*self.routing_table.inner.read();
        self.entry.with(|e| f(inner, e))
    }

    pub(super) fn operate_mut<T, F>(&self, f: F) -> T
    where
        F: FnOnce(&mut RoutingTableInner, &mut BucketEntryInner) -> T,
    {
        let inner = &mut *self.routing_table.inner.write();
        self.entry.with_mut(|e| f(inner, e))
    }

    pub fn peer_info(&self, routing_domain: RoutingDomain) -> Option<PeerInfo> {
        self.operate(|_rti, e| e.peer_info(self.node_id(), routing_domain))
    }
    pub fn has_valid_signed_node_info(&self, opt_routing_domain: Option<RoutingDomain>) -> bool {
        self.operate(|_rti, e| e.has_valid_signed_node_info(opt_routing_domain))
    }
    pub fn has_seen_our_node_info(&self, routing_domain: RoutingDomain) -> bool {
        self.operate(|_rti, e| e.has_seen_our_node_info(routing_domain))
    }
    pub fn set_seen_our_node_info(&self, routing_domain: RoutingDomain) {
        self.operate_mut(|_rti, e| e.set_seen_our_node_info(routing_domain, true));
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
        let target_rpi =
            self.operate(|_rt, e| e.node_info(routing_domain).map(|n| n.relay_peer_info))?;
        target_rpi.and_then(|t| {
            // If relay is ourselves, then return None, because we can't relay through ourselves
            // and to contact this node we should have had an existing inbound connection
            if t.node_id.key == self.routing_table.node_id() {
                return None;
            }

            // Register relay node and return noderef
            self.routing_table
                .register_node_with_signed_node_info(
                    routing_domain,
                    t.node_id.key,
                    t.signed_node_info,
                    false,
                )
                .map(|mut nr| {
                    nr.set_filter(self.filter_ref().cloned());
                    nr
                })
        })
    }
    pub fn first_filtered_dial_info_detail(
        &self,
        routing_domain: Option<RoutingDomain>,
    ) -> Option<DialInfoDetail> {
        self.operate(|_rt, e| {
            // Prefer local dial info first unless it is filtered out
            if routing_domain == None || routing_domain == Some(RoutingDomain::LocalNetwork) {
                e.node_info(RoutingDomain::LocalNetwork).and_then(|l| {
                    l.first_filtered_dial_info_detail(|did| {
                        if let Some(filter) = self.filter.as_ref() {
                            did.matches_filter(filter)
                        } else {
                            true
                        }
                    })
                })
            } else {
                None
            }
            .or_else(|| {
                if routing_domain == None || routing_domain == Some(RoutingDomain::PublicInternet) {
                    e.node_info(RoutingDomain::PublicInternet).and_then(|n| {
                        n.first_filtered_dial_info_detail(|did| {
                            if let Some(filter) = self.filter.as_ref() {
                                did.matches_filter(filter)
                            } else {
                                true
                            }
                        })
                    })
                } else {
                    None
                }
            })
        })
    }

    pub fn all_filtered_dial_info_details<F>(
        &self,
        routing_domain: Option<RoutingDomain>,
    ) -> Vec<DialInfoDetail> {
        let mut out = Vec::new();
        self.operate(|_rt, e| {
            // Prefer local dial info first unless it is filtered out
            if routing_domain == None || routing_domain == Some(RoutingDomain::LocalNetwork) {
                if let Some(ni) = e.node_info(RoutingDomain::LocalNetwork) {
                    out.append(&mut ni.all_filtered_dial_info_details(|did| {
                        if let Some(filter) = self.filter.as_ref() {
                            did.matches_filter(filter)
                        } else {
                            true
                        }
                    }))
                }
            }
            if routing_domain == None || routing_domain == Some(RoutingDomain::PublicInternet) {
                if let Some(ni) = e.node_info(RoutingDomain::PublicInternet) {
                    out.append(&mut ni.all_filtered_dial_info_details(|did| {
                        if let Some(filter) = self.filter.as_ref() {
                            did.matches_filter(filter)
                        } else {
                            true
                        }
                    }))
                }
            }
        });
        out.remove_duplicates();
        out
    }

    pub async fn last_connection(&self) -> Option<ConnectionDescriptor> {
        // Get the last connection and the last time we saw anything with this connection
        let (last_connection, last_seen) =
            self.operate(|_rti, e| e.last_connection(self.filter.clone()))?;

        // Should we check the connection table?
        if last_connection.protocol_type().is_connection_oriented() {
            // Look the connection up in the connection manager and see if it's still there
            let connection_manager = self.routing_table.network_manager().connection_manager();
            connection_manager.get_connection(last_connection).await?;
        } else {
            // If this is not connection oriented, then we check our last seen time
            // to see if this mapping has expired (beyond our timeout)
            let cur_ts = intf::get_timestamp();
            if (last_seen + (CONNECTIONLESS_TIMEOUT_SECS as u64 * 1_000_000u64)) < cur_ts {
                return None;
            }
        }
        Some(last_connection)
    }

    pub fn clear_last_connections(&self) {
        self.operate_mut(|_rti, e| e.clear_last_connections())
    }

    pub fn set_last_connection(&self, connection_descriptor: ConnectionDescriptor, ts: u64) {
        self.operate_mut(|_rti, e| e.set_last_connection(connection_descriptor, ts))
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
            rti.self_transfer_stats_accounting.add_up(bytes);
            e.question_sent(ts, bytes, expects_answer);
        })
    }
    pub fn stats_question_rcvd(&self, ts: u64, bytes: u64) {
        self.operate_mut(|rti, e| {
            rti.self_transfer_stats_accounting.add_down(bytes);
            e.question_rcvd(ts, bytes);
        })
    }
    pub fn stats_answer_sent(&self, bytes: u64) {
        self.operate_mut(|rti, e| {
            rti.self_transfer_stats_accounting.add_up(bytes);
            e.answer_sent(bytes);
        })
    }
    pub fn stats_answer_rcvd(&self, send_ts: u64, recv_ts: u64, bytes: u64) {
        self.operate_mut(|rti, e| {
            rti.self_transfer_stats_accounting.add_down(bytes);
            rti.self_latency_stats_accounting
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
            #[cfg(feature = "tracking")]
            track_id: e.track(),
        }
    }
}

impl PartialEq for NodeRef {
    fn eq(&self, other: &Self) -> bool {
        self.node_id == other.node_id
    }
}

impl Eq for NodeRef {}

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
