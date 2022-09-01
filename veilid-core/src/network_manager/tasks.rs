use super::*;

use crate::dht::*;
use crate::xx::*;
use futures_util::FutureExt;
use stop_token::future::FutureExt as StopFutureExt;

impl NetworkManager {
    // Bootstrap lookup process
    #[instrument(level = "trace", skip(self), ret, err)]
    pub(super) async fn resolve_bootstrap(
        &self,
        bootstrap: Vec<String>,
    ) -> EyreResult<BootstrapRecordMap> {
        // Resolve from bootstrap root to bootstrap hostnames
        let mut bsnames = Vec::<String>::new();
        for bh in bootstrap {
            // Get TXT record for bootstrap (bootstrap.veilid.net, or similar)
            let records = intf::txt_lookup(&bh).await?;
            for record in records {
                // Split the bootstrap name record by commas
                for rec in record.split(',') {
                    let rec = rec.trim();
                    // If the name specified is fully qualified, go with it
                    let bsname = if rec.ends_with('.') {
                        rec.to_string()
                    }
                    // If the name is not fully qualified, prepend it to the bootstrap name
                    else {
                        format!("{}.{}", rec, bh)
                    };

                    // Add to the list of bootstrap name to look up
                    bsnames.push(bsname);
                }
            }
        }

        // Get bootstrap nodes from hostnames concurrently
        let mut unord = FuturesUnordered::new();
        for bsname in bsnames {
            unord.push(async move {
                // look up boostrap node txt records
                let bsnirecords = match intf::txt_lookup(&bsname).await {
                    Err(e) => {
                        warn!("bootstrap node txt lookup failed for {}: {}", bsname, e);
                        return None;
                    }
                    Ok(v) => v,
                };
                // for each record resolve into key/bootstraprecord pairs
                let mut bootstrap_records: Vec<(DHTKey, BootstrapRecord)> = Vec::new();
                for bsnirecord in bsnirecords {
                    // Bootstrap TXT Record Format Version 0:
                    // txt_version,min_version,max_version,nodeid,hostname,dialinfoshort*
                    //
                    // Split bootstrap node record by commas. Example:
                    // 0,0,0,7lxDEabK_qgjbe38RtBa3IZLrud84P6NhGP-pRTZzdQ,bootstrap-1.dev.veilid.net,T5150,U5150,W5150/ws
                    let records: Vec<String> = bsnirecord
                        .trim()
                        .split(',')
                        .map(|x| x.trim().to_owned())
                        .collect();
                    if records.len() < 6 {
                        warn!("invalid number of fields in bootstrap txt record");
                        continue;
                    }

                    // Bootstrap TXT record version
                    let txt_version: u8 = match records[0].parse::<u8>() {
                        Ok(v) => v,
                        Err(e) => {
                            warn!(
                                "invalid txt_version specified in bootstrap node txt record: {}",
                                e
                            );
                            continue;
                        }
                    };
                    if txt_version != BOOTSTRAP_TXT_VERSION {
                        warn!("unsupported bootstrap txt record version");
                        continue;
                    }

                    // Min/Max wire protocol version
                    let min_version: u8 = match records[1].parse::<u8>() {
                        Ok(v) => v,
                        Err(e) => {
                            warn!(
                                "invalid min_version specified in bootstrap node txt record: {}",
                                e
                            );
                            continue;
                        }
                    };
                    let max_version: u8 = match records[2].parse::<u8>() {
                        Ok(v) => v,
                        Err(e) => {
                            warn!(
                                "invalid max_version specified in bootstrap node txt record: {}",
                                e
                            );
                            continue;
                        }
                    };

                    // Node Id
                    let node_id_str = &records[3];
                    let node_id_key = match DHTKey::try_decode(node_id_str) {
                        Ok(v) => v,
                        Err(e) => {
                            warn!(
                                "Invalid node id in bootstrap node record {}: {}",
                                node_id_str, e
                            );
                            continue;
                        }
                    };

                    // Hostname
                    let hostname_str = &records[4];

                    // If this is our own node id, then we skip it for bootstrap, in case we are a bootstrap node
                    if self.routing_table().node_id() == node_id_key {
                        continue;
                    }

                    // Resolve each record and store in node dial infos list
                    let mut bootstrap_record = BootstrapRecord {
                        min_version,
                        max_version,
                        dial_info_details: Vec::new(),
                    };
                    for rec in &records[5..] {
                        let rec = rec.trim();
                        let dial_infos = match DialInfo::try_vec_from_short(rec, hostname_str) {
                            Ok(dis) => dis,
                            Err(e) => {
                                warn!("Couldn't resolve bootstrap node dial info {}: {}", rec, e);
                                continue;
                            }
                        };

                        for di in dial_infos {
                            bootstrap_record.dial_info_details.push(DialInfoDetail {
                                dial_info: di,
                                class: DialInfoClass::Direct,
                            });
                        }
                    }
                    bootstrap_records.push((node_id_key, bootstrap_record));
                }
                Some(bootstrap_records)
            });
        }

        let mut bsmap = BootstrapRecordMap::new();
        while let Some(bootstrap_records) = unord.next().await {
            if let Some(bootstrap_records) = bootstrap_records {
                for (bskey, mut bsrec) in bootstrap_records {
                    let rec = bsmap.entry(bskey).or_insert_with(|| BootstrapRecord {
                        min_version: bsrec.min_version,
                        max_version: bsrec.max_version,
                        dial_info_details: Vec::new(),
                    });
                    rec.dial_info_details.append(&mut bsrec.dial_info_details);
                }
            }
        }

        Ok(bsmap)
    }

