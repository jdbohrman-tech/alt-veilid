use super::*;
use core::sync::atomic::Ordering;
use rkyv::{Archive as RkyvArchive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

/// Routing Table Bucket
/// Stores map of public keys to entries, which may be in multiple routing tables per crypto kind
/// Keeps entries at a particular 'dht distance' from this cryptokind's node id
/// Helps to keep managed lists at particular distances so we can evict nodes by priority
/// where the priority comes from liveness and age of the entry (older is better)
pub struct Bucket {
    /// handle to the routing table
    routing_table: RoutingTable,
    /// Map of keys to entries for this bucket
    entries: BTreeMap<TypedKey, Arc<BucketEntry>>,
    /// The most recent entry in this bucket
    newest_entry: Option<TypedKey>,
    /// The crypto kind in use for the public keys in this bucket
    kind: CryptoKind,
}
pub(super) type EntriesIter<'a> =
    alloc::collections::btree_map::Iter<'a, TypedKey, Arc<BucketEntry>>;

#[derive(Debug, RkyvArchive, RkyvSerialize, RkyvDeserialize)]
#[archive_attr(repr(C), derive(CheckBytes))]
struct SerializedBucketEntryData {
    key: TypedKey,
    value: u32, // index into serialized entries list
}

#[derive(Debug, RkyvArchive, RkyvSerialize, RkyvDeserialize)]
#[archive_attr(repr(C), derive(CheckBytes))]
struct SerializedBucketData {
    entries: Vec<SerializedBucketEntryData>,
    newest_entry: Option<TypedKey>,
}

fn state_ordering(state: BucketEntryState) -> usize {
    match state {
        BucketEntryState::Dead => 0,
        BucketEntryState::Unreliable => 1,
        BucketEntryState::Reliable => 2,
    }
}

impl Bucket {
    pub fn new(routing_table: RoutingTable, kind: CryptoKind) -> Self {
        Self {
            routing_table,
            entries: BTreeMap::new(),
            newest_entry: None,
            kind,
        }
    }

    pub(super) fn load_bucket(
        &mut self,
        data: Vec<u8>,
        all_entries: &[Arc<BucketEntry>],
    ) -> EyreResult<()> {
        let bucket_data: SerializedBucketData = from_rkyv(data)?;

        for e in bucket_data.entries {
            self.entries
                .insert(e.key, all_entries[e.value as usize].clone());
        }

        self.newest_entry = bucket_data.newest_entry;

        Ok(())
    }

    pub(super) fn save_bucket(
        &self,
        all_entries: &mut Vec<Arc<BucketEntry>>,
        entry_map: &mut HashMap<*const BucketEntry, u32>,
    ) -> EyreResult<Vec<u8>> {
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
        let bucket_data = SerializedBucketData {
            entries,
            newest_entry: self.newest_entry.clone(),
        };
        let out = to_rkyv(&bucket_data)?;
        Ok(out)
    }

    /// Create a new entry with a node_id of this crypto kind and return it
    pub(super) fn add_entry(&mut self, node_id: TypedKey) -> NodeRef {
        assert_eq!(node_id.kind, self.kind);

        log_rtab!("Node added: {}", node_id);

        // Add new entry
        let entry = Arc::new(BucketEntry::new());
        entry.with_mut_inner(|e| e.add_node_id(node_id));
        self.entries.insert(node_id.key, entry.clone());

        // This is now the newest bucket entry
        self.newest_entry = Some(node_id.key);

        // Get a node ref to return since this is new
        NodeRef::new(self.routing_table.clone(), entry, None)
    }

    /// Add an existing entry with a new node_id for this crypto kind
    pub(super) fn add_existing_entry(&mut self, node_id: TypedKey, entry: Arc<BucketEntry>) {
        assert_eq!(node_id.kind, self.kind);

        log_rtab!("Existing node added: {}", node_id);

        // Add existing entry
        entry.with_mut_inner(|e| e.add_node_id(node_id));
        self.entries.insert(node_id.key, entry);

        // This is now the newest bucket entry
        self.newest_entry = Some(node_id.key);

        // No need to return a noderef here because the noderef will already exist in the caller
    }

    /// Remove an entry with a node_id for this crypto kind from the bucket
    fn remove_entry(&mut self, node_id: &TypedKey) {
        log_rtab!("Node removed: {}:{}", self.kind, node_id);

        // Remove the entry
        self.entries.remove(node_id);

        // newest_entry is updated by kick_bucket()
    }

    pub(super) fn entry(&self, key: &TypedKey) -> Option<Arc<BucketEntry>> {
        self.entries.get(key).cloned()
    }

    pub(super) fn entries(&self) -> EntriesIter {
        self.entries.iter()
    }

    pub(super) fn kick(&mut self, bucket_depth: usize) -> Option<BTreeSet<TypedKey>> {
        // Get number of entries to attempt to purge from bucket
        let bucket_len = self.entries.len();

        // Don't bother kicking bucket unless it is full
        if bucket_len <= bucket_depth {
            return None;
        }

        // Try to purge the newest entries that overflow the bucket
        let mut dead_node_ids: BTreeSet<TypedKey> = BTreeSet::new();
        let mut extra_entries = bucket_len - bucket_depth;

        // Get the sorted list of entries by their kick order
        let mut sorted_entries: Vec<(TypedKey, Arc<BucketEntry>)> = self
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

        self.newest_entry = None;
        for entry in sorted_entries {
            // If we're not evicting more entries, exit, noting this may be the newest entry
            if extra_entries == 0 {
                // The first 'live' entry we find is our newest entry
                if self.newest_entry.is_none() {
                    self.newest_entry = Some(entry.0);
                }
                break;
            }
            extra_entries -= 1;

            // if this entry has references we can't drop it yet
            if entry.1.ref_count.load(Ordering::Acquire) > 0 {
                // The first 'live' entry we fine is our newest entry
                if self.newest_entry.is_none() {
                    self.newest_entry = Some(entry.0);
                }
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
