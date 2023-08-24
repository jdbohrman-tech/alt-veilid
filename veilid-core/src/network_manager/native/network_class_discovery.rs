/// Detect NetworkClass and DialInfo for the PublicInternet RoutingDomain
/// Also performs UPNP/IGD mapping if enabled and possible
use super::*;
use futures_util::stream::FuturesUnordered;
use futures_util::FutureExt;
use stop_token::future::FutureExt as StopTokenFutureExt;

const PORT_MAP_VALIDATE_TRY_COUNT: usize = 3;
const PORT_MAP_VALIDATE_DELAY_MS: u32 = 500;
const PORT_MAP_TRY_COUNT: usize = 3;

struct DetectedPublicDialInfo {
    dial_info: DialInfo,
    class: DialInfoClass,
}
struct DiscoveryContextInner {
    // per-protocol
    intf_addrs: Option<Vec<SocketAddress>>,
    protocol_type: Option<ProtocolType>,
    address_type: Option<AddressType>,
    // first node contacted
    external_1_dial_info: Option<DialInfo>,
    external_1_address: Option<SocketAddress>,
    node_1: Option<NodeRef>,
    // detected public dialinfo
    detected_network_class: Option<NetworkClass>,
    detected_public_dial_info: Option<DetectedPublicDialInfo>,
}

#[derive(Clone)]
pub struct DiscoveryContext {
    routing_table: RoutingTable,
    net: Network,
    inner: Arc<Mutex<DiscoveryContextInner>>,
}

#[derive(Clone, Debug)]
struct DetectedDialInfo {
    dial_info: DialInfo,
    dial_info_class: DialInfoClass,
    network_class: NetworkClass,
}

impl DiscoveryContext {
    pub fn new(routing_table: RoutingTable, net: Network) -> Self {
        Self {
            routing_table,
            net,
            inner: Arc::new(Mutex::new(DiscoveryContextInner {
                // per-protocol
                intf_addrs: None,
                protocol_type: None,
                address_type: None,
                external_1_dial_info: None,
                external_1_address: None,
                node_1: None,
                detected_network_class: None,
                detected_public_dial_info: None,
            })),
        }
    }

    ///////
    // Utilities

    // Pick the best network class we have seen so far
    pub fn set_detected_network_class(&self, network_class: NetworkClass) {
        let mut inner = self.inner.lock();
        debug!(target: "net",
            protocol_type=?inner.protocol_type,
            address_type=?inner.address_type,
            ?network_class,
            "set_detected_network_class"
        );
        inner.detected_network_class = Some(network_class);
    }

    pub fn set_detected_public_dial_info(&self, dial_info: DialInfo, class: DialInfoClass) {
        let mut inner = self.inner.lock();
        debug!(target: "net",
            protocol_type=?inner.protocol_type,
            address_type=?inner.address_type,
            ?dial_info,
            ?class,
            "set_detected_public_dial_info"
        );
        inner.detected_public_dial_info = Some(DetectedPublicDialInfo { dial_info, class });
    }

    // Ask for a public address check from a particular noderef
    // This is done over the normal port using RPC
    #[instrument(level = "trace", skip(self), ret)]
    async fn request_public_address(&self, node_ref: NodeRef) -> Option<SocketAddress> {
        let rpc = self.routing_table.rpc_processor();

        let res = network_result_value_or_log!(match rpc.rpc_call_status(Destination::direct(node_ref.clone())).await {
                Ok(v) => v,
                Err(e) => {
                    log_net!(error
                        "failed to get status answer from {:?}: {}",
                        node_ref, e
                    );
                    return None;
                }
            } => [ format!(": node_ref={}", node_ref) ] {
                return None;
            }
        );

        log_net!(
            "request_public_address {:?}: Value({:?})",
            node_ref,
            res.answer
        );
        res.answer.map(|si| si.socket_address)
    }

