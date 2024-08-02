/// Microseconds since epoch
use super::*;

aligned_u64_type!(TimestampDuration);
aligned_u64_type_default_math_impl!(TimestampDuration);

impl fmt::Display for TimestampDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (&self.0 as &dyn fmt::Display).fmt(f)
    }
}

impl FromStr for TimestampDuration {
    type Err = <u64 as FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(TimestampDuration(u64::from_str(s)?))
    }
}

impl fmt::Debug for TimestampDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (&self.0 as &dyn fmt::Debug).fmt(f)
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
