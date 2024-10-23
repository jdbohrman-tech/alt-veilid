mod network_interfaces_task;
mod update_network_class_task;
mod upnp_task;

use super::*;

impl Network {
    pub fn setup_tasks(&self) {
        // Set update network class tick task
        {
            let this = self.clone();
            self.unlocked_inner
                .update_network_class_task
                .set_routine(move |s, l, t| {
                    Box::pin(this.clone().update_network_class_task_routine(
                        s,
                        Timestamp::new(l),
                        Timestamp::new(t),
                    ))
                });
        }
        // Set network interfaces tick task
        {
            let this = self.clone();
            self.unlocked_inner
                .network_interfaces_task
                .set_routine(move |s, l, t| {
                    Box::pin(this.clone().network_interfaces_task_routine(
                        s,
                        Timestamp::new(l),
                        Timestamp::new(t),
                    ))
                });
        }
        // Set upnp tick task
        {
            let this = self.clone();
            self.unlocked_inner.upnp_task.set_routine(move |s, l, t| {
                Box::pin(
                    this.clone()
                        .upnp_task_routine(s, Timestamp::new(l), Timestamp::new(t)),
                )
            });
        }
    }

    #[instrument(level = "trace", target = "net", name = "Network::tick", skip_all, err)]
    pub async fn tick(&self) -> EyreResult<()> {
        let Ok(_guard) = self.unlocked_inner.startup_lock.enter() else {
            log_net!(debug "ignoring due to not started up");
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
            self.unlocked_inner.network_interfaces_task.tick().await?;

            // Check our public dial info to see if it has changed
            let public_internet_network_class = self
                .routing_table()
                .get_network_class(RoutingDomain::PublicInternet)
                .unwrap_or(NetworkClass::Invalid);
            let needs_public_dial_info_check = self.needs_public_dial_info_check();
            if public_internet_network_class == NetworkClass::Invalid
                || needs_public_dial_info_check
            {
                let routing_table = self.routing_table();
                let rth = routing_table.get_routing_table_health();

                // We want at least two live entries per crypto kind before we start doing this (bootstrap)
                let mut has_at_least_two = true;
                for ck in VALID_CRYPTO_KINDS {
                    if rth
                        .live_entry_counts
                        .get(&(RoutingDomain::PublicInternet, ck))
                        .copied()
                        .unwrap_or_default()
                        < 2
                    {
                        has_at_least_two = false;
                        break;
                    }
                }

                if has_at_least_two {
                    self.unlocked_inner.update_network_class_task.tick().await?;
                }
            }
        }

        // If we need to tick upnp, do it
        if upnp {
            self.unlocked_inner.upnp_task.tick().await?;
        }

        Ok(())
    }
}
