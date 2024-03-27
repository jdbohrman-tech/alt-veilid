use crate::*;
use crypto::Crypto;
use network_manager::*;
use routing_table::*;
use storage_manager::*;

struct AttachmentManagerInner {
    last_attachment_state: AttachmentState,
    last_routing_table_health: Option<RoutingTableHealth>,
    maintain_peers: bool,
    attach_ts: Option<Timestamp>,
    update_callback: Option<UpdateCallback>,
    attachment_maintainer_jh: Option<MustJoinHandle<()>>,
}

struct AttachmentManagerUnlockedInner {
    config: VeilidConfig,
    network_manager: NetworkManager,
}

#[derive(Clone)]
pub(crate) struct AttachmentManager {
    inner: Arc<Mutex<AttachmentManagerInner>>,
    unlocked_inner: Arc<AttachmentManagerUnlockedInner>,
}

impl AttachmentManager {
    fn new_unlocked_inner(
        config: VeilidConfig,
        storage_manager: StorageManager,
        table_store: TableStore,
        #[cfg(feature = "unstable-blockstore")] block_store: BlockStore,
        crypto: Crypto,
    ) -> AttachmentManagerUnlockedInner {
        AttachmentManagerUnlockedInner {
            config: config.clone(),
            network_manager: NetworkManager::new(
                config,
                storage_manager,
                table_store,
                #[cfg(feature = "unstable-blockstore")]
                block_store,
                crypto,
            ),
        }
    }
    fn new_inner() -> AttachmentManagerInner {
        AttachmentManagerInner {
            last_attachment_state: AttachmentState::Detached,
            last_routing_table_health: None,
            maintain_peers: false,
            attach_ts: None,
            update_callback: None,
            attachment_maintainer_jh: None,
        }
    }
    pub fn new(
        config: VeilidConfig,
        storage_manager: StorageManager,
        table_store: TableStore,
        #[cfg(feature = "unstable-blockstore")] block_store: BlockStore,
        crypto: Crypto,
    ) -> Self {
        Self {
            inner: Arc::new(Mutex::new(Self::new_inner())),
            unlocked_inner: Arc::new(Self::new_unlocked_inner(
                config,
                storage_manager,
                table_store,
                #[cfg(feature = "unstable-blockstore")]
                block_store,
                crypto,
            )),
        }
    }

    pub fn config(&self) -> VeilidConfig {
        self.unlocked_inner.config.clone()
    }

    pub fn network_manager(&self) -> NetworkManager {
        self.unlocked_inner.network_manager.clone()
    }

    // pub fn is_attached(&self) -> bool {
    //     let s = self.inner.lock().last_attachment_state;
    //     !matches!(s, AttachmentState::Detached | AttachmentState::Detaching)
    // }
    // pub fn is_detached(&self) -> bool {
    //     let s = self.inner.lock().last_attachment_state;
    //     matches!(s, AttachmentState::Detached)
    // }

    // pub fn get_attach_timestamp(&self) -> Option<Timestamp> {
    //     self.inner.lock().attach_ts
    // }

    fn translate_routing_table_health(
        health: &RoutingTableHealth,
        config: &VeilidConfigRoutingTable,
    ) -> AttachmentState {
        if health.reliable_entry_count >= config.limit_over_attached.try_into().unwrap() {
            return AttachmentState::OverAttached;
        }
        if health.reliable_entry_count >= config.limit_fully_attached.try_into().unwrap() {
            return AttachmentState::FullyAttached;
        }
        if health.reliable_entry_count >= config.limit_attached_strong.try_into().unwrap() {
            return AttachmentState::AttachedStrong;
        }
        if health.reliable_entry_count >= config.limit_attached_good.try_into().unwrap() {
            return AttachmentState::AttachedGood;
        }
        if health.reliable_entry_count >= config.limit_attached_weak.try_into().unwrap()
            || health.unreliable_entry_count >= config.limit_attached_weak.try_into().unwrap()
        {
            return AttachmentState::AttachedWeak;
        }
        AttachmentState::Attaching
    }

    /// Update attachment and network readiness state
    /// and possibly send a VeilidUpdate::Attachment
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

