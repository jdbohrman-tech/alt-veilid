use super::*;

/// The core representation of the RouteSpecStore that can be serialized
#[derive(Debug, Clone, Default, RkyvArchive, RkyvSerialize, RkyvDeserialize)]
#[archive_attr(repr(C, align(8)), derive(CheckBytes))]
pub struct RouteSpecStoreContent {
    /// All of the route sets we have allocated so far indexed by key
    id_by_key: HashMap<PublicKey, RouteId>,
    /// All of the route sets we have allocated so far
    details: HashMap<RouteId, RouteSetSpecDetail>,
}

impl RouteSpecStoreContent {
    pub async fn load(routing_table: RoutingTable) -> EyreResult<RouteSpecStoreContent> {
        // Deserialize what we can
        let table_store = routing_table.network_manager().table_store();
        let rsstdb = table_store.open("RouteSpecStore", 1).await?;
        let mut content: RouteSpecStoreContent =
            rsstdb.load_rkyv(0, b"content")?.unwrap_or_default();

        // Look up all route hop noderefs since we can't serialize those
        let mut dead_ids = Vec::new();
        for (rsid, rssd) in content.details.iter_mut() {
            // Get best route since they all should resolve
            let Some(pk) = rssd.get_best_route_set_key() else {
                dead_ids.push(rsid.clone());
                continue;
            };
            let Some(rsd) = rssd.get_route_by_key(&pk) else {
                dead_ids.push(rsid.clone());
                continue;
            };
            // Go through best route and resolve noderefs
            let mut hop_node_refs = Vec::with_capacity(rsd.hops.len());
            for h in &rsd.hops {
                let Some(nr) = routing_table.lookup_node_ref(TypedKey::new(rsd.crypto_kind, *h)) else {
                    dead_ids.push(rsid.clone());
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

        // Load secrets from pstore
        let pstore = routing_table.network_manager().protected_store();
        let secret_key_map: HashMap<PublicKey, SecretKey> = pstore
            .load_user_secret_rkyv("RouteSpecStore")
            .await?
            .unwrap_or_default();

        // Ensure we got secret keys for all the public keys
        let mut got_secret_key_ids = HashSet::new();
        for (rsid, rssd) in content.details.iter_mut() {
            let mut found_all = true;
            for (pk, rsd) in rssd.iter_route_set_mut() {
                if let Some(sk) = secret_key_map.get(pk) {
                    rsd.secret_key = *sk;
                } else {
                    found_all = false;
                    break;
                }
            }
            if found_all {
                got_secret_key_ids.insert(rsid.clone());
            }
        }

        // If we missed any, nuke those route ids
        let dead_ids: Vec<RouteId> = content
            .details
            .keys()
            .filter_map(|id| {
                if !got_secret_key_ids.contains(id) {
                    Some(*id)
                } else {
                    None
                }
            })
            .collect();
        for id in dead_ids {
            log_rtab!(debug "missing secret key, killing off private route: {}", id);
            content.remove_detail(&id);
        }

        Ok(content)
    }

    pub async fn save(&self, routing_table: RoutingTable) -> EyreResult<()> {
        // Save all the fields we care about to the frozen blob in table storage
        // This skips #[with(Skip)] saving the secret keys, we save them in the protected store instead
        let table_store = routing_table.network_manager().table_store();
        let rsstdb = table_store.open("RouteSpecStore", 1).await?;
        rsstdb.store_rkyv(0, b"content", self).await?;

        // // Keep secrets in protected store as well
        let pstore = routing_table.network_manager().protected_store();

        let mut out: HashMap<PublicKey, SecretKey> = HashMap::new();
        for (rsid, rssd) in self.details.iter() {
            for (pk, rsd) in rssd.iter_route_set() {
                out.insert(*pk, rsd.secret_key);
            }
        }

        let _ = pstore.save_user_secret_rkyv("RouteSpecStore", &out).await?; // ignore if this previously existed or not

        Ok(())
    }

    pub fn add_detail(&mut self, id: RouteId, detail: RouteSetSpecDetail) {
        assert!(!self.details.contains_key(&id));

        // also store in id by key table
        for (pk, _) in detail.iter_route_set() {
            self.id_by_key.insert(*pk, id.clone());
        }
        self.details.insert(id.clone(), detail);
    }
    pub fn remove_detail(&mut self, id: &RouteId) -> Option<RouteSetSpecDetail> {
        let detail = self.details.remove(id)?;
        for (pk, _) in detail.iter_route_set() {
            self.id_by_key.remove(&pk).unwrap();
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
    pub fn iter_ids(&self) -> std::collections::hash_map::Keys<RouteId, RouteSetSpecDetail> {
        self.details.keys()
    }
    pub fn iter_details(&self) -> std::collections::hash_map::Iter<RouteId, RouteSetSpecDetail> {
        self.details.iter()
    }

    /// Clean up local allocated routes
    /// Resets publication status and statistics for when our node info changes
    /// Routes must be republished
    pub fn reset_details(&mut self) {
        for (_k, v) in &mut self.details {
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