    // find fast peers with a particular address type, and ask them to tell us what our external address is
    // This is done over the normal port using RPC
    #[instrument(level = "trace", skip(self), ret)]
    async fn discover_external_address(
        &self,
        protocol_type: ProtocolType,
        address_type: AddressType,
        ignore_node_ids: Option<TypedKeyGroup>,
    ) -> Option<(SocketAddress, NodeRef)> {
        let node_count = {
            let config = self.routing_table.network_manager().config();
            let c = config.get();
            c.network.dht.max_find_node_count as usize
        };
        let routing_domain = RoutingDomain::PublicInternet;

        // Build an filter that matches our protocol and address type
        // and excludes relayed nodes so we can get an accurate external address
        let dial_info_filter = DialInfoFilter::all()
            .with_protocol_type(protocol_type)
            .with_address_type(address_type);
        let inbound_dial_info_entry_filter = RoutingTable::make_inbound_dial_info_entry_filter(
            routing_domain,
            dial_info_filter.clone(),
        );
        let disallow_relays_filter = Box::new(
            move |rti: &RoutingTableInner, v: Option<Arc<BucketEntry>>| {
                let v = v.unwrap();
                v.with(rti, |_rti, e| {
                    if let Some(n) = e.signed_node_info(routing_domain) {
                        n.relay_ids().is_empty()
                    } else {
                        false
                    }
                })
            },
        ) as RoutingTableEntryFilter;
        let will_validate_dial_info_filter = Box::new(
            move |rti: &RoutingTableInner, v: Option<Arc<BucketEntry>>| {
                let entry = v.unwrap();
                entry.with(rti, move |_rti, e| {
                    e.node_info(routing_domain)
                        .map(|ni| {
                            ni.has_capability(CAP_VALIDATE_DIAL_INFO)
                                && ni.is_fully_direct_inbound()
                        })
                        .unwrap_or(false)
                })
            },
        ) as RoutingTableEntryFilter;

        let mut filters = VecDeque::from([
            inbound_dial_info_entry_filter,
            disallow_relays_filter,
            will_validate_dial_info_filter,
        ]);
        if let Some(ignore_node_ids) = ignore_node_ids {
            let ignore_nodes_filter = Box::new(
                move |rti: &RoutingTableInner, v: Option<Arc<BucketEntry>>| {
                    let v = v.unwrap();
                    v.with(rti, |_rti, e| !e.node_ids().contains_any(&ignore_node_ids))
                },
            ) as RoutingTableEntryFilter;
            filters.push_back(ignore_nodes_filter);
        }

        // Find public nodes matching this filter
        let peers = self
            .routing_table
            .find_fast_public_nodes_filtered(node_count, filters);
        if peers.is_empty() {
            log_net!(debug
                "no external address detection peers of type {:?}:{:?}",
                protocol_type,
                address_type
            );
            return None;
        }

        // For each peer, ask them for our public address, filtering on desired dial info
        for mut peer in peers {
            peer.set_filter(Some(
                NodeRefFilter::new()
                    .with_routing_domain(routing_domain)
                    .with_dial_info_filter(dial_info_filter.clone()),
            ));
            if let Some(sa) = self.request_public_address(peer.clone()).await {
                return Some((sa, peer));
            }
        }
        log_net!(debug "no peers responded with an external address");
        None
    }

    // This pulls the already-detected local interface dial info from the routing table
    #[instrument(level = "trace", skip(self), ret)]
    fn get_local_addresses(
        &self,
        protocol_type: ProtocolType,
        address_type: AddressType,
    ) -> Vec<SocketAddress> {
        let filter = DialInfoFilter::all()
            .with_protocol_type(protocol_type)
            .with_address_type(address_type);
        self.routing_table
            .dial_info_details(RoutingDomain::LocalNetwork)
            .iter()
            .filter_map(|did| {
                if did.dial_info.matches_filter(&filter) {
                    Some(did.dial_info.socket_address())
                } else {
                    None
                }
            })
            .collect()
    }

    #[instrument(level = "trace", skip(self), ret)]
    async fn validate_dial_info(
        &self,
        node_ref: NodeRef,
        dial_info: DialInfo,
        redirect: bool,
    ) -> bool {
        let rpc = self.routing_table.rpc_processor();

        // asking for node validation doesn't have to use the dial info filter of the dial info we are validating
        let mut node_ref = node_ref.clone();
        node_ref.set_filter(None);

        // ask the node to send us a dial info validation receipt
        let out = rpc
            .rpc_call_validate_dial_info(node_ref.clone(), dial_info, redirect)
            .await
            .map_err(logthru_net!(
                "failed to send validate_dial_info to {:?}",
                node_ref
            ))
            .unwrap_or(false);
        out
    }