    // 'direct' bootstrap task routine for systems incapable of resolving TXT records, such as browser WASM
    pub(super) async fn direct_bootstrap_task_routine(
        self,
        stop_token: StopToken,
        bootstrap_dialinfos: Vec<DialInfo>,
    ) -> EyreResult<()> {
        let mut unord = FuturesUnordered::new();
        let routing_table = self.routing_table();

        for bootstrap_di in bootstrap_dialinfos {
            let peer_info = self.boot_request(bootstrap_di).await?;

            // Got peer info, let's add it to the routing table
            for pi in peer_info {
                let k = pi.node_id.key;
                // Register the node
                if let Some(nr) = routing_table.register_node_with_signed_node_info(
                    RoutingDomain::PublicInternet,
                    k,
                    pi.signed_node_info,
                    false,
                ) {
                    // Add this our futures to process in parallel
                    let routing_table = routing_table.clone();
                    unord.push(
                        // lets ask bootstrap to find ourselves now
                        async move { routing_table.reverse_find_node(nr, true).await },
                    );
                }
            }
        }

        // Wait for all bootstrap operations to complete before we complete the singlefuture
        while let Ok(Some(_)) = unord.next().timeout_at(stop_token.clone()).await {}

        Ok(())
    }

    #[instrument(level = "trace", skip(self), err)]
    pub(super) async fn bootstrap_task_routine(self, stop_token: StopToken) -> EyreResult<()> {
        let (bootstrap, bootstrap_nodes) = {
            let c = self.config.get();
            (
                c.network.bootstrap.clone(),
                c.network.bootstrap_nodes.clone(),
            )
        };
        let routing_table = self.routing_table();

        log_net!(debug "--- bootstrap_task");

        // See if we are specifying a direct dialinfo for bootstrap, if so use the direct mechanism
        if !bootstrap.is_empty() && bootstrap_nodes.is_empty() {
            let mut bootstrap_dialinfos = Vec::<DialInfo>::new();
            for b in &bootstrap {
                if let Ok(bootstrap_di_vec) = DialInfo::try_vec_from_url(&b) {
                    for bootstrap_di in bootstrap_di_vec {
                        bootstrap_dialinfos.push(bootstrap_di);
                    }
                }
            }
            if bootstrap_dialinfos.len() > 0 {
                return self
                    .direct_bootstrap_task_routine(stop_token, bootstrap_dialinfos)
                    .await;
            }
        }

        // If we aren't specifying a bootstrap node list explicitly, then pull from the bootstrap server(s)
        let bsmap: BootstrapRecordMap = if !bootstrap_nodes.is_empty() {
            let mut bsmap = BootstrapRecordMap::new();
            let mut bootstrap_node_dial_infos = Vec::new();
            for b in bootstrap_nodes {
                let ndis = NodeDialInfo::from_str(b.as_str())
                    .wrap_err("Invalid node dial info in bootstrap entry")?;
                bootstrap_node_dial_infos.push(ndis);
            }
            for ndi in bootstrap_node_dial_infos {
                let node_id = ndi.node_id.key;
                bsmap
                    .entry(node_id)
                    .or_insert_with(|| BootstrapRecord {
                        min_version: MIN_VERSION,
                        max_version: MAX_VERSION,
                        dial_info_details: Vec::new(),
                    })
                    .dial_info_details
                    .push(DialInfoDetail {
                        dial_info: ndi.dial_info,
                        class: DialInfoClass::Direct, // Bootstraps are always directly reachable
                    });
            }
            bsmap
        } else {
            // Resolve bootstrap servers and recurse their TXT entries
            self.resolve_bootstrap(bootstrap).await?
        };

        // Map all bootstrap entries to a single key with multiple dialinfo

        // Run all bootstrap operations concurrently
        let mut unord = FuturesUnordered::new();
        for (k, mut v) in bsmap {
            // Sort dial info so we get the preferred order correct
            v.dial_info_details.sort();

            log_net!("--- bootstrapping {} with {:?}", k.encode(), &v);

            // Make invalid signed node info (no signature)
            if let Some(nr) = routing_table.register_node_with_signed_node_info(
                RoutingDomain::PublicInternet,
                k,
                SignedNodeInfo::with_no_signature(NodeInfo {
                    network_class: NetworkClass::InboundCapable, // Bootstraps are always inbound capable
                    outbound_protocols: ProtocolTypeSet::only(ProtocolType::UDP), // Bootstraps do not participate in relaying and will not make outbound requests, but will have UDP enabled
                    address_types: AddressTypeSet::all(), // Bootstraps are always IPV4 and IPV6 capable
                    min_version: v.min_version, // Minimum protocol version specified in txt record
                    max_version: v.max_version, // Maximum protocol version specified in txt record
                    dial_info_detail_list: v.dial_info_details, // Dial info is as specified in the bootstrap list
                    relay_peer_info: None, // Bootstraps never require a relay themselves
                }),
                true,
            ) {
                // Add this our futures to process in parallel
                let routing_table = routing_table.clone();
                unord.push(async move {
                    // Need VALID signed peer info, so ask bootstrap to find_node of itself
                    // which will ensure it has the bootstrap's signed peer info as part of the response
                    let _ = routing_table.find_target(nr.clone()).await;

                    // Ensure we got the signed peer info
                    if !nr.has_valid_signed_node_info(Some(RoutingDomain::PublicInternet)) {
                        log_net!(warn
                            "bootstrap at {:?} did not return valid signed node info",
                            nr
                        );
                        // If this node info is invalid, it will time out after being unpingable
                    } else {
                        // otherwise this bootstrap is valid, lets ask it to find ourselves now
                        routing_table.reverse_find_node(nr, true).await
                    }
                });
            }
        }

        // Wait for all bootstrap operations to complete before we complete the singlefuture
        while let Ok(Some(_)) = unord.next().timeout_at(stop_token.clone()).await {}
        Ok(())
    }

