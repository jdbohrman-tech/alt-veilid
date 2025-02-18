pub mod rolling_transfers;

use super::*;

impl NetworkManager {
    pub fn setup_tasks(&self) {
        // Set rolling transfers tick task
        impl_setup_task!(
            self,
            Self,
            rolling_transfers_task,
            rolling_transfers_task_routine
        );

        // Set address filter task
        {
            let registry = self.registry();
            self.address_filter_task.set_routine(move |s, l, t| {
                let registry = registry.clone();
                Box::pin(async move {
                    registry
                        .network_manager()
                        .address_filter()
                        .address_filter_task_routine(s, Timestamp::new(l), Timestamp::new(t))
                        .await
                })
            });
        }
    }

    #[instrument(level = "trace", name = "NetworkManager::tick", skip_all, err)]
    pub async fn tick(&self) -> EyreResult<()> {
        let net = self.net();
        let receipt_manager = self.receipt_manager();

        // Run the rolling transfers task
        self.rolling_transfers_task.tick().await?;

        // Run the address filter task
        self.address_filter_task.tick().await?;

        // Run the low level network tick
        net.tick().await?;

        // Run the receipt manager tick
        receipt_manager.tick().await?;

        // Purge the client allowlist
        self.purge_client_allowlist();

        Ok(())
    }

    pub async fn cancel_tasks(&self) {
        veilid_log!(self debug "stopping receipt manager tasks");
        let receipt_manager = self.receipt_manager();
        receipt_manager.cancel_tasks().await;

        let net = self.net();
        net.cancel_tasks().await;

        veilid_log!(self debug "stopping rolling transfers task");
        if let Err(e) = self.rolling_transfers_task.stop().await {
            veilid_log!(self warn "rolling_transfers_task not stopped: {}", e);
        }

        veilid_log!(self debug "stopping address filter task");
        if let Err(e) = self.address_filter_task.stop().await {
            veilid_log!(self warn "address_filter_task not stopped: {}", e);
        }
    }
}
