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
        let (rpc_processor, offline_subkey_writes) = {
            let inner = self.lock().await?;

            let Some(rpc_processor) = inner.rpc_processor.clone() else {
                return Ok(());
            };

            (rpc_processor, inner.offline_subkey_writes.clone())
        };

        // make a safety selection that is conservative
        for (key, osw) in offline_subkey_writes {
            if poll!(stop_token.clone()).is_ready() {
                break;
            }
            for subkey in osw.subkeys.iter() {
                let subkey_result = {
                    let mut inner = self.lock().await?;
                    inner.handle_get_local_value(key, subkey, true).await
                };
                let Ok(subkey_result) = subkey_result else {
                    continue;
                };
                let Some(value) = subkey_result.value else {
                    continue;
                };
                let Some(descriptor) = subkey_result.descriptor else {
                    continue;
                };
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
