use super::*;

/// What remote private routes have seen
#[derive(Debug, Clone, Default)]
pub(crate) struct RemotePrivateRouteInfo {
    /// The private routes themselves
    private_routes: Vec<PrivateRoute>,
    /// Did this remote private route see our node info due to no safety route in use
    last_seen_our_node_info_ts: Timestamp,
    /// Last time this remote private route was requested for any reason (cache expiration)
    last_touched_ts: Timestamp,
    /// Stats
    stats: RouteStats,
}

impl RemotePrivateRouteInfo {
    pub fn new(private_routes: Vec<PrivateRoute>, cur_ts: Timestamp) -> Self {
        RemotePrivateRouteInfo {
            private_routes,
            last_seen_our_node_info_ts: Timestamp::new(0),
            last_touched_ts: cur_ts,
            stats: RouteStats::new(cur_ts),
        }
    }
    pub fn get_private_routes(&self) -> &[PrivateRoute] {
        &self.private_routes
    }
    pub fn best_private_route(&self) -> Option<PrivateRoute> {
        self.private_routes
            .iter()
            .reduce(|acc, x| {
                if x.public_key < acc.public_key {
                    x
                } else {
                    acc
                }
            })
            .filter(|x| VALID_CRYPTO_KINDS.contains(&x.public_key.kind))
            .cloned()
    }
    pub fn get_stats(&self) -> &RouteStats {
        &self.stats
    }
    pub fn get_stats_mut(&mut self) -> &mut RouteStats {
        &mut self.stats
    }

    pub fn has_seen_our_node_info_ts(&self, our_node_info_ts: Timestamp) -> bool {
        self.last_seen_our_node_info_ts == our_node_info_ts
    }
    pub fn set_last_seen_our_node_info_ts(&mut self, last_seen_our_node_info_ts: Timestamp) {
        self.last_seen_our_node_info_ts = last_seen_our_node_info_ts;
    }

    // Check to see if this remote private route has expired
    pub fn did_expire(&self, cur_ts: Timestamp) -> bool {
        cur_ts.saturating_sub(self.last_touched_ts) >= REMOTE_PRIVATE_ROUTE_CACHE_EXPIRY
    }

    /// Start fresh if this had expired
    pub fn unexpire(&mut self, cur_ts: Timestamp) {
        self.last_seen_our_node_info_ts = Timestamp::new(0);
        self.last_touched_ts = cur_ts;
        self.stats = RouteStats::new(cur_ts);
    }

    /// Note when this was last used
    pub fn touch(&mut self, cur_ts: Timestamp) {
        self.last_touched_ts = cur_ts;
    }
}