    #[instrument(level = "trace", skip(self), ret)]
    async fn try_upnp_port_mapping(&self) -> Option<DialInfo> {
        let (pt, llpt, at, external_address_1, node_1, local_port) = {
            let inner = self.inner.lock();
            let pt = inner.protocol_type.unwrap();
            let llpt = pt.low_level_protocol_type();
            let at = inner.address_type.unwrap();
            let external_address_1 = inner.external_1_address.unwrap();
            let node_1 = inner.node_1.as_ref().unwrap().clone();
            let local_port = self.net.get_local_port(pt).unwrap();
            (pt, llpt, at, external_address_1, node_1, local_port)
        };

        let mut tries = 0;
        loop {
            tries += 1;

            // Attempt a port mapping. If this doesn't succeed, it's not going to
            let Some(mapped_external_address) = self
                .net
                .unlocked_inner
                .igd_manager
                .map_any_port(llpt, at, local_port, Some(external_address_1.to_ip_addr()))
                .await else
            {
                return None;
            };

            // Make dial info from the port mapping
            let external_mapped_dial_info =
                self.make_dial_info(SocketAddress::from_socket_addr(mapped_external_address), pt);

            // Attempt to validate the port mapping
            let mut validate_tries = 0;
            loop {
                validate_tries += 1;

                // Ensure people can reach us. If we're firewalled off, this is useless
                if self
                    .validate_dial_info(node_1.clone(), external_mapped_dial_info.clone(), false)
                    .await
                {
                    return Some(external_mapped_dial_info);
                }

                if validate_tries == PORT_MAP_VALIDATE_TRY_COUNT {
                    log_net!(debug "UPNP port mapping succeeded but port {}/{} is still unreachable.\nretrying\n",
                    local_port, match llpt {
                        LowLevelProtocolType::UDP => "udp",
                        LowLevelProtocolType::TCP => "tcp",
                    });
                    sleep(PORT_MAP_VALIDATE_DELAY_MS).await
                } else {
                    break;
                }
            }

            // Release the mapping if we're still unreachable
            let _ = self
                .net
                .unlocked_inner
                .igd_manager
                .unmap_port(llpt, at, external_address_1.port())
                .await;

            if tries == PORT_MAP_TRY_COUNT {
                warn!("UPNP port mapping succeeded but port {}/{} is still unreachable.\nYou may need to add a local firewall allowed port on this machine.\n",
                    local_port, match llpt {
                        LowLevelProtocolType::UDP => "udp",
                        LowLevelProtocolType::TCP => "tcp",
                    }
                );
                break;
            }
        }
        None
    }

    #[instrument(level = "trace", skip(self), ret)]
    async fn try_port_mapping(&self) -> Option<DialInfo> {
        let enable_upnp = {
            let c = self.net.config.get();
            c.network.upnp
        };

        if enable_upnp {
            return self.try_upnp_port_mapping().await;
        }

        None
    }

    fn make_dial_info(&self, addr: SocketAddress, protocol_type: ProtocolType) -> DialInfo {
        match protocol_type {
            ProtocolType::UDP => DialInfo::udp(addr),
            ProtocolType::TCP => DialInfo::tcp(addr),
            ProtocolType::WS => {
                let c = self.net.config.get();
                DialInfo::try_ws(
                    addr,
                    format!("ws://{}/{}", addr, c.network.protocol.ws.path),
                )
                .unwrap()
            }
            ProtocolType::WSS => panic!("none of the discovery functions are used for wss"),
        }
    }

    ///////
    // Per-protocol discovery routines

    #[instrument(level = "trace", skip(self))]
    pub fn protocol_begin(&self, protocol_type: ProtocolType, address_type: AddressType) {
        // Get our interface addresses
        let intf_addrs = self.get_local_addresses(protocol_type, address_type);

        let mut inner = self.inner.lock();
        inner.intf_addrs = Some(intf_addrs);
        inner.protocol_type = Some(protocol_type);
        inner.address_type = Some(address_type);
        inner.external_1_dial_info = None;
        inner.external_1_address = None;
        inner.node_1 = None;
    }

