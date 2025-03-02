use super::*;

impl NetworkManager {
    pub fn debug_info_nodeinfo(&self) -> String {
        let mut out = String::new();
        let inner = self.inner.lock();
        out += &format!(
            "Relay Worker Deque Latency:\n{}",
            indent_all_string(&inner.stats.relay_worker_dequeue_latency)
        );
        out += "\n";
        out += &format!(
            "Relay Worker Process Latency:\n{}",
            indent_all_string(&inner.stats.relay_worker_process_latency)
        );
        out
    }

    pub fn debug(&self) -> String {
        let stats = self.get_stats();

        let mut out = String::new();
        out += "Network Manager\n";
        out += "---------------\n";
        let mut out = format!(
            "Transfer stats:\n{}\n",
            indent_all_string(&stats.self_stats.transfer_stats)
        );
        out += &self.debug_info_nodeinfo();

        out += "Node Contact Method Cache\n";
        out += "-------------------------\n";
        out += &self.inner.lock().node_contact_method_cache.debug();

        out
    }
}
