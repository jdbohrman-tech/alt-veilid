use crate::{network_manager::StartupDisposition, *};
use routing_table::RoutingTableHealth;

impl_veilid_log_facility!("attach");

#[derive(Debug, Clone)]
pub struct AttachmentManagerStartupContext {
    pub startup_lock: Arc<StartupLock>,
}
impl AttachmentManagerStartupContext {
    pub fn new() -> Self {
        Self {
            startup_lock: Arc::new(StartupLock::new()),
        }
    }
}
impl Default for AttachmentManagerStartupContext {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
struct AttachmentManagerInner {
    last_attachment_state: AttachmentState,
    last_routing_table_health: Option<RoutingTableHealth>,
    maintain_peers: bool,
    started_ts: Timestamp,
    attach_ts: Option<Timestamp>,
    attachment_maintainer_jh: Option<MustJoinHandle<()>>,
}

#[derive(Debug)]
pub struct AttachmentManager {
    registry: VeilidComponentRegistry,
    inner: Mutex<AttachmentManagerInner>,
    startup_context: AttachmentManagerStartupContext,
}

impl_veilid_component!(AttachmentManager);

impl AttachmentManager {
    fn new_inner() -> AttachmentManagerInner {
        AttachmentManagerInner {
            last_attachment_state: AttachmentState::Detached,
            last_routing_table_health: None,
            maintain_peers: false,
            started_ts: Timestamp::now(),
            attach_ts: None,
            attachment_maintainer_jh: None,
        }
    }
    pub fn new(
        registry: VeilidComponentRegistry,
        startup_context: AttachmentManagerStartupContext,
    ) -> Self {
        Self {
            registry,
            inner: Mutex::new(Self::new_inner()),
            startup_context,
        }
    }

    pub fn is_attached(&self) -> bool {
        let s = self.inner.lock().last_attachment_state;
        !matches!(s, AttachmentState::Detached | AttachmentState::Detaching)
    }

    #[allow(dead_code)]
    pub fn is_detached(&self) -> bool {
        let s = self.inner.lock().last_attachment_state;
        matches!(s, AttachmentState::Detached)
    }

    #[allow(dead_code)]
    pub fn get_attach_timestamp(&self) -> Option<Timestamp> {
        self.inner.lock().attach_ts
    }

    fn translate_routing_table_health(
        health: &RoutingTableHealth,
        config: &VeilidConfigRoutingTable,
    ) -> AttachmentState {
        if health.reliable_entry_count
            >= TryInto::<usize>::try_into(config.limit_over_attached).unwrap()
        {
            return AttachmentState::OverAttached;
        }
        if health.reliable_entry_count
            >= TryInto::<usize>::try_into(config.limit_fully_attached).unwrap()
        {
            return AttachmentState::FullyAttached;
        }
        if health.reliable_entry_count
            >= TryInto::<usize>::try_into(config.limit_attached_strong).unwrap()
        {
            return AttachmentState::AttachedStrong;
        }
        if health.reliable_entry_count
            >= TryInto::<usize>::try_into(config.limit_attached_good).unwrap()
        {
            return AttachmentState::AttachedGood;
        }
        if health.reliable_entry_count
            >= TryInto::<usize>::try_into(config.limit_attached_weak).unwrap()
            || health.unreliable_entry_count
                >= TryInto::<usize>::try_into(config.limit_attached_weak).unwrap()
        {
            return AttachmentState::AttachedWeak;
        }
        AttachmentState::Attaching
    }

    /// Update attachment and network readiness state
    /// and possibly send a VeilidUpdate::Attachment.
    fn update_attachment(&self) {
        // update the routing table health
        let routing_table = self.network_manager().routing_table();
        let health = routing_table.get_routing_table_health();
        let opt_update = {
            let mut inner = self.inner.lock();

            // Check if the routing table health is different
            if let Some(last_routing_table_health) = &inner.last_routing_table_health {
                // If things are the same, just return
                if last_routing_table_health == &health {
                    return;
                }
            }

            // Swap in new health numbers
            let opt_previous_health = inner.last_routing_table_health.take();
            inner.last_routing_table_health = Some(health.clone());

            // Calculate new attachment state
            let config = self.config();
            let routing_table_config = &config.get().network.routing_table;
            let previous_attachment_state = inner.last_attachment_state;
            inner.last_attachment_state =
                AttachmentManager::translate_routing_table_health(&health, routing_table_config);

            // Send update if one of:
            // * the attachment state has changed
            // * routing domain readiness has changed
            // * this is our first routing table health check
            let send_update = previous_attachment_state != inner.last_attachment_state
                || opt_previous_health
                    .map(|x| {
                        x.public_internet_ready != health.public_internet_ready
                            || x.local_network_ready != health.local_network_ready
                    })
                    .unwrap_or(true);
            if send_update {
                Some(Self::get_veilid_state_inner(&inner))
            } else {
                None
            }
        };

        // Send the update outside of the lock
        if let Some(update) = opt_update {
            (self.update_callback())(VeilidUpdate::Attachment(update));
        }
    }