    // Ping each node in the routing table if they need to be pinged
    // to determine their reliability
    #[instrument(level = "trace", skip(self), err)]
    pub(super) async fn ping_validator_task_routine(
        self,
        stop_token: StopToken,
        _last_ts: u64,
        cur_ts: u64,
    ) -> EyreResult<()> {
        let rpc = self.rpc_processor();
        let routing_table = self.routing_table();

        let mut unord = FuturesUnordered::new();

        let node_refs = routing_table.get_nodes_needing_ping(cur_ts);
        let mut mapped_port_info = routing_table.get_mapped_port_info();
        let opt_public_internet_relay_nr = routing_table.relay_node(RoutingDomain::PublicInternet);
        let opt_public_internet_relay_id = opt_public_internet_relay_nr.map(|nr| nr.node_id());
        // let opt_local_network_relay_nr = routing_table.relay_node(RoutingDomain::LocalNetwork);
        // let opt_local_network_relay_id = opt_local_network_relay_nr.map(|nr| nr.node_id());

        // Public Internet Routing Domain
        let dids = routing_table.all_filtered_dial_info_details(
            Some(RoutingDomain::PublicInternet),
            &DialInfoFilter::global(),
        );

        for nr in node_refs {
            let rpc = rpc.clone();
            if Some(nr.node_id()) == opt_public_internet_relay_id {
                // Relay nodes get pinged over all protocols we have inbound dialinfo for
                // This is so we can preserve the inbound NAT mappings at our router
                for did in &dids {
                    // Do we need to do this ping?
                    // Check if we have already pinged over this low-level-protocol/address-type/port combo
                    // We want to ensure we do the bare minimum required here
                    let pt = did.dial_info.protocol_type();
                    let at = did.dial_info.address_type();
                    let needs_ping = if let Some((llpt, port)) =
                        mapped_port_info.protocol_to_port.get(&(pt, at))
                    {
                        mapped_port_info
                            .low_level_protocol_ports
                            .remove(&(*llpt, at, *port))
                    } else {
                        false
                    };
                    if needs_ping {
                        let rpc = rpc.clone();
                        let dif = did.dial_info.make_filter(true);
                        let nr_filtered = nr.filtered_clone(dif);
                        log_net!("--> Keepalive ping to {:?}", nr_filtered);
                        unord.push(async move { rpc.rpc_call_status(nr_filtered).await }.boxed());
                    }
                }
            } else {
                // Just do a single ping with the best protocol for all the other nodes
                unord.push(async move { rpc.rpc_call_status(nr).await }.boxed());
            }
        }

        // Wait for futures to complete
        while let Ok(Some(_)) = unord.next().timeout_at(stop_token.clone()).await {}

        Ok(())
    }

