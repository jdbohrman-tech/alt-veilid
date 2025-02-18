pub mod check_active_watches;
pub mod check_watched_records;
pub mod flush_record_stores;
pub mod offline_subkey_writes;
pub mod send_value_changes;

use super::*;

impl StorageManager {
    pub(super) fn setup_tasks(&self) {
        // Set flush records tick task
        veilid_log!(self debug "starting flush record stores task");
        impl_setup_task!(
            self,
            Self,
            flush_record_stores_task,
            flush_record_stores_task_routine
        );

        // Set offline subkey writes tick task
        veilid_log!(self debug "starting offline subkey writes task");
        impl_setup_task!(
            self,
            Self,
            offline_subkey_writes_task,
            offline_subkey_writes_task_routine
        );

        // Set send value changes tick task
        veilid_log!(self debug "starting send value changes task");
        impl_setup_task!(
            self,
            Self,
            send_value_changes_task,
            send_value_changes_task_routine
        );

        // Set check active watches tick task
        veilid_log!(self debug "starting check active watches task");
        impl_setup_task!(
            self,
            Self,
            check_active_watches_task,
            check_active_watches_task_routine
        );

        // Set check watched records tick task
        veilid_log!(self debug "starting checked watched records task");
        impl_setup_task!(
            self,
            Self,
            check_watched_records_task,
            check_watched_records_task_routine
        );
    }

    #[instrument(parent = None, level = "trace", target = "stor", name = "StorageManager::tick", skip_all, err)]
    pub async fn tick(&self) -> EyreResult<()> {
        // Run the flush stores task
        self.flush_record_stores_task.tick().await?;

        // Check active watches
        self.check_active_watches_task.tick().await?;

        // Check watched records
        self.check_watched_records_task.tick().await?;

        // Run online-only tasks
        if self.dht_is_online() {
            // Run offline subkey writes task if there's work to be done
            if self.has_offline_subkey_writes().await {
                self.offline_subkey_writes_task.tick().await?;
            }

            // Send value changed notifications
            self.send_value_changes_task.tick().await?;
        }
        Ok(())
    }

    #[instrument(level = "trace", target = "stor", skip_all)]
    pub(super) async fn cancel_tasks(&self) {
        veilid_log!(self debug "stopping check watched records task");
        if let Err(e) = self.check_watched_records_task.stop().await {
            veilid_log!(self warn "check_watched_records_task not stopped: {}", e);
        }
        veilid_log!(self debug "stopping check active watches task");
        if let Err(e) = self.check_active_watches_task.stop().await {
            veilid_log!(self warn "check_active_watches_task not stopped: {}", e);
        }
        veilid_log!(self debug "stopping send value changes task");
        if let Err(e) = self.send_value_changes_task.stop().await {
            veilid_log!(self warn "send_value_changes_task not stopped: {}", e);
        }
        veilid_log!(self debug "stopping flush record stores task");
        if let Err(e) = self.flush_record_stores_task.stop().await {
            veilid_log!(self warn "flush_record_stores_task not stopped: {}", e);
        }
        veilid_log!(self debug "stopping offline subkey writes task");
        if let Err(e) = self.offline_subkey_writes_task.stop().await {
            veilid_log!(self warn "offline_subkey_writes_task not stopped: {}", e);
        }
    }
}
