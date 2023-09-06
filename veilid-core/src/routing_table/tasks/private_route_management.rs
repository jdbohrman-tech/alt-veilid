use super::*;

use futures_util::stream::{FuturesUnordered, StreamExt};
use futures_util::FutureExt;
use stop_token::future::FutureExt as StopFutureExt;

const BACKGROUND_SAFETY_ROUTE_COUNT: usize = 2;

impl RoutingTable {
    fn get_background_safety_route_count(&self) -> usize {
        let c = self.config.get();
        if c.capabilities.disable.contains(&CAP_ROUTE) {
            0
        } else {
            BACKGROUND_SAFETY_ROUTE_COUNT
        }
    }
    /// Fastest routes sort
    fn route_sort_latency_fn(a: &(RouteId, u64), b: &(RouteId, u64)) -> cmp::Ordering {
        let mut al = a.1;
        let mut bl = b.1;
        // Treat zero latency as uncalculated
        if al == 0 {
            al = u64::MAX;
        }
        if bl == 0 {
            bl = u64::MAX;
        }
        // Less is better
        al.cmp(&bl)
    }

    /// Get the list of routes to test and drop
    ///
    /// Allocated routes to test:
    /// * if a route 'needs_testing'
    ///   . all published allocated routes
    ///   . the fastest 0..N default length routes
    /// Routes to drop:
    /// * if a route 'needs_testing'
    ///   . the N.. default routes
    ///   . the rest of the allocated unpublished routes
    ///
    /// If a route doesn't 'need_testing', then we neither test nor drop it
    #[instrument(level = "trace", skip(self))]
    fn get_allocated_routes_to_test(&self, cur_ts: Timestamp) -> Vec<RouteId> {
        let default_route_hop_count =
            self.with_config(|c| c.network.rpc.default_route_hop_count as usize);

        let rss = self.route_spec_store();
        let mut must_test_routes = Vec::<RouteId>::new();
        let mut unpublished_routes = Vec::<(RouteId, u64)>::new();
        let mut expired_routes = Vec::<RouteId>::new();
        rss.list_allocated_routes(|k, v| {
            let stats = v.get_stats();
            // Ignore nodes that don't need testing
            if !stats.needs_testing(cur_ts) {
                return Option::<()>::None;
            }
            // If this has been published, always test if we need it
            // Also if the route has never been tested, test it at least once
            if v.is_published() || stats.last_tested_ts.is_none() {
                must_test_routes.push(*k);
            }
            // If this is a default route hop length, include it in routes to keep alive
            else if v.hop_count() == default_route_hop_count {
                unpublished_routes.push((*k, stats.latency_stats.average.as_u64()));
            }
            // Else this is a route that hasnt been used recently enough and we can tear it down
            else {
                expired_routes.push(*k);
            }
            Option::<()>::None
        });

        // Sort unpublished routes by speed if we know the speed
        unpublished_routes.sort_by(Self::route_sort_latency_fn);

        // Save up to N unpublished routes and test them
        let background_safety_route_count = self.get_background_safety_route_count();
        for x in 0..(usize::min(background_safety_route_count, unpublished_routes.len())) {
            must_test_routes.push(unpublished_routes[x].0);
        }

        // Kill off all but N unpublished routes rather than testing them
        if unpublished_routes.len() > background_safety_route_count {
            for x in &unpublished_routes[background_safety_route_count..] {
                expired_routes.push(x.0);
            }
        }

        // Process dead routes
        for r in expired_routes {
            log_rtab!(debug "Expired route: {}", r);
            rss.release_route(r);
        }

        // return routes to test
        must_test_routes
    }

    /// Test set of routes and remove the ones that don't test clean
    #[instrument(level = "trace", skip(self, stop_token), err)]
    async fn test_route_set(
        &self,
        stop_token: StopToken,
        routes_needing_testing: Vec<RouteId>,
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
            dead_routes: Vec<RouteId>,
        }

        let ctx = Arc::new(Mutex::new(TestRouteContext::default()));
        {
            let mut unord = FuturesUnordered::new();
            for r in routes_needing_testing {
                let rss = rss.clone();
                let ctx = ctx.clone();
                unord.push(
                    async move {
                        let success = match rss.test_route(r).await {
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
        }

        // Process failed routes
        let ctx = Arc::try_unwrap(ctx).unwrap().into_inner();
        for r in ctx.dead_routes {
            log_rtab!(debug "Dead route failed to test: {}", r);
            rss.release_route(r);
        }

        Ok(())
    }

    /// Keep private routes assigned and accessible
    #[instrument(level = "trace", skip(self, stop_token), err)]
    pub(crate) async fn private_route_management_task_routine(
        self,
        stop_token: StopToken,
        _last_ts: Timestamp,
        cur_ts: Timestamp,
    ) -> EyreResult<()> {
        // Test locally allocated routes first
        // This may remove dead routes
        let routes_needing_testing = self.get_allocated_routes_to_test(cur_ts);
        if !routes_needing_testing.is_empty() {
            self.test_route_set(stop_token.clone(), routes_needing_testing)
                .await?;
        }

        // Ensure we have a minimum of N allocated local, unpublished routes with the default number of hops and all our supported crypto kinds
        let default_route_hop_count =
            self.with_config(|c| c.network.rpc.default_route_hop_count as usize);
        let mut local_unpublished_route_count = 0usize;
        let rss = self.route_spec_store();
        rss.list_allocated_routes(|_k, v| {
            if !v.is_published()
                && v.hop_count() == default_route_hop_count
                && v.get_route_set_keys().kinds() == VALID_CRYPTO_KINDS
            {
                local_unpublished_route_count += 1;
            }
            Option::<()>::None
        });

        let background_safety_route_count = self.get_background_safety_route_count();

        if local_unpublished_route_count < background_safety_route_count {
            let routes_to_allocate = background_safety_route_count - local_unpublished_route_count;

            // Newly allocated routes
            let mut newly_allocated_routes = Vec::new();
            for _n in 0..routes_to_allocate {
                // Parameters here must be the most inclusive safety route spec
                // These will be used by test_remote_route as well
                if let Some(k) = rss.allocate_route(
                    &VALID_CRYPTO_KINDS,
                    Stability::default(),
                    Sequencing::EnsureOrdered,
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