    // Ask our remaining peers to give us more peers before we go
    // back to the bootstrap servers to keep us from bothering them too much
    #[instrument(level = "trace", skip(self), err)]
    pub(super) async fn peer_minimum_refresh_task_routine(
        self,
        stop_token: StopToken,
    ) -> EyreResult<()> {
        let routing_table = self.routing_table();
        let cur_ts = intf::get_timestamp();

        // get list of all peers we know about, even the unreliable ones, and ask them to find nodes close to our node too
        let noderefs = routing_table.get_all_nodes(cur_ts);

        // do peer minimum search concurrently
        let mut unord = FuturesUnordered::new();
        for nr in noderefs {
            log_net!("--- peer minimum search with {:?}", nr);
            let routing_table = routing_table.clone();
            unord.push(async move { routing_table.reverse_find_node(nr, false).await });
        }
        while let Ok(Some(_)) = unord.next().timeout_at(stop_token.clone()).await {}

        Ok(())
    }

    // Keep relays assigned and accessible
    #[instrument(level = "trace", skip(self), err)]
    pub(super) async fn relay_management_task_routine(
        self,
        _stop_token: StopToken,
        _last_ts: u64,
        cur_ts: u64,
    ) -> EyreResult<()> {
        // Get our node's current node info and network class and do the right thing
        let routing_table = self.routing_table();
        let node_info = routing_table.get_own_node_info(RoutingDomain::PublicInternet);
        let network_class = self.get_network_class(RoutingDomain::PublicInternet);
        let mut node_info_changed = false;

        // Do we know our network class yet?
        if let Some(network_class) = network_class {
            // If we already have a relay, see if it is dead, or if we don't need it any more
            let has_relay = {
                if let Some(relay_node) = routing_table.relay_node(RoutingDomain::PublicInternet) {
                    let state = relay_node.state(cur_ts);
                    // Relay node is dead or no longer needed
                    if matches!(state, BucketEntryState::Dead) {
                        info!("Relay node died, dropping relay {}", relay_node);
                        routing_table.set_relay_node(RoutingDomain::PublicInternet, None);
                        node_info_changed = true;
                        false
                    } else if !node_info.requires_relay() {
                        info!(
                            "Relay node no longer required, dropping relay {}",
                            relay_node
                        );
                        routing_table.set_relay_node(RoutingDomain::PublicInternet, None);
                        node_info_changed = true;
                        false
                    } else {
                        true
                    }
                } else {
                    false
                }
            };

            // Do we need a relay?
            if !has_relay && node_info.requires_relay() {
                // Do we need an outbound relay?
                if network_class.outbound_wants_relay() {
                    // The outbound relay is the host of the PWA
                    if let Some(outbound_relay_peerinfo) = intf::get_outbound_relay_peer().await {
                        // Register new outbound relay
                        if let Some(nr) = routing_table.register_node_with_signed_node_info(
                            RoutingDomain::PublicInternet,
                            outbound_relay_peerinfo.node_id.key,
                            outbound_relay_peerinfo.signed_node_info,
                            false,
                        ) {
                            info!("Outbound relay node selected: {}", nr);
                            routing_table.set_relay_node(RoutingDomain::PublicInternet, Some(nr));
                            node_info_changed = true;
                        }
                    }
                // Otherwise we must need an inbound relay
                } else {
                    // Find a node in our routing table that is an acceptable inbound relay
                    if let Some(nr) =
                        routing_table.find_inbound_relay(RoutingDomain::PublicInternet, cur_ts)
                    {
                        info!("Inbound relay node selected: {}", nr);
                        routing_table.set_relay_node(RoutingDomain::PublicInternet, Some(nr));
                        node_info_changed = true;
                    }
                }
            }
        }

        // Re-send our node info if we selected a relay
        if node_info_changed {
            self.send_node_info_updates(RoutingDomain::PublicInternet, true)
                .await;
        }

        Ok(())
    }

