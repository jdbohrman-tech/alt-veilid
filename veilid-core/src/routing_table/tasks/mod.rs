pub mod bootstrap;
pub mod closest_peers_refresh;
pub mod kick_buckets;
pub mod peer_minimum_refresh;
pub mod ping_validator;
pub mod private_route_management;
pub mod relay_management;
pub mod update_statistics;

use super::*;

impl_veilid_log_facility!("rtab");

impl RoutingTable {
    pub fn setup_tasks(&self) {
        // Set rolling transfers tick task
        impl_setup_task!(
            self,
            Self,
            rolling_transfers_task,
            rolling_transfers_task_routine
        );

        // Set update state stats tick task
        impl_setup_task!(
            self,
            Self,
            update_state_stats_task,
            update_state_stats_task_routine
        );

        // Set rolling answers tick task
        impl_setup_task!(
            self,
            Self,
            rolling_answers_task,
            rolling_answers_task_routine
        );

        // Set kick buckets tick task
        impl_setup_task!(self, Self, kick_buckets_task, kick_buckets_task_routine);

        // Set bootstrap tick task
        impl_setup_task!(self, Self, bootstrap_task, bootstrap_task_routine);

        // Set peer minimum refresh tick task
        impl_setup_task!(
            self,
            Self,
            peer_minimum_refresh_task,
            peer_minimum_refresh_task_routine
        );

        // Set closest peers refresh tick task
        impl_setup_task!(
            self,
            Self,
            closest_peers_refresh_task,
            closest_peers_refresh_task_routine
        );

        // Set ping validator PublicInternet tick task
        impl_setup_task!(
            self,
            Self,
            ping_validator_public_internet_task,
            ping_validator_public_internet_task_routine
        );

        // Set ping validator LocalNetwork tick task
        impl_setup_task!(
            self,
            Self,
            ping_validator_local_network_task,
            ping_validator_local_network_task_routine
        );

        // Set ping validator PublicInternet Relay tick task
        impl_setup_task!(
            self,
            Self,
            ping_validator_public_internet_relay_task,
            ping_validator_public_internet_relay_task_routine
        );

        // Set ping validator Active Watch tick task
        impl_setup_task!(
            self,
            Self,
            ping_validator_active_watch_task,
            ping_validator_active_watch_task_routine
        );

        // Set relay management tick task
        impl_setup_task!(
            self,
            Self,
            relay_management_task,
            relay_management_task_routine
        );

        // Set private route management tick task
        impl_setup_task!(
            self,
            Self,
            private_route_management_task,
            private_route_management_task_routine
        );
    }