    // Get our first node's view of our external IP address via normal RPC
    #[instrument(level = "trace", skip(self), ret)]
    pub async fn protocol_get_external_address_1(&self) -> bool {
        let (protocol_type, address_type) = {
            let inner = self.inner.lock();
            (inner.protocol_type.unwrap(), inner.address_type.unwrap())
        };

        // Get our external address from some fast node, call it node 1
        let (external_1, node_1) = match self
            .discover_external_address(protocol_type, address_type, None)
            .await
        {
            None => {
                // If we can't get an external address, exit but don't throw an error so we can try again later
                log_net!(debug "couldn't get external address 1 for {:?} {:?}", protocol_type, address_type);
                return false;
            }
            Some(v) => v,
        };
        let external_1_dial_info = self.make_dial_info(external_1, protocol_type);

        let mut inner = self.inner.lock();
        inner.external_1_dial_info = Some(external_1_dial_info);
        inner.external_1_address = Some(external_1);
        inner.node_1 = Some(node_1);

        log_net!(debug
            "external_1_dial_info: {:?}\nexternal_1_address: {:?}\nnode_1: {:?}",
            inner.external_1_dial_info, inner.external_1_address, inner.node_1
        );

        true
    }

    // If we know we are not behind NAT, check our firewall status
    #[instrument(level = "trace", skip(self), err)]
    pub async fn protocol_process_no_nat(&self) -> EyreResult<()> {
        // Do these detections in parallel, but with ordering preference
        let mut ord = FuturesOrdered::new();

        // UPNP Automatic Mapping
        ///////////
        let this = self.clone();
        let do_mapped_fut: SendPinBoxFuture<Option<DetectedDialInfo>> = Box::pin(async move {
            // Attempt a port mapping via all available and enabled mechanisms
            // Try this before the direct mapping in the event that we are restarting
            // and may not have recorded a mapping created the last time
            if let Some(external_mapped_dial_info) = this.try_port_mapping().await {
                // Got a port mapping, let's use it
                return Some(DetectedDialInfo {
                    dial_info: external_mapped_dial_info.clone(),
                    dial_info_class: DialInfoClass::Mapped,
                    network_class: NetworkClass::InboundCapable,
                });
            }
            None
        });
        ord.push_back(do_mapped_fut);

        let this = self.clone();
        let do_direct_fut: SendPinBoxFuture<Option<DetectedDialInfo>> = Box::pin(async move {
            let (node_1, external_1_dial_info) = {
                let inner = this.inner.lock();
                (
                    inner.node_1.as_ref().unwrap().clone(),
                    inner.external_1_dial_info.as_ref().unwrap().clone(),
                )
            };
            // Do a validate_dial_info on the external address from a redirected node
            if this
                .validate_dial_info(node_1.clone(), external_1_dial_info.clone(), true)
                .await
            {
                // Add public dial info with Direct dialinfo class
                Some(DetectedDialInfo {
                    dial_info: external_1_dial_info.clone(),
                    dial_info_class: DialInfoClass::Direct,
                    network_class: NetworkClass::InboundCapable,
                })
            } else {
                // Add public dial info with Blocked dialinfo class
                Some(DetectedDialInfo {
                    dial_info: external_1_dial_info.clone(),
                    dial_info_class: DialInfoClass::Blocked,
                    network_class: NetworkClass::InboundCapable,
                })
            }
        });

        ord.push_back(do_direct_fut);

        while let Some(res) = ord.next().await {
            if let Some(ddi) = res {
                self.set_detected_public_dial_info(ddi.dial_info, ddi.dial_info_class);
                self.set_detected_network_class(ddi.network_class);
                break;
            }
        }

        Ok(())
    }