    // Compute transfer statistics for the low level network
    #[instrument(level = "trace", skip(self), err)]
    pub(super) async fn rolling_transfers_task_routine(
        self,
        _stop_token: StopToken,
        last_ts: u64,
        cur_ts: u64,
    ) -> EyreResult<()> {
        // log_net!("--- network manager rolling_transfers task");
        {
            let inner = &mut *self.inner.lock();

            // Roll the low level network transfer stats for our address
            inner
                .stats
                .self_stats
                .transfer_stats_accounting
                .roll_transfers(last_ts, cur_ts, &mut inner.stats.self_stats.transfer_stats);

            // Roll all per-address transfers
            let mut dead_addrs: HashSet<PerAddressStatsKey> = HashSet::new();
            for (addr, stats) in &mut inner.stats.per_address_stats {
                stats.transfer_stats_accounting.roll_transfers(
                    last_ts,
                    cur_ts,
                    &mut stats.transfer_stats,
                );

                // While we're here, lets see if this address has timed out
                if cur_ts - stats.last_seen_ts >= IPADDR_MAX_INACTIVE_DURATION_US {
                    // it's dead, put it in the dead list
                    dead_addrs.insert(*addr);
                }
            }

            // Remove the dead addresses from our tables
            for da in &dead_addrs {
                inner.stats.per_address_stats.remove(da);
            }
        }

        // Send update
        self.send_network_update();

        Ok(())
    }

    // Clean up the public address check tables, removing entries that have timed out
    #[instrument(level = "trace", skip(self), err)]
    pub(super) async fn public_address_check_task_routine(
        self,
        stop_token: StopToken,
        _last_ts: u64,
        cur_ts: u64,
    ) -> EyreResult<()> {
        // go through public_address_inconsistencies_table and time out things that have expired
        let mut inner = self.inner.lock();
        for (_, pait_v) in &mut inner.public_address_inconsistencies_table {
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
}