    /// Ticks about once per second
    /// to run tick tasks which may run at slower tick rates as configured
    #[instrument(level = "trace", name = "RoutingTable::tick", skip_all, err)]
    pub async fn tick(&self) -> EyreResult<()> {
        // Don't tick if paused
        let opt_tick_guard = {
            let inner = self.inner.read();
            inner.critical_sections.try_lock_tag(LOCK_TAG_TICK)
        };
        let Some(_tick_guard) = opt_tick_guard else {
            return Ok(());
        };

        // Do rolling transfers every ROLLING_TRANSFERS_INTERVAL_SECS secs
        self.rolling_transfers_task.tick().await?;

        // Do state stats update every UPDATE_STATE_STATS_INTERVAL_SECS secs
        self.update_state_stats_task.tick().await?;

        // Do rolling answers every ROLLING_ANSWER_INTERVAL_SECS secs
        self.rolling_answers_task.tick().await?;

        // Kick buckets task
        let kick_bucket_queue_count = self.kick_queue.lock().len();
        if kick_bucket_queue_count > 0 {
            self.kick_buckets_task.tick().await?;
        }

        // Refresh entry counts
        let entry_counts = {
            let mut inner = self.inner.write();
            inner.refresh_cached_entry_counts()
        };

        // Only do the rest if the network has started
        if !self.network_manager().network_is_started() {
            return Ok(());
        }

        let min_peer_count = self
            .config()
            .with(|c| c.network.dht.min_peer_count as usize);

        // Figure out which tables need bootstrap or peer minimum refresh
        let mut needs_bootstrap = false;
        let mut needs_peer_minimum_refresh = false;
        for ck in VALID_CRYPTO_KINDS {
            let eckey = (RoutingDomain::PublicInternet, ck);
            let cnt = entry_counts.get(&eckey).copied().unwrap_or_default();
            if cnt < MIN_PUBLIC_INTERNET_ROUTING_DOMAIN_NODE_COUNT {
                needs_bootstrap = true;
            } else if cnt < min_peer_count {
                needs_peer_minimum_refresh = true;
            }
        }
        if needs_bootstrap {
            self.bootstrap_task.tick().await?;
        }
        if needs_peer_minimum_refresh {
            self.peer_minimum_refresh_task.tick().await?;
        }

        // Ping validate some nodes to groom the table
        self.ping_validator_public_internet_task.tick().await?;
        self.ping_validator_local_network_task.tick().await?;
        self.ping_validator_public_internet_relay_task
            .tick()
            .await?;
        self.ping_validator_active_watch_task.tick().await?;

        // Run the relay management task
        self.relay_management_task.tick().await?;

        // Get more nodes if we need to
        if !needs_bootstrap && !needs_peer_minimum_refresh {
            // Run closest peers refresh task
            self.closest_peers_refresh_task.tick().await?;
        }

        // Only perform these operations if we already have a published peer info
        if self
            .get_published_peer_info(RoutingDomain::PublicInternet)
            .is_some()
        {
            // Run the private route management task
            self.private_route_management_task.tick().await?;
        }

        Ok(())
    }
    pub async fn pause_tasks(&self) -> AsyncTagLockGuard<&'static str> {
        let critical_sections = self.inner.read().critical_sections.clone();
        critical_sections.lock_tag(LOCK_TAG_TICK).await
    }

    pub async fn cancel_tasks(&self) {
        // Cancel all tasks being ticked
        veilid_log!(self debug "stopping rolling transfers task");
        if let Err(e) = self.rolling_transfers_task.stop().await {
            veilid_log!(self warn "rolling_transfers_task not stopped: {}", e);
        }
        veilid_log!(self debug "stopping update state stats task");
        if let Err(e) = self.update_state_stats_task.stop().await {
            veilid_log!(self warn "update_state_stats_task not stopped: {}", e);
        }
        veilid_log!(self debug "stopping rolling answers task");
        if let Err(e) = self.rolling_answers_task.stop().await {
            veilid_log!(self warn "rolling_answers_task not stopped: {}", e);
        }
        veilid_log!(self debug "stopping kick buckets task");
        if let Err(e) = self.kick_buckets_task.stop().await {
            veilid_log!(self warn "kick_buckets_task not stopped: {}", e);
        }
        veilid_log!(self debug "stopping bootstrap task");
        if let Err(e) = self.bootstrap_task.stop().await {
            veilid_log!(self warn "bootstrap_task not stopped: {}", e);
        }
        veilid_log!(self debug "stopping peer minimum refresh task");
        if let Err(e) = self.peer_minimum_refresh_task.stop().await {
            veilid_log!(self warn "peer_minimum_refresh_task not stopped: {}", e);
        }

        veilid_log!(self debug "stopping ping_validator tasks");
        if let Err(e) = self.ping_validator_public_internet_task.stop().await {
            veilid_log!(self warn "ping_validator_public_internet_task not stopped: {}", e);
        }
        if let Err(e) = self.ping_validator_local_network_task.stop().await {
            veilid_log!(self warn "ping_validator_local_network_task not stopped: {}", e);
        }
        if let Err(e) = self.ping_validator_public_internet_relay_task.stop().await {
            veilid_log!(self warn
                "ping_validator_public_internet_relay_task not stopped: {}",
                e
            );
        }
        if let Err(e) = self.ping_validator_active_watch_task.stop().await {
            veilid_log!(self warn "ping_validator_active_watch_task not stopped: {}", e);
        }

        veilid_log!(self debug "stopping relay management task");
        if let Err(e) = self.relay_management_task.stop().await {
            veilid_log!(self warn "relay_management_task not stopped: {}", e);
        }
        veilid_log!(self debug "stopping private route management task");
        if let Err(e) = self.private_route_management_task.stop().await {
            veilid_log!(self warn "private_route_management_task not stopped: {}", e);
        }
        veilid_log!(self debug "stopping closest peers refresh task");
        if let Err(e) = self.closest_peers_refresh_task.stop().await {
            veilid_log!(self warn "closest_peers_refresh_task not stopped: {}", e);
        }
    }
}
