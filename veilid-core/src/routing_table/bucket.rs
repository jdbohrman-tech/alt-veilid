use super::*;
use core::sync::atomic::Ordering;

impl_veilid_log_facility!("rtab");

/// Routing Table Bucket
/// Stores map of public keys to entries, which may be in multiple routing tables per crypto kind
/// Keeps entries at a particular 'dht distance' from this cryptokind's node id
/// Helps to keep managed lists at particular distances so we can evict nodes by priority
/// where the priority comes from liveness and age of the entry (older is better)
pub struct Bucket {
    /// Component registryo accessor
    registry: VeilidComponentRegistry,
    /// Map of keys to entries for this bucket
    entries: BTreeMap<NodeId, Arc<BucketEntry>>,
    /// The crypto kind in use for the public keys in this bucket
    kind: CryptoKind,
}
pub(super) type EntriesIter<'a> = alloc::collections::btree_map::Iter<'a, NodeId, Arc<BucketEntry>>;

#[derive(Debug, Serialize, Deserialize)]
struct SerializedBucketEntryData {
    key: NodeId,
    value: u32, // index into serialized entries list
}

#[derive(Debug, Serialize, Deserialize)]
struct SerializedBucketData {
    entries: Vec<SerializedBucketEntryData>,
}

impl_veilid_component_registry_accessor!(Bucket);

impl Bucket {
    pub fn new(registry: VeilidComponentRegistry, kind: CryptoKind) -> Self {
        Self {
            registry,
            entries: BTreeMap::new(),
            kind,
        }
    }

    pub(super) fn load_bucket(
        &mut self,
        data: Vec<u8>,
        all_entries: &[Arc<BucketEntry>],
    ) -> EyreResult<()> {
        let bucket_data: SerializedBucketData = deserialize_json_bytes(&data)?;

        for e in bucket_data.entries {
            self.entries
                .insert(e.key, all_entries[e.value as usize].clone());
        }

        Ok(())
    }

    pub(super) fn save_bucket(
        &self,
        all_entries: &mut Vec<Arc<BucketEntry>>,
        entry_map: &mut HashMap<*const BucketEntry, u32>,
    ) -> Vec<u8> {
        let mut entries = Vec::new();
        for (k, v) in &self.entries {
            let entry_index = entry_map.entry(Arc::as_ptr(v)).or_insert_with(|| {
                let entry_index = all_entries.len();
                all_entries.push(v.clone());
                entry_index as u32
            });
            entries.push(SerializedBucketEntryData {
                key: *k,
                value: *entry_index,
            });
        }
        let bucket_data = SerializedBucketData { entries };

        serialize_json_bytes(bucket_data)
    }

    /// Create a new entry with a node_id of this crypto kind and return it
    pub(super) fn add_new_entry(&mut self, node_id_key: NodeId) -> Arc<BucketEntry> {
        veilid_log!(self trace "Node added: {}:{}", self.kind, node_id_key);

        // Add new entry
        let entry = Arc::new(BucketEntry::new(TypedNodeId::new(self.kind, node_id_key)));
        self.entries.insert(node_id_key, entry.clone());

        // Return the new entry
        entry
    }

    /// Add an existing entry with a new node_id for this crypto kind
    pub(super) fn add_existing_entry(&mut self, node_id_key: NodeId, entry: Arc<BucketEntry>) {
        veilid_log!(self trace "Existing node added: {}:{}", self.kind, node_id_key);

        // Add existing entry
        self.entries.insert(node_id_key, entry);
    }

    /// Remove an entry with a node_id for this crypto kind from the bucket
    pub(super) fn remove_entry(&mut self, node_id_key: &NodeId) {
        veilid_log!(self trace "Node removed: {}:{}", self.kind, node_id_key);

        // Remove the entry
        self.entries.remove(node_id_key);
    }

    pub(super) fn entry(&self, key: &NodeId) -> Option<Arc<BucketEntry>> {
        self.entries.get(key).cloned()
    }

    pub(super) fn entries(&self) -> EntriesIter {
        self.entries.iter()
    }

    pub(super) fn kick(
        &mut self,
        bucket_depth: usize,
        exempt_peers: &BTreeSet<NodeId>,
    ) -> Option<BTreeSet<NodeId>> {
        // Get number of entries to attempt to purge from bucket
        let bucket_len = self.entries.len();

        // Don't bother kicking bucket unless it is full
        if bucket_len <= bucket_depth {
            return None;
        }

        // Try to purge the newest entries that overflow the bucket
        let mut dead_node_ids: BTreeSet<NodeId> = BTreeSet::new();
        let mut extra_entries = bucket_len - bucket_depth;

        // Get the sorted list of entries by their kick order
        let mut sorted_entries: Vec<(NodeId, Arc<BucketEntry>)> =
            self.entries.iter().map(|(k, v)| (*k, v.clone())).collect();
        let cur_ts = Timestamp::now();
        sorted_entries.sort_by(|a, b| -> core::cmp::Ordering {
            if a.0 == b.0 {
                return core::cmp::Ordering::Equal;
            }
            a.1.with_inner(|ea| {
                b.1.with_inner(|eb| {
                    let astate = ea.state(cur_ts).ordering();
                    let bstate = eb.state(cur_ts).ordering();
                    // first kick punished nodes, then dead nodes, then unreliable nodes
                    if astate < bstate {
                        return core::cmp::Ordering::Less;
                    }
                    if astate > bstate {
                        return core::cmp::Ordering::Greater;
                    }
                    // then kick by time added, most recent nodes are kicked first
                    let ata = ea.peer_stats().time_added;
                    let bta = eb.peer_stats().time_added;
                    bta.cmp(&ata)
                })
            })
        });

        for entry in sorted_entries {
            // If we're not evicting more entries, exit, noting this may be the newest entry
            if extra_entries == 0 {
                break;
            }
            extra_entries -= 1;

            // if this entry has NodeRef references we can't drop it yet
            if entry.1.ref_count.load(Ordering::Acquire) > 0 {
                continue;
            }

            // if this entry is one of our exempt entries, don't drop it
            if exempt_peers.contains(&entry.0) {
                continue;
            }

            // if no references, lets evict it
            dead_node_ids.insert(entry.0);

            // And remove the node id from the entry
            entry.1.with_mut_inner(|e| e.remove_node_id(self.kind));
        }

        // Now purge the dead node ids
        for id in &dead_node_ids {
            // Remove the entry
            // The entry may not be completely gone after this happens
            // because it may still be in another bucket for a different CryptoKind
            self.remove_entry(id);
        }

        if !dead_node_ids.is_empty() {
            Some(dead_node_ids)
        } else {
            None
        }
    }
}
