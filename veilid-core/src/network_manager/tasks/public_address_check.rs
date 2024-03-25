use super::*;

impl NetworkManager {
    // Clean up the public address check tables, removing entries that have timed out
    #[instrument(level = "trace", skip(self), err)]
    pub(crate) async fn public_address_check_task_routine(
        self,
        stop_token: StopToken,
        _last_ts: Timestamp,
        cur_ts: Timestamp,
    ) -> EyreResult<()> {
        // go through public_address_inconsistencies_table and time out things that have expired
        let mut inner = self.inner.lock();
        for pait_v in inner.public_address_inconsistencies_table.values_mut() {
            let mut expired = Vec::new();
            for (addr, exp_ts) in pait_v.iter() {
                if *exp_ts <= cur_ts {
                    expired.push(*addr);
                }
            }
            for exp in expired {
                pait_v.remove(&exp);
            }
        }
        Ok(())
    }

    // Determine if a local IP address has changed
    // this means we should restart the low level network and and recreate all of our dial info
    // Wait until we have received confirmation from N different peers
    pub fn report_local_network_socket_address(
        &self,
        _socket_address: SocketAddress,
        _flow: Flow,
        _reporting_peer: NodeRef,
    ) {
        // XXX: Nothing here yet.
    }

    // Determine if a global IP address has changed
    // this means we should recreate our public dial info if it is not static and rediscover it
    // Wait until we have received confirmation from N different peers
    pub fn report_public_internet_socket_address(
        &self,
        socket_address: SocketAddress, // the socket address as seen by the remote peer
        flow: Flow,                    // the flow used
        reporting_peer: NodeRef,       // the peer's noderef reporting the socket address
    ) {
        log_network_result!(trace "report_global_socket_address\nsocket_address: {:#?}\nflow: {:#?}\nreporting_peer: {:#?}", socket_address, flow, reporting_peer);

        // Ignore these reports if we are currently detecting public dial info
        let net = self.net();
        if net.needs_public_dial_info_check() {
            return;
        }

        // If we are a webapp we should skip this completely
        // because we will never get inbound dialinfo directly on our public ip address
        // If we have an invalid network class, this is not necessary yet
        let routing_table = self.routing_table();
        let public_internet_network_class = routing_table
            .get_network_class(RoutingDomain::PublicInternet)
            .unwrap_or(NetworkClass::Invalid);
        if matches!(
            public_internet_network_class,
            NetworkClass::Invalid | NetworkClass::WebApp
        ) {
            return;
        }

        let (detect_address_changes, ip6_prefix_size) = self.with_config(|c| {
            (
                c.network.detect_address_changes,
                c.network.max_connections_per_ip6_prefix_size as usize,
            )
        });

        // Get the ip(block) this report is coming from
        let reporting_ipblock = ip_to_ipblock(ip6_prefix_size, flow.remote_address().ip_addr());

        // Reject public address reports from nodes that we know are behind symmetric nat or
        // nodes that must be using a relay for everything
        let Some(node_info) = reporting_peer.node_info(RoutingDomain::PublicInternet) else {
            return;
        };
        if node_info.network_class() != NetworkClass::InboundCapable {
            return;
        }

        // If the socket address reported is the same as the reporter, then this is coming through a relay
        // or it should be ignored due to local proximity (nodes on the same network block should not be trusted as
        // public ip address reporters, only disinterested parties)
        if reporting_ipblock == ip_to_ipblock(ip6_prefix_size, socket_address.ip_addr()) {
            return;
        }

        // Check if the public address report is coming from a node/block that gives an 'inconsistent' location
        // meaning that the node may be not useful for public address detection
        // This is done on a per address/protocol basis

        let mut inner = self.inner.lock();
        let inner = &mut *inner;

        let addr_proto_type_key =
            PublicAddressCheckCacheKey(flow.protocol_type(), flow.address_type());
        if inner
            .public_address_inconsistencies_table
            .get(&addr_proto_type_key)
            .map(|pait| pait.contains_key(&reporting_ipblock))
            .unwrap_or(false)
        {
            return;
        }

        // Insert this new public address into the lru cache for the address check
        // if we've seen this address before, it brings it to the front
        let pacc = inner
            .public_address_check_cache
            .entry(addr_proto_type_key)
            .or_insert_with(|| LruCache::new(PUBLIC_ADDRESS_CHECK_CACHE_SIZE));
        pacc.insert(reporting_ipblock, socket_address);

        // Determine if our external address has likely changed
        let mut bad_public_address_detection_punishment: Option<
            Box<dyn FnOnce() + Send + 'static>,
        > = None;

        let needs_public_address_detection = if matches!(
            public_internet_network_class,
            NetworkClass::InboundCapable
        ) {
            // Get the dial info filter for this connection so we can check if we have any public dialinfo that may have changed
            let dial_info_filter = flow.make_dial_info_filter();

            // Get current external ip/port from registered global dialinfo
            let current_addresses: BTreeSet<SocketAddress> = routing_table
                .all_filtered_dial_info_details(
                    RoutingDomain::PublicInternet.into(),
                    &dial_info_filter,
                )
                .iter()
                .map(|did| {
                    // Strip port from direct and mapped addresses
                    // as the incoming dialinfo may not match the outbound
                    // connections' NAT mapping. In this case we only check for IP address changes.
                    if did.class == DialInfoClass::Direct || did.class == DialInfoClass::Mapped {
                        did.dial_info.socket_address().with_port(0)
                    } else {
                        did.dial_info.socket_address()
                    }
                })
                .collect();

            // If we are inbound capable, but start to see inconsistent socket addresses from multiple reporting peers
            // then we zap the network class and re-detect it

            // Keep list of the origin ip blocks of inconsistent public address reports
            let mut inconsistencies = Vec::new();

            // Iteration goes from most recent to least recent node/address pair
            for (reporting_ip_block, a) in pacc {
                // If this address is not one of our current addresses (inconsistent)
                // and we haven't already denylisted the reporting source,
                // Also check address with port zero in the event we are only checking changes to ip addresses
                if !current_addresses.contains(a)
                    && !current_addresses.contains(&a.with_port(0))
                    && !inner
                        .public_address_inconsistencies_table
                        .get(&addr_proto_type_key)
                        .map(|pait| pait.contains_key(reporting_ip_block))
                        .unwrap_or(false)
                {
                    // Record the origin of the inconsistency
                    log_network_result!(debug "inconsistency added from {:?}: reported {:?} with current_addresses = {:?}", reporting_ip_block, a, current_addresses);

                    inconsistencies.push(*reporting_ip_block);
                }
            }

            // If we have enough inconsistencies to consider changing our public dial info,
            // add them to our denylist (throttling) and go ahead and check for new
            // public dialinfo
            let inconsistent = if inconsistencies.len() >= PUBLIC_ADDRESS_CHANGE_DETECTION_COUNT {
                let exp_ts = get_aligned_timestamp() + PUBLIC_ADDRESS_INCONSISTENCY_TIMEOUT_US;
                let pait = inner
                    .public_address_inconsistencies_table
                    .entry(addr_proto_type_key)
                    .or_default();
                for i in &inconsistencies {
                    pait.insert(*i, exp_ts);
                }

                // Run this routine if the inconsistent nodes turn out to be lying
                let this = self.clone();
                bad_public_address_detection_punishment = Some(Box::new(move || {
                    let mut inner = this.inner.lock();
                    let pait = inner
                        .public_address_inconsistencies_table
                        .entry(addr_proto_type_key)
                        .or_default();
                    let exp_ts = get_aligned_timestamp()
                        + PUBLIC_ADDRESS_INCONSISTENCY_PUNISHMENT_TIMEOUT_US;
                    for i in inconsistencies {
                        pait.insert(i, exp_ts);
                    }
                }));

                true
            } else {
                false
            };

            // // debug code
            // if inconsistent {
            //     log_net!("public_address_check_cache: {:#?}\ncurrent_addresses: {:#?}\ninconsistencies: {}", inner
            //                 .public_address_check_cache, current_addresses, inconsistencies);
            // }

            inconsistent
        } else if matches!(public_internet_network_class, NetworkClass::OutboundOnly) {
            // If we are currently outbound only, we don't have any public dial info
            // but if we are starting to see consistent socket address from multiple reporting peers
            // then we may be become inbound capable, so zap the network class so we can re-detect it and any public dial info

            let mut consistencies = 0;
            let mut consistent = false;
            let mut current_address = Option::<SocketAddress>::None;

            // Iteration goes from most recent to least recent node/address pair
            for (_, a) in pacc {
                if let Some(current_address) = current_address {
                    if current_address == *a {
                        consistencies += 1;
                        if consistencies >= PUBLIC_ADDRESS_CHANGE_DETECTION_COUNT {
                            consistent = true;
                            break;
                        }
                    }
                } else {
                    current_address = Some(*a);
                }
            }
            consistent
        } else {
            // If we are a webapp we never do this.
            // If we have invalid network class, then public address detection is already going to happen via the network_class_discovery task

            // we should have checked for this condition earlier at the top of this function
            unreachable!();
        };

        if needs_public_address_detection {
            if detect_address_changes {
                // Reset the address check cache now so we can start detecting fresh
                info!("Public address has changed, detecting public dial info");

                inner.public_address_check_cache.clear();

                // Re-detect the public dialinfo
                net.set_needs_public_dial_info_check(bad_public_address_detection_punishment);
            } else {
                warn!("Public address may have changed. Restarting the server may be required.");
                warn!("report_global_socket_address\nsocket_address: {:#?}\nflow: {:#?}\nreporting_peer: {:#?}", socket_address, flow, reporting_peer);
                warn!(
                    "public_address_check_cache: {:#?}",
                    inner.public_address_check_cache
                );
            }
        }
    }
}
