use super::*;

// Field accessors
pub(crate) trait NodeRefAccessorsTrait {
    fn entry(&self) -> Arc<BucketEntry>;
    fn sequencing(&self) -> Sequencing;
    fn routing_domain_set(&self) -> RoutingDomainSet;
    fn filter(&self) -> NodeRefFilter;
    fn take_filter(&mut self) -> NodeRefFilter;
    fn dial_info_filter(&self) -> DialInfoFilter;
    // fn node_info_outbound_filter(&self, routing_domain: RoutingDomain) -> DialInfoFilter;
    // fn is_filter_dead(&self) -> bool;
}

// Operate on entry
pub(crate) trait NodeRefOperateTrait {
    fn operate<T, F>(&self, f: F) -> T
    where
        F: FnOnce(&RoutingTableInner, &BucketEntryInner) -> T;
    fn operate_mut<T, F>(&self, f: F) -> T
    where
        F: FnOnce(&mut RoutingTableInner, &mut BucketEntryInner) -> T;
}

// Common Operations
pub(crate) trait NodeRefCommonTrait: NodeRefAccessorsTrait + NodeRefOperateTrait {
    fn same_entry<T: NodeRefAccessorsTrait>(&self, other: &T) -> bool {
        Arc::ptr_eq(&self.entry(), &other.entry())
    }

    fn same_bucket_entry(&self, entry: &Arc<BucketEntry>) -> bool {
        Arc::ptr_eq(&self.entry(), entry)
    }

    fn node_ids(&self) -> TypedKeyGroup {
        self.operate(|_rti, e| e.node_ids())
    }
    fn best_node_id(&self) -> TypedKey {
        self.operate(|_rti, e| e.best_node_id())
    }

    fn update_node_status(&self, routing_domain: RoutingDomain, node_status: NodeStatus) {
        self.operate_mut(|_rti, e| {
            e.update_node_status(routing_domain, node_status);
        });
    }
    fn best_routing_domain(&self) -> Option<RoutingDomain> {
        self.operate(|rti, e| e.best_routing_domain(rti, self.routing_domain_set()))
    }

    // fn envelope_support(&self) -> Vec<u8> {
    //     self.operate(|_rti, e| e.envelope_support())
    // }
    fn add_envelope_version(&self, envelope_version: u8) {
        self.operate_mut(|_rti, e| e.add_envelope_version(envelope_version))
    }
    // fn set_envelope_support(&self, envelope_support: Vec<u8>) {
    //     self.operate_mut(|_rti, e| e.set_envelope_support(envelope_support))
    // }
    fn best_envelope_version(&self) -> Option<u8> {
        self.operate(|_rti, e| e.best_envelope_version())
    }
    fn state_reason(&self, cur_ts: Timestamp) -> BucketEntryStateReason {
        self.operate(|_rti, e| e.state_reason(cur_ts))
    }
    fn state(&self, cur_ts: Timestamp) -> BucketEntryState {
        self.operate(|_rti, e| e.state(cur_ts))
    }
    fn peer_stats(&self) -> PeerStats {
        self.operate(|_rti, e| e.peer_stats().clone())
    }

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
    fn has_seen_our_node_info_ts(&self, routing_domain: RoutingDomain) -> bool {
        self.operate(|rti, e| {
            let Some(our_node_info_ts) = rti
                .get_published_peer_info(routing_domain)
                .map(|pi| pi.signed_node_info().timestamp())
            else {
                return false;
            };
            e.has_seen_our_node_info_ts(routing_domain, our_node_info_ts)
        })
    }
    fn set_seen_our_node_info_ts(&self, routing_domain: RoutingDomain, seen_ts: Timestamp) {
        self.operate_mut(|_rti, e| e.set_seen_our_node_info_ts(routing_domain, seen_ts));
    }
    // fn network_class(&self, routing_domain: RoutingDomain) -> Option<NetworkClass> {
    //     self.operate(|_rt, e| e.node_info(routing_domain).map(|n| n.network_class()))
    // }
    // fn outbound_protocols(&self, routing_domain: RoutingDomain) -> Option<ProtocolTypeSet> {
    //     self.operate(|_rt, e| e.node_info(routing_domain).map(|n| n.outbound_protocols()))
    // }
    // fn address_types(&self, routing_domain: RoutingDomain) -> Option<AddressTypeSet> {
    //     self.operate(|_rt, e| e.node_info(routing_domain).map(|n| n.address_types()))
    // }

