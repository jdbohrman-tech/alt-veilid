use super::*;
use core::sync::atomic::Ordering;

/// Routing Table Bucket
/// Stores map of public keys to entries, which may be in multiple routing tables per crypto kind
/// Keeps entries at a particular 'dht distance' from this cryptokind's node id
/// Helps to keep managed lists at particular distances so we can evict nodes by priority
/// where the priority comes from liveness and age of the entry (older is better)
pub struct Bucket {
    /// Map of keys to entries for this bucket
    entries: BTreeMap<PublicKey, Arc<BucketEntry>>,
    /// The crypto kind in use for the public keys in this bucket
    kind: CryptoKind,
}
pub(super) type EntriesIter<'a> =
    alloc::collections::btree_map::Iter<'a, PublicKey, Arc<BucketEntry>>;

#[derive(Debug, Serialize, Deserialize)]
struct SerializedBucketEntryData {
    key: PublicKey,
    value: u32, // index into serialized entries list
}

#[derive(Debug, Serialize, Deserialize)]
struct SerializedBucketData {
    entries: Vec<SerializedBucketEntryData>,
}

fn state_ordering(state: BucketEntryState) -> usize {
    match state {
        BucketEntryState::Dead => 0,
        BucketEntryState::Unreliable => 1,
        BucketEntryState::Reliable => 2,
    }
}

impl Bucket {
    pub fn new(kind: CryptoKind) -> Self {
        Self {
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
        let out = serialize_json_bytes(&bucket_data);
        out
    }

    /// Create a new entry with a node_id of this crypto kind and return it
    pub(super) fn add_new_entry(&mut self, node_id_key: PublicKey) -> Arc<BucketEntry> {
        log_rtab!("Node added: {}:{}", self.kind, node_id_key);

        // Add new entry
        let entry = Arc::new(BucketEntry::new(TypedKey::new(self.kind, node_id_key)));
        self.entries.insert(node_id_key, entry.clone());

        // Return the new entry
        entry
    }

    /// Add an existing entry with a new node_id for this crypto kind
    pub(super) fn add_existing_entry(&mut self, node_id_key: PublicKey, entry: Arc<BucketEntry>) {
        log_rtab!("Existing node added: {}:{}", self.kind, node_id_key);

        // Add existing entry
        self.entries.insert(node_id_key, entry);
    }

    /// Remove an entry with a node_id for this crypto kind from the bucket
    pub(super) fn remove_entry(&mut self, node_id_key: &PublicKey) {
        log_rtab!("Node removed: {}:{}", self.kind, node_id_key);

        // Remove the entry
        self.entries.remove(node_id_key);
    }

    pub(super) fn entry(&self, key: &PublicKey) -> Option<Arc<BucketEntry>> {
        self.entries.get(key).cloned()
    }

    pub(super) fn entries(&self) -> EntriesIter {
        self.entries.iter()
    }

    pub(super) fn kick(&mut self, bucket_depth: usize) -> Option<BTreeSet<PublicKey>> {
        // Get number of entries to attempt to purge from bucket
        let bucket_len = self.entries.len();

        // Don't bother kicking bucket unless it is full
        if bucket_len <= bucket_depth {
            return None;
        }

        // Try to purge the newest entries that overflow the bucket
        let mut dead_node_ids: BTreeSet<PublicKey> = BTreeSet::new();
        let mut extra_entries = bucket_len - bucket_depth;

        // Get the sorted list of entries by their kick order
        let mut sorted_entries: Vec<(PublicKey, Arc<BucketEntry>)> = self
            .entries
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        let cur_ts = get_aligned_timestamp();
        sorted_entries.sort_by(|a, b| -> core::cmp::Ordering {
            if a.0 == b.0 {
                return core::cmp::Ordering::Equal;
            }
            a.1.with_inner(|ea| {
                b.1.with_inner(|eb| {
                    let astate = state_ordering(ea.state(cur_ts));
                    let bstate = state_ordering(eb.state(cur_ts));
                    // first kick dead nodes, then unreliable nodes
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

            // if this entry has references we can't drop it yet
            if entry.1.ref_count.load(Ordering::Acquire) > 0 {
                continue;
            }

            // if no references, lets evict it
            dead_node_ids.insert(entry.0);
        }

        // Now purge the dead node ids
        for id in &dead_node_ids {
            // Remove the entry
            self.remove_entry(id);
        }

        if !dead_node_ids.is_empty() {
            Some(dead_node_ids)
        } else {
            None
        }
    }
}