    // If we know we are behind NAT check what kind
    #[instrument(level = "trace", skip(self), ret, err)]
    pub async fn protocol_process_nat(&self) -> EyreResult<bool> {
        // Get the external dial info for our use here
        let (node_1, external_1_dial_info, external_1_address, protocol_type, address_type) = {
            let inner = self.inner.lock();
            (
                inner.node_1.as_ref().unwrap().clone(),
                inner.external_1_dial_info.as_ref().unwrap().clone(),
                inner.external_1_address.unwrap(),
                inner.protocol_type.unwrap(),
                inner.address_type.unwrap(),
            )
        };

        // Do these detections in parallel, but with ordering preference
        let mut ord = FuturesOrdered::new();

        // UPNP Automatic Mapping
        ///////////
        let this = self.clone();
        let do_mapped_fut: SendPinBoxFuture<Option<DetectedDialInfo>> = Box::pin(async move {
            // Attempt a port mapping via all available and enabled mechanisms
            // Try this before the direct mapping in the event that we are restarting
            // and may not have recorded a mapping created the last time
            if let Some(external_mapped_dial_info) = this.try_port_mapping().await {
                // Got a port mapping, let's use it
                return Some(DetectedDialInfo {
                    dial_info: external_mapped_dial_info.clone(),
                    dial_info_class: DialInfoClass::Mapped,
                    network_class: NetworkClass::InboundCapable,
                });
            }
            None
        });
        ord.push_back(do_mapped_fut);

        // Manual Mapping Detection
        ///////////
        let this = self.clone();
        if let Some(local_port) = this.net.get_local_port(protocol_type) {
            if external_1_dial_info.port() != local_port {
                let c_external_1_dial_info = external_1_dial_info.clone();
                let c_node_1 = node_1.clone();
                let do_manual_map_fut: SendPinBoxFuture<Option<DetectedDialInfo>> =
                    Box::pin(async move {
                        // Do a validate_dial_info on the external address, but with the same port as the local port of local interface, from a redirected node
                        // This test is to see if a node had manual port forwarding done with the same port number as the local listener
                        let mut external_1_dial_info_with_local_port =
                            c_external_1_dial_info.clone();
                        external_1_dial_info_with_local_port.set_port(local_port);

                        if this
                            .validate_dial_info(
                                c_node_1.clone(),
                                external_1_dial_info_with_local_port.clone(),
                                true,
                            )
                            .await
                        {
                            // Add public dial info with Direct dialinfo class
                            return Some(DetectedDialInfo {
                                dial_info: external_1_dial_info_with_local_port,
                                dial_info_class: DialInfoClass::Direct,
                                network_class: NetworkClass::InboundCapable,
                            });
                        }

                        None
                    });
                ord.push_back(do_manual_map_fut);
            }
        }

        // Full Cone NAT Detection
        ///////////
        let this = self.clone();
        let c_node_1 = node_1.clone();
        let c_external_1_dial_info = external_1_dial_info.clone();
        let do_full_cone_fut: SendPinBoxFuture<Option<DetectedDialInfo>> = Box::pin(async move {
            // Let's see what kind of NAT we have
            // Does a redirected dial info validation from a different address and a random port find us?
            if this
                .validate_dial_info(c_node_1.clone(), c_external_1_dial_info.clone(), true)
                .await
            {
                // Yes, another machine can use the dial info directly, so Full Cone
                // Add public dial info with full cone NAT network class

                return Some(DetectedDialInfo {
                    dial_info: c_external_1_dial_info,
                    dial_info_class: DialInfoClass::FullConeNAT,
                    network_class: NetworkClass::InboundCapable,
                });
            }
            None
        });
        ord.push_back(do_full_cone_fut);

        // Run detections in parallel and take the first one, ordered by preference, that returns a result
        while let Some(res) = ord.next().await {
            if let Some(ddi) = res {
                self.set_detected_public_dial_info(ddi.dial_info, ddi.dial_info_class);
                self.set_detected_network_class(ddi.network_class);
                return Ok(true);
            }
        }

        // We are restricted, determine what kind of restriction
        // Get the external dial info for our use here
        let (node_1, external_1_dial_info, external_1_address, protocol_type, address_type) = {
            let inner = self.inner.lock();
            (
                inner.node_1.as_ref().unwrap().clone(),
                inner.external_1_dial_info.as_ref().unwrap().clone(),
                inner.external_1_address.unwrap(),
                inner.protocol_type.unwrap(),
                inner.address_type.unwrap(),
            )
        };
        // Get our external address from some fast node, that is not node 1, call it node 2
        let (external_2_address, node_2) = match self
            .discover_external_address(protocol_type, address_type, Some(node_1.node_ids()))
            .await
        {
            None => {
                // If we can't get an external address, allow retry
                log_net!(debug "failed to discover external address 2 for {:?}:{:?}, skipping node {:?}", protocol_type, address_type, node_1);
                return Ok(false);
            }
            Some(v) => v,
        };

        log_net!(debug
            "external_2_address: {:?}\nnode_2: {:?}",
            external_2_address, node_2
        );

        // If we have two different external addresses, then this is a symmetric NAT
        if external_2_address != external_1_address {
            // Symmetric NAT is outbound only, no public dial info will work
            self.set_detected_network_class(NetworkClass::OutboundOnly);

            // No more retries
            return Ok(true);
        }

        // If we're going to end up as a restricted NAT of some sort
        // Address is the same, so it's address or port restricted

        // Do a validate_dial_info on the external address from a random port
        if self
            .validate_dial_info(node_2.clone(), external_1_dial_info.clone(), false)
            .await
        {
            // Got a reply from a non-default port, which means we're only address restricted
            self.set_detected_public_dial_info(
                external_1_dial_info,
                DialInfoClass::AddressRestrictedNAT,
            );
        } else {
            // Didn't get a reply from a non-default port, which means we are also port restricted
            self.set_detected_public_dial_info(
                external_1_dial_info,
                DialInfoClass::PortRestrictedNAT,
            );
        }
        self.set_detected_network_class(NetworkClass::InboundCapable);

        // Allow another retry because sometimes trying again will get us Full Cone NAT instead
        Ok(false)
    }
}

