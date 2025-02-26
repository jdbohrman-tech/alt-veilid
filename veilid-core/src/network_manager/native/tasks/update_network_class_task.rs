/// Detect NetworkClass and DialInfo for the DialInfo for the PublicInternet RoutingDomain
use super::*;
use futures_util::stream::FuturesUnordered;
use stop_token::future::FutureExt as StopTokenFutureExt;

type InboundProtocolMap = HashMap<(AddressType, LowLevelProtocolType, u16), Vec<ProtocolType>>;

impl Network {
    #[instrument(parent = None, level = "trace", skip(self), err)]
    pub async fn update_network_class_task_routine(
        &self,
        stop_token: StopToken,
        l: Timestamp,
        t: Timestamp,
    ) -> EyreResult<()> {
        // Network lock ensures only one task operating on the low level network state
        // can happen at the same time. This a blocking lock so we can ensure this runs
        // as soon as network_interfaces_task is finished
        let _guard = self.network_task_lock.lock().await;

        // Do the public dial info check
        let finished = self.do_public_dial_info_check(stop_token, l, t).await?;

        // Done with public dial info check
        if finished {
            let mut inner = self.inner.lock();

            // Don't try to re-do OutboundOnly dialinfo for another 10 seconds
            inner.next_outbound_only_dial_info_check = Timestamp::now()
                + TimestampDuration::new_secs(UPDATE_OUTBOUND_ONLY_NETWORK_CLASS_PERIOD_SECS)
        }

        Ok(())
    }

    #[instrument(level = "trace", skip(self, editor))]
    fn process_detected_dial_info(
        &self,
        editor: &mut RoutingDomainEditorPublicInternet,
        ddi: DetectedDialInfo,
    ) {
        match ddi {
            DetectedDialInfo::SymmetricNAT => {}
            DetectedDialInfo::Detected(did) => {
                // We got a dialinfo, add it and tag us as inbound capable
                editor.add_dial_info(did.dial_info.clone(), did.class);
            }
        }
    }

    #[instrument(level = "trace", skip(self, editor))]
    fn update_with_detection_result(
        &self,
        editor: &mut RoutingDomainEditorPublicInternet,
        inbound_protocol_map: &InboundProtocolMap,
        dr: DetectionResult,
    ) {
        // Found some new dial info for this protocol/address combination
        self.process_detected_dial_info(editor, dr.ddi.clone());

        // Add additional dialinfo for protocols on the same port
        match &dr.ddi {
            DetectedDialInfo::SymmetricNAT => {}
            DetectedDialInfo::Detected(did) => {
                let ipmkey = (
                    did.dial_info.address_type(),
                    did.dial_info.protocol_type().low_level_protocol_type(),
                    dr.config.port,
                );
                if let Some(ipm) = inbound_protocol_map.get(&ipmkey) {
                    for additional_pt in ipm.iter().skip(1) {
                        // Make dialinfo for additional protocol type
                        let additional_ddi = DetectedDialInfo::Detected(DialInfoDetail {
                            dial_info: self
                                .make_dial_info(did.dial_info.socket_address(), *additional_pt),
                            class: did.class,
                        });
                        // Add additional dialinfo
                        self.process_detected_dial_info(editor, additional_ddi);
                    }
                }
            }
        }
    }

