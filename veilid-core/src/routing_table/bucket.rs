use super::*;
use core::sync::atomic::Ordering;
use rkyv::{Archive as RkyvArchive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

pub struct Bucket {
    routing_table: RoutingTable,
    entries: BTreeMap<DHTKey, Arc<BucketEntry>>,
    newest_entry: Option<DHTKey>,
}
pub(super) type EntriesIter<'a> = alloc::collections::btree_map::Iter<'a, DHTKey, Arc<BucketEntry>>;

#[derive(Debug, RkyvArchive, RkyvSerialize, RkyvDeserialize)]
#[archive_attr(repr(C), derive(CheckBytes))]
struct BucketEntryData {
    key: DHTKey,
    value: Vec<u8>,
}

#[derive(Debug, RkyvArchive, RkyvSerialize, RkyvDeserialize)]
#[archive_attr(repr(C), derive(CheckBytes))]
struct BucketData {
    entries: Vec<BucketEntryData>,
    newest_entry: Option<DHTKey>,
}

fn state_ordering(state: BucketEntryState) -> usize {
    match state {
        BucketEntryState::Dead => 0,
        BucketEntryState::Unreliable => 1,
        BucketEntryState::Reliable => 2,
    }
}

impl Bucket {
    pub fn new(routing_table: RoutingTable) -> Self {
        Self {
            routing_table,
            entries: BTreeMap::new(),
            newest_entry: None,
        }
    }

    pub(super) fn load_bucket(&mut self, data: Vec<u8>) -> EyreResult<()> {
        let bucket_data: BucketData = from_rkyv(data)?;

        for e in bucket_data.entries {
            let entryinner = from_rkyv(e.value).wrap_err("failed to deserialize bucket entry")?;
            self.entries
                .insert(e.key, Arc::new(BucketEntry::new_with_inner(entryinner)));
        }

        self.newest_entry = bucket_data.newest_entry;

        Ok(())
    }
    pub(super) fn save_bucket(&self) -> EyreResult<Vec<u8>> {
        let mut entries = Vec::new();
        for (k, v) in &self.entries {
            let entry_bytes = v.with_inner(|e| to_rkyv(e))?;
            entries.push(BucketEntryData {
                key: *k,
                value: entry_bytes,
            });
        }
        let bucket_data = BucketData {
            entries,
            newest_entry: self.newest_entry.clone(),
        };
        let out = to_rkyv(&bucket_data)?;
        Ok(out)
    }

    pub(super) fn add_entry(&mut self, node_id: DHTKey) -> NodeRef {
        log_rtab!("Node added: {}", node_id.encode());

        // Add new entry
        self.entries.insert(node_id, Arc::new(BucketEntry::new()));

        // This is now the newest bucket entry
        self.newest_entry = Some(node_id);

        // Get a node ref to return
        let entry = self.entries.get(&node_id).unwrap().clone();
        NodeRef::new(self.routing_table.clone(), node_id, entry, None)
    }

    pub(super) fn remove_entry(&mut self, node_id: &DHTKey) {
        log_rtab!("Node removed: {}", node_id);

        // Remove the entry
        self.entries.remove(node_id);

        // newest_entry is updated by kick_bucket()
    }

    pub(super) fn entry(&self, key: &DHTKey) -> Option<Arc<BucketEntry>> {
        self.entries.get(key).cloned()
    }

    pub(super) fn entries(&self) -> EntriesIter {
        self.entries.iter()
    }

    pub(super) fn kick(&mut self, bucket_depth: usize) -> Option<BTreeSet<DHTKey>> {
        // Get number of entries to attempt to purge from bucket
        let bucket_len = self.entries.len();

        // Don't bother kicking bucket unless it is full
        if bucket_len <= bucket_depth {
            return None;
        }

        // Try to purge the newest entries that overflow the bucket
        let mut dead_node_ids: BTreeSet<DHTKey> = BTreeSet::new();
        let mut extra_entries = bucket_len - bucket_depth;

        // Get the sorted list of entries by their kick order
        let mut sorted_entries: Vec<(DHTKey, Arc<BucketEntry>)> = self
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