    fn update_attaching_detaching_state(&self, state: AttachmentState) {
        let uptime;
        let attached_uptime;
        {
            let mut inner = self.inner.lock();

            // Clear routing table health so when we start measuring it we start from scratch
            inner.last_routing_table_health = None;

            // Set attachment state directly
            inner.last_attachment_state = state;

            // Set timestamps
            if state == AttachmentState::Attaching {
                inner.attach_ts = Some(Timestamp::now());
            } else if state == AttachmentState::Detached {
                inner.attach_ts = None;
            } else if state == AttachmentState::Detaching {
                // ok
            } else {
                unreachable!("don't use this for attached states, use update_attachment()");
            }

            let now = Timestamp::now();
            uptime = now - inner.started_ts;
            attached_uptime = inner.attach_ts.map(|ts| now - ts);
        };

        // Send update
        (self.update_callback())(VeilidUpdate::Attachment(Box::new(VeilidStateAttachment {
            state,
            public_internet_ready: false,
            local_network_ready: false,
            uptime,
            attached_uptime,
        })))
    }

    async fn startup(&self) -> EyreResult<StartupDisposition> {
        let guard = self.startup_context.startup_lock.startup()?;

        let rpc_processor = self.rpc_processor();
        let network_manager = self.network_manager();

        // Startup network manager
        network_manager.startup().await?;

        // Startup rpc processor
        if let Err(e) = rpc_processor.startup().await {
            network_manager.shutdown().await;
            return Err(e);
        }

        // Startup routing table
        let routing_table = self.routing_table();
        if let Err(e) = routing_table.startup().await {
            rpc_processor.shutdown().await;
            network_manager.shutdown().await;
            return Err(e);
        }

        // Startup successful
        guard.success();

        // Inform api clients that things have changed
        veilid_log!(self trace "sending network state update to api clients");
        network_manager.send_network_update();

        Ok(StartupDisposition::Success)
    }

    async fn shutdown(&self) {
        let guard = self
            .startup_context
            .startup_lock
            .shutdown()
            .await
            .expect("should be started up");

        let routing_table = self.routing_table();
        let rpc_processor = self.rpc_processor();
        let network_manager = self.network_manager();

        // Shutdown RoutingTable
        routing_table.shutdown().await;

        // Shutdown NetworkManager
        network_manager.shutdown().await;

        // Shutdown RPCProcessor
        rpc_processor.shutdown().await;

        // Shutdown successful
        guard.success();

        // send update
        veilid_log!(self debug "sending network state update to api clients");
        network_manager.send_network_update();
    }

    async fn tick(&self) -> EyreResult<()> {
        // Run the network manager tick
        let network_manager = self.network_manager();
        network_manager.tick().await?;

        // Run the routing table tick
        let routing_table = self.routing_table();
        routing_table.tick().await?;

        Ok(())
    }

