use super::*;

/// The core representation of the RouteSpecStore that can be serialized
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub(super) struct RouteSpecStoreContent {
    /// All of the route sets we have allocated so far indexed by key (many to one)
    id_by_key: HashMap<PublicKey, RouteId>,
    /// All of the route sets we have allocated so far
    details: HashMap<RouteId, RouteSetSpecDetail>,
}

impl RouteSpecStoreContent {
    pub fn new() -> Self {
        Self {
            id_by_key: HashMap::new(),
            details: HashMap::new(),
        }
    }

    pub async fn load(routing_table: RoutingTable) -> EyreResult<RouteSpecStoreContent> {
        // Deserialize what we can
        let table_store = routing_table.network_manager().table_store();
        let rsstdb = table_store.open("RouteSpecStore", 1).await?;
        let mut content: RouteSpecStoreContent =
            rsstdb.load_json(0, b"content").await?.unwrap_or_default();

        // Look up all route hop noderefs since we can't serialize those
        let mut dead_ids = Vec::new();
        for (rsid, rssd) in content.details.iter_mut() {
            // Get best route since they all should resolve
            let Some(pk) = rssd.get_best_route_set_key() else {
                dead_ids.push(*rsid);
                continue;
            };
            let Some(rsd) = rssd.get_route_by_key(&pk) else {
                dead_ids.push(*rsid);
                continue;
            };
            // Go through best route and resolve noderefs
            let mut hop_node_refs = Vec::with_capacity(rsd.hops.len());
            for h in &rsd.hops {
                let Ok(Some(nr)) =
                    routing_table.lookup_node_ref(TypedKey::new(rsd.crypto_kind, *h))
                else {
                    dead_ids.push(*rsid);
                    break;
                };
                hop_node_refs.push(nr);
            }

            // Apply noderefs
            rssd.set_hop_node_refs(hop_node_refs);
        }
        for id in dead_ids {
            log_rtab!(debug "no entry, killing off private route: {}", id);
            content.remove_detail(&id);
        }

        Ok(content)
    }

    pub async fn save(&self, routing_table: RoutingTable) -> EyreResult<()> {
        // Save all the fields we care about to the frozen blob in table storage
        // This skips #[with(Skip)] saving the secret keys, we save them in the protected store instead
        let table_store = routing_table.network_manager().table_store();
        let rsstdb = table_store.open("RouteSpecStore", 1).await?;
        rsstdb.store_json(0, b"content", self).await?;

        Ok(())
    }

    pub fn add_detail(&mut self, id: RouteId, detail: RouteSetSpecDetail) {
        assert!(!self.details.contains_key(&id));

        // also store in id by key table
        for (pk, _) in detail.iter_route_set() {
            self.id_by_key.insert(*pk, id);
        }
        self.details.insert(id, detail);
    }
    pub fn remove_detail(&mut self, id: &RouteId) -> Option<RouteSetSpecDetail> {
        let detail = self.details.remove(id)?;
        for (pk, _) in detail.iter_route_set() {
            self.id_by_key.remove(pk).unwrap();
        }
        Some(detail)
    }
    pub fn get_detail_count(&self) -> usize {
        self.details.len()
    }
    pub fn get_detail(&self, id: &RouteId) -> Option<&RouteSetSpecDetail> {
        self.details.get(id)
    }
    pub fn get_detail_mut(&mut self, id: &RouteId) -> Option<&mut RouteSetSpecDetail> {
        self.details.get_mut(id)
    }
    pub fn get_id_by_key(&self, key: &PublicKey) -> Option<RouteId> {
        self.id_by_key.get(key).cloned()
    }
    // pub fn iter_ids(&self) -> std::collections::hash_map::Keys<RouteId, RouteSetSpecDetail> {
    //     self.details.keys()
    // }
    pub fn iter_details(&self) -> std::collections::hash_map::Iter<RouteId, RouteSetSpecDetail> {
        self.details.iter()
    }

    /// Clean up local allocated routes
    /// Resets publication status and statistics for when our node info changes
    /// Routes must be republished
    pub fn reset_details(&mut self) {
        for v in self.details.values_mut() {
            // Must republish route now
            v.set_published(false);
            // Restart stats for routes so we test the route again
            v.get_stats_mut().reset();
        }
    }

    /// Roll transfer statistics
    pub fn roll_transfers(&mut self, last_ts: Timestamp, cur_ts: Timestamp) {
        for rssd in self.details.values_mut() {
            rssd.get_stats_mut().roll_transfers(last_ts, cur_ts);
        }
    }
}
