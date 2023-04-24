pub mod flush_record_stores;

use super::*;

impl StorageManager {
    pub(crate) fn setup_tasks(&self) {
        // Set rolling transfers tick task
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
    }

    pub async fn tick(&self) -> EyreResult<()> {
        // Run the rolling transfers task
        self.unlocked_inner.flush_record_stores_task.tick().await?;

        Ok(())
    }

    pub(crate) async fn cancel_tasks(&self) {
        debug!("stopping flush record stores task");
        if let Err(e) = self.unlocked_inner.flush_record_stores_task.stop().await {
            warn!("flush_record_stores_task not stopped: {}", e);
        }
    }
}
