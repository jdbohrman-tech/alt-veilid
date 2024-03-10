use super::*;
use futures_util::*;

impl StorageManager {
    // Best-effort write subkeys to the network that were written offline
    #[instrument(level = "trace", skip(self), err)]
    pub(crate) async fn offline_subkey_writes_task_routine(
        self,
        stop_token: StopToken,
        _last_ts: Timestamp,
        _cur_ts: Timestamp,
    ) -> EyreResult<()> {
        let offline_subkey_writes = {
            let inner = self.lock().await?;
            inner.offline_subkey_writes.clone()
        };

        // make a safety selection that is conservative
        for (key, osw) in offline_subkey_writes {
            if poll!(stop_token.clone()).is_ready() {
                log_stor!(debug "Offline subkey writes cancelled.");
                break;
            }
            let Some(rpc_processor) = self.online_writes_ready().await? else {
                log_stor!(debug "Offline subkey writes stopped for network.");
                break;
            };
            for subkey in osw.subkeys.iter() {
                let get_result = {
                    let mut inner = self.lock().await?;
                    inner.handle_get_local_value(key, subkey, true).await
                };
                let Ok(get_result) = get_result else {
                    log_stor!(debug "Offline subkey write had no subkey result: {}:{}", key, subkey);
                    continue;
                };
                let Some(value) = get_result.opt_value else {
                    log_stor!(debug "Offline subkey write had no subkey value: {}:{}", key, subkey);
                    continue;
                };
                let Some(descriptor) = get_result.opt_descriptor else {
                    log_stor!(debug "Offline subkey write had no descriptor: {}:{}", key, subkey);
                    continue;
                };
                log_stor!(debug "Offline subkey write: {}:{} len={}", key, subkey, value.value_data().data().len());
                if let Err(e) = self
                    .outbound_set_value(
                        rpc_processor.clone(),
                        key,
                        subkey,
                        osw.safety_selection,
                        value,
                        descriptor,
                    )
                    .await
                {
                    log_stor!(debug "failed to write offline subkey: {}", e);
                }
            }
            let mut inner = self.lock().await?;
            inner.offline_subkey_writes.remove(&key);
        }

        Ok(())
    }
}
