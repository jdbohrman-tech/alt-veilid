mod outbound_watch;
mod outbound_watch_parameters;
mod outbound_watch_state;
mod per_node_state;

pub(in crate::storage_manager) use outbound_watch::*;
pub(in crate::storage_manager) use outbound_watch_parameters::*;
pub(in crate::storage_manager) use outbound_watch_state::*;
pub(in crate::storage_manager) use per_node_state::*;

use super::*;

use serde_with::serde_as;

impl_veilid_log_facility!("stor");

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub(in crate::storage_manager) struct OutboundWatchManager {
    /// Each watch per record key
    #[serde(skip)]
    pub outbound_watches: HashMap<TypedRecordKey, OutboundWatch>,
    /// Last known active watch per node+record
    #[serde_as(as = "Vec<(_, _)>")]
    pub per_node_states: HashMap<PerNodeKey, PerNodeState>,
    /// Value changed updates that need inpection to determine if they should be reported
    #[serde(skip)]
    pub needs_change_inspection: HashMap<TypedRecordKey, ValueSubkeyRangeSet>,
}

impl fmt::Display for OutboundWatchManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = format!("outbound_watches({}): [\n", self.outbound_watches.len());
        {
            let mut keys = self.outbound_watches.keys().copied().collect::<Vec<_>>();
            keys.sort();

            for k in keys {
                let v = self.outbound_watches.get(&k).unwrap();
                out += &format!("  {}:\n{}\n", k, indent_all_by(4, v.to_string()));
            }
        }
        out += "]\n";
        out += &format!("per_node_states({}): [\n", self.per_node_states.len());
        {
            let mut keys = self.per_node_states.keys().copied().collect::<Vec<_>>();
            keys.sort();

            for k in keys {
                let v = self.per_node_states.get(&k).unwrap();
                out += &format!("  {}:\n{}\n", k, indent_all_by(4, v.to_string()));
            }
        }
        out += "]\n";
        out += &format!(
            "needs_change_inspection({}): [\n",
            self.needs_change_inspection.len()
        );
        {
            let mut keys = self
                .needs_change_inspection
                .keys()
                .copied()
                .collect::<Vec<_>>();
            keys.sort();

            for k in keys {
                let v = self.needs_change_inspection.get(&k).unwrap();
                out += &format!("  {}: {}\n", k, v);
            }
        }
        out += "]\n";

        write!(f, "{}", out)
    }
}

impl Default for OutboundWatchManager {
    fn default() -> Self {
        Self::new()
    }
}

impl OutboundWatchManager {
    pub fn new() -> Self {
        Self {
            outbound_watches: HashMap::new(),
            per_node_states: HashMap::new(),
            needs_change_inspection: HashMap::new(),
        }
    }

    pub fn prepare(&mut self, routing_table: VeilidComponentGuard<'_, RoutingTable>) {
        for (pnk, pns) in &mut self.per_node_states {
            pns.watch_node_ref = match routing_table.lookup_node_ref(pnk.node_id) {
                Ok(v) => v,
                Err(e) => {
                    veilid_log!(routing_table debug "Error looking up outbound watch node ref: {}", e);
                    None
                }
            };
        }
        self.per_node_states
            .retain(|_, v| v.watch_node_ref.is_some());

        let keys = self.per_node_states.keys().copied().collect::<HashSet<_>>();

        for v in self.outbound_watches.values_mut() {
            if let Some(state) = v.state_mut() {
                state.edit(&self.per_node_states, |editor| {
                    editor.retain_nodes(|n| keys.contains(n));
                })
            }
        }
    }

    pub fn set_desired_watch(
        &mut self,
        record_key: TypedRecordKey,
        desired_watch: Option<OutboundWatchParameters>,
    ) {
        match self.outbound_watches.get_mut(&record_key) {
            Some(w) => {
                // Replace desired watch
                w.set_desired(desired_watch);

                // Remove if the watch is done (shortcut the dead state)
                if w.state().is_none() && w.state().is_none() {
                    self.outbound_watches.remove(&record_key);
                }
            }
            None => {
                // Watch does not exist, add one if that's what is desired
                if let Some(desired) = desired_watch {
                    self.outbound_watches
                        .insert(record_key, OutboundWatch::new(record_key, desired));
                }
            }
        }
    }

    pub fn set_next_reconcile_ts(&mut self, record_key: TypedRecordKey, next_ts: Timestamp) {
        if let Some(outbound_watch) = self.outbound_watches.get_mut(&record_key) {
            if let Some(state) = outbound_watch.state_mut() {
                state.edit(&self.per_node_states, |editor| {
                    editor.set_next_reconcile_ts(next_ts);
                });
            }
        }
    }

    /// Iterate all per-node watches and remove ones with dead nodes from outbound watches
    /// This may trigger reconciliation to increase the number of active per-node watches
    /// for an outbound watch that is still alive
    pub fn update_per_node_states(&mut self, cur_ts: Timestamp) {
        // Node is unreachable
        let mut dead_pnks = HashSet::new();
        // Per-node expiration reached
        let mut expired_pnks = HashSet::new();
        // Count reached
        let mut finished_pnks = HashSet::new();

        for (pnk, pns) in &self.per_node_states {
            if pns.count == 0 {
                // If per-node watch is done, add to finished list
                finished_pnks.insert(*pnk);
            } else if !pns
                .watch_node_ref
                .as_ref()
                .unwrap()
                .state(cur_ts)
                .is_alive()
            {
                // If node is unreachable add to dead list
                dead_pnks.insert(*pnk);
            } else if cur_ts >= pns.expiration_ts {
                // If per-node watch has expired add to expired list
                expired_pnks.insert(*pnk);
            }
        }

        // Go through and remove nodes that are dead or finished from active states
        // If an expired per-node watch is still referenced, it may be renewable
        // so remove it from the expired list
        for v in self.outbound_watches.values_mut() {
            let Some(current) = v.state_mut() else {
                continue;
            };

            // Don't drop expired per-node watches that could be renewed (still referenced by this watch)
            for node in current.nodes() {
                expired_pnks.remove(node);
            }

            // Remove dead and finished per-node watch nodes from this outbound watch
            current.edit(&self.per_node_states, |editor| {
                editor.retain_nodes(|x| !dead_pnks.contains(x) && !finished_pnks.contains(x));
            });
        }

        // Drop finished per-node watches and unreferenced expired per-node watches
        self.per_node_states
            .retain(|k, _| !finished_pnks.contains(k) && !expired_pnks.contains(k));
    }

    /// Set a record up to be inspected for changed subkeys
    pub fn enqueue_change_inspect(
        &mut self,
        record_key: TypedRecordKey,
        subkeys: ValueSubkeyRangeSet,
    ) {
        self.needs_change_inspection
            .entry(record_key)
            .and_modify(|x| *x = x.union(&subkeys))
            .or_insert(subkeys);
    }
}
