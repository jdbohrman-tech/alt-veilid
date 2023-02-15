use super::*;

use futures_util::stream::{FuturesUnordered, StreamExt};
use futures_util::FutureExt;
use stop_token::future::FutureExt as StopFutureExt;

impl RoutingTable {
    // Ping each node in the routing table if they need to be pinged
    // to determine their reliability
    #[instrument(level = "trace", skip(self), err)]
    fn ping_validator_public_internet(
        &self,
        cur_ts: Timestamp,
        unord: &mut FuturesUnordered<
            SendPinBoxFuture<Result<NetworkResult<Answer<Option<SenderInfo>>>, RPCError>>,
        >,
    ) -> EyreResult<()> {
        let rpc = self.rpc_processor();

        // Get all nodes needing pings in the PublicInternet routing domain
        let node_refs = self.get_nodes_needing_ping(RoutingDomain::PublicInternet, cur_ts);

        // Look up any NAT mappings we may need to try to preserve with keepalives
        let mut mapped_port_info = self.get_low_level_port_info();

        // Get the PublicInternet relay if we are using one
        let opt_relay_nr = self.relay_node(RoutingDomain::PublicInternet);

        // Get our publicinternet dial info
        let dids = self.all_filtered_dial_info_details(
            RoutingDomain::PublicInternet.into(),
            &DialInfoFilter::all(),
        );

        // For all nodes needing pings, figure out how many and over what protocols
        for nr in node_refs {
            // If this is our relay, let's check for NAT keepalives
            let mut did_pings = false;
            if let Some(relay_nr) = opt_relay_nr {
                if nr.same_entry(&relay_nr) {
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
                            let dif = did.dial_info.make_filter();
                            let nr_filtered =
                                nr.filtered_clone(NodeRefFilter::new().with_dial_info_filter(dif));
                            log_net!("--> Keepalive ping to {:?}", nr_filtered);
                            unord.push(
                                async move {
                                    rpc.rpc_call_status(Destination::direct(nr_filtered)).await
                                }
                                .instrument(Span::current())
                                .boxed(),
                            );
                            did_pings = true;
                        }
                    }
                }
            }
            // Just do a single ping with the best protocol for all the other nodes,
            // ensuring that we at least ping a relay with -something- even if we didnt have
            // any mapped ports to preserve
            if !did_pings {
                let rpc = rpc.clone();
                unord.push(
                    async move { rpc.rpc_call_status(Destination::direct(nr)).await }
                        .instrument(Span::current())
                        .boxed(),
                );
            }
        }

        Ok(())
    }

    // Ping each node in the LocalNetwork routing domain if they
    // need to be pinged to determine their reliability
    #[instrument(level = "trace", skip(self), err)]
    fn ping_validator_local_network(
        &self,
        cur_ts: Timestamp,
        unord: &mut FuturesUnordered<
            SendPinBoxFuture<Result<NetworkResult<Answer<Option<SenderInfo>>>, RPCError>>,
        >,
    ) -> EyreResult<()> {
        let rpc = self.rpc_processor();

        // Get all nodes needing pings in the LocalNetwork routing domain
        let node_refs = self.get_nodes_needing_ping(RoutingDomain::LocalNetwork, cur_ts);

        // For all nodes needing pings, figure out how many and over what protocols
        for nr in node_refs {
            let rpc = rpc.clone();

            // Just do a single ping with the best protocol for all the nodes
            unord.push(
                async move { rpc.rpc_call_status(Destination::direct(nr)).await }
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
        self.ping_validator_public_internet(cur_ts, &mut unord)?;

        // LocalNetwork
        self.ping_validator_local_network(cur_ts, &mut unord)?;

        // Wait for ping futures to complete in parallel
        while let Ok(Some(_)) = unord.next().timeout_at(stop_token.clone()).await {}

        Ok(())
    }
}
