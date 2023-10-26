use super::*;

/// Keepalive pings are done occasionally to ensure holepunched public dialinfo
/// remains valid, as well as to make sure we remain in any relay node's routing table
const KEEPALIVE_PING_INTERVAL_SECS: u32 = 10;

use futures_util::stream::{FuturesUnordered, StreamExt};
use futures_util::FutureExt;
use stop_token::future::FutureExt as StopFutureExt;

type PingValidatorFuture =
    SendPinBoxFuture<Result<NetworkResult<Answer<Option<SenderInfo>>>, RPCError>>;

impl RoutingTable {
    // Ping each node in the routing table if they need to be pinged
    // to determine their reliability
    #[instrument(level = "trace", skip(self), err)]
    async fn relay_keepalive_public_internet(
        &self,
        cur_ts: Timestamp,
        relay_nr: NodeRef,
        unord: &mut FuturesUnordered<PingValidatorFuture>,
    ) -> EyreResult<()> {
        let rpc = self.rpc_processor();
        // Get our publicinternet dial info
        let dids = self.all_filtered_dial_info_details(
            RoutingDomain::PublicInternet.into(),
            &DialInfoFilter::all(),
        );

        let opt_relay_keepalive_ts = self.relay_node_last_keepalive(RoutingDomain::PublicInternet);
        let relay_needs_keepalive = opt_relay_keepalive_ts
            .map(|kts| {
                cur_ts.saturating_sub(kts).as_u64()
                    >= (KEEPALIVE_PING_INTERVAL_SECS as u64 * 1_000_000u64)
            })
            .unwrap_or(true);

        if !relay_needs_keepalive {
            return Ok(());
        }
        // Say we're doing this keepalive now
        self.edit_routing_domain(RoutingDomain::PublicInternet)
            .set_relay_node_keepalive(Some(cur_ts))
            .commit(false)
            .await;

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
                let relay_nr_filtered =
                    relay_nr.filtered_clone(NodeRefFilter::new().with_dial_info_filter(dif));
                relay_noderefs.push(relay_nr_filtered);
            }
        }
        // Add noderef filters for ordered or unordered sequencing if we havent already seen those
        if !got_ordered {
            let (_, nrf) = NodeRefFilter::new().with_sequencing(Sequencing::EnsureOrdered);
            let mut relay_nr_filtered = relay_nr.filtered_clone(nrf);
            relay_nr_filtered.set_sequencing(Sequencing::EnsureOrdered);
            relay_noderefs.push(relay_nr_filtered);
        }
        if !got_unordered {
            relay_noderefs.push(relay_nr);
        }

        for relay_nr_filtered in relay_noderefs {
            let rpc = rpc.clone();

            #[cfg(feature = "network-result-extra")]
            log_rtab!(debug "--> Keepalive ping to {:?}", relay_nr_filtered);
            #[cfg(not(feature = "network-result-extra"))]
            log_rtab!("--> Keepalive ping to {:?}", relay_nr_filtered);

            unord.push(
                async move {
                    let out = rpc
                        .rpc_call_status(Destination::direct(relay_nr_filtered), true)
                        .await;
                    out
                }
                .instrument(Span::current())
                .boxed(),
            );
        }
        Ok(())
    }
    // Ping each node in the routing table if they need to be pinged
    // to determine their reliability
    #[instrument(level = "trace", skip(self), err)]
    async fn ping_validator_public_internet(
        &self,
        cur_ts: Timestamp,
        unord: &mut FuturesUnordered<PingValidatorFuture>,
    ) -> EyreResult<()> {
        let rpc = self.rpc_processor();

        // Get all nodes needing pings in the PublicInternet routing domain
        let node_refs = self.get_nodes_needing_ping(RoutingDomain::PublicInternet, cur_ts);

        // Get the PublicInternet relay if we are using one
        let opt_relay_nr = self.relay_node(RoutingDomain::PublicInternet);

        // If this is our relay, let's check for NAT keepalives
        if let Some(relay_nr) = opt_relay_nr {
            self.relay_keepalive_public_internet(cur_ts, relay_nr, unord)
                .await?;
        }

        // Just do a single ping with the best protocol for all the other nodes to check for liveness
        for nr in node_refs {
            let rpc = rpc.clone();
            log_rtab!("--> Validator ping to {:?}", nr);
            unord.push(
                async move { rpc.rpc_call_status(Destination::direct(nr), false).await }
                    .instrument(Span::current())
                    .boxed(),
            );
        }

        Ok(())
    }

    // Ping each node in the LocalNetwork routing domain if they
    // need to be pinged to determine their reliability
    #[instrument(level = "trace", skip(self), err)]
    async fn ping_validator_local_network(
        &self,
        cur_ts: Timestamp,
        unord: &mut FuturesUnordered<PingValidatorFuture>,
    ) -> EyreResult<()> {
        let rpc = self.rpc_processor();

        // Get all nodes needing pings in the LocalNetwork routing domain
        let node_refs = self.get_nodes_needing_ping(RoutingDomain::LocalNetwork, cur_ts);

        // For all nodes needing pings, figure out how many and over what protocols
        for nr in node_refs {
            let rpc = rpc.clone();

            // Just do a single ping with the best protocol for all the nodes
            unord.push(
                async move { rpc.rpc_call_status(Destination::direct(nr), false).await }
                    .instrument(Span::current())
                    .boxed(),
            );
        }

        Ok(())
    }

    // Ping each node in the routing table if they need to be pinged
    // to determine their reliability
    #[instrument(level = "trace", skip(self), err)]
    pub(crate) async fn ping_validator_task_routine(
        self,
        stop_token: StopToken,
        _last_ts: Timestamp,
        cur_ts: Timestamp,
    ) -> EyreResult<()> {
        let mut unord = FuturesUnordered::new();

        // PublicInternet
        self.ping_validator_public_internet(cur_ts, &mut unord)
            .await?;

        // LocalNetwork
        self.ping_validator_local_network(cur_ts, &mut unord)
            .await?;

        // Wait for ping futures to complete in parallel
        while let Ok(Some(_)) = unord.next().timeout_at(stop_token.clone()).await {}

        Ok(())
    }
}
