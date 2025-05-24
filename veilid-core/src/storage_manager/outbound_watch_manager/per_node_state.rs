use super::*;

impl_veilid_log_facility!("stor");

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub(in crate::storage_manager) struct PerNodeKey {
    /// Watched record key
    pub record_key: TypedRecordKey,
    /// Watching node id
    pub node_id: TypedNodeId,
}

impl fmt::Display for PerNodeKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}@{}", self.record_key, self.node_id)
    }
}

impl FromStr for PerNodeKey {
    type Err = VeilidAPIError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (rkey, nid) = s
            .split_once('@')
            .ok_or_else(|| VeilidAPIError::parse_error("invalid per-node key", s))?;
        Ok(PerNodeKey {
            record_key: TypedRecordKey::from_str(rkey)?,
            node_id: TypedNodeId::from_str(nid)?,
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(in crate::storage_manager) struct PerNodeState {
    /// Watch Id
    pub watch_id: u64,
    /// SafetySelection used to contact the node
    pub safety_selection: SafetySelection,
    /// What key was used to perform the watch
    pub opt_watcher: Option<KeyPair>,
    /// The expiration of a successful watch
    pub expiration_ts: Timestamp,
    /// How many value change notifications are left
    pub count: u32,
    /// Resolved watch node reference
    #[serde(skip)]
    pub watch_node_ref: Option<NodeRef>,
    /// Which private route is responsible for receiving ValueChanged notifications
    pub opt_value_changed_route: Option<PublicKey>,
}

impl fmt::Display for PerNodeState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ watch_id={}, safety_selection={:?}, opt_watcher={}, expiration_ts={}, count={}, watch_node_ref={}, opt_value_changed_route={} }}",
            self.watch_id,
            self.safety_selection,
            if let Some(watcher) = &self.opt_watcher {
                watcher.to_string()
            } else {
                "None".to_owned()
            },
            self.expiration_ts,
            self.count,
            if let Some(watch_node_ref) = &self.watch_node_ref {
                watch_node_ref.to_string()
            } else {
                "None".to_string()
            },
            if let Some(value_changed_route)= &self.opt_value_changed_route {
                value_changed_route.to_string()
            } else {
                "None".to_string()
            }
        )
    }
}
