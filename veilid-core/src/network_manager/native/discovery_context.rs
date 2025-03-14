/// Context detection of public dial info for a single protocol and address type
/// Also performs UPNP/IGD mapping if enabled and possible
use super::*;
use futures_util::stream::FuturesUnordered;
use igd_manager::{IGDAddressType, IGDProtocolType};
use stop_token::future::FutureExt as _;

impl_veilid_log_facility!("net");

const EXTERNAL_INFO_NODE_COUNT: usize = 20;
const EXTERNAL_INFO_CONCURRENCY: usize = 20;
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

#[derive(Clone, Debug)]
enum DetectionResultKind {
    Result {
        result: DetectionResult,
        possibilities: Vec<DialInfoClassPossibility>,
    },
    Failure {
        possibilities: Vec<DialInfoClassPossibility>,
    },
}
////////////////////////////////////////////////////////////////////////////

type DialInfoClassPossibility = (DialInfoClass, usize);

#[derive(Debug)]
struct DialInfoClassAllPossibilities {
    remaining: BTreeMap<DialInfoClass, usize>,
}

impl DialInfoClassAllPossibilities {
    pub fn new() -> Self {
        Self {
            remaining: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, possibilities: &[DialInfoClassPossibility]) {
        for (k, v) in possibilities {
            *self.remaining.entry(*k).or_default() += v;
        }
    }
    pub fn remove(&mut self, possibilities: &[DialInfoClassPossibility]) {
        for (k, v) in possibilities {
            *self.remaining.entry(*k).or_default() -= v;
        }
    }
    pub fn any_better(&mut self, dial_info_class: DialInfoClass) -> bool {
        let best_available_order: [DialInfoClassSet; 4] = [
            DialInfoClass::Mapped.into(),
            DialInfoClass::Direct | DialInfoClass::Blocked,
            DialInfoClass::FullConeNAT.into(),
            DialInfoClass::AddressRestrictedNAT | DialInfoClass::PortRestrictedNAT,
        ];

        for bestdicset in best_available_order {
            // Already got the best we've checked so far?
            if bestdicset.contains(dial_info_class) {
                // We can just stop here since nothing else is going to be better
                return false;
            }

            // Get the total remaining possibilities left at this level
            let mut remaining = 0usize;
            for bestdic in bestdicset {
                remaining += self.remaining.get(&bestdic).copied().unwrap_or_default()
            }

            if remaining > 0 {
                // There's some things worth waiting for that could be better than dial_info_class
                return true;
            }
        }

        // Nothing left to wait for
        false
    }
}

impl Default for DialInfoClassAllPossibilities {
    fn default() -> Self {
        Self::new()
    }
}

////////////////////////////////////////////////////////////////////////////

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
    external_infos: Vec<ExternalInfo>,
    mapped_dial_info: Option<DialInfo>,
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
    stop_token: StopToken,
}

impl_veilid_component_registry_accessor!(DiscoveryContext);

impl core::ops::Deref for DiscoveryContext {
    type Target = DiscoveryContextUnlockedInner;

    fn deref(&self) -> &Self::Target {
        &self.unlocked_inner
    }
}

