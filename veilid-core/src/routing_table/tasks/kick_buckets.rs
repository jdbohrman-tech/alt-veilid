use super::*;

/// How many 'reliable' nodes closest to our own node id to keep
const KEEP_N_CLOSEST_RELIABLE_PEERS_COUNT: usize = 16;

/// How many 'unreliable' nodes closest to our own node id to keep
const KEEP_N_CLOSEST_UNRELIABLE_PEERS_COUNT: usize = 8;

impl RoutingTable {
    // Kick the queued buckets in the routing table to free dead nodes if necessary
    // Attempts to keep the size of the routing table down to the bucket depth
    #[instrument(level = "trace", skip(self), err)]
    pub async fn kick_buckets_task_routine(
        &self,
        _stop_token: StopToken,
        _last_ts: Timestamp,
        cur_ts: Timestamp,
    ) -> EyreResult<()> {
        let crypto = self.crypto();
        let kick_queue: Vec<BucketIndex> = core::mem::take(&mut *self.kick_queue.lock())
            .into_iter()
            .collect();
        let mut inner = self.inner.write();

        // Get our exempt nodes for each crypto kind
        let mut exempt_peers_by_kind = BTreeMap::<CryptoKind, BTreeSet<NodeId>>::new();

        for kind in VALID_CRYPTO_KINDS {
            let our_node_id = self.node_id(kind);
            let Some(buckets) = inner.buckets.get(&kind) else {
                continue;
            };
            let sort = make_closest_node_id_sort(&crypto, our_node_id);

            let mut closest_peers = BTreeSet::<NodeId>::new();
            let mut closest_unreliable_count = 0usize;
            let mut closest_reliable_count = 0usize;

            // Iterate buckets backward, sort entries by closest distance first
            'outer: for bucket in buckets.iter().rev() {
                let mut entries = bucket.entries().collect::<Vec<_>>();
                entries.sort_by(|a, b| sort(a.0, b.0));
                for (key, entry) in entries {
                    // See if this entry is a distance-metric capability node
                    // If not, disqualify it from this closest_nodes list
                    if !entry.with(&inner, |_rti, e| {
                        e.has_any_capabilities(
                            RoutingDomain::PublicInternet,
                            DISTANCE_METRIC_CAPABILITIES,
                        )
                    }) {
                        continue;
                    }

                    let state = entry.with(&inner, |_rti, e| e.state(cur_ts));
                    match state {
                        BucketEntryState::Dead | BucketEntryState::Punished => {
                            // Do nothing with dead  or punished entries
                        }
                        BucketEntryState::Unreliable => {
                            // Add to closest unreliable nodes list
                            if closest_unreliable_count < KEEP_N_CLOSEST_UNRELIABLE_PEERS_COUNT {
                                closest_peers.insert(*key);
                                closest_unreliable_count += 1;
                            }
                        }
                        BucketEntryState::Reliable => {
                            // Add to closest reliable nodes list
                            if closest_reliable_count < KEEP_N_CLOSEST_RELIABLE_PEERS_COUNT {
                                closest_peers.insert(*key);
                                closest_reliable_count += 1;
                            }
                        }
                    }
                    if closest_unreliable_count == KEEP_N_CLOSEST_UNRELIABLE_PEERS_COUNT
                        && closest_reliable_count == KEEP_N_CLOSEST_RELIABLE_PEERS_COUNT
                    {
                        break 'outer;
                    }
                }
            }

            exempt_peers_by_kind.insert(kind, closest_peers);
        }

        for bucket_index in kick_queue {
            inner.kick_bucket(bucket_index, &exempt_peers_by_kind[&bucket_index.0]);
        }
        Ok(())
    }
}
