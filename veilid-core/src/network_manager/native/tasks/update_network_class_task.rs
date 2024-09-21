/// Detect NetworkClass and DialInfo for the DialInfo for the PublicInternet RoutingDomain
use super::*;
use futures_util::stream::FuturesUnordered;
use stop_token::future::FutureExt as StopTokenFutureExt;

impl Network {
    #[instrument(parent = None, level = "trace", skip(self), err)]
    pub async fn update_network_class_task_routine(
        self,
        stop_token: StopToken,
        l: Timestamp,
        t: Timestamp,
    ) -> EyreResult<()> {
        let _guard = self.unlocked_inner.network_task_lock.lock().await;

        // Do the public dial info check
        let out = self.do_public_dial_info_check(stop_token, l, t).await;

        // Done with public dial info check
        {
            let mut inner = self.inner.lock();
            inner.needs_public_dial_info_check = false;
            inner.public_dial_info_check_punishment = None;
        }

        out
    }

    #[instrument(level = "trace", skip(self), err)]
    pub async fn update_with_detected_dial_info(&self, ddi: DetectedDialInfo) -> EyreResult<()> {
        let existing_network_class = self
            .routing_table()
            .get_network_class(RoutingDomain::PublicInternet)
            .unwrap_or_default();

        match ddi {
            DetectedDialInfo::SymmetricNAT => {
                // If we get any symmetric nat dialinfo, this whole network class is outbound only,
                // and all dial info should be treated as invalid
                if !matches!(existing_network_class, NetworkClass::OutboundOnly) {
                    let mut editor = self.routing_table().edit_public_internet_routing_domain();

                    editor.clear_dial_info_details(None, None);
                    editor.set_network_class(Some(NetworkClass::OutboundOnly));
                    editor.commit(true).await;
                }
            }
            DetectedDialInfo::Detected(did) => {
                // get existing dial info into table by protocol/address type
                let mut existing_dial_info =
                    BTreeMap::<(ProtocolType, AddressType), DialInfoDetail>::new();
                for did in self.routing_table().all_filtered_dial_info_details(
                    RoutingDomain::PublicInternet.into(),
                    &DialInfoFilter::all(),
                ) {
                    // Only need to keep one per pt/at pair, since they will all have the same dialinfoclass
                    existing_dial_info.insert(
                        (did.dial_info.protocol_type(), did.dial_info.address_type()),
                        did,
                    );
                }
                // We got a dial info, upgrade everything unless we are fixed to outbound only due to a symmetric nat
                if !matches!(existing_network_class, NetworkClass::OutboundOnly) {
                    // Get existing dial info for protocol/address type combination
                    let pt = did.dial_info.protocol_type();
                    let at = did.dial_info.address_type();

                    // See what operations to perform with this dialinfo
                    let mut clear = false;
                    let mut add = false;

                    if let Some(edi) = existing_dial_info.get(&(pt, at)) {
                        // Is the dial info class better than our existing dial info?
                        // Or is the new dial info the same class, but different? Only change if things are different.
                        if did.class < edi.class
                            || (did.class == edi.class && did.dial_info != edi.dial_info)
                        {
                            // Better or same dial info class was found, clear existing dialinfo for this pt/at pair
                            // Only keep one dial info per protocol/address type combination
                            clear = true;
                            add = true;
                        }
                        // Otherwise, don't upgrade, don't add, this is worse than what we have already
                    } else {
                        // No existing dial info of this type accept it, no need to upgrade, but add it
                        add = true;
                    }

                    if clear || add {
                        let mut editor = self.routing_table().edit_public_internet_routing_domain();

                        if clear {
                            editor.clear_dial_info_details(
                                Some(did.dial_info.address_type()),
                                Some(did.dial_info.protocol_type()),
                            );
                        }

                        if add {
                            editor.add_dial_info(did.dial_info.clone(), did.class);
                        }

                        editor.set_network_class(Some(NetworkClass::InboundCapable));
                        editor.commit(true).await;
                    }
                }
            }
        }
        Ok(())
    }

