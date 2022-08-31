use super::*;

use crate::dht::*;
use crate::xx::*;
use crate::*;

pub type LowLevelProtocolPorts = BTreeSet<(LowLevelProtocolType, AddressType, u16)>;
pub type ProtocolToPortMapping = BTreeMap<(ProtocolType, AddressType), (LowLevelProtocolType, u16)>;
#[derive(Clone, Debug)]
pub struct MappedPortInfo {
    pub low_level_protocol_ports: LowLevelProtocolPorts,
    pub protocol_to_port: ProtocolToPortMapping,
}

impl RoutingTable {
    // Makes a filter that finds nodes with a matching inbound dialinfo
    pub fn make_inbound_dial_info_entry_filter(
        dial_info_filter: DialInfoFilter,
    ) -> impl FnMut(&BucketEntryInner) -> bool {
        // does it have matching public dial info?
        move |e| {
            for rd in RoutingDomain::all() {
                if let Some(ni) = e.node_info(rd) {
                    if ni
                        .first_filtered_dial_info_detail(|did| {
                            did.matches_filter(&dial_info_filter)
                        })
                        .is_some()
                    {
                        return true;
                    }
                }
            }
            false
        }
    }

    // Makes a filter that finds nodes capable of dialing a particular outbound dialinfo
    pub fn make_outbound_dial_info_entry_filter(
        dial_info: DialInfo,
    ) -> impl FnMut(&BucketEntryInner) -> bool {
        // does the node's outbound capabilities match the dialinfo?
        move |e| {
            for rd in RoutingDomain::all() {
                if let Some(ni) = e.node_info(rd) {
                    let mut dif = DialInfoFilter::all();
                    dif = dif.with_protocol_type_set(ni.outbound_protocols);
                    dif = dif.with_address_type_set(ni.address_types);
                    if dial_info.matches_filter(&dif) {
                        return true;
                    }
                }
            }
            false
        }
    }

    // Make a filter that wraps another filter
    pub fn combine_filters<F, G>(mut f1: F, mut f2: G) -> impl FnMut(&BucketEntryInner) -> bool
    where
        F: FnMut(&BucketEntryInner) -> bool,
        G: FnMut(&BucketEntryInner) -> bool,
    {
        move |e| {
            if !f1(e) {
                return false;
            }
            if !f2(e) {
                return false;
            }
            true
        }
    }

    // Retrieve the fastest nodes in the routing table matching an entry filter
    pub fn find_fast_public_nodes_filtered<F>(
        &self,
        node_count: usize,
        mut entry_filter: F,
    ) -> Vec<NodeRef>
    where
        F: FnMut(&BucketEntryInner) -> bool,
    {
        self.find_fastest_nodes(
            // count
            node_count,
            // filter
            Some(|_k: DHTKey, v: Option<Arc<BucketEntry>>| {
                let entry = v.unwrap();
                entry.with(|e| {
                    // skip nodes on local network
                    if e.node_info(RoutingDomain::LocalNetwork).is_some() {
                        return false;
                    }
                    // skip nodes not on public internet
                    if e.node_info(RoutingDomain::PublicInternet).is_none() {
                        return false;
                    }
                    // skip nodes that dont match entry filter
                    entry_filter(e)
                })
            }),
            // transform
            |k: DHTKey, v: Option<Arc<BucketEntry>>| {
                NodeRef::new(self.clone(), k, v.unwrap().clone(), None)
            },
        )
    }

