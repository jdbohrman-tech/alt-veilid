use super::*;

/// The core representation of the RouteSpecStore that can be serialized
#[derive(Debug, Clone, Default, RkyvArchive, RkyvSerialize, RkyvDeserialize)]
#[archive_attr(repr(C, align(8)), derive(CheckBytes))]
pub struct RouteSpecStoreContent {
    /// All of the route sets we have allocated so far indexed by key
    id_by_key: HashMap<PublicKey, RouteSetSpecId>,
    /// All of the route sets we have allocated so far
    details: HashMap<RouteSetSpecId, RouteSetSpecDetail>,
}

impl RouteSpecStoreContent {
    pub fn add_detail(&mut self, detail: RouteSetSpecDetail) -> RouteSetSpecId {
        // generate unique key string
        let id = detail.make_id();
        assert!(!self.details.contains_key(&id));

        // also store in id by key table
        for (pk, _) in detail.iter_route_set() {
            self.id_by_key.insert(*pk, id.clone());
        }
        self.details.insert(id.clone(), detail);

        id
    }
    pub fn remove_detail(&mut self, id: &RouteSetSpecId) -> Option<RouteSetSpecDetail> {
        let detail = self.details.remove(id)?;
        for (pk, _) in detail.iter_route_set() {
            self.id_by_key.remove(&pk).unwrap();
        }
        Some(detail)
    }
    pub fn get_detail(&self, id: &RouteSetSpecId) -> Option<&RouteSetSpecDetail> {
        self.details.get(id)
    }
    pub fn get_detail_mut(&mut self, id: &RouteSetSpecId) -> Option<&mut RouteSetSpecDetail> {
        self.details.get_mut(id)
    }
    pub fn get_id_by_key(&self, key: &PublicKey) -> Option<RouteSetSpecId> {
        self.id_by_key.get(key).cloned()
    }
    pub fn iter_ids(&self) -> std::collections::hash_map::Keys<RouteSetSpecId, RouteSetSpecDetail> {
        self.details.keys()
    }
    pub fn iter_details(
        &self,
    ) -> std::collections::hash_map::Iter<RouteSetSpecId, RouteSetSpecDetail> {
        self.details.iter()
    }
    pub fn iter_details_mut(
        &mut self,
    ) -> std::collections::hash_map::IterMut<RouteSetSpecId, RouteSetSpecDetail> {
        self.details.iter_mut()
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
}