impl Network {
    #[instrument(level = "trace", skip(self, context), err)]
    pub async fn update_protocol_dialinfo(
        &self,
        context: &DiscoveryContext,
        protocol_type: ProtocolType,
        address_type: AddressType,
    ) -> EyreResult<()> {
        let mut retry_count = {
            let c = self.config.get();
            c.network.restricted_nat_retries
        };

        // Start doing protocol
        context.protocol_begin(protocol_type, address_type);

        // Loop for restricted NAT retries
        loop {
            log_net!(debug
                "=== update_protocol_dialinfo {:?} {:?} tries_left={} ===",
                address_type,
                protocol_type,
                retry_count
            );
            // Get our external address from some fast node, call it node 1
            if !context.protocol_get_external_address_1().await {
                // If we couldn't get an external address, then we should just try the whole network class detection again later
                return Ok(());
            }

            // If our local interface list contains external_1 then there is no NAT in place
            {
                let res = {
                    let inner = context.inner.lock();
                    inner
                        .intf_addrs
                        .as_ref()
                        .unwrap()
                        .contains(inner.external_1_address.as_ref().unwrap())
                };
                if res {
                    // No NAT
                    context.protocol_process_no_nat().await?;

                    // No more retries
                    break;
                }
            }

            // There is -some NAT-
            if context.protocol_process_nat().await? {
                // We either got dial info or a network class without one
                break;
            }

            // If we tried everything, break anyway after N attempts
            if retry_count == 0 {
                break;
            }
            retry_count -= 1;
        }

        Ok(())
    }

