use super::*;

use futures_util::stream::{FuturesUnordered, StreamExt};
use stop_token::future::FutureExt as StopFutureExt;

pub const BOOTSTRAP_TXT_VERSION_0: u8 = 0;

#[derive(Clone, Debug)]
pub struct BootstrapRecord {
    node_ids: TypedKeyGroup,
    envelope_support: Vec<u8>,
    dial_info_details: Vec<DialInfoDetail>,
}
impl BootstrapRecord {
    pub fn merge(&mut self, other: BootstrapRecord) {
        self.node_ids.add_all(&other.node_ids);
        for x in other.envelope_support {
            if !self.envelope_support.contains(&x) {
                self.envelope_support.push(x);
                self.envelope_support.sort();
            }
        }
        for did in other.dial_info_details {
            if !self.dial_info_details.contains(&did) {
                self.dial_info_details.push(did);
            }
        }
    }
}

impl RoutingTable {
    /// Process bootstrap version 0
    async fn process_bootstrap_records_v0(
        &self,
        records: Vec<String>,
    ) -> EyreResult<Option<BootstrapRecord>> {
        // Bootstrap TXT Record Format Version 0:
        // txt_version|envelope_support|node_ids|hostname|dialinfoshort*
        //
        // Split bootstrap node record by '|' and then lists by ','. Example:
        // 0|0|VLD0:7lxDEabK_qgjbe38RtBa3IZLrud84P6NhGP-pRTZzdQ|bootstrap-1.dev.veilid.net|T5150,U5150,W5150/ws

        if records.len() != 5 {
            bail!("invalid number of fields in bootstrap v0 txt record");
        }

        // Envelope support
        let mut envelope_support = Vec::new();
        for ess in records[1].split(',') {
            let ess = ess.trim();
            let es = match ess.parse::<u8>() {
                Ok(v) => v,
                Err(e) => {
                    bail!(
                        "invalid envelope version specified in bootstrap node txt record: {}",
                        e
                    );
                }
            };
            envelope_support.push(es);
        }
        envelope_support.dedup();
        envelope_support.sort();

        // Node Id
        let mut node_ids = TypedKeyGroup::new();
        for node_id_str in records[2].split(',') {
            let node_id_str = node_id_str.trim();
            let node_id = match TypedKey::from_str(node_id_str) {
                Ok(v) => v,
                Err(e) => {
                    bail!(
                        "Invalid node id in bootstrap node record {}: {}",
                        node_id_str,
                        e
                    );
                }
            };
            node_ids.add(node_id);
        }

        // If this is our own node id, then we skip it for bootstrap, in case we are a bootstrap node
        if self.unlocked_inner.matches_own_node_id(&node_ids) {
            return Ok(None);
        }

        // Hostname
        let hostname_str = records[3].trim();

        // Resolve each record and store in node dial infos list
        let mut dial_info_details = Vec::new();
        for rec in records[4].split(',') {
            let rec = rec.trim();
            let dial_infos = match DialInfo::try_vec_from_short(rec, hostname_str) {
                Ok(dis) => dis,
                Err(e) => {
                    warn!("Couldn't resolve bootstrap node dial info {}: {}", rec, e);
                    continue;
                }
            };

            for di in dial_infos {
                dial_info_details.push(DialInfoDetail {
                    dial_info: di,
                    class: DialInfoClass::Direct,
                });
            }
        }

        Ok(Some(BootstrapRecord {
            node_ids,
            envelope_support,
            dial_info_details,
        }))
    }