    #[instrument(level = "trace", skip(self), err)]
    pub async fn do_public_dial_info_check(
        &self,
        stop_token: StopToken,
        _l: Timestamp,
        _t: Timestamp,
    ) -> EyreResult<()> {
        // Figure out if we can optimize TCP/WS checking since they are often on the same port
        let (protocol_config, inbound_protocol_map) = {
            let mut inner = self.inner.lock();
            let Some(protocol_config) = inner
                .network_state
                .as_ref()
                .map(|ns| ns.protocol_config.clone())
            else {
                bail!("should not be doing public dial info check before we have an initial network state");
            };

            // Allow network to be cleared if external addresses change
            inner.network_already_cleared = false;

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
        let existing_public_dial_info: HashSet<DialInfoDetail> = self
            .routing_table()
            .all_filtered_dial_info_details(
                RoutingDomain::PublicInternet.into(),
                &DialInfoFilter::all(),
            )
            .into_iter()
            .collect();

        // Set most permissive network config
        let mut editor = self.routing_table().edit_public_internet_routing_domain();
        editor.setup_network(
            protocol_config.outbound,
            protocol_config.inbound,
            protocol_config.family_global,
            protocol_config.public_internet_capabilities.clone(),
        );
        editor.commit(true).await;

        // Create a callback to clear the network if we need to 'start over'
        let this = self.clone();
        let clear_network_callback: ClearNetworkCallback = Arc::new(move || {
            let this = this.clone();
            Box::pin(async move {
                // Ensure we only do this once per network class discovery
                {
                    let mut inner = this.inner.lock();
                    if inner.network_already_cleared {
                        return;
                    }
                    inner.network_already_cleared = true;
                }
                let mut editor = this.routing_table().edit_public_internet_routing_domain();
                editor.clear_dial_info_details(None, None);
                editor.set_network_class(None);
                editor.commit(true).await;
            })
        });

        // Process all protocol and address combinations
        let mut unord = FuturesUnordered::new();

        for ((at, _llpt, port), protocols) in &inbound_protocol_map {
            let first_pt = protocols.first().unwrap();

            let discovery_context = DiscoveryContext::new(
                self.routing_table(),
                self.clone(),
                *first_pt,
                *at,
                *port,
                clear_network_callback.clone(),
            );
            discovery_context.discover(&mut unord).await;
        }

        // Wait for all discovery futures to complete and apply discoverycontexts
        let mut all_address_types = AddressTypeSet::new();
        loop {
            match unord
                .next()
                .timeout_at(stop_token.clone())
                .in_current_span()
                .await
            {
                Ok(Some(Some(dr))) => {
                    // Found some new dial info for this protocol/address combination
                    self.update_with_detected_dial_info(dr.ddi.clone()).await?;

                    // Add the external address kinds to the set we've seen
                    all_address_types |= dr.external_address_types;

                    // Add additional dialinfo for protocols on the same port
                    if let DetectedDialInfo::Detected(did) = &dr.ddi {
                        let ipmkey = (
                            did.dial_info.address_type(),
                            did.dial_info.protocol_type().low_level_protocol_type(),
                            dr.local_port,
                        );
                        if let Some(ipm) = inbound_protocol_map.get(&ipmkey) {
                            for additional_pt in ipm.iter().skip(1) {
                                // Make dialinfo for additional protocol type
                                let additional_ddi = DetectedDialInfo::Detected(DialInfoDetail {
                                    dial_info: self.make_dial_info(
                                        did.dial_info.socket_address(),
                                        *additional_pt,
                                    ),
                                    class: did.class,
                                });
                                // Add additional dialinfo
                                self.update_with_detected_dial_info(additional_ddi).await?;
                            }
                        }
                    }
                }
                Ok(Some(None)) => {
                    // Found no new dial info for this protocol/address combination
                }
                Ok(None) => {
                    // All done, normally
                    break;
                }
                Err(_) => {
                    // Stop token, exit early without error propagation
                    return Ok(());
                }
            }
        }

        // All done

        log_net!(debug "Network class discovery finished with address_types {:?}", all_address_types);

        // Set the address types we've seen
        editor.setup_network(
            protocol_config.outbound,
            protocol_config.inbound,
            all_address_types,
            protocol_config.public_internet_capabilities,
        );
        if editor.commit(true).await {
            editor.publish();
        }

        // See if the dial info changed
        let new_public_dial_info: HashSet<DialInfoDetail> = self
            .routing_table()
            .all_filtered_dial_info_details(
                RoutingDomain::PublicInternet.into(),
                &DialInfoFilter::all(),
            )
            .into_iter()
            .collect();

        // Punish nodes that told us our public address had changed when it didn't
        if new_public_dial_info == existing_public_dial_info {
            if let Some(punish) = self.inner.lock().public_dial_info_check_punishment.take() {
                punish();
            }
        }

        Ok(())
    }

    /// Make a dialinfo from an address and protocol type
    pub fn make_dial_info(&self, addr: SocketAddress, protocol_type: ProtocolType) -> DialInfo {
        match protocol_type {
            ProtocolType::UDP => DialInfo::udp(addr),
            ProtocolType::TCP => DialInfo::tcp(addr),
            ProtocolType::WS => {
                let c = self.config.get();
                DialInfo::try_ws(
                    addr,
                    format!("ws://{}/{}", addr, c.network.protocol.ws.path),
                )
                .unwrap()
            }
            ProtocolType::WSS => {
                let c = self.config.get();
                DialInfo::try_wss(
                    addr,
                    format!("wss://{}/{}", addr, c.network.protocol.wss.path),
                )
                .unwrap()
            }
        }
    }
}