impl DiscoveryContext {
    pub fn new(
        registry: VeilidComponentRegistry,
        config: DiscoveryContextConfig,
        stop_token: StopToken,
    ) -> Self {
        let routing_table = registry.routing_table();
        let intf_addrs =
            Self::get_local_addresses(&routing_table, config.protocol_type, config.address_type);

        Self {
            registry,
            unlocked_inner: Arc::new(DiscoveryContextUnlockedInner { config, intf_addrs }),
            inner: Arc::new(Mutex::new(DiscoveryContextInner {
                external_infos: Vec::new(),
                mapped_dial_info: None,
            })),
            stop_token,
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

        let res = network_result_value_or_log!(self match rpc.rpc_call_status(Destination::direct(node_ref.clone())).await {
                Ok(v) => v,
                Err(e) => {
                    veilid_log!(self error
                        "failed to get status answer from {:?}: {}",
                        node_ref, e
                    );
                    return None;
                }
            } => [ format!(": node_ref={}", node_ref) ] {
                return None;
            }
        );

        veilid_log!(self debug target:"network_result",
            "request_public_address {:?}: Value({:?})",
            node_ref,
            res.answer
        );
        res.answer.opt_sender_info.map(|si| si.socket_address)
    }

    // find fast peers with a particular address type, and ask them to tell us what our external address is
    // This is done over the normal port using RPC
    #[instrument(level = "trace", skip(self), ret)]
    async fn discover_external_addresses(&self) -> bool {
        let node_count = EXTERNAL_INFO_NODE_COUNT;
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
            veilid_log!(self debug
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
            if unord.len() == EXTERNAL_INFO_CONCURRENCY {
                // Process one
                match unord
                    .next()
                    .timeout_at(self.stop_token.clone())
                    .in_current_span()
                    .await
                {
                    Ok(Some(Some(ei))) => {
                        external_address_infos.push(ei);
                        if external_address_infos.len() == EXTERNAL_INFO_VALIDATIONS {
                            break;
                        }
                    }
                    Ok(Some(None)) => {
                        // Found no public address from this node
                    }
                    Ok(None) => {
                        // Should never happen in this loop
                        unreachable!();
                    }
                    Err(_) => {
                        // stop requested
                        return false;
                    }
                }
            }
        }
        // Finish whatever is left if we need to
        while external_address_infos.len() < EXTERNAL_INFO_VALIDATIONS {
            match unord
                .next()
                .timeout_at(self.stop_token.clone())
                .in_current_span()
                .await
            {
                Ok(Some(Some(ei))) => {
                    external_address_infos.push(ei);
                }
                Ok(Some(None)) => {
                    // Found no public address from this node
                }
                Ok(None) => {
                    // No nodes left to wait for
                    break;
                }
                Err(_) => {
                    // stop requested
                    return false;
                }
            }
        }
        if external_address_infos.len() < EXTERNAL_INFO_VALIDATIONS {
            veilid_log!(self debug "not enough peers ({}<{}) responded with an external address for type {:?}:{:?}",
                external_address_infos.len(),
                EXTERNAL_INFO_VALIDATIONS,
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
            inner.external_infos = external_address_infos;
            veilid_log!(self debug "External Addresses ({:?}:{:?}):\n{}",
                protocol_type,
                address_type,
                inner.external_infos.iter().map(|x| format!("    {} <- {}",x.address, x.node)).collect::<Vec<_>>().join("\n"));
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
                veilid_log!(self trace "failed to send validate_dial_info to {:?}: {}", node_ref, e);
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

        let igd_manager = self.network_manager().net().igd_manager.clone();

        // Attempt a port mapping. If this doesn't succeed, it's not going to
        let mapped_external_address = igd_manager
            .map_any_port(igd_protocol_type, igd_address_type, local_port, None)
            .await?;

        // Make dial info from the port mapping
        let external_mapped_dial_info = self.network_manager().net().make_dial_info(
            SocketAddress::from_socket_addr(mapped_external_address),
            protocol_type,
        );

        Some(external_mapped_dial_info)
    }

    fn matches_mapped_dial_info(&self, dial_info: &DialInfo) -> bool {
        let mut skip = false;
        if let Some(mapped_dial_info) = self.inner.lock().mapped_dial_info.as_ref() {
            if mapped_dial_info == dial_info {
                skip = true;
            }
        }
        skip
    }

    ///////
    // Per-protocol discovery routines

    // If we know we are not behind NAT, check our firewall status
    #[instrument(level = "trace", skip(self), ret)]
    fn protocol_process_mapped_dial_info(
        &self,
        all_possibilities: &mut DialInfoClassAllPossibilities,
        unord: &mut FuturesUnordered<PinBoxFutureStatic<DetectionResultKind>>,
    ) {
        let (external_infos, mapped_dial_info) = {
            let inner = self.inner.lock();
            let Some(mapped_dial_info) = inner.mapped_dial_info.clone() else {
                return;
            };

            (inner.external_infos.clone(), mapped_dial_info)
        };

        // Have all the external validator nodes check us
        for external_info in external_infos {
            let possibilities = vec![(DialInfoClass::Mapped, 1)];
            all_possibilities.add(&possibilities);

            let this = self.clone();
            let mapped_dial_info = mapped_dial_info.clone();
            let do_no_nat_fut: PinBoxFutureStatic<DetectionResultKind> = Box::pin(async move {
                // Do a validate_dial_info on the external address from a redirected node
                if this
                    .validate_dial_info(external_info.node.clone(), mapped_dial_info.clone(), true)
                    .await
                {
                    // Add public dial info with Direct dialinfo class
                    DetectionResultKind::Result {
                        possibilities,
                        result: DetectionResult {
                            config: this.config,
                            ddi: DetectedDialInfo::Detected(DialInfoDetail {
                                dial_info: mapped_dial_info.clone(),
                                class: DialInfoClass::Mapped,
                            }),
                            external_address_types: AddressTypeSet::only(
                                external_info.address.address_type(),
                            ),
                        },
                    }
                } else {
                    DetectionResultKind::Failure { possibilities }
                }
            });

            unord.push(do_no_nat_fut);
        }
    }

    // If we know we are not behind NAT, check our firewall status
    #[instrument(level = "trace", skip(self), ret)]
    fn protocol_process_no_nat(
        &self,
        all_possibilities: &mut DialInfoClassAllPossibilities,
        unord: &mut FuturesUnordered<PinBoxFutureStatic<DetectionResultKind>>,
    ) {
        let external_infos = self.inner.lock().external_infos.clone();

        // Have all the external validator nodes check us
        for external_info in external_infos {
            // If this is the same as an existing upnp mapping, skip it, since
            // we are already validating that
            if self.matches_mapped_dial_info(&external_info.dial_info) {
                continue;
            }

            let possibilities = vec![(DialInfoClass::Direct, 1), (DialInfoClass::Blocked, 1)];
            all_possibilities.add(&possibilities);

            let this = self.clone();
            let do_no_nat_fut: PinBoxFutureStatic<DetectionResultKind> = Box::pin(async move {
                // Do a validate_dial_info on the external address from a redirected node
                if this
                    .validate_dial_info(
                        external_info.node.clone(),
                        external_info.dial_info.clone(),
                        true,
                    )
                    .await
                {
                    // Add public dial info with Direct dialinfo class
                    DetectionResultKind::Result {
                        possibilities,
                        result: DetectionResult {
                            config: this.config,
                            ddi: DetectedDialInfo::Detected(DialInfoDetail {
                                dial_info: external_info.dial_info.clone(),
                                class: DialInfoClass::Direct,
                            }),
                            external_address_types: AddressTypeSet::only(
                                external_info.address.address_type(),
                            ),
                        },
                    }
                } else {
                    // Add public dial info with Blocked dialinfo class
                    DetectionResultKind::Result {
                        possibilities,
                        result: DetectionResult {
                            config: this.config,
                            ddi: DetectedDialInfo::Detected(DialInfoDetail {
                                dial_info: external_info.dial_info.clone(),
                                class: DialInfoClass::Blocked,
                            }),
                            external_address_types: AddressTypeSet::only(
                                external_info.address.address_type(),
                            ),
                        },
                    }
                }
            });

            unord.push(do_no_nat_fut);
        }
    }

    // If we know we are behind NAT check what kind
    #[instrument(level = "trace", skip(self), ret)]
    fn protocol_process_nat(
        &self,
        all_possibilities: &mut DialInfoClassAllPossibilities,
        unord: &mut FuturesUnordered<PinBoxFutureStatic<DetectionResultKind>>,
    ) {
        // Get the external dial info histogram for our use here
        let external_info = {
            let inner = self.inner.lock();
            inner.external_infos.clone()
        };
        let local_port = self.config.port;

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
            let do_symmetric_nat_fut: PinBoxFutureStatic<DetectionResultKind> =
                Box::pin(async move {
                    DetectionResultKind::Result {
                        // Don't bother tracking possibilities for SymmetricNAT
                        // it's never going to be 'better than' anything else
                        possibilities: vec![],
                        result: DetectionResult {
                            config: this.config,
                            ddi: DetectedDialInfo::SymmetricNAT,
                            external_address_types,
                        },
                    }
                });
            unord.push(do_symmetric_nat_fut);

            return;
        }

        // Manual Mapping Detection
        // If we have no external address that matches our local port, then lets try that port
        // on our best external address and see if there's a port forward someone added manually
        ///////////
        if local_port_matching_external_info.is_none() && best_external_info.is_some() {
            let c_external_1 = best_external_info.as_ref().unwrap().clone();

            // Do a validate_dial_info on the external address, but with the same port as the local port of local interface, from a redirected node
            // This test is to see if a node had manual port forwarding done with the same port number as the local listener
            let mut external_1_dial_info_with_local_port = c_external_1.dial_info.clone();
            external_1_dial_info_with_local_port.set_port(local_port);

            // If this is the same as an existing upnp mapping, skip it, since
            // we are already validating that
            if !self.matches_mapped_dial_info(&external_1_dial_info_with_local_port) {
                let possibilities = vec![(DialInfoClass::Direct, 1)];
                all_possibilities.add(&possibilities);

                let c_this = self.clone();
                let do_manual_map_fut: PinBoxFutureStatic<DetectionResultKind> =
                    Box::pin(async move {
                        if c_this
                            .validate_dial_info(
                                c_external_1.node.clone(),
                                external_1_dial_info_with_local_port.clone(),
                                true,
                            )
                            .await
                        {
                            // Add public dial info with Direct dialinfo class
                            return DetectionResultKind::Result {
                                possibilities,
                                result: DetectionResult {
                                    config: c_this.config,
                                    ddi: DetectedDialInfo::Detected(DialInfoDetail {
                                        dial_info: external_1_dial_info_with_local_port,
                                        class: DialInfoClass::Direct,
                                    }),
                                    external_address_types: AddressTypeSet::only(
                                        c_external_1.address.address_type(),
                                    ),
                                },
                            };
                        }

                        DetectionResultKind::Failure { possibilities }
                    });
                unord.push(do_manual_map_fut);
            }
        }

        // NAT Detection
        ///////////

        let retry_count = self.config().with(|c| c.network.restricted_nat_retries);

        // Full Cone NAT Detection
        ///////////

        let possibilities = vec![(DialInfoClass::FullConeNAT, 1)];
        all_possibilities.add(&possibilities);

        let c_this = self.clone();
        let c_external_1 = external_info.first().cloned().unwrap();

        // If this is the same as an existing upnp mapping, skip it, since
        // we are already validating that
        if !self.matches_mapped_dial_info(&c_external_1.dial_info) {
            let do_full_cone_fut: PinBoxFutureStatic<DetectionResultKind> = Box::pin(async move {
                let mut retry_count = retry_count;

                // Let's see what kind of NAT we have
                // Does a redirected dial info validation from a different address and a random port find us?
                loop {
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

                        return DetectionResultKind::Result {
                            possibilities,
                            result: DetectionResult {
                                config: c_this.config,
                                ddi: DetectedDialInfo::Detected(DialInfoDetail {
                                    dial_info: c_external_1.dial_info,
                                    class: DialInfoClass::FullConeNAT,
                                }),
                                external_address_types: AddressTypeSet::only(
                                    c_external_1.address.address_type(),
                                ),
                            },
                        };
                    }
                    if retry_count == 0 {
                        break;
                    }
                    retry_count -= 1;
                }

                DetectionResultKind::Failure { possibilities }
            });
            unord.push(do_full_cone_fut);

            let possibilities = vec![
                (DialInfoClass::AddressRestrictedNAT, 1),
                (DialInfoClass::PortRestrictedNAT, 1),
            ];
            all_possibilities.add(&possibilities);

            let c_this = self.clone();
            let c_external_1 = external_info.first().cloned().unwrap();
            let c_external_2 = external_info.get(1).cloned().unwrap();
            let do_restricted_cone_fut: PinBoxFutureStatic<DetectionResultKind> =
                Box::pin(async move {
                    let mut retry_count = retry_count;

                    // We are restricted, determine what kind of restriction

                    // If we're going to end up as a restricted NAT of some sort
                    // Address is the same, so it's address or port restricted

                    loop {
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
                            return DetectionResultKind::Result {
                                possibilities,
                                result: DetectionResult {
                                    config: c_this.config,
                                    ddi: DetectedDialInfo::Detected(DialInfoDetail {
                                        dial_info: c_external_1.dial_info.clone(),
                                        class: DialInfoClass::AddressRestrictedNAT,
                                    }),
                                    external_address_types: AddressTypeSet::only(
                                        c_external_1.address.address_type(),
                                    ),
                                },
                            };
                        }

                        if retry_count == 0 {
                            break;
                        }
                        retry_count -= 1;
                    }

                    // Didn't get a reply from a non-default port, which means we are also port restricted
                    DetectionResultKind::Result {
                        possibilities,
                        result: DetectionResult {
                            config: c_this.config,
                            ddi: DetectedDialInfo::Detected(DialInfoDetail {
                                dial_info: c_external_1.dial_info.clone(),
                                class: DialInfoClass::PortRestrictedNAT,
                            }),
                            external_address_types: AddressTypeSet::only(
                                c_external_1.address.address_type(),
                            ),
                        },
                    }
                });
            unord.push(do_restricted_cone_fut);
        }
    }

