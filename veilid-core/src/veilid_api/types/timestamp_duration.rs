/// Microseconds since epoch
use super::*;

aligned_u64_type!(TimestampDuration);
aligned_u64_type_default_display_impl!(TimestampDuration);
aligned_u64_type_default_math_impl!(TimestampDuration);

impl fmt::Debug for TimestampDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", debug_duration(self.as_u64()))
    }
}

impl TimestampDuration {
    pub fn new_secs<N: num_traits::Unsigned + num_traits::ToPrimitive>(secs: N) -> Self {
        TimestampDuration::new(secs.to_u64().unwrap() * 1_000_000u64)
    }
    pub fn new_ms<N: num_traits::Unsigned + num_traits::ToPrimitive>(ms: N) -> Self {
        TimestampDuration::new(ms.to_u64().unwrap() * 1_000u64)
    }
}
