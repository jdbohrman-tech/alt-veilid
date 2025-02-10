use super::*;

impl Network {
    #[instrument(parent = None, level = "trace", target = "net", skip_all, err)]
    pub(super) async fn upnp_task_routine(
        &self,
        _stop_token: StopToken,
        _l: Timestamp,
        _t: Timestamp,
    ) -> EyreResult<()> {
        if !self.igd_manager.tick().await? {
            info!("upnp failed, restarting local network");
            let mut inner = self.inner.lock();
            inner.network_needs_restart = true;
        }

        Ok(())
    }
}