    // Retrieve up to N of each type of protocol capable nodes
    pub fn find_bootstrap_nodes_filtered(&self, max_per_type: usize) -> Vec<NodeRef> {
        let protocol_types = vec![
            ProtocolType::UDP,
            ProtocolType::TCP,
            ProtocolType::WS,
            ProtocolType::WSS,
        ];
        let mut nodes_proto_v4 = vec![0usize, 0usize, 0usize, 0usize];
        let mut nodes_proto_v6 = vec![0usize, 0usize, 0usize, 0usize];

        self.find_fastest_nodes(
            // count
            protocol_types.len() * 2 * max_per_type,
            // filter
            Some(move |_k: DHTKey, v: Option<Arc<BucketEntry>>| {
                let entry = v.unwrap();
                entry.with(|e| {
                    // skip nodes on our local network here
                    if e.node_info(RoutingDomain::LocalNetwork).is_some() {
                        return false;
                    }

                    // does it have some dial info we need?
                    let filter = |n: NodeInfo| {
                        let mut keep = false;
                        for did in n.dial_info_detail_list {
                            if did.dial_info.is_global() {
                                if matches!(did.dial_info.address_type(), AddressType::IPV4) {
                                    for (n, protocol_type) in protocol_types.iter().enumerate() {
                                        if nodes_proto_v4[n] < max_per_type
                                            && did.dial_info.protocol_type() == *protocol_type
                                        {
                                            nodes_proto_v4[n] += 1;
                                            keep = true;
                                        }
                                    }
                                } else if matches!(did.dial_info.address_type(), AddressType::IPV6)
                                {
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
                        }
                        keep
                    };

                    e.node_info(RoutingDomain::PublicInternet)
                        .map(filter)
                        .unwrap_or(false)
                })
            }),
            // transform
            |k: DHTKey, v: Option<Arc<BucketEntry>>| {
                NodeRef::new(self.clone(), k, v.unwrap().clone(), None)
            },
        )
    }

    // Get our own node's peer info (public node info) so we can share it with other nodes
    pub fn get_own_peer_info(&self, routing_domain: RoutingDomain) -> PeerInfo {
        PeerInfo::new(
            NodeId::new(self.node_id()),
            self.get_own_signed_node_info(routing_domain),
        )
    }

    pub fn get_own_signed_node_info(&self, routing_domain: RoutingDomain) -> SignedNodeInfo {
        let node_id = NodeId::new(self.node_id());
        let secret = self.node_id_secret();
        SignedNodeInfo::with_secret(self.get_own_node_info(routing_domain), node_id, &secret)
            .unwrap()
    }

    pub fn get_own_node_info(&self, routing_domain: RoutingDomain) -> NodeInfo {
        let netman = self.network_manager();
        let relay_node = self.relay_node(routing_domain);
        let pc = netman.get_protocol_config();
        NodeInfo {
            network_class: netman
                .get_network_class(routing_domain)
                .unwrap_or(NetworkClass::Invalid),
            outbound_protocols: pc.outbound,
            address_types: pc.family_global,
            min_version: MIN_VERSION,
            max_version: MAX_VERSION,
            dial_info_detail_list: self.dial_info_details(routing_domain),
            relay_peer_info: relay_node.and_then(|rn| rn.peer_info(routing_domain).map(Box::new)),
        }
    }

    pub fn filter_has_valid_signed_node_info(
        &self,
        v: Option<Arc<BucketEntry>>,
        own_peer_info_is_valid: bool,
        opt_routing_domain: Option<RoutingDomain>,
    ) -> bool {
        let routing_table = self.clone();
        match v {
            None => own_peer_info_is_valid,
            Some(entry) => entry.with(|e| e.has_valid_signed_node_info(opt_routing_domain)),
        }
    }

    pub fn transform_to_peer_info(
        &self,
        routing_domain: RoutingDomain,
        k: DHTKey,
        v: Option<Arc<BucketEntry>>,
        own_peer_info: &PeerInfo,
    ) -> PeerInfo {
        let routing_table = self.clone();
        match v {
            None => own_peer_info.clone(),
            Some(entry) => entry.with(|e| e.peer_info(k, routing_domain).unwrap()),
        }
    }

    pub fn find_peers_with_sort_and_filter<F, C, T, O>(
        &self,
        node_count: usize,
        cur_ts: u64,
        mut filter: F,
        compare: C,
        mut transform: T,
    ) -> Vec<O>
    where
        F: FnMut(DHTKey, Option<Arc<BucketEntry>>) -> bool,
        C: FnMut(
            &(DHTKey, Option<Arc<BucketEntry>>),
            &(DHTKey, Option<Arc<BucketEntry>>),
        ) -> core::cmp::Ordering,
        T: FnMut(DHTKey, Option<Arc<BucketEntry>>) -> O,
    {
        let inner = self.inner.read();
        let self_node_id = inner.node_id;

        // collect all the nodes for sorting
        let mut nodes =
            Vec::<(DHTKey, Option<Arc<BucketEntry>>)>::with_capacity(inner.bucket_entry_count + 1);

        // add our own node (only one of there with the None entry)
        if filter(self_node_id, None) {
            nodes.push((self_node_id, None));
        }

        // add all nodes from buckets
        Self::with_entries(&*inner, cur_ts, BucketEntryState::Unreliable, |k, v| {
            // Apply filter
            if filter(k, Some(v.clone())) {
                nodes.push((k, Some(v.clone())));
            }
            Option::<()>::None
        });

        // sort by preference for returning nodes
        nodes.sort_by(compare);

        // return transformed vector for filtered+sorted nodes
        let cnt = usize::min(node_count, nodes.len());
        let mut out = Vec::<O>::with_capacity(cnt);
        for node in nodes {
            let val = transform(node.0, node.1);
            out.push(val);
        }

        out
    }

    pub fn find_fastest_nodes<T, F, O>(
        &self,
        node_count: usize,
        mut filter: Option<F>,
        transform: T,
    ) -> Vec<O>
    where
        F: FnMut(DHTKey, Option<Arc<BucketEntry>>) -> bool,
        T: FnMut(DHTKey, Option<Arc<BucketEntry>>) -> O,
    {
        let cur_ts = intf::get_timestamp();
        let out = self.find_peers_with_sort_and_filter(
            node_count,
            cur_ts,
            // filter
            |k, v| {
                if let Some(entry) = &v {
                    // always filter out dead nodes
                    if entry.with(|e| e.state(cur_ts) == BucketEntryState::Dead) {
                        false
                    } else {
                        filter.as_mut().map(|f| f(k, v)).unwrap_or(true)
                    }
                } else {
                    // always filter out self peer, as it is irrelevant to the 'fastest nodes' search
                    false
                }
            },
            // sort
            |(a_key, a_entry), (b_key, b_entry)| {
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
                ae.with(|ae| {
                    be.with(|be| {
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

    pub fn find_closest_nodes<F, T, O>(
        &self,
        node_id: DHTKey,
        mut filter: Option<F>,
        mut transform: T,
    ) -> Vec<O>
    where
        T: FnMut(DHTKey, Option<Arc<BucketEntry>>) -> O,
        F: FnMut(DHTKey, Option<Arc<BucketEntry>>) -> bool,
    {
        let cur_ts = intf::get_timestamp();
        let node_count = {
            let c = self.config.get();
            c.network.dht.max_find_node_count as usize
        };
        let out = self.find_peers_with_sort_and_filter(
            node_count,
            cur_ts,
            // filter
            |k, v| filter.as_mut().map(|f| f(k, v)).unwrap_or(true),
            // sort
            |(a_key, a_entry), (b_key, b_entry)| {
                // same nodes are always the same
                if a_key == b_key {
                    return core::cmp::Ordering::Equal;
                }

                // reliable nodes come first, pessimistically treating our own node as unreliable
                let ra = a_entry
                    .as_ref()
                    .map_or(false, |x| x.with(|x| x.check_reliable(cur_ts)));
                let rb = b_entry
                    .as_ref()
                    .map_or(false, |x| x.with(|x| x.check_reliable(cur_ts)));
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

    // Build a map of protocols to low level ports
    // This way we can get the set of protocols required to keep our NAT mapping alive for keepalive pings
    // Only one protocol per low level protocol/port combination is required
    // For example, if WS/WSS and TCP protocols are on the same low-level TCP port, only TCP keepalives will be required
    // and we do not need to do WS/WSS keepalive as well. If they are on different ports, then we will need WS/WSS keepalives too.
    pub fn get_mapped_port_info(&self) -> MappedPortInfo {
        let mut low_level_protocol_ports =
            BTreeSet::<(LowLevelProtocolType, AddressType, u16)>::new();
        let mut protocol_to_port =
            BTreeMap::<(ProtocolType, AddressType), (LowLevelProtocolType, u16)>::new();
        let our_dids = self.all_filtered_dial_info_details(
            Some(RoutingDomain::PublicInternet),
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
        MappedPortInfo {
            low_level_protocol_ports,
            protocol_to_port,
        }
    }

    fn make_relay_node_filter(&self) -> impl Fn(&BucketEntryInner) -> bool {
        // Get all our outbound protocol/address types
        let outbound_dif = self
            .network_manager()
            .get_outbound_dial_info_filter(RoutingDomain::PublicInternet);
        let mapped_port_info = self.get_mapped_port_info();

        move |e: &BucketEntryInner| {
            // Ensure this node is not on our local network
            let has_local_dial_info = e
                .local_node_info()
                .map(|l| l.has_dial_info())
                .unwrap_or(false);
            if has_local_dial_info {
                return false;
            }

            // Disqualify nodes that don't cover all our inbound ports for tcp and udp
            // as we need to be able to use the relay for keepalives for all nat mappings
            let mut low_level_protocol_ports = mapped_port_info.low_level_protocol_ports.clone();

            let can_serve_as_relay = e
                .node_info()
                .map(|n| {
                    let dids =
                        n.all_filtered_dial_info_details(|did| did.matches_filter(&outbound_dif));
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
    pub fn find_inbound_relay(&self, cur_ts: u64) -> Option<NodeRef> {
        // Get relay filter function
        let relay_node_filter = self.make_relay_node_filter();

        // Go through all entries and find fastest entry that matches filter function
        let inner = self.inner.read();
        let inner = &*inner;
        let mut best_inbound_relay: Option<(DHTKey, Arc<BucketEntry>)> = None;

        // Iterate all known nodes for candidates
        Self::with_entries(inner, cur_ts, BucketEntryState::Unreliable, |k, v| {
            let v2 = v.clone();
            v.with(|e| {
                // Ensure we have the node's status
                if let Some(node_status) = e.peer_stats().status.clone() {
                    // Ensure the node will relay
                    if node_status.will_relay {
                        // Compare against previous candidate
                        if let Some(best_inbound_relay) = best_inbound_relay.as_mut() {
                            // Less is faster
                            let better = best_inbound_relay.1.with(|best| {
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
            if let Some(rpi) = &p.signed_node_info.node_info.relay_peer_info {
                if rpi.node_id == p.node_id {
                    continue;
                }
            }

            // register the node if it's new
            if let Some(nr) = self.register_node_with_signed_node_info(
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
                .rpc_call_find_node(
                    Destination::Direct(node_ref.clone()),
                    node_id,
                    None,
                    rpc_processor.make_respond_to_sender(node_ref.clone()),
                )
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
        let closest_nodes = network_result_value_or_log!(debug match self.find_self(node_ref.clone()).await {
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
                network_result_value_or_log!(debug match self.find_self(closest_nr.clone()).await {
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
}