    // Bootstrap lookup process
    #[instrument(level = "trace", skip(self), ret, err)]
    pub(crate) async fn resolve_bootstrap(
        &self,
        bootstrap: Vec<String>,
    ) -> EyreResult<Vec<BootstrapRecord>> {
        // Resolve from bootstrap root to bootstrap hostnames
        let mut bsnames = Vec::<String>::new();
        for bh in bootstrap {
            // Get TXT record for bootstrap (bootstrap.veilid.net, or similar)
            let records = match intf::txt_lookup(&bh).await {
                Ok(v) => v,
                Err(e) => {
                    warn!(
                        "Network may be down. No bootstrap resolution for '{}': {}",
                        bh, e
                    );
                    continue;
                }
            };
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
                    // look up bootstrap node txt records
                    let bsnirecords = match intf::txt_lookup(&bsname).await {
                        Err(e) => {
                            warn!(
                                "Network may be down. Bootstrap node txt lookup failed for {}: {}",
                                bsname, e
                            );
                            return None;
                        }
                        Ok(v) => v,
                    };
                    // for each record resolve into key/bootstraprecord pairs
                    let mut bootstrap_records: Vec<BootstrapRecord> = Vec::new();
                    for bsnirecord in bsnirecords {
                        // All formats split on '|' character
                        let records: Vec<String> = bsnirecord
                            .trim()
                            .split('|')
                            .map(|x| x.trim().to_owned())
                            .collect();

                        // Bootstrap TXT record version
                        let txt_version: u8 = match records[0].parse::<u8>() {
                            Ok(v) => v,
                            Err(e) => {
                                log_rtab!(warn
                                "invalid txt_version specified in bootstrap node txt record: {}",
                                e
                            );
                                continue;
                            }
                        };
                        let bootstrap_record = match txt_version {
                            BOOTSTRAP_TXT_VERSION_0 => {
                                match self.process_bootstrap_records_v0(records).await {
                                    Err(e) => {
                                        log_rtab!(error
                                            "couldn't process v0 bootstrap records from {}: {}",
                                            bsname, e
                                        );
                                        continue;
                                    }
                                    Ok(Some(v)) => v,
                                    Ok(None) => {
                                        // skipping
                                        continue;
                                    }
                                }
                            }
                            _ => {
                                log_rtab!(warn "unsupported bootstrap txt record version");
                                continue;
                            }
                        };

                        bootstrap_records.push(bootstrap_record);
                    }
                    Some(bootstrap_records)
                }
                .instrument(Span::current()),
            );
        }

        let mut merged_bootstrap_records: Vec<BootstrapRecord> = Vec::new();
        while let Some(bootstrap_records) = unord.next().await {
            let Some(bootstrap_records) = bootstrap_records else {
                continue;
            };
            for mut bsrec in bootstrap_records {
                let mut mbi = 0;
                while mbi < merged_bootstrap_records.len() {
                    let mbr = &mut merged_bootstrap_records[mbi];
                    if mbr.node_ids.contains_any(&bsrec.node_ids) {
                        // Merge record, pop this one out
                        let mbr = merged_bootstrap_records.remove(mbi);
                        bsrec.merge(mbr);
                    } else {
                        // No overlap, go to next record
                        mbi += 1;
                    }
                }
                // Append merged record
                merged_bootstrap_records.push(bsrec);
            }
        }

        // ensure dial infos are sorted
        for mbr in &mut merged_bootstrap_records {
            mbr.dial_info_details.sort();
        }

        Ok(merged_bootstrap_records)
    }

    //#[instrument(level = "trace", skip(self), err)]
    pub(crate) fn bootstrap_with_peer(self, crypto_kinds: Vec<CryptoKind>, pi: PeerInfo, unord: &FuturesUnordered<SendPinBoxFuture<()>>) {

        log_rtab!(
            "--- bootstrapping {} with {:?}",
            pi.node_ids(),
            pi.signed_node_info().node_info().dial_info_detail_list()
        );

        let nr =
        match self.register_node_with_peer_info(RoutingDomain::PublicInternet, pi, true) {
            Ok(nr) => nr,
            Err(e) => {
                log_rtab!(error "failed to register bootstrap peer info: {}", e);
                return;
            }
        };
        
        // Add this our futures to process in parallel
        for crypto_kind in crypto_kinds {

            // Bootstrap this crypto kind
            let nr = nr.clone();
            let routing_table = self.clone();
            unord.push(Box::pin(
                async move {
                    // Get what contact method would be used for contacting the bootstrap
                    let bsdi = match routing_table
                        .network_manager()
                        .get_node_contact_method(nr.clone())
                    {
                        Ok(NodeContactMethod::Direct(v)) => v,
                        Ok(v) => {
                            log_rtab!(warn "invalid contact method for bootstrap, restarting network: {:?}", v);
                            routing_table.network_manager().restart_network();
                            return;
                        }
                        Err(e) => {
                            log_rtab!(warn "unable to bootstrap: {}", e);
                            return;
                        }
                    };

                    // Need VALID signed peer info, so ask bootstrap to find_node of itself
                    // which will ensure it has the bootstrap's signed peer info as part of the response
                    let _ = routing_table.find_target(crypto_kind, nr.clone()).await;

                    // Ensure we got the signed peer info
                    if !nr.signed_node_info_has_valid_signature(RoutingDomain::PublicInternet) {
                        log_rtab!(warn "bootstrap server is not responding");
                        log_rtab!(debug "bootstrap server is not responding for dialinfo: {}", bsdi);
                        
                        // Try a different dialinfo next time
                        routing_table.network_manager().address_filter().set_dial_info_failed(bsdi);
                    } else {
                        // otherwise this bootstrap is valid, lets ask it to find ourselves now
                        routing_table.reverse_find_node(crypto_kind, nr, true).await
                    }
                }
                .instrument(Span::current()),
            ));
        }
    }

    #[instrument(level = "trace", skip(self), err)]
    pub(crate) async fn bootstrap_with_peer_list(self, peers: Vec<PeerInfo>, stop_token: StopToken) -> EyreResult<()> {

        log_rtab!(debug "  bootstrapped peers: {:?}", &peers);

        // Get crypto kinds to bootstrap
        let crypto_kinds = self.get_bootstrap_crypto_kinds();

        log_rtab!(debug "  bootstrapped crypto kinds: {:?}", &crypto_kinds);

        // Run all bootstrap operations concurrently
        let mut unord = FuturesUnordered::<SendPinBoxFuture<()>>::new();
        for peer in peers {
            self.clone().bootstrap_with_peer(crypto_kinds.clone(), peer, &unord);
        }

        // Wait for all bootstrap operations to complete before we complete the singlefuture
        while let Ok(Some(_)) = unord.next().timeout_at(stop_token.clone()).await {}
        Ok(())
    }

    // Get counts by crypto kind and figure out which crypto kinds need bootstrapping
    fn get_bootstrap_crypto_kinds(&self) -> Vec<CryptoKind> {
        let entry_count = self.inner.read().cached_entry_counts();
        let mut crypto_kinds = Vec::new();
        for crypto_kind in VALID_CRYPTO_KINDS {
            // Do we need to bootstrap this crypto kind?
            let eckey = (RoutingDomain::PublicInternet, crypto_kind);
            let cnt = entry_count.get(&eckey).copied().unwrap_or_default();
            if cnt == 0 {
                crypto_kinds.push(crypto_kind);
            }
        }
        crypto_kinds
    }


    #[instrument(level = "trace", skip(self), err)]
    pub(crate) async fn bootstrap_task_routine(self, stop_token: StopToken) -> EyreResult<()> {
        let bootstrap = self
            .unlocked_inner
            .with_config(|c| c.network.routing_table.bootstrap.clone());

        // Don't bother if bootstraps aren't configured
        if bootstrap.is_empty() {
            return Ok(());
        }

        log_rtab!(debug "--- bootstrap_task");

        // See if we are specifying a direct dialinfo for bootstrap, if so use the direct mechanism
        let mut bootstrap_dialinfos = Vec::<DialInfo>::new();
        for b in &bootstrap {
            if let Ok(bootstrap_di_vec) = DialInfo::try_vec_from_url(b) {
                for bootstrap_di in bootstrap_di_vec {
                    bootstrap_dialinfos.push(bootstrap_di);
                }
            }
        }
        
        // Get a peer list from bootstrap to process
        let peers = if !bootstrap_dialinfos.is_empty() {
            // Direct bootstrap
            let network_manager = self.network_manager();

            let mut peer_map = HashMap::<TypedKeyGroup, PeerInfo>::new();
            for bootstrap_di in bootstrap_dialinfos {
                log_rtab!(debug "direct bootstrap with: {}", bootstrap_di);
                let peers = network_manager.boot_request(bootstrap_di).await?;
                for peer in peers {
                    if !peer_map.contains_key(peer.node_ids()) {
                        peer_map.insert(peer.node_ids().clone(), peer);
                    }
                }
            }
            peer_map.into_values().collect()
        } else {
            // If not direct, resolve bootstrap servers and recurse their TXT entries
            let bsrecs = self.resolve_bootstrap(bootstrap).await?;
            let peers : Vec<PeerInfo> = bsrecs.into_iter().map(|bsrec| {
                // Get crypto support from list of node ids
                let crypto_support = bsrec.node_ids.kinds();

                // Make unsigned SignedNodeInfo
                let sni =
                    SignedNodeInfo::Direct(SignedDirectNodeInfo::with_no_signature(NodeInfo::new(
                        NetworkClass::InboundCapable, // Bootstraps are always inbound capable
                        ProtocolTypeSet::only(ProtocolType::UDP), // Bootstraps do not participate in relaying and will not make outbound requests, but will have UDP enabled
                        AddressTypeSet::all(), // Bootstraps are always IPV4 and IPV6 capable
                        bsrec.envelope_support, // Envelope support is as specified in the bootstrap list
                        crypto_support,         // Crypto support is derived from list of node ids
                        vec![],                 // Bootstrap needs no capabilities
                        bsrec.dial_info_details, // Dial info is as specified in the bootstrap list
                    )));

                PeerInfo::new(bsrec.node_ids, sni)
            }).collect();

            peers
        };

        self.clone().bootstrap_with_peer_list(peers, stop_token).await        
    }
}
