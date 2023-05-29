use super::*;

impl StorageManager {
    pub(crate) async fn debug_local_records(&self) -> String {
        let inner = self.inner.lock().await;
        let Some(local_record_store) = &inner.local_record_store else {
            return "not initialized".to_owned();
        };
        local_record_store.debug_records()
    }
    pub(crate) async fn debug_remote_records(&self) -> String {
        let inner = self.inner.lock().await;
        let Some(remote_record_store) = &inner.remote_record_store else {
            return "not initialized".to_owned();
        };
        remote_record_store.debug_records()
    }
}
