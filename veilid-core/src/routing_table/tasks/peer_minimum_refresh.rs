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
        let min_peer_count = self.with_config(|c| c.network.dht.min_peer_count as usize);

        // For the PublicInternet routing domain, get list of all peers we know about
        // even the unreliable ones, and ask them to find nodes close to our node too
        let routing_table = self.clone();
        let noderefs = routing_table.find_fastest_nodes(
            min_peer_count,
            VecDeque::new(),
            |_rti, entry: Option<Arc<BucketEntry>>| {
                NodeRef::new(routing_table.clone(), entry.unwrap().clone(), None)
            },
        );

        let mut ord = FuturesOrdered::new();
        for nr in noderefs {
            let routing_table = self.clone();
            ord.push_back(
                async move { routing_table.reverse_find_node(nr, false).await }
                    .instrument(Span::current()),
            );
        }

        // do peer minimum search in order from fastest to slowest
        while let Ok(Some(_)) = ord.next().timeout_at(stop_token.clone()).await {}

        Ok(())
    }
}