    #[instrument(parent = None, level = "debug", skip_all)]
    async fn attachment_maintainer(&self) {
        veilid_log!(self debug "attachment starting");
        self.update_attaching_detaching_state(AttachmentState::Attaching);

        let network_manager = self.network_manager();

        let mut restart;
        let mut restart_delay;
        while self.inner.lock().maintain_peers {
            restart = false;
            restart_delay = 1;

            match self.startup().await {
                Err(err) => {
                    error!("attachment startup failed: {}", err);
                    restart = true;
                }
                Ok(StartupDisposition::BindRetry) => {
                    veilid_log!(self info "waiting for network to bind...");
                    restart = true;
                    restart_delay = 10;
                }
                Ok(StartupDisposition::Success) => {
                    veilid_log!(self debug "started maintaining peers");

                    while self.inner.lock().maintain_peers {
                        // tick network manager
                        let next_tick_ts = get_timestamp() + 1_000_000u64;
                        if let Err(err) = self.tick().await {
                            error!("Error in attachment tick: {}", err);
                            self.inner.lock().maintain_peers = false;
                            restart = true;
                            break;
                        }

                        // see if we need to restart the network
                        if network_manager.network_needs_restart() {
                            veilid_log!(self info "Restarting network");
                            restart = true;
                            break;
                        }

                        // Update attachment and network readiness state
                        // and possibly send a VeilidUpdate::Attachment
                        self.update_attachment();

                        // sleep should be at the end in case maintain_peers changes state
                        let wait_duration = next_tick_ts
                            .saturating_sub(get_timestamp())
                            .clamp(0, 1_000_000u64);
                        sleep((wait_duration / 1_000) as u32).await;
                    }
                    veilid_log!(self debug "stopped maintaining peers");

                    if !restart {
                        self.update_attaching_detaching_state(AttachmentState::Detaching);
                        veilid_log!(self debug "attachment stopping");
                    }

                    veilid_log!(self debug "shutting down attachment");
                    self.shutdown().await;
                }
            }

            if !restart {
                break;
            }

            veilid_log!(self debug "completely restarting attachment");

            // chill out for a second first, give network stack time to settle out
            for _ in 0..restart_delay {
                if !self.inner.lock().maintain_peers {
                    break;
                }
                sleep(1000).await;
            }
        }

        self.update_attaching_detaching_state(AttachmentState::Detached);
        veilid_log!(self debug "attachment stopped");
    }

    #[instrument(level = "debug", skip_all, err)]
    pub async fn init_async(&self) -> EyreResult<()> {
        Ok(())
    }

    #[instrument(level = "debug", skip_all, err)]
    pub async fn post_init_async(&self) -> EyreResult<()> {
        Ok(())
    }

    #[instrument(level = "debug", skip_all)]
    pub async fn pre_terminate_async(&self) {
        // Ensure we detached
        self.detach().await;
    }

    #[instrument(level = "debug", skip_all)]
    pub async fn terminate_async(&self) {}

    #[instrument(level = "trace", skip_all)]
    pub async fn attach(&self) -> bool {
        // Create long-running connection maintenance routine
        let mut inner = self.inner.lock();
        if inner.attachment_maintainer_jh.is_some() {
            return false;
        }
        inner.maintain_peers = true;
        let registry = self.registry();
        inner.attachment_maintainer_jh = Some(spawn("attachment maintainer", async move {
            let this = registry.attachment_manager();
            this.attachment_maintainer().await;
        }));

        true
    }

    #[instrument(level = "trace", skip_all)]
    pub async fn detach(&self) -> bool {
        let attachment_maintainer_jh = {
            let mut inner = self.inner.lock();
            let attachment_maintainer_jh = inner.attachment_maintainer_jh.take();
            if attachment_maintainer_jh.is_some() {
                // Terminate long-running connection maintenance routine
                inner.maintain_peers = false;
            }
            attachment_maintainer_jh
        };
        if let Some(jh) = attachment_maintainer_jh {
            jh.await;
            true
        } else {
            false
        }
    }

    // pub fn get_attachment_state(&self) -> AttachmentState {
    //     self.inner.lock().last_attachment_state
    // }

    fn get_veilid_state_inner(inner: &AttachmentManagerInner) -> Box<VeilidStateAttachment> {
        let now = Timestamp::now();
        let uptime = now - inner.started_ts;
        let attached_uptime = inner.attach_ts.map(|ts| now - ts);

        Box::new(VeilidStateAttachment {
            state: inner.last_attachment_state,
            public_internet_ready: inner
                .last_routing_table_health
                .as_ref()
                .map(|x| x.public_internet_ready)
                .unwrap_or(false),
            local_network_ready: inner
                .last_routing_table_health
                .as_ref()
                .map(|x| x.local_network_ready)
                .unwrap_or(false),
            uptime,
            attached_uptime,
        })
    }

    pub fn get_veilid_state(&self) -> Box<VeilidStateAttachment> {
        let inner = self.inner.lock();
        Self::get_veilid_state_inner(&inner)
    }
}
