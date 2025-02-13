/// Context detection of public dial info for a single protocol and address type
/// Also performs UPNP/IGD mapping if enabled and possible
use super::*;
use futures_util::stream::FuturesUnordered;
use igd_manager::{IGDAddressType, IGDProtocolType};

const PORT_MAP_VALIDATE_TRY_COUNT: usize = 3;
const PORT_MAP_VALIDATE_DELAY_MS: u32 = 500;
const PORT_MAP_TRY_COUNT: usize = 3;
const EXTERNAL_INFO_VALIDATIONS: usize = 5;

// Detection result of dial info detection futures
#[derive(Clone, Debug)]
pub enum DetectedDialInfo {
    SymmetricNAT,
    Detected(DialInfoDetail),
}

// Detection result of external address
#[derive(Clone, Debug)]
pub struct DetectionResult {
    pub config: DiscoveryContextConfig,
    pub ddi: DetectedDialInfo,
    pub external_address_types: AddressTypeSet,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DiscoveryContextConfig {
    pub protocol_type: ProtocolType,
    pub address_type: AddressType,
    pub port: u16,
}

// Result of checking external address
#[derive(Clone, Debug)]
struct ExternalInfo {
    dial_info: DialInfo,
    address: SocketAddress,
    node: NodeRef,
}

struct DiscoveryContextInner {
    external_info: Vec<ExternalInfo>,
}

pub(super) struct DiscoveryContextUnlockedInner {
    config: DiscoveryContextConfig,

    // per-protocol
    intf_addrs: Vec<SocketAddress>,
}

#[derive(Clone)]
pub(super) struct DiscoveryContext {
    registry: VeilidComponentRegistry,
    unlocked_inner: Arc<DiscoveryContextUnlockedInner>,
    inner: Arc<Mutex<DiscoveryContextInner>>,
}

impl_veilid_component_registry_accessor!(DiscoveryContext);

impl core::ops::Deref for DiscoveryContext {
    type Target = DiscoveryContextUnlockedInner;

    fn deref(&self) -> &Self::Target {
        &self.unlocked_inner
    }
}

impl DiscoveryContext {
    pub fn new(registry: VeilidComponentRegistry, config: DiscoveryContextConfig) -> Self {
        let routing_table = registry.routing_table();
        let intf_addrs =
            Self::get_local_addresses(&routing_table, config.protocol_type, config.address_type);

        Self {
            registry,
            unlocked_inner: Arc::new(DiscoveryContextUnlockedInner { config, intf_addrs }),
            inner: Arc::new(Mutex::new(DiscoveryContextInner {
                external_info: Vec::new(),
            })),
        }
    }

    ///////
    // Utilities

