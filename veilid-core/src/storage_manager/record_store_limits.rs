use super::*;

/// Configuration for the record store
#[derive(Debug, Default, Copy, Clone)]
pub struct RecordStoreLimits {
    /// Number of subkeys to keep in the memory cache
    pub subkey_cache_size: u32,
    /// Limit on the total number of records in the table store
    pub max_records: Option<u32>,
    /// Limit on the amount of subkey cache memory to use before evicting cache items
    pub max_subkey_cache_memory_mb: Option<u32>,
    /// Limit on the amount of disk space to use for subkey data
    pub max_disk_space_mb: Option<u32>,
}
