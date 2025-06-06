mod network_interfaces_task;
mod update_network_class_task;
mod upnp_task;

use super::*;

impl Network {
    pub fn setup_tasks(&self) {
        // Set update network class tick task
        let this = self.clone();
        self.update_network_class_task.set_routine(move |s, l, t| {
            let this = this.clone();
            Box::pin(async move {
                this.update_network_class_task_routine(s, Timestamp::new(l), Timestamp::new(t))
                    .await
            })
        });

        // Set network interfaces tick task
        let this = self.clone();
        self.network_interfaces_task.set_routine(move |s, l, t| {
            let this = this.clone();
            Box::pin(async move {
                this.network_interfaces_task_routine(s, Timestamp::new(l), Timestamp::new(t))
                    .await
            })
        });

        // Set upnp tick task
        {
            let this = self.clone();
            self.upnp_task.set_routine(move |s, l, t| {
                let this = this.clone();
                Box::pin(async move {
                    this.upnp_task_routine(s, Timestamp::new(l), Timestamp::new(t))
                        .await
                })
            });
        }
    }

    // Determine if we need to check for public dialinfo
    fn wants_update_network_class_tick(&self) -> bool {
        let routing_table = self.routing_table();

        let public_internet_network_class =
            routing_table.get_network_class(RoutingDomain::PublicInternet);

        let needs_update_network_class = self.needs_update_network_class();
        if needs_update_network_class
            || public_internet_network_class == NetworkClass::Invalid
            || (public_internet_network_class == NetworkClass::OutboundOnly
                && self.inner.lock().next_outbound_only_dial_info_check <= Timestamp::now())
        {
            let live_entry_counts = routing_table.cached_live_entry_counts();

            // Bootstrap needs to have gotten us connectivity nodes
            let mut has_at_least_two = true;
            for ck in VALID_CRYPTO_KINDS {
                if live_entry_counts
                    .connectivity_capabilities
                    .get(&(RoutingDomain::PublicInternet, ck))
                    .copied()
                    .unwrap_or_default()
                    < MIN_BOOTSTRAP_CONNECTIVITY_PEERS
                {
                    has_at_least_two = false;
                    break;
                }
            }

            has_at_least_two
        } else {
            false
        }
    }

    #[instrument(level = "trace", target = "net", name = "Network::tick", skip_all, err)]
    pub async fn tick(&self) -> EyreResult<()> {
        let Ok(_guard) = self.startup_lock.enter() else {
            veilid_log!(self debug "ignoring due to not started up");
            return Ok(());
        };

        // Ignore this tick if we need to restart
        if self.needs_restart() {
            return Ok(());
        }

        let (detect_address_changes, upnp) = {
            let config = self.network_manager().config();
            let c = config.get();
            (c.network.detect_address_changes, c.network.upnp)
        };

        // If we need to figure out our network class, tick the task for it
        if detect_address_changes {
            // Check our network interfaces to see if they have changed
            self.network_interfaces_task.tick().await?;

            if self.wants_update_network_class_tick() {
                self.update_network_class_task.tick().await?;
            }
        }

        // If we need to tick upnp, do it
        if upnp {
            self.upnp_task.tick().await?;
        }

        Ok(())
    }

    pub async fn cancel_tasks(&self) {
        veilid_log!(self debug "stopping upnp task");
        if let Err(e) = self.upnp_task.stop().await {
            veilid_log!(self warn "upnp_task not stopped: {}", e);
        }
        veilid_log!(self debug "stopping network interfaces task");
        if let Err(e) = self.network_interfaces_task.stop().await {
            veilid_log!(self warn "network_interfaces_task not stopped: {}", e);
        }
        veilid_log!(self debug "stopping update network class task");
        if let Err(e) = self.update_network_class_task.stop().await {
            veilid_log!(self warn "update_network_class_task not stopped: {}", e);
        }
    }
}
