use super::*;

use futures_util::stream::{FuturesUnordered, StreamExt};
use stop_token::future::FutureExt as StopFutureExt;

pub const BOOTSTRAP_TXT_VERSION: u8 = 0;

#[derive(Clone, Debug)]
pub struct BootstrapRecord {
    min_version: u8,
    max_version: u8,
    dial_info_details: Vec<DialInfoDetail>,
}
pub type BootstrapRecordMap = BTreeMap<DHTKey, BootstrapRecord>;

impl RoutingTable {
    // Bootstrap lookup process
    #[instrument(level = "trace", skip(self), ret, err)]
    pub(crate) async fn resolve_bootstrap(
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
            unord.push(
                async move {
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
                        if self.node_id() == node_id_key {
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
                                    warn!(
                                        "Couldn't resolve bootstrap node dial info {}: {}",
                                        rec, e
                                    );
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
                }
                .instrument(Span::current()),
            );
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
    #[instrument(level = "trace", skip(self), err)]
    pub(crate) async fn direct_bootstrap_task_routine(
        self,
        stop_token: StopToken,
        bootstrap_dialinfos: Vec<DialInfo>,
    ) -> EyreResult<()> {
        let mut unord = FuturesUnordered::new();
        let network_manager = self.network_manager();

        for bootstrap_di in bootstrap_dialinfos {
            log_rtab!(debug "direct bootstrap with: {}", bootstrap_di);
            let peer_info = network_manager.boot_request(bootstrap_di).await?;

            log_rtab!(debug "  direct bootstrap peerinfo: {:?}", peer_info);

            // Got peer info, let's add it to the routing table
            for pi in peer_info {
                let k = pi.node_id.key;
                // Register the node
                if let Some(nr) = self.register_node_with_signed_node_info(
                    RoutingDomain::PublicInternet,
                    k,
                    pi.signed_node_info,
                    false,
                ) {
                    // Add this our futures to process in parallel
                    let routing_table = self.clone();
                    unord.push(
                        // lets ask bootstrap to find ourselves now
                        async move { routing_table.reverse_find_node(nr, true).await }
                            .instrument(Span::current()),
                    );
                }
            }
        }

        // Wait for all bootstrap operations to complete before we complete the singlefuture
        while let Ok(Some(_)) = unord.next().timeout_at(stop_token.clone()).await {}

        Ok(())
    }

    #[instrument(level = "trace", skip(self), err)]
    pub(crate) async fn bootstrap_task_routine(self, stop_token: StopToken) -> EyreResult<()> {
        let (bootstrap, bootstrap_nodes) = self.with_config(|c| {
            (
                c.network.bootstrap.clone(),
                c.network.bootstrap_nodes.clone(),
            )
        });

        log_rtab!(debug "--- bootstrap_task");

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
                let (id_str, di_str) = b
                    .split_once('@')
                    .ok_or_else(|| eyre!("Invalid node dial info in bootstrap entry"))?;
                let node_id =
                    NodeId::from_str(id_str).wrap_err("Invalid node id in bootstrap entry")?;
                let dial_info =
                    DialInfo::from_str(di_str).wrap_err("Invalid dial info in bootstrap entry")?;
                bootstrap_node_dial_infos.push((node_id, dial_info));
            }
            for (node_id, dial_info) in bootstrap_node_dial_infos {
                bsmap
                    .entry(node_id.key)
                    .or_insert_with(|| BootstrapRecord {
                        min_version: MIN_CRYPTO_VERSION,
                        max_version: MAX_CRYPTO_VERSION,
                        dial_info_details: Vec::new(),
                    })
                    .dial_info_details
                    .push(DialInfoDetail {
                        dial_info,
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

            log_rtab!("--- bootstrapping {} with {:?}", k.encode(), &v);

            // Make invalid signed node info (no signature)
            if let Some(nr) = self.register_node_with_signed_node_info(
                RoutingDomain::PublicInternet,
                k,
                SignedNodeInfo::Direct(SignedDirectNodeInfo::with_no_signature(NodeInfo {
                    network_class: NetworkClass::InboundCapable, // Bootstraps are always inbound capable
                    outbound_protocols: ProtocolTypeSet::only(ProtocolType::UDP), // Bootstraps do not participate in relaying and will not make outbound requests, but will have UDP enabled
                    address_types: AddressTypeSet::all(), // Bootstraps are always IPV4 and IPV6 capable
                    min_version: v.min_version, // Minimum crypto version specified in txt record
                    max_version: v.max_version, // Maximum crypto version specified in txt record
                    dial_info_detail_list: v.dial_info_details, // Dial info is as specified in the bootstrap list
                })),
                true,
            ) {
                // Add this our futures to process in parallel
                let routing_table = self.clone();
                unord.push(
                    async move {
                        // Need VALID signed peer info, so ask bootstrap to find_node of itself
                        // which will ensure it has the bootstrap's signed peer info as part of the response
                        let _ = routing_table.find_target(nr.clone()).await;

                        // Ensure we got the signed peer info
                        if !nr.signed_node_info_has_valid_signature(RoutingDomain::PublicInternet) {
                            log_rtab!(warn
                                "bootstrap at {:?} did not return valid signed node info",
                                nr
                            );
                            // If this node info is invalid, it will time out after being unpingable
                        } else {
                            // otherwise this bootstrap is valid, lets ask it to find ourselves now
                            routing_table.reverse_find_node(nr, true).await
                        }
                    }
                    .instrument(Span::current()),
                );
            }
        }

        // Wait for all bootstrap operations to complete before we complete the singlefuture
        while let Ok(Some(_)) = unord.next().timeout_at(stop_token.clone()).await {}
        Ok(())
    }
}