    // This pulls the already-detected local interface dial info from the routing table
    #[instrument(level = "trace", skip(routing_table), ret)]
    fn get_local_addresses(
        routing_table: &RoutingTable,
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
    async fn request_public_address(&self, node_ref: FilteredNodeRef) -> Option<SocketAddress> {
        let rpc = self.rpc_processor();

        // Ensure a fresh connection is made so it comes from our public address
        // This will only clear the dialinfo filtered flows, as this is a FilteredNodeRef
        // filtered down to the protocol/address type we are checking the public address for
        node_ref.clear_last_flows();

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

        log_network_result!(
            debug "request_public_address {:?}: Value({:?})",
            node_ref,
            res.answer
        );
        res.answer.opt_sender_info.map(|si| si.socket_address)
    }

    // find fast peers with a particular address type, and ask them to tell us what our external address is
    // This is done over the normal port using RPC
    #[instrument(level = "trace", skip(self), ret)]
    async fn discover_external_addresses(&self) -> bool {
        let node_count = self
            .config()
            .with(|c| c.network.dht.max_find_node_count as usize);
        let routing_domain = RoutingDomain::PublicInternet;

        let protocol_type = self.config.protocol_type;
        let address_type = self.config.address_type;
        let port = self.config.port;

        // Build an filter that matches our protocol and address type
        // and excludes relayed nodes so we can get an accurate external address
        let dial_info_filter = DialInfoFilter::all()
            .with_protocol_type(protocol_type)
            .with_address_type(address_type);
        let inbound_dial_info_entry_filter =
            RoutingTable::make_inbound_dial_info_entry_filter(routing_domain, dial_info_filter);
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
        let nodes = self.routing_table().find_fast_non_local_nodes_filtered(
            routing_domain,
            node_count,
            filters,
        );
        if nodes.is_empty() {
            log_net!(debug
                "no external address detection peers of type {:?}:{:?}",
                protocol_type,
                address_type
            );
            return false;
        }

        // For each peer, ask them for our public address, filtering on desired dial info

        let get_public_address_func = |node: NodeRef| {
            let this = self.clone();
            let node = node.custom_filtered(
                NodeRefFilter::new()
                    .with_routing_domain(routing_domain)
                    .with_dial_info_filter(dial_info_filter),
            );
            async move {
                if let Some(address) = this.request_public_address(node.clone()).await {
                    let dial_info = this
                        .network_manager()
                        .net()
                        .make_dial_info(address, protocol_type);
                    return Some(ExternalInfo {
                        dial_info,
                        address,
                        node: node.unfiltered(),
                    });
                }
                None
            }
        };

        let mut external_address_infos = Vec::new();
        let mut unord = FuturesUnordered::new();
        for node in nodes.iter().cloned() {
            let gpa_future = get_public_address_func(node);
            unord.push(gpa_future);

            // Always process N at a time so we get all addresses in parallel if possible
            if unord.len() == EXTERNAL_INFO_VALIDATIONS {
                // Process one
                if let Some(Some(ei)) = unord.next().in_current_span().await {
                    external_address_infos.push(ei);
                    if external_address_infos.len() == EXTERNAL_INFO_VALIDATIONS {
                        break;
                    }
                }
            }
        }
        // Finish whatever is left if we need to
        if external_address_infos.len() < EXTERNAL_INFO_VALIDATIONS {
            while let Some(res) = unord.next().in_current_span().await {
                if let Some(ei) = res {
                    external_address_infos.push(ei);
                    if external_address_infos.len() == EXTERNAL_INFO_VALIDATIONS {
                        break;
                    }
                }
            }
        }
        if external_address_infos.len() < EXTERNAL_INFO_VALIDATIONS {
            log_net!(debug "not enough peers responded with an external address for type {:?}:{:?}",
                protocol_type,
                address_type);
            return false;
        }

        // Try to make preferential port come first
        external_address_infos.sort_by(|a, b| {
            let acmp = a.address.ip_addr().cmp(&b.address.ip_addr());
            if acmp != cmp::Ordering::Equal {
                return acmp;
            }
            if a.address.port() == b.address.port() {
                return cmp::Ordering::Equal;
            }
            if a.address.port() == port {
                return cmp::Ordering::Less;
            }
            if b.address.port() == port {
                return cmp::Ordering::Greater;
            }
            a.address.port().cmp(&b.address.port())
        });

        {
            let mut inner = self.inner.lock();
            inner.external_info = external_address_infos;
            log_net!(debug "External Addresses ({:?}:{:?}):\n{}",
                protocol_type,
                address_type,
                inner.external_info.iter().map(|x| format!("    {} <- {}",x.address, x.node)).collect::<Vec<_>>().join("\n"));
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
        // ask the node to send us a dial info validation receipt
        // no need to clear_last_flows here, because the dial_info is always returned via the
        // send_out_of_band_receipt mechanism, which will always create a new flow
        // and the outgoing rpc call is safely able to use existing flows
        match self
            .rpc_processor()
            .rpc_call_validate_dial_info(node_ref.clone(), dial_info, redirect)
            .await
        {
            Err(e) => {
                log_net!("failed to send validate_dial_info to {:?}: {}", node_ref, e);
                false
            }
            Ok(v) => v,
        }
    }

    #[instrument(level = "trace", skip(self), ret)]
    async fn try_upnp_port_mapping(&self) -> Option<DialInfo> {
        let protocol_type = self.config.protocol_type;
        let address_type = self.config.address_type;
        let local_port = self.config.port;

        let igd_protocol_type = match protocol_type.low_level_protocol_type() {
            LowLevelProtocolType::UDP => IGDProtocolType::UDP,
            LowLevelProtocolType::TCP => IGDProtocolType::TCP,
        };
        let igd_address_type = match address_type {
            AddressType::IPV6 => IGDAddressType::IPV6,
            AddressType::IPV4 => IGDAddressType::IPV4,
        };

        let external_1 = self.inner.lock().external_info.first().unwrap().clone();

        let igd_manager = self.network_manager().net().igd_manager.clone();
        let mut tries = 0;
        loop {
            tries += 1;

            // Attempt a port mapping. If this doesn't succeed, it's not going to
            let mapped_external_address = igd_manager
                .map_any_port(
                    igd_protocol_type,
                    igd_address_type,
                    local_port,
                    Some(external_1.address.ip_addr()),
                )
                .await?;

            // Make dial info from the port mapping
            let external_mapped_dial_info = self.network_manager().net().make_dial_info(
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

                if validate_tries != PORT_MAP_VALIDATE_TRY_COUNT {
                    log_net!(debug "UPNP port mapping succeeded but port {}/{} is still unreachable.\nretrying\n",
                    local_port, igd_protocol_type);
                    sleep(PORT_MAP_VALIDATE_DELAY_MS).await
                } else {
                    break;
                }
            }

            // Release the mapping if we're still unreachable
            let _ = igd_manager
                .unmap_port(
                    igd_protocol_type,
                    igd_address_type,
                    external_1.address.port(),
                )
                .await;

            if tries == PORT_MAP_TRY_COUNT {
                warn!("UPNP port mapping succeeded but port {}/{} is still unreachable.\nYou may need to add a local firewall allowed port on this machine.\n",
                    local_port, igd_protocol_type
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
        unord: &mut FuturesUnordered<SendPinBoxFuture<Option<DetectionResult>>>,
    ) {
        let external_1 = self.inner.lock().external_info.first().cloned().unwrap();

        let this = self.clone();
        let do_no_nat_fut: SendPinBoxFuture<Option<DetectionResult>> = Box::pin(async move {
            // Do a validate_dial_info on the external address from a redirected node
            if this
                .validate_dial_info(external_1.node.clone(), external_1.dial_info.clone(), true)
                .await
            {
                // Add public dial info with Direct dialinfo class
                Some(DetectionResult {
                    config: this.config,
                    ddi: DetectedDialInfo::Detected(DialInfoDetail {
                        dial_info: external_1.dial_info.clone(),
                        class: DialInfoClass::Direct,
                    }),
                    external_address_types: AddressTypeSet::only(external_1.address.address_type()),
                })
            } else {
                // Add public dial info with Blocked dialinfo class
                Some(DetectionResult {
                    config: this.config,
                    ddi: DetectedDialInfo::Detected(DialInfoDetail {
                        dial_info: external_1.dial_info.clone(),
                        class: DialInfoClass::Blocked,
                    }),
                    external_address_types: AddressTypeSet::only(external_1.address.address_type()),
                })
            }
        });
        unord.push(do_no_nat_fut);
    }

    // If we know we are behind NAT check what kind
    #[instrument(level = "trace", skip(self), ret)]
    async fn protocol_process_nat(
        &self,
        unord: &mut FuturesUnordered<SendPinBoxFuture<Option<DetectionResult>>>,
    ) {
        let external_info = {
            let inner = self.inner.lock();
            inner.external_info.clone()
        };
        let local_port = self.config.port;

        // Get the external dial info histogram for our use here
        let mut external_info_addr_port_hist = HashMap::<SocketAddress, usize>::new();
        let mut external_info_addr_hist = HashMap::<Address, usize>::new();
        for ei in &external_info {
            external_info_addr_port_hist
                .entry(ei.address)
                .and_modify(|n| *n += 1)
                .or_insert(1);
            external_info_addr_hist
                .entry(ei.address.address())
                .and_modify(|n| *n += 1)
                .or_insert(1);
        }

        // If we have two different external addresses, then this is a symmetric NAT
        // If just the port differs, and one is the preferential port we still accept
        // this as an inbound capable dialinfo for holepunch
        let different_addresses = external_info_addr_hist.len() > 1;
        let mut best_external_info = None;
        let mut local_port_matching_external_info = None;
        let mut external_address_types = AddressTypeSet::new();

        // Get the most popular external port from our sampling
        // There will always be a best external info
        let mut best_ei_address = None;
        let mut best_ei_cnt = 0;
        for eiph in &external_info_addr_port_hist {
            if *eiph.1 > best_ei_cnt {
                best_ei_address = Some(*eiph.0);
                best_ei_cnt = *eiph.1;
            }
        }
        // In preference order, pick out the best external address and if we have one the one that
        // matches our local port number (may be the same)
        for ei in &external_info {
            if ei.address.port() == local_port && local_port_matching_external_info.is_none() {
                local_port_matching_external_info = Some(ei.clone());
            }
            if best_ei_address.unwrap() == ei.address && best_external_info.is_none() {
                best_external_info = Some(ei.clone());
            }
            external_address_types |= ei.address.address_type();
        }

        // There is a popular port on the best external info (more than one external address sample with same port)
        let same_address_has_popular_port = !different_addresses && best_ei_cnt > 1;

        // If we have different addresses in our samples, or no single address has a popular port
        // then we consider this a symmetric NAT
        if different_addresses || !same_address_has_popular_port {
            let this = self.clone();
            let do_symmetric_nat_fut: SendPinBoxFuture<Option<DetectionResult>> =
                Box::pin(async move {
                    Some(DetectionResult {
                        config: this.config,
                        ddi: DetectedDialInfo::SymmetricNAT,
                        external_address_types,
                    })
                });
            unord.push(do_symmetric_nat_fut);
            return;
        }

        // Manual Mapping Detection
        // If we have no external address that matches our local port, then lets try that port
        // on our best external address and see if there's a port forward someone added manually
        ///////////
        let this = self.clone();
        if local_port_matching_external_info.is_none() && best_external_info.is_some() {
            let c_external_1 = best_external_info.as_ref().unwrap().clone();
            let c_this = this.clone();
            let do_manual_map_fut: SendPinBoxFuture<Option<DetectionResult>> =
                Box::pin(async move {
                    // Do a validate_dial_info on the external address, but with the same port as the local port of local interface, from a redirected node
                    // This test is to see if a node had manual port forwarding done with the same port number as the local listener
                    let mut external_1_dial_info_with_local_port = c_external_1.dial_info.clone();
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
                        return Some(DetectionResult {
                            config: c_this.config,
                            ddi: DetectedDialInfo::Detected(DialInfoDetail {
                                dial_info: external_1_dial_info_with_local_port,
                                class: DialInfoClass::Direct,
                            }),
                            external_address_types: AddressTypeSet::only(
                                c_external_1.address.address_type(),
                            ),
                        });
                    }

                    None
                });
            unord.push(do_manual_map_fut);
        }

        // NAT Detection
        ///////////

        // Full Cone NAT Detection
        ///////////
        let this = self.clone();
        let do_nat_detect_fut: SendPinBoxFuture<Option<DetectionResult>> = Box::pin(async move {
            let mut retry_count = this.config().with(|c| c.network.restricted_nat_retries);

            // Loop for restricted NAT retries
            loop {
                let mut ord = FuturesOrdered::new();

                let c_this = this.clone();
                let c_external_1 = external_info.first().cloned().unwrap();
                let do_full_cone_fut: SendPinBoxFuture<Option<DetectionResult>> =
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

                            return Some(DetectionResult {
                                config: c_this.config,
                                ddi: DetectedDialInfo::Detected(DialInfoDetail {
                                    dial_info: c_external_1.dial_info,
                                    class: DialInfoClass::FullConeNAT,
                                }),
                                external_address_types: AddressTypeSet::only(
                                    c_external_1.address.address_type(),
                                ),
                            });
                        }
                        None
                    });
                ord.push_back(do_full_cone_fut);

                let c_this = this.clone();
                let c_external_1 = external_info.first().cloned().unwrap();
                let c_external_2 = external_info.get(1).cloned().unwrap();
                let do_restricted_cone_fut: SendPinBoxFuture<Option<DetectionResult>> =
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
                            return Some(DetectionResult {
                                config: c_this.config,
                                ddi: DetectedDialInfo::Detected(DialInfoDetail {
                                    dial_info: c_external_1.dial_info.clone(),
                                    class: DialInfoClass::AddressRestrictedNAT,
                                }),
                                external_address_types: AddressTypeSet::only(
                                    c_external_1.address.address_type(),
                                ),
                            });
                        }
                        // Didn't get a reply from a non-default port, which means we are also port restricted
                        Some(DetectionResult {
                            config: c_this.config,
                            ddi: DetectedDialInfo::Detected(DialInfoDetail {
                                dial_info: c_external_1.dial_info.clone(),
                                class: DialInfoClass::PortRestrictedNAT,
                            }),
                            external_address_types: AddressTypeSet::only(
                                c_external_1.address.address_type(),
                            ),
                        })
                    });
                ord.push_back(do_restricted_cone_fut);

                // Return the first result we get
                let mut some_dr = None;
                while let Some(res) = ord.next().await {
                    if let Some(dr) = res {
                        some_dr = Some(dr);
                        break;
                    }
                }

                if let Some(dr) = some_dr {
                    if let DetectedDialInfo::Detected(did) = &dr.ddi {
                        // If we got something better than restricted NAT or we're done retrying
                        if did.class < DialInfoClass::AddressRestrictedNAT || retry_count == 0 {
                            return Some(dr);
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
    #[instrument(level = "trace", skip(self))]
    pub async fn discover(
        &self,
        unord: &mut FuturesUnordered<SendPinBoxFuture<Option<DetectionResult>>>,
    ) {
        let enable_upnp = self.config().with(|c| c.network.upnp);

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
            let do_mapped_fut: SendPinBoxFuture<Option<DetectionResult>> = Box::pin(async move {
                // Attempt a port mapping via all available and enabled mechanisms
                // Try this before the direct mapping in the event that we are restarting
                // and may not have recorded a mapping created the last time
                if let Some(external_mapped_dial_info) = this.try_upnp_port_mapping().await {
                    // Got a port mapping, let's use it
                    return Some(DetectionResult {
                        config: this.config,
                        ddi: DetectedDialInfo::Detected(DialInfoDetail {
                            dial_info: external_mapped_dial_info.clone(),
                            class: DialInfoClass::Mapped,
                        }),
                        external_address_types: AddressTypeSet::only(
                            external_mapped_dial_info.address_type(),
                        ),
                    });
                }
                None
            });
            unord.push(do_mapped_fut);
        }

        // NAT Detection
        ///////////

        // If our local interface list contains external_1 then there is no NAT in place
        let local_address_in_external_info = self
            .inner
            .lock()
            .external_info
            .iter()
            .find_map(|ei| self.intf_addrs.contains(&ei.address).then_some(true))
            .unwrap_or_default();

        if local_address_in_external_info {
            self.protocol_process_no_nat(unord).await;
        } else {
            self.protocol_process_nat(unord).await;
        }
    }
}
