use super::*;

use futures_util::stream::{FuturesOrdered, StreamExt};
use stop_token::future::FutureExt as StopFutureExt;

impl RoutingTable {
    // Ask our remaining peers to give us more peers before we go
    // back to the bootstrap servers to keep us from bothering them too much
    // This only adds PublicInternet routing domain peers. The discovery
    // mechanism for LocalNetwork suffices for locating all the local network
    // peers that are available. This, however, may query other LocalNetwork
    // nodes for their PublicInternet peers, which is a very fast way to get
    // a new node online.
    #[instrument(level = "trace", skip(self), err)]
    pub(crate) async fn peer_minimum_refresh_task_routine(
        self,
        stop_token: StopToken,
    ) -> EyreResult<()> {
        // Get counts by crypto kind
        let entry_count = self.inner.read().cached_entry_counts();

        let (min_peer_count, min_peer_refresh_time_ms) = self.with_config(|c| {
            (
                c.network.dht.min_peer_count as usize,
                c.network.dht.min_peer_refresh_time_ms,
            )
        });

        // For the PublicInternet routing domain, get list of all peers we know about
        // even the unreliable ones, and ask them to find nodes close to our node too

        let mut ord = FuturesOrdered::new();
        let cur_ts = get_timestamp();

        for crypto_kind in VALID_CRYPTO_KINDS {
            // Do we need to peer minimum refresh this crypto kind?
            let eckey = (RoutingDomain::PublicInternet, crypto_kind);
            let cnt = entry_count.get(&eckey).copied().unwrap_or_default();
            if cnt == 0 || cnt > min_peer_count {
                // If we have enough nodes, skip it
                // If we have zero nodes, bootstrap will get it
                continue;
            }

            let routing_table = self.clone();
            let mut filters = VecDeque::new();
            let filter = Box::new(
                move |rti: &RoutingTableInner, opt_entry: Option<Arc<BucketEntry>>| {
                    let entry = opt_entry.unwrap().clone();
                    entry.with(rti, |_rti, e| {
                        // Keep only the entries that contain the crypto kind we're looking for
                        let compatible_crypto = e.crypto_kinds().contains(&crypto_kind);
                        if !compatible_crypto {
                            return false;
                        }
                        // Keep only the entries we haven't talked to in the min_peer_refresh_time
                        if let Some(last_q_ts) = e.peer_stats().rpc_stats.last_question_ts {
                            if cur_ts.saturating_sub(last_q_ts.as_u64())
                                < (min_peer_refresh_time_ms as u64 * 1_000u64)
                            {
                                return false;
                            }
                        }
                        true
                    })
                },
            ) as RoutingTableEntryFilter;
            filters.push_front(filter);

            let noderefs = routing_table.find_fastest_nodes(
                min_peer_count,
                filters,
                |_rti, entry: Option<Arc<BucketEntry>>| {
                    NodeRef::new(routing_table.clone(), entry.unwrap().clone(), None)
                },
            );

            for nr in noderefs {
                let routing_table = self.clone();
                ord.push_back(
                    async move {
                        routing_table
                            .reverse_find_node(crypto_kind, nr, false)
                            .await
                    }
                    .instrument(Span::current()),
                );
            }
        }

        // do peer minimum search in order from fastest to slowest
        while let Ok(Some(_)) = ord.next().timeout_at(stop_token.clone()).await {}

        Ok(())
    }
}
