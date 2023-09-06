/// Context detection of public dial info for a single protocol and address type
/// Also performs UPNP/IGD mapping if enabled and possible
use super::*;
use futures_util::stream::FuturesUnordered;

const PORT_MAP_VALIDATE_TRY_COUNT: usize = 3;
const PORT_MAP_VALIDATE_DELAY_MS: u32 = 500;
const PORT_MAP_TRY_COUNT: usize = 3;

// Detection result of dial info detection futures
#[derive(Clone, Debug)]
pub enum DetectedDialInfo {
    SymmetricNAT,
    Detected(DialInfoDetail),
}

// Result of checking external address
#[derive(Clone, Debug)]
struct ExternalInfo {
    dial_info: DialInfo,
    address: SocketAddress,
    node: NodeRef,
}

struct DiscoveryContextInner {
    // first node contacted
    external_1: Option<ExternalInfo>,
    // second node contacted
    external_2: Option<ExternalInfo>,
}

struct DiscoveryContextUnlockedInner {
    routing_table: RoutingTable,
    net: Network,
    // per-protocol
    intf_addrs: Vec<SocketAddress>,
    protocol_type: ProtocolType,
    address_type: AddressType,
}

#[derive(Clone)]
pub struct DiscoveryContext {
    unlocked_inner: Arc<DiscoveryContextUnlockedInner>,
    inner: Arc<Mutex<DiscoveryContextInner>>,
}

impl DiscoveryContext {
    pub fn new(
        routing_table: RoutingTable,
        net: Network,
        protocol_type: ProtocolType,
        address_type: AddressType,
    ) -> Self {
        let intf_addrs =
            Self::get_local_addresses(routing_table.clone(), protocol_type, address_type);

        Self {
            unlocked_inner: Arc::new(DiscoveryContextUnlockedInner {
                routing_table,
                net,
                intf_addrs,
                protocol_type,
                address_type,
            }),
            inner: Arc::new(Mutex::new(DiscoveryContextInner {
                external_1: None,
                external_2: None,
            })),
        }
    }

    ///////
    // Utilities

