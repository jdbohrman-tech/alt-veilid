pub mod flush_record_stores;
pub mod offline_subkey_writes;

use super::*;

impl StorageManager {
    pub(crate) fn setup_tasks(&self) {
        // Set flush records tick task
        debug!("starting flush record stores task");
        {
            let this = self.clone();
            self.unlocked_inner
                .flush_record_stores_task
                .set_routine(move |s, l, t| {
                    Box::pin(
                        this.clone()
                            .flush_record_stores_task_routine(
                                s,
                                Timestamp::new(l),
                                Timestamp::new(t),
                            )
                            .instrument(trace_span!(
                                parent: None,
                                "StorageManager flush record stores task routine"
                            )),
                    )
                });
        }
        // Set offline subkey writes tick task
        debug!("starting offline subkey writes task");
        {
            let this = self.clone();
            self.unlocked_inner
                .offline_subkey_writes_task
                .set_routine(move |s, l, t| {
                    Box::pin(
                        this.clone()
                            .offline_subkey_writes_task_routine(
                                s,
                                Timestamp::new(l),
                                Timestamp::new(t),
                            )
                            .instrument(trace_span!(
                                parent: None,
                                "StorageManager offline subkey writes task routine"
                            )),
                    )
                });
        }
    }

    pub async fn tick(&self) -> EyreResult<()> {
        // Run the rolling transfers task
        self.unlocked_inner.flush_record_stores_task.tick().await?;

        // Run offline subkey writes task if there's work to be done
        if self.online_writes_ready().await?.is_some() && self.has_offline_subkey_writes().await? {
            self.unlocked_inner
                .offline_subkey_writes_task
                .tick()
                .await?;
        }
        Ok(())
    }

    pub(crate) async fn cancel_tasks(&self) {
        debug!("stopping flush record stores task");
        if let Err(e) = self.unlocked_inner.flush_record_stores_task.stop().await {
            warn!("flush_record_stores_task not stopped: {}", e);
        }
        debug!("stopping offline subkey writes task");
        if let Err(e) = self.unlocked_inner.offline_subkey_writes_task.stop().await {
            warn!("offline_subkey_writes_task not stopped: {}", e);
        }
    }
}
