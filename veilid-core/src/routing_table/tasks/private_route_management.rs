use super::*;

use futures_util::stream::{FuturesUnordered, StreamExt};
use futures_util::FutureExt;
use stop_token::future::FutureExt as StopFutureExt;

const BACKGROUND_SAFETY_ROUTE_COUNT: usize = 2;

impl RoutingTable {
    /// Test set of routes and remove the ones that don't test clean
    #[instrument(level = "trace", skip(self, stop_token), err)]
    async fn test_route_set(
        &self,
        stop_token: StopToken,
        routes_needing_testing: Vec<DHTKey>,
    ) -> EyreResult<()> {
        if routes_needing_testing.is_empty() {
            return Ok(());
        }
        log_rtab!("Testing routes: {:?}", routes_needing_testing);

        // Test all the routes that need testing at the same time
        let rss = self.route_spec_store();
        #[derive(Default, Debug)]
        struct TestRouteContext {
            failed: bool,
            dead_routes: Vec<DHTKey>,
        }

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
                            log_rtab!(error "Test route failed: {}", e);
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

        Ok(())
    }

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

        // Test locally allocated routes first
        // This may remove dead routes
        let rss = self.route_spec_store();
        let routes_needing_testing = rss.list_allocated_routes(|k, v| {
            let stats = v.get_stats();
            if stats.needs_testing(cur_ts) {
                return Some(*k);
            } else {
                return None;
            }
        });
        if !routes_needing_testing.is_empty() {
            self.test_route_set(stop_token.clone(), routes_needing_testing)
                .await?;
        }

        // Ensure we have a minimum of N allocated local, unpublished routes with the default number of hops
        let default_route_hop_count =
            self.with_config(|c| c.network.rpc.default_route_hop_count as usize);
        let mut local_unpublished_route_count = 0usize;
        rss.list_allocated_routes(|_k, v| {
            if !v.is_published() && v.hop_count() == default_route_hop_count {
                local_unpublished_route_count += 1;
            }
            Option::<()>::None
        });
        if local_unpublished_route_count < BACKGROUND_SAFETY_ROUTE_COUNT {
            let routes_to_allocate = BACKGROUND_SAFETY_ROUTE_COUNT - local_unpublished_route_count;

            // Newly allocated routes
            let mut newly_allocated_routes = Vec::new();
            for _n in 0..routes_to_allocate {
                // Parameters here must be the default safety route spec
                // These will be used by test_remote_route as well
                if let Some(k) = rss.allocate_route(
                    Stability::default(),
                    Sequencing::default(),
                    default_route_hop_count,
                    DirectionSet::all(),
                    &[],
                )? {
                    newly_allocated_routes.push(k);
                }
            }

            // Immediately test them
            if !newly_allocated_routes.is_empty() {
                self.test_route_set(stop_token.clone(), newly_allocated_routes)
                    .await?;
            }
        }

        // Test remote routes next
        let remote_routes_needing_testing = rss.list_remote_routes(|k, v| {
            let stats = v.get_stats();
            if stats.needs_testing(cur_ts) {
                return Some(*k);
            } else {
                return None;
            }
        });
        if !remote_routes_needing_testing.is_empty() {
            self.test_route_set(stop_token.clone(), remote_routes_needing_testing)
                .await?;
        }

        // Send update (also may send updates for released routes done by other parts of the program)
        rss.send_route_update();

        Ok(())
    }
}