    // This pulls the already-detected local interface dial info from the routing table
    #[instrument(level = "trace", skip(routing_table), ret)]
    fn get_local_addresses(
        routing_table: RoutingTable,
        protocol_type: ProtocolType,
        address_type: AddressType,
    ) -> Vec<SocketAddress> {
        let filter = DialInfoFilter::all()
            .with_protocol_type(protocol_type)
            .with_address_type(address_type);
        routing_table
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

    // Ask for a public address check from a particular noderef
    // This is done over the normal port using RPC
    #[instrument(level = "trace", skip(self), ret)]
    async fn request_public_address(&self, node_ref: NodeRef) -> Option<SocketAddress> {
        let rpc = self.unlocked_inner.routing_table.rpc_processor();

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
    async fn discover_external_addresses(&self) -> bool {
        let node_count = {
            let config = self.unlocked_inner.routing_table.network_manager().config();
            let c = config.get();
            c.network.dht.max_find_node_count as usize
        };
        let routing_domain = RoutingDomain::PublicInternet;
        let protocol_type = self.unlocked_inner.protocol_type;
        let address_type = self.unlocked_inner.address_type;

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

        let filters = VecDeque::from([
            inbound_dial_info_entry_filter,
            disallow_relays_filter,
            will_validate_dial_info_filter,
        ]);

        // Find public nodes matching this filter
        let nodes = self
            .unlocked_inner
            .routing_table
            .find_fast_public_nodes_filtered(node_count, filters);
        if nodes.is_empty() {
            log_net!(debug
                "no external address detection peers of type {:?}:{:?}",
                protocol_type,
                address_type
            );
            return false;
        }

        // For each peer, ask them for our public address, filtering on desired dial info
        let mut unord = FuturesUnordered::new();

        let get_public_address_func = |node: NodeRef| {
            let this = self.clone();
            let node = node.filtered_clone(
                NodeRefFilter::new()
                    .with_routing_domain(routing_domain)
                    .with_dial_info_filter(dial_info_filter.clone()),
            );
            async move {
                if let Some(address) = this.request_public_address(node.clone()).await {
                    let dial_info = this
                        .unlocked_inner
                        .net
                        .make_dial_info(address, this.unlocked_inner.protocol_type);
                    return Some(ExternalInfo {
                        dial_info,
                        address,
                        node,
                    });
                }
                None
            }
        };

        let mut external_address_infos = Vec::new();

        for ni in 0..nodes.len() - 1 {
            let node = nodes[ni].clone();

            let gpa_future = get_public_address_func(node);
            unord.push(gpa_future);

            // Always process two at a time so we get both addresses in parallel if possible
            if unord.len() == 2 {
                // Process one
                if let Some(Some(ei)) = unord.next().await {
                    external_address_infos.push(ei);
                    if external_address_infos.len() == 2 {
                        break;
                    }
                }
            }
        }
        // Finish whatever is left if we need to
        if external_address_infos.len() < 2 {
            while let Some(res) = unord.next().await {
                if let Some(ei) = res {
                    external_address_infos.push(ei);
                    if external_address_infos.len() == 2 {
                        break;
                    }
                }
            }
        }
        if external_address_infos.len() < 2 {
            log_net!(debug "not enough peers responded with an external address");
            return false;
        }

        {
            let mut inner = self.inner.lock();
            inner.external_1 = Some(external_address_infos[0].clone());
            log_net!(debug "external_1: {:?}", inner.external_1);
            inner.external_2 = Some(external_address_infos[1].clone());
            log_net!(debug "external_2: {:?}", inner.external_2);
        }

        true
    }

    #[instrument(level = "trace", skip(self), ret)]
    async fn validate_dial_info(
        &self,
        node_ref: NodeRef,
        dial_info: DialInfo,
        redirect: bool,
    ) -> bool {
        let rpc_processor = self.unlocked_inner.routing_table.rpc_processor();

        // asking for node validation doesn't have to use the dial info filter of the dial info we are validating
        let mut node_ref = node_ref.clone();
        node_ref.set_filter(None);

        // ask the node to send us a dial info validation receipt
        let out = rpc_processor
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
        let protocol_type = self.unlocked_inner.protocol_type;
        let low_level_protocol_type = protocol_type.low_level_protocol_type();
        let address_type = self.unlocked_inner.address_type;
        let local_port = self
            .unlocked_inner
            .net
            .get_local_port(protocol_type)
            .unwrap();
        let external_1 = self.inner.lock().external_1.as_ref().unwrap().clone();

        let igd_manager = self.unlocked_inner.net.unlocked_inner.igd_manager.clone();
        let mut tries = 0;
        loop {
            tries += 1;

            // Attempt a port mapping. If this doesn't succeed, it's not going to
            let Some(mapped_external_address) = igd_manager
                .map_any_port(low_level_protocol_type, address_type, local_port, Some(external_1.address.to_ip_addr()))
                .await else
            {
                return None;
            };

            // Make dial info from the port mapping
            let external_mapped_dial_info = self.unlocked_inner.net.make_dial_info(
                SocketAddress::from_socket_addr(mapped_external_address),
                protocol_type,
            );

            // Attempt to validate the port mapping
            let mut validate_tries = 0;
            loop {
                validate_tries += 1;

                // Ensure people can reach us. If we're firewalled off, this is useless
                if self
                    .validate_dial_info(
                        external_1.node.clone(),
                        external_mapped_dial_info.clone(),
                        false,
                    )
                    .await
                {
                    return Some(external_mapped_dial_info);
                }

                if validate_tries == PORT_MAP_VALIDATE_TRY_COUNT {
                    log_net!(debug "UPNP port mapping succeeded but port {}/{} is still unreachable.\nretrying\n",
                    local_port, match low_level_protocol_type {
                        LowLevelProtocolType::UDP => "udp",
                        LowLevelProtocolType::TCP => "tcp",
                    });
                    sleep(PORT_MAP_VALIDATE_DELAY_MS).await
                } else {
                    break;
                }
            }

            // Release the mapping if we're still unreachable
            let _ = igd_manager
                .unmap_port(
                    low_level_protocol_type,
                    address_type,
                    external_1.address.port(),
                )
                .await;

            if tries == PORT_MAP_TRY_COUNT {
                warn!("UPNP port mapping succeeded but port {}/{} is still unreachable.\nYou may need to add a local firewall allowed port on this machine.\n",
                    local_port, match low_level_protocol_type {
                        LowLevelProtocolType::UDP => "udp",
                        LowLevelProtocolType::TCP => "tcp",
                    }
                );
                break;
            }
        }
        None
    }

    ///////
    // Per-protocol discovery routines

    // If we know we are not behind NAT, check our firewall status
    #[instrument(level = "trace", skip(self), ret)]
    async fn protocol_process_no_nat(
        &self,
        unord: &mut FuturesUnordered<SendPinBoxFuture<Option<DetectedDialInfo>>>,
    ) {
        let external_1 = self.inner.lock().external_1.as_ref().unwrap().clone();

        let this = self.clone();
        let do_no_nat_fut: SendPinBoxFuture<Option<DetectedDialInfo>> = Box::pin(async move {
            // Do a validate_dial_info on the external address from a redirected node
            if this
                .validate_dial_info(external_1.node.clone(), external_1.dial_info.clone(), true)
                .await
            {
                // Add public dial info with Direct dialinfo class
                Some(DetectedDialInfo::Detected(DialInfoDetail {
                    dial_info: external_1.dial_info.clone(),
                    class: DialInfoClass::Direct,
                }))
            } else {
                // Add public dial info with Blocked dialinfo class
                Some(DetectedDialInfo::Detected(DialInfoDetail {
                    dial_info: external_1.dial_info.clone(),
                    class: DialInfoClass::Blocked,
                }))
            }
        });
        unord.push(do_no_nat_fut);
    }

    // If we know we are behind NAT check what kind
    #[instrument(level = "trace", skip(self), ret)]
    async fn protocol_process_nat(
        &self,
        unord: &mut FuturesUnordered<SendPinBoxFuture<Option<DetectedDialInfo>>>,
    ) {
        // Get the external dial info for our use here
        let (external_1, external_2) = {
            let inner = self.inner.lock();
            (
                inner.external_1.as_ref().unwrap().clone(),
                inner.external_2.as_ref().unwrap().clone(),
            )
        };

        // If we have two different external addresses, then this is a symmetric NAT
        if external_2.address != external_1.address {
            let do_symmetric_nat_fut: SendPinBoxFuture<Option<DetectedDialInfo>> =
                Box::pin(async move { Some(DetectedDialInfo::SymmetricNAT) });
            unord.push(do_symmetric_nat_fut);
            return;
        }

        // Manual Mapping Detection
        ///////////
        let this = self.clone();
        if let Some(local_port) = self
            .unlocked_inner
            .net
            .get_local_port(self.unlocked_inner.protocol_type)
        {
            if external_1.dial_info.port() != local_port {
                let c_external_1 = external_1.clone();
                let do_manual_map_fut: SendPinBoxFuture<Option<DetectedDialInfo>> =
                    Box::pin(async move {
                        // Do a validate_dial_info on the external address, but with the same port as the local port of local interface, from a redirected node
                        // This test is to see if a node had manual port forwarding done with the same port number as the local listener
                        let mut external_1_dial_info_with_local_port =
                            c_external_1.dial_info.clone();
                        external_1_dial_info_with_local_port.set_port(local_port);

                        if this
                            .validate_dial_info(
                                c_external_1.node.clone(),
                                external_1_dial_info_with_local_port.clone(),
                                true,
                            )
                            .await
                        {
                            // Add public dial info with Direct dialinfo class
                            return Some(DetectedDialInfo::Detected(DialInfoDetail {
                                dial_info: external_1_dial_info_with_local_port,
                                class: DialInfoClass::Direct,
                            }));
                        }

                        None
                    });
                unord.push(do_manual_map_fut);
            }
        }

        // NAT Detection
        ///////////

        // Full Cone NAT Detection
        ///////////
        let this = self.clone();
        let do_nat_detect_fut: SendPinBoxFuture<Option<DetectedDialInfo>> = Box::pin(async move {
            let mut retry_count = {
                let c = this.unlocked_inner.net.config.get();
                c.network.restricted_nat_retries
            };

            // Loop for restricted NAT retries
            loop {
                let mut ord = FuturesOrdered::new();

                let c_this = this.clone();
                let c_external_1 = external_1.clone();
                let do_full_cone_fut: SendPinBoxFuture<Option<DetectedDialInfo>> =
                    Box::pin(async move {
                        // Let's see what kind of NAT we have
                        // Does a redirected dial info validation from a different address and a random port find us?
                        if c_this
                            .validate_dial_info(
                                c_external_1.node.clone(),
                                c_external_1.dial_info.clone(),
                                true,
                            )
                            .await
                        {
                            // Yes, another machine can use the dial info directly, so Full Cone
                            // Add public dial info with full cone NAT network class

                            return Some(DetectedDialInfo::Detected(DialInfoDetail {
                                dial_info: c_external_1.dial_info,
                                class: DialInfoClass::FullConeNAT,
                            }));
                        }
                        None
                    });
                ord.push_back(do_full_cone_fut);

                let c_this = this.clone();
                let c_external_1 = external_1.clone();
                let c_external_2 = external_2.clone();
                let do_restricted_cone_fut: SendPinBoxFuture<Option<DetectedDialInfo>> =
                    Box::pin(async move {
                        // We are restricted, determine what kind of restriction

                        // If we're going to end up as a restricted NAT of some sort
                        // Address is the same, so it's address or port restricted

                        // Do a validate_dial_info on the external address from a random port
                        if c_this
                            .validate_dial_info(
                                c_external_2.node.clone(),
                                c_external_1.dial_info.clone(),
                                false,
                            )
                            .await
                        {
                            // Got a reply from a non-default port, which means we're only address restricted
                            return Some(DetectedDialInfo::Detected(DialInfoDetail {
                                dial_info: c_external_1.dial_info.clone(),
                                class: DialInfoClass::AddressRestrictedNAT,
                            }));
                        }
                        // Didn't get a reply from a non-default port, which means we are also port restricted
                        Some(DetectedDialInfo::Detected(DialInfoDetail {
                            dial_info: c_external_1.dial_info.clone(),
                            class: DialInfoClass::PortRestrictedNAT,
                        }))
                    });
                ord.push_back(do_restricted_cone_fut);

                // Return the first result we get
                let mut some_ddi = None;
                while let Some(res) = ord.next().await {
                    if let Some(ddi) = res {
                        some_ddi = Some(ddi);
                        break;
                    }
                }

                if let Some(ddi) = some_ddi {
                    if let DetectedDialInfo::Detected(did) = &ddi {
                        // If we got something better than restricted NAT or we're done retrying
                        if did.class < DialInfoClass::AddressRestrictedNAT || retry_count == 0 {
                            return Some(ddi);
                        }
                    }
                }
                if retry_count == 0 {
                    break;
                }
                retry_count -= 1;
            }

            None
        });
        unord.push(do_nat_detect_fut);
    }

    /// Add discovery futures to an unordered set that may detect dialinfo when they complete
    pub async fn discover(
        &self,
        unord: &mut FuturesUnordered<SendPinBoxFuture<Option<DetectedDialInfo>>>,
    ) {
        let enable_upnp = {
            let c = self.unlocked_inner.net.config.get();
            c.network.upnp
        };

        // Do this right away because it's fast and every detection is going to need it
        // Get our external addresses from two fast nodes
        if !self.discover_external_addresses().await {
            // If we couldn't get an external address, then we should just try the whole network class detection again later
            return;
        }

        // UPNP Automatic Mapping
        ///////////
        if enable_upnp {
            let this = self.clone();
            let do_mapped_fut: SendPinBoxFuture<Option<DetectedDialInfo>> = Box::pin(async move {
                // Attempt a port mapping via all available and enabled mechanisms
                // Try this before the direct mapping in the event that we are restarting
                // and may not have recorded a mapping created the last time
                if let Some(external_mapped_dial_info) = this.try_upnp_port_mapping().await {
                    // Got a port mapping, let's use it
                    return Some(DetectedDialInfo::Detected(DialInfoDetail {
                        dial_info: external_mapped_dial_info.clone(),
                        class: DialInfoClass::Mapped,
                    }));
                }
                None
            });
            unord.push(do_mapped_fut);
        }

        // NAT Detection
        ///////////

        // If our local interface list contains external_1 then there is no NAT in place
        let external_1 = self.inner.lock().external_1.as_ref().unwrap().clone();

        if self.unlocked_inner.intf_addrs.contains(&external_1.address) {
            self.protocol_process_no_nat(unord).await;
        } else {
            self.protocol_process_nat(unord).await;
        }
    }
}
