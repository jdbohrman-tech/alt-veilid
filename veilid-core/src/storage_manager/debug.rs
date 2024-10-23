use super::*;

impl StorageManager {
    pub async fn debug_local_records(&self) -> String {
        let inner = self.inner.lock().await;
        let Some(local_record_store) = &inner.local_record_store else {
            return "not initialized".to_owned();
        };
        local_record_store.debug_records()
    }
    pub async fn debug_remote_records(&self) -> String {
        let inner = self.inner.lock().await;
        let Some(remote_record_store) = &inner.remote_record_store else {
            return "not initialized".to_owned();
        };
        remote_record_store.debug_records()
    }
    pub async fn debug_opened_records(&self) -> String {
        let inner = self.inner.lock().await;
        let mut out = "[\n".to_owned();
        for (k, v) in &inner.opened_records {
            let writer = if let Some(w) = v.writer() {
                w.to_string()
            } else {
                "".to_owned()
            };
            let watch = if let Some(w) = v.active_watch() {
                format!("  watch: {:?}\n", w)
            } else {
                "".to_owned()
            };
            out += &format!("  {} {}{}\n", k, writer, watch);
        }
        format!("{}]\n", out)
    }
    pub async fn debug_offline_records(&self) -> String {
        let inner = self.inner.lock().await;
        let mut out = "[\n".to_owned();
        for (k, v) in &inner.offline_subkey_writes {
            out += &format!("  {}:{:?}\n", k, v);
        }
        format!("{}]\n", out)
    }

    pub async fn purge_local_records(&self, reclaim: Option<usize>) -> String {
        let mut inner = self.inner.lock().await;
        let Some(local_record_store) = &mut inner.local_record_store else {
            return "not initialized".to_owned();
        };
        let reclaimed = local_record_store
            .reclaim_space(reclaim.unwrap_or(usize::MAX))
            .await;
        inner.offline_subkey_writes.clear();
        format!("Local records purged: reclaimed {} bytes", reclaimed)
    }
    pub async fn purge_remote_records(&self, reclaim: Option<usize>) -> String {
        let mut inner = self.inner.lock().await;
        let Some(remote_record_store) = &mut inner.remote_record_store else {
            return "not initialized".to_owned();
        };
        let reclaimed = remote_record_store
            .reclaim_space(reclaim.unwrap_or(usize::MAX))
            .await;
        format!("Remote records purged: reclaimed {} bytes", reclaimed)
    }
    pub async fn debug_local_record_subkey_info(
        &self,
        key: TypedKey,
        subkey: ValueSubkey,
    ) -> String {
        let inner = self.inner.lock().await;
        let Some(local_record_store) = &inner.local_record_store else {
            return "not initialized".to_owned();
        };
        local_record_store
            .debug_record_subkey_info(key, subkey)
            .await
    }
    pub async fn debug_remote_record_subkey_info(
        &self,
        key: TypedKey,
        subkey: ValueSubkey,
    ) -> String {
        let inner = self.inner.lock().await;
        let Some(remote_record_store) = &inner.remote_record_store else {
            return "not initialized".to_owned();
        };
        remote_record_store
            .debug_record_subkey_info(key, subkey)
            .await
    }
    pub async fn debug_local_record_info(&self, key: TypedKey) -> String {
        let inner = self.inner.lock().await;
        let Some(local_record_store) = &inner.local_record_store else {
            return "not initialized".to_owned();
        };
        let local_debug = local_record_store.debug_record_info(key);

        let opened_debug = if let Some(o) = inner.opened_records.get(&key) {
            format!("Opened Record: {:#?}\n", o)
        } else {
            "".to_owned()
        };

        format!("{}\n{}", local_debug, opened_debug)
    }

    pub async fn debug_remote_record_info(&self, key: TypedKey) -> String {
        let inner = self.inner.lock().await;
        let Some(remote_record_store) = &inner.remote_record_store else {
            return "not initialized".to_owned();
        };
        remote_record_store.debug_record_info(key)
    }
}
