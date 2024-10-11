/// Microseconds since epoch
use super::*;

aligned_u64_type!(TimestampDuration);
aligned_u64_type_default_debug_impl!(TimestampDuration);
aligned_u64_type_default_math_impl!(TimestampDuration);

impl fmt::Display for TimestampDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", display_duration(self.as_u64()))
    }
}

impl TimestampDuration {
    pub const fn new_secs(secs: u32) -> Self {
        TimestampDuration::new(secs as u64 * 1_000_000u64)
    }
    pub const fn new_ms(ms: u64) -> Self {
        TimestampDuration::new(ms * 1_000u64)
    }
}
