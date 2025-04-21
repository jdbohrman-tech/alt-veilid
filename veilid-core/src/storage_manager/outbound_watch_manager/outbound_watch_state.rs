use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(in crate::storage_manager) struct OutboundWatchState {
    /// Requested parameters
    params: OutboundWatchParameters,
    /// Nodes that have an active watch on our behalf
    nodes: Vec<PerNodeKey>,
    /// How many value change updates remain
    remaining_count: u32,
    /// The next earliest time we are willing to try to reconcile and improve the watch
    opt_next_reconcile_ts: Option<Timestamp>,
    /// The number of nodes we got at our last reconciliation
    opt_last_consensus_node_count: Option<usize>,
    /// Calculated field: minimum expiration time for all our nodes
    min_expiration_ts: Timestamp,
    /// Calculated field: the set of value changed routes for this watch from all per node watches
    value_changed_routes: BTreeSet<PublicKey>,
}

impl fmt::Display for OutboundWatchState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut value_changed_routes = self
            .value_changed_routes
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        value_changed_routes.sort();

        write!(
            f,
            r#"params: {}
nodes: [{}]
remaining_count: {}
opt_next_reconcile_ts: {}
opt_consensus_node_count: {}
min_expiration_ts: {}
value_changed_routes: [{}]"#,
            self.params,
            self.nodes
                .iter()
                .map(|x| x.node_id.to_string())
                .collect::<Vec<_>>()
                .join(","),
            self.remaining_count,
            if let Some(next_reconcile_ts) = &self.opt_next_reconcile_ts {
                next_reconcile_ts.to_string()
            } else {
                "None".to_owned()
            },
            if let Some(consensus_node_count) = &self.opt_last_consensus_node_count {
                consensus_node_count.to_string()
            } else {
                "None".to_owned()
            },
            self.min_expiration_ts,
            value_changed_routes.join(","),
        )
    }
}

pub(in crate::storage_manager) struct OutboundWatchStateEditor<'a> {
    state: &'a mut OutboundWatchState,
}

impl OutboundWatchStateEditor<'_> {
    pub fn set_params(&mut self, params: OutboundWatchParameters) {
        self.state.params = params;
    }
    pub fn add_nodes<I: IntoIterator<Item = PerNodeKey>>(&mut self, nodes: I) {
        for node in nodes {
            if !self.state.nodes.contains(&node) {
                self.state.nodes.push(node);
            }
        }
    }
    pub fn retain_nodes<F: FnMut(&PerNodeKey) -> bool>(&mut self, f: F) {
        self.state.nodes.retain(f);
    }
    pub fn set_remaining_count(&mut self, remaining_count: u32) {
        self.state.remaining_count = remaining_count;
    }
    pub fn set_next_reconcile_ts(&mut self, next_reconcile_ts: Timestamp) {
        self.state.opt_next_reconcile_ts = Some(next_reconcile_ts);
    }
    pub fn update_last_consensus_node_count(&mut self) {
        self.state.opt_last_consensus_node_count = Some(self.state.nodes().len());
    }
}

impl OutboundWatchState {
    pub fn new(params: OutboundWatchParameters) -> Self {
        let remaining_count = params.count;
        let min_expiration_ts = params.expiration_ts;

        Self {
            params,
            nodes: vec![],
            remaining_count,
            opt_next_reconcile_ts: None,
            opt_last_consensus_node_count: None,
            min_expiration_ts,
            value_changed_routes: BTreeSet::new(),
        }
    }

    pub fn params(&self) -> &OutboundWatchParameters {
        &self.params
    }
    pub fn nodes(&self) -> &Vec<PerNodeKey> {
        &self.nodes
    }
    pub fn remaining_count(&self) -> u32 {
        self.remaining_count
    }
    pub fn next_reconcile_ts(&self) -> Option<Timestamp> {
        self.opt_next_reconcile_ts
    }
    pub fn last_consensus_node_count(&self) -> Option<usize> {
        self.opt_last_consensus_node_count
    }
    pub fn min_expiration_ts(&self) -> Timestamp {
        self.min_expiration_ts
    }
    pub fn value_changed_routes(&self) -> &BTreeSet<PublicKey> {
        &self.value_changed_routes
    }

    /// Get the parameters we use if we're updating this state's per node watches
    pub fn get_per_node_params(
        &self,
        desired: &OutboundWatchParameters,
    ) -> OutboundWatchParameters {
        // Change the params to update count
        if self.params() != desired {
            // If parameters are changing, just use the desired parameters
            desired.clone()
        } else {
            // If this is a renewal of the same parameters,
            // use the current remaining update count for the rpc
            let mut renew_params = desired.clone();
            renew_params.count = self.remaining_count();
            renew_params
        }
    }

    pub fn edit<R, F: FnOnce(&mut OutboundWatchStateEditor) -> R>(
        &mut self,
        per_node_state: &HashMap<PerNodeKey, PerNodeState>,
        closure: F,
    ) -> R {
        let mut editor = OutboundWatchStateEditor { state: self };
        let res = closure(&mut editor);

        // Update calculated fields
        self.min_expiration_ts = self
            .nodes
            .iter()
            .map(|x| per_node_state.get(x).unwrap().expiration_ts)
            .reduce(|a, b| a.min(b))
            .unwrap_or(self.params.expiration_ts);

        self.value_changed_routes = self
            .nodes
            .iter()
            .filter_map(|x| per_node_state.get(x).unwrap().opt_value_changed_route)
            .collect();

        res
    }

    pub fn watch_node_refs(
        &self,
        per_node_state: &HashMap<PerNodeKey, PerNodeState>,
    ) -> Vec<NodeRef> {
        self.nodes
            .iter()
            .map(|x| {
                per_node_state
                    .get(x)
                    .unwrap()
                    .watch_node_ref
                    .clone()
                    .unwrap()
            })
            .collect()
    }
}