    #[instrument(level = "trace", skip(self), err)]
    pub async fn do_public_dial_info_check(
        &self,
        stop_token: StopToken,
        _l: Timestamp,
        cur_ts: Timestamp,
    ) -> EyreResult<bool> {
        // Figure out if we can optimize TCP/WS checking since they are often on the same port
        let (protocol_config, inbound_protocol_map) = {
            let inner = self.inner.lock();
            let Some(protocol_config) = inner
                .network_state
                .as_ref()
                .map(|ns| ns.protocol_config.clone())
            else {
                bail!("should not be doing public dial info check before we have an initial network state");
            };

            let mut inbound_protocol_map =
                HashMap::<(AddressType, LowLevelProtocolType, u16), Vec<ProtocolType>>::new();
            for at in protocol_config.family_global {
                for pt in protocol_config.inbound {
                    let key = (pt, at);

                    // Skip things with static public dialinfo
                    // as they don't need to participate in discovery
                    if inner.static_public_dial_info.contains(pt) {
                        continue;
                    }

                    if let Some(pla) = inner.preferred_local_addresses.get(&key) {
                        let llpt = pt.low_level_protocol_type();
                        let itmkey = (at, llpt, pla.port());
                        inbound_protocol_map
                            .entry(itmkey)
                            .and_modify(|x| x.push(pt))
                            .or_insert_with(|| vec![pt]);
                    }
                }
            }

            (protocol_config, inbound_protocol_map)
        };

        // Save off existing public dial info for change detection later
        let routing_table = self.routing_table();

        // Set most permissive network config and start from scratch
        let mut editor = routing_table.edit_public_internet_routing_domain();
        editor.setup_network(
            protocol_config.outbound,
            protocol_config.inbound,
            protocol_config.family_global,
            protocol_config.public_internet_capabilities.clone(),
            false,
        );
        editor.clear_dial_info_details(None, None);
        editor.commit(true).await;

        // Process all protocol and address combinations
        let mut unord = FuturesUnordered::new();
        let mut context_configs = HashSet::new();
        for ((address_type, _llpt, port), protocols) in inbound_protocol_map.clone() {
            let protocol_type = *protocols.first().unwrap();
            let dcc = DiscoveryContextConfig {
                protocol_type,
                address_type,
                port,
            };
            context_configs.insert(dcc);
            let discovery_context = DiscoveryContext::new(self.registry(), dcc, stop_token.clone());
            unord.push(discovery_context.discover());
        }

        // Wait for all discovery futures to complete and apply discoverycontexts
        let mut external_address_types = AddressTypeSet::new();
        let mut detection_results = HashMap::<DiscoveryContextConfig, DetectionResult>::new();
        loop {
            match unord
                .next()
                .timeout_at(stop_token.clone())
                .in_current_span()
                .await
            {
                Ok(Some(Some(dr))) => {
                    // Got something for this config
                    context_configs.remove(&dr.config);

                    // Add the external address kinds to the set we've seen
                    external_address_types |= dr.external_address_types;

                    // Save best detection result for each discovery context config
                    detection_results.insert(dr.config, dr);
                }
                Ok(Some(None)) => {
                    // Found no dial info for this protocol/address combination
                }
                Ok(None) => {
                    // All done, normally
                    break;
                }
                Err(_) => {
                    // Stop token, exit early without error propagation
                    return Ok(true);
                }
            }
        }

        // Apply best effort coalesced detection results
        for (_, dr) in detection_results {
            // Import the dialinfo
            self.update_with_detection_result(&mut editor, &inbound_protocol_map, dr);
        }

        let end_ts = Timestamp::now();

        // If we got no external address types, try again
        if external_address_types.is_empty() {
            veilid_log!(self debug "Network class discovery failed in {}, trying again, got no external address types", end_ts - cur_ts);
            return Ok(false);
        }

        // See if we have any discovery contexts that did not complete for a
        // particular protocol type if its external address type was supported.
        let mut success = true;
        for cc in &context_configs {
            if external_address_types.contains(cc.address_type) {
                success = false;
                break;
            }
        }

        if !success {
            veilid_log!(self debug "Network class discovery failed in {}, trying again, needed {:?}", end_ts - cur_ts, context_configs);
            return Ok(false);
        }

        // All done
        veilid_log!(self debug "Network class discovery finished in {} with address_types {:?}", end_ts - cur_ts, external_address_types);

        // Set the address types we've seen and confirm the network class
        editor.setup_network(
            protocol_config.outbound,
            protocol_config.inbound,
            external_address_types,
            protocol_config.public_internet_capabilities,
            true,
        );
        if editor.commit(true).await {
            editor.publish();
        }

        // Say we no longer need an update
        self.inner.lock().needs_update_network_class = false;

        Ok(true)
    }

    /// Make a dialinfo from an address and protocol type
    pub fn make_dial_info(&self, addr: SocketAddress, protocol_type: ProtocolType) -> DialInfo {
        match protocol_type {
            ProtocolType::UDP => DialInfo::udp(addr),
            ProtocolType::TCP => DialInfo::tcp(addr),
            ProtocolType::WS => DialInfo::try_ws(
                addr,
                self.config()
                    .with(|c| format!("ws://{}/{}", addr, c.network.protocol.ws.path)),
            )
            .unwrap(),
            ProtocolType::WSS => DialInfo::try_wss(
                addr,
                self.config()
                    .with(|c| format!("wss://{}/{}", addr, c.network.protocol.wss.path)),
            )
            .unwrap(),
        }
    }
}
