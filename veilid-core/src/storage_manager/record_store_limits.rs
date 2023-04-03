use super::*;

/// Configuration for the record store
#[derive(Debug, Default, Copy, Clone)]
pub struct RecordStoreLimits {
    pub record_cache_size: u32,
    pub subkey_cache_size: u32,
    pub max_records: Option<u32>,
    pub max_cache_memory_mb: Option<u32>,
    pub max_disk_space_mb: Option<u32>,
}
