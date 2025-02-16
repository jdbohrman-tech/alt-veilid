use super::*;

impl Network {
    #[instrument(level = "trace", target = "net", skip_all, err)]
    pub(super) async fn network_interfaces_task_routine(
        &self,
        stop_token: StopToken,
        _l: Timestamp,
        _t: Timestamp,
    ) -> EyreResult<()> {
        // Network lock ensures only one task operating on the low level network state
        // can happen at the same time. Try lock is here to give preference to other longer
        // running processes like update_network_class_task.
        let _guard = match self.network_task_lock.try_lock() {
            Ok(v) => v,
            Err(_) => {
                // If we can't get the lock right now, then
                return Ok(());
            }
        };

        self.update_network_state(stop_token).await?;

        Ok(())
    }

    // See if our interface addresses have changed, if so redo public dial info if necessary
    async fn update_network_state(&self, _stop_token: StopToken) -> EyreResult<bool> {
        let mut local_network_changed = false;
        let mut public_internet_changed = false;

        let last_network_state = self.last_network_state();
        let new_network_state = match self.make_network_state().await {
            Ok(v) => v,
            Err(e) => {
                log_net!(debug "Skipping network state update: {}", e);
                return Ok(false);
            }
        };

        if last_network_state.is_none() || new_network_state != last_network_state.unwrap() {
            // Save new network state
            {
                let mut inner = self.inner.lock();
                inner.network_state = Some(new_network_state.clone());
            }

            // network state has changed
            let routing_table = self.routing_table();

            let mut editor_local_network = routing_table.edit_local_network_routing_domain();
            editor_local_network.set_local_networks(new_network_state.local_networks);
            editor_local_network.clear_dial_info_details(None, None);

            let mut editor_public_internet = routing_table.edit_public_internet_routing_domain();

            // Update protocols
            self.register_all_dial_info(&mut editor_public_internet, &mut editor_local_network)
                .await?;

            local_network_changed = editor_local_network.commit(true).await;
            public_internet_changed = editor_public_internet.commit(true).await;

            // Update local network now
            if local_network_changed {
                editor_local_network.publish();
            }
        }

        // If any of the new addresses were PublicInternet addresses, re-run public dial info check
        if public_internet_changed {
            self.set_needs_public_dial_info_check(None);
        }

        Ok(local_network_changed || public_internet_changed)
    }
}
