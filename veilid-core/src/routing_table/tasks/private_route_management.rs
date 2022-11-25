use super::super::*;
use crate::xx::*;

use futures_util::stream::{FuturesOrdered, StreamExt};
use stop_token::future::FutureExt as StopFutureExt;

impl RoutingTable {
    // Keep private routes assigned and accessible
    #[instrument(level = "trace", skip(self), err)]
    pub(crate) async fn private_route_management_task_routine(
        self,
        _stop_token: StopToken,
        _last_ts: u64,
        cur_ts: u64,
    ) -> EyreResult<()> {
        // Get our node's current node info and network class and do the right thing
        let own_peer_info = self.get_own_peer_info(RoutingDomain::PublicInternet);
        let network_class = self.get_network_class(RoutingDomain::PublicInternet);

        // Get routing domain editor
        let mut editor = self.edit_routing_domain(RoutingDomain::PublicInternet);

        // Do we know our network class yet?
        if let Some(network_class) = network_class {

            // see if we have any routes that need testing
        }

        // Commit the changes
        editor.commit().await;

        Ok(())
    }
}
