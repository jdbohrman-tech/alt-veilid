use super::*;

impl RPCProcessor {
    pub fn debug_info_nodeinfo(&self) -> String {
        let mut out = String::new();
        let inner = self.inner.lock();
        out += &format!(
            "RPC Worker Dequeue Latency:\n{}",
            indent_all_string(&inner.rpc_worker_dequeue_latency)
        );
        out += "\n";
        out += &format!(
            "RPC Worker Process Latency:\n{}",
            indent_all_string(&inner.rpc_worker_process_latency)
        );
        out
    }
}
