use super::*;

/// Keepalive pings are done occasionally to ensure holepunched public dialinfo
/// remains valid, as well as to make sure we remain in any relay node's routing table
const RELAY_KEEPALIVE_PING_INTERVAL_SECS: u32 = 10;

/// Keepalive pings are done for active watch nodes to make sure they are still there
const ACTIVE_WATCH_KEEPALIVE_PING_INTERVAL_SECS: u32 = 10;

/// Ping queue processing depth per validator
const MAX_PARALLEL_PINGS: usize = 8;

use futures_util::FutureExt;

type PingValidatorFuture = SendPinBoxFuture<Result<(), RPCError>>;

impl RoutingTable {
    // Task routine for PublicInternet status pings
    #[instrument(level = "trace", skip(self), err)]
    pub async fn ping_validator_public_internet_task_routine(
        &self,
        stop_token: StopToken,
        _last_ts: Timestamp,
        cur_ts: Timestamp,
    ) -> EyreResult<()> {
        let mut future_queue: VecDeque<PingValidatorFuture> = VecDeque::new();

        self.ping_validator_public_internet(cur_ts, &mut future_queue)
            .await?;

        self.process_ping_validation_queue("PublicInternet", stop_token, cur_ts, future_queue)
            .await;

        Ok(())
    }

    // Task routine for LocalNetwork status pings
    #[instrument(level = "trace", skip(self), err)]
    pub async fn ping_validator_local_network_task_routine(
        &self,
        stop_token: StopToken,
        _last_ts: Timestamp,
        cur_ts: Timestamp,
    ) -> EyreResult<()> {
        let mut future_queue: VecDeque<PingValidatorFuture> = VecDeque::new();

        self.ping_validator_local_network(cur_ts, &mut future_queue)
            .await?;

        self.process_ping_validation_queue("LocalNetwork", stop_token, cur_ts, future_queue)
            .await;

        Ok(())
    }

    // Task routine for PublicInternet relay keepalive pings
    #[instrument(level = "trace", skip(self), err)]
    pub async fn ping_validator_public_internet_relay_task_routine(
        &self,
        stop_token: StopToken,
        _last_ts: Timestamp,
        cur_ts: Timestamp,
    ) -> EyreResult<()> {
        let mut future_queue: VecDeque<PingValidatorFuture> = VecDeque::new();

        self.relay_keepalive_public_internet(cur_ts, &mut future_queue)
            .await?;

        self.process_ping_validation_queue("RelayKeepalive", stop_token, cur_ts, future_queue)
            .await;

        Ok(())
    }

    // Task routine for active watch keepalive pings
    #[instrument(level = "trace", skip(self), err)]
    pub async fn ping_validator_active_watch_task_routine(
        &self,
        stop_token: StopToken,
        _last_ts: Timestamp,
        cur_ts: Timestamp,
    ) -> EyreResult<()> {
        let mut future_queue: VecDeque<PingValidatorFuture> = VecDeque::new();

        self.active_watches_keepalive_public_internet(cur_ts, &mut future_queue)
            .await?;

        self.process_ping_validation_queue("WatchKeepalive", stop_token, cur_ts, future_queue)
            .await;

        Ok(())
    }

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