    fn relay(&self, routing_domain: RoutingDomain) -> EyreResult<Option<FilteredNodeRef>> {
        self.operate_mut(|rti, e| {
            let Some(sni) = e.signed_node_info(routing_domain) else {
                return Ok(None);
            };
            let Some(rpi) = sni.relay_peer_info(routing_domain) else {
                return Ok(None);
            };
            // If relay is ourselves, then return None, because we can't relay through ourselves
            // and to contact this node we should have had an existing inbound connection
            if rti.routing_table().matches_own_node_id(rpi.node_ids()) {
                bail!("Can't relay though ourselves");
            }

            // Register relay node and return noderef
            let nr = rti.register_node_with_peer_info(rpi, false)?;
            Ok(Some(nr))
        })
    }
    // DialInfo
    fn first_dial_info_detail(&self) -> Option<DialInfoDetail> {
        let routing_domain_set = self.routing_domain_set();
        let dial_info_filter = self.dial_info_filter();
        let sequencing = self.sequencing();
        let (ordered, dial_info_filter) = dial_info_filter.apply_sequencing(sequencing);
        let sort = ordered.then_some(DialInfoDetail::ordered_sequencing_sort);

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

    fn dial_info_details(&self) -> Vec<DialInfoDetail> {
        let routing_domain_set = self.routing_domain_set();
        let dial_info_filter = self.dial_info_filter();
        let sequencing = self.sequencing();
        let (ordered, dial_info_filter) = dial_info_filter.apply_sequencing(sequencing);
        let sort = ordered.then_some(DialInfoDetail::ordered_sequencing_sort);

        let mut out = Vec::new();

        if dial_info_filter.is_dead() {
            return out;
        }

        let filter = |did: &DialInfoDetail| did.matches_filter(&dial_info_filter);

        self.operate(|_rt, e| {
            for routing_domain in routing_domain_set {
                if let Some(ni) = e.node_info(routing_domain) {
                    let mut dids = ni.filtered_dial_info_details(sort, filter);
                    out.append(&mut dids);
                }
            }
        });
        out.remove_duplicates();
        out
    }

    /// Get the most recent 'last connection' to this node matching the node ref filter
    /// Filtered first and then sorted by ordering preference and then by most recent
    fn last_flow(&self) -> Option<Flow> {
        self.operate(|rti, e| {
            // apply sequencing to filter and get sort
            let sequencing = self.sequencing();
            let filter = self.filter();
            let (ordered, filter) = filter.apply_sequencing(sequencing);
            let mut last_flows = e.last_flows(rti, true, filter);

            if ordered {
                last_flows.sort_by(|a, b| {
                    ProtocolType::ordered_sequencing_sort(a.0.protocol_type(), b.0.protocol_type())
                });
            }

            last_flows.first().map(|x| x.0)
        })
    }

    /// Get all the 'last connection' flows for this node matching the node ref filter
    /// Filtered first and then sorted by ordering preference and then by most recent
    #[expect(dead_code)]
    fn last_flows(&self) -> Vec<Flow> {
        self.operate(|rti, e| {
            // apply sequencing to filter and get sort
            let sequencing = self.sequencing();
            let filter = self.filter();
            let (ordered, filter) = filter.apply_sequencing(sequencing);
            let mut last_flows = e.last_flows(rti, true, filter);

            if ordered {
                last_flows.sort_by(|a, b| {
                    ProtocolType::ordered_sequencing_sort(a.0.protocol_type(), b.0.protocol_type())
                });
            }

            last_flows.into_iter().map(|x| x.0).collect()
        })
    }

    fn clear_last_flows(&self) {
        self.operate_mut(|_rti, e| e.clear_last_flows(self.dial_info_filter()))
    }

    fn set_last_flow(&self, flow: Flow, ts: Timestamp) {
        self.operate_mut(|rti, e| {
            e.set_last_flow(flow, ts);
            rti.touch_recent_peer(e.best_node_id(), flow);
        })
    }

    fn clear_last_flow(&self, flow: Flow) {
        self.operate_mut(|_rti, e| {
            e.remove_last_flow(flow);
        })
    }

    fn is_relaying(&self, routing_domain: RoutingDomain) -> bool {
        self.operate(|rti, e| {
            let Some(relay_ids) = e
                .signed_node_info(routing_domain)
                .map(|sni| sni.relay_ids())
            else {
                return false;
            };
            let our_node_ids = rti.routing_table().node_ids();
            our_node_ids.contains_any(&relay_ids)
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

    fn report_protected_connection_dropped(&self) {
        self.stats_failed_to_send(Timestamp::now(), false);
    }

    fn report_failed_route_test(&self) {
        self.stats_failed_to_send(Timestamp::now(), false);
    }

    fn stats_question_sent(&self, ts: Timestamp, bytes: ByteCount, expects_answer: bool) {
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
    fn stats_lost_answer(&self) {
        self.operate_mut(|_rti, e| {
            e.lost_answer();
        })
    }
    fn stats_failed_to_send(&self, ts: Timestamp, expects_answer: bool) {
        self.operate_mut(|_rti, e| {
            e.failed_to_send(ts, expects_answer);
        })
    }
    fn report_sender_info(
        &self,
        routing_domain: RoutingDomain,
        protocol_type: ProtocolType,
        address_type: AddressType,
        sender_info: SenderInfo,
    ) -> Option<SenderInfo> {
        self.operate_mut(|_rti, e| {
            e.report_sender_info(
                LastSenderInfoKey(routing_domain, protocol_type, address_type),
                sender_info,
            )
        })
    }
}
