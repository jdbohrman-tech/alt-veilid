use super::*;

impl NetworkManager {
    // Clean up the public address check tables, removing entries that have timed out
    #[instrument(level = "trace", skip(self), err)]
    pub(crate) async fn public_address_check_task_routine(
        self,
        stop_token: StopToken,
        _last_ts: u64,
        cur_ts: u64,
    ) -> EyreResult<()> {
        // go through public_address_inconsistencies_table and time out things that have expired
        let mut inner = self.inner.lock();
        for (_, pait_v) in &mut inner.public_address_inconsistencies_table {
            let mut expired = Vec::new();
            for (addr, exp_ts) in pait_v.iter() {
                if *exp_ts <= cur_ts {
                    expired.push(*addr);
                }
            }
            for exp in expired {
                pait_v.remove(&exp);
            }
        }
        Ok(())
    }
}
