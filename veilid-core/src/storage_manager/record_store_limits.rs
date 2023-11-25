use super::*;

/// Configuration for the record store
#[derive(Debug, Default, Copy, Clone)]
pub struct RecordStoreLimits {
    /// Number of subkeys to keep in the memory cache
    pub subkey_cache_size: usize,
    /// Maximum size of an individual subkey
    pub max_subkey_size: usize,
    /// Maximum total record data size per record
    pub max_record_total_size: usize,
    /// Limit on the total number of records in the table store
    pub max_records: Option<usize>,
    /// Limit on the amount of subkey cache memory to use before evicting cache items
    pub max_subkey_cache_memory_mb: Option<usize>,
    /// Limit on the amount of storage space to use for subkey data and record data
    pub max_storage_space_mb: Option<usize>,
    /// Max number of anonymous watches
    pub public_watch_limit: usize,
    /// Max number of watches per schema member
    pub member_watch_limit: usize,
    /// Max expiration duration per watch
    pub max_watch_expiration: TimestampDuration,
    /// Min expiration duration per watch
    pub min_watch_expiration: TimestampDuration,
}