    /// Run a discovery for a particular context
    /// Returns None if no detection was possible
    /// Returns Some(DetectionResult) with the best detection result for this context
    #[instrument(level = "trace", skip(self))]
    pub async fn discover(self) -> Option<DetectionResult> {
        // Do this right away because it's fast and every detection is going to need it
        // Get our external addresses from a bunch of fast nodes
        if !self.discover_external_addresses().await {
            // If we couldn't get an external address, then we should just try the whole network class detection again later
            return None;
        }

        // The set of futures we're going to wait on to determine dial info class for this context
        let mut unord = FuturesUnordered::<PinBoxFutureStatic<DetectionResultKind>>::new();

        // Used to determine what is still worth waiting for since we always want to return the
        // best available dial info class. Once there are no better options in our waiting set
        // we can just return what we've got.
        let mut all_possibilities = DialInfoClassAllPossibilities::new();

        // UPNP Automatic Mapping
        ///////////

        let enable_upnp = self.config().with(|c| c.network.upnp);
        if enable_upnp {
            // Attempt a port mapping via all available and enabled mechanisms
            // Try this before the direct mapping in the event that we are restarting
            // and may not have recorded a mapping created the last time
            if let Some(external_mapped_dial_info) = self.try_upnp_port_mapping().await {
                // Got a port mapping, store it
                self.inner.lock().mapped_dial_info = Some(external_mapped_dial_info);

                // And validate it
                self.protocol_process_mapped_dial_info(&mut all_possibilities, &mut unord);
            }
        }

        // NAT Detection
        ///////////

        // If our local interface list contains any of the external addresses then there is no NAT in place
        let local_address_in_external_info = self
            .inner
            .lock()
            .external_infos
            .iter()
            .find_map(|ei| self.intf_addrs.contains(&ei.address).then_some(true))
            .unwrap_or_default();

        if local_address_in_external_info {
            self.protocol_process_no_nat(&mut all_possibilities, &mut unord);
        } else {
            self.protocol_process_nat(&mut all_possibilities, &mut unord);
        }

        // Wait for the best detection result to roll in
        let mut opt_best_detection_result: Option<DetectionResult> = None;
        loop {
            match unord
                .next()
                .timeout_at(self.stop_token.clone())
                .in_current_span()
                .await
            {
                Ok(Some(DetectionResultKind::Result {
                    result,
                    possibilities,
                })) => {
                    // Remove possible dial info classes from our available set
                    all_possibilities.remove(&possibilities);

                    // Get best detection result for each discovery context config
                    if let Some(best_detection_result) = &mut opt_best_detection_result {
                        let ddi = &mut best_detection_result.ddi;
                        // Upgrade existing dialinfo
                        match ddi {
                            DetectedDialInfo::SymmetricNAT => {
                                // Whatever we got is better than or equal to symmetric
                                *ddi = result.ddi;
                            }
                            DetectedDialInfo::Detected(cur_did) => match result.ddi {
                                DetectedDialInfo::SymmetricNAT => {
                                    // Nothing is worse than this
                                }
                                DetectedDialInfo::Detected(did) => {
                                    // Pick the best dial info class we detected
                                    // because some nodes could be degenerate and if any node can validate a
                                    // better dial info class we should go with it and leave the
                                    // degenerate nodes in the dust to fade into obscurity
                                    if did.class < cur_did.class {
                                        cur_did.class = did.class;
                                    }
                                }
                            },
                        }
                        best_detection_result.external_address_types |=
                            result.external_address_types;
                    } else {
                        opt_best_detection_result = Some(result);
                    }
                }
                Ok(Some(DetectionResultKind::Failure { possibilities })) => {
                    // Found no dial info for this protocol/address combination

                    // Remove possible dial info classes from our available set
                    all_possibilities.remove(&possibilities);
                }
                Ok(None) => {
                    // All done, normally
                    break;
                }
                Err(_) => {
                    // Stop token, exit early without error propagation
                    return None;
                }
            }

            // See if there's any better results worth waiting for
            if let Some(best_detection_result) = &opt_best_detection_result {
                if let DetectedDialInfo::Detected(did) = &best_detection_result.ddi {
                    // If nothing else is going to be a better result, just stop here
                    if !all_possibilities.any_better(did.class) {
                        break;
                    }
                }
            }
        }

        opt_best_detection_result
    }
}
