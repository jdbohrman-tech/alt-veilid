use super::*;

/// How many nodes to consult for closest peers simultaneously
pub const CLOSEST_PEERS_REQUEST_COUNT: usize = 5;

use futures_util::stream::{FuturesUnordered, StreamExt};
use stop_token::future::FutureExt as StopFutureExt;

impl RoutingTable {
    /// Ask our closest peers to give us more peers close to ourselves. This will
    /// assist with the DHT and other algorithms that utilize the distance metric.
    #[instrument(level = "trace", skip(self), err)]
    pub async fn closest_peers_refresh_task_routine(self, stop_token: StopToken) -> EyreResult<()> {
        let mut unord = FuturesUnordered::new();

        for crypto_kind in VALID_CRYPTO_KINDS {
            // Get our node id for this cryptokind
            let self_node_id = self.node_id(crypto_kind);

            let routing_table = self.clone();
            let mut filters = VecDeque::new();
            let filter = Box::new(
                move |rti: &RoutingTableInner, opt_entry: Option<Arc<BucketEntry>>| {
                    // Exclude our own node
                    let Some(entry) = opt_entry else {
                        return false;
                    };

                    entry.with(rti, |_rti, e| {
                        // Keep only the entries that contain the crypto kind we're looking for
                        let compatible_crypto = e.crypto_kinds().contains(&crypto_kind);
                        if !compatible_crypto {
                            return false;
                        }
                        // Keep only the entries that participate in distance-metric relevant capabilities
                        // This would be better to be 'has_any_capabilities' but for now until out capnp gets
                        // this ability, it will do.
                        if !e.has_all_capabilities(
                            RoutingDomain::PublicInternet,
                            DISTANCE_METRIC_CAPABILITIES,
                        ) {
                            return false;
                        }
                        true
                    })
                },
            ) as RoutingTableEntryFilter;
            filters.push_front(filter);

            let noderefs = routing_table
                .find_preferred_closest_nodes(
                    CLOSEST_PEERS_REQUEST_COUNT,
                    self_node_id,
                    filters,
                    |_rti, entry: Option<Arc<BucketEntry>>| {
                        NodeRef::new(routing_table.clone(), entry.unwrap().clone())
                    },
                )
                .unwrap();

            for nr in noderefs {
                let routing_table = self.clone();
                unord.push(
                    async move {
                        // This would be better if it were 'any' instead of 'all' capabilities
                        // but that requires extending the capnp to support it.
                        routing_table
                            .reverse_find_node(
                                crypto_kind,
                                nr,
                                false,
                                DISTANCE_METRIC_CAPABILITIES.to_vec(),
                            )
                            .await
                    }
                    .instrument(Span::current()),
                );
            }
        }

        // do closest peers search in parallel
        while let Ok(Some(_)) = unord.next().timeout_at(stop_token.clone()).await {}

        Ok(())
    }
}