    #[instrument(level = "trace", skip(self), err)]
    pub async fn do_public_dial_info_check(
        &self,
        stop_token: StopToken,
        _l: u64,
        _t: u64,
    ) -> EyreResult<()> {
        let routing_table = self.routing_table();

        // Figure out if we can optimize TCP/WS checking since they are often on the same port
        let (protocol_config, existing_network_class, tcp_same_port) = {
            let inner = self.inner.lock();
            let protocol_config = inner.protocol_config;
            let existing_network_class =
                routing_table.get_network_class(RoutingDomain::PublicInternet);
            let tcp_same_port = if protocol_config.inbound.contains(ProtocolType::TCP)
                && protocol_config.inbound.contains(ProtocolType::WS)
            {
                inner.tcp_port == inner.ws_port
            } else {
                false
            };
            (protocol_config, existing_network_class, tcp_same_port)
        };

        // Process all protocol and address combinations
        let mut futures = FuturesUnordered::new();
        // Do UDPv4+v6 at the same time as everything else
        if protocol_config.inbound.contains(ProtocolType::UDP) {
            // UDPv4
            if protocol_config.family_global.contains(AddressType::IPV4) {
                futures.push(
                    async {
                        let udpv4_context =
                            DiscoveryContext::new(self.routing_table(), self.clone());
                        if let Err(e) = self
                            .update_protocol_dialinfo(
                                &udpv4_context,
                                ProtocolType::UDP,
                                AddressType::IPV4,
                            )
                            .await
                        {
                            log_net!(debug "Failed UDPv4 dialinfo discovery: {}", e);
                            return None;
                        }
                        Some(vec![udpv4_context])
                    }
                    .instrument(trace_span!("do_public_dial_info_check UDPv4"))
                    .boxed(),
                );
            }

            // UDPv6
            if protocol_config.family_global.contains(AddressType::IPV6) {
                futures.push(
                    async {
                        let udpv6_context =
                            DiscoveryContext::new(self.routing_table(), self.clone());
                        if let Err(e) = self
                            .update_protocol_dialinfo(
                                &udpv6_context,
                                ProtocolType::UDP,
                                AddressType::IPV6,
                            )
                            .await
                        {
                            log_net!(debug "Failed UDPv6 dialinfo discovery: {}", e);
                            return None;
                        }
                        Some(vec![udpv6_context])
                    }
                    .instrument(trace_span!("do_public_dial_info_check UDPv6"))
                    .boxed(),
                );
            }
        }

        // Do TCPv4. Possibly do WSv4 if it is on a different port
        if protocol_config.family_global.contains(AddressType::IPV4) {
            if protocol_config.inbound.contains(ProtocolType::TCP) {
                futures.push(
                    async {
                        // TCPv4
                        let tcpv4_context =
                            DiscoveryContext::new(self.routing_table(), self.clone());
                        if let Err(e) = self
                            .update_protocol_dialinfo(
                                &tcpv4_context,
                                ProtocolType::TCP,
                                AddressType::IPV4,
                            )
                            .await
                        {
                            log_net!(debug "Failed TCPv4 dialinfo discovery: {}", e);
                            return None;
                        }
                        Some(vec![tcpv4_context])
                    }
                    .instrument(trace_span!("do_public_dial_info_check TCPv4"))
                    .boxed(),
                );
            }

            if protocol_config.inbound.contains(ProtocolType::WS) && !tcp_same_port {
                futures.push(
                    async {
                        // WSv4
                        let wsv4_context =
                            DiscoveryContext::new(self.routing_table(), self.clone());
                        if let Err(e) = self
                            .update_protocol_dialinfo(
                                &wsv4_context,
                                ProtocolType::WS,
                                AddressType::IPV4,
                            )
                            .await
                        {
                            log_net!(debug "Failed WSv4 dialinfo discovery: {}", e);
                            return None;
                        }
                        Some(vec![wsv4_context])
                    }
                    .instrument(trace_span!("do_public_dial_info_check WSv4"))
                    .boxed(),
                );
            }
        }

        // Do TCPv6. Possibly do WSv6 if it is on a different port
        if protocol_config.family_global.contains(AddressType::IPV6) {
            if protocol_config.inbound.contains(ProtocolType::TCP) {
                futures.push(
                    async {
                        // TCPv6
                        let tcpv6_context =
                            DiscoveryContext::new(self.routing_table(), self.clone());
                        if let Err(e) = self
                            .update_protocol_dialinfo(
                                &tcpv6_context,
                                ProtocolType::TCP,
                                AddressType::IPV6,
                            )
                            .await
                        {
                            log_net!(debug "Failed TCPv6 dialinfo discovery: {}", e);
                            return None;
                        }
                        Some(vec![tcpv6_context])
                    }
                    .instrument(trace_span!("do_public_dial_info_check TCPv6"))
                    .boxed(),
                );
            }

            // WSv6
            if protocol_config.inbound.contains(ProtocolType::WS) && !tcp_same_port {
                futures.push(
                    async {
                        let wsv6_context =
                            DiscoveryContext::new(self.routing_table(), self.clone());
                        if let Err(e) = self
                            .update_protocol_dialinfo(
                                &wsv6_context,
                                ProtocolType::WS,
                                AddressType::IPV6,
                            )
                            .await
                        {
                            log_net!(debug "Failed WSv6 dialinfo discovery: {}", e);
                            return None;
                        }
                        Some(vec![wsv6_context])
                    }
                    .instrument(trace_span!("do_public_dial_info_check WSv6"))
                    .boxed(),
                );
            }
        }

        // Wait for all discovery futures to complete and collect contexts
        let mut contexts = Vec::<DiscoveryContext>::new();
        let mut new_network_class = Option::<NetworkClass>::None;
        loop {
            match futures.next().timeout_at(stop_token.clone()).await {
                Ok(Some(ctxvec)) => {
                    if let Some(ctxvec) = ctxvec {
                        for ctx in ctxvec {
                            if let Some(nc) = ctx.inner.lock().detected_network_class {
                                if let Some(last_nc) = new_network_class {
                                    if nc < last_nc {
                                        new_network_class = Some(nc);
                                    }
                                } else {
                                    new_network_class = Some(nc);
                                }
                            }

                            contexts.push(ctx);
                        }
                    }
                }
                Ok(None) => {
                    // Normal completion
                    break;
                }
                Err(_) => {
                    // Stop token, exit early without error propagation
                    return Ok(());
                }
            }
        }

        // If a network class could be determined
        // see about updating our public dial info
        let mut changed = false;
        let mut editor = routing_table.edit_routing_domain(RoutingDomain::PublicInternet);
        if new_network_class.is_some() {
            // Get existing public dial info
            let existing_public_dial_info: HashSet<DialInfoDetail> = routing_table
                .all_filtered_dial_info_details(
                    RoutingDomain::PublicInternet.into(),
                    &DialInfoFilter::all(),
                )
                .into_iter()
                .collect();

            // Get new public dial info and ensure it is valid
            let mut new_public_dial_info: HashSet<DialInfoDetail> = HashSet::new();
            for ctx in contexts {
                let inner = ctx.inner.lock();
                if let Some(pdi) = &inner.detected_public_dial_info {
                    if routing_table
                        .ensure_dial_info_is_valid(RoutingDomain::PublicInternet, &pdi.dial_info)
                    {
                        new_public_dial_info.insert(DialInfoDetail {
                            class: pdi.class,
                            dial_info: pdi.dial_info.clone(),
                        });
                    }

                    // duplicate for same port
                    if tcp_same_port && pdi.dial_info.protocol_type() == ProtocolType::TCP {
                        let ws_dial_info =
                            ctx.make_dial_info(pdi.dial_info.socket_address(), ProtocolType::WS);
                        if routing_table
                            .ensure_dial_info_is_valid(RoutingDomain::PublicInternet, &ws_dial_info)
                        {
                            new_public_dial_info.insert(DialInfoDetail {
                                class: pdi.class,
                                dial_info: ws_dial_info,
                            });
                        }
                    }
                }
            }

            // Is the public dial info different?
            if existing_public_dial_info != new_public_dial_info {
                // If so, clear existing public dial info and re-register the new public dial info
                editor.clear_dial_info_details();
                for did in new_public_dial_info {
                    if let Err(e) = editor.register_dial_info(did.dial_info, did.class) {
                        log_net!(error "Failed to register detected public dial info: {}", e);
                    }
                }
                changed = true;
            }

            // Is the network class different?
            if existing_network_class != new_network_class {
                editor.set_network_class(new_network_class);
                changed = true;
                log_net!(debug "PublicInternet network class changed to {:?}", new_network_class);
            }
        } else if existing_network_class.is_some() {
            // Network class could not be determined
            editor.clear_dial_info_details();
            editor.set_network_class(None);
            editor.clear_relay_node();
            changed = true;
            log_net!(debug "PublicInternet network class cleared");
        }

        // Punish nodes that told us our public address had changed when it didn't
        if !changed {
            if let Some(punish) = self.inner.lock().public_dial_info_check_punishment.take() {
                punish();
            }
        } else {
            // Commit updates
            editor.commit();
        }

        Ok(())
    }
    #[instrument(level = "trace", skip(self), err)]
    pub async fn update_network_class_task_routine(
        self,
        stop_token: StopToken,
        l: u64,
        t: u64,
    ) -> EyreResult<()> {
        // Do the public dial info check
        let out = self.do_public_dial_info_check(stop_token, l, t).await;

        // Done with public dial info check
        self.inner.lock().needs_public_dial_info_check = false;

        out
    }
}