    // Ping the relay to keep it alive, over every protocol it is relaying for us
    #[instrument(level = "trace", skip(self, futurequeue), err)]
    async fn relay_keepalive_public_internet(
        &self,
        cur_ts: Timestamp,
        futurequeue: &mut VecDeque<PingValidatorFuture>,
    ) -> EyreResult<()> {
        // Get the PublicInternet relay if we are using one
        let Some(relay_nr) = self.relay_node(RoutingDomain::PublicInternet) else {
            return Ok(());
        };

        // Get our publicinternet dial info
        let dids = self.all_filtered_dial_info_details(
            RoutingDomain::PublicInternet.into(),
            &DialInfoFilter::all(),
        );

        let opt_relay_keepalive_ts = self.relay_node_last_keepalive(RoutingDomain::PublicInternet);
        let relay_needs_keepalive = opt_relay_keepalive_ts
            .map(|kts| {
                cur_ts.saturating_sub(kts).as_u64()
                    >= (RELAY_KEEPALIVE_PING_INTERVAL_SECS as u64 * 1_000_000u64)
            })
            .unwrap_or(true);

        if !relay_needs_keepalive {
            return Ok(());
        }
        // Say we're doing this keepalive now
        self.inner
            .write()
            .set_relay_node_last_keepalive(RoutingDomain::PublicInternet, cur_ts);

        // We need to keep-alive at one connection per ordering for relays
        // but also one per NAT mapping that we need to keep open for our inbound dial info
        let mut got_unordered = false;
        let mut got_ordered = false;

        // Look up any NAT mappings we may need to try to preserve with keepalives
        let mut mapped_port_info = self.get_low_level_port_info();

        // Relay nodes get pinged over all protocols we have inbound dialinfo for
        // This is so we can preserve the inbound NAT mappings at our router
        let mut relay_noderefs = vec![];
        for did in &dids {
            // Can skip the ones that are direct, those are not mapped or natted
            // because we can have both direct and natted dialinfo on the same
            // node, for example ipv4 can be natted, while ipv6 is direct
            if did.class == DialInfoClass::Direct {
                continue;
            }
            // Do we need to do this ping?
            // Check if we have already pinged over this low-level-protocol/address-type/port combo
            // We want to ensure we do the bare minimum required here
            let pt = did.dial_info.protocol_type();
            let at = did.dial_info.address_type();
            let needs_ping_for_protocol =
                if let Some((llpt, port)) = mapped_port_info.protocol_to_port.get(&(pt, at)) {
                    mapped_port_info
                        .low_level_protocol_ports
                        .remove(&(*llpt, at, *port))
                } else {
                    false
                };
            if needs_ping_for_protocol {
                if pt.is_ordered() {
                    got_ordered = true;
                } else {
                    got_unordered = true;
                }
                let dif = did.dial_info.make_filter();

                relay_noderefs
                    .push(relay_nr.filtered_clone(NodeRefFilter::new().with_dial_info_filter(dif)));
            }
        }
        // Add noderef filters for ordered or unordered sequencing if we havent already seen those
        if !got_ordered {
            relay_noderefs.push(relay_nr.sequencing_clone(Sequencing::EnsureOrdered));
        }
        if !got_unordered {
            relay_noderefs.push(relay_nr);
        }

        for relay_nr_filtered in relay_noderefs {
            futurequeue.push_back(
                async move {
                    log_rtab!("--> PublicInternet Relay ping to {:?}", relay_nr_filtered);
                    let rpc_processor = relay_nr_filtered.rpc_processor();
                    let _ = rpc_processor
                        .rpc_call_status(Destination::direct(relay_nr_filtered))
                        .await?;
                    Ok(())
                }
                .boxed(),
            );
        }
        Ok(())
    }

    // Ping the active watch nodes to ensure they are still there
    #[instrument(level = "trace", skip(self, futurequeue), err)]
    async fn active_watches_keepalive_public_internet(
        &self,
        cur_ts: Timestamp,
        futurequeue: &mut VecDeque<PingValidatorFuture>,
    ) -> EyreResult<()> {
        let watches_need_keepalive = {
            let mut inner = self.inner.write();
            let need = inner
                .opt_active_watch_keepalive_ts
                .map(|kts| {
                    cur_ts.saturating_sub(kts).as_u64()
                        >= (ACTIVE_WATCH_KEEPALIVE_PING_INTERVAL_SECS as u64 * 1_000_000u64)
                })
                .unwrap_or(true);
            if need {
                inner.opt_active_watch_keepalive_ts = Some(cur_ts);
            }
            need
        };

        if !watches_need_keepalive {
            return Ok(());
        }

        // Get all the active watches from the storage manager
        let watch_destinations = self.storage_manager().get_active_watch_nodes().await;

        for watch_destination in watch_destinations {
            let registry = self.registry();
            futurequeue.push_back(
                async move {
                    log_rtab!("--> Watch Keepalive ping to {:?}", watch_destination);
                    let rpc_processor = registry.rpc_processor();
                    let _ = rpc_processor.rpc_call_status(watch_destination).await?;
                    Ok(())
                }
                .boxed(),
            );
        }
        Ok(())
    }