            // If we don't have an update callback yet for some reason, just return now
            let Some(update_callback) = inner.update_callback.clone() else {
                return;
            };

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
                Some((update_callback, Self::get_veilid_state_inner(&inner)))
            } else {
                None
            }
        };

        // Send the update outside of the lock
        if let Some(update) = opt_update {
            (update.0)(VeilidUpdate::Attachment(update.1));
        }
    }

    fn update_attaching_detaching_state(&self, state: AttachmentState) {
        let update_callback = {
            let mut inner = self.inner.lock();

            // Clear routing table health so when we start measuring it we start from scratch
            inner.last_routing_table_health = None;

            // Set attachment state directly
            inner.last_attachment_state = state;

            // Set timestamps
            if state == AttachmentState::Attaching {
                inner.attach_ts = Some(get_aligned_timestamp());
            } else if state == AttachmentState::Detached {
                inner.attach_ts = None;
            } else if state == AttachmentState::Detaching {
                // ok
            } else {
                unreachable!("don't use this for attached states, use update_attachment()");
            }

            // Get callback
            inner.update_callback.clone()
        };

        // Send update
        if let Some(update_callback) = update_callback {
            update_callback(VeilidUpdate::Attachment(Box::new(VeilidStateAttachment {
                state,
                public_internet_ready: false,
                local_network_ready: false,
            })))
        }
    }

    #[instrument(level = "debug", skip(self))]
    async fn attachment_maintainer(self) {
        log_net!(debug "attachment starting");
        self.update_attaching_detaching_state(AttachmentState::Attaching);

        let netman = self.network_manager();

        let mut restart;
        loop {
            restart = false;
            if let Err(err) = netman.startup().await {
                error!("network startup failed: {}", err);
                netman.shutdown().await;
                break;
            }

            log_net!(debug "started maintaining peers");
            while self.inner.lock().maintain_peers {
                // tick network manager
                if let Err(err) = netman.tick().await {
                    error!("Error in network manager: {}", err);
                    self.inner.lock().maintain_peers = false;
                    restart = true;
                    break;
                }

                // see if we need to restart the network
                if netman.needs_restart() {
                    info!("Restarting network");
                    restart = true;
                    break;
                }

                // Update attachment and network readiness state
                // and possibly send a VeilidUpdate::Attachment
                self.update_attachment();

                // sleep should be at the end in case maintain_peers changes state
                sleep(1000).await;
            }
            log_net!(debug "stopped maintaining peers");

            if !restart {
                self.update_attaching_detaching_state(AttachmentState::Detaching);
                log_net!(debug "attachment stopping");
            }

            log_net!(debug "stopping network");
            netman.shutdown().await;

            if !restart {
                break;
            }

            log_net!(debug "completely restarting attachment");
            // chill out for a second first, give network stack time to settle out
            sleep(1000).await;
        }

        self.update_attaching_detaching_state(AttachmentState::Detached);
        log_net!(debug "attachment stopped");
    }

    #[instrument(level = "debug", skip_all, err)]
    pub async fn init(&self, update_callback: UpdateCallback) -> EyreResult<()> {
        {
            let mut inner = self.inner.lock();
            inner.update_callback = Some(update_callback.clone());
        }

        self.network_manager().init(update_callback).await?;

        Ok(())
    }

    #[instrument(level = "debug", skip(self))]
    pub async fn terminate(&self) {
        // Ensure we detached
        self.detach().await;
        self.network_manager().terminate().await;
        self.inner.lock().update_callback = None;
    }

    #[instrument(level = "trace", skip(self))]
    pub async fn attach(&self) -> bool {
        // Create long-running connection maintenance routine
        let mut inner = self.inner.lock();
        if inner.attachment_maintainer_jh.is_some() {
            return false;
        }
        inner.maintain_peers = true;
        inner.attachment_maintainer_jh = Some(spawn(self.clone().attachment_maintainer()));

        true
    }

    #[instrument(level = "trace", skip(self))]
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
        })
    }

    pub fn get_veilid_state(&self) -> Box<VeilidStateAttachment> {
        let inner = self.inner.lock();
        Self::get_veilid_state_inner(&inner)
    }
}
