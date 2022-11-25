use super::super::*;
use crate::xx::*;

use futures_util::stream::{FuturesUnordered, StreamExt};
use futures_util::FutureExt;
use stop_token::future::FutureExt as StopFutureExt;

impl RoutingTable {
    /// Keep private routes assigned and accessible
    #[instrument(level = "trace", skip(self, stop_token), err)]
    pub(crate) async fn private_route_management_task_routine(
        self,
        stop_token: StopToken,
        _last_ts: u64,
        cur_ts: u64,
    ) -> EyreResult<()> {
        // Get our node's current node info and network class and do the right thing
        let network_class = self
            .get_network_class(RoutingDomain::PublicInternet)
            .unwrap_or(NetworkClass::Invalid);

        // If we don't know our network class then don't do this yet
        if network_class == NetworkClass::Invalid {
            return Ok(());
        }

        // Collect any routes that need that need testing
        let rss = self.route_spec_store();
        let mut routes_needing_testing = rss.list_allocated_routes(|k, v| {
            let stats = v.get_stats();
            if stats.needs_testing(cur_ts) {
                return Some(*k);
            } else {
                return None;
            }
        });
        let mut remote_routes_needing_testing = rss.list_remote_routes(|k, v| {
            let stats = v.get_stats();
            if stats.needs_testing(cur_ts) {
                return Some(*k);
            } else {
                return None;
            }
        });
        routes_needing_testing.append(&mut remote_routes_needing_testing);

        // Test all the routes that need testing at the same time
        #[derive(Default, Debug)]
        struct TestRouteContext {
            failed: bool,
            dead_routes: Vec<DHTKey>,
        }

        if !routes_needing_testing.is_empty() {
            let mut unord = FuturesUnordered::new();
            let ctx = Arc::new(Mutex::new(TestRouteContext::default()));
            for r in routes_needing_testing {
                let rss = rss.clone();
                let ctx = ctx.clone();
                unord.push(
                    async move {
                        let success = match rss.test_route(&r).await {
                            Ok(v) => v,
                            Err(e) => {
                                log_rtab!(error "test route failed: {}", e);
                                ctx.lock().failed = true;
                                return;
                            }
                        };
                        if success {
                            // Route is okay, leave it alone
                            return;
                        }
                        // Route test failed
                        ctx.lock().dead_routes.push(r);
                    }
                    .instrument(Span::current())
                    .boxed(),
                );
            }

            // Wait for test_route futures to complete in parallel
            while let Ok(Some(_)) = unord.next().timeout_at(stop_token.clone()).await {}

            // Process failed routes
            let ctx = &mut *ctx.lock();
            for r in &ctx.dead_routes {
                log_rtab!(debug "Dead route: {}", &r);
                rss.release_route(r);
            }
        }

        // Send update (also may send updates for released routes done by other parts of the program)
        rss.send_route_update();

        Ok(())
    }
}