    // Ping each node in the routing table if they need to be pinged
    // to determine their reliability
    #[instrument(level = "trace", skip(self, futurequeue), err)]
    async fn ping_validator_public_internet(
        &self,
        cur_ts: Timestamp,
        futurequeue: &mut VecDeque<PingValidatorFuture>,
    ) -> EyreResult<()> {
        // Get all nodes needing pings in the PublicInternet routing domain
        let node_refs = self.get_nodes_needing_ping(RoutingDomain::PublicInternet, cur_ts);

        // Just do a single ping with the best protocol for all the other nodes to check for liveness
        for nr in node_refs {
            let nr = nr.sequencing_clone(Sequencing::PreferOrdered);

            futurequeue.push_back(
                async move {
                    #[cfg(feature = "verbose-tracing")]
                    log_rtab!(debug "--> PublicInternet Validator ping to {:?}", nr);
                    let rpc_processor = nr.rpc_processor();
                    let _ = rpc_processor
                        .rpc_call_status(Destination::direct(nr))
                        .await?;
                    Ok(())
                }
                .boxed(),
            );
        }

        Ok(())
    }

    // Ping each node in the LocalNetwork routing domain if they
    // need to be pinged to determine their reliability
    #[instrument(level = "trace", skip(self, futurequeue), err)]
    async fn ping_validator_local_network(
        &self,
        cur_ts: Timestamp,
        futurequeue: &mut VecDeque<PingValidatorFuture>,
    ) -> EyreResult<()> {
        // Get all nodes needing pings in the LocalNetwork routing domain
        let node_refs = self.get_nodes_needing_ping(RoutingDomain::LocalNetwork, cur_ts);

        // Just do a single ping with the best protocol for all the other nodes to check for liveness
        for nr in node_refs {
            let nr = nr.sequencing_clone(Sequencing::PreferOrdered);

            // Just do a single ping with the best protocol for all the nodes
            futurequeue.push_back(
                async move {
                    #[cfg(feature = "verbose-tracing")]
                    log_rtab!(debug "--> LocalNetwork Validator ping to {:?}", nr);
                    let rpc_processor = nr.rpc_processor();
                    let _ = rpc_processor
                        .rpc_call_status(Destination::direct(nr))
                        .await?;
                    Ok(())
                }
                .boxed(),
            );
        }

        Ok(())
    }

    // Common handler for running ping validations in a batch
    async fn process_ping_validation_queue(
        &self,
        name: &str,
        stop_token: StopToken,
        cur_ts: Timestamp,
        future_queue: VecDeque<PingValidatorFuture>,
    ) {
        let count = future_queue.len();
        if count == 0 {
            return;
        }
        log_rtab!(debug "[{}] Ping validation queue: {} remaining", name, count);

        let atomic_count = AtomicUsize::new(count);
        process_batched_future_queue(future_queue, MAX_PARALLEL_PINGS, stop_token, |res| async {
            if let Err(e) = res {
                log_rtab!(error "[{}] Error performing status ping: {}", name, e);
            }
            let remaining = atomic_count.fetch_sub(1, Ordering::AcqRel) - 1;
            if remaining > 0 {
                log_rtab!(debug "[{}] Ping validation queue: {} remaining", name, remaining);
            }
        })
        .await;
        let done_ts = Timestamp::now();
        log_rtab!(debug
            "[{}] Ping validation queue finished {} pings in {}",
            name,
            count,
            done_ts - cur_ts
        );
    }
}
