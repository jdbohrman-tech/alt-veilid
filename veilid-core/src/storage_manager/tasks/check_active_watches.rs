use super::*;

impl StorageManager {
    // Check if watches either have dead nodes or if the watch has expired
    #[instrument(level = "trace", skip(self), err)]
    pub(super) async fn check_active_watches_task_routine(
        self,
        stop_token: StopToken,
        _last_ts: Timestamp,
        _cur_ts: Timestamp,
    ) -> EyreResult<()> {
        {
            let mut inner = self.inner.lock().await;
            let Some(routing_table) = inner.opt_routing_table.clone() else {
                return Ok(());
            };
            let rss = routing_table.route_spec_store();

            let opt_update_callback = inner.update_callback.clone();

            let cur_ts = get_aligned_timestamp();
            for (k, v) in inner.opened_records.iter_mut() {
                // If no active watch, then skip this
                let Some(active_watch) = v.active_watch() else {
                    continue;
                };

                // See if the active watch's node is dead
                let mut is_dead = false;
                if matches!(
                    active_watch.watch_node.state(cur_ts),
                    BucketEntryState::Dead
                ) {
                    // Watched node is dead
                    is_dead = true;
                }

                // See if the private route we're using is dead
                if !is_dead {
                    if let Some(value_changed_route) = active_watch.opt_value_changed_route {
                        if rss.get_route_id_for_key(&value_changed_route).is_none() {
                            // Route we would receive value changes on is dead
                            is_dead = true;
                        }
                    }
                }
                // See if the watch is expired
                if !is_dead && active_watch.expiration_ts <= cur_ts {
                    // Watch has expired
                    is_dead = true;
                }

                if is_dead {
                    v.clear_active_watch();

                    if let Some(update_callback) = opt_update_callback.clone() {
                        // Send valuechange with dead count and no subkeys
                        update_callback(VeilidUpdate::ValueChange(Box::new(VeilidValueChange {
                            key: *k,
                            subkeys: ValueSubkeyRangeSet::new(),
                            count: 0,
                            value: ValueData::default(),
                        })));
                    }
                }
            }
        }

        Ok(())
    }
}
